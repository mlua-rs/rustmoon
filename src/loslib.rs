use crate::lstate::lua_State;

use crate::lapi::{
    lua_createtable, lua_pushinteger, lua_pushstring, lua_setfield, lua_settop, lua_toboolean,
    lua_type,
};
use crate::lauxlib::{
    luaL_Buffer, luaL_Reg, luaL_argerror, luaL_buffinit, luaL_checkinteger, luaL_checkoption,
    luaL_checktype, luaL_checkversion_, luaL_error, luaL_optinteger, luaL_optlstring,
    luaL_pushresult, luaL_setfuncs,
};

use libc::{
    c_char, c_int, c_long, c_longlong, c_ulong, c_void, clock_t, close, difftime, exit, getenv,
    memcmp, memcpy, mkstemp, ptrdiff_t, remove, rename, setlocale, size_t, strcmp, strcpy, system,
    time,
};

use crate::lapi::{lua_getfield, lua_pushboolean, lua_pushfstring, lua_pushnumber, lua_tointegerx};
use crate::lauxlib::{luaL_checklstring, luaL_execresult, luaL_fileresult, luaL_prepbuffsize};
use crate::lstate::lua_close;
use crate::types::{lua_Integer, lua_Number};

pub const L_MAXDATEFIELD: libc::c_int = libc::INT_MAX / 2;
pub const LUA_STRFTIMEOPTIONS: [libc::c_char; 78] = unsafe {
    *::core::mem::transmute::<&[u8; 78], &[libc::c_char; 78]>(
        b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\0",
    )
};

extern "C" {
    fn clock() -> clock_t;
    fn strftime(_: *mut c_char, _: size_t, _: *const c_char, _: *const tm) -> size_t;
    fn gmtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
    fn localtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
    fn mktime(_: *mut tm) -> time_t;
}
pub type time_t = c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *mut c_char,
}

