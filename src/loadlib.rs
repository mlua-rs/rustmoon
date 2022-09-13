/*
** Dynamic library loader for Lua
*/

use std::ptr;

use libc::{c_char, c_int, c_void, fclose, fopen, getenv, strchr, strlen};

use crate::lapi::{
    lua_call, lua_createtable, lua_getfield, lua_insert, lua_isfunction, lua_isnil, lua_isstring,
    lua_newtable, lua_pop, lua_pushboolean, lua_pushcclosure, lua_pushcfunction, lua_pushfstring,
    lua_pushglobaltable, lua_pushlightuserdata, lua_pushlstring, lua_pushnil, lua_pushstring,
    lua_pushvalue, lua_rawgeti, lua_rawgetp, lua_rawseti, lua_rawsetp, lua_remove, lua_setfield,
    lua_setmetatable, lua_settop, lua_toboolean, lua_tostring, lua_touserdata, lua_upvalueindex,
};
use crate::lauxlib::{
    luaL_Buffer, luaL_Reg, luaL_addvalue, luaL_buffinit, luaL_checkstring, luaL_error,
    luaL_getsubtable, luaL_gsub, luaL_len, luaL_loadfile, luaL_newlib, luaL_optstring,
    luaL_pushresult, luaL_setfuncs, LUA_LOADED_TABLE, LUA_PRELOAD_TABLE,
};
use crate::lstate::lua_State;
use crate::lualib::LUA_VERSUFFIX;
use crate::types::{lua_CFunction, lua_Integer, LUA_OK, LUA_REGISTRYINDEX, LUA_TNIL, LUA_TTABLE};

/*
** unique key for table in the registry that keeps handles
** for all loaded C libraries
*/
static CLIBS: u8 = 0;

const LIB_FAIL: *const c_char = cstr!("absent");

const DLMSG: *const c_char = cstr!("dynamic libraries not supported");

// dummy implementation
unsafe fn lsys_unloadlib(_lib: *mut c_void) {}

// dummy implementation
unsafe fn lsys_load(L: *mut lua_State, _path: *const c_char, _seeglb: c_int) -> *mut c_void {
    lua_pushstring(L, DLMSG);
    return ptr::null_mut();
}

unsafe fn lsys_sym(L: *mut lua_State, _lib: *mut c_void, _sym: *const c_char) -> lua_CFunction {
    lua_pushstring(L, DLMSG);
    return None;
}

/*
**
** Set Paths
**
*/

unsafe extern "C" fn noenv(L: *mut lua_State) -> libc::c_int {
    lua_getfield(L, LUA_REGISTRYINDEX, cstr!("LUA_NOENV)"));
    let b = lua_toboolean(L, -1);
    lua_pop(L, 1); /* remove value */
    return b;
}

/*
** Set a path
*/
unsafe fn setpath(
    L: *mut lua_State,
    fieldname: *const c_char,
    envname: *const c_char,
    dft: *const c_char,
) {
    let nver = lua_pushfstring(L, cstr!("%s%s"), envname, LUA_VERSUFFIX);
    let mut path: *const c_char = getenv(nver); /* use versioned name */
    if path.is_null() {
        /* no environment variable? */
        path = getenv(envname); /* try unversioned name */
    }
    if path.is_null() || noenv(L) != 0 {
        /* no environment variable? */
        lua_pushstring(L, dft); /* use default */
    } else {
        /* replace ";;" by ";AUXMARK;" and then AUXMARK by default path */
        path = luaL_gsub(L, path, cstr!(";;"), cstr!(";\x01;"));
        luaL_gsub(L, path, cstr!("\x01"), dft);
        lua_remove(L, -2); /* remove result from 1st 'gsub' */
    }
    lua_setfield(L, -3, fieldname); /* package[fieldname] = path value */
    lua_pop(L, 1); /* pop versioned variable name */
}

/*
** return registry.CLIBS[path]
*/
unsafe fn checkclib(L: *mut lua_State, path: *const c_char) -> *mut c_void {
    lua_rawgetp(L, LUA_REGISTRYINDEX, &CLIBS as *const u8 as *const c_void);
    lua_getfield(L, -1, path);
    let plib = lua_touserdata(L, -1); /* plib = CLIBS[path] */
    lua_pop(L, 2); /* pop CLIBS table and 'plib' */
    return plib;
}

