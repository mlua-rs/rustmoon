/*
** Standard mathematical library
*/

use std::f64::consts::{E, PI};
use std::os::raw::c_int;
use std::ptr;

use rand::Rng;

use crate::lapi::{
    lua_compare, lua_gettop, lua_isinteger, lua_numbertointeger, lua_pushboolean, lua_pushinteger,
    lua_pushnil, lua_pushnumber, lua_pushstring, lua_pushvalue, lua_setfield, lua_settop,
    lua_tointeger, lua_tointegerx, lua_type,
};
use crate::lauxlib::{
    luaL_Reg, luaL_argcheck, luaL_checkany, luaL_checkinteger, luaL_checknumber, luaL_error,
    luaL_newlib, luaL_optnumber, lua_isnoneornil,
};
use crate::lstate::lua_State;
use crate::types::{lua_Integer, lua_Number, lua_Unsigned, LUA_OPLT, LUA_TNUMBER};

unsafe extern "C" fn math_abs(L: *mut lua_State) -> c_int {
    if lua_isinteger(L, 1) != 0 {
        let mut n = lua_tointeger(L, 1);
        if n < 0 {
            n = 0u64.wrapping_sub(n as _) as _;
        }
        lua_pushinteger(L, n);
    } else {
        let n = luaL_checknumber(L, 1);
        lua_pushnumber(L, n.abs());
    }
    1
}

unsafe extern "C" fn math_sin(L: *mut lua_State) -> c_int {
    let n = luaL_checknumber(L, 1);
    lua_pushnumber(L, n.sin());
    1
}

unsafe extern "C" fn math_cos(L: *mut lua_State) -> c_int {
    let n = luaL_checknumber(L, 1);
    lua_pushnumber(L, n.cos());
    1
}

unsafe extern "C" fn math_tan(L: *mut lua_State) -> c_int {
    let n = luaL_checknumber(L, 1);
    lua_pushnumber(L, n.tan());
    1
}

unsafe extern "C" fn math_asin(L: *mut lua_State) -> c_int {
    let n = luaL_checknumber(L, 1);
    lua_pushnumber(L, n.asin());
    1
}

unsafe extern "C" fn math_acos(L: *mut lua_State) -> c_int {
    let n = luaL_checknumber(L, 1);
    lua_pushnumber(L, n.acos());
    1
}

unsafe extern "C" fn math_atan(L: *mut lua_State) -> c_int {
    let y = luaL_checknumber(L, 1);
    let x = luaL_optnumber(L, 2, 1.);
    lua_pushnumber(L, y.atan2(x));
    1
}

unsafe extern "C" fn math_toint(L: *mut lua_State) -> c_int {
    let mut valid = 0;
    let n = lua_tointegerx(L, 1, &mut valid);
    if valid != 0 {
        lua_pushinteger(L, n);
    } else {
        luaL_checkany(L, 1);
        lua_pushnil(L); // value is not convertible to integer
    }
    1
}

#[inline]
unsafe fn pushnumint(L: *mut lua_State, n: lua_Number) {
    if let Some(i) = lua_numbertointeger(n) {
        lua_pushinteger(L, i); // result is integer
    } else {
        lua_pushnumber(L, n); //  result is float
    }
}

unsafe extern "C" fn math_floor(L: *mut lua_State) -> c_int {
    if lua_isinteger(L, 1) != 0 {
        lua_settop(L, 1); // integer is its own floor
    } else {
        let n = luaL_checknumber(L, 1);
        pushnumint(L, n.floor());
    }
    1
}

unsafe extern "C" fn math_ceil(L: *mut lua_State) -> c_int {
    if lua_isinteger(L, 1) != 0 {
        lua_settop(L, 1); // integer is its own ceil
    } else {
        let n = luaL_checknumber(L, 1);
        pushnumint(L, n.ceil());
    }
    1
}