unsafe fn l_checktime(L: *mut lua_State, arg: c_int) -> time_t {
    let t: lua_Integer = luaL_checkinteger(L, arg);
    let _ = t == t || luaL_argerror(L, arg, cstr!("time out-of-bounds")) != 0;
    return t;
}
unsafe extern "C" fn os_execute(L: *mut lua_State) -> c_int {
    let cmd: *const c_char = luaL_optlstring(L, 1 as c_int, 0 as *const c_char, 0 as *mut usize);
    let stat: c_int = system(cmd);
    if !cmd.is_null() {
        return luaL_execresult(L, stat);
    } else {
        lua_pushboolean(L, stat);
        return 1;
    };
}
unsafe extern "C" fn os_remove(L: *mut lua_State) -> c_int {
    let filename: *const c_char = luaL_checklstring(L, 1, 0 as *mut size_t);
    return luaL_fileresult(L, (remove(filename) == 0) as c_int, filename);
}
unsafe extern "C" fn os_rename(L: *mut lua_State) -> c_int {
    let fromname: *const c_char = luaL_checklstring(L, 1, 0 as *mut size_t);
    let toname: *const c_char = luaL_checklstring(L, 2, 0 as *mut size_t);
    return luaL_fileresult(
        L,
        (rename(fromname, toname) == 0) as c_int,
        0 as *const c_char,
    );
}
unsafe extern "C" fn os_tmpname(L: *mut lua_State) -> c_int {
    let mut buff: [c_char; 32] = [0; 32];
    strcpy(buff.as_mut_ptr(), cstr!("/tmp/lua_XXXXXX"));
    let mut err: c_int = mkstemp(buff.as_mut_ptr());
    if err != -(1) {
        close(err);
    }
    err = (err == -(1)) as c_int;
    if err != 0 {
        luaL_error(L, cstr!("unable to generate a unique filename"));
    }
    lua_pushstring(L, buff.as_mut_ptr());
    return 1;
}
unsafe extern "C" fn os_getenv(L: *mut lua_State) -> c_int {
    lua_pushstring(L, getenv(luaL_checklstring(L, 1, 0 as *mut size_t)));
    return 1;
}
unsafe extern "C" fn os_clock(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, clock() as lua_Number / 1000000 as lua_Number);
    return 1;
}
unsafe fn setfield(L: *mut lua_State, key: *const c_char, value: c_int) {
    lua_pushinteger(L, value as lua_Integer);
    lua_setfield(L, -(2), key);
}
unsafe fn setboolfield(L: *mut lua_State, key: *const c_char, value: c_int) {
    if value < 0 {
        return;
    }
    lua_pushboolean(L, value);
    lua_setfield(L, -(2), key);
}
unsafe fn setallfields(L: *mut lua_State, stm: *mut tm) {
    setfield(L, cstr!("sec"), (*stm).tm_sec);
    setfield(L, cstr!("min"), (*stm).tm_min);
    setfield(L, cstr!("hour"), (*stm).tm_hour);
    setfield(L, cstr!("day"), (*stm).tm_mday);
    setfield(L, cstr!("month"), (*stm).tm_mon + 1);
    setfield(L, cstr!("year"), (*stm).tm_year + 1900);
    setfield(L, cstr!("wday"), (*stm).tm_wday + 1);
    setfield(L, cstr!("yday"), (*stm).tm_yday + 1);
    setboolfield(L, cstr!("isdst"), (*stm).tm_isdst);
}
unsafe fn getboolfield(L: *mut lua_State, key: *const c_char) -> c_int {
    let res: c_int = if lua_getfield(L, -(1), key) == 0 {
        -(1)
    } else {
        lua_toboolean(L, -(1))
    };
    lua_settop(L, -(1) - 1);
    return res;
}
unsafe fn getfield(L: *mut lua_State, key: *const c_char, d: c_int, delta: c_int) -> c_int {
    let mut isnum: c_int = 0;
    let t: c_int = lua_getfield(L, -(1), key);
    let mut res: lua_Integer = lua_tointegerx(L, -(1), &mut isnum);
    if isnum == 0 {
        if t != 0 {
            luaL_error(L, cstr!("field '%s' is not an integer"), key);
        } else {
            if d < 0 {
                luaL_error(L, cstr!("field '%s' missing in date table"), key);
            }
        }
        res = d as lua_Integer;
    } else {
        if !(i64::from(-L_MAXDATEFIELD) <= res && res <= L_MAXDATEFIELD as i64) {
            luaL_error(L, cstr!("field '%s' is out-of-bound"), key);
        }
        res -= delta as c_longlong;
    }
    lua_settop(L, -(1) - 1);
    return res as i32;
}
unsafe fn checkoption(
    L: *mut lua_State,
    conv: *const c_char,
    convlen: ptrdiff_t,
    buff: *mut c_char,
) -> *const c_char {
    let mut option = LUA_STRFTIMEOPTIONS.as_ptr();
    let mut oplen: c_int = 1;
    while *option as c_int != '\0' as i32 && oplen as ptrdiff_t <= convlen {
        if *option as c_int == '|' as i32 {
            oplen += 1;
        } else if memcmp(
            conv as *const c_void,
            option as *const c_void,
            oplen as size_t,
        ) == 0
        {
            memcpy(buff as *mut c_void, conv as *const c_void, oplen as usize);
            *buff.offset(oplen as isize) = '\0' as i32 as c_char;
            return conv.offset(oplen as isize);
        }
        option = option.offset(oplen as isize);
    }
    luaL_argerror(
        L,
        1,
        lua_pushfstring(L, cstr!("invalid conversion specifier '%%%s'"), conv),
    );
    return conv;
}
unsafe extern "C" fn os_date(L: *mut lua_State) -> c_int {
    let mut slen: usize = 0;
    let mut s: *const c_char = luaL_optlstring(L, 1, cstr!("%c"), &mut slen);
    let mut t: time_t = if lua_type(L, 2) <= 0 {
        time(0 as *mut time_t)
    } else {
        l_checktime(L, 2)
    };
    let se: *const c_char = s.offset(slen as isize);
    let mut tmr: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *mut c_char,
    };
    let stm;
    if *s as c_int == '!' as i32 {
        stm = gmtime_r(&mut t, &mut tmr);
        s = s.offset(1);
    } else {
        stm = localtime_r(&mut t, &mut tmr);
    }
    if stm.is_null() {
        luaL_error(
            L,
            cstr!("time result cannot be represented in this installation"),
        );
    }
    if strcmp(s, cstr!("*t")) == 0 {
        lua_createtable(L, 0, 9);
        setallfields(L, stm);
    } else {
        let mut cc: [c_char; 4] = [0; 4];
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            initb: [0; 8192],
        };
        cc[0 as usize] = '%' as i32 as c_char;
        luaL_buffinit(L, &mut b);
        while s < se {
            if *s as c_int != '%' as i32 {
                (b.n < b.size || !(luaL_prepbuffsize(&mut b, 1 as size_t)).is_null()) as c_int;
                let fresh0 = s;
                s = s.offset(1);
                let fresh1 = b.n;
                b.n = (b.n).wrapping_add(1);
                *(b.b).offset(fresh1 as isize) = *fresh0;
            } else {
                let buff: *mut c_char = luaL_prepbuffsize(&mut b, 250);
                s = s.offset(1);
                s = checkoption(
                    L,
                    s,
                    se.offset_from(s) as ptrdiff_t,
                    cc.as_mut_ptr().offset(1),
                );
                let reslen: usize = strftime(buff, 250, cc.as_mut_ptr(), stm);
                b.n = (b.n as c_ulong).wrapping_add(reslen as u64) as usize;
            }
        }
        luaL_pushresult(&mut b);
    }
    return 1;
}
unsafe extern "C" fn os_time(L: *mut lua_State) -> c_int {
    let t: time_t;
    if lua_type(L, 1) <= 0 {
        t = time(0 as *mut time_t);
    } else {
        let mut ts: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *mut c_char,
        };
        luaL_checktype(L, 1, 5);
        lua_settop(L, 1);
        ts.tm_sec = getfield(L, cstr!("sec"), 0, 0);
        ts.tm_min = getfield(L, cstr!("min"), 0, 0);
        ts.tm_hour = getfield(L, cstr!("hour"), 12, 0);
        ts.tm_mday = getfield(L, cstr!("day"), -(1), 0);
        ts.tm_mon = getfield(L, cstr!("month"), -(1), 1);
        ts.tm_year = getfield(L, cstr!("year"), -(1), 1900);
        ts.tm_isdst = getboolfield(L, cstr!("isdst"));
        t = mktime(&mut ts);
        setallfields(L, &mut ts);
    }
    if t != t || t == -(1) {
        luaL_error(
            L,
            cstr!("time result cannot be represented in this installation"),
        );
    }
    lua_pushinteger(L, t as lua_Integer);
    return 1;
}
unsafe extern "C" fn os_difftime(L: *mut lua_State) -> c_int {
    let t1: time_t = l_checktime(L, 1);
    let t2: time_t = l_checktime(L, 2);
    lua_pushnumber(L, difftime(t1, t2));
    return 1;
}
unsafe extern "C" fn os_setlocale(L: *mut lua_State) -> c_int {
    static mut cat: [c_int; 6] = [0, 1, 2, 3, 4, 5];
    static mut catnames: [*const c_char; 7] = [
        cstr!("all"),
        cstr!("collate"),
        cstr!("ctype"),
        cstr!("monetary"),
        cstr!("numeric"),
        cstr!("time"),
        0 as *const c_char,
    ];
    let l: *const c_char = luaL_optlstring(L, 1 as c_int, 0 as *const c_char, 0 as *mut usize);
    let op: c_int = luaL_checkoption(L, 2, cstr!("all"), catnames.as_ptr());
    lua_pushstring(L, setlocale(cat[op as usize], l));
    return 1;
}
unsafe extern "C" fn os_exit(L: *mut lua_State) -> c_int {
    let status: c_int;
    if lua_type(L, 1) == 1 {
        status = if lua_toboolean(L, 1) != 0 { 0 } else { 1 };
    } else {
        status = luaL_optinteger(L, 1, 0 as lua_Integer) as c_int;
    }
    if lua_toboolean(L, 2) != 0 {
        lua_close(L);
    }
    if !L.is_null() {
        exit(status);
    }
    return 0;
}
static mut syslib: [luaL_Reg; 12] = {
    [
        {
            let init = luaL_Reg {
                name: cstr!("clock"),
                func: Some(os_clock as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("date"),
                func: Some(os_date as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("difftime"),
                func: Some(os_difftime as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("execute"),
                func: Some(os_execute as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("exit"),
                func: Some(os_exit as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("getenv"),
                func: Some(os_getenv as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("remove"),
                func: Some(os_remove as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("rename"),
                func: Some(os_rename as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("setlocale"),
                func: Some(os_setlocale as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("time"),
                func: Some(os_time as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("tmpname"),
                func: Some(os_tmpname as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: 0 as *const c_char,
                func: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaopen_os(L: *mut lua_State) -> c_int {
    luaL_checkversion_(
        L,
        503 as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as usize)
            .wrapping_mul(16 as usize)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as usize),
    );
    lua_createtable(
        L,
        0,
        (::std::mem::size_of::<[luaL_Reg; 12]>())
            .wrapping_div(::std::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1) as c_int,
    );
    luaL_setfuncs(L, syslib.as_ptr(), 0);
    return 1;
}
