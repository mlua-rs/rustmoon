/*
** Type definitions and some generic functions for Lua objects
*/

use std::ffi::VaList;
use std::mem::size_of;
use std::ptr;

use libc::{
    c_char, c_int, c_uint, c_ulong, c_void, memcpy, size_t, strchr, strlen, strpbrk, strtod,
};

use crate::ldebug::luaG_runerror;
use crate::ldo::{luaD_checkstack, luaD_inctop};
use crate::lfunc::UpVal;
use crate::lgc::isdead;
use crate::llimits::{lu_byte, Instruction, L_Umaxalign};
use crate::lstate::{gco2ccl, gco2cl, gco2lcl, gco2t, gco2th, gco2ts, gco2u, lua_State};
use crate::lstring::luaS_newlstr;
use crate::ltm::{luaT_trybinTM, TMS, TM_ADD};
use crate::lvm::{luaV_concat, luaV_div, luaV_mod, luaV_shiftl, tointeger, tonumber};
use crate::types::{
    lua_CFunction, lua_Integer, lua_Number, lua_Unsigned, LUA_NUMTAGS, LUA_OPADD, LUA_OPBAND,
    LUA_OPBNOT, LUA_OPBOR, LUA_OPBXOR, LUA_OPDIV, LUA_OPIDIV, LUA_OPMOD, LUA_OPMUL, LUA_OPPOW,
    LUA_OPSHL, LUA_OPSHR, LUA_OPSUB, LUA_OPUNM, LUA_TBOOLEAN, LUA_TFUNCTION, LUA_TLIGHTUSERDATA,
    LUA_TNIL, LUA_TNUMBER, LUA_TSTRING, LUA_TTABLE, LUA_TTHREAD, LUA_TUSERDATA,
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

pub unsafe fn getstr_c(ts: *const TString) -> *const c_char {
    (ts as *const c_char).add(size_of::<UTString>())
}

/* get the actual string (array of bytes) from a Lua value */
pub unsafe fn svalue(o: *const TValue) -> *mut c_char {
    getstr(tsvalue(o))
}

/* get string length from 'TString *s' */
pub unsafe fn tsslen(s: *const TString) -> usize {
    if (*s).tt as c_int == LUA_TSHRSTR {
        (*s).shrlen as usize
    } else {
        (*s).u.lnglen
    }
}

/* get string length from 'TValue *o' */
pub unsafe fn vslen(o: *const TValue) -> usize {
    tsslen(tsvalue(o))
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

pub unsafe fn isLfunction(o: *const TValue) -> bool {
    ttisLclosure(o)
}

pub unsafe fn getproto(o: *const TValue) -> *mut Proto {
    (*clLvalue(o)).p
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

/* copy a value into a key without messing up field 'next' */
pub unsafe fn setnodekey(L: *mut lua_State, key: *mut TKey, obj: *const TValue) {
    (*key).nk.value_ = (*obj).value_;
    (*key).nk.tt_ = (*obj).tt_;
    checkliveness(L, obj);
}

/*
** 'module' operation for hashing (size is always a power of 2)
*/
macro_rules! lmod {
    ($x:expr, $size:expr) => {{
        debug_assert!($size & ($size - 1) == 0);
        ($x & ($size - 1)) as c_int
    }};
}

pub unsafe fn sizenode(t: *const Table) -> usize {
    1 << (*t).lsizenode as usize
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

/* size of buffer for 'luaO_utf8esc' function */
pub const UTF8BUFFSZ: usize = 8;

/*
** converts an integer to a "floating point byte", represented as
** (eeeeexxx), where the real value is (1xxx) * 2^(eeeee - 1) if
** eeeee != 0 and (xxx) otherwise.
*/
#[no_mangle]
pub unsafe extern "C" fn luaO_int2fb(mut x: c_uint) -> c_int {
    /* exponent */
    let mut e = 0;
    if x < 8 {
        return x as c_int;
    }
    while x >= (8 << 4) {
        /* coarse steps */
        x = (x + 0xf) >> 4; /* x = ceil(x / 16) */
        e += 4;
    }
    while x >= (8 << 1) {
        /* fine steps */
        x = (x + 1) >> 1; /* x = ceil(x / 2) */
        e += 1;
    }
    return ((e + 1) << 3) | ((x as c_int) - 8);
}

/* converts back */
#[no_mangle]
pub unsafe extern "C" fn luaO_fb2int(x: c_int) -> c_int {
    return if x < 8 {
        x
    } else {
        ((x & 7) + 8) << ((x >> 3) - 1)
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaO_ceillog2(x: c_uint) -> c_int {
    (x as f32).log2().ceil() as c_int
}

unsafe extern "C" fn intarith(
    L: *mut lua_State,
    op: c_int,
    v1: lua_Integer,
    v2: lua_Integer,
) -> lua_Integer {
    match op {
        LUA_OPADD => v1.wrapping_add(v2),
        LUA_OPSUB => v1.wrapping_sub(v2),
        LUA_OPMUL => v1.wrapping_mul(v2),
        LUA_OPMOD => luaV_mod(L, v1, v2),
        LUA_OPIDIV => luaV_div(L, v1, v2),
        LUA_OPBAND => v1 & v2,
        LUA_OPBOR => v1 | v2,
        LUA_OPBXOR => v1 ^ v2,
        LUA_OPSHL => luaV_shiftl(v1, v2),
        LUA_OPSHR => luaV_shiftl(v1, -v2),
        LUA_OPUNM => (0 as lua_Integer).wrapping_sub(v1),
        LUA_OPBNOT => !0 ^ v1,
        _ => 0,
    }
}

unsafe extern "C" fn numarith(
    _L: *mut lua_State,
    op: c_int,
    v1: lua_Number,
    v2: lua_Number,
) -> lua_Number {
    match op {
        LUA_OPADD => v1 + v2,
        LUA_OPSUB => v1 - v2,
        LUA_OPMUL => v1 * v2,
        LUA_OPDIV => v1 / v2,
        LUA_OPPOW => v1.powf(v2),
        LUA_OPIDIV => (v1 / v2).floor(),
        LUA_OPUNM => -v1,
        LUA_OPMOD => {
            let mut m = v1 % v2;
            if m * v2 < 0. {
                m += v2;
            }
            m
        }
        _ => 0.,
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaO_arith(
    L: *mut lua_State,
    op: c_int,
    p1: *const TValue,
    p2: *const TValue,
    res: *mut TValue,
) {
    match op {
        LUA_OPBAND | LUA_OPBOR | LUA_OPBXOR | LUA_OPSHL | LUA_OPSHR | LUA_OPBNOT => {
            /* operate only on integers */
            let mut i1: lua_Integer = 0;
            let mut i2: lua_Integer = 0;
            if tointeger(p1, &mut i1) != 0 && tointeger(p2, &mut i2) != 0 {
                setivalue(res, intarith(L, op, i1, i2));
                return;
            }
        }
        LUA_OPDIV | LUA_OPPOW => {
            /* operate only on floats */
            let mut n1: lua_Number = 0.;
            let mut n2: lua_Number = 0.;
            if tonumber(p1, &mut n1) != 0 && tonumber(p2, &mut n2) != 0 {
                setfltvalue(res, numarith(L, op, n1, n2));
                return;
            }
        }
        _ => {
            /* other operations */
            let mut n1: lua_Number = 0.;
            let mut n2: lua_Number = 0.;
            if ttisinteger(p1) && ttisinteger(p2) {
                setivalue(res, intarith(L, op, ivalue(p1), ivalue(p2)));
                return;
            } else if tonumber(p1, &mut n1) != 0 && tonumber(p2, &mut n2) != 0 {
                setfltvalue(res, numarith(L, op, n1, n2));
                return;
            }
        }
    }
    /* could not perform raw operation; try metamethod */
    luaT_trybinTM(L, p1, p2, res, ((op - LUA_OPADD) + TM_ADD as i32) as TMS);
}

#[no_mangle]
pub unsafe extern "C" fn luaO_hexavalue(c: c_int) -> c_int {
    let chr = char::from_u32_unchecked(c as u32);
    if chr.is_ascii_digit() {
        return c - '0' as i32;
    } else {
        return (chr.to_ascii_lowercase() as i32 - 'a' as i32 + 10) as c_int;
    };
}

unsafe extern "C" fn isneg(s: *mut *const c_char) -> c_int {
    if **s == b'-' as c_char {
        *s = (*s).offset(1);
        return 1;
    } else {
        if **s == b'+' as c_char {
            *s = (*s).offset(1);
        }
    }
    return 0;
}

unsafe extern "C" fn l_str2dloc(
    s: *const c_char,
    result: *mut lua_Number,
    mode: c_int,
) -> *const c_char {
    let mut endptr = ptr::null_mut();
    *result = if mode == 'x' as i32 {
        /* try to convert */
        strtod(s, &mut endptr)
    } else {
        strtod(s, &mut endptr)
    };
    if endptr == s as *mut c_char {
        /* nothing recognized? */
        return ptr::null();
    }
    while char::from_u32_unchecked(*endptr as u32).is_ascii_whitespace() {
        /* skip trailing spaces */
        endptr = endptr.offset(1);
    }
    /* OK if no trailing characters */
    return if *endptr == b'\0' as c_char {
        endptr
    } else {
        ptr::null()
    };
}

/*
** Convert string 's' to a Lua number (put in 'result'). Return NULL
** on fail or the address of the ending '\0' on success.
** 'pmode' points to (and 'mode' contains) special things in the string:
** - 'x'/'X' means an hexadecimal numeral
** - 'n'/'N' means 'inf' or 'nan' (which should be rejected)
** - '.' just optimizes the search for the common case (nothing special)
*/
unsafe extern "C" fn l_str2d(s: *const c_char, result: *mut lua_Number) -> *const c_char {
    let pmode = strpbrk(s, cstr!(".xXnN"));
    let mode = if !pmode.is_null() {
        char::from_u32_unchecked(*pmode as u32).to_ascii_lowercase() as c_int
    } else {
        0
    };
    if mode == b'n' as i32 {
        /* reject 'inf' and 'nan' */
        return ptr::null();
    }
    /* try to convert */
    return l_str2dloc(s, result, mode);
}

pub const MAXBY10: lua_Unsigned = (lua_Integer::MAX / 10) as lua_Unsigned;
pub const MAXLASTD: lua_Unsigned = (lua_Integer::MAX % 10) as lua_Unsigned;

unsafe extern "C" fn l_str2int(mut s: *const c_char, result: *mut lua_Integer) -> *const c_char {
    let mut a: lua_Unsigned = 0;
    let mut empty = 1;
    /* skip initial spaces */
    while char::from_u32_unchecked(*s as u32).is_ascii_whitespace() {
        s = s.offset(1);
    }
    let neg = isneg(&mut s);
    if *s == b'0' as c_char && (*s.add(1) == b'x' as c_char || *s.add(1) == 'X' as c_char) {
        /* hex? */
        /* skip '0x' */
        s = s.add(2);
        while char::from_u32_unchecked(*s as u32).is_ascii_hexdigit() {
            a = a
                .wrapping_mul(16)
                .wrapping_add(luaO_hexavalue(*s as c_int) as lua_Unsigned);
            empty = 0;
            s = s.add(1);
        }
    } else {
        /* decimal */
        while char::from_u32_unchecked(*s as u32).is_ascii_digit() {
            let d = (*s - '0' as c_char) as lua_Unsigned;
            if a >= MAXBY10 && (a > MAXBY10 || d > (MAXLASTD + neg as lua_Unsigned)) {
                /* overflow? */
                /* do not accept it (as integer) */
                return ptr::null();
            }
            a = a.wrapping_mul(10).wrapping_add(d);
            empty = 0;
            s = s.add(1);
        }
    }
    while char::from_u32_unchecked(*s as u32).is_ascii_whitespace() {
        /* skip trailing spaces */
        s = s.add(1);
    }
    if empty != 0 || *s != b'\0' as c_char {
        /* something wrong in the numeral */
        return ptr::null();
    } else {
        *result = if neg != 0 {
            (0 as lua_Unsigned).wrapping_sub(a) as lua_Integer
        } else {
            a as lua_Integer
        };
        return s;
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaO_str2num(s: *const c_char, o: *mut TValue) -> size_t {
    let mut i = 0;
    /* try as an integer */
    let e = l_str2int(s, &mut i);
    if !e.is_null() {
        setivalue(o, i);
        /* success; return string size */
        return e.offset_from(s) as usize + 1;
    }
    /* else try as a float */
    let mut n = 0.;
    let e = l_str2d(s, &mut n);
    if !e.is_null() {
        setfltvalue(o, n);
        /* success; return string size */
        return e.offset_from(s) as usize + 1;
    }
    /* conversion failed */
    0
}

#[no_mangle]
pub unsafe extern "C" fn luaO_utf8esc(buff: *mut c_char, mut x: c_ulong) -> c_int {
    let mut n = 1; /* number of bytes put in buffer (backwards) */
    debug_assert!(x <= 0x10FFFF);
    if x < 0x80 {
        /* ascii? */
        *buff.add(UTF8BUFFSZ - 1) = x as c_char;
    } else {
        /* need continuation bytes */
        let mut mfb = 0x3f; /* maximum that fits in first byte */
        loop {
            /* add continuation bytes */
            *buff.add(UTF8BUFFSZ - n) = (0x80 | (x & 0x3f)) as c_char;
            n += 1;
            x >>= 6; /* remove added bits */
            mfb >>= 1; /* now there is one less bit available in first byte */
            if !(x > mfb) {
                break;
            }
            /* still needs continuation byte? */
        }
        /* add first byte */
        *buff.add(UTF8BUFFSZ - n) = ((!mfb << 1) | x) as c_char;
    }
    return n as c_int;
}

/* maximum length of the conversion of a number to a string */
pub const MAXNUMBER2STR: usize = 50;

/*
** Convert a number object to a string
*/
#[no_mangle]
pub unsafe extern "C" fn luaO_tostring(L: *mut lua_State, obj: StkId) {
    debug_assert!(ttisnumber(obj));
    let buff = if ttisinteger(obj) {
        format!("{}", ivalue(obj))
    } else {
        format!("{}", fltvalue(obj))
    };
    setsvalue(
        L,
        obj,
        luaS_newlstr(L, buff.as_ptr() as *const c_char, buff.len()),
    );
}

unsafe extern "C" fn pushstr(L: *mut lua_State, str: *const c_char, l: size_t) {
    setsvalue(L, (*L).top, luaS_newlstr(L, str, l));
    luaD_inctop(L);
}

/*
** this function handles only '%d', '%c', '%f', '%p', and '%s'
   conventional formats, plus Lua-specific '%I' and '%U'
*/
#[no_mangle]
pub unsafe extern "C" fn luaO_pushvfstring(
    L: *mut lua_State,
    mut fmt: *const c_char,
    mut argp: VaList,
) -> *const c_char {
    // let mut current_block: u64;
    let mut n = 0;
    loop {
        let e = strchr(fmt, '%' as i32);
        if e.is_null() {
            break;
        }
        pushstr(L, fmt, e.offset_from(fmt) as usize);
        match *e.add(1) as u8 {
            b's' => {
                /* zero-terminated string */
                let mut s: *const c_char = argp.arg();
                if s.is_null() {
                    s = cstr!("(null)");
                }
                pushstr(L, s, strlen(s));
            }
            b'c' => {
                /* an 'int' as a character */
                let buff = argp.arg::<i32>() as c_char;
                if let Some(buff) = char::from_u32(buff as u32)
                    .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
                {
                    pushstr(L, &(buff as c_char), 1);
                } else {
                    /* non-printable character; print its code */
                    luaO_pushfstring(L, cstr!("<\\%d>"), buff as u8 as c_uint);
                }
            }
            b'd' => {
                /* an 'int' */
                setivalue((*L).top, argp.arg::<i32>() as lua_Integer);
                luaD_inctop(L);
                luaO_tostring(L, (*L).top.sub(1));
            }
            b'I' => {
                /* a 'lua_Integer' */
                setivalue((*L).top, argp.arg());
                luaD_inctop(L);
                luaO_tostring(L, (*L).top.sub(1));
            }
            b'f' => {
                /* a 'lua_Number' */
                setfltvalue((*L).top, argp.arg());
                luaD_inctop(L);
                luaO_tostring(L, (*L).top.sub(1));
            }
            b'p' => {
                /* a pointer */
                let p = argp.arg::<*const c_void>();
                let pstr = format!("{:p}", p);
                pushstr(L, pstr.as_ptr() as *const c_char, pstr.len());
            }
            b'U' => {
                /* an 'int' as a UTF-8 sequence */
                let mut buff = [0 as c_char; UTF8BUFFSZ];
                let l = luaO_utf8esc(buff.as_mut_ptr(), argp.arg::<u64>());
                pushstr(
                    L,
                    buff.as_ptr().offset(UTF8BUFFSZ as isize - l as isize),
                    l as usize,
                );
            }
            b'%' => {
                pushstr(L, cstr!("%"), 1);
            }
            _ => {
                luaG_runerror(
                    L,
                    cstr!("invalid option '%%%c' to 'lua_pushfstring'"),
                    *e.offset(1) as i32,
                );
            }
        }
        n += 2;
        fmt = e.add(2);
    }
    luaD_checkstack(L, 1);
    pushstr(L, fmt, strlen(fmt));
    if n > 0 {
        luaV_concat(L, n + 1);
    }
    return svalue((*L).top.sub(1));
}

#[no_mangle]
pub unsafe extern "C" fn luaO_pushfstring(
    L: *mut lua_State,
    fmt: *const c_char,
    mut args: ...
) -> *const c_char {
    return luaO_pushvfstring(L, fmt, args.as_va_list());
}

#[no_mangle]
pub unsafe extern "C" fn luaO_chunkid(
    mut out: *mut c_char,
    source: *const c_char,
    mut bufflen: size_t,
) {
    let mut l = strlen(source);
    if *source == b'=' as _ {
        /* 'literal' source */
        if l <= bufflen {
            /* small enough? */
            memcpy(out as *mut c_void, source.add(1) as *const c_void, l);
        } else {
            /* truncate it */
            memcpy(
                out as *mut c_void,
                source.add(1) as *const c_void,
                bufflen - 1,
            );
            out = out.add(bufflen - 1);
            *out = b'\0' as _;
        }
    } else if *source == '@' as _ {
        /* file name */
        if l <= bufflen {
            /* small enough? */
            memcpy(out as *mut c_void, source.add(1) as *const c_void, l);
        } else {
            /* add '...' before rest of name */
            memcpy(out as *mut c_void, cstr!("...") as *const c_void, 3);
            out = out.add(3);
            bufflen -= 3;
            memcpy(
                out as *mut c_void,
                source.add(1 + l - bufflen) as *const c_void,
                bufflen,
            );
        }
    } else {
        /* string; format as [string "source"] */
        let nl = strchr(source, '\n' as i32); /* find first new line (if any) */
        /* add prefix */
        memcpy(out as *mut c_void, cstr!("[string \"") as *const c_void, 9);
        out = out.add(9);
        /* save space for prefix + suffix + '\0' */
        bufflen -= 9 + 3 + 2 + 1;
        if l < bufflen && nl.is_null() {
            /* small one-line source? */
            /* keep it */
            memcpy(out as *mut c_void, source as *const c_void, l);
            out = out.add(l);
        } else {
            if !nl.is_null() {
                /* stop at first newline */
                l = nl.offset_from(source) as usize;
            }
            if l > bufflen {
                l = bufflen;
            }
            memcpy(out as *mut c_void, source as *const c_void, l);
            out = out.add(l);
            memcpy(out as *mut c_void, cstr!("...") as *const c_void, 3);
            out = out.add(3);
        }
        memcpy(out as *mut c_void, cstr!("\"]") as *const c_void, 4);
    };
}