unsafe extern "C" fn math_fmod(L: *mut lua_State) -> c_int {
    if lua_isinteger(L, 1) != 0 && lua_isinteger(L, 2) != 0 {
        let d = lua_tointeger(L, 2);
        if (d as lua_Unsigned).wrapping_add(1) <= 1 {
            // special cases: -1 or 0
            luaL_argcheck(L, d != 0, 2, cstr!("zero"));
            lua_pushinteger(L, 0); // avoid overflow with 0x80000... / -1
        } else {
            lua_pushinteger(L, lua_tointeger(L, 1) % d);
        }
    } else {
        lua_pushnumber(L, luaL_checknumber(L, 1) % luaL_checknumber(L, 2));
    }
    1
}

unsafe extern "C" fn math_modf(L: *mut lua_State) -> c_int {
    if lua_isinteger(L, 1) != 0 {
        lua_settop(L, 1); // number is its own integer part
        lua_pushnumber(L, 0.0); // no fractional part
    } else {
        let n = luaL_checknumber(L, 1);
        // integer part (rounds toward zero)
        let ip = n.trunc();
        pushnumint(L, ip);
        // fractional part (test needed for inf/-inf)
        lua_pushnumber(L, if n == ip { 0. } else { n - ip });
    }
    2
}

unsafe extern "C" fn math_sqrt(L: *mut lua_State) -> c_int {
    let n = luaL_checknumber(L, 1);
    lua_pushnumber(L, n.sqrt());
    1
}

unsafe extern "C" fn math_ult(L: *mut lua_State) -> c_int {
    let a = luaL_checkinteger(L, 1);
    let b = luaL_checkinteger(L, 2);
    lua_pushboolean(L, ((a as lua_Unsigned) < (b as lua_Unsigned)) as _);
    1
}

unsafe extern "C" fn math_log(L: *mut lua_State) -> c_int {
    let x = luaL_checknumber(L, 1);
    let res = if lua_isnoneornil(L, 2) {
        x.log(E)
    } else {
        let base = luaL_checknumber(L, 2);
        match base {
            _ if base == 2. => x.log2(),
            _ if base == 10. => x.log10(),
            _ => x.log(E) / base.log(E),
        }
    };
    lua_pushnumber(L, res);
    1
}

unsafe extern "C" fn math_exp(L: *mut lua_State) -> c_int {
    let n = luaL_checknumber(L, 1);
    lua_pushnumber(L, n.exp());
    1
}

unsafe extern "C" fn math_deg(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, luaL_checknumber(L, 1) * (180.0 / PI));
    1
}

unsafe extern "C" fn math_rad(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, luaL_checknumber(L, 1) * (PI / 180.0));
    1
}

unsafe extern "C" fn math_min(L: *mut lua_State) -> c_int {
    let n = lua_gettop(L); // number of arguments
    luaL_argcheck(L, n >= 1, 1, cstr!("value expected"));
    let mut imin = 1; // index of current minimum value
    for i in 2..=n {
        if lua_compare(L, i, imin, LUA_OPLT) != 0 {
            imin = i;
        }
    }
    lua_pushvalue(L, imin);
    1
}

unsafe extern "C" fn math_max(L: *mut lua_State) -> c_int {
    let n = lua_gettop(L); // number of arguments
    luaL_argcheck(L, n >= 1, 1, cstr!("value expected"));
    let mut imax = 1; // index of current maximum value
    for i in 2..=n {
        if lua_compare(L, imax, i, LUA_OPLT) != 0 {
            imax = i;
        }
    }
    lua_pushvalue(L, imax);
    1
}

unsafe extern "C" fn math_random(L: *mut lua_State) -> c_int {
    let mut rng = rand::thread_rng();
    let (low, up) = match lua_gettop(L) {
        // check number of arguments
        0 => {
            lua_pushnumber(L, rng.gen()); // between 0 and 1
            return 1;
        }
        1 => (1, luaL_checkinteger(L, 1)), // only upper limit
        2 => (luaL_checkinteger(L, 1), luaL_checkinteger(L, 2)), // lower and upper limits
        _ => luaL_error(L, cstr!("wrong number of arguments")),
    };
    // random integer in the interval [low, up]
    luaL_argcheck(L, low <= up, 1, cstr!("interval is empty"));
    let ok = low >= 0 || up <= lua_Integer::MAX.wrapping_add(low);
    luaL_argcheck(L, ok, 1, cstr!("interval too large"));
    lua_pushinteger(L, rng.gen_range(low..=up));
    1
}

