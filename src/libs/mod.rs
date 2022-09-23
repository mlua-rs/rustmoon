use std::os::raw::c_int;

use crate::lstate::lua_State;

pub use lcorolib::luaopen_coroutine;
pub use lmathlib::luaopen_math;
pub use lutf8lib::luaopen_utf8;

extern "C" {
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    pub fn luaopen_package(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    pub fn luaopen_io(L: *mut lua_State) -> c_int;
    pub fn luaopen_os(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_debug(L: *mut lua_State) -> c_int;
}

mod lcorolib;
mod lmathlib;
mod lutf8lib;
