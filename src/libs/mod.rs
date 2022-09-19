use libc::c_int;

use crate::lstate::lua_State;

pub use lmathlib::luaopen_math;

extern "C" {
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    pub fn luaopen_package(L: *mut lua_State) -> c_int;
    pub fn luaopen_coroutine(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    pub fn luaopen_io(L: *mut lua_State) -> c_int;
    pub fn luaopen_os(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_utf8(L: *mut lua_State) -> c_int;
    pub fn luaopen_debug(L: *mut lua_State) -> c_int;
}

mod lmathlib;
