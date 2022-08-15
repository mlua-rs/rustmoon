/*
** Lua virtual machine
*/

use libc::c_int;

use crate::lobject::{fltvalue, ttisfloat, TValue};
use crate::types::lua_Number;

pub unsafe fn tonumber(o: *const TValue, n: *mut lua_Number) -> c_int {
    if ttisfloat(o) {
        *n = fltvalue(o);
        1
    } else {
        luaV_tonumber_(o, n)
    }
}

extern "C" {
    pub fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> c_int;
}
