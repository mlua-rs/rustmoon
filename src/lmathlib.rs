use libc::{c_double, c_int, c_longlong, c_ulong, c_ulonglong, intptr_t, srand};
use rand::prelude::*;

use crate::lapi::{
    lua_compare, lua_gettop, lua_isinteger, lua_isnoneornil, lua_pushboolean, lua_pushinteger,
    lua_pushnil, lua_pushnumber, lua_pushvalue, lua_setfield, lua_tointeger, lua_tointegerx, lua_settop, lua_type, lua_pushstring
};
use crate::lauxlib::{
    luaL_Reg, luaL_argerror, luaL_checkany, luaL_checkinteger, luaL_checknumber, luaL_newlib,
    luaL_optnumber,
};
use crate::lstate::lua_State;
use crate::types::{lua_CFunction, lua_Integer, lua_Number, lua_Unsigned, LUA_TNUMBER};

pub const NULL: c_int = 0 as c_int;
pub const PI: c_double = 3.141592653589793238462643383279502884f64;
pub const __LONG_LONG_MAX__: c_longlong = 9223372036854775807;

pub const RAND_MAX: c_int = 2147483647 as c_int;
pub const L_RANDMAX: c_int = RAND_MAX;

pub const LUA_VERSION_NUM: c_int = 503 as c_int;
pub const LUAL_NUMSIZES: c_ulong = (::core::mem::size_of::<lua_Integer>() as c_ulong)
    .wrapping_mul(16 as c_int as c_ulong)
    .wrapping_add(::core::mem::size_of::<lua_Number>() as c_ulong);

pub const LLONG_MAX: c_longlong = __LONG_LONG_MAX__;
pub const LLONG_MIN: c_longlong = -__LONG_LONG_MAX__ - 1 as c_longlong;
pub const HUGE_VAL: c_double = ::core::f64::INFINITY;

pub const LUA_MININTEGER: c_longlong = LLONG_MIN;
pub const LUA_MAXINTEGER: c_longlong = LLONG_MAX;
pub const LUA_OPLT: c_int = 1 as c_int;

extern "C" {
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
    pub fn rand() -> c_double;

    pub fn luaL_error(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        args: ...
    ) -> c_int;
}

/* ** (The range comparisons are tricky because of rounding. The tests
** here assume a two-complement representation, where MININTEGER always
** has an exact representation as a float; MAXINTEGER may not have one,
** and therefore its conversion to float may have an ill-defined value.)
*/

#[inline(always)]
unsafe extern "C" fn lua_numbertointeger(n: lua_Number, p: *mut lua_Integer) -> bool {
    if n >= LUA_MININTEGER as f64 && n < -(LUA_MININTEGER as f64) {
        (*p) = n as lua_Integer;
        return true;
    }
    return false;
}

// FIXME static
#[no_mangle]
unsafe extern "C" fn pushnumint(L: *mut lua_State, d: lua_Number) {
    let n: *mut lua_Integer = Box::into_raw(Box::new(0));
    if lua_numbertointeger(d, n) {
        lua_pushinteger(L, *n);
    } else {
        lua_pushnumber(L, d);
    };
}

unsafe extern "C" fn math_floor(L: *mut lua_State) -> c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
    } else {
        let d = floor(luaL_checknumber(L, 1 as libc::c_int));
        pushnumint(L, d);
    }
    return 1;
}

unsafe extern "C" fn math_ceil(L: *mut lua_State) -> c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
    } else {
        let d = ceil(luaL_checknumber(L, 1 as libc::c_int));
        pushnumint(L, d);
    }
    return 1;
}

unsafe extern "C" fn math_type(L: *mut lua_State) -> c_int {
    if lua_type(L, 1 as libc::c_int) == LUA_TNUMBER {
        if lua_isinteger(L, 1 as libc::c_int) != 0 {
            lua_pushstring(L, cstr!("integer"));
        } else {
            lua_pushstring(L, cstr!("float"));
        }
    } else {
        luaL_checkany(L, 1 as libc::c_int);
        lua_pushnil(L);
    }
    return 1;
}

/*
** next function does not use 'modf', avoiding problems with 'double*'
** (which is not compatible with 'float*') when lua_Number is not
** 'double'.
*/

unsafe extern "C" fn math_modf(L: *mut lua_State) -> c_int {
    if lua_isinteger(L, 1) != 0 {
        lua_settop(L, 1);
        lua_pushnumber(L, 0.0);
    } else {
        let n = luaL_checknumber(L, 1);
        let ip: f64 = if n < 0.0 {
            ceil(n)
        } else {
            floor(n)
        };
        pushnumint(L, ip);
        lua_pushnumber(L, if n == ip { 0.0 } else { n - ip });
    }
    return 2;
}

