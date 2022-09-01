/*
** Lua API
*/

use libc::c_int;

use crate::lstate::lua_State;
use crate::types::{lua_Number, LUA_MULTRET};

pub(crate) unsafe fn api_incr_top(L: *mut lua_State) {
    (*L).top = (*L).top.add(1);
    debug_assert!((*L).top <= (*(*L).ci).top, "stack overflow");
}

pub(crate) unsafe fn adjustresults(L: *mut lua_State, nres: i32) {
    if nres == LUA_MULTRET && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top;
    }
}

pub(crate) unsafe fn api_checknelems(L: *mut lua_State, n: i32) {
    debug_assert!(
        (n as isize) < (*L).top.offset_from((*(*L).ci).func),
        "not enough elements in the stack"
    );
}

pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -n - 1)
}

extern "C" {
    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
}
