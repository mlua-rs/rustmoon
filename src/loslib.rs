use crate::lstate::lua_State;

use crate::lapi::{
    lua_createtable, lua_pushinteger, lua_pushstring, lua_settop, lua_toboolean, lua_type, lua_setfield
};
use crate::lauxlib::{
    luaL_Buffer, luaL_Reg, luaL_argerror, luaL_buffinit, luaL_checkinteger, luaL_checkoption,
    luaL_checktype, luaL_checkversion_, luaL_error, luaL_optinteger, luaL_optlstring,
    luaL_pushresult, luaL_setfuncs,
};

extern "C" {
    fn setlocale(_: libc::c_int, _: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn getenv(_: *const libc::c_char) -> *mut libc::c_char;
    fn system(_: *const libc::c_char) -> libc::c_int;
    fn mkstemp(_: *mut libc::c_char) -> libc::c_int;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn clock() -> clock_t;
    fn difftime(_: time_t, _: time_t) -> libc::c_double;
    fn mktime(_: *mut tm) -> time_t;
    fn strftime(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: *const tm) -> size_t;
    fn time(_: *mut time_t) -> time_t;
    fn gmtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
    fn localtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
    fn lua_close(L: *mut lua_State);
    fn lua_tointegerx(L: *mut lua_State, idx: libc::c_int, isnum: *mut libc::c_int) -> lua_Integer;
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> *const libc::c_char;
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> libc::c_int;
    fn luaL_checklstring(
        L: *mut lua_State,
        arg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn remove(_: *const libc::c_char) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn luaL_fileresult(
        L: *mut lua_State,
        stat: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_execresult(L: *mut lua_State, stat: libc::c_int) -> libc::c_int;
    fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut libc::c_char;
    fn close(_: libc::c_int) -> libc::c_int;
}
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_clock_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type size_t = __darwin_size_t;
pub type clock_t = __darwin_clock_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *mut libc::c_char,
}
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type lua_Number = libc::c_double;
pub type lua_Integer = libc::c_longlong;
pub type lua_CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;

unsafe extern "C" fn l_checktime(L: *mut lua_State, arg: libc::c_int) -> time_t {
    let t: lua_Integer = luaL_checkinteger(L, arg);
    (t as time_t as libc::c_longlong == t
        || luaL_argerror(
            L,
            arg,
            b"time out-of-bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    return t as time_t;
}
unsafe extern "C" fn os_execute(L: *mut lua_State) -> libc::c_int {
    let cmd: *const libc::c_char = luaL_optlstring(
        L,
        1 as libc::c_int,
        0 as *const libc::c_char,
        0 as *mut usize,
    );
    let stat: libc::c_int = system(cmd);
    if !cmd.is_null() {
        return luaL_execresult(L, stat);
    } else {
        lua_pushboolean(L, stat);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn os_remove(L: *mut lua_State) -> libc::c_int {
    let filename: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, 0 as *mut size_t);
    return luaL_fileresult(
        L,
        (remove(filename) == 0 as libc::c_int) as libc::c_int,
        filename,
    );
}
unsafe extern "C" fn os_rename(L: *mut lua_State) -> libc::c_int {
    let fromname: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, 0 as *mut size_t);
    let toname: *const libc::c_char = luaL_checklstring(L, 2 as libc::c_int, 0 as *mut size_t);
    return luaL_fileresult(
        L,
        (rename(fromname, toname) == 0 as libc::c_int) as libc::c_int,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn os_tmpname(L: *mut lua_State) -> libc::c_int {
    let mut buff: [libc::c_char; 32] = [0; 32];
    strcpy(
        buff.as_mut_ptr(),
        b"/tmp/lua_XXXXXX\0" as *const u8 as *const libc::c_char,
    );
    let mut err: libc::c_int = mkstemp(buff.as_mut_ptr());
    if err != -(1 as libc::c_int) {
        close(err);
    }
    err = (err == -(1 as libc::c_int)) as libc::c_int;
    if err != 0 {
        return luaL_error(
            L,
            b"unable to generate a unique filename\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_pushstring(L, buff.as_mut_ptr());
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_getenv(L: *mut lua_State) -> libc::c_int {
    lua_pushstring(
        L,
        getenv(luaL_checklstring(L, 1 as libc::c_int, 0 as *mut size_t)),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_clock(L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        clock() as lua_Number / 1000000 as libc::c_int as clock_t as lua_Number,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn setfield(L: *mut lua_State, key: *const libc::c_char, value: libc::c_int) {
    lua_pushinteger(L, value as lua_Integer);
    lua_setfield(L, -(2 as libc::c_int), key);
}
unsafe extern "C" fn setboolfield(L: *mut lua_State, key: *const libc::c_char, value: libc::c_int) {
    if value < 0 as libc::c_int {
        return;
    }
    lua_pushboolean(L, value);
    lua_setfield(L, -(2 as libc::c_int), key);
}
unsafe extern "C" fn setallfields(L: *mut lua_State, stm: *mut tm) {
    setfield(
        L,
        b"sec\0" as *const u8 as *const libc::c_char,
        (*stm).tm_sec,
    );
    setfield(
        L,
        b"min\0" as *const u8 as *const libc::c_char,
        (*stm).tm_min,
    );
    setfield(
        L,
        b"hour\0" as *const u8 as *const libc::c_char,
        (*stm).tm_hour,
    );
    setfield(
        L,
        b"day\0" as *const u8 as *const libc::c_char,
        (*stm).tm_mday,
    );
    setfield(
        L,
        b"month\0" as *const u8 as *const libc::c_char,
        (*stm).tm_mon + 1 as libc::c_int,
    );
    setfield(
        L,
        b"year\0" as *const u8 as *const libc::c_char,
        (*stm).tm_year + 1900 as libc::c_int,
    );
    setfield(
        L,
        b"wday\0" as *const u8 as *const libc::c_char,
        (*stm).tm_wday + 1 as libc::c_int,
    );
    setfield(
        L,
        b"yday\0" as *const u8 as *const libc::c_char,
        (*stm).tm_yday + 1 as libc::c_int,
    );
    setboolfield(
        L,
        b"isdst\0" as *const u8 as *const libc::c_char,
        (*stm).tm_isdst,
    );
}
unsafe extern "C" fn getboolfield(L: *mut lua_State, key: *const libc::c_char) -> libc::c_int {
    let res: libc::c_int = if lua_getfield(L, -(1 as libc::c_int), key) == 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        lua_toboolean(L, -(1 as libc::c_int))
    };
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return res;
}
unsafe extern "C" fn getfield(
    L: *mut lua_State,
    key: *const libc::c_char,
    d: libc::c_int,
    delta: libc::c_int,
) -> libc::c_int {
    let mut isnum: libc::c_int = 0;
    let t: libc::c_int = lua_getfield(L, -(1 as libc::c_int), key);
    let mut res: lua_Integer = lua_tointegerx(L, -(1 as libc::c_int), &mut isnum);
    if isnum == 0 {
        if t != 0 as libc::c_int {
            return luaL_error(
                L,
                b"field '%s' is not an integer\0" as *const u8 as *const libc::c_char,
                key,
            );
        } else {
            if d < 0 as libc::c_int {
                return luaL_error(
                    L,
                    b"field '%s' missing in date table\0" as *const u8 as *const libc::c_char,
                    key,
                );
            }
        }
        res = d as lua_Integer;
    } else {
        if !(-(2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_longlong <= res
            && res <= (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_longlong)
        {
            return luaL_error(
                L,
                b"field '%s' is out-of-bound\0" as *const u8 as *const libc::c_char,
                key,
            );
        }
        res -= delta as libc::c_longlong;
    }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return res as libc::c_int;
}
unsafe extern "C" fn checkoption(
    L: *mut lua_State,
    conv: *const libc::c_char,
    convlen: ptrdiff_t,
    buff: *mut libc::c_char,
) -> *const libc::c_char {
    let mut option: *const libc::c_char =
        b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\0"
            as *const u8 as *const libc::c_char;
    let mut oplen: libc::c_int = 1 as libc::c_int;
    while *option as libc::c_int != '\0' as i32 && oplen as libc::c_long <= convlen {
        if *option as libc::c_int == '|' as i32 {
            oplen += 1;
        } else if memcmp(
            conv as *const libc::c_void,
            option as *const libc::c_void,
            oplen as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            memcpy(
                buff as *mut libc::c_void,
                conv as *const libc::c_void,
                oplen as libc::c_ulong,
            );
            *buff.offset(oplen as isize) = '\0' as i32 as libc::c_char;
            return conv.offset(oplen as isize);
        }
        option = option.offset(oplen as isize);
    }
    luaL_argerror(
        L,
        1 as libc::c_int,
        lua_pushfstring(
            L,
            b"invalid conversion specifier '%%%s'\0" as *const u8 as *const libc::c_char,
            conv,
        ),
    );
    return conv;
}
unsafe extern "C" fn os_date(L: *mut lua_State) -> libc::c_int {
    let mut slen: usize = 0;
    let mut s: *const libc::c_char = luaL_optlstring(
        L,
        1 as libc::c_int,
        b"%c\0" as *const u8 as *const libc::c_char,
        &mut slen,
    );
    let mut t: time_t = if lua_type(L, 2 as libc::c_int) <= 0 as libc::c_int {
        time(0 as *mut time_t)
    } else {
        l_checktime(L, 2 as libc::c_int)
    };
    let se: *const libc::c_char = s.offset(slen as isize);
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
        tm_zone: 0 as *mut libc::c_char,
    };
    let stm;
    if *s as libc::c_int == '!' as i32 {
        stm = gmtime_r(&mut t, &mut tmr);
        s = s.offset(1);
    } else {
        stm = localtime_r(&mut t, &mut tmr);
    }
    if stm.is_null() {
        return luaL_error(
            L,
            b"time result cannot be represented in this installation\0" as *const u8
                as *const libc::c_char,
        );
    }
    if strcmp(s, b"*t\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        lua_createtable(L, 0 as libc::c_int, 9 as libc::c_int);
        setallfields(L, stm);
    } else {
        let mut cc: [libc::c_char; 4] = [0; 4];
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            initb: [0; 8192],
        };
        cc[0 as libc::c_int as usize] = '%' as i32 as libc::c_char;
        luaL_buffinit(L, &mut b);
        while s < se {
            if *s as libc::c_int != '%' as i32 {
                (b.n < b.size || !(luaL_prepbuffsize(&mut b, 1 as libc::c_int as size_t)).is_null())
                    as libc::c_int;
                let fresh0 = s;
                s = s.offset(1);
                let fresh1 = b.n;
                b.n = (b.n).wrapping_add(1);
                *(b.b).offset(fresh1 as isize) = *fresh0;
            } else {
                let buff: *mut libc::c_char =
                    luaL_prepbuffsize(&mut b, 250 as libc::c_int as size_t);
                s = s.offset(1);
                s = checkoption(
                    L,
                    s,
                    se.offset_from(s) as libc::c_long,
                    cc.as_mut_ptr().offset(1 as libc::c_int as isize),
                );
                let reslen: size_t =
                    strftime(buff, 250 as libc::c_int as size_t, cc.as_mut_ptr(), stm);
                b.n = (b.n as libc::c_ulong).wrapping_add(reslen) as usize;
            }
        }
        luaL_pushresult(&mut b);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_time(L: *mut lua_State) -> libc::c_int {
    let t: time_t;
    if lua_type(L, 1 as libc::c_int) <= 0 as libc::c_int {
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
            tm_zone: 0 as *mut libc::c_char,
        };
        luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
        lua_settop(L, 1 as libc::c_int);
        ts.tm_sec = getfield(
            L,
            b"sec\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        ts.tm_min = getfield(
            L,
            b"min\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        ts.tm_hour = getfield(
            L,
            b"hour\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int,
            0 as libc::c_int,
        );
        ts.tm_mday = getfield(
            L,
            b"day\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            0 as libc::c_int,
        );
        ts.tm_mon = getfield(
            L,
            b"month\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            1 as libc::c_int,
        );
        ts.tm_year = getfield(
            L,
            b"year\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            1900 as libc::c_int,
        );
        ts.tm_isdst = getboolfield(L, b"isdst\0" as *const u8 as *const libc::c_char);
        t = mktime(&mut ts);
        setallfields(L, &mut ts);
    }
    if t != t as lua_Integer as time_t || t == -(1 as libc::c_int) as time_t {
        return luaL_error(
            L,
            b"time result cannot be represented in this installation\0" as *const u8
                as *const libc::c_char,
        );
    }
    lua_pushinteger(L, t as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_difftime(L: *mut lua_State) -> libc::c_int {
    let t1: time_t = l_checktime(L, 1 as libc::c_int);
    let t2: time_t = l_checktime(L, 2 as libc::c_int);
    lua_pushnumber(L, difftime(t1, t2));
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_setlocale(L: *mut lua_State) -> libc::c_int {
    static mut cat: [libc::c_int; 6] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
    ];
    static mut catnames: [*const libc::c_char; 7] = [
        b"all\0" as *const u8 as *const libc::c_char,
        b"collate\0" as *const u8 as *const libc::c_char,
        b"ctype\0" as *const u8 as *const libc::c_char,
        b"monetary\0" as *const u8 as *const libc::c_char,
        b"numeric\0" as *const u8 as *const libc::c_char,
        b"time\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let l: *const libc::c_char = luaL_optlstring(
        L,
        1 as libc::c_int,
        0 as *const libc::c_char,
        0 as *mut usize,
    );
    let op: libc::c_int = luaL_checkoption(
        L,
        2 as libc::c_int,
        b"all\0" as *const u8 as *const libc::c_char,
        catnames.as_ptr(),
    );
    lua_pushstring(L, setlocale(cat[op as usize], l));
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_exit(L: *mut lua_State) -> libc::c_int {
    let status: libc::c_int;
    if lua_type(L, 1 as libc::c_int) == 1 as libc::c_int {
        status = if lua_toboolean(L, 1 as libc::c_int) != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    } else {
        status =
            luaL_optinteger(L, 1 as libc::c_int, 0 as libc::c_int as lua_Integer) as libc::c_int;
    }
    if lua_toboolean(L, 2 as libc::c_int) != 0 {
        lua_close(L);
    }
    if !L.is_null() {
        exit(status);
    }
    return 0 as libc::c_int;
}
static mut syslib: [luaL_Reg; 12] = {
    [
        {
            let init = luaL_Reg {
                name: b"clock\0" as *const u8 as *const libc::c_char,
                func: Some(os_clock as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"date\0" as *const u8 as *const libc::c_char,
                func: Some(os_date as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"difftime\0" as *const u8 as *const libc::c_char,
                func: Some(os_difftime as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"execute\0" as *const u8 as *const libc::c_char,
                func: Some(os_execute as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"exit\0" as *const u8 as *const libc::c_char,
                func: Some(os_exit as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"getenv\0" as *const u8 as *const libc::c_char,
                func: Some(os_getenv as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"remove\0" as *const u8 as *const libc::c_char,
                func: Some(os_remove as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"rename\0" as *const u8 as *const libc::c_char,
                func: Some(os_rename as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"setlocale\0" as *const u8 as *const libc::c_char,
                func: Some(os_setlocale as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"time\0" as *const u8 as *const libc::c_char,
                func: Some(os_time as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"tmpname\0" as *const u8 as *const libc::c_char,
                func: Some(os_tmpname as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaopen_os(L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(
        L,
        503 as libc::c_int as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as usize)
            .wrapping_mul(16 as libc::c_int as usize)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as usize),
    );
    lua_createtable(
        L,
        0 as libc::c_int,
        (::std::mem::size_of::<[luaL_Reg; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, syslib.as_ptr(), 0 as libc::c_int);
    return 1 as libc::c_int;
}
