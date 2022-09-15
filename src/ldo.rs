/*
** Stack and Call structure of Lua
*/

use std::env;
use std::panic::{self, catch_unwind, panic_any, resume_unwind, PanicInfo};
use std::process::abort;
use std::ptr;
use std::sync::Arc;

use libc::{c_char, c_int, c_void, ptrdiff_t, strchr};

use crate::lapi::{adjustresults, api_checknelems, api_incr_top};
use crate::ldebug::{luaG_runerror, luaG_typeerror};
use crate::lfunc::{luaF_close, luaF_initupvals};
use crate::lgc::luaC_checkGC;
use crate::llimits::{LUAI_MAXCCALLS, LUAI_MAXSTACK, LUA_IDSIZE};
use crate::lmem::{luaM_freearray, luaM_reallocvector};
use crate::lobject::{
    clCvalue, clLvalue, fvalue, luaO_nilobject_, luaO_pushfstring, setnilvalue, setobj, setsvalue,
    ttisfunction, ttype, Proto, StkId, TValue, LUA_TCCL, LUA_TLCF, LUA_TLCL,
};
use crate::lopcodes::{GET_OPCODE, OP_TAILCALL};
use crate::lparser::{luaY_parser, Dyndata};
use crate::lstate::{
    getoah, isLua, luaE_extendCI, luaE_freeCI, luaE_shrinkCI, lua_State, CallInfo, CIST_HOOKED,
    CIST_LUA, CIST_TAIL, CIST_YPCALL, EXTRA_STACK,
};
use crate::lstring::{luaS_new, luaS_newliteral};
use crate::ltm::{luaT_gettmbyobj, TM_CALL};
use crate::lundump::luaU_undump;
use crate::lvm::{luaV_execute, luaV_finishOp};
use crate::lzio::{luaZ_freebuffer, zgetc, Mbuffer, ZIO};
use crate::types::{
    lua_Debug, lua_KContext, lua_KFunction, LUA_ERRERR, LUA_ERRMEM, LUA_ERRRUN, LUA_ERRSYNTAX,
    LUA_HOOKCALL, LUA_HOOKRET, LUA_HOOKTAILCALL, LUA_MASKCALL, LUA_MASKLINE, LUA_MASKRET,
    LUA_MINSTACK, LUA_MULTRET, LUA_OK, LUA_SIGNATURE, LUA_YIELD,
};

/*
** Macro to check stack size and grow stack if needed.  Parameters
** 'pre'/'pos' allow the macro to preserve a pointer into the
** stack across reallocations, doing the work only when needed.
** 'condmovestack' is used in heavy tests to force a stack reallocation
** at every check.
*/
pub unsafe fn luaD_checkstackaux(
    L: *mut lua_State,
    n: i32,
    mut pre: impl FnMut(),
    mut pos: impl FnMut(),
) {
    if (*L).stack_last.offset_from((*L).top) <= n as isize {
        pre();
        luaD_growstack(L, n);
        pos();
    } else {
        #[cfg(debug_assertions)]
        if env::var("LUA_HARDSTACKTESTS").as_deref() == Ok("1") {
            let sz = (*L).stacksize;
            pre();
            // realloc stack keeping its size
            luaD_reallocstack(L, sz);
            pos();
        }
    }
}

/* In general, 'pre'/'pos' are empty (nothing to save) */
#[inline(always)]
pub unsafe fn luaD_checkstack(L: *mut lua_State, n: i32) {
    luaD_checkstackaux(L, n, || (), || ());
}

#[inline(always)]
pub unsafe fn savestack(L: *mut lua_State, p: *const TValue) -> ptrdiff_t {
    (p as *const c_char).offset_from((*L).stack as *const c_char)
}

#[inline(always)]
pub unsafe fn restorestack(L: *mut lua_State, n: ptrdiff_t) -> *mut TValue {
    ((*L).stack as *mut c_char).offset(n) as *mut TValue
}

/* type of protected functions, to be ran by 'runprotected' */
pub type Pfunc = Option<unsafe extern "C" fn(*mut lua_State, *mut c_void)>;

const fn errorstatus(s: c_int) -> bool {
    s > LUA_YIELD
}

/* chain list of long jump buffers */
#[repr(C)]
pub struct lua_longjmp {
    pub previous: *mut lua_longjmp,
    pub status: c_int,
}

