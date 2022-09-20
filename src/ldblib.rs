use std::ffi::c_char;
use std::io;

use libc::{c_int, c_ulong, fflush, fgets, fprintf, intptr_t, strcmp, FILE};

use crate::lapi::{
    lua_checkstack, lua_getmetatable, lua_getupvalue, lua_getuservalue, lua_insert, lua_isfunction,
    lua_isthread, lua_pcall, lua_pop, lua_pushlightuserdata, lua_pushnil, lua_pushstring,
    lua_pushvalue, lua_rotate, lua_setmetatable, lua_settop, lua_setupvalue, lua_setuservalue,
    lua_tolstring, lua_tostring, lua_tothread, lua_type, lua_upvalueid, lua_upvaluejoin, lua_xmove,
};
use crate::lauxlib::{
    luaL_Reg, luaL_argerror, luaL_checkany, luaL_checkinteger, luaL_checktype, luaL_error,
    luaL_loadbuffer, luaL_newlib, luaL_optinteger, luaL_traceback, lua_isnoneornil,
};
use crate::ldebug::{lua_getlocal, lua_getstack, lua_setlocal};
use crate::lstate::{lua_State, CallInfo};
use crate::types::{
    lua_CFunction, lua_Debug, lua_Integer, LUA_REGISTRYINDEX, LUA_TFUNCTION, LUA_TNIL, LUA_TTABLE,
    LUA_TUSERDATA,
};

static mut HOOKKEY: c_int = 0 as c_int;

pub const NULL: c_int = 0 as c_int;

unsafe extern "C" fn getthread(L: *mut lua_State, arg: *mut c_int) -> *mut lua_State {
    if lua_isthread(L, 1) {
        *arg = 1 as c_int;
        return lua_tothread(L, 1 as c_int);
    } else {
        *arg = 0 as c_int;
        return L;
    };
}

unsafe extern "C" fn checkstack(L: *mut lua_State, L1: *mut lua_State, n: c_int) {
    if L != L1 && lua_checkstack(L1, n) == 0 {
        luaL_error(L, cstr!("stack overflow") as *const u8 as *const c_char);
    }
}

unsafe extern "C" fn db_debug(L: *mut lua_State) -> c_int {
    loop {
        let mut buffer: [c_char; 250] = [0; 250];
        if (fgets(
            buffer.as_mut_ptr(),
            ::core::mem::size_of::<[c_char; 250]>() as c_ulong as c_int,
            io::stdin as *mut FILE,
        ))
        .is_null()
            || strcmp(
                buffer.as_mut_ptr(),
                cstr!("cont\n") as *const u8 as *const c_char,
            ) == 0 as c_int
        {
            return 0 as c_int;
        }
        let name = String::from("=(debug command)");
        if luaL_loadbuffer(
            L,
            buffer.as_ptr() as *const i8,
            buffer.len(),
            name.as_ptr() as *const i8,
        ) != 0
            || lua_pcall(L, 0, 0, 0) != 0
        {
            fprintf(
                io::stderr as *mut FILE,
                cstr!("%s\n") as *const u8 as *const c_char,
                lua_tolstring(L, -(1 as c_int), 0 as *mut usize),
            );
            fflush(io::stderr as *mut FILE);
        }
        lua_settop(L, 0 as c_int);
    }
}

unsafe extern "C" fn db_traceback(L: *mut lua_State) -> c_int {
    let mut arg: c_int = 0;
    let L1 = getthread(L, &mut arg);
    let msg = lua_tostring(L, arg + 1);
    if msg.is_null() && !lua_isnoneornil(L, arg + 1) {
        lua_pushvalue(L, arg + 1 as c_int);
    } else {
        let level = luaL_optinteger(
            L,
            arg + 2 as c_int,
            (if L == L1 { 1 as c_int } else { 0 as c_int }) as lua_Integer,
        ) as c_int;
        luaL_traceback(L, L1, msg, level);
    }
    return 1 as c_int;
}

