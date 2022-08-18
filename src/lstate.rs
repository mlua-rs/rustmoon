/*
** Global State
*/

use std::mem::{size_of, size_of_val};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

use libc::{c_char, c_int, c_short, c_uint, c_ushort, c_void, memcpy, ptrdiff_t};

use crate::lapi::{api_incr_top, lua_version};
use crate::ldo::{luaD_rawrunprotected, lua_longjmp};
use crate::lfunc::{luaF_close, UpVal};
use crate::lgc::{bitmask, luaC_checkGC, luaC_freeallobjects, luaC_white, GCSpause, WHITE0BIT};
use crate::llex::luaX_init;
use crate::llimits::{l_mem, lu_byte, lu_mem, sig_atomic_t, Instruction, STRCACHE_M, STRCACHE_N};
use crate::lmem::{luaM_free, luaM_freearray, luaM_new, luaM_newobject, luaM_newvector};
use crate::lobject::{
    luaO_nilobject_, novariant, sethvalue, setnilvalue, setthvalue, CClosure, Closure, GCObject,
    LClosure, Proto, StkId, TString, TValue, Table, Udata, Value, LUA_TCCL, LUA_TLCL, LUA_TPROTO,
};
use crate::lstring::{luaS_hash, luaS_init};
use crate::ltable::{luaH_new, luaH_resize, luaH_setint};
use crate::ltm::luaT_init;
use crate::ltm::TM_N;
use crate::types::{
    lua_Alloc, lua_CFunction, lua_Hook, lua_KContext, lua_KFunction, lua_Number, LUA_MINSTACK,
    LUA_NUMTAGS, LUA_OK, LUA_RIDX_GLOBALS, LUA_RIDX_LAST, LUA_RIDX_MAINTHREAD, LUA_TFUNCTION,
    LUA_TSTRING, LUA_TTABLE, LUA_TTHREAD, LUA_TUSERDATA,
};

/*
** Some notes about garbage-collected objects: All objects in Lua must
** be kept somehow accessible until being freed, so all objects always
** belong to one (and only one) of these lists, using field 'next' of
** the 'CommonHeader' for the link:
**
** 'allgc': all objects not marked for finalization;
** 'finobj': all objects marked for finalization;
** 'tobefnz': all objects ready to be finalized;
** 'fixedgc': all objects that are not to be collected (currently
** only small strings, such as reserved words).
**
** Moreover, there is another set of lists that control gray objects.
** These lists are linked by fields 'gclist'. (All objects that
** can become gray have such a field. The field is not the same
** in all objects, but it always has this name.)  Any gray object
** must belong to one of these lists, and all objects in these lists
** must be gray:
**
** 'gray': regular gray objects, still waiting to be visited.
** 'grayagain': objects that must be revisited at the atomic phase.
**   That includes
**   - black objects got in a write barrier;
**   - all kinds of weak tables during propagation phase;
**   - all threads.
** 'weak': tables with weak values to be cleared;
** 'ephemeron': ephemeron tables with white->white entries;
** 'allweak': tables with weak keys and/or weak values to be cleared.
** The last three lists are used only during the atomic phase.
*/

/* extra stack space to handle TM calls and some other extras */
pub const EXTRA_STACK: usize = 5;

pub const BASIC_STACK_SIZE: usize = 2 * LUA_MINSTACK;

/* kinds of Garbage Collection */
pub const KGC_NORMAL: c_int = 0;
pub const KGC_EMERGENCY: c_int = 1; /* gc was forced by an allocation failure */

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub hash: *mut *mut TString,
    pub nuse: c_int, /* number of elements */
    pub size: c_int,
}

/*
** Information about a call.
** When a thread yields, 'func' is adjusted to pretend that the
** top function has only the yielded values in its stack; in that
** case, the actual 'func' value is saved in field 'extra'.
** When a function calls another with a continuation, 'extra' keeps
** the function index so that, in case of errors, the continuation
** function can be called with the correct top.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: StkId,             /* function index in the stack */
    pub top: StkId,              /* top for this function */
    pub previous: *mut CallInfo, /* dynamic call link */
    pub next: *mut CallInfo,     /* dynamic call link */
    pub u: C2RustUnnamed_0,
    pub extra: ptrdiff_t,
    pub nresults: c_short, /* expected number of results from this function */
    pub callstatus: c_ushort,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub l: C2RustUnnamed_2, /* only for Lua functions */
    pub c: C2RustUnnamed_1, /* only for C functions */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub k: lua_KFunction, /* continuation in case of yields */
    pub old_errfunc: ptrdiff_t,
    pub ctx: lua_KContext, /* context info. in case of yields */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub base: StkId, /* base for this function */
    pub savedpc: *const Instruction,
}

