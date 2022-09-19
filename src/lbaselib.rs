use std::ptr;

/*
** Basic library
*/
use libc::{
    c_char, c_double, c_int, c_longlong, c_uint, c_ulonglong, c_void, isalnum, isdigit, size_t,
    strspn, toupper,
};

use crate::lapi::{
    lua_call, lua_callk, lua_concat, lua_error, lua_gc, lua_getglobal, lua_geti, lua_getmetatable,
    lua_gettop, lua_insert, lua_isnil, lua_isstring, lua_load, lua_next, lua_pcallk, lua_pop,
    lua_pushboolean, lua_pushcfunction, lua_pushglobaltable, lua_pushinteger, lua_pushliteral,
    lua_pushnil, lua_pushnumber, lua_pushstring, lua_pushvalue, lua_rawequal, lua_rawget,
    lua_rawlen, lua_rawset, lua_remove, lua_rotate, lua_setfield, lua_setmetatable, lua_settop,
    lua_setupvalue, lua_stringtonumber, lua_toboolean, lua_tolstring, lua_tostring, lua_type,
    lua_typename,
};
use crate::lauxlib::{
    luaL_Reg, luaL_argerror, luaL_checkany, luaL_checkinteger, luaL_checkoption, luaL_checkstack,
    luaL_checktype, luaL_error, luaL_getmetafield, luaL_loadbufferx, luaL_loadfile, luaL_loadfilex,
    luaL_optinteger, luaL_optstring, luaL_setfuncs, luaL_tolstring, luaL_where, lua_isnone,
    lua_isnoneornil, lua_replace, lua_writeline, lua_writestring,
};
use crate::lstate::lua_State;
use crate::types::{
    lua_CFunction, lua_Integer, lua_KContext, lua_Number, lua_Unsigned, LUA_MULTRET, LUA_OK,
    LUA_TFUNCTION, LUA_TNIL, LUA_TNONE, LUA_TNUMBER, LUA_TSTRING, LUA_TTABLE, LUA_YIELD,
};

pub const LUA_GCSTOP: c_int = 0;
pub const LUA_GCRESTART: c_int = 1;
pub const LUA_GCCOLLECT: c_int = 2;
pub const LUA_GCCOUNT: c_int = 3;
pub const LUA_GCCOUNTB: c_int = 4;
pub const LUA_GCSTEP: c_int = 5;
pub const LUA_GCSETPAUSE: c_int = 6;
pub const LUA_GCSETSTEPMUL: c_int = 7;
pub const LUA_GCISRUNNING: c_int = 9;

unsafe extern "C" fn luaB_print(L: *mut lua_State) -> c_int {
    let n = lua_gettop(L); /* number of arguments */
    let mut i: c_int = 1;
    lua_getglobal(L, cstr!("tostring"));
    while i <= n {
        let mut l: size_t = 0;
        lua_pushvalue(L, -(1 as c_int)); /* function to be called */
        lua_pushvalue(L, i); /* value to print */
        lua_call(L, 1, 1);
        let s: *const c_char = lua_tolstring(L, -1, &mut l); /* get result */
        if s.is_null() {
            luaL_error(L, cstr!("'tostring' must return a string to 'print'"));
        }
        if i > 1 as c_int {
            lua_writestring(cstr!("\t"), 1);
        }
        lua_writestring(s, l);
        lua_pop(L, 1); /* pop result */
        i += 1;
    }
    lua_writeline();
    return 0 as c_int;
}

pub const SPACECHARS: [c_char; 7] =
    unsafe { *::core::mem::transmute::<&[u8; 7], &[c_char; 7]>(b" \x0C\n\r\t\x0B\0") };

unsafe extern "C" fn b_str2int(
    mut s: *const c_char,
    base: c_int,
    pn: *mut lua_Integer,
) -> *const c_char {
    let mut n: lua_Unsigned = 0;
    let mut neg: c_int = 0;
    s = s.offset(strspn(s, SPACECHARS.as_ptr()) as isize); /* skip initial spaces */
    if *s as c_int == '-' as i32 {
        /* handle signal */
        s = s.offset(1);
        neg = 1;
    } else if *s as c_int == '+' as i32 {
        s = s.offset(1);
    }
    if isalnum(*s as i32) == 0 {
        /* no digit? */
        return ptr::null_mut();
    }
    loop {
        let digit: c_int = if isdigit(*s as c_int) != 0 {
            *s as c_int - '0' as i32
        } else {
            toupper(*s as c_int) - 'A' as i32 + 10 as c_int
        };
        if digit >= base {
            /* invalid numeral */
            return ptr::null_mut();
        }
        n = n.wrapping_mul(base as u64).wrapping_add(digit as u64);
        s = s.offset(1);
        if !(isalnum(*s as i32) != 0) {
            break;
        }
    }
    s = s.offset(strspn(s, SPACECHARS.as_ptr()) as isize); /* skip trailing spaces */
    *pn = (if neg != 0 {
        (0 as c_uint as c_ulonglong).wrapping_sub(n)
    } else {
        n
    }) as lua_Integer;
    return s;
}

