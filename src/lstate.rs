use libc::{c_int, c_short, c_uint, c_ushort, c_void, ptrdiff_t};

use crate::ldo::lua_longjmp;
use crate::lfunc::UpVal;
use crate::llimits::{l_mem, lu_byte, lu_mem, sig_atomic_t, Instruction, STRCACHE_M, STRCACHE_N};
use crate::lobject::{
    CClosure, Closure, GCObject, LClosure, Proto, StkId, TString, TValue, Table, Udata, LUA_TCCL,
    LUA_TLCL, LUA_TPROTO,
};
use crate::ltm::TM_N;
use crate::types::{
    lua_Alloc, lua_CFunction, lua_Hook, lua_KContext, lua_KFunction, lua_Number, LUA_NUMTAGS,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub hash: *mut *mut TString,
    pub nuse: c_int,
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

/* macros to convert a GCObject into a specific value */

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
pub unsafe fn gco2p(o: *mut GCObject) -> *mut Proto {
    debug_assert!((*o).tt == LUA_TPROTO as lu_byte);
    &mut (*(o as *mut GCUnion)).p
}