/*
** Bits in CallInfo status
*/
pub const CIST_OAH: c_ushort = 1 << 0; /* original value of 'allowhook' */
pub const CIST_LUA: c_ushort = 1 << 1; /* call is running a Lua function */
pub const CIST_HOOKED: c_ushort = 1 << 2; /* call is running a debug hook */
pub const CIST_FRESH: c_ushort = 1 << 3; /* call is running on a fresh invocation of luaV_execute */
pub const CIST_YPCALL: c_ushort = 1 << 4; /* call is a yieldable protected call */
pub const CIST_TAIL: c_ushort = 1 << 5; /* call was tail called */
pub const CIST_HOOKYIELD: c_ushort = 1 << 6; /* last hook called yielded */
pub const CIST_LEQ: c_ushort = 1 << 7; /* using __lt for __le */
pub const CIST_FIN: c_ushort = 1 << 8; /* call is running a finalizer */

pub unsafe fn isLua(ci: *const CallInfo) -> bool {
    (*ci).callstatus & CIST_LUA != 0
}

/* assume that CIST_OAH has offset 0 and that 'v' is strictly 0/1 */
pub fn setoah(st: &mut c_ushort, v: c_ushort) {
    *st = ((*st) & !CIST_OAH) | v;
}

pub const fn getoah(st: c_ushort) -> c_ushort {
    st & CIST_OAH
}

/*
** 'global state', shared by all threads of this state
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_State {
    pub frealloc: lua_Alloc, /* function to reallocate memory */
    pub ud: *mut c_void,     /* auxiliary data to 'frealloc' */
    pub totalbytes: l_mem,   /* number of bytes currently allocated - GCdebt */
    pub GCdebt: l_mem,       /* bytes allocated not yet compensated by the collector */
    pub GCmemtrav: lu_mem,   /* memory traversed by the GC */
    pub GCestimate: lu_mem,  /* an estimate of the non-garbage memory in use */
    pub strt: stringtable,   /* hash table for strings */
    pub l_registry: TValue,
    pub seed: c_uint, /* randomized seed for hashes */
    pub currentwhite: lu_byte,
    pub gcstate: lu_byte,            /* state of garbage collector */
    pub gckind: lu_byte,             /* kind of GC running */
    pub gcrunning: lu_byte,          /* true if GC is running */
    pub allgc: *mut GCObject,        /* list of all collectable objects */
    pub sweepgc: *mut *mut GCObject, /* current position of sweep in list */
    pub finobj: *mut GCObject,       /* list of collectable objects with finalizers */
    pub gray: *mut GCObject,         /* list of gray objects */
    pub grayagain: *mut GCObject,    /* list of objects to be traversed atomically */
    pub weak: *mut GCObject,         /* list of tables with weak values */
    pub ephemeron: *mut GCObject,    /* list of ephemeron tables (weak keys) */
    pub allweak: *mut GCObject,      /* list of all-weak tables */
    pub tobefnz: *mut GCObject,      /* list of userdata to be GC */
    pub fixedgc: *mut GCObject,      /* list of objects not to be collected */
    pub twups: *mut lua_State,       /* list of threads with open upvalues */
    pub gcfinnum: c_uint,            /* number of finalizers to call in each GC step */
    pub gcpause: c_int,              /* size of pause between successive GCs */
    pub gcstepmul: c_int,            /* GC 'granularity' */
    pub panic: lua_CFunction,        /* to be called in unprotected errors */
    pub mainthread: *mut lua_State,
    pub version: *const lua_Number,    /* pointer to version number */
    pub memerrmsg: *mut TString,       /* memory-error message */
    pub tmname: [*mut TString; TM_N],  /* array with tag-method names */
    pub mt: [*mut Table; LUA_NUMTAGS], /* metatables for basic types */
    pub strcache: [[*mut TString; STRCACHE_M]; STRCACHE_N], /* cache for strings in API */
}

