/*
** Debug Interface
*/

use std::ptr;

use libc::{c_char, c_int, c_uint, strchr, strcmp};

use crate::lapi::api_incr_top;
use crate::ldo::{luaD_callnoyield, luaD_hook, luaD_throw, restorestack, savestack};
use crate::lfunc::luaF_getlocalname;
use crate::lgc::luaC_checkGC;
use crate::llex::LUA_ENV;
use crate::llimits::{Instruction, LUA_IDSIZE};
use crate::lobject::{
    clLvalue, clvalue, getstr, isLfunction, luaO_chunkid, luaO_pushfstring, luaO_pushvfstring,
    setbvalue, sethvalue, setnilvalue, setobj, svalue, ttisclosure, ttisfunction, ttisnumber,
    ttisstring, Closure, LClosure, Proto, StkId, TString, TValue, Value, LUA_TCCL,
};
use crate::lopcodes::{
    testAMode, GETARG_Ax, GETARG_Bx, GETARG_sBx, OpCode, GETARG_A, GETARG_B, GETARG_C, GET_OPCODE,
    INDEXK, ISK, OP_ADD, OP_BAND, OP_BNOT, OP_BOR, OP_BXOR, OP_CALL, OP_CONCAT, OP_DIV, OP_EQ,
    OP_GETTABLE, OP_GETTABUP, OP_GETUPVAL, OP_IDIV, OP_JMP, OP_LE, OP_LEN, OP_LOADK, OP_LOADKX,
    OP_LOADNIL, OP_LT, OP_MOD, OP_MOVE, OP_MUL, OP_POW, OP_SELF, OP_SETTABLE, OP_SETTABUP, OP_SHL,
    OP_SHR, OP_SUB, OP_TAILCALL, OP_TFORCALL, OP_UNM,
};
use crate::lstate::{isLua, lua_State, CallInfo, CIST_FIN, CIST_HOOKED, CIST_HOOKYIELD, CIST_TAIL};
use crate::ltable::{luaH_new, luaH_setint};
use crate::ltm::{
    luaT_objtypename, TMS, TM_ADD, TM_BNOT, TM_CONCAT, TM_EQ, TM_INDEX, TM_LE, TM_LEN, TM_LT,
    TM_NEWINDEX, TM_UNM,
};
use crate::lvm::{tointeger, tonumber};
use crate::types::{
    lua_Debug, lua_Hook, lua_Integer, LUA_ERRRUN, LUA_HOOKCOUNT, LUA_HOOKLINE, LUA_MASKCOUNT,
    LUA_MASKLINE, LUA_YIELD,
};

unsafe fn pcRel(pc: *const Instruction, p: *const Proto) -> isize {
    pc.offset_from((*p).code) - 1
}

unsafe fn getfuncline(f: *const Proto, pc: isize) -> c_int {
    if !(*f).lineinfo.is_null() {
        *(*f).lineinfo.offset(pc)
    } else {
        -1
    }
}

unsafe fn noLuaClosure(f: *const Closure) -> bool {
    f.is_null() || (*f).c.tt == LUA_TCCL as u8
}

/* Active Lua function (given call info) */
unsafe fn ci_func(ci: *const CallInfo) -> *mut LClosure {
    clLvalue((*ci).func)
}

unsafe extern "C" fn currentpc(ci: *mut CallInfo) -> isize {
    debug_assert!(isLua(ci));
    pcRel((*ci).u.l.savedpc, (*ci_func(ci)).p)
}

unsafe extern "C" fn currentline(ci: *mut CallInfo) -> c_int {
    getfuncline((*ci_func(ci)).p, currentpc(ci))
}

/*
** If function yielded, its 'func' can be in the 'extra' field. The
** next function restores 'func' to its correct value for debugging
** purposes. (It exchanges 'func' and 'extra'; so, when called again,
** after debugging, it also "re-restores" ** 'func' to its altered value.
*/
unsafe extern "C" fn swapextra(L: *mut lua_State) {
    if (*L).status as c_int == LUA_YIELD {
        let mut ci = (*L).ci; /* get function that yielded */
        let temp = (*ci).func; /* exchange its 'func' and 'extra' values */
        (*ci).func = restorestack(L, (*ci).extra);
        (*ci).extra = savestack(L, temp);
    }
}

