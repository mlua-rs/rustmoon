use std::mem::size_of;

use libc::{c_char, c_int, c_uint, c_void, size_t};

use crate::lfunc::UpVal;
use crate::lgc::isdead;
use crate::llimits::{lu_byte, Instruction, L_Umaxalign};
use crate::lstate::lua_State;
use crate::types::{
    lua_CFunction, lua_Integer, lua_Number, LUA_NUMTAGS, LUA_TFUNCTION, LUA_TNIL, LUA_TSTRING,
};

/*
** Extra tags for non-values
*/
pub const LUA_TPROTO: c_int = LUA_NUMTAGS as c_int; /* function prototypes */
pub const LUA_TDEADKEY: c_int = LUA_NUMTAGS as c_int + 1; /* removed keys in tables */

/*
** number of all possible tags (including LUA_TNONE but excluding DEADKEY)
*/
pub const LUA_TOTALTAGS: usize = LUA_TPROTO as usize + 2;

/*
** tags for Tagged Values have the following use of bits:
** bits 0-3: actual tag (a LUA_T* value)
** bits 4-5: variant bits
** bit 6: whether value is collectable
*/

/*
** LUA_TFUNCTION variants:
** 0 - Lua function
** 1 - light C function
** 2 - regular C function (closure)
*/

/* Variant tags for functions */
pub const LUA_TLCL: c_int = LUA_TFUNCTION | (0 << 4); /* Lua closure */
pub const LUA_TLCF: c_int = LUA_TFUNCTION | (1 << 4); /* light C function */
pub const LUA_TCCL: c_int = LUA_TFUNCTION | (2 << 4); /* C closure */

/* Variant tags for strings */
pub const LUA_TSHRSTR: c_int = LUA_TSTRING | (0 << 4); /* short strings */
pub const LUA_TLNGSTR: c_int = LUA_TSTRING | (1 << 4); /* long strings */

/* Bit mark for collectable types */
pub const BIT_ISCOLLECTABLE: c_int = 1 << 6;

/*
** Common type has only the common header
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCObject {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
}

/*
** Tagged Values. This is the basic representation of values in Lua,
** an actual value plus a tag with its type.
*/

/*
** Union of all Lua values
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub gc: *mut GCObject, /* collectable objects */
    pub p: *mut c_void,    /* light userdata */
    pub b: c_int,          /* booleans */
    pub f: lua_CFunction,  /* light C functions */
    pub i: lua_Integer,    /* integer numbers */
    pub n: lua_Number,     /* float numbers */
}

pub type TValue = lua_TValue;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_TValue {
    pub value_: Value,
    pub tt_: c_int,
}

/* tag with no variants (bits 0-3) */
pub const fn novariant(x: c_int) -> c_int {
    x & 0x0F
}

/* type tag of a TValue (bits 0-3 for tags + variant bits 4-5) */
#[inline(always)]
pub unsafe fn ttype(o: *mut TValue) -> c_int {
    (*o).tt_ & 0x3F
}

/* type tag of a TValue with no variants (bits 0-3) */
pub unsafe fn ttnov(o: *mut TValue) -> c_int {
    novariant((*o).tt_)
}

/* Macros to access values */
pub unsafe fn gcvalue(o: *mut TValue) -> *mut GCObject {
    debug_assert!(iscollectable(o));
    (*o).value_.gc
}

pub unsafe fn iscollectable(o: *mut TValue) -> bool {
    (*o).tt_ & BIT_ISCOLLECTABLE != 0
}

/* Macros for internal tests */

pub unsafe fn righttt(obj: *mut TValue) -> bool {
    ttype(obj) == (*gcvalue(obj)).tt as c_int
}

pub unsafe fn checkliveness(L: *mut lua_State, obj: *mut TValue) {
    debug_assert!(
        !iscollectable(obj) || (righttt(obj) && (L.is_null() || !isdead((*L).l_G, gcvalue(obj))))
    );
}