unsafe extern "C" fn luaB_tonumber(L: *mut lua_State) -> c_int {
    if lua_isnoneornil(L, 2) {
        /* standard conversion? */
        luaL_checkany(L, 1);
        if lua_type(L, 1) == LUA_TNUMBER {
            /* already a number? */
            lua_settop(L, 1); /* yes; return it */
            return 1;
        } else {
            let mut l: size_t = 0;
            let s = lua_tolstring(L, 1, &mut l);
            if !s.is_null() && lua_stringtonumber(L, s) == l.wrapping_add(1)
            /* successful conversion to number */
            {
                return 1;
            }
            /* else not a number */
        }
    } else {
        let mut l_0: size_t = 0;
        let s_0: *const c_char;
        let mut n: lua_Integer = 0;
        let base: c_int = luaL_checkinteger(L, 2) as c_int;
        luaL_checktype(L, 1 as c_int, LUA_TSTRING); /* no numbers as strings */
        s_0 = lua_tolstring(L, 1, &mut l_0);
        if b_str2int(s_0, base, &mut n) == s_0.offset(l_0 as isize) {
            lua_pushinteger(L, n);
            return 1;
        } /* else not a number */
    } /* else not a number */
    lua_pushnil(L); /* not a number */
    return 1;
}

unsafe extern "C" fn luaB_error(L: *mut lua_State) -> c_int {
    let level: c_int = luaL_optinteger(L, 2, 1) as c_int;
    lua_settop(L, 1);
    if lua_type(L, 1) == LUA_TSTRING && level > 0 {
        luaL_where(L, level); /* add extra information */
        lua_pushvalue(L, 1);
        lua_concat(L, 2);
    }
    lua_error(L)
}

unsafe extern "C" fn luaB_getmetatable(L: *mut lua_State) -> c_int {
    luaL_checkany(L, 1 as c_int);
    if lua_getmetatable(L, 1) == 0 {
        lua_pushnil(L);
        return 1; /* no metatable */
    }
    luaL_getmetafield(L, 1, cstr!("__metatable"));
    return 1; /* returns either __metatable field (if present) or metatable */
}

unsafe extern "C" fn luaB_setmetatable(L: *mut lua_State) -> c_int {
    let _t = lua_type(L, 2);
    // TODO: FIX
    luaL_checktype(L, 1, LUA_TTABLE); // FIXME? t is unused?
    if luaL_getmetafield(L, 1, cstr!("__metatable")) != LUA_TNIL {
        luaL_error(L, cstr!("cannot change a protected metatable"));
    }
    lua_settop(L, 2);
    lua_setmetatable(L, 1);
    return 1;
}

unsafe extern "C" fn luaB_rawequal(L: *mut lua_State) -> c_int {
    luaL_checkany(L, 1);
    luaL_checkany(L, 2);
    lua_pushboolean(L, lua_rawequal(L, 1, 2));
    return 1;
}

unsafe extern "C" fn luaB_rawlen(L: *mut lua_State) -> c_int {
    let t = lua_type(L, 1);
    if !(t == LUA_TTABLE || t == LUA_TSTRING) {
        luaL_argerror(L, t, cstr!("table or string expected"));
    }
    lua_pushinteger(L, lua_rawlen(L, 1) as lua_Integer);
    return 1;
}

unsafe extern "C" fn luaB_rawget(L: *mut lua_State) -> c_int {
    luaL_checktype(L, 1, LUA_TTABLE);
    luaL_checkany(L, 2);
    lua_settop(L, 2);
    lua_rawget(L, 1);
    return 1;
}

unsafe extern "C" fn luaB_rawset(L: *mut lua_State) -> c_int {
    luaL_checktype(L, 1, LUA_TTABLE);
    luaL_checkany(L, 2);
    luaL_checkany(L, 3);
    lua_settop(L, 3);
    lua_rawset(L, 1);
    return 1;
}