/*
** This function can be called asynchronously (e.g. during a signal).
** Fields 'oldpc', 'basehookcount', and 'hookcount' (set by
** 'resethookcount') are for debug only, and it is no problem if they
** get arbitrary values (causes at most one wrong hook call). 'hookmask'
** is an atomic value. We assume that pointers are atomic too (e.g., gcc
** ensures that for all platforms where it runs). Moreover, 'hook' is
** always checked before being called (see 'luaD_hook').
*/
#[no_mangle]
pub unsafe extern "C" fn lua_sethook(
    L: *mut lua_State,
    mut func: lua_Hook,
    mut mask: c_int,
    count: c_int,
) {
    if func.is_none() || mask == 0 {
        // turn off hooks
        mask = 0;
        func = None;
    }
    if isLua((*L).ci) {
        (*L).oldpc = (*(*L).ci).u.l.savedpc;
    }
    (*L).hook = func;
    (*L).basehookcount = count;
    (*L).hookcount = (*L).basehookcount;
    (*L).hookmask = mask;
}

#[no_mangle]
pub unsafe extern "C" fn lua_gethook(L: *mut lua_State) -> lua_Hook {
    return (*L).hook;
}

#[no_mangle]
pub unsafe extern "C" fn lua_gethookmask(L: *mut lua_State) -> c_int {
    return (*L).hookmask;
}

#[no_mangle]
pub unsafe extern "C" fn lua_gethookcount(L: *mut lua_State) -> c_int {
    return (*L).basehookcount;
}

#[no_mangle]
pub unsafe extern "C" fn lua_getstack(
    L: *mut lua_State,
    mut level: c_int,
    ar: *mut lua_Debug,
) -> c_int {
    if level < 0 {
        /* invalid (negative) level */
        return 0;
    }
    let mut ci = (*L).ci;
    while level > 0 && ci != &mut (*L).base_ci as *mut CallInfo {
        level -= 1;
        ci = (*ci).previous;
    }
    let status;
    if level == 0 && ci != &mut (*L).base_ci as *mut CallInfo {
        /* level found? */
        status = 1;
        (*ar).i_ci = ci;
    } else {
        /* no such level */
        status = 0;
    }
    return status;
}

unsafe extern "C" fn upvalname(p: *mut Proto, uv: c_int) -> *const c_char {
    debug_assert!(uv < (*p).sizeupvalues);
    let s = (*(*p).upvalues.add(uv as usize)).name;
    if s.is_null() {
        return cstr!("?");
    } else {
        return getstr(s);
    };
}

unsafe extern "C" fn findvararg(ci: *mut CallInfo, n: c_int, pos: *mut StkId) -> *const c_char {
    let nparams = (*(*clLvalue((*ci).func)).p).numparams;
    let nvararg = (*ci).u.l.base.offset_from((*ci).func) as c_int - nparams as c_int;
    if n <= -nvararg {
        /* no such vararg */
        return ptr::null_mut();
    } else {
        *pos = ((*ci).func).offset((nparams as c_int - n) as isize);
        /* generic name for any vararg */
        return cstr!("(*vararg)");
    };
}

unsafe extern "C" fn findlocal(
    L: *mut lua_State,
    ci: *mut CallInfo,
    n: c_int,
    pos: *mut StkId,
) -> *const c_char {
    let mut name = ptr::null();
    let base;
    if isLua(ci) {
        if n < 0 {
            /* access to vararg values? */
            return findvararg(ci, n, pos);
        } else {
            base = (*ci).u.l.base;
            name = luaF_getlocalname((*ci_func(ci)).p, n, currentpc(ci) as c_int);
        }
    } else {
        base = ((*ci).func).add(1);
    }
    if name.is_null() {
        /* no 'standard' name? */
        let limit = if ci == (*L).ci {
            (*L).top
        } else {
            (*(*ci).next).func
        };
        if limit.offset_from(base) >= n as isize && n > 0 {
            /* is 'n' inside 'ci' stack? */
            /* generic name for any valid slot */
            name = cstr!("(*temporary)");
        } else {
            /* no name */
            return ptr::null();
        }
    }
    *pos = base.offset((n - 1) as isize);
    return name;
}

