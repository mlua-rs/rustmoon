use libc::{c_int, c_uint, c_void, size_t};

use crate::llimits::lu_byte;
use crate::types::{lua_CFunction, lua_Integer, lua_Number};

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
