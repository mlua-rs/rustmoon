/*
** Initialization of libraries
*/

use crate::lapi::lua_pop;
use crate::lauxlib::{luaL_Reg, luaL_requiref};
use crate::libs::{
    luaopen_base, luaopen_coroutine, luaopen_debug, luaopen_io, luaopen_math, luaopen_os,
    luaopen_package, luaopen_string, luaopen_table, luaopen_utf8,
};
use crate::lstate::lua_State;

/*
** If you embed Lua in your program and need to open the standard
** libraries, call luaL_openlibs in your program. If you need a
** different set of libraries, copy this file to your project and edit
** it to suit your needs.
**
** You can also *preload* libraries, so that a later 'require' can
** open the library, which is already linked to the application.
** For that, do the following code:
**
**  luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE);
**  lua_pushcfunction(L, luaopen_modname);
**  lua_setfield(L, -2, modname);
**  lua_pop(L, 1);  // remove PRELOAD table
*/

/*
** these libs are loaded by lua.c and are readily available to any Lua
** program
*/
const loadedlibs: [luaL_Reg; 10] = [
    luaL_Reg {
        name: cstr!("_G"),
        func: Some(luaopen_base),
    },
    luaL_Reg {
        name: cstr!("package"), // LUA_LOADLIBNAME
        func: Some(luaopen_package),
    },
    luaL_Reg {
        name: cstr!("coroutine"), // LUA_COLIBNAME
        func: Some(luaopen_coroutine),
    },
    luaL_Reg {
        name: cstr!("table"), // LUA_TABLIBNAME
        func: Some(luaopen_table),
    },
    luaL_Reg {
        name: cstr!("io"), // LUA_IOLIBNAME
        func: Some(luaopen_io),
    },
    luaL_Reg {
        name: cstr!("os"), // LUA_OSLIBNAME
        func: Some(luaopen_os),
    },
    luaL_Reg {
        name: cstr!("string"), // LUA_STRLIBNAME
        func: Some(luaopen_string),
    },
    luaL_Reg {
        name: cstr!("math"), // LUA_MATHLIBNAME
        func: Some(luaopen_math),
    },
    luaL_Reg {
        name: cstr!("utf8"), // LUA_UTF8LIBNAME
        func: Some(luaopen_utf8),
    },
    luaL_Reg {
        name: cstr!("debug"), // LUA_DBLIBNAME
        func: Some(luaopen_debug),
    },
];

#[no_mangle]
pub unsafe extern "C" fn luaL_openlibs(L: *mut lua_State) {
    /* "require" functions from 'loadedlibs' and set results to global table */
    for lib in &loadedlibs {
        luaL_requiref(L, lib.name, lib.func, 1);
        lua_pop(L, 1);
    }
}