unsafe extern "C" fn luaB_collectgarbage(L: *mut lua_State) -> c_int {
    static mut opts: [*const c_char; 9] = [
        cstr!("stop"),
        cstr!("restart"),
        cstr!("collect"),
        cstr!("count"),
        cstr!("step"),
        cstr!("setpause"),
        cstr!("setstepmul"),
        cstr!("isrunning"),
        ptr::null_mut() as *const c_char,
    ];
    static mut optsnum: [c_int; 8] = [
        LUA_GCSTOP,
        LUA_GCRESTART,
        LUA_GCCOLLECT,
        LUA_GCCOUNT,
        LUA_GCSTEP,
        LUA_GCSETPAUSE,
        LUA_GCSETSTEPMUL,
        LUA_GCISRUNNING,
    ];
    let o = optsnum[luaL_checkoption(L, 1 as c_int, cstr!("collect"), opts.as_ptr()) as usize];
    let ex = luaL_optinteger(L, 2 as c_int, 0 as c_int as lua_Integer) as c_int;
    let res = lua_gc(L, o, ex);
    match o {
        LUA_GCCOUNT => {
            let b = lua_gc(L, LUA_GCCOUNTB, 0 as c_int);
            lua_pushnumber(
                L,
                res as lua_Number + b as lua_Number / 1024 as c_int as c_double,
            );
            return 1 as c_int;
        }
        LUA_GCSTEP | LUA_GCISRUNNING => {
            lua_pushboolean(L, res);
            return 1 as c_int;
        }
        _ => {
            lua_pushinteger(L, res as lua_Integer);
            return 1 as c_int;
        }
    };
}

unsafe extern "C" fn luaB_type(L: *mut lua_State) -> c_int {
    let t = lua_type(L, 1);
    if !(t != LUA_TNONE) {
        luaL_argerror(L, t, cstr!("value expected"));
    }
    lua_pushstring(L, lua_typename(L, t));
    return 1;
}

unsafe extern "C" fn pairsmeta(
    L: *mut lua_State,
    method: *const c_char,
    iszero: c_int,
    iter: lua_CFunction,
) -> c_int {
    luaL_checkany(L, 1);
    if luaL_getmetafield(L, 1, method) == LUA_TNIL {
        /* no metamethod? */
        lua_pushcfunction(L, iter); /* will return generator, */
        lua_pushvalue(L, 1); /* state, */
        if iszero != 0 {
            lua_pushinteger(L, 0); /* and initial value */
        } else {
            lua_pushnil(L);
        }
    } else {
        lua_pushvalue(L, 1); /* argument 'self' to metamethod */
        lua_call(L, 1, 3); /* get 3 values from metamethod */
    }
    return 3;
}

unsafe extern "C" fn luaB_next(L: *mut lua_State) -> c_int {
    luaL_checktype(L, 1, LUA_TTABLE);
    lua_settop(L, 2); /* create a 2nd argument if there isn't one */
    if lua_next(L, 1) != 0 {
        return 2;
    } else {
        lua_pushnil(L);
        return 1;
    };
}

unsafe extern "C" fn luaB_pairs(L: *mut lua_State) -> c_int {
    return pairsmeta(
        L,
        cstr!("__pairs"),
        0,
        Some(luaB_next as unsafe extern "C" fn(*mut lua_State) -> c_int),
    );
}

/*
** Traversal function for 'ipairs'
*/

unsafe extern "C" fn ipairsaux(L: *mut lua_State) -> c_int {
    let i = luaL_checkinteger(L, 2) + 1;
    lua_pushinteger(L, i);
    return if lua_geti(L, 1, i) == LUA_TNIL { 1 } else { 2 };
}

/*
** 'ipairs' function. Returns 'ipairsaux', given "table", 0.
** (The given "table" may not be a table.)
*/

unsafe extern "C" fn luaB_ipairs(L: *mut lua_State) -> c_int {
    luaL_checkany(L, 1 as c_int);
    lua_pushcfunction(L, Some(ipairsaux)); /* iteration function */
    lua_pushvalue(L, 1); /* state */
    lua_pushinteger(L, 0); /* initial value */
    return 3;
}

unsafe extern "C" fn load_aux(L: *mut lua_State, status: c_int, envidx: c_int) -> c_int {
    if status == LUA_OK {
        if envidx != 0 {
            lua_pushvalue(L, envidx);
            if (lua_setupvalue(L, -2, 1)).is_null() {
                lua_pop(L, 1);
            }
        }
        return 1;
    } else {
        lua_pushnil(L);
        lua_insert(L, -2);
        return 2;
    };
}

