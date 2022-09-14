/*
** Auxiliary functions for building Lua libraries
*/

use std::mem;
use std::ptr;

use libc::{c_char, c_int, c_void, size_t};

use crate::lapi::lua_createtable;
use crate::llimits::LUAL_BUFFERSIZE;
use crate::lstate::lua_State;
use crate::types::lua_Integer;
use crate::types::lua_Number;
use crate::types::LUA_VERSION_NUM;
use crate::types::{lua_CFunction, LUA_ERRERR};

/* extra error code for 'luaL_loadfilex' */
pub const LUA_ERRFILE: c_int = LUA_ERRERR + 1;

/* key, in the registry, for table of loaded modules */
pub const LUA_LOADED_TABLE: *const c_char = cstr!("_LOADED");

/* key, in the registry, for table of preloaded loaders */
pub const LUA_PRELOAD_TABLE: *const c_char = cstr!("_PRELOAD");

#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: lua_CFunction,
}

const LUAL_NUMSIZES: usize = mem::size_of::<lua_Integer>() * 16 + mem::size_of::<lua_Number>();

pub unsafe fn luaL_checkversion(L: *mut lua_State) {
    luaL_checkversion_(L, LUA_VERSION_NUM, LUAL_NUMSIZES)
}

/* predefined references */
pub const LUA_NOREF: c_int = -2;
pub const LUA_REFNIL: c_int = -1;

pub unsafe fn luaL_newlibtable(L: *mut lua_State, l: *const luaL_Reg) {
    let len = {
        let mut l = l;
        let mut i = 0;
        while !(*l).name.is_null() {
            i += 1;
            l = l.add(1);
        }
        i
    };
    lua_createtable(L, 0, len);
}

pub unsafe fn luaL_newlib(L: *mut lua_State, l: *const luaL_Reg) {
    luaL_checkversion(L);
    luaL_newlibtable(L, l);
    luaL_setfuncs(L, l, 0);
}

pub unsafe fn luaL_loadfile(L: *mut lua_State, filename: *const c_char) -> c_int {
    luaL_loadfilex(L, filename, ptr::null())
}

pub unsafe fn luaL_checkstring(L: *mut lua_State, n: c_int) -> *const c_char {
    luaL_checklstring(L, n, ptr::null_mut())
}

pub unsafe fn luaL_optstring(L: *mut lua_State, arg: c_int, def: *const c_char) -> *const c_char {
    luaL_optlstring(L, arg, def, ptr::null_mut())
}

extern "C" {
    pub fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t);
    pub fn luaL_getmetafield(L: *mut lua_State, obj: c_int, event: *const c_char) -> c_int;
    pub fn luaL_callmeta(L: *mut lua_State, obj: c_int, event: *const c_char) -> c_int;
    pub fn luaL_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) -> *const c_char;
    pub fn luaL_argerror(L: *mut lua_State, arg: c_int, extramsg: *const c_char) -> c_int;
    pub fn luaL_checklstring(L: *mut lua_State, arg: c_int, len: *mut size_t) -> *const c_char;
    pub fn luaL_optlstring(
        L: *mut lua_State,
        arg: c_int,
        def: *const c_char,
        len: *mut size_t,
    ) -> *const c_char;
    pub fn luaL_checknumber(L: *mut lua_State, arg: c_int) -> lua_Number;
    pub fn luaL_optnumber(L: *mut lua_State, arg: c_int, def: lua_Number) -> lua_Number;
    pub fn luaL_checkinteger(L: *mut lua_State, arg: c_int) -> lua_Integer;
    pub fn luaL_optinteger(L: *mut lua_State, arg: c_int, def: lua_Integer) -> lua_Integer;
    pub fn luaL_checkstack(L: *mut lua_State, space: c_int, msg: *const c_char);
    pub fn luaL_checktype(L: *mut lua_State, arg: c_int, t: c_int);
    pub fn luaL_checkany(L: *mut lua_State, arg: c_int);
    pub fn luaL_newmetatable(L: *mut lua_State, tname: *const c_char) -> c_int;
    pub fn luaL_setmetatable(L: *mut lua_State, tname: *const c_char);
    pub fn luaL_testudata(L: *mut lua_State, ud: c_int, tname: *const c_char) -> *mut c_void;
    pub fn luaL_checkudata(L: *mut lua_State, ud: c_int, tname: *const c_char) -> *mut c_void;
    pub fn luaL_where(L: *mut lua_State, level: c_int);
    pub fn luaL_error(L: *mut lua_State, fmt: *const c_char, args: ...) -> c_int;
    pub fn luaL_checkoption(
        L: *mut lua_State,
        arg: c_int,
        def: *const c_char,
        lst: *const *const c_char,
    ) -> c_int;
    pub fn luaL_fileresult(L: *mut lua_State, stat: c_int, fname: *const c_char) -> c_int;
    pub fn luaL_execresult(L: *mut lua_State, stat: c_int) -> c_int;
    pub fn luaL_ref(L: *mut lua_State, t: c_int) -> c_int;
    pub fn luaL_unref(L: *mut lua_State, t: c_int, r#ref: c_int);
    pub fn luaL_loadfilex(L: *mut lua_State, filename: *const c_char, mode: *const c_char)
        -> c_int;
    pub fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const c_char,
        size: size_t,
        name: *const c_char,
        mode: *const c_char,
    ) -> c_int;
    pub fn luaL_loadstring(L: *mut lua_State, s: *const c_char) -> c_int;
    pub fn luaL_newstate() -> *mut lua_State;
    pub fn luaL_len(L: *mut lua_State, idx: c_int) -> lua_Integer;
    pub fn luaL_gsub(
        L: *mut lua_State,
        s: *const c_char,
        p: *const c_char,
        r: *const c_char,
    ) -> *const c_char;
    pub fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: c_int);
    pub fn luaL_getsubtable(L: *mut lua_State, idx: c_int, fname: *const c_char) -> c_int;
    pub fn luaL_traceback(L: *mut lua_State, L1: *mut lua_State, msg: *const c_char, level: c_int);
    pub fn luaL_requiref(
        L: *mut lua_State,
        modname: *const c_char,
        openf: lua_CFunction,
        glb: c_int,
    );
}

/*
** Generic Buffer manipulation
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut c_char, /* buffer address */
    pub size: size_t,   /* buffer size */
    pub n: size_t,      /* number of characters in buffer */
    pub L: *mut lua_State,
    pub initb: [c_char; LUAL_BUFFERSIZE], /* initial buffer */
}

impl luaL_Buffer {
    pub const fn new() -> Self {
        luaL_Buffer {
            b: ptr::null_mut(),
            size: 0,
            n: 0,
            L: ptr::null_mut(),
            initb: [0; LUAL_BUFFERSIZE],
        }
    }
}

extern "C" {
    pub fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    pub fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut c_char;
    pub fn luaL_addlstring(B: *mut luaL_Buffer, s: *const c_char, l: size_t);
    pub fn luaL_addstring(B: *mut luaL_Buffer, s: *const c_char);
    pub fn luaL_addvalue(B: *mut luaL_Buffer);
    pub fn luaL_pushresult(B: *mut luaL_Buffer);
    pub fn luaL_pushresultsize(B: *mut luaL_Buffer, sz: size_t);
    pub fn luaL_buffinitsize(L: *mut lua_State, B: *mut luaL_Buffer, sz: size_t) -> *mut c_char;
}
