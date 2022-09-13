

use crate::lstate::lua_State;
use crate::lauxlib::{luaL_Buffer, luaL_Reg};
use crate::types::lua_CFunction;
use libc::{c_int, c_char};

type size_t = libc::c_ulong;
type lua_Integer = libc::c_longlong;
type lua_Number = libc::c_double;

extern "C" {
    pub fn luaL_optinteger(
        L: *mut lua_State,
        arg: c_int,
        def: lua_Integer,
    ) -> lua_Integer;
    pub fn luaL_checklstring(
        L: *mut lua_State,
        arg: c_int,
        len: *mut size_t,
    ) -> *const c_char;
    pub fn luaL_argerror(
        L: *mut lua_State,
        arg: c_int,
        extramsg: *const c_char,
    ) -> c_int;
    pub fn luaL_error(
        L: *mut lua_State,
        fmt: *const c_char,
        args: ...
    ) -> c_int;
    pub fn luaL_checkstack(
        L: *mut lua_State,
        space: c_int,
        msg: *const c_char,
    );
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_gettop(L: *mut lua_State) -> c_int;
    pub fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    pub fn luaL_addvalue(B: *mut luaL_Buffer);
    pub fn luaL_pushresult(B: *mut luaL_Buffer);
    pub fn luaL_checkinteger(
        L: *mut lua_State,
        arg: c_int,
    ) -> lua_Integer;
    pub fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const c_char,
        args: ...
    ) -> *const c_char;
    pub fn lua_pushcclosure(
        L: *mut lua_State,
        fn_0: lua_CFunction,
        n: c_int,
    );
    pub fn lua_pushvalue(L: *mut lua_State, idx: c_int);
    pub fn lua_tointegerx(
        L: *mut lua_State,
        idx: c_int,
        pisnum: *mut c_int,
    ) -> lua_Integer;
    pub fn lua_createtable(
        L: *mut lua_State,
        narray: c_int,
        nrec: c_int,
    );
    pub fn lua_setfield(
        L: *mut lua_State,
        idx: libc::c_int,
        k: *const c_char,
    );
    pub fn luaL_setfuncs(
        L: *mut lua_State,
        l: *const luaL_Reg,
        nup: c_int,
    );
    pub fn lua_pushlstring(
        L: *mut lua_State,
        s: *const c_char,
        len: size_t,
    ) -> *const c_char;
    pub fn luaL_checkversion_(
        L: *mut lua_State,
        ver: lua_Number,
        sz: size_t,
    );
}

/* from strlib */
/* translate a relative string position: negative means back from end */
fn u_posrelat(pos: lua_Integer, len: size_t) -> lua_Integer {
    if pos >= 0 as c_int as libc::c_longlong {
        return pos
    } else if (0 as libc::c_uint as libc::c_ulong).wrapping_sub(pos as size_t) > len {
        return 0 as c_int as lua_Integer
    } else {
        return len as lua_Integer + pos + 1 as c_int as libc::c_longlong
    };
}

/*
** codepoint(s, [i, [j]])  -> returns codepoints for all characters
** that start in the range [i,j]
*/
unsafe extern "C" fn codepoint(L: *mut lua_State) -> c_int {
    let mut len: size_t = 0;
    let mut s: *const c_char = luaL_checklstring(L, 1 as c_int, &mut len);
    let posi: lua_Integer = u_posrelat(
        luaL_optinteger(L, 2 as c_int, 1 as c_int as lua_Integer),
        len,
    );
    let pose: lua_Integer = u_posrelat(
        luaL_optinteger(L, 3 as c_int, posi),
        len,
    );
    let mut n: c_int;
    let se: *const c_char;
    (posi >= 1 as c_int as libc::c_longlong
        || luaL_argerror(
            L,
            2 as c_int,
            b"out of range\0" as *const u8 as *const c_char,
        ) != 0) as c_int;
    (pose <= len as lua_Integer
        || luaL_argerror(
            L,
            3 as c_int,
            b"out of range\0" as *const u8 as *const c_char,
        ) != 0) as c_int;
    if posi > pose {
        return 0 as c_int;
    }
    if pose - posi >= 2147483647 as c_int as libc::c_longlong {
        return luaL_error(
            L,
            b"string slice too long\0" as *const u8 as *const c_char,
        );
    }
    n = (pose - posi) as c_int + 1 as c_int;
    luaL_checkstack(
        L,
        n,
        b"string slice too long\0" as *const u8 as *const c_char,
    );
    n = 0 as c_int;
    se = s.offset(pose as isize);
    s = s.offset((posi - 1 as c_int as libc::c_longlong) as isize);
    while s < se {
        let mut code: c_int = 0;
        s = utf8_decode(s, &mut code);
        if s.is_null() {
            return luaL_error(
                L,
                b"invalid UTF-8 code\0" as *const u8 as *const c_char,
            );
        }
        lua_pushinteger(L, code as lua_Integer);
        n += 1;
    }
    return n;
}