/*
** registry.CLIBS[path] = plib        -- for queries
** registry.CLIBS[#CLIBS + 1] = plib  -- also keep a list of all libraries
*/
unsafe fn addtoclib(L: *mut lua_State, path: *const c_char, plib: *mut c_void) {
    lua_rawgetp(L, LUA_REGISTRYINDEX, &CLIBS as *const u8 as *const c_void);
    lua_pushlightuserdata(L, plib);
    lua_pushvalue(L, -1);
    lua_setfield(L, -3, path); /* CLIBS[path] = plib */
    lua_rawseti(L, -2, luaL_len(L, -2) + 1); /* CLIBS[#CLIBS + 1] = plib */
    lua_pop(L, 1); /* pop CLIBS table */
}

/*
** __gc tag method for CLIBS table: calls 'lsys_unloadlib' for all lib
** handles in list CLIBS
*/
unsafe extern "C" fn gctm(L: *mut lua_State) -> c_int {
    let mut n = luaL_len(L, 1);
    while n >= 1 {
        /* for each handle, in reverse order */
        lua_rawgeti(L, 1, n); /* get handle CLIBS[n] */
        lsys_unloadlib(lua_touserdata(L, -1));
        lua_pop(L, 1); /* pop handle */
        n -= 1;
    }
    return 0;
}

/* error codes for 'lookforfunc' */
pub const ERRLIB: c_int = 1;
pub const ERRFUNC: c_int = 2;

/*
** Look for a C function named 'sym' in a dynamically loaded library
** 'path'.
** First, check whether the library is already loaded; if not, try
** to load it.
** Then, if 'sym' is '*', return true (as library has been loaded).
** Otherwise, look for symbol 'sym' in the library and push a
** C function with that symbol.
** Return 0 and 'true' or a function in the stack; in case of
** errors, return an error code and an error message in the stack.
*/
unsafe fn lookforfunc(L: *mut lua_State, path: *const c_char, sym: *const c_char) -> c_int {
    let mut reg = checkclib(L, path); /* check loaded C libraries */
    if reg.is_null() {
        /* must load library? */
        reg = lsys_load(L, path, (*sym == b'*' as c_char) as c_int); /* global symbols if 'sym'=='*' */
        if reg.is_null() {
            return ERRLIB; /* unable to load library */
        }
        addtoclib(L, path, reg);
    }
    if *sym == b'*' as c_char {
        /* loading only library (no function)? */
        lua_pushboolean(L, 1); /* return 'true' */
        return 0; /* no errors */
    } else {
        let f = lsys_sym(L, reg, sym);
        if f.is_none() {
            return ERRFUNC; /* unable to find function */
        }
        lua_pushcfunction(L, f); /* else create new function */
        return 0; /* no errors */
    };
}

unsafe extern "C" fn ll_loadlib(L: *mut lua_State) -> c_int {
    let path = luaL_checkstring(L, 1);
    let init = luaL_checkstring(L, 2);
    let stat = lookforfunc(L, path, init);
    if stat == 0 {
        /* no errors? */
        return 1; /* return the loaded function */
    } else {
        /* error; error message is on stack top */
        lua_pushnil(L);
        lua_insert(L, -2);
        lua_pushstring(
            L,
            if stat == ERRLIB {
                LIB_FAIL
            } else {
                cstr!("init")
            },
        );
        return 3; /* return nil, error message, and where */
    };
}

/*
**
** 'require' function
**
*/

unsafe fn readable(filename: *const c_char) -> c_int {
    let f = fopen(filename, cstr!("r")); /* try to open file */
    if f.is_null() {
        return 0; /* open failed */
    }
    fclose(f);
    return 1;
}

unsafe fn pushnexttemplate(L: *mut lua_State, mut path: *const c_char) -> *const c_char {
    while *path == b';' as c_char {
        /* skip separators */
        path = path.add(1);
    }
    if *path == 0 {
        return ptr::null(); /* no more templates */
    }
    let mut l: *const c_char = strchr(path, b';' as c_int); /* find next separator */
    if l.is_null() {
        l = path.add(strlen(path) as usize);
    }
    lua_pushlstring(L, path, l.offset_from(path) as usize); /* template */
    return l;
}