unsafe extern "C" fn db_getuservalue(L: *mut lua_State) -> c_int {
    if lua_type(L, 1 as c_int) != LUA_TUSERDATA {
        lua_pushnil(L);
    } else {
        lua_getuservalue(L, 1 as c_int);
    }
    return 1 as c_int;
}

unsafe extern "C" fn db_setuservalue(L: *mut lua_State) -> c_int {
    luaL_checktype(L, 1 as c_int, LUA_TUSERDATA);
    luaL_checkany(L, 2 as c_int);
    lua_settop(L, 2 as c_int);
    lua_setuservalue(L, 1 as c_int);
    return 1 as c_int;
}

unsafe extern "C" fn db_setlocal(L: *mut lua_State) -> c_int {
    let mut arg: c_int = 0;
    let name: *const c_char;
    let L1 = getthread(L, &mut arg);
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const c_char,
        namewhat: 0 as *const c_char,
        what: 0 as *const c_char,
        source: 0 as *const c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let level = luaL_checkinteger(L, arg + 1 as c_int) as c_int;
    let nvar = luaL_checkinteger(L, arg + 2 as c_int) as c_int;
    if lua_getstack(L1, level, &mut ar) == 0 {
        return luaL_argerror(L, arg + 1 as c_int, cstr!("level out of range"));
    }
    luaL_checkany(L, arg + 3 as c_int);
    lua_settop(L, arg + 3 as c_int);
    checkstack(L, L1, 1 as c_int);
    lua_xmove(L, L1, 1 as c_int);
    name = lua_setlocal(L1, &mut ar, nvar);
    if name.is_null() {
        lua_pop(L1, 1);
    }
    lua_pushstring(L, name);
    return 1 as c_int;
}

unsafe extern "C" fn db_getlocal(L: *mut lua_State) -> c_int {
    let mut arg: c_int = 0;
    let L1 = getthread(L, &mut arg);
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const c_char,
        namewhat: 0 as *const c_char,
        what: 0 as *const c_char,
        source: 0 as *const c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let name: *const c_char;
    let nvar = luaL_checkinteger(L, arg + 2 as c_int) as c_int;
    if lua_isfunction(L, arg + 1) != 0 {
        lua_pushvalue(L, arg + 1 as c_int);
        lua_pushstring(L, lua_getlocal(L, NULL as *const lua_Debug, nvar));
        return 1 as c_int;
    } else {
        let level = luaL_checkinteger(L, arg + 1 as c_int) as c_int;
        if lua_getstack(L1, level, &mut ar) == 0 {
            return luaL_argerror(L, arg + 1 as c_int, cstr!("level out of range"));
        }
        checkstack(L, L1, 1 as c_int);
        name = lua_getlocal(L1, &mut ar, nvar);
        if !name.is_null() {
            lua_xmove(L1, L, 1 as c_int);
            lua_pushstring(L, name);
            lua_rotate(L, -(2 as c_int), 1 as c_int);
            return 2 as c_int;
        } else {
            lua_pushnil(L);
            return 1 as c_int;
        }
    };
}

unsafe extern "C" fn db_getregistry(L: *mut lua_State) -> c_int {
    lua_pushvalue(L, LUA_REGISTRYINDEX);
    return 1 as c_int;
}

unsafe extern "C" fn db_setmetatable(L: *mut lua_State) -> c_int {
    let t = lua_type(L, 2 as c_int);
    if t != LUA_TNIL && t != LUA_TTABLE {
        return luaL_argerror(L, 2, cstr!("nil or table expected"));
    }
    lua_settop(L, 2 as c_int);
    lua_setmetatable(L, 1 as c_int);
    return 1 as c_int;
}

unsafe extern "C" fn db_getmetatable(L: *mut lua_State) -> c_int {
    luaL_checkany(L, 1 as c_int);
    if lua_getmetatable(L, 1 as c_int) == 0 {
        lua_pushnil(L);
    }
    return 1 as c_int;
}

