/*
** Coroutine Library
*/

use std::os::raw::c_int;
use std::ptr;

use crate::lapi::{
    lua_checkstack, lua_concat, lua_error, lua_gettop, lua_insert, lua_pop, lua_pushboolean,
    lua_pushcclosure, lua_pushliteral, lua_pushthread, lua_pushvalue, lua_status, lua_tothread,
    lua_type, lua_upvalueindex, lua_xmove,
};
use crate::lauxlib::{luaL_Reg, luaL_argcheck, luaL_checktype, luaL_newlib, luaL_where};
use crate::ldebug::lua_getstack;
use crate::ldo::{lua_isyieldable, lua_resume, lua_yield};
use crate::lstate::{lua_State, lua_newthread};
use crate::types::{lua_Debug, LUA_OK, LUA_TFUNCTION, LUA_TSTRING, LUA_YIELD};

unsafe fn getco(L: *mut lua_State) -> *mut lua_State {
    let co = lua_tothread(L, 1);
    luaL_argcheck(L, !co.is_null(), 1, cstr!("thread expected"));
    return co;
}

unsafe fn auxresume(L: *mut lua_State, co: *mut lua_State, narg: c_int) -> c_int {
    if lua_checkstack(co, narg) == 0 {
        lua_pushliteral(L, "too many arguments to resume");
        return -1; // error flag
    }
    if lua_status(co) == LUA_OK && lua_gettop(co) == 0 {
        lua_pushliteral(L, "cannot resume dead coroutine");
        return -1; // error flag
    }
    lua_xmove(L, co, narg);
    let status = lua_resume(co, L, narg);
    if status == LUA_OK || status == LUA_YIELD {
        let nres = lua_gettop(co);
        if lua_checkstack(L, nres + 1) == 0 {
            lua_pop(co, nres); // remove results anyway
            lua_pushliteral(L, "too many results to resume");
            return -1; // error flag
        }
        lua_xmove(co, L, nres); // move yielded values
        return nres;
    } else {
        lua_xmove(co, L, 1); // move error message
        return -1; // error flag
    };
}

unsafe extern "C" fn luaB_coresume(L: *mut lua_State) -> c_int {
    let co = getco(L);
    let r = auxresume(L, co, lua_gettop(L) - 1);
    if r < 0 {
        lua_pushboolean(L, 0);
        lua_insert(L, -2);
        2 // return false + error message
    } else {
        lua_pushboolean(L, 1);
        lua_insert(L, -(r + 1));
        r + 1 // return true + 'resume' returns
    }
}

unsafe extern "C" fn luaB_auxwrap(L: *mut lua_State) -> c_int {
    let co = lua_tothread(L, lua_upvalueindex(1));
    let r = auxresume(L, co, lua_gettop(L));
    if r < 0 {
        if lua_type(L, -1) == LUA_TSTRING {
            // error object is a string?
            luaL_where(L, 1); // add extra info
            lua_insert(L, -2);
            lua_concat(L, 2);
        }
        lua_error(L); // propagate error
    }
    r
}

unsafe extern "C" fn luaB_cocreate(L: *mut lua_State) -> c_int {
    luaL_checktype(L, 1, LUA_TFUNCTION);
    let new_thread = lua_newthread(L);
    lua_pushvalue(L, 1); // move function to top
    lua_xmove(L, new_thread, 1); // move function from L to new thread
    1
}

unsafe extern "C" fn luaB_cowrap(L: *mut lua_State) -> c_int {
    luaB_cocreate(L);
    lua_pushcclosure(L, Some(luaB_auxwrap), 1);
    1
}

unsafe extern "C" fn luaB_yield(L: *mut lua_State) -> c_int {
    lua_yield(L, lua_gettop(L))
}

unsafe extern "C" fn luaB_costatus(L: *mut lua_State) -> c_int {
    let co = getco(L);
    if L == co {
        lua_pushliteral(L, "running");
    } else {
        match lua_status(co) {
            LUA_YIELD => {
                lua_pushliteral(L, "suspended");
            }
            LUA_OK => {
                let mut ar = lua_Debug::new();
                if lua_getstack(co, 0, &mut ar) > 0 {
                    // does it have frames?
                    lua_pushliteral(L, "normal"); // it is running
                } else if lua_gettop(co) == 0 {
                    lua_pushliteral(L, "dead");
                } else {
                    lua_pushliteral(L, "suspended"); // initial state
                }
            }
            _ => {
                // some error occurred
                lua_pushliteral(L, "dead");
            }
        }
    }
    1
}

unsafe extern "C" fn luaB_yieldable(L: *mut lua_State) -> c_int {
    lua_pushboolean(L, lua_isyieldable(L));
    1
}

unsafe extern "C" fn luaB_corunning(L: *mut lua_State) -> c_int {
    let ismain = lua_pushthread(L);
    lua_pushboolean(L, ismain);
    2
}

const funcs: [luaL_Reg; 8] = [
    luaL_Reg::new(cstr!("create"), Some(luaB_cocreate)),
    luaL_Reg::new(cstr!("resume"), Some(luaB_coresume)),
    luaL_Reg::new(cstr!("running"), Some(luaB_corunning)),
    luaL_Reg::new(cstr!("status"), Some(luaB_costatus)),
    luaL_Reg::new(cstr!("wrap"), Some(luaB_cowrap)),
    luaL_Reg::new(cstr!("yield"), Some(luaB_yield)),
    luaL_Reg::new(cstr!("isyieldable"), Some(luaB_yieldable)),
    luaL_Reg::new(ptr::null(), None),
];

pub unsafe extern "C" fn luaopen_coroutine(L: *mut lua_State) -> c_int {
    luaL_newlib(L, funcs.as_ptr());
    1
}