#[no_mangle]
pub unsafe extern "C" fn lua_getlocal(
    L: *mut lua_State,
    ar: *const lua_Debug,
    n: c_int,
) -> *const c_char {
    let name;
    swapextra(L);
    if ar.is_null() {
        /* information about non-active function? */
        if !isLfunction((*L).top.sub(1)) {
            /* not a Lua function? */
            name = ptr::null();
        } else {
            /* consider live variables at function start (parameters) */
            name = luaF_getlocalname((*clLvalue((*L).top.sub(1))).p, n, 0);
        }
    } else {
        /* active function; get information through 'ar' */
        let mut pos = ptr::null_mut();
        name = findlocal(L, (*ar).i_ci, n, &mut pos);
        if !name.is_null() {
            setobj(L, (*L).top, pos);
            api_incr_top(L);
        }
    }
    swapextra(L);
    return name;
}

#[no_mangle]
pub unsafe extern "C" fn lua_setlocal(
    L: *mut lua_State,
    ar: *const lua_Debug,
    n: c_int,
) -> *const c_char {
    let mut pos = ptr::null_mut();
    swapextra(L);
    let name = findlocal(L, (*ar).i_ci, n, &mut pos);
    if !name.is_null() {
        setobj(L, pos, (*L).top.sub(1));
        (*L).top = (*L).top.sub(1); // pop value
    }
    swapextra(L);
    return name;
}

unsafe extern "C" fn funcinfo(ar: *mut lua_Debug, cl: *mut Closure) {
    if noLuaClosure(cl) {
        (*ar).source = cstr!("=[C]");
        (*ar).linedefined = -1;
        (*ar).lastlinedefined = -1;
        (*ar).what = cstr!("C");
    } else {
        let p = (*cl).l.p;
        (*ar).source = if !(*p).source.is_null() {
            getstr((*p).source)
        } else {
            cstr!("=?")
        };
        (*ar).linedefined = (*p).linedefined;
        (*ar).lastlinedefined = (*p).lastlinedefined;
        (*ar).what = if (*ar).linedefined == 0 {
            cstr!("main")
        } else {
            cstr!("Lua")
        };
    }
    luaO_chunkid(((*ar).short_src).as_mut_ptr(), (*ar).source, LUA_IDSIZE);
}

unsafe extern "C" fn collectvalidlines(L: *mut lua_State, f: *mut Closure) {
    if noLuaClosure(f) {
        setnilvalue((*L).top);
        api_incr_top(L);
    } else {
        let mut v = TValue {
            value_: Value {
                gc: ptr::null_mut(),
            },
            tt_: 0,
        };
        let lineinfo = (*(*f).l.p).lineinfo;
        let t = luaH_new(L); /* new table to store active lines */
        sethvalue(L, (*L).top, t); /* push it on stack */
        api_incr_top(L);
        setbvalue(&mut v, true); /* boolean 'true' to be the value of all indices */
        for i in 0..(*(*f).l.p).sizelineinfo {
            /* for all lines with code */
            /* table[line] = true */
            luaH_setint(L, t, *lineinfo.offset(i as isize) as lua_Integer, &mut v);
        }
    };
}

unsafe extern "C" fn getfuncname(
    L: *mut lua_State,
    ci: *mut CallInfo,
    name: *mut *const c_char,
) -> *const c_char {
    if ci.is_null() {
        /* no 'ci'? no info */
        return ptr::null();
    } else if (*ci).callstatus & CIST_FIN != 0 {
        /* is this a finalizer? */
        *name = cstr!("__gc");
        return cstr!("metamethod"); /* report it as such */
    } else if (*ci).callstatus & CIST_TAIL == 0 && isLua((*ci).previous) {
        /* calling function is a known Lua function? */
        return funcnamefromcode(L, (*ci).previous, name);
    } else {
        /* no way to find a name */
        return ptr::null();
    };
}

