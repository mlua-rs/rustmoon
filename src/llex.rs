/*
** Lexical Analyzer
*/

use crate::lstate::lua_State;

extern "C" {
    pub fn luaX_init(L: *mut lua_State);
}