/*
** 'per thread' state
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_State {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nci: c_ushort, /* number of items in 'ci' list */
    pub status: lu_byte,
    pub top: StkId, /* first free slot in the stack */
    pub l_G: *mut global_State,
    pub ci: *mut CallInfo,         /* call info for current function */
    pub oldpc: *const Instruction, /* last pc traced */
    pub stack_last: StkId,         /* last free slot in the stack */
    pub stack: StkId,              /* stack base */
    pub openupval: *mut UpVal,     /* list of open upvalues in this stack */
    pub gclist: *mut GCObject,
    pub twups: *mut lua_State,      /* list of threads with open upvalues */
    pub errorJmp: *mut lua_longjmp, /* current error recover point */
    pub base_ci: CallInfo,          /* CallInfo for first level (C calling Lua) */
    pub hook: lua_Hook,
    pub errfunc: ptrdiff_t, /* current error handling function (stack index) */
    pub stacksize: c_int,
    pub basehookcount: c_int,
    pub hookcount: c_int,
    pub nny: c_ushort,     /* number of non-yieldable calls in stack */
    pub nCcalls: c_ushort, /* number of nested C calls */
    pub hookmask: sig_atomic_t,
    pub allowhook: lu_byte,
}

/*
** Union of all collectable objects (only for conversions)
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    pub gc: GCObject,
    pub ts: TString,
    pub u: Udata,
    pub cl: Closure,
    pub h: Table,
    pub p: Proto,
    pub th: lua_State,
}

/*
 * macros to convert a GCObject into a specific value
 */

#[inline(always)]
pub unsafe fn gco2ts(o: *mut GCObject) -> *mut TString {
    debug_assert!(novariant((*o).tt as c_int) == LUA_TSTRING);
    &mut (*(o as *mut GCUnion)).ts
}

#[inline(always)]
pub unsafe fn gco2u(o: *mut GCObject) -> *mut Udata {
    debug_assert!((*o).tt == LUA_TUSERDATA as lu_byte);
    &mut (*(o as *mut GCUnion)).u
}

#[inline(always)]
pub unsafe fn gco2lcl(o: *mut GCObject) -> *mut LClosure {
    debug_assert!((*o).tt == LUA_TLCL as lu_byte);
    &mut (*(o as *mut GCUnion)).cl.l
}

#[inline(always)]
pub unsafe fn gco2ccl(o: *mut GCObject) -> *mut CClosure {
    debug_assert!((*o).tt == LUA_TCCL as lu_byte);
    &mut (*(o as *mut GCUnion)).cl.c
}

#[inline(always)]
pub unsafe fn gco2cl(o: *mut GCObject) -> *mut Closure {
    debug_assert!(novariant((*o).tt as c_int) == LUA_TFUNCTION);
    &mut (*(o as *mut GCUnion)).cl
}

#[inline(always)]
pub unsafe fn gco2t(o: *mut GCObject) -> *mut Table {
    debug_assert!((*o).tt == LUA_TTABLE as lu_byte);
    &mut (*(o as *mut GCUnion)).h
}

#[inline(always)]
pub unsafe fn gco2p(o: *mut GCObject) -> *mut Proto {
    debug_assert!((*o).tt == LUA_TPROTO as lu_byte);
    &mut (*(o as *mut GCUnion)).p
}

#[inline(always)]
pub unsafe fn gco2th(o: *mut GCObject) -> *mut lua_State {
    debug_assert!((*o).tt == LUA_TTHREAD as lu_byte);
    &mut (*(o as *mut GCUnion)).th
}

/* macro to convert a Lua object into a GCObject */
macro_rules! obj2gco {
    ($v:expr) => {{
        debug_assert!(crate::lobject::novariant((*$v).tt as c_int) < crate::lobject::LUA_TDEADKEY);
        &mut (*($v as *mut crate::lstate::GCUnion)).gc as *mut crate::lobject::GCObject
    }};
}

/* actual number of total bytes allocated */
#[inline(always)]
pub unsafe fn gettotalbytes(g: *mut global_State) -> usize {
    ((*g).totalbytes + (*g).GCdebt) as usize
}

pub const LUAI_GCPAUSE: c_int = 200; /* 200% */

