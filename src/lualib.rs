/*
** Lua standard libraries
*/

use libc::c_char;

/* version suffix for environment variable names */
pub const LUA_VERSUFFIX: *const c_char = cstr!("_5_3");

pub use crate::linit::luaL_openlibs;
