/*
** Global State
*/

use libc::{c_int, c_short, c_uint, c_ushort, c_void, ptrdiff_t};

use crate::ldo::lua_longjmp;
use crate::lfunc::UpVal;
use crate::llimits::{l_mem, lu_byte, lu_mem, sig_atomic_t, Instruction, STRCACHE_M, STRCACHE_N};
use crate::lobject::{
    novariant, CClosure, Closure, GCObject, LClosure, Proto, StkId, TString, TValue, Table, Udata,
    LUA_TCCL, LUA_TLCL, LUA_TPROTO,
};
use crate::ltm::TM_N;
use crate::types::{
    lua_Alloc, lua_CFunction, lua_Hook, lua_KContext, lua_KFunction, lua_Number, LUA_NUMTAGS,
    LUA_TFUNCTION, LUA_TSTRING, LUA_TTABLE, LUA_TTHREAD, LUA_TUSERDATA,
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

// #define BASIC_STACK_SIZE        (2*LUA_MINSTACK)

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
// #define setoah(st,v)	((st) = ((st) & ~CIST_OAH) | (v))
// #define getoah(st)	((st) & CIST_OAH)

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
        &mut (*($v as *mut crate::lstate::GCUnion)).gc as *mut GCObject
    }};
}
