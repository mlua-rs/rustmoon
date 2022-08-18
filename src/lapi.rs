/*
** Lua API
*/

use crate::lstate::lua_State;
use crate::types::lua_Number;

pub unsafe fn api_incr_top(L: *mut lua_State) {
    (*L).top = (*L).top.add(1);
    debug_assert!((*L).top <= (*(*L).ci).top, "stack overflow");
}

extern "C" {
    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;
}