unsafe extern "C" fn searchpath(
    L: *mut lua_State,
    mut name: *const c_char,
    mut path: *const c_char,
    sep: *const c_char,
    dirsep: *const c_char,
) -> *const c_char {
    /* to build error message */
    let mut msg = luaL_Buffer::new();
    luaL_buffinit(L, &mut msg);
    if *sep != 0 {
        /* non-empty separator? */
        name = luaL_gsub(L, name, sep, dirsep); /* replace it by 'dirsep' */
    }
    while {
        path = pushnexttemplate(L, path);
        !path.is_null()
    } {
        let filename = luaL_gsub(L, lua_tostring(L, -1), cstr!("?"), name);
        lua_remove(L, -2); /* remove path template */
        if readable(filename) != 0 {
            /* does file exist and is readable? */
            return filename; /* return that file name */
        }
        lua_pushfstring(L, cstr!("\n\tno file '%s'"), filename);
        lua_remove(L, -2); /* remove file name */
        luaL_addvalue(&mut msg); /* concatenate error msg. entry */
    }
    luaL_pushresult(&mut msg); /* create error message */
    return ptr::null();
}

unsafe extern "C" fn ll_searchpath(L: *mut lua_State) -> c_int {
    let f = searchpath(
        L,
        luaL_checkstring(L, 1),
        luaL_checkstring(L, 2),
        luaL_optstring(L, 3, cstr!(".")),
        luaL_optstring(L, 4, cstr!("/")), // TODO: path::MAIN_SEPARATOR
    );
    if !f.is_null() {
        return 1;
    } else {
        /* error message is on top of the stack */
        lua_pushnil(L);
        lua_insert(L, -2);
        return 2; /* return nil + error message */
    }
}

unsafe fn findfile(
    L: *mut lua_State,
    name: *const c_char,
    pname: *const c_char,
    dirsep: *const c_char,
) -> *const c_char {
    lua_getfield(L, lua_upvalueindex(1), pname);
    let path = lua_tostring(L, -1);
    if path.is_null() {
        luaL_error(L, cstr!("'package.%s' must be a string"), pname);
    }
    return searchpath(L, name, path, cstr!("."), dirsep);
}

unsafe fn checkload(L: *mut lua_State, stat: c_int, filename: *const c_char) -> libc::c_int {
    if stat != 0 {
        /* module loaded successfully? */
        lua_pushstring(L, filename); /* will be 2nd argument to module */
        return 2; /* return open function and file name */
    } else {
        return luaL_error(
            L,
            cstr!("error loading module '%s' from file '%s':\n\t%s"),
            lua_tostring(L, 1),
            filename,
            lua_tostring(L, -1),
        );
    };
}

unsafe extern "C" fn searcher_Lua(L: *mut lua_State) -> c_int {
    let name = luaL_checkstring(L, 1);
    let filename = findfile(L, name, cstr!("path"), cstr!("/")); // TODO: path::MAIN_SEPARATOR
    if filename.is_null() {
        /* module not found in this path */
        return 1;
    }
    return checkload(L, (luaL_loadfile(L, filename) == LUA_OK) as c_int, filename);
}

/*
** Try to find a load function for module 'modname' at file 'filename'.
** First, change '.' to '_' in 'modname'; then, if 'modname' has
** the form X-Y (that is, it has an "ignore mark"), build a function
** name "luaopen_X" and look for it. (For compatibility, if that
** fails, it also tries "luaopen_Y".) If there is no ignore mark,
** look for a function named "luaopen_modname".
*/
unsafe fn loadfunc(L: *mut lua_State, filename: *const c_char, modname: *const c_char) -> c_int {
    let mut modname = luaL_gsub(L, modname, cstr!("."), cstr!("_")); // LUA_OFSEP
    let mark = strchr(modname, '-' as c_int); // LUA_IGMARK
    if !mark.is_null() {
        let openfunc = lua_pushlstring(L, modname, mark.offset_from(modname) as usize);
        let openfunc = lua_pushfstring(L, cstr!("luaopen_%s"), openfunc);
        let stat = lookforfunc(L, filename, openfunc);
        if stat != ERRFUNC {
            return stat;
        }
        modname = mark.add(1); /* else go ahead and try old-style name */
    }
    let openfunc = lua_pushfstring(L, cstr!("luaopen_%s"), modname);
    return lookforfunc(L, filename, openfunc);
}