unsafe extern "C" fn math_randomseed(_L: *mut lua_State) -> c_int {
    // TODO: Use ReseedingRng
    0
}

unsafe extern "C" fn math_type(L: *mut lua_State) -> c_int {
    if lua_type(L, 1) == LUA_TNUMBER {
        if lua_isinteger(L, 1) != 0 {
            lua_pushstring(L, cstr!("integer"));
        } else {
            lua_pushstring(L, cstr!("float"));
        }
    } else {
        luaL_checkany(L, 1);
        lua_pushnil(L);
    }
    1
}

static mut mathlib: [luaL_Reg; 28] = [
    luaL_Reg {
        name: cstr!("abs"),
        func: Some(math_abs),
    },
    luaL_Reg {
        name: cstr!("acos"),
        func: Some(math_acos),
    },
    luaL_Reg {
        name: cstr!("asin"),
        func: Some(math_asin),
    },
    luaL_Reg {
        name: cstr!("atan"),
        func: Some(math_atan),
    },
    luaL_Reg {
        name: cstr!("ceil"),
        func: Some(math_ceil),
    },
    luaL_Reg {
        name: cstr!("cos"),
        func: Some(math_cos),
    },
    luaL_Reg {
        name: cstr!("deg"),
        func: Some(math_deg),
    },
    luaL_Reg {
        name: cstr!("exp"),
        func: Some(math_exp),
    },
    luaL_Reg {
        name: cstr!("tointeger"),
        func: Some(math_toint),
    },
    luaL_Reg {
        name: cstr!("floor"),
        func: Some(math_floor),
    },
    luaL_Reg {
        name: cstr!("fmod"),
        func: Some(math_fmod),
    },
    luaL_Reg {
        name: cstr!("ult"),
        func: Some(math_ult),
    },
    luaL_Reg {
        name: cstr!("log"),
        func: Some(math_log),
    },
    luaL_Reg {
        name: cstr!("max"),
        func: Some(math_max),
    },
    luaL_Reg {
        name: cstr!("min"),
        func: Some(math_min),
    },
    luaL_Reg {
        name: cstr!("modf"),
        func: Some(math_modf),
    },
    luaL_Reg {
        name: cstr!("rad"),
        func: Some(math_rad),
    },
    luaL_Reg {
        name: cstr!("random"),
        func: Some(math_random),
    },
    luaL_Reg {
        name: cstr!("randomseed"),
        func: Some(math_randomseed),
    },
    luaL_Reg {
        name: cstr!("sin"),
        func: Some(math_sin),
    },
    luaL_Reg {
        name: cstr!("sqrt"),
        func: Some(math_sqrt),
    },
    luaL_Reg {
        name: cstr!("tan"),
        func: Some(math_tan),
    },
    luaL_Reg {
        name: cstr!("type"),
        func: Some(math_type),
    },
    // placeholders for constants
    luaL_Reg {
        name: cstr!("pi"),
        func: None,
    },
    luaL_Reg {
        name: cstr!("huge"),
        func: None,
    },
    luaL_Reg {
        name: cstr!("maxinteger"),
        func: None,
    },
    luaL_Reg {
        name: cstr!("mininteger"),
        func: None,
    },
    luaL_Reg {
        name: ptr::null(),
        func: None,
    },
];

#[no_mangle]
pub unsafe extern "C" fn luaopen_math(L: *mut lua_State) -> c_int {
    luaL_newlib(L, mathlib.as_ptr());
    lua_pushnumber(L, PI);
    lua_setfield(L, -2, cstr!("pi"));
    lua_pushnumber(L, lua_Number::INFINITY);
    lua_setfield(L, -2, cstr!("huge"));
    lua_pushinteger(L, lua_Integer::MAX);
    lua_setfield(L, -2, cstr!("maxinteger"));
    lua_pushinteger(L, lua_Integer::MIN);
    lua_setfield(L, -2, cstr!("mininteger"));
    1
}