unsafe extern "C" fn auxupvalue(L: *mut lua_State, get: c_int) -> c_int {
    let name: *const c_char;
    let n = luaL_checkinteger(L, 2 as c_int) as c_int;
    luaL_checktype(L, 1 as c_int, LUA_TFUNCTION);
    name = if get != 0 {
        lua_getupvalue(L, 1 as c_int, n)
    } else {
        lua_setupvalue(L, 1 as c_int, n)
    };
    if name.is_null() {
        return 0 as c_int;
    }
    lua_pushstring(L, name);
    lua_insert(L, -(get + 1 as c_int));
    return get + 1 as c_int;
}

unsafe extern "C" fn db_getupvalue(L: *mut lua_State) -> c_int {
    return auxupvalue(L, 1 as c_int);
}

unsafe extern "C" fn db_setupvalue(L: *mut lua_State) -> c_int {
    luaL_checkany(L, 3 as c_int);
    return auxupvalue(L, 0 as c_int);
}

unsafe extern "C" fn checkupval(L: *mut lua_State, argf: c_int, argnup: c_int) -> c_int {
    let nup = luaL_checkinteger(L, argnup) as c_int;
    luaL_checktype(L, argf, LUA_TFUNCTION);
    if lua_getupvalue(L, argf, nup) == 0 as *const c_char {
        luaL_argerror(L, argf, cstr!("invalid upvalue index"));
    }
    return nup;
}

unsafe extern "C" fn db_upvalueid(L: *mut lua_State) -> c_int {
    let n = checkupval(L, 1 as c_int, 2 as c_int);
    lua_pushlightuserdata(L, lua_upvalueid(L, 1 as c_int, n));
    return 1 as c_int;
}

unsafe extern "C" fn db_upvaluejoin(L: *mut lua_State) -> c_int {
    let n1 = checkupval(L, 1 as c_int, 2 as c_int);
    let n2 = checkupval(L, 3 as c_int, 4 as c_int);
    lua_upvaluejoin(L, 1 as c_int, n1, 3 as c_int, n2);
    return 0 as c_int;
}

static mut dblib: [luaL_Reg; 17] = unsafe {
    [
        {
            let init = luaL_Reg {
                name: b"debug\0" as *const u8 as *const c_char,
                func: Some(db_debug as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"getuservalue\0" as *const u8 as *const c_char,
                func: Some(db_getuservalue as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"gethook\0" as *const u8 as *const c_char,
                func: Some(db_gethook as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"getinfo\0" as *const u8 as *const c_char,
                func: Some(db_getinfo as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"getlocal\0" as *const u8 as *const c_char,
                func: Some(db_getlocal as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"getregistry\0" as *const u8 as *const c_char,
                func: Some(db_getregistry as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"getmetatable\0" as *const u8 as *const c_char,
                func: Some(db_getmetatable as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"getupvalue\0" as *const u8 as *const c_char,
                func: Some(db_getupvalue as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"upvaluejoin\0" as *const u8 as *const c_char,
                func: Some(db_upvaluejoin as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"upvalueid\0" as *const u8 as *const c_char,
                func: Some(db_upvalueid as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"setuservalue\0" as *const u8 as *const c_char,
                func: Some(db_setuservalue as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"sethook\0" as *const u8 as *const c_char,
                func: Some(db_sethook as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"setlocal\0" as *const u8 as *const c_char,
                func: Some(db_setlocal as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"setmetatable\0" as *const u8 as *const c_char,
                func: Some(db_setmetatable as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"setupvalue\0" as *const u8 as *const c_char,
                func: Some(db_setupvalue as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"traceback\0" as *const u8 as *const c_char,
                func: Some(db_traceback as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: NULL as *const c_char,
                func: ::core::mem::transmute::<intptr_t, lua_CFunction>(NULL as intptr_t),
            };
            init
        },
    ]
};

#[no_mangle]
pub unsafe extern "C" fn luaopen_debug(L: *mut lua_State) -> c_int {
    luaL_newlib(L, dblib.as_ptr());
    return 1 as c_int;
}

extern "C" {
    fn db_gethook(L: *mut lua_State) -> c_int;
    fn db_getinfo(L: *mut lua_State) -> c_int;
    fn db_sethook(L: *mut lua_State) -> c_int;
}
