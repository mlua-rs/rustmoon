use std::mem::size_of;
use std::ptr;

use libc::{c_char, c_int, c_uint, c_void, size_t};

use crate::lfunc::UpVal;
use crate::lgc::isdead;
use crate::llimits::{lu_byte, Instruction, L_Umaxalign};
use crate::lstate::{gco2ccl, gco2cl, gco2lcl, gco2t, gco2th, gco2ts, gco2u, lua_State};
use crate::types::{
    lua_CFunction, lua_Integer, lua_Number, LUA_NUMTAGS, LUA_TBOOLEAN, LUA_TFUNCTION,
    LUA_TLIGHTUSERDATA, LUA_TNIL, LUA_TNUMBER, LUA_TSTRING, LUA_TTABLE, LUA_TTHREAD, LUA_TUSERDATA,
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

/* Variant tags for numbers */
pub const LUA_TNUMFLT: c_int = LUA_TNUMBER | (0 << 4); /* float numbers */
pub const LUA_TNUMINT: c_int = LUA_TNUMBER | (1 << 4); /* integer numbers */

/* Bit mark for collectable types */
pub const BIT_ISCOLLECTABLE: c_int = 1 << 6;

/* mark a tag as collectable */
pub const fn ctb(t: c_int) -> c_int {
    t | BIT_ISCOLLECTABLE
}

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

/* macro defining a nil value */
// #define NILCONSTANT	{NULL}, LUA_TNIL

/* raw type tag of a TValue */
#[inline(always)]
pub unsafe fn rttype(o: *const TValue) -> c_int {
    (*o).tt_
}

/* tag with no variants (bits 0-3) */
pub const fn novariant(x: c_int) -> c_int {
    x & 0x0F
}

/* type tag of a TValue (bits 0-3 for tags + variant bits 4-5) */
#[inline(always)]
pub unsafe fn ttype(o: *const TValue) -> c_int {
    rttype(o) & 0x3F
}

/* type tag of a TValue with no variants (bits 0-3) */
#[inline(always)]
pub unsafe fn ttnov(o: *const TValue) -> c_int {
    novariant(rttype(o))
}

/*
 * Macros to test type
 */
pub unsafe fn checktag(o: *const TValue, t: c_int) -> bool {
    rttype(o) == t
}

pub unsafe fn checktype(o: *const TValue, t: c_int) -> bool {
    ttnov(o) == t
}

pub unsafe fn ttisnumber(o: *const TValue) -> bool {
    checktype(o, LUA_TNUMBER)
}

pub unsafe fn ttisfloat(o: *const TValue) -> bool {
    checktag(o, LUA_TNUMFLT)
}

pub unsafe fn ttisinteger(o: *const TValue) -> bool {
    checktag(o, LUA_TNUMINT)
}

pub unsafe fn ttisnil(o: *const TValue) -> bool {
    checktag(o, LUA_TNIL)
}

pub unsafe fn ttisboolean(o: *const TValue) -> bool {
    checktag(o, LUA_TBOOLEAN)
}

pub unsafe fn ttislightuserdata(o: *const TValue) -> bool {
    checktag(o, LUA_TLIGHTUSERDATA)
}

pub unsafe fn ttisstring(o: *const TValue) -> bool {
    checktype(o, LUA_TSTRING)
}

pub unsafe fn ttisshrstring(o: *const TValue) -> bool {
    checktag(o, ctb(LUA_TSHRSTR))
}

pub unsafe fn ttislngstring(o: *const TValue) -> bool {
    checktag(o, ctb(LUA_TLNGSTR))
}

pub unsafe fn ttistable(o: *const TValue) -> bool {
    checktag(o, ctb(LUA_TTABLE))
}

pub unsafe fn ttisfunction(o: *const TValue) -> bool {
    checktype(o, LUA_TFUNCTION)
}

pub unsafe fn ttisclosure(o: *const TValue) -> bool {
    (rttype(o) & 0x1F) == LUA_TFUNCTION
}

pub unsafe fn ttisCclosure(o: *const TValue) -> bool {
    checktag(o, ctb(LUA_TCCL))
}

pub unsafe fn ttisLclosure(o: *const TValue) -> bool {
    checktag(o, ctb(LUA_TLCL))
}

pub unsafe fn ttislcf(o: *const TValue) -> bool {
    checktag(o, LUA_TLCF)
}

pub unsafe fn ttisfulluserdata(o: *const TValue) -> bool {
    checktag(o, ctb(LUA_TUSERDATA))
}

pub unsafe fn ttisthread(o: *const TValue) -> bool {
    checktag(o, ctb(LUA_TTHREAD))
}

pub unsafe fn ttisdeadkey(o: *const TValue) -> bool {
    checktag(o, LUA_TDEADKEY)
}

/*
 * Macros to access values
 */

pub unsafe fn ivalue(o: *const TValue) -> lua_Integer {
    debug_assert!(ttisinteger(o));
    (*o).value_.i
}

pub unsafe fn fltvalue(o: *const TValue) -> lua_Number {
    debug_assert!(ttisfloat(o));
    (*o).value_.n
}

pub unsafe fn nvalue(o: *const TValue) -> lua_Number {
    debug_assert!(ttisnumber(o));
    if ttisinteger(o) {
        ivalue(o) as lua_Number
    } else {
        fltvalue(o)
    }
}

pub unsafe fn gcvalue(o: *const TValue) -> *mut GCObject {
    debug_assert!(iscollectable(o));
    (*o).value_.gc
}

pub unsafe fn pvalue(o: *const TValue) -> *mut c_void {
    debug_assert!(ttislightuserdata(o));
    (*o).value_.p
}

pub unsafe fn tsvalue(o: *const TValue) -> *mut TString {
    debug_assert!(ttisstring(o));
    gco2ts((*o).value_.gc)
}

pub unsafe fn uvalue(o: *const TValue) -> *mut Udata {
    debug_assert!(ttisfulluserdata(o));
    gco2u((*o).value_.gc)
}

pub unsafe fn clvalue(o: *const TValue) -> *mut Closure {
    debug_assert!(ttisclosure(o));
    gco2cl((*o).value_.gc)
}

pub unsafe fn clLvalue(o: *const TValue) -> *mut LClosure {
    debug_assert!(ttisLclosure(o));
    gco2lcl((*o).value_.gc)
}

pub unsafe fn clCvalue(o: *const TValue) -> *mut CClosure {
    debug_assert!(ttisCclosure(o));
    gco2ccl((*o).value_.gc)
}

pub unsafe fn fvalue(o: *const TValue) -> lua_CFunction {
    debug_assert!(ttislcf(o));
    (*o).value_.f
}

pub unsafe fn hvalue(o: *const TValue) -> *mut Table {
    debug_assert!(ttistable(o));
    gco2t((*o).value_.gc)
}

pub unsafe fn bvalue(o: *const TValue) -> bool {
    debug_assert!(ttisboolean(o));
    (*o).value_.b != 0
}

pub unsafe fn thvalue(o: *const TValue) -> *mut lua_State {
    debug_assert!(ttisthread(o));
    gco2th((*o).value_.gc)
}

/* a dead value may get the 'gc' field, but cannot access its contents */
pub unsafe fn deadvalue(o: *const TValue) -> *mut c_void {
    debug_assert!(ttisdeadkey(o));
    (*o).value_.gc as *mut c_void
}

pub unsafe fn l_isfalse(o: *const TValue) -> bool {
    ttisnil(o) || (ttisboolean(o) && !bvalue(o))
}

pub unsafe fn iscollectable(o: *const TValue) -> bool {
    (*o).tt_ & BIT_ISCOLLECTABLE != 0
}

/* Macros for internal tests */

pub unsafe fn righttt(obj: *const TValue) -> bool {
    ttype(obj) == (*gcvalue(obj)).tt as c_int
}

pub unsafe fn checkliveness(L: *mut lua_State, obj: *const TValue) {
    debug_assert!(
        !iscollectable(obj) || (righttt(obj) && (L.is_null() || !isdead((*L).l_G, gcvalue(obj))))
    );
}

/* Macros to set values */

#[inline(always)]
pub unsafe fn setfltvalue(obj: *mut TValue, x: lua_Number) {
    (*obj).value_.n = x;
    (*obj).tt_ = LUA_TNUMFLT;
}

#[inline(always)]
pub unsafe fn chgfltvalue(obj: *mut TValue, x: lua_Number) {
    debug_assert!(ttisfloat(obj));
    (*obj).value_.n = x;
}

#[inline(always)]
pub unsafe fn setivalue(obj: *mut TValue, x: lua_Integer) {
    (*obj).value_.i = x;
    (*obj).tt_ = LUA_TNUMINT;
}

#[inline(always)]
pub unsafe fn chgivalue(obj: *mut TValue, x: lua_Integer) {
    debug_assert!(ttisinteger(obj));
    (*obj).value_.i = x;
}

#[inline(always)]
pub unsafe fn setnilvalue(obj: *mut TValue) {
    (*obj).tt_ = LUA_TNIL;
}

pub unsafe fn setfvalue(obj: *mut TValue, x: lua_CFunction) {
    (*obj).value_.f = x;
    (*obj).tt_ = LUA_TLCF;
}

pub unsafe fn setpvalue(obj: *mut TValue, x: *mut c_void) {
    (*obj).value_.p = x;
    (*obj).tt_ = LUA_TLIGHTUSERDATA;
}

pub unsafe fn setbvalue(obj: *mut TValue, x: bool) {
    (*obj).value_.b = x as c_int;
    (*obj).tt_ = LUA_TBOOLEAN;
}

pub unsafe fn setgcovalue(obj: *mut TValue, x: *mut GCObject) {
    (*obj).value_.gc = x;
    (*obj).tt_ = ctb((*x).tt as c_int) as c_int;
}

pub unsafe fn setsvalue(L: *mut lua_State, obj: *mut TValue, x: *mut TString) {
    (*obj).value_.gc = obj2gco!(x);
    (*obj).tt_ = ctb((*x).tt as c_int) as c_int;
    checkliveness(L, obj);
}

pub unsafe fn setuvalue(L: *mut lua_State, obj: *mut TValue, x: *mut Udata) {
    (*obj).value_.gc = obj2gco!(x);
    (*obj).tt_ = ctb(LUA_TUSERDATA) as c_int;
    checkliveness(L, obj);
}

pub unsafe fn setthvalue(L: *mut lua_State, obj: *mut TValue, x: *mut lua_State) {
    (*obj).value_.gc = obj2gco!(x);
    (*obj).tt_ = ctb(LUA_TTHREAD) as c_int;
    checkliveness(L, obj);
}

pub unsafe fn setclLvalue(L: *mut lua_State, obj: *mut TValue, x: *mut LClosure) {
    (*obj).value_.gc = obj2gco!(x);
    (*obj).tt_ = ctb(LUA_TLCL) as c_int;
    checkliveness(L, obj);
}

pub unsafe fn setclCvalue(L: *mut lua_State, obj: *mut TValue, x: *mut CClosure) {
    (*obj).value_.gc = obj2gco!(x);
    (*obj).tt_ = ctb(LUA_TCCL) as c_int;
    checkliveness(L, obj);
}

pub unsafe fn sethvalue(L: *mut lua_State, obj: *mut TValue, x: *mut Table) {
    (*obj).value_.gc = obj2gco!(x);
    (*obj).tt_ = ctb(LUA_TTABLE) as c_int;
    checkliveness(L, obj);
}

pub unsafe fn setdeadvalue(obj: *mut TValue) {
    (*obj).tt_ = LUA_TDEADKEY;
}

#[inline(always)]
pub unsafe fn setobj(L: *mut lua_State, obj1: *mut TValue, obj2: *const TValue) {
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
pub unsafe fn getstr(ts: *mut TString) -> *mut c_char {
    (ts as *mut c_char).add(size_of::<UTString>())
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
** Ensures that address after this type is always fully aligned.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union UUdata {
    pub dummy: L_Umaxalign, /* ensures maximum alignment for 'local' udata */
    pub uv: Udata,
}

/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
pub unsafe fn getudatamem(u: *mut Udata) -> *mut c_void {
    (u as *mut c_char).add(size_of::<UUdata>()) as *mut c_void
}

pub unsafe fn setuservalue(L: *mut lua_State, u: *mut Udata, o: *const TValue) {
    (*u).user_ = (*o).value_;
    (*u).ttuv_ = rttype(o) as lu_byte;
    checkliveness(L, o);
}

pub unsafe fn getuservalue(L: *mut lua_State, u: *const Udata, o: *mut TValue) {
    (*o).value_ = (*u).user_;
    (*o).tt_ = (*u).ttuv_ as c_int;
    checkliveness(L, o);
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

/*
** 'module' operation for hashing (size is always a power of 2)
*/
pub const fn lmod(s: c_uint, size: c_int) -> c_uint {
    debug_assert!(size & (size - 1) == 0);
    s & (size - 1) as c_uint
}

/*
** (address of) a fixed nil value
*/
#[no_mangle]
pub static mut luaO_nilobject_: TValue = lua_TValue {
    value_: Value {
        gc: ptr::null_mut(),
    },
    tt_: LUA_TNIL,
};
