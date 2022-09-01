/*
** load precompiled Lua chunks
*/

use libc::c_char;

use crate::lobject::LClosure;
use crate::lstate::lua_State;
use crate::lzio::ZIO;

extern "C" {
    pub fn luaU_undump(L: *mut lua_State, Z: *mut ZIO, name: *const c_char) -> *mut LClosure;
}