pub const LUAI_GCMUL: c_int = 200; /* GC runs 'twice the speed' of memory allocation */

/*
** a macro to help the creation of a unique random seed when a state is
** created; the seed is used to randomize hashes.
*/
pub fn luai_makeseed() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|n| n.as_secs() as u32)
        .unwrap_or_default()
}

/*
** thread state + extra space
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LX {
    pub extra_: [u8; size_of::<*mut u8>()],
    pub l: lua_State,
}

/*
** Main thread combines a thread state and the global state
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LG {
    pub l: LX,
    pub g: global_State,
}

unsafe fn fromstate(L: *mut lua_State) -> *mut u8 {
    (L as *mut u8).sub(size_of::<*mut u8>())
}

/*
** Compute an initial seed as random as possible. Rely on Address Space
** Layout Randomization (if present) to increase randomness..
*/
unsafe fn addbuff<T>(b: &mut [c_char], p: &mut usize, e: *const T) {
    let t = e as usize;
    memcpy(
        b.as_mut_ptr().add(*p) as _,
        &t as *const _ as *const _,
        size_of::<usize>(),
    );
    *p += size_of::<usize>();
}

unsafe extern "C" fn makeseed(L: *mut lua_State) -> c_uint {
    let mut buff = [0 as c_char; 4 * size_of::<usize>()];
    let h = luai_makeseed();
    let mut p = 0;
    addbuff(&mut buff, &mut p, L); /* heap variable */
    addbuff(&mut buff, &mut p, &h); /* local variable */
    addbuff(&mut buff, &mut p, &luaO_nilobject_); /* global variable */
    addbuff(&mut buff, &mut p, &lua_newstate); /* public function */
    debug_assert!(p == size_of_val(&buff));
    return luaS_hash(buff.as_mut_ptr(), p, h);
}

/*
** set GCdebt to a new value keeping the value (totalbytes + GCdebt)
** invariant (and avoiding underflows in 'totalbytes')
*/
#[no_mangle]
pub unsafe extern "C" fn luaE_setdebt(g: *mut global_State, debt: l_mem) {
    let tb = gettotalbytes(g);
    debug_assert!(tb > 0);
    (*g).totalbytes = tb as isize - debt;
    (*g).GCdebt = debt;
}

#[no_mangle]
pub unsafe extern "C" fn luaE_extendCI(mut L: *mut lua_State) -> *mut CallInfo {
    let mut ci = luaM_new::<CallInfo>(L);
    debug_assert!((*(*L).ci).next.is_null());
    (*(*L).ci).next = ci;
    (*ci).previous = (*L).ci;
    (*ci).next = ptr::null_mut();
    (*L).nci = (*L).nci.wrapping_add(1);
    return ci;
}

/*
** free all CallInfo structures not in use by a thread
*/
#[no_mangle]
pub unsafe extern "C" fn luaE_freeCI(L: *mut lua_State) {
    let mut ci = (*L).ci;
    let mut next = (*ci).next;
    (*ci).next = ptr::null_mut();
    while !next.is_null() {
        ci = next;
        next = (*ci).next;
        luaM_free(L, ci);
        (*L).nci = (*L).nci.wrapping_sub(1);
    }
}

/*
** free half of the CallInfo structures not in use by a thread
*/
#[no_mangle]
pub unsafe extern "C" fn luaE_shrinkCI(L: *mut lua_State) {
    let mut ci = (*L).ci;
    let mut next2; /* next's next */
    /* while there are two nexts */
    while !((*ci).next).is_null() && !(*(*ci).next).next.is_null() {
        next2 = (*(*ci).next).next;
        luaM_free(L, (*ci).next); /* free next */
        (*L).nci = (*L).nci.wrapping_sub(1);
        (*ci).next = next2; /* remove 'next' from the list */
        (*next2).previous = ci;
        ci = next2; /* keep next's next */
    }
}