unsafe extern "C" fn luaB_loadfile(L: *mut lua_State) -> c_int {
    let fname = luaL_optstring(L, 1, ptr::null_mut());
    let mode = luaL_optstring(L, 2, ptr::null_mut());
    let env = if !lua_isnone(L, 3) { 3 } else { 0 }; /* 'env' index or 0 if no 'env' */
    let status = luaL_loadfilex(L, fname, mode);
    return load_aux(L, status, env);
}

/*
** {======================================================
** Generic Read function
** =======================================================
*/

/*
** reserved slot, above all arguments, to hold a copy of the returned
** string to avoid it being collected while parsed. 'load' has four
** optional arguments (chunk, source name, mode, and environment).
*/
pub const RESERVEDSLOT: c_int = 5;

/*
** Reader for generic 'load' function: 'lua_load' uses the
** stack for internal stuff, so the reader cannot change the
** stack top. Instead, it keeps its resulting string in a
** reserved slot inside the stack.
*/

unsafe extern "C" fn generic_reader(
    L: *mut lua_State,
    _ud: *mut c_void,
    size: *mut size_t,
) -> *const c_char {
    luaL_checkstack(L, 2, cstr!("too many nested functions"));
    lua_pushvalue(L, 1); /* get function */
    lua_call(L, 0, 1); /* call it */
    if lua_isnil(L, -1) != 0 {
        lua_pop(L, 1); /* pop result */
        *size = 0;
        return ptr::null_mut();
    } else {
        if lua_isstring(L, -1) == 0 {
            luaL_error(L, cstr!("reader function must return a string"));
        }
    }
    lua_replace(L, RESERVEDSLOT); /* save string in reserved slot */
    return lua_tolstring(L, RESERVEDSLOT, size);
}

unsafe extern "C" fn luaB_load(L: *mut lua_State) -> c_int {
    let status;
    let mut l: size_t = 0;
    let s = lua_tolstring(L, 1 as c_int, &mut l);
    let mode = luaL_optstring(L, 3, cstr!("bt"));
    let env = if !lua_isnone(L, 4) { 4 } else { 0 };
    if !s.is_null() {
        let chunkname = luaL_optstring(L, 2, s);
        status = luaL_loadbufferx(L, s, l, chunkname, mode);
    } else {
        let chunkname_0 = luaL_optstring(L, 2, cstr!("=(load)"));
        luaL_checktype(L, 1, LUA_TFUNCTION);
        lua_settop(L, RESERVEDSLOT);
        status = lua_load(
            L,
            Some(
                generic_reader
                    as unsafe extern "C" fn(
                        *mut lua_State,
                        *mut c_void,
                        *mut size_t,
                    ) -> *const c_char,
            ),
            ptr::null_mut(),
            chunkname_0,
            mode,
        );
    }
    return load_aux(L, status, env);
}

/* }====================================================== */

unsafe extern "C" fn dofilecont(L: *mut lua_State, _d1: c_int, _d2: lua_KContext) -> c_int {
    return lua_gettop(L) - 1;
}

unsafe extern "C" fn luaB_dofile(L: *mut lua_State) -> c_int {
    let fname = luaL_optstring(L, 1, ptr::null_mut());
    lua_settop(L, 1 as c_int);
    if luaL_loadfile(L, fname) != LUA_OK {
        lua_error(L)
    }
    lua_callk(
        L,
        0,
        LUA_MULTRET,
        0,
        Some(dofilecont as unsafe extern "C" fn(*mut lua_State, c_int, lua_KContext) -> c_int),
    );
    return dofilecont(L, 0 as c_int, 0 as c_int as lua_KContext);
}

unsafe extern "C" fn luaB_assert(L: *mut lua_State) -> c_int {
    if lua_toboolean(L, 1) != 0 {
        /* condition is true? */
        return lua_gettop(L); /* return all arguments */
    } else {
        luaL_checkany(L, 1); /* there must be a condition */
        lua_remove(L, 1); /* remove it */
        lua_pushliteral(L, "assertion failed!"); /* default message */
        lua_settop(L, 1); /* leave only message (default if no other one) */
        return luaB_error(L); /* call 'error' */
    };
}

unsafe extern "C" fn luaB_select(L: *mut lua_State) -> c_int {
    let n = lua_gettop(L);
    if lua_type(L, 1) == LUA_TSTRING && *lua_tostring(L, 1) as c_int == '#' as i32 {
        lua_pushinteger(L, n as lua_Integer - 1);
        return 1;
    } else {
        let mut i = luaL_checkinteger(L, 1);
        if i < 0 {
            i = n as c_longlong + i;
        } else if i > n as c_longlong {
            i = n as lua_Integer;
        }
        if i < 1 {
            luaL_argerror(L, 1, cstr!("index out of range"));
        }
        return n - i as c_int;
    };
}

