/*
** Lua API
*/

use libc::c_int;

use crate::lstate::lua_State;
use crate::types::{lua_KContext, lua_KFunction, lua_Number, LUA_MULTRET};

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

pub unsafe fn lua_call(L: *mut lua_State, n: c_int, r: c_int) {
    lua_callk(L, n, r, 0, None)
}

pub unsafe fn lua_pcall(L: *mut lua_State, n: c_int, r: c_int, f: c_int) -> c_int {
    lua_pcallk(L, n, r, f, 0, None)
}

extern "C" {
    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
    pub fn lua_pcallk(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        errfunc: c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> c_int;
    pub fn lua_callk(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    );
}