unsafe extern "C" fn auxgetinfo(
    L: *mut lua_State,
    mut what: *const c_char,
    ar: *mut lua_Debug,
    f: *mut Closure,
    ci: *mut CallInfo,
) -> c_int {
    let mut status = 1;
    while *what != 0 {
        match *what as u8 {
            b'S' => {
                funcinfo(ar, f);
            }
            b'l' => {
                (*ar).currentline = if !ci.is_null() && isLua(ci) {
                    currentline(ci)
                } else {
                    -1
                };
            }
            b'u' => {
                (*ar).nups = if f.is_null() { 0 } else { (*f).c.nupvalues };
                if noLuaClosure(f) {
                    (*ar).isvararg = 1;
                    (*ar).nparams = 0;
                } else {
                    (*ar).isvararg = (*(*f).l.p).is_vararg as c_char;
                    (*ar).nparams = (*(*f).l.p).numparams;
                }
            }
            b't' => {
                (*ar).istailcall = if !ci.is_null() {
                    ((*ci).callstatus & CIST_TAIL) as c_char
                } else {
                    0
                };
            }
            b'n' => {
                (*ar).namewhat = getfuncname(L, ci, &mut (*ar).name);
                if (*ar).namewhat.is_null() {
                    (*ar).namewhat = cstr!(""); /* not found */
                    (*ar).name = ptr::null();
                }
            }
            b'L' | b'f' => { /* handled by lua_getinfo */ }
            _ => {
                status = 0; // invalid option
            }
        }
        what = what.offset(1);
    }
    return status;
}

#[no_mangle]
pub unsafe extern "C" fn lua_getinfo(
    L: *mut lua_State,
    mut what: *const c_char,
    ar: *mut lua_Debug,
) -> c_int {
    let ci;
    let func;
    swapextra(L);
    if *what == b'>' as c_char {
        ci = ptr::null_mut();
        func = ((*L).top).sub(1);
        assert!(ttisfunction(func), "function expected");
        what = what.add(1); // skip the '>'
        (*L).top = ((*L).top).sub(1); // pop function
    } else {
        ci = (*ar).i_ci;
        func = (*ci).func;
        debug_assert!(ttisfunction((*ci).func));
    }
    let cl = if ttisclosure(func) {
        clvalue(func)
    } else {
        ptr::null_mut()
    };
    let status = auxgetinfo(L, what, ar, cl, ci);
    if !strchr(what, b'f' as i32).is_null() {
        setobj(L, (*L).top, func);
        api_incr_top(L);
    }
    swapextra(L); // correct before option 'L', which can raise a mem. error
    if !strchr(what, 'L' as i32).is_null() {
        collectvalidlines(L, cl);
    }
    return status;
}

/*
** Symbolic Execution
*/

/*
** find a "name" for the RK value 'c'
*/
unsafe extern "C" fn kname(p: *mut Proto, pc: c_int, c: c_int, name: *mut *const c_char) {
    if ISK(c as c_uint) {
        // is 'c' a constant?
        let kvalue = &mut *((*p).k).add(INDEXK(c as c_uint) as usize) as *mut TValue;
        if ttisstring(kvalue) {
            // literal constant?
            *name = svalue(kvalue); // it is its own name
            return;
        }
    } else {
        // 'c' is a register
        let what = getobjname(p, pc, c, name); // search for 'c'
        if !what.is_null() && *what == b'c' as c_char {
            // found a constant name?
            return; // 'name' already filled
        }
    }
    // no reasonable name found
    *name = cstr!("?");
}

unsafe extern "C" fn filterpc(pc: c_int, jmptarget: c_int) -> c_int {
    if pc < jmptarget {
        // is code conditional (inside a jump)?
        return -1; // cannot know who sets that register
    } else {
        return pc; // current position sets that register
    };
}

/*
** try to find last instruction before 'lastpc' that modified register 'reg'
*/
unsafe extern "C" fn findsetreg(p: *mut Proto, lastpc: c_int, reg: c_int) -> c_int {
    let mut setreg = -1; // keep last instruction that changed 'reg'
    let mut jmptarget = 0; // any code before this address is conditional
    for pc in 0..lastpc {
        let i: Instruction = *((*p).code).offset(pc as isize);
        let op: OpCode = GET_OPCODE(i);
        let a = GETARG_A(i);
        match op {
            OP_LOADNIL => {
                let b = GETARG_B(i);
                if a <= reg && reg <= a + b {
                    // set registers from 'a' to 'a+b'
                    setreg = filterpc(pc, jmptarget);
                }
            }
            OP_TFORCALL => {
                if reg >= a + 2 {
                    // affect all regs above its base
                    setreg = filterpc(pc, jmptarget);
                }
            }
            OP_CALL | OP_TAILCALL => {
                if reg >= a {
                    // affect all registers above base
                    setreg = filterpc(pc, jmptarget);
                }
            }
            OP_JMP => {
                let b = GETARG_sBx(i);
                let dest = pc + 1 + b;
                // jump is forward and do not skip 'lastpc'?
                if pc < dest && dest <= lastpc {
                    if dest > jmptarget {
                        jmptarget = dest; // update 'jmptarget'
                    }
                }
            }
            _ => {
                if testAMode(op) && reg == a {
                    // any instruction that set A
                    setreg = filterpc(pc, jmptarget);
                }
            }
        }
    }
    return setreg;
}

