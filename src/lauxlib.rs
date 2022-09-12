/*
** Auxiliary functions for building Lua libraries
*/

use libc::{c_char, c_int};

use crate::{lstate::lua_State, types::lua_CFunction};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: lua_CFunction,
}

extern "C" {
    pub fn luaL_requiref(
        L: *mut lua_State,
        modname: *const c_char,
        openf: lua_CFunction,
        glb: c_int,
    );
    pub fn luaL_newstate() -> *mut lua_State;
    pub fn luaL_loadstring(L: *mut lua_State, s: *const c_char) -> c_int;
    pub fn luaL_loadfilex(L: *mut lua_State, filename: *const c_char, mode: *const c_char)
        -> c_int;
}