unsafe extern "C" fn searcher_C(L: *mut lua_State) -> c_int {
    let name = luaL_checkstring(L, 1);
    let filename = findfile(L, name, cstr!("cpath"), cstr!("/")); // TODO: path::MAIN_SEPARATOR
    if filename.is_null() {
        return 1; /* module not found in this path */
    }
    return checkload(L, (loadfunc(L, filename, name) == 0) as c_int, filename);
}

unsafe extern "C" fn searcher_Croot(L: *mut lua_State) -> c_int {
    let name = luaL_checkstring(L, 1);
    let p = strchr(name, b'.' as i32);
    if p.is_null() {
        return 0; /* is root */
    }
    lua_pushlstring(L, name, p.offset_from(name) as usize);
    let filename = findfile(
        L,
        lua_tostring(L, -1),
        cstr!("cpath"),
        cstr!("/"), // TODO: path::MAIN_SEPARATOR
    );
    if filename.is_null() {
        return 1; /* root not found */
    }
    let stat = loadfunc(L, filename, name);
    if stat != 0 {
        if stat != ERRFUNC {
            return checkload(L, 0, filename); /* real error */
        } else {
            /* open function not found */
            lua_pushfstring(L, cstr!("\n\tno module '%s' in file '%s'"), name, filename);
            return 1;
        }
    }
    lua_pushstring(L, filename); /* will be 2nd argument to module */
    return 2;
}

unsafe extern "C" fn searcher_preload(L: *mut lua_State) -> c_int {
    let name = luaL_checkstring(L, 1);
    lua_getfield(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE);
    if lua_getfield(L, -1, name) == LUA_TNIL {
        /* not found? */
        lua_pushfstring(L, cstr!("\n\tno field package.preload['%s']"), name);
    }
    return 1;
}

unsafe extern "C" fn findloader(L: *mut lua_State, name: *const c_char) {
    let mut msg = luaL_Buffer::new(); /* to build error message */
    luaL_buffinit(L, &mut msg);
    /* push 'package.searchers' to index 3 in the stack */
    if lua_getfield(L, lua_upvalueindex(1), cstr!("searchers")) != LUA_TTABLE {
        luaL_error(L, cstr!("'package.searchers' must be a table"));
    }
    /* iterate over available searchers to find a loader */
    let mut i = 1;
    loop {
        if lua_rawgeti(L, 3, i) == LUA_TNIL {
            /* no more searchers? */
            lua_pop(L, 1); /* remove nil */
            luaL_pushresult(&mut msg); /* create error message */
            luaL_error(
                L,
                cstr!("module '%s' not found:%s"),
                name,
                lua_tostring(L, -1),
            );
        }
        lua_pushstring(L, name);
        lua_call(L, 1, 2); /* call it */
        if lua_isfunction(L, -2) != 0 {
            return; /* module loader found */
        } else {
            if lua_isstring(L, -2) != 0 {
                /* searcher returned error message? */
                lua_pop(L, 1); /* remove extra return */
                luaL_addvalue(&mut msg); /* concatenate error message */
            } else {
                lua_pop(L, 2); /* remove both returns */
            }
        }
        i += 1;
    }
}

unsafe extern "C" fn ll_require(L: *mut lua_State) -> c_int {
    let name = luaL_checkstring(L, 1);
    lua_settop(L, 1); /* LOADED table will be at index 2 */
    lua_getfield(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE);
    lua_getfield(L, 2, name); /* LOADED[name] */
    if lua_toboolean(L, -1) != 0 {
        return 1; /* package is already loaded */
    }
    /* else must load package */
    lua_pop(L, 1); /* remove 'getfield' result */
    findloader(L, name);
    lua_pushstring(L, name); /* pass name as argument to module loader */
    lua_insert(L, -2); /* name is 1st argument (before search data) */
    lua_call(L, 2, 1); /* run loader to load module */
    if lua_isnil(L, -1) == 0 {
        lua_setfield(L, 2, name); /* LOADED[name] = returned value */
    }
    if lua_getfield(L, 2, name) == LUA_TNIL {
        /* module set no value? */
        lua_pushboolean(L, 1); /* use true as result */
        lua_pushvalue(L, -1); /* extra copy to be returned */
        lua_setfield(L, 2, name); /* LOADED[name] = true */
    }
    return 1;
}