/*
** Decode one UTF-8 sequence, returning NULL if byte sequence is invalid.
*/
unsafe extern "C" fn utf8_decode(
    o: *const c_char,
    val: *mut c_int,
) -> *const c_char {
    static mut limits: [libc::c_uint; 4] = [
        0xff as c_int as libc::c_uint,
        0x7f as c_int as libc::c_uint,
        0x7ff as c_int as libc::c_uint,
        0xffff as c_int as libc::c_uint,
    ];
    let mut s: *const libc::c_uchar = o as *const libc::c_uchar;
    let mut c: libc::c_uint = *s.offset(0 as c_int as isize) as libc::c_uint;
    let mut res: libc::c_uint = 0 as c_int as libc::c_uint;
    if c < 0x80 as c_int as libc::c_uint {
        res = c;
    } else {
        let mut count: c_int = 0 as c_int;
        while c & 0x40 as c_int as libc::c_uint != 0 {
            count += 1;
            let cc: c_int = *s.offset(count as isize) as c_int;
            if cc & 0xc0 as c_int != 0x80 as c_int {
                return 0 as *const c_char;
            }
            res = res << 6 as c_int | (cc & 0x3f as c_int) as libc::c_uint;
            c <<= 1 as c_int;
        }
        res |= (c & 0x7f as c_int as libc::c_uint) << count * 5 as c_int;
        if count > 3 as c_int || res > 0x10ffff as c_int as libc::c_uint
            || res <= limits[count as usize]
        {
            return 0 as *const c_char;
        }
        s = s.offset(count as isize);
    }
    if !val.is_null() {
        *val = res as c_int;
    }
    return (s as *const c_char).offset(1 as c_int as isize);
}

/*
** utf8len(s [, i [, j]]) --> number of characters that start in the
** range [i,j], or nil + current position if 's' is not well formed in
** that interval
*/
unsafe extern "C" fn utflen(L: *mut lua_State) -> c_int {
    let mut n: c_int = 0 as c_int;
    let mut len: size_t = 0;
    let s: *const c_char = luaL_checklstring(L, 1 as c_int, &mut len);
    let mut posi: lua_Integer = u_posrelat(
        luaL_optinteger(L, 2 as c_int, 1 as c_int as lua_Integer),
        len,
    );
    let mut posj: lua_Integer = u_posrelat(
        luaL_optinteger(L, 3 as c_int, -(1 as c_int) as lua_Integer),
        len,
    );
    (1 as c_int as libc::c_longlong <= posi
        && {
            posi -= 1;
            posi <= len as lua_Integer
        }
        || luaL_argerror(
            L,
            2 as c_int,
            b"initial position out of string\0" as *const u8 as *const c_char,
        ) != 0) as c_int;
    posj -= 1;
    (posj < len as lua_Integer
        || luaL_argerror(
            L,
            3 as c_int,
            b"final position out of string\0" as *const u8 as *const c_char,
        ) != 0) as c_int;
    while posi <= posj {
        let s1: *const c_char = utf8_decode(
            s.offset(posi as isize),
            0 as *mut c_int,
        );
        if s1.is_null() {
            lua_pushnil(L);
            lua_pushinteger(L, posi + 1 as c_int as libc::c_longlong);
            return 2 as c_int;
        }
        posi = s1.offset_from(s) as libc::c_long as lua_Integer;
        n += 1;
    }
    lua_pushinteger(L, n as lua_Integer);
    return 1 as c_int;
}


/*
** utfchar(n1, n2, ...)  -> char(n1)..char(n2)...
*/
unsafe extern "C" fn utfchar(L: *mut lua_State) -> c_int {
    let n: c_int = lua_gettop(L);
    if n == 1 as c_int {
        pushutfchar(L, 1 as c_int);
    } else {
        let mut i: c_int;
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            initb: [0; 8192],
        };
        luaL_buffinit(L, &mut b);
        i = 1 as c_int;
        while i <= n {
            pushutfchar(L, i);
            luaL_addvalue(&mut b);
            i += 1;
        }
        luaL_pushresult(&mut b);
    }
    return 1 as c_int;
}

unsafe extern "C" fn pushutfchar(L: *mut lua_State, arg: c_int) {
    let code: lua_Integer = luaL_checkinteger(L, arg);
    (0 as c_int as libc::c_longlong <= code
        && code <= 0x10ffff as c_int as libc::c_longlong
        || luaL_argerror(
            L,
            arg,
            b"value out of range\0" as *const u8 as *const c_char,
        ) != 0) as c_int;
    lua_pushfstring(
        L,
        b"%U\0" as *const u8 as *const c_char,
        code as libc::c_long,
    );
}