unsafe extern "C" fn getobjname(
    p: *mut Proto,
    lastpc: c_int,
    reg: c_int,
    name: *mut *const c_char,
) -> *const c_char {
    *name = luaF_getlocalname(p, reg + 1, lastpc);
    if !(*name).is_null() {
        // is a local?
        return cstr!("local");
    }
    // else try symbolic execution
    let pc = findsetreg(p, lastpc, reg);
    if pc != -1 {
        // could find instruction?
        let i: Instruction = *((*p).code).offset(pc as isize);
        let op: OpCode = GET_OPCODE(i);
        match op {
            OP_MOVE => {
                let b = GETARG_B(i); // move from 'b' to 'a'
                if b < GETARG_A(i) {
                    return getobjname(p, pc, b, name); // get name for 'b'
                }
            }
            OP_GETTABUP | OP_GETTABLE => {
                let k = GETARG_C(i); // key index
                let t = GETARG_B(i); // table index
                let vn = if op == OP_GETTABLE {
                    // name of indexed variable
                    luaF_getlocalname(p, t + 1, pc)
                } else {
                    upvalname(p, t)
                };
                kname(p, pc, k, name);
                return if !vn.is_null() && strcmp(vn, LUA_ENV) == 0 {
                    cstr!("global")
                } else {
                    cstr!("field")
                };
            }
            OP_GETUPVAL => {
                *name = upvalname(p, GETARG_B(i));
                return cstr!("upvalue");
            }
            OP_LOADK | OP_LOADKX => {
                let b = if op == OP_LOADK {
                    GETARG_Bx(i)
                } else {
                    GETARG_Ax(*(*p).code.add((pc + 1) as usize))
                };
                if ttisstring((*p).k.add(b as usize)) {
                    *name = svalue((*p).k.add(b as usize));
                    return cstr!("constant");
                }
            }
            OP_SELF => {
                let k = GETARG_C(i); // key index
                kname(p, pc, k, name);
                return cstr!("method");
            }
            _ => {}
        }
    }
    // could not find reasonable name
    return ptr::null();
}

/*
** Try to find a name for a function based on the code that called it.
** (Only works when function was called by a Lua function.)
** Returns what the name is (e.g., "for iterator", "method",
** "metamethod") and sets '*name' to point to the name.
*/
unsafe extern "C" fn funcnamefromcode(
    L: *mut lua_State,
    ci: *mut CallInfo,
    name: *mut *const c_char,
) -> *const c_char {
    let tm;
    let p = (*ci_func(ci)).p; // calling function
    let pc = currentpc(ci); // calling instruction index
    let i = *((*p).code).offset(pc); // calling instruction
    if (*ci).callstatus & CIST_HOOKED != 0 {
        // was it called inside a hook?
        *name = cstr!("?");
        return cstr!("hook");
    }
    match GET_OPCODE(i) {
        OP_CALL | OP_TAILCALL => return getobjname(p, pc as c_int, GETARG_A(i), name), // get function name
        OP_TFORCALL => {
            *name = cstr!("for iterator");
            return cstr!("for iterator");
        }
        /* other instructions can do calls through metamethods */
        OP_SELF | OP_GETTABUP | OP_GETTABLE => {
            tm = TM_INDEX;
        }
        OP_SETTABUP | OP_SETTABLE => {
            tm = TM_NEWINDEX;
        }
        OP_ADD | OP_SUB | OP_MUL | OP_MOD | OP_POW | OP_DIV | OP_IDIV | OP_BAND | OP_BOR
        | OP_BXOR | OP_SHL | OP_SHR => {
            let offset = GET_OPCODE(i) as i32 - OP_ADD as i32; // ORDER OP
            tm = (offset + TM_ADD as i32) as TMS; // ORDER TM
        }
        OP_UNM => {
            tm = TM_UNM;
        }
        OP_BNOT => {
            tm = TM_BNOT;
        }
        OP_LEN => {
            tm = TM_LEN;
        }
        OP_CONCAT => {
            tm = TM_CONCAT;
        }
        OP_EQ => {
            tm = TM_EQ;
        }
        OP_LT => {
            tm = TM_LT;
        }
        OP_LE => {
            tm = TM_LE;
        }
        _ => return ptr::null(), // cannot find a reasonable name
    }
    *name = getstr((*(*L).l_G).tmname[tm as usize]);
    return cstr!("metamethod");
}