unsafe extern "C" fn math_fmod(L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1) != 0 && lua_isinteger(L, 2 as libc::c_int) != 0 {
        let d = lua_tointeger(L, 2);
        if d == 0 {
            luaL_argerror(L, 2, cstr!("zero"));
        }
        if (d as lua_Unsigned).wrapping_add(1) <= 1 {
            lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
        } else {
            lua_pushinteger(L, lua_tointeger(L, 1) % d);
        }
    } else {
        lua_pushnumber(
            L,
            fmod(
                luaL_checknumber(L, 1),
                luaL_checknumber(L, 2),
            ),
        );
    }
    return 1;
}

/*
** This function uses 'double' (instead of 'lua_Number') to ensure that
** all bits from 'l_rand' can be represented, and that 'RANDMAX + 1.0'
** will keep full precision (ensuring that 'r' is always less than 1.0.)
*/
unsafe extern "C" fn math_random(L: *mut lua_State) -> libc::c_int {
    let low: lua_Integer;
    let up: lua_Integer;
    let mut r = rand::thread_rng().gen();
    match lua_gettop(L) {
        0 => {
            lua_pushnumber(L, r);
            return 1 as libc::c_int;
        }
        1 => {
            low = 1 as libc::c_int as lua_Integer;
            up = luaL_checkinteger(L, 1 as libc::c_int);
        }
        2 => {
            low = luaL_checkinteger(L, 1 as libc::c_int);
            up = luaL_checkinteger(L, 2 as libc::c_int);
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
            );
        }
    }

    /* random integer in the interval [low, up] */
    if !(low <= up) {
        luaL_argerror(L, 1, cstr!("interval is empty"));
        return 0;
    }
    if !(low >= 0 || up <= LUA_MAXINTEGER + low) {
        luaL_argerror(L, 1, cstr!("interval too large"));
        return 0;
    }

    r *= (up - low) as libc::c_double + 1.0f64;
    lua_pushinteger(L, r as lua_Integer + low);
    return 1 as libc::c_int;
}

unsafe extern "C" fn math_abs(L: *mut lua_State) -> c_int {
    if lua_isinteger(L, 1 as c_int) != 0 {
        let mut n = lua_tointeger(L, 1);
        if n < 0 as c_int as c_longlong {
            n = (0 as libc::c_uint as c_ulonglong).wrapping_sub(n as lua_Unsigned) as lua_Integer;
        }
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, fabs(luaL_checknumber(L, 1 as c_int)));
    }
    return 1 as c_int;
}

unsafe extern "C" fn math_sin(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, sin(luaL_checknumber(L, 1 as c_int)));
    return 1 as c_int;
}

unsafe extern "C" fn math_cos(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, cos(luaL_checknumber(L, 1 as c_int)));
    return 1 as c_int;
}

unsafe extern "C" fn math_tan(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, tan(luaL_checknumber(L, 1 as c_int)));
    return 1 as c_int;
}

unsafe extern "C" fn math_asin(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, asin(luaL_checknumber(L, 1 as c_int)));
    return 1 as c_int;
}

unsafe extern "C" fn math_acos(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, acos(luaL_checknumber(L, 1 as c_int)));
    return 1 as c_int;
}

unsafe extern "C" fn math_atan(L: *mut lua_State) -> c_int {
    let y = luaL_checknumber(L, 1 as c_int);
    let x = luaL_optnumber(L, 2 as c_int, 1 as c_int as lua_Number);
    lua_pushnumber(L, atan2(y, x));
    return 1 as c_int;
}

unsafe extern "C" fn math_toint(L: *mut lua_State) -> libc::c_int {
    let mut valid: libc::c_int = 0;
    let n = lua_tointegerx(L, 1 as libc::c_int, &mut valid);
    if valid != 0 {
        lua_pushinteger(L, n);
    } else {
        luaL_checkany(L, 1 as libc::c_int);
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}

unsafe extern "C" fn math_sqrt(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, sqrt(luaL_checknumber(L, 1 as c_int)));
    return 1 as c_int;
}

unsafe extern "C" fn math_ult(L: *mut lua_State) -> c_int {
    let a = luaL_checkinteger(L, 1 as c_int);
    let b = luaL_checkinteger(L, 2 as c_int);
    lua_pushboolean(L, ((a as lua_Unsigned) < b as lua_Unsigned) as c_int);
    return 1 as c_int;
}

unsafe extern "C" fn math_log(L: *mut lua_State) -> c_int {
    let x = luaL_checknumber(L, 1 as c_int);

    let res: lua_Number;
    if lua_isnoneornil(L, 2) {
        res = log(x);
    } else {
        let base = luaL_checknumber(L, 2 as c_int);
        if base == 2.0 {
            res = log2(x);
        } else if base == 10.0 {
            res = log10(x);
        } else {
            res = log(x) / log(base);
        }
    }
    lua_pushnumber(L, res as f64);
    return 1 as c_int;
}

unsafe extern "C" fn math_exp(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, exp(luaL_checknumber(L, 1 as c_int)));
    return 1 as c_int;
}

