/*
** load precompiled Lua chunks
*/

use libc::c_char;

use crate::lobject::LClosure;
use crate::lstate::lua_State;
use crate::lzio::ZIO;
use crate::types::{lua_Integer, lua_Number};

/* data to catch conversion errors */
pub const LUAC_DATA: &'static [u8] = b"\x19\x93\r\n\x1a\n";

pub const LUAC_INT: lua_Integer = 0x5678;
pub const LUAC_NUM: lua_Number = 370.5;

// TODO: Rewrite this
pub const LUAC_VERSION: u8 = 5 * 16 + 3;
pub const LUAC_FORMAT: u8 = 0; /* this is the official format */

extern "C" {
    pub fn luaU_undump(L: *mut lua_State, Z: *mut ZIO, name: *const c_char) -> *mut LClosure;
}