/*
** The subtraction of two potentially unrelated pointers is
** not ISO C, but it should not crash a program; the subsequent
** checks are ISO C and ensure a correct result.
*/
unsafe extern "C" fn isinstack(ci: *mut CallInfo, o: *const TValue) -> bool {
    let i = o.offset_from((*ci).u.l.base);
    return 0 <= i
        && i < (*ci).top.offset_from((*ci).u.l.base)
        && (*ci).u.l.base.offset(i) as *const _ == o;
}

/*
** Checks whether value 'o' came from an upvalue. (That can only happen
** with instructions OP_GETTABUP/OP_SETTABUP, which operate directly on
** upvalues.)
*/
unsafe extern "C" fn getupvalname(
    ci: *mut CallInfo,
    o: *const TValue,
    name: *mut *const c_char,
) -> *const c_char {
    let c = ci_func(ci);
    for i in 0..(*c).nupvalues {
        if (**(*c).upvals.as_ptr().offset(i as isize)).v as *const _ == o {
            *name = upvalname((*c).p, i as c_int);
            return cstr!("upvalue");
        }
    }
    return ptr::null();
}

unsafe extern "C" fn varinfo(L: *mut lua_State, o: *const TValue) -> *const c_char {
    let mut name = ptr::null();
    let ci = (*L).ci;
    let mut kind = ptr::null();
    if isLua(ci) {
        kind = getupvalname(ci, o, &mut name); // check whether 'o' is an upvalue
        if kind.is_null() && isinstack(ci, o) {
            // no? try a register
            kind = getobjname(
                (*ci_func(ci)).p,
                currentpc(ci) as c_int,
                o.offset_from((*ci).u.l.base) as c_int,
                &mut name,
            );
        }
    }
    return if !kind.is_null() {
        luaO_pushfstring(L, cstr!(" (%s '%s')"), kind, name)
    } else {
        cstr!("")
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaG_typeerror(
    L: *mut lua_State,
    o: *const TValue,
    op: *const c_char,
) -> ! {
    let t = luaT_objtypename(L, o);
    luaG_runerror(L, cstr!("attempt to %s a %s value%s"), op, t, varinfo(L, o));
}

#[no_mangle]
pub unsafe extern "C" fn luaG_concaterror(
    L: *mut lua_State,
    mut p1: *const TValue,
    p2: *const TValue,
) -> ! {
    if ttisstring(p1) || ttisnumber(p1) {
        p1 = p2;
    }
    luaG_typeerror(L, p1, cstr!("concatenate"));
}

#[no_mangle]
pub unsafe extern "C" fn luaG_opinterror(
    L: *mut lua_State,
    p1: *const TValue,
    mut p2: *const TValue,
    msg: *const c_char,
) -> ! {
    let mut temp = 0.;
    if tonumber(p1, &mut temp) == 0 {
        // first operand is wrong?
        p2 = p1; // now second is wrong
    }
    luaG_typeerror(L, p2, msg);
}

/*
** Error when both values are convertible to numbers, but not to integers
*/
#[no_mangle]
pub unsafe extern "C" fn luaG_tointerror(
    L: *mut lua_State,
    p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut temp = 0;
    if tointeger(p1, &mut temp) == 0 {
        p2 = p1;
    }
    luaG_runerror(
        L,
        cstr!("number%s has no integer representation"),
        varinfo(L, p2),
    );
}

#[no_mangle]
pub unsafe extern "C" fn luaG_ordererror(
    L: *mut lua_State,
    p1: *const TValue,
    p2: *const TValue,
) -> ! {
    let t1 = luaT_objtypename(L, p1);
    let t2 = luaT_objtypename(L, p2);
    if strcmp(t1, t2) == 0 {
        luaG_runerror(L, cstr!("attempt to compare two %s values"), t1);
    } else {
        luaG_runerror(L, cstr!("attempt to compare %s with %s"), t1, t2);
    };
}

/* add src:line information to 'msg' */
#[no_mangle]
pub unsafe extern "C" fn luaG_addinfo(
    L: *mut lua_State,
    msg: *const c_char,
    src: *mut TString,
    line: c_int,
) -> *const c_char {
    let mut buff = [0 as c_char; LUA_IDSIZE];
    if !src.is_null() {
        luaO_chunkid(buff.as_mut_ptr(), getstr(src), LUA_IDSIZE);
    } else {
        // no source available; use "?" instead
        buff[0] = b'?' as c_char;
    }
    return luaO_pushfstring(L, cstr!("%s:%d: %s"), buff.as_mut_ptr(), line, msg);
}

#[no_mangle]
pub unsafe extern "C" fn luaG_errormsg(L: *mut lua_State) -> ! {
    // is there an error handling function?
    if (*L).errfunc != 0 {
        let errfunc = restorestack(L, (*L).errfunc);
        setobj(L, (*L).top, (*L).top.sub(1)); // move argument
        setobj(L, (*L).top.sub(1), errfunc); // push function
        (*L).top = ((*L).top).add(1); // assume EXTRA_STACK
        luaD_callnoyield(L, ((*L).top).sub(2), 1); // call it
    }
    luaD_throw(L, LUA_ERRRUN);
}

#[no_mangle]
pub unsafe extern "C" fn luaG_runerror(L: *mut lua_State, fmt: *const c_char, mut args: ...) -> ! {
    let ci = (*L).ci;
    luaC_checkGC(L); // error message uses memory
    let msg = luaO_pushvfstring(L, fmt, args.as_va_list()); // format message
    if isLua(ci) {
        // if Lua function, add source:line information
        luaG_addinfo(L, msg, (*(*ci_func(ci)).p).source, currentline(ci));
    }
    luaG_errormsg(L);
}

#[no_mangle]
pub unsafe extern "C" fn luaG_traceexec(L: *mut lua_State) {
    let ci = (*L).ci;
    let mask = (*L).hookmask;
    (*L).hookcount -= 1;
    let counthook = (*L).hookcount == 0 && mask & LUA_MASKCOUNT != 0;
    if counthook {
        // reset count
        (*L).hookcount = (*L).basehookcount;
    } else if mask & LUA_MASKLINE == 0 {
        return; // no line hook and count != 0; nothing to be done
    }
    if (*ci).callstatus & CIST_HOOKYIELD != 0 {
        // called hook last time?
        (*ci).callstatus &= !CIST_HOOKYIELD; // erase mark
        return; // do not call hook again (VM yielded, so it did not move)
    }
    if counthook {
        luaD_hook(L, LUA_HOOKCOUNT, -1); // call count hook
    }
    if mask & LUA_MASKLINE != 0 {
        let p = (*ci_func(ci)).p;
        let npc = pcRel((*ci).u.l.savedpc, p);
        let newline = getfuncline(p, npc);
        if
        // call linehook when enter a new function,
        npc == 0
            //  when jump back (loop), or when
            || (*ci).u.l.savedpc <= (*L).oldpc
            // enter a new line
            || newline != getfuncline(p, pcRel((*L).oldpc, p))
        {
            luaD_hook(L, LUA_HOOKLINE, newline); // call line hook
        }
    }
    (*L).oldpc = (*ci).u.l.savedpc;
    if (*L).status == LUA_YIELD as u8 {
        // did hook yield?
        if counthook {
            (*L).hookcount = 1; // undo decrement to zero
        }
        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).sub(1); // undo increment (resume will increment it again)
        (*ci).callstatus |= CIST_HOOKYIELD; // mark that it yielded
        (*ci).func = ((*L).top).sub(1); // protect stack below results
        luaD_throw(L, LUA_YIELD);
    }
}