struct LuaPanic(Arc<Box<dyn Fn(&PanicInfo<'_>) + Sync + Send + 'static>>);

// struct LuaLongjmp(*mut lua_longjmp);

// unsafe impl Send for LuaLongjmp {}

unsafe extern "C" fn seterrorobj(L: *mut lua_State, errcode: c_int, oldtop: StkId) {
    match errcode {
        LUA_ERRMEM => {
            /* memory error? */
            setsvalue(L, oldtop, (*(*L).l_G).memerrmsg); /* reuse preregistered msg. */
        }
        LUA_ERRERR => {
            setsvalue(L, oldtop, luaS_newliteral(L, "error in error handling"));
        }
        _ => {
            setobj(L, oldtop, (*L).top.sub(1)); /* error message on current top */
        }
    }
    (*L).top = oldtop.add(1);
}

#[no_mangle]
pub unsafe extern "C" fn luaD_throw(L: *mut lua_State, errcode: c_int) -> ! {
    if !((*L).errorJmp).is_null() {
        /* thread has an error handler */
        (*(*L).errorJmp).status = errcode; /* set status */
        let prev_hook = Arc::new(panic::take_hook());
        let prev_hook2 = prev_hook.clone();
        panic::set_hook(Box::new(move |panic_info| {
            if !panic_info.payload().is::<LuaPanic>() {
                prev_hook(panic_info);
            }
        }));
        panic_any(LuaPanic(prev_hook2)); /* jump to it */
    } else {
        /* thread has no error handler */
        let g = (*L).l_G;
        (*L).status = errcode as u8; /* mark it as dead */
        if !((*(*g).mainthread).errorJmp).is_null() {
            /* main thread has a handler */
            setobj(L, (*(*g).mainthread).top, (*L).top.sub(1)); /* copy error obj. */
            (*(*g).mainthread).top = (*(*g).mainthread).top.add(1);
            luaD_throw((*g).mainthread, errcode); /* re-throw in main thread */
        } else {
            /* no handler at all; abort */
            if let Some(panic_fn) = (*g).panic {
                /* has panic function */
                seterrorobj(L, errcode, (*L).top); /* assume EXTRA_STACK */
                if (*(*L).ci).top < (*L).top {
                    (*(*L).ci).top = (*L).top; /* pushing msg. can break this invariant */
                }
                panic_fn(L); /* call panic function (last chance to jump out) */
            }
            abort();
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaD_rawrunprotected(
    L: *mut lua_State,
    f: Pfunc,
    ud: *mut c_void,
) -> c_int {
    let oldnCcalls = (*L).nCcalls;
    let mut lj = lua_longjmp {
        previous: (*L).errorJmp, /* chain new error handler */
        status: LUA_OK,
    };
    (*L).errorJmp = &mut lj;
    if let Err(err) = catch_unwind(|| (f.unwrap())(L, ud)) {
        match err.downcast::<LuaPanic>() {
            Ok(p) => {
                if lj.status == 0 {
                    lj.status = -1;
                }
                drop(panic::take_hook());
                if let Ok(h) = Arc::try_unwrap(p.0) {
                    panic::set_hook(h);
                }
            }
            Err(err) => resume_unwind(err),
        }
    }
    (*L).errorJmp = lj.previous; /* restore old error handler */
    (*L).nCcalls = oldnCcalls;
    return lj.status;
}

/*
** Stack reallocation
*/

unsafe extern "C" fn correctstack(L: *mut lua_State, oldstack: *mut TValue) {
    (*L).top = ((*L).stack).offset(((*L).top).offset_from(oldstack));
    let mut up = (*L).openupval;
    while !up.is_null() {
        (*up).v = ((*L).stack).offset(((*up).v).offset_from(oldstack));
        up = (*up).u.open.next;
    }
    let mut ci = (*L).ci;
    while !ci.is_null() {
        (*ci).top = ((*L).stack).offset(((*ci).top).offset_from(oldstack));
        (*ci).func = ((*L).stack).offset(((*ci).func).offset_from(oldstack));
        if isLua(ci) {
            (*ci).u.l.base = ((*L).stack).offset(((*ci).u.l.base).offset_from(oldstack));
        }
        ci = (*ci).previous;
    }
}

// some space for error handling
const ERRORSTACKSIZE: usize = LUAI_MAXSTACK + 200;

#[no_mangle]
pub unsafe extern "C" fn luaD_reallocstack(L: *mut lua_State, newsize: c_int) {
    let oldstack = (*L).stack;
    let mut lim = (*L).stacksize;
    debug_assert!((newsize as usize) <= LUAI_MAXSTACK || (newsize as usize) == ERRORSTACKSIZE);
    debug_assert!(
        (*L).stack_last.offset_from((*L).stack) as usize == (*L).stacksize as usize - EXTRA_STACK
    );
    luaM_reallocvector::<TValue>(
        L,
        &mut (*L).stack,
        (*L).stacksize as usize,
        newsize as usize,
    );
    while lim < newsize {
        setnilvalue((*L).stack.add(lim as usize)); /* erase new segment */
        lim += 1;
    }
    (*L).stacksize = newsize;
    (*L).stack_last = ((*L).stack).offset(newsize as isize - EXTRA_STACK as isize);
    correctstack(L, oldstack);
}

#[no_mangle]
pub unsafe extern "C" fn luaD_growstack(L: *mut lua_State, n: c_int) {
    let size = (*L).stacksize as usize;
    if size as usize > LUAI_MAXSTACK {
        // error after extra size?
        luaD_throw(L, LUA_ERRERR);
    } else {
        let needed =
            ((*L).top.offset_from((*L).stack) + n as isize + EXTRA_STACK as isize) as usize;
        let mut newsize = 2 * size;
        if newsize > LUAI_MAXSTACK {
            newsize = LUAI_MAXSTACK;
        }
        if newsize < needed {
            newsize = needed;
        }
        if newsize > LUAI_MAXSTACK {
            luaD_reallocstack(L, ERRORSTACKSIZE as c_int);
            luaG_runerror(L, cstr!("stack overflow"));
        } else {
            luaD_reallocstack(L, newsize as c_int);
        }
    };
}

unsafe extern "C" fn stackinuse(L: *mut lua_State) -> c_int {
    let mut lim = (*L).top;
    let mut ci = (*L).ci;
    while !ci.is_null() {
        if lim < (*ci).top {
            lim = (*ci).top;
        }
        ci = (*ci).previous;
    }
    return (lim.offset_from((*L).stack) + 1) as c_int; // part of stack in usep
}

#[no_mangle]
pub unsafe extern "C" fn luaD_shrinkstack(L: *mut lua_State) {
    let inuse = stackinuse(L) as usize;
    let mut goodsize = inuse + (inuse / 8) + (2 * EXTRA_STACK);
    if goodsize > LUAI_MAXSTACK {
        goodsize = LUAI_MAXSTACK; // respect stack limit
    }
    if (*L).stacksize as usize > LUAI_MAXSTACK {
        // had been handling stack overflow?
        luaE_freeCI(L); // free all CIs (list grew because of an error)
    } else {
        luaE_shrinkCI(L); // shrink list
    }
    /* if thread is currently not handling a stack overflow and its
    good size is smaller than current size, shrink its stack */
    if inuse <= LUAI_MAXSTACK - EXTRA_STACK && goodsize < (*L).stacksize as usize {
        luaD_reallocstack(L, goodsize as i32);
    } else {
        /* don't change stack; (change only for debugging) */
        #[cfg(debug_assertions)]
        if env::var("LUA_HARDSTACKTESTS").as_deref() == Ok("1") {
            // realloc stack keeping its size
            luaD_reallocstack(L, (*L).stacksize);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaD_inctop(L: *mut lua_State) {
    luaD_checkstack(L, 1);
    (*L).top = ((*L).top).offset(1);
}

/*
** Call a hook for the given event. Make sure there is a hook to be
** called. (Both 'L->hook' and 'L->hookmask', which triggers this
** function, can be changed asynchronously by signals.)
*/
#[no_mangle]
pub unsafe extern "C" fn luaD_hook(L: *mut lua_State, event: c_int, line: c_int) {
    let hook = (*L).hook;
    if hook.is_some() && (*L).allowhook != 0 {
        let mut ci = (*L).ci;
        let top = savestack(L, (*L).top);
        let ci_top = savestack(L, (*ci).top);
        let mut ar = lua_Debug {
            event: 0,
            name: ptr::null(),
            namewhat: ptr::null(),
            what: ptr::null(),
            source: ptr::null(),
            currentline: 0,
            linedefined: 0,
            lastlinedefined: 0,
            nups: 0,
            nparams: 0,
            isvararg: 0,
            istailcall: 0,
            short_src: [0; LUA_IDSIZE],
            i_ci: ptr::null_mut(),
        };
        ar.event = event;
        ar.currentline = line;
        ar.i_ci = ci;
        luaD_checkstack(L, LUA_MINSTACK as i32); /* ensure minimum stack size */
        (*ci).top = ((*L).top).add(LUA_MINSTACK);
        debug_assert!((*ci).top <= (*L).stack_last);
        (*L).allowhook = 0; /* cannot call hooks inside a hook */
        (*ci).callstatus |= CIST_HOOKED;
        (hook.unwrap())(L, &mut ar);
        debug_assert!((*L).allowhook == 0);
        (*L).allowhook = 1;
        (*ci).top = restorestack(L, ci_top);
        (*L).top = restorestack(L, top);
        (*ci).callstatus &= !CIST_HOOKED;
    }
}

unsafe extern "C" fn callhook(L: *mut lua_State, ci: *mut CallInfo) {
    let mut hook = LUA_HOOKCALL;
    (*ci).u.l.savedpc = ((*ci).u.l.savedpc).add(1); /* hooks assume 'pc' is already incremented */
    if isLua((*ci).previous) && GET_OPCODE(*((*(*ci).previous).u.l.savedpc.sub(1))) == OP_TAILCALL {
        (*ci).callstatus |= CIST_TAIL;
        hook = LUA_HOOKTAILCALL;
    }
    luaD_hook(L, hook, -1);
    (*ci).u.l.savedpc = ((*ci).u.l.savedpc).sub(1); /* correct 'pc' */
}

unsafe extern "C" fn adjust_varargs(L: *mut lua_State, p: *mut Proto, actual: c_int) -> StkId {
    let nfixargs = (*p).numparams;
    /* move fixed parameters to final position */
    let fixed = ((*L).top).offset(-(actual as isize)); /* first fixed argument */
    let base = (*L).top; /* final position of first argument */
    let mut i = 0;
    while i < nfixargs as i32 && i < actual {
        setobj(L, (*L).top, fixed.add(i as usize));
        (*L).top = (*L).top.add(1);
        setnilvalue(fixed.add(i as usize)); /* erase original copy (for GC) */
        i += 1;
    }
    while i < nfixargs as i32 {
        setnilvalue((*L).top); /* complete missing arguments */
        (*L).top = (*L).top.add(1);
        i += 1;
    }
    return base;
}

/*
** Check whether __call metafield of 'func' is a function. If so, put
** it in stack below original 'func' so that 'luaD_precall' can call
** it. Raise an error if __call metafield is not a function.
*/
unsafe extern "C" fn tryfuncTM(L: *mut lua_State, func: StkId) {
    let tm = luaT_gettmbyobj(L, func, TM_CALL);
    if !ttisfunction(tm) {
        luaG_typeerror(L, func, cstr!("call"));
    }
    /* Open a hole inside the stack at 'func' */
    let mut p = (*L).top;
    while p > func {
        setobj(L, p, p.sub(1));
        p = p.sub(1);
    }
    (*L).top = ((*L).top).offset(1); /* slot ensured by caller */
    setobj(L, func, tm); /* tag method is the new function to be called */
}

/*
** Given 'nres' results at 'firstResult', move 'wanted' of them to 'res'.
** Handle most typical cases (zero results for commands, one result for
** expressions, multiple results for tail calls/single parameters)
** separated.
*/
unsafe extern "C" fn moveresults(
    L: *mut lua_State,
    mut firstResult: *const TValue,
    res: StkId,
    nres: c_int,
    wanted: c_int,
) -> c_int {
    /* handle typical cases separately */
    match wanted {
        0 => { /* nothing to move */ }
        1 => {
            /* one result needed */
            if nres == 0 {
                /* no results? */
                firstResult = &luaO_nilobject_; /* adjust with nil */
            }
            setobj(L, res, firstResult); /* move it to proper place */
        }
        LUA_MULTRET => {
            for i in 0..nres {
                /* move all results to correct place */
                setobj(L, res.add(i as usize), firstResult.add(i as usize));
            }
            (*L).top = res.add(nres as usize);
            return 0; /* wanted == LUA_MULTRET */
        }
        _ => {
            if wanted <= nres {
                /* enough results? */
                for i in 0..wanted {
                    /* move wanted results to correct place */
                    setobj(L, res.add(i as usize), firstResult.add(i as usize));
                }
            } else {
                /* not enough results; use all of them plus nils */
                for i in 0..nres {
                    /* move wanted results to correct place */
                    setobj(L, res.add(i as usize), firstResult.add(i as usize));
                }
                for i in nres..wanted {
                    /* complete wanted number of results */
                    setnilvalue(res.add(i as usize));
                }
            }
        }
    }
    (*L).top = res.add(wanted as usize); /* top points after the last result */
    return 1;
}

/*
** Finishes a function call: calls hook if necessary, removes CallInfo,
** moves current number of results to proper place; returns 0 iff call
** wanted multiple (variable number of) results.
*/
#[no_mangle]
pub unsafe extern "C" fn luaD_poscall(
    L: *mut lua_State,
    ci: *mut CallInfo,
    mut firstResult: StkId,
    nres: c_int,
) -> c_int {
    let wanted = (*ci).nresults as c_int;
    if (*L).hookmask & (LUA_MASKRET | LUA_MASKLINE) != 0 {
        if (*L).hookmask & LUA_MASKRET != 0 {
            let fr = savestack(L, firstResult); /* hook may change stack */
            luaD_hook(L, LUA_HOOKRET, -1);
            firstResult = restorestack(L, fr);
        }
        (*L).oldpc = (*(*ci).previous).u.l.savedpc; /* 'oldpc' for caller function */
    }
    let res = (*ci).func; /* res == final position of 1st result */
    (*L).ci = (*ci).previous; /* back to caller */
    /* move results to proper place */
    return moveresults(L, firstResult, res, nres, wanted);
}

pub unsafe fn next_ci(L: *mut lua_State) -> *mut CallInfo {
    (*L).ci = if !(*(*L).ci).next.is_null() {
        (*(*L).ci).next
    } else {
        luaE_extendCI(L)
    };
    (*L).ci
}

/* macro to check stack size, preserving 'p' */
unsafe fn checkstackp(L: *mut lua_State, n: i32, p: *mut *mut TValue) {
    let mut t = 0isize;
    let t = &mut t as *mut isize;
    luaD_checkstackaux(
        L,
        n,
        move || {
            *t = savestack(L, *p); /* save 'p' */
            luaC_checkGC(L); /* stack grow uses memory */
        },
        move || {
            *p = restorestack(L, *t); /* 'pos' part: restore 'p' */
        },
    );
}

/*
** Prepares a function call: checks the stack, creates a new CallInfo
** entry, fills in the relevant information, calls hook if needed.
** If function is a C function, does the call, too. (Otherwise, leave
** the execution ('luaV_execute') to the caller, to allow stackless
** calls.) Returns true if function has been executed (C function).
*/
#[no_mangle]
pub unsafe extern "C" fn luaD_precall(
    L: *mut lua_State,
    mut func: StkId,
    nresults: c_int,
) -> c_int {
    match ttype(func) {
        t @ LUA_TCCL | t @ LUA_TLCF => {
            let f = if t == LUA_TCCL {
                (*clCvalue(func)).f
            } else {
                fvalue(func)
            };
            checkstackp(L, LUA_MINSTACK as i32, &mut func); /* ensure minimum stack size */
            let ci = next_ci(L); /* now 'enter' new function */
            (*ci).nresults = nresults as i16;
            (*ci).func = func;
            (*ci).top = (*L).top.add(LUA_MINSTACK);
            debug_assert!((*ci).top <= (*L).stack_last);
            (*ci).callstatus = 0;
            if (*L).hookmask & LUA_MASKCALL != 0 {
                luaD_hook(L, LUA_HOOKCALL, -1);
            }
            /* n is number of returns */
            let n = f.expect("non-null function pointer")(L); /* do the actual call */
            api_checknelems(L, n);
            luaD_poscall(L, ci, (*L).top.sub(n as usize), n);
            return 1;
        }
        LUA_TLCL => {
            /* Lua function: prepare its call */
            let p = (*clLvalue(func)).p;
            let mut n = (*L).top.offset_from(func) - 1; /* number of real arguments */
            let fsize = (*p).maxstacksize; /* frame size */
            checkstackp(L, fsize as i32, &mut func);
            let base = if (*p).is_vararg != 0 {
                adjust_varargs(L, p, n as i32)
            } else {
                /* non vararg function */
                while n < (*p).numparams as isize {
                    setnilvalue((*L).top); /* complete missing arguments */
                    (*L).top = (*L).top.add(1);
                    n += 1;
                }
                func.add(1)
            };
            /* now 'enter' new function */
            let ci = next_ci(L);
            (*ci).nresults = nresults as i16;
            (*ci).func = func;
            (*ci).u.l.base = base;
            (*ci).top = base.add(fsize as usize);
            (*L).top = (*ci).top;
            (*ci).u.l.savedpc = (*p).code; /* starting point */
            (*ci).callstatus = CIST_LUA;
            if (*L).hookmask & LUA_MASKCALL != 0 {
                callhook(L, ci);
            }
            return 0;
        }
        _ => {
            /* not a function */
            checkstackp(L, 1, &mut func); /* ensure space for metamethod */
            tryfuncTM(L, func); /* try to get '__call' metamethod */
            return luaD_precall(L, func, nresults); /* now it must be a function */
        }
    }
}

/*
** Check appropriate error for stack overflow ("regular" overflow or
** overflow while handling stack overflow). If 'nCalls' is larger than
** LUAI_MAXCCALLS (which means it is handling a "regular" overflow) but
** smaller than 9/8 of LUAI_MAXCCALLS, does not report an error (to
** allow overflow handling to work)
*/
unsafe extern "C" fn stackerror(L: *mut lua_State) {
    if (*L).nCcalls == LUAI_MAXCCALLS {
        luaG_runerror(L, cstr!("C stack overflow"));
    } else {
        if (*L).nCcalls >= LUAI_MAXCCALLS + (LUAI_MAXCCALLS >> 3) {
            luaD_throw(L, LUA_ERRERR); /* error while handing stack error */
        }
    }
}

/*
** Call a function (C or Lua). The function to be called is at *func.
** The arguments are on the stack, right after the function.
** When returns, all the results are on the stack, starting at the original
** function position.
*/
#[no_mangle]
pub unsafe extern "C" fn luaD_call(L: *mut lua_State, func: StkId, nResults: c_int) {
    (*L).nCcalls += 1;
    if (*L).nCcalls >= LUAI_MAXCCALLS {
        stackerror(L);
    }
    if luaD_precall(L, func, nResults) == 0 {
        /* is a Lua function? */
        luaV_execute(L); /* call it */
    }
    (*L).nCcalls -= 1;
}

/*
** Similar to 'luaD_call', but does not allow yields during the call
*/
#[no_mangle]
pub unsafe extern "C" fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: c_int) {
    (*L).nny += 1;
    luaD_call(L, func, nResults);
    (*L).nny -= 1;
}

/*
** Completes the execution of an interrupted C function, calling its
** continuation function.
*/
unsafe extern "C" fn finishCcall(L: *mut lua_State, status: c_int) {
    let mut ci = (*L).ci;
    /* must have a continuation and must be able to call it */
    debug_assert!((*ci).u.c.k.is_some() && (*L).nny == 0);
    /* error status can only happen in a protected call */
    debug_assert!((*ci).callstatus & CIST_YPCALL != 0 || status == LUA_YIELD);
    if (*ci).callstatus & CIST_YPCALL != 0 {
        /* was inside a pcall? */
        (*ci).callstatus &= !CIST_YPCALL; /* continuation is also inside it */
        (*L).errfunc = (*ci).u.c.old_errfunc; /* with the same error function */
    }
    /* finish 'lua_callk'/'lua_pcall'; CIST_YPCALL and 'errfunc' already
    handled */
    adjustresults(L, (*ci).nresults as i32);
    /* call continuation function */
    let n = ((*ci).u.c.k).expect("non-null function pointer")(L, status, (*ci).u.c.ctx);
    api_checknelems(L, n);
    luaD_poscall(L, ci, ((*L).top).offset(-(n as isize)), n); /* finish 'luaD_precall' */
}

/*
** Executes "full continuation" (everything in the stack) of a
** previously interrupted coroutine until the stack is empty (or another
** interruption long-jumps out of the loop). If the coroutine is
** recovering from an error, 'ud' points to the error status, which must
** be passed to the first continuation function (otherwise the default
** status is LUA_YIELD).
*/
unsafe extern "C" fn unroll(L: *mut lua_State, ud: *mut c_void) {
    if !ud.is_null() {
        /* error status? */
        finishCcall(L, *(ud as *mut c_int)); /* finish 'lua_pcallk' callee */
    }
    while (*L).ci != &mut (*L).base_ci {
        /* something in the stack */
        if !isLua((*L).ci) {
            /* C function? */
            finishCcall(L, LUA_YIELD); /* complete its execution */
        } else {
            /* Lua function */
            luaV_finishOp(L); /* finish interrupted instruction */
            luaV_execute(L); /* execute down to higher C 'boundary' */
        }
    }
}

/*
** Try to find a suspended protected call (a "recover point") for the
** given thread.
*/
unsafe extern "C" fn findpcall(L: *mut lua_State) -> *mut CallInfo {
    // let mut ci = 0 as *mut CallInfo;
    let mut ci = (*L).ci;
    while !ci.is_null() {
        /* search for a pcall */
        if (*ci).callstatus & CIST_YPCALL != 0 {
            return ci;
        }
        ci = (*ci).previous;
    }
    return ptr::null_mut(); /* no pending pcall */
}

/*
** Recovers from an error in a coroutine. Finds a recover point (if
** there is one) and completes the execution of the interrupted
** 'luaD_pcall'. If there is no recover point, returns zero.
*/
unsafe extern "C" fn recover(L: *mut lua_State, status: c_int) -> c_int {
    // let mut oldtop = 0 as *mut TValue;
    let ci = findpcall(L);
    if ci.is_null() {
        /* no recovery point */
        return 0;
    }
    /* "finish" luaD_pcall */
    let oldtop = restorestack(L, (*ci).extra);
    luaF_close(L, oldtop);
    seterrorobj(L, status, oldtop);
    (*L).ci = ci;
    (*L).allowhook = getoah((*ci).callstatus as u16) as u8; /* restore original 'allowhook' */
    (*L).nny = 0; /* should be zero to be yieldable */
    luaD_shrinkstack(L);
    (*L).errfunc = (*ci).u.c.old_errfunc;
    return 1; /* continue running the coroutine */
}

/*
** Signal an error in the call to 'lua_resume', not in the execution
** of the coroutine itself. (Such errors should not be handled by any
** coroutine error handler and should not kill the coroutine.)
*/
unsafe extern "C" fn resume_error(L: *mut lua_State, msg: *const c_char, narg: c_int) -> c_int {
    (*L).top = ((*L).top).sub(narg as usize); /* remove args from the stack */
    setsvalue(L, (*L).top, luaS_new(L, msg)); /* push error message */
    api_incr_top(L);
    return LUA_ERRRUN;
}

/*
** Do the work for 'lua_resume' in protected mode. Most of the work
** depends on the status of the coroutine: initial state, suspended
** inside a hook, or regularly suspended (optionally with a continuation
** function), plus erroneous cases: non-suspended coroutine or dead
** coroutine.
*/
unsafe extern "C" fn resume(L: *mut lua_State, ud: *mut c_void) {
    let mut n = *(ud as *const c_int); /* number of arguments */
    let mut firstArg = ((*L).top).sub(n as usize); /* first argument */
    let ci = (*L).ci;
    if (*L).status as c_int == LUA_OK {
        /* starting a coroutine? */
        if luaD_precall(L, firstArg.sub(1), LUA_MULTRET) == 0 {
            /* Lua function? */
            luaV_execute(L); /* call it */
        }
    } else {
        /* resuming from previous yield */
        debug_assert!((*L).status as c_int == LUA_YIELD);
        (*L).status = LUA_OK as u8; /* mark that it is running (again) */
        (*ci).func = restorestack(L, (*ci).extra);
        if isLua(ci) {
            /* yielded inside a hook? */
            luaV_execute(L); /* just continue running Lua code */
        } else {
            /* 'common' yield */
            if let Some(cont) = (*ci).u.c.k {
                /* does it have a continuation function? */
                n = cont(L, LUA_YIELD, (*ci).u.c.ctx); /* call continuation */
                api_checknelems(L, n);
                firstArg = (*L).top.sub(n as usize); /* yield results come from continuation */
            }
            luaD_poscall(L, ci, firstArg, n); /* finish 'luaD_precall' */
        }
        unroll(L, ptr::null_mut()); /* run continuation */
    }
}

#[no_mangle]
pub unsafe extern "C" fn lua_resume(
    L: *mut lua_State,
    from: *mut lua_State,
    mut nargs: c_int,
) -> c_int {
    let oldnny = (*L).nny; /* save "number of non-yieldable" calls */
    if (*L).status as c_int == LUA_OK {
        /* may be starting a coroutine */
        if (*L).ci != &mut (*L).base_ci {
            /* not in base level? */
            return resume_error(L, cstr!("cannot resume non-suspended coroutine"), nargs);
        }
    } else if (*L).status as c_int != LUA_YIELD {
        return resume_error(L, cstr!("cannot resume dead coroutine"), nargs);
    }
    (*L).nCcalls = if !from.is_null() {
        (*from).nCcalls + 1
    } else {
        1
    };
    if (*L).nCcalls >= LUAI_MAXCCALLS {
        return resume_error(L, cstr!("C stack overflow"), nargs);
    }
    (*L).nny = 0; /* allow yields */
    api_checknelems(
        L,
        if (*L).status as c_int == LUA_OK {
            nargs + 1
        } else {
            nargs
        },
    );
    let mut status = luaD_rawrunprotected(L, Some(resume), &mut nargs as *mut c_int as *mut c_void);
    if status == -1 {
        /* error calling 'lua_resume'? */
        status = LUA_ERRRUN;
    } else {
        /* continue running after recoverable errors */
        while errorstatus(status) && recover(L, status) != 0 {
            status =
                luaD_rawrunprotected(L, Some(unroll), &mut status as *mut c_int as *mut c_void);
        }
        if errorstatus(status) {
            /* unrecoverable error? */
            (*L).status = status as u8; /* mark thread as 'dead' */
            seterrorobj(L, status, (*L).top); /* push error message */
            (*(*L).ci).top = (*L).top;
        } else {
            debug_assert!(status == (*L).status as c_int); /* normal end or yield */
        }
    }
    (*L).nny = oldnny; /* restore 'nny' */
    (*L).nCcalls -= 1;
    debug_assert!((*L).nCcalls == if !from.is_null() { (*from).nCcalls } else { 0 });
    return status;
}

#[no_mangle]
pub unsafe extern "C" fn lua_isyieldable(L: *mut lua_State) -> c_int {
    return ((*L).nny == 0) as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_yieldk(
    L: *mut lua_State,
    nresults: c_int,
    ctx: lua_KContext,
    k: lua_KFunction,
) -> c_int {
    let mut ci = (*L).ci;
    api_checknelems(L, nresults);
    if (*L).nny > 0 {
        if L != (*(*L).l_G).mainthread {
            luaG_runerror(L, cstr!("attempt to yield across a C-call boundary"));
        } else {
            luaG_runerror(L, cstr!("attempt to yield from outside a coroutine"));
        }
    }
    (*L).status = LUA_YIELD as u8;
    (*ci).extra = savestack(L, (*ci).func); /* save current 'func' */
    if isLua(ci) {
        /* inside a hook? */
        debug_assert!(k.is_none(), "hooks cannot continue after yielding",);
    } else {
        (*ci).u.c.k = k;
        if ((*ci).u.c.k).is_some() {
            /* is there a continuation? */
            (*ci).u.c.ctx = ctx; /* save context */
        }
        (*ci).func = ((*L).top).offset(-nresults as isize - 1); /* protect stack below results */
        luaD_throw(L, LUA_YIELD);
    }
    debug_assert!((*ci).callstatus & CIST_HOOKED != 0); /* must be inside a hook */
    return 0; /* return to 'luaD_hook' */
}

#[no_mangle]
pub unsafe extern "C" fn luaD_pcall(
    L: *mut lua_State,
    func: Pfunc,
    u: *mut c_void,
    old_top: ptrdiff_t,
    ef: ptrdiff_t,
) -> c_int {
    // let mut status: c_int = 0;
    let old_ci = (*L).ci;
    let old_allowhooks = (*L).allowhook;
    let old_nny = (*L).nny;
    let old_errfunc = (*L).errfunc;
    (*L).errfunc = ef;
    let status = luaD_rawrunprotected(L, func, u);
    if status != LUA_OK {
        /* an error occurred? */
        let oldtop = restorestack(L, old_top);
        luaF_close(L, oldtop); /* close possible pending closures */
        seterrorobj(L, status, oldtop);
        (*L).ci = old_ci;
        (*L).allowhook = old_allowhooks;
        (*L).nny = old_nny;
        luaD_shrinkstack(L);
    }
    (*L).errfunc = old_errfunc;
    return status;
}

/*
** Execute a protected parser.
*/

/* data to 'f_parser' */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SParser {
    pub z: *mut ZIO,
    pub buff: Mbuffer, /* dynamic structure used by the scanner */
    pub dyd: Dyndata,  /* dynamic structures used by the parser */
    pub mode: *const c_char,
    pub name: *const c_char,
}

impl SParser {
    const fn new() -> Self {
        SParser {
            z: ptr::null_mut(),
            buff: Mbuffer::new(),
            dyd: Dyndata::new(),
            mode: ptr::null(),
            name: ptr::null(),
        }
    }
}

unsafe extern "C" fn checkmode(L: *mut lua_State, mode: *const c_char, x: *const c_char) {
    if !mode.is_null() && strchr(mode, *x as c_int).is_null() {
        luaO_pushfstring(
            L,
            cstr!("attempt to load a %s chunk (mode is '%s')"),
            x,
            mode,
        );
        luaD_throw(L, LUA_ERRSYNTAX);
    }
}

unsafe extern "C" fn f_parser(L: *mut lua_State, ud: *mut c_void) {
    let p = ud as *mut SParser;
    let c = zgetc((*p).z); /* read first character */
    let cl = if c == LUA_SIGNATURE[0] as i32 {
        checkmode(L, (*p).mode, cstr!("binary"));
        luaU_undump(L, (*p).z, (*p).name)
    } else {
        checkmode(L, (*p).mode, cstr!("text"));
        luaY_parser(L, (*p).z, &mut (*p).buff, &mut (*p).dyd, (*p).name, c)
    };
    debug_assert!((*cl).nupvalues as i32 == (*(*cl).p).sizeupvalues);
    luaF_initupvals(L, cl);
}

#[no_mangle]
pub unsafe extern "C" fn luaD_protectedparser(
    L: *mut lua_State,
    z: *mut ZIO,
    name: *const c_char,
    mode: *const c_char,
) -> c_int {
    let mut p = SParser::new();
    (*L).nny += 1; /* cannot yield during parsing */
    p.z = z;
    p.name = name;
    p.mode = mode;
    let status = luaD_pcall(
        L,
        Some(f_parser),
        &mut p as *mut SParser as *mut c_void,
        savestack(L, (*L).top),
        (*L).errfunc,
    );
    luaZ_freebuffer(L, &mut p.buff);
    luaM_freearray(L, p.dyd.actvar.arr, p.dyd.actvar.size as usize);
    luaM_freearray(L, p.dyd.gt.arr, p.dyd.gt.size as usize);
    luaM_freearray(L, p.dyd.label.arr, p.dyd.label.size as usize);
    (*L).nny -= 1;
    return status;
}