/*
** offset(s, n, [i])  -> index where n-th character counting from
**   position 'i' starts; 0 means character at 'i'.
*/
unsafe extern "C" fn byteoffset(L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut n: lua_Integer = luaL_checkinteger(L, 2 as libc::c_int);
    let mut posi: lua_Integer = (if n >= 0 as libc::c_int as libc::c_longlong {
        1 as libc::c_int as libc::c_ulong
    } else {
        len.wrapping_add(1 as libc::c_int as libc::c_ulong)
    }) as lua_Integer;
    posi = u_posrelat(luaL_optinteger(L, 3 as libc::c_int, posi), len);
    (1 as libc::c_int as libc::c_longlong <= posi
        && {
            posi -= 1;
            posi <= len as lua_Integer
        }
        || luaL_argerror(
            L,
            3 as libc::c_int,
            b"position out of range\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    if n == 0 as libc::c_int as libc::c_longlong {
        while posi > 0 as libc::c_int as libc::c_longlong
            && *s.offset(posi as isize) as libc::c_int & 0xc0 as libc::c_int
                == 0x80 as libc::c_int
        {
            posi -= 1;
        }
    } else {
        if *s.offset(posi as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        {
            return luaL_error(
                L,
                b"initial position is a continuation byte\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if n < 0 as libc::c_int as libc::c_longlong {
            while n < 0 as libc::c_int as libc::c_longlong
                && posi > 0 as libc::c_int as libc::c_longlong
            {
                loop {
                    posi -= 1;
                    if !(posi > 0 as libc::c_int as libc::c_longlong
                        && *s.offset(posi as isize) as libc::c_int & 0xc0 as libc::c_int
                            == 0x80 as libc::c_int)
                    {
                        break;
                    }
                }
                n += 1;
            }
        } else {
            n -= 1;
            while n > 0 as libc::c_int as libc::c_longlong && posi < len as lua_Integer {
                loop {
                    posi += 1;
                    if !(*s.offset(posi as isize) as libc::c_int & 0xc0 as libc::c_int
                        == 0x80 as libc::c_int)
                    {
                        break;
                    }
                }
                n -= 1;
            }
        }
    }
    if n == 0 as libc::c_int as libc::c_longlong {
        lua_pushinteger(L, posi + 1 as libc::c_int as libc::c_longlong);
    } else {
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}

unsafe extern "C" fn iter_aux(L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut n: lua_Integer = lua_tointegerx(L, 2 as libc::c_int, 0 as *mut libc::c_int)
        - 1 as libc::c_int as libc::c_longlong;
    if n < 0 as libc::c_int as libc::c_longlong {
        n = 0 as libc::c_int as lua_Integer;
    } else if n < len as lua_Integer {
        n += 1;
        while *s.offset(n as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        {
            n += 1;
        }
    }
    if n >= len as lua_Integer {
        return 0 as libc::c_int
    } else {
        let mut code: libc::c_int = 0;
        let next: *const libc::c_char = utf8_decode(s.offset(n as isize), &mut code);
        if next.is_null()
            || *next as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int
        {
            return luaL_error(
                L,
                b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
            );
        }
        lua_pushinteger(L, n + 1 as libc::c_int as libc::c_longlong);
        lua_pushinteger(L, code as lua_Integer);
        return 2 as libc::c_int;
    };
}

unsafe extern "C" fn iter_codes(L: *mut lua_State) -> libc::c_int {
    luaL_checklstring(L, 1 as libc::c_int, 0 as *mut size_t);
    lua_pushcclosure(
        L,
        Some(iter_aux as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_pushvalue(L, 1 as libc::c_int);
    lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
    return 3 as libc::c_int;
}

static mut funcs: [luaL_Reg; 7] = {
    [
        {
            let init = luaL_Reg {
                name: b"offset\0" as *const u8 as *const libc::c_char,
                func: Some(
                    byteoffset as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"codepoint\0" as *const u8 as *const libc::c_char,
                func: Some(
                    codepoint as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"char\0" as *const u8 as *const libc::c_char,
                func: Some(
                    utfchar as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"len\0" as *const u8 as *const libc::c_char,
                func: Some(utflen as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"codes\0" as *const u8 as *const libc::c_char,
                func: Some(
                    iter_codes as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"charpattern\0" as *const u8 as *const libc::c_char,
                func: None,
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
pub unsafe extern "C" fn luaopen_utf8(L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(
        L,
        503 as libc::c_int as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0 as libc::c_int,
        (::std::mem::size_of::<[luaL_Reg; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, funcs.as_ptr(), 0 as libc::c_int);
    lua_pushlstring(
        L,
        b"[\0-\x7F\xC2-\xF4][\x80-\xBF]*\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"charpattern\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}