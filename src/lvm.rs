/*
** Lua virtual machine
*/

use std::ptr;

use libc::c_int;

use crate::lobject::{fltvalue, ivalue, ttisfloat, ttisinteger, TValue};
use crate::lstate::lua_State;
use crate::types::{lua_Integer, lua_Number};

pub unsafe fn tonumber(o: *const TValue, n: *mut lua_Number) -> c_int {
    if ttisfloat(o) {
        *n = fltvalue(o);
        1
    } else {
        luaV_tonumber_(o, n)
    }
}

pub unsafe fn tointeger(o: *const TValue, i: *mut lua_Integer) -> c_int {
    if ttisinteger(o) {
        *i = ivalue(o);
        1
    } else {
        luaV_tointeger(o, i, 0 /* LUA_FLOORN2I */)
    }
}

pub unsafe fn luaV_rawequalobj(t1: *const TValue, t2: *const TValue) -> c_int {
    luaV_equalobj(ptr::null_mut(), t1, t2)
}

extern "C" {
    pub fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> c_int;
    pub fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer, mode: c_int) -> c_int;
    pub fn luaV_div(L: *mut lua_State, m: lua_Integer, n: lua_Integer) -> lua_Integer;
    pub fn luaV_mod(L: *mut lua_State, m: lua_Integer, n: lua_Integer) -> lua_Integer;
    pub fn luaV_shiftl(x: lua_Integer, y: lua_Integer) -> lua_Integer;
    pub fn luaV_equalobj(L: *mut lua_State, t1: *const TValue, t2: *const TValue) -> c_int;
}