/* Macros to set values */

#[inline(always)]
pub unsafe fn setnilvalue(obj: *mut TValue) {
    (*obj).tt_ = LUA_TNIL;
}

#[inline(always)]
pub unsafe fn setobj(L: *mut lua_State, obj1: *mut TValue, obj2: *mut TValue) {
    *obj1 = *obj2;
    checkliveness(L, obj1);
}

/*
** ======================================================
** types and prototypes
** =======================================================
*/

pub type StkId = *mut TValue; /* index to stack elements */

/*
** Header for string value; string bytes follow the end of this structure
** (aligned according to 'UTString'; see next).
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub extra: lu_byte, /* reserved words for short strings; "has hash" for longs */
    pub shrlen: lu_byte, /* length for short strings */
    pub hash: c_uint,
    pub u: C2RustUnnamed_5,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub lnglen: size_t,      /* length for long strings */
    pub hnext: *mut TString, /* linked list for hash table */
}

/*
** Ensures that address after this type is always fully aligned.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString {
    pub dummy: L_Umaxalign,
    pub tsv: TString,
}

/*
** Get the actual string (array of bytes) from a 'TString'.
** (Access to 'extra' ensures that value is really a 'TString'.)
*/
pub unsafe fn getstr(ts: *mut TString) -> *const c_char {
    (ts as *const c_char).add(size_of::<UTString>())
}

/*
** Header for userdata; memory area follows the end of this structure
** (aligned according to 'UUdata'; see next).
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte, /* user value's tag */
    pub metatable: *mut Table,
    pub len: size_t,  /* number of bytes */
    pub user_: Value, /* user value */
}

/*
** Description of an upvalue for function prototypes
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}

/*
** Description of a local variable for function prototypes
** (used for debug information)
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: c_int,
    pub endpc: c_int,
}

/*
** Function Prototypes
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proto {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub numparams: lu_byte, /* number of fixed parameters */
    pub is_vararg: lu_byte,
    pub maxstacksize: lu_byte, /* number of registers needed by this function */
    pub sizeupvalues: c_int,   /* size of 'upvalues' */
    pub sizek: c_int,          /* size of 'k' */
    pub sizecode: c_int,
    pub sizelineinfo: c_int,
    pub sizep: c_int, /* size of 'p' */
    pub sizelocvars: c_int,
    pub linedefined: c_int,       /* debug information  */
    pub lastlinedefined: c_int,   /* debug information  */
    pub k: *mut TValue,           /* constants used by the function */
    pub code: *mut Instruction,   /* opcodes */
    pub p: *mut *mut Proto,       /* functions defined inside the function */
    pub lineinfo: *mut c_int,     /* map from opcodes to source lines (debug information) */
    pub locvars: *mut LocVar,     /* information about local variables (debug information) */
    pub upvalues: *mut Upvaldesc, /* upvalue information */
    pub cache: *mut LClosure,     /* last-created closure with this prototype */
    pub source: *mut TString,     /* used for debug information */
    pub gclist: *mut GCObject,
}

/*
** Closures
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1], /* list of upvalues */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1], /* list of upvalues */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
}

/*
** Tables
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub union TKey {
    pub nk: C2RustUnnamed_6,
    pub tvk: TValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub value_: Value,
    pub tt_: c_int,
    pub next: c_int, /* for chaining (offset for next node) */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub i_val: TValue,
    pub i_key: TKey,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub flags: lu_byte,     /* 1<<p means tagmethod(p) is not present */
    pub lsizenode: lu_byte, /* log2 of size of 'node' array */
    pub sizearray: c_uint,  /* size of 'array' array */
    pub array: *mut TValue, /* array part */
    pub node: *mut Node,
    pub lastfree: *mut Node, /* any free position is before this position */
    pub metatable: *mut Table,
    pub gclist: *mut GCObject,
}