unsafe extern "C" fn stack_init(L1: *mut lua_State, L: *mut lua_State) {
    /* initialize stack array */
    (*L1).stack = luaM_newvector::<TValue>(L, BASIC_STACK_SIZE);
    (*L1).stacksize = BASIC_STACK_SIZE as c_int;
    for i in 0..BASIC_STACK_SIZE {
        setnilvalue((*L1).stack.add(i)); /* erase new stack */
    }
    (*L1).top = (*L1).stack;
    (*L1).stack_last = ((*L1).stack).offset((*L1).stacksize as isize - EXTRA_STACK as isize);
    /* initialize first ci */
    let ci = &mut (*L1).base_ci;
    (*ci).previous = ptr::null_mut();
    (*ci).next = ptr::null_mut();
    (*ci).callstatus = 0;
    (*ci).func = (*L1).top;
    setnilvalue((*L1).top); /* 'function' entry for this 'ci' */
    (*L1).top = ((*L1).top).add(1);
    (*ci).top = ((*L1).top).add(LUA_MINSTACK);
    (*L1).ci = ci;
}

unsafe extern "C" fn freestack(mut L: *mut lua_State) {
    if ((*L).stack).is_null() {
        /* stack not completely built yet */
        return;
    }
    (*L).ci = &mut (*L).base_ci; /* free the entire 'ci' list */
    luaE_freeCI(L);
    luaM_freearray(L, (*L).stack, (*L).stacksize as usize); /* free stack array */
}

/*
** Create registry table and its predefined values
*/
unsafe extern "C" fn init_registry(L: *mut lua_State, g: *mut global_State) {
    let mut temp = TValue {
        value_: Value {
            gc: ptr::null_mut(),
        },
        tt_: 0,
    };
    /* create registry */
    let registry = luaH_new(L);
    sethvalue(L, &mut (*g).l_registry, registry);
    luaH_resize(L, registry, LUA_RIDX_LAST as c_uint, 0);
    /* registry[LUA_RIDX_MAINTHREAD] = L */
    setthvalue(L, &mut temp, L); /* temp = L */
    luaH_setint(L, registry, LUA_RIDX_MAINTHREAD, &mut temp);
    /* registry[LUA_RIDX_GLOBALS] = table of globals */
    sethvalue(L, &mut temp, luaH_new(L)); /* temp = new table (global table) */
    luaH_setint(L, registry, LUA_RIDX_GLOBALS, &mut temp);
}

/*
** open parts of the state that may cause memory-allocation errors.
** ('g->version' != NULL flags that the state was completely build)
*/
unsafe extern "C" fn f_luaopen(L: *mut lua_State, _ud: *mut c_void) {
    let g = (*L).l_G;
    stack_init(L, L); /* init stack */
    init_registry(L, g);
    luaS_init(L);
    luaT_init(L);
    luaX_init(L);
    (*g).gcrunning = 1; /* allow gc */
    (*g).version = lua_version(ptr::null_mut());
}

/*
** preinitialize a thread with consistent values without allocating
** any memory (to avoid errors)
*/
unsafe extern "C" fn preinit_thread(L: *mut lua_State, g: *mut global_State) {
    (*L).l_G = g;
    (*L).stack = ptr::null_mut();
    (*L).ci = ptr::null_mut();
    (*L).nci = 0;
    (*L).stacksize = 0;
    (*L).twups = L; /* thread has no upvalues */
    (*L).errorJmp = ptr::null_mut();
    (*L).nCcalls = 0;
    (*L).hook = None;
    (*L).hookmask = 0;
    (*L).basehookcount = 0;
    (*L).allowhook = 1;
    (*L).hookcount = (*L).basehookcount;
    (*L).openupval = ptr::null_mut();
    (*L).nny = 1;
    (*L).status = LUA_OK as lu_byte;
    (*L).errfunc = 0;
}

unsafe extern "C" fn close_state(L: *mut lua_State) {
    let g = (*L).l_G;
    luaF_close(L, (*L).stack); /* close all upvalues for this thread */
    luaC_freeallobjects(L); /* collect all objects */
    luaM_freearray(L, (*g).strt.hash, (*g).strt.size as usize);
    freestack(L);
    debug_assert!(gettotalbytes(g) == size_of::<LG>());
    /* free main block */
    ((*g).frealloc.expect("non-null function pointer"))(
        (*g).ud,
        fromstate(L) as *mut c_void,
        size_of::<LG>(),
        0,
    );
}

