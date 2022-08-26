/*
** Lexical Analyzer
*/

use libc::c_char;

use crate::lstate::lua_State;

pub const LUA_ENV: *const c_char = cstr!("_ENV");

extern "C" {
    pub fn luaX_init(L: *mut lua_State);
}