static mut pk_funcs: [luaL_Reg; 8] = [
    luaL_Reg {
        name: cstr!("loadlib"),
        func: Some(ll_loadlib),
    },
    luaL_Reg {
        name: cstr!("searchpath"),
        func: Some(ll_searchpath),
    },
    /* placeholders */
    luaL_Reg {
        name: cstr!("preload"),
        func: None,
    },
    luaL_Reg {
        name: cstr!("cpath"),
        func: None,
    },
    luaL_Reg {
        name: cstr!("path"),
        func: None,
    },
    luaL_Reg {
        name: cstr!("searchers"),
        func: None,
    },
    luaL_Reg {
        name: cstr!("loaded"),
        func: None,
    },
    luaL_Reg {
        name: ptr::null(),
        func: None,
    },
];

static mut ll_funcs: [luaL_Reg; 2] = [
    luaL_Reg {
        name: cstr!("require"),
        func: Some(ll_require),
    },
    luaL_Reg {
        name: ptr::null(),
        func: None,
    },
];

unsafe extern "C" fn createsearcherstable(L: *mut lua_State) {
    static mut searchers: [lua_CFunction; 5] = [
        Some(searcher_preload),
        Some(searcher_Lua),
        Some(searcher_C),
        Some(searcher_Croot),
        None,
    ];
    /* create 'searchers' table */
    lua_createtable(L, (searchers.len() - 1) as c_int, 0);
    /* fill it with predefined searchers */
    let mut i = 0;
    while searchers[i].is_some() {
        lua_pushvalue(L, -2); /* set 'package' as upvalue for all searchers */
        lua_pushcclosure(L, searchers[i], 1);
        lua_rawseti(L, -2, (i + 1) as lua_Integer);
        i += 1;
    }
    /* put it in field 'searchers' */
    lua_setfield(L, -2, cstr!("searchers"));
}

/*
** create table CLIBS to keep track of loaded C libraries,
** setting a finalizer to close all libraries when closing state.
*/
unsafe fn createclibstable(L: *mut lua_State) {
    lua_newtable(L); /* create CLIBS table */
    lua_createtable(L, 0, 1); /* create metatable for CLIBS */
    lua_pushcfunction(L, Some(gctm));
    lua_setfield(L, -2, cstr!("__gc")); /* set finalizer for CLIBS table */
    lua_setmetatable(L, -2);
    lua_rawsetp(L, LUA_REGISTRYINDEX, &CLIBS as *const u8 as *const c_void); /* set CLIBS table in registry */
}

#[no_mangle]
pub unsafe extern "C" fn luaopen_package(L: *mut lua_State) -> c_int {
    createclibstable(L);
    luaL_newlib(L, pk_funcs.as_ptr()); /* create 'package' table */
    createsearcherstable(L);
    /* set paths */
    setpath(
        L,
        cstr!("path"),
        cstr!("LUA_PATH"),
        // TODO: LUA_PATH_DEFAULT
        cstr!("/usr/local/share/lua/5.3/?.lua;/usr/local/share/lua/5.3/?/init.lua;/usr/local/lib/lua/5.3/?.lua;/usr/local/lib/lua/5.3/?/init.lua;./?.lua;./?/init.lua"),
    );
    setpath(
        L,
        cstr!("cpath"),
        cstr!("LUA_CPATH"),
        // TODO: LUA_CPATH_DEFAULT
        cstr!("/usr/local/lib/lua/5.3/?.so;/usr/local/lib/lua/5.3/loadall.so;./?.so"),
    );
    /* store config information */
    lua_pushstring(L, cstr!("/\n;\n?\n!\n-\n\0")); // TODO: path::MAIN_SEPARATOR
    lua_setfield(L, -2, cstr!("config"));
    /* set field 'loaded' */
    luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE);
    lua_setfield(L, -2, cstr!("loaded"));
    /* set field 'preload' */
    luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE);
    lua_setfield(L, -2, cstr!("preload"));
    lua_pushglobaltable(L);
    lua_pushvalue(L, -2); /* set 'package' as upvalue for next lib */
    luaL_setfuncs(L, ll_funcs.as_ptr(), 1); /* open lib into global table */
    lua_pop(L, 1); /* pop global table */
    return 1; /* return 'package' table */
}
