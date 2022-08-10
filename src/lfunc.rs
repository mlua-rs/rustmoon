use libc::c_int;

use crate::llimits::lu_mem;
use crate::lobject::TValue;

/*
** Upvalues for Lua closures
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub v: *mut TValue,   /* points to stack or to its own value */
    pub refcount: lu_mem, /* reference counter */
    pub u: C2RustUnnamed_3,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub open: C2RustUnnamed_4, /* (when open) */
    pub value: TValue,         /* the value (when closed) */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub next: *mut UpVal, /* linked list */
    pub touched: c_int,   /* mark to avoid cycles with dead threads */
}
