use core::ffi::c_double;
use std::convert::TryInto;
use std::os::raw::c_int;

use libc::{size_t, srand};

use crate::lapi::{lua_pushnumber, lua_setfield, lua_version};
use crate::lauxlib::{luaL_Reg, luaL_newlib};
use crate::lobject::TValue;
use crate::lstate::lua_State;
use crate::lvm::tointeger;
use crate::types::{lua_CFunction, lua_Integer, lua_Number, lua_Unsigned};

pub const NULL: libc::c_int = 0 as libc::c_int;
pub const PI: libc::c_double = 3.141592653589793238462643383279502884f64;
pub const __LONG_LONG_MAX__: libc::c_longlong = 9223372036854775807;

pub const RAND_MAX: libc::c_int = 2147483647 as libc::c_int;
pub const L_RANDMAX: libc::c_int = RAND_MAX;

pub const LUA_VERSION_NUM: libc::c_int = 503 as libc::c_int;
pub const LUAL_NUMSIZES: libc::c_ulong = (::core::mem::size_of::<lua_Integer>() as libc::c_ulong)
    .wrapping_mul(16 as libc::c_int as libc::c_ulong)
    .wrapping_add(::core::mem::size_of::<lua_Number>() as libc::c_ulong);

pub const LLONG_MAX: libc::c_longlong = __LONG_LONG_MAX__;
pub const LLONG_MIN: libc::c_longlong = -__LONG_LONG_MAX__ - 1 as libc::c_longlong;
pub const HUGE_VAL: libc::c_double = ::core::f64::INFINITY;

pub const LUA_MININTEGER: libc::c_longlong = LLONG_MIN;
pub const LUA_MAXINTEGER: libc::c_longlong = LLONG_MAX;
pub const LUA_OPLT: libc::c_int = 1 as libc::c_int;

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

#[no_mangle]
pub unsafe extern "C" fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) {
    let v = lua_version(L);
    if sz != LUAL_NUMSIZES.try_into().unwrap() {
        luaL_error(
            L,
            b"core and library have incompatible numeric types\0" as *const u8
                as *const libc::c_char,
        );
    }
    if v != lua_version(NULL as *mut lua_State) {
        luaL_error(
            L,
            b"multiple Lua VMs detected\0" as *const u8 as *const libc::c_char,
        );
    } else if *v != ver {
        luaL_error(
            L,
            b"version mismatch: app. needs %f, Lua core provides %f\0" as *const u8
                as *const libc::c_char,
            ver,
            *v,
        );
    }
}

static mut mathlib: [luaL_Reg; 28] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"abs\0" as *const u8 as *const libc::c_char,
                func: Some(math_abs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"acos\0" as *const u8 as *const libc::c_char,
                func: Some(math_acos as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"asin\0" as *const u8 as *const libc::c_char,
                func: Some(math_asin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"atan\0" as *const u8 as *const libc::c_char,
                func: Some(math_atan as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ceil\0" as *const u8 as *const libc::c_char,
                func: Some(math_ceil as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"cos\0" as *const u8 as *const libc::c_char,
                func: Some(math_cos as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"deg\0" as *const u8 as *const libc::c_char,
                func: Some(math_deg as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"exp\0" as *const u8 as *const libc::c_char,
                func: Some(math_exp as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tointeger\0" as *const u8 as *const libc::c_char,
                func: Some(math_toint as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"floor\0" as *const u8 as *const libc::c_char,
                func: Some(math_floor as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fmod\0" as *const u8 as *const libc::c_char,
                func: Some(math_fmod as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ult\0" as *const u8 as *const libc::c_char,
                func: Some(math_ult as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"log\0" as *const u8 as *const libc::c_char,
                func: Some(math_log as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"max\0" as *const u8 as *const libc::c_char,
                func: Some(math_max as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"min\0" as *const u8 as *const libc::c_char,
                func: Some(math_min as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"modf\0" as *const u8 as *const libc::c_char,
                func: Some(math_modf as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rad\0" as *const u8 as *const libc::c_char,
                func: Some(math_rad as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"random\0" as *const u8 as *const libc::c_char,
                func: Some(math_random as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"randomseed\0" as *const u8 as *const libc::c_char,
                func: Some(math_randomseed as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sin\0" as *const u8 as *const libc::c_char,
                func: Some(math_sin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sqrt\0" as *const u8 as *const libc::c_char,
                func: Some(math_sqrt as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tan\0" as *const u8 as *const libc::c_char,
                func: Some(math_tan as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(math_type as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pi\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<libc::intptr_t, lua_CFunction>(
                    NULL as libc::intptr_t,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"huge\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<libc::intptr_t, lua_CFunction>(
                    NULL as libc::intptr_t,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"maxinteger\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<libc::intptr_t, lua_CFunction>(
                    NULL as libc::intptr_t,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"mininteger\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<libc::intptr_t, lua_CFunction>(
                    NULL as libc::intptr_t,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<libc::intptr_t, lua_CFunction>(
                    NULL as libc::intptr_t,
                ),
            };
            init
        },
    ]
};

/* /*
** Open math library
*/
LUAMOD_API int luaopen_math (lua_State *L) {
  luaL_newlib(L, mathlib);
  lua_pushnumber(L, PI);
  lua_setfield(L, -2, "pi");
  lua_pushnumber(L, (lua_Number)HUGE_VAL);
  lua_setfield(L, -2, "huge");
  lua_pushinteger(L, LUA_MAXINTEGER);
  lua_setfield(L, -2, "maxinteger");
  lua_pushinteger(L, LUA_MININTEGER);
  lua_setfield(L, -2, "mininteger");
  return 1;
} */

#[no_mangle]
pub unsafe extern "C" fn luaopen_math(L: *mut lua_State) -> libc::c_int {
    luaL_newlib(L, mathlib.as_ptr());
    lua_pushnumber(L, PI);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"pi\0" as *const u8 as *const libc::c_char,
    );
    lua_pushnumber(L, HUGE_VAL);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"huge\0" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(L, LUA_MAXINTEGER);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"maxinteger\0" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(L, LUA_MININTEGER);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"mininteger\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}

extern "C" {
    pub fn math_toint(L: *mut lua_State) -> libc::c_int;
    pub fn math_type(L: *mut lua_State) -> libc::c_int;
    pub fn math_random(L: *mut lua_State) -> libc::c_int;
}
