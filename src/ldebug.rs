use libc::c_char;

use crate::lobject::TValue;
use crate::lstate::lua_State;

extern "C" {
    pub fn luaG_concaterror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
    pub fn luaG_tointerror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
    pub fn luaG_opinterror(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        msg: *const c_char,
    ) -> !;
}
