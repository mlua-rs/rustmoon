use core::ffi::c_double;
use std::os::raw::c_int;

use libc::srand;

use crate::lapi::lua_pushnumber;
use crate::lobject::TValue;
use crate::lstate::lua_State;
use crate::lvm::tointeger;
use crate::types::{lua_Integer, lua_Number, lua_Unsigned};

pub const PI: libc::c_double = 3.141592653589793238462643383279502884f64;

pub const LUA_OPLT: libc::c_int = 1 as libc::c_int;
pub const LUAL_NUMSIZES: libc::c_ulong = (::core::mem::size_of::<lua_Integer>() as libc::c_ulong)
    .wrapping_mul(16 as libc::c_int as libc::c_ulong)
    .wrapping_add(::core::mem::size_of::<lua_Number>() as libc::c_ulong);

extern "C" {
    pub fn index2addr(L: *mut lua_State, idx: libc::c_int) -> *mut TValue;
    pub fn pushnumint(L: *mut lua_State, d: lua_Number);
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    pub fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    pub fn luaL_checknumber(L: *mut lua_State, arg: libc::c_int) -> lua_Number;
    pub fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    pub fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, args: ...) -> libc::c_int;
    pub fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    pub fn lua_compare(
        L: *mut lua_State,
        index1: libc::c_int,
        index2: libc::c_int,
        op: libc::c_int,
    ) -> libc::c_int;
    pub fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    pub fn lua_isinteger(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    pub fn luaL_optnumber(L: *mut lua_State, arg: libc::c_int, def: lua_Number) -> lua_Number;
    pub fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;

    pub fn sin(x: c_double) -> c_double;
    pub fn cos(x: c_double) -> c_double;
    pub fn tan(x: c_double) -> c_double;
    pub fn acos(x: c_double) -> c_double;
    pub fn asin(x: c_double) -> c_double;
    pub fn atan2(x: c_double, y: c_double) -> c_double;
    pub fn ceil(x: c_double) -> c_double;
    pub fn floor(x: c_double) -> c_double;
    pub fn exp(x: c_double) -> c_double;
    pub fn fabs(x: c_double) -> c_double;
    pub fn sqrt(x: c_double) -> c_double;
    pub fn fmod(x: c_double, y: c_double) -> c_double;

    pub fn log(x: c_double) -> c_double;
    pub fn log2(x: c_double) -> c_double;
    pub fn log10(x: c_double) -> c_double;
}

#[inline(always)]
pub unsafe fn lua_tointeger(L: *mut lua_State, idx: libc::c_int) -> lua_Integer {
    let mut res: lua_Integer = 0;
    let o: *const TValue = index2addr(L, idx);
    let isnum = tointeger(o, &mut res);
    if isnum == 0 {
        res = 0 as libc::c_int as lua_Integer;
    }
    return res;
}

#[inline(always)]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, n: c_int) -> bool {
    return lua_type(L, n) <= 0;
}

#[no_mangle]
pub unsafe extern "C" fn math_abs(L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        let mut n = lua_tointeger(L, 1);
        if n < 0 as libc::c_int as libc::c_longlong {
            n = (0 as libc::c_uint as libc::c_ulonglong).wrapping_sub(n as lua_Unsigned)
                as lua_Integer;
        }
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, fabs(luaL_checknumber(L, 1 as libc::c_int)));
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_sin(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, sin(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_cos(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, cos(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_tan(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, tan(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_asin(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, asin(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_acos(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, acos(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_atan(L: *mut lua_State) -> libc::c_int {
    let y = luaL_checknumber(L, 1 as libc::c_int);
    let x = luaL_optnumber(L, 2 as libc::c_int, 1 as libc::c_int as lua_Number);
    lua_pushnumber(L, atan2(y, x));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_floor(L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
    } else {
        let d = floor(luaL_checknumber(L, 1 as libc::c_int));
        pushnumint(L, d);
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_ceil(L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
    } else {
        let d = ceil(luaL_checknumber(L, 1 as libc::c_int));
        pushnumint(L, d);
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_fmod(L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 && lua_isinteger(L, 2 as libc::c_int) != 0 {
        let d = lua_tointeger(L, 2);
        if (d as lua_Unsigned).wrapping_add(1 as libc::c_uint as libc::c_ulonglong)
            <= 1 as libc::c_uint as libc::c_ulonglong
        {
            lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
        } else {
            lua_pushinteger(L, lua_tointeger(L, 1) % d);
        }
    } else {
        lua_pushnumber(
            L,
            fmod(
                luaL_checknumber(L, 1 as libc::c_int),
                luaL_checknumber(L, 2 as libc::c_int),
            ),
        );
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_modf(L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
        lua_pushnumber(L, 0 as libc::c_int as lua_Number);
    } else {
        let n = luaL_checknumber(L, 1 as libc::c_int);
        let ip = if n < 0 as libc::c_int as libc::c_double {
            ceil(n)
        } else {
            floor(n)
        };
        pushnumint(L, ip);
        lua_pushnumber(L, if n == ip { 0.0 } else { n - ip });
    }
    return 2 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_sqrt(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, sqrt(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_ult(L: *mut lua_State) -> libc::c_int {
    let a = luaL_checkinteger(L, 1 as libc::c_int);
    let b = luaL_checkinteger(L, 2 as libc::c_int);
    lua_pushboolean(L, ((a as lua_Unsigned) < b as lua_Unsigned) as libc::c_int);
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_log(L: *mut lua_State) -> libc::c_int {
    let x = luaL_checknumber(L, 1 as libc::c_int);

    let res: lua_Number;
    if lua_isnoneornil(L, 2) {
        res = log(x);
    } else {
        let base = luaL_checknumber(L, 2 as libc::c_int);
        if base == 2.0 {
            res = log2(x);
        } else if base == 10.0 {
            res = log10(x);
        } else {
            res = log(x) / log(base);
        }
    }
    lua_pushnumber(L, res as f64);
    return 1 as libc::c_int;
}

// #[no_mangle]
pub unsafe extern "C" fn math_exp(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, exp(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_deg(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, luaL_checknumber(L, 1 as libc::c_int) * (180.0 / PI));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_rad(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, luaL_checknumber(L, 1 as libc::c_int) * (PI / 180.0));
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_min(L: *mut lua_State) -> libc::c_int {
    let n = lua_gettop(L);
    let mut imin = 1 as libc::c_int;
    let mut i: libc::c_int;
    i = 2 as libc::c_int;
    while i <= n {
        if lua_compare(L, i, imin, LUA_OPLT) != 0 {
            imin = i;
        }
        i += 1;
    }
    lua_pushvalue(L, imin);
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_max(L: *mut lua_State) -> libc::c_int {
    let n = lua_gettop(L);
    let mut imax = 1 as libc::c_int;
    let mut i: libc::c_int;
    i = 2 as libc::c_int;
    while i <= n {
        if lua_compare(L, imax, i, LUA_OPLT) != 0 {
            imax = i;
        }
        i += 1;
    }
    lua_pushvalue(L, imax);
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn math_randomseed(L: *mut lua_State) -> libc::c_int {
    srand(luaL_checknumber(L, 1 as libc::c_int) as lua_Integer as libc::c_uint);
    return 0 as libc::c_int;
}