unsafe extern "C" fn math_deg(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, luaL_checknumber(L, 1 as c_int) * (180.0 / PI));
    return 1 as c_int;
}

unsafe extern "C" fn math_rad(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, luaL_checknumber(L, 1 as c_int) * (PI / 180.0));
    return 1 as c_int;
}

unsafe extern "C" fn math_min(L: *mut lua_State) -> c_int {
    let n = lua_gettop(L);
    if n < 1 {
        return luaL_argerror(
            L,
            1,
            b"value expected\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut imin = 1 as c_int;
    let mut i: c_int;
    i = 2 as c_int;
    while i <= n {
        if lua_compare(L, i, imin, LUA_OPLT) != 0 {
            imin = i;
        }
        i += 1;
    }
    lua_pushvalue(L, imin);
    return 1 as c_int;
}

unsafe extern "C" fn math_max(L: *mut lua_State) -> c_int {
    let n = lua_gettop(L);
    if n < 1 {
        return luaL_argerror(
            L,
            1,
            b"value expected\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut imax = 1 as c_int;
    let mut i: c_int;
    i = 2 as c_int;
    while i <= n {
        if lua_compare(L, imax, i, LUA_OPLT) != 0 {
            imax = i;
        }
        i += 1;
    }
    lua_pushvalue(L, imax);
    return 1 as c_int;
}

unsafe extern "C" fn math_randomseed(L: *mut lua_State) -> c_int {
    srand(luaL_checknumber(L, 1 as c_int) as lua_Integer as libc::c_uint);
    return 0 as c_int;
}

static mut mathlib: [luaL_Reg; 28] = unsafe {
    [
        {
            let init = luaL_Reg {
                name: b"abs\0" as *const u8 as *const libc::c_char,
                func: Some(math_abs as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"acos\0" as *const u8 as *const libc::c_char,
                func: Some(math_acos as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"asin\0" as *const u8 as *const libc::c_char,
                func: Some(math_asin as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"atan\0" as *const u8 as *const libc::c_char,
                func: Some(math_atan as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"ceil\0" as *const u8 as *const libc::c_char,
                func: Some(math_ceil as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"cos\0" as *const u8 as *const libc::c_char,
                func: Some(math_cos as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"deg\0" as *const u8 as *const libc::c_char,
                func: Some(math_deg as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"exp\0" as *const u8 as *const libc::c_char,
                func: Some(math_exp as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"tointeger\0" as *const u8 as *const libc::c_char,
                func: Some(math_toint as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"floor\0" as *const u8 as *const libc::c_char,
                func: Some(math_floor as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"fmod\0" as *const u8 as *const libc::c_char,
                func: Some(math_fmod as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"ult\0" as *const u8 as *const libc::c_char,
                func: Some(math_ult as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"log\0" as *const u8 as *const libc::c_char,
                func: Some(math_log as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"max\0" as *const u8 as *const libc::c_char,
                func: Some(math_max as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"min\0" as *const u8 as *const libc::c_char,
                func: Some(math_min as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"modf\0" as *const u8 as *const libc::c_char,
                func: Some(math_modf as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"rad\0" as *const u8 as *const libc::c_char,
                func: Some(math_rad as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"random\0" as *const u8 as *const libc::c_char,
                func: Some(math_random as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"randomseed\0" as *const u8 as *const libc::c_char,
                func: Some(math_randomseed as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"sin\0" as *const u8 as *const libc::c_char,
                func: Some(math_sin as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"sqrt\0" as *const u8 as *const libc::c_char,
                func: Some(math_sqrt as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"tan\0" as *const u8 as *const libc::c_char,
                func: Some(math_tan as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(math_type as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"pi\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<intptr_t, lua_CFunction>(NULL as intptr_t),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"huge\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<intptr_t, lua_CFunction>(NULL as intptr_t),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"maxinteger\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<intptr_t, lua_CFunction>(NULL as intptr_t),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"mininteger\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<intptr_t, lua_CFunction>(NULL as intptr_t),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<intptr_t, lua_CFunction>(NULL as intptr_t),
            };
            init
        },
    ]
};

#[no_mangle]
pub unsafe extern "C" fn luaopen_math(L: *mut lua_State) -> c_int {
    luaL_newlib(L, mathlib.as_ptr());
    lua_pushnumber(L, PI);
    lua_setfield(
        L,
        -(2 as c_int),
        b"pi\0" as *const u8 as *const libc::c_char,
    );
    lua_pushnumber(L, HUGE_VAL);
    lua_setfield(
        L,
        -(2 as c_int),
        b"huge\0" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(L, LUA_MAXINTEGER);
    lua_setfield(
        L,
        -(2 as c_int),
        b"maxinteger\0" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(L, LUA_MININTEGER);
    lua_setfield(
        L,
        -(2 as c_int),
        b"mininteger\0" as *const u8 as *const libc::c_char,
    );
    return 1 as c_int;
}