pub unsafe fn lua_getextraspace(L: *mut lua_State) -> *mut c_void {
    (L as *mut u8).sub(size_of::<*mut u8>()) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn lua_newthread(L: *mut lua_State) -> *mut lua_State {
    let mut g = (*L).l_G;
    luaC_checkGC(L);
    /* create new thread */
    let L1 = &mut (*luaM_newobject::<LX>(L, LUA_TTHREAD as u8)).l as *mut lua_State;
    (*L1).marked = luaC_white(g);
    (*L1).tt = LUA_TTHREAD as lu_byte;
    /* link it on list 'allgc' */
    (*L1).next = (*g).allgc;
    (*g).allgc = obj2gco!(L1);
    /* anchor it on L stack */
    setthvalue(L, (*L).top, L1);
    api_incr_top(L);
    preinit_thread(L1, g);
    (*L1).hookmask = (*L).hookmask;
    (*L1).basehookcount = (*L).basehookcount;
    (*L1).hook = (*L).hook;
    (*L1).hookcount = (*L1).basehookcount;
    /* initialize L1 extra space */
    memcpy(
        lua_getextraspace(L1),
        lua_getextraspace((*g).mainthread),
        size_of::<*mut u8>(),
    );
    /* init stack */
    stack_init(L1, L);
    return L1;
}

#[no_mangle]
pub unsafe extern "C" fn luaE_freethread(L: *mut lua_State, L1: *mut lua_State) {
    let l = fromstate(L1) as *mut LX;
    /* close all upvalues for this thread */
    luaF_close(L1, (*L1).stack);
    debug_assert!((*L1).openupval.is_null());
    freestack(L1);
    luaM_free(L, l);
}

#[no_mangle]
pub unsafe extern "C" fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State {
    // let mut i: libc::c_int = 0;
    // let mut L = 0 as *mut lua_State;
    // let mut g = 0 as *mut global_State;
    let l = (f.expect("non-null function pointer"))(
        ud,
        ptr::null_mut(),
        LUA_TTHREAD as usize,
        size_of::<LG>(),
    ) as *mut LG;
    if l.is_null() {
        return ptr::null_mut();
    }
    let mut L = &mut (*l).l.l as *mut lua_State;
    let g = &mut (*l).g;
    (*L).next = ptr::null_mut();
    (*L).tt = LUA_TTHREAD as lu_byte;
    (*g).currentwhite = bitmask(WHITE0BIT);
    (*L).marked = luaC_white(g);
    preinit_thread(L, g);
    (*g).frealloc = f;
    (*g).ud = ud;
    (*g).mainthread = L;
    (*g).seed = makeseed(L);
    (*g).gcrunning = 0; /* no GC while building state */
    (*g).GCestimate = 0;
    (*g).strt.nuse = 0;
    (*g).strt.size = 0;
    (*g).strt.hash = ptr::null_mut();
    setnilvalue(&mut (*g).l_registry);
    (*g).panic = None;
    (*g).version = ptr::null();
    (*g).gcstate = GCSpause as lu_byte;
    (*g).gckind = KGC_NORMAL as lu_byte;
    (*g).allgc = ptr::null_mut();
    (*g).finobj = ptr::null_mut();
    (*g).tobefnz = ptr::null_mut();
    (*g).fixedgc = ptr::null_mut();
    (*g).sweepgc = ptr::null_mut();
    (*g).gray = ptr::null_mut();
    (*g).grayagain = ptr::null_mut();
    (*g).weak = ptr::null_mut();
    (*g).ephemeron = ptr::null_mut();
    (*g).allweak = ptr::null_mut();
    (*g).twups = ptr::null_mut();
    (*g).totalbytes = size_of::<LG>() as isize;
    (*g).GCdebt = 0;
    (*g).gcfinnum = 0;
    (*g).gcpause = LUAI_GCPAUSE;
    (*g).gcstepmul = LUAI_GCMUL;
    for i in 0..LUA_NUMTAGS {
        (*g).mt[i] = ptr::null_mut();
    }
    if luaD_rawrunprotected(L, Some(f_luaopen), ptr::null_mut()) != LUA_OK {
        /* memory allocation error: free partial state */
        close_state(L);
        L = ptr::null_mut();
    }
    return L;
}

#[no_mangle]
pub unsafe extern "C" fn lua_close(mut L: *mut lua_State) {
    /* only the main thread can be closed */
    L = (*(*L).l_G).mainthread;
    close_state(L);
}