/*
** Continuation function for 'pcall' and 'xpcall'. Both functions
** already pushed a 'true' before doing the call, so in case of success
** 'finishpcall' only has to return everything in the stack minus
** 'extra' values (where 'extra' is exactly the number of items to be
** ignored).
*/

unsafe extern "C" fn finishpcall(L: *mut lua_State, status: c_int, extra: lua_KContext) -> c_int {
    if status != LUA_OK && status != LUA_YIELD {
        /* error? */
        lua_pushboolean(L, 0); /* first result (false) */
        lua_pushvalue(L, -2); /* error message */
        return 2; /* return false, msg */
    } else {
        return lua_gettop(L) - extra as c_int; /* return all results */
    };
}

unsafe extern "C" fn luaB_pcall(L: *mut lua_State) -> c_int {
    luaL_checkany(L, 1);
    lua_pushboolean(L, 1); /* first result if no errors */
    lua_insert(L, 1); /* put it in place */
    let status = lua_pcallk(L, lua_gettop(L) - 2, LUA_MULTRET, 0, 0, Some(finishpcall));
    return finishpcall(L, status, 0);
}

/*
** Do a protected call with error handling. After 'lua_rotate', the
** stack will have <f, err, true, f, [args...]>; so, the function passes
** 2 to 'finishpcall' to skip the 2 first values when returning results.
*/
unsafe extern "C" fn luaB_xpcall(L: *mut lua_State) -> c_int {
    let n = lua_gettop(L);
    luaL_checktype(L, 2, LUA_TFUNCTION); /* check error function */
    lua_pushboolean(L, 1); /* first result */
    lua_pushvalue(L, 1); /* function */
    lua_rotate(L, 3, 2); /* move them below function's arguments */
    let status = lua_pcallk(L, n - 2, LUA_MULTRET, 2, 2, Some(finishpcall));
    return finishpcall(L, status, 2);
}

unsafe extern "C" fn luaB_tostring(L: *mut lua_State) -> c_int {
    luaL_checkany(L, 1);
    luaL_tolstring(L, 1, ptr::null_mut());
    return 1;
}

static mut base_funcs: [luaL_Reg; 25] = {
    [
        {
            let init = luaL_Reg {
                name: cstr!("assert"),
                func: Some(luaB_assert),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("collectgarbage"),
                func: Some(luaB_collectgarbage),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("dofile"),
                func: Some(luaB_dofile),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("error"),
                func: Some(luaB_error),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("getmetatable"),
                func: Some(luaB_getmetatable),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("ipairs"),
                func: Some(luaB_ipairs),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("loadfile"),
                func: Some(luaB_loadfile),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("load"),
                func: Some(luaB_load),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("next"),
                func: Some(luaB_next),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("pairs"),
                func: Some(luaB_pairs),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("pcall"),
                func: Some(luaB_pcall),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("print"),
                func: Some(luaB_print),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("rawequal"),
                func: Some(luaB_rawequal),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("rawlen"),
                func: Some(luaB_rawlen),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("rawget"),
                func: Some(luaB_rawget),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("rawset"),
                func: Some(luaB_rawset),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("select"),
                func: Some(luaB_select),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("setmetatable"),
                func: Some(luaB_setmetatable),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("tonumber"),
                func: Some(luaB_tonumber),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("tostring"),
                func: Some(luaB_tostring),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("type"),
                func: Some(luaB_type),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("xpcall"),
                func: Some(luaB_xpcall),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("_G"),
                func: None,
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("_VERSION"),
                func: None,
            };
            init
        },
        {
            let init = luaL_Reg {
                name: ptr::null_mut() as *const c_char,
                func: None,
            };
            init
        },
    ]
};

#[no_mangle]
pub unsafe extern "C" fn luaopen_base(L: *mut lua_State) -> c_int {
    /* open lib into global table */
    lua_pushglobaltable(L);
    luaL_setfuncs(L, base_funcs.as_ptr(), 0);
    /* set global _G */
    lua_pushvalue(L, -1);
    lua_setfield(L, -2, cstr!("_G"));
    /* set global _VERSION */
    lua_pushliteral(L, "Lua 5.3");
    lua_setfield(L, -2, cstr!("_VERSION"));
    return 1;
}
