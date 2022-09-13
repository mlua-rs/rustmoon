

use crate::lstate::lua_State;
use libc::{c_int, c_char};

type size_t = libc::c_ulong;
type lua_Integer = libc::c_longlong;

extern "C" {
    pub fn luaL_optinteger(
        L: *mut lua_State,
        arg: libc::c_int,
        def: lua_Integer,
    ) -> lua_Integer;
    pub fn luaL_checklstring(
        L: *mut lua_State,
        arg: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    pub fn luaL_argerror(
        L: *mut lua_State,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    pub fn luaL_error(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        args: ...
    ) -> libc::c_int;
    pub fn luaL_checkstack(
        L: *mut lua_State,
        space: libc::c_int,
        msg: *const libc::c_char,
    );
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    pub fn lua_pushnil(L: *mut lua_State);
}

/* from strlib */
/* translate a relative string position: negative means back from end */
// TODO static
#[no_mangle]
unsafe extern "C" fn u_posrelat(mut pos: lua_Integer, mut len: size_t) -> lua_Integer {
    if pos >= 0 as libc::c_int as libc::c_longlong {
        return pos
    } else if (0 as libc::c_uint as libc::c_ulong).wrapping_sub(pos as size_t) > len {
        return 0 as libc::c_int as lua_Integer
    } else {
        return len as lua_Integer + pos + 1 as libc::c_int as libc::c_longlong
    };
}

/*
** codepoint(s, [i, [j]])  -> returns codepoints for all characters
** that start in the range [i,j]
*/
// TODO static
#[no_mangle]
pub unsafe extern "C" fn codepoint(mut L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut posi: lua_Integer = u_posrelat(
        luaL_optinteger(L, 2 as libc::c_int, 1 as libc::c_int as lua_Integer),
        len,
    );
    let mut pose: lua_Integer = u_posrelat(
        luaL_optinteger(L, 3 as libc::c_int, posi),
        len,
    );
    let mut n: libc::c_int = 0;
    let mut se: *const libc::c_char = 0 as *const libc::c_char;
    (posi >= 1 as libc::c_int as libc::c_longlong
        || luaL_argerror(
            L,
            2 as libc::c_int,
            b"out of range\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    (pose <= len as lua_Integer
        || luaL_argerror(
            L,
            3 as libc::c_int,
            b"out of range\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    if posi > pose {
        return 0 as libc::c_int;
    }
    if pose - posi >= 2147483647 as libc::c_int as libc::c_longlong {
        return luaL_error(
            L,
            b"string slice too long\0" as *const u8 as *const libc::c_char,
        );
    }
    n = (pose - posi) as libc::c_int + 1 as libc::c_int;
    luaL_checkstack(
        L,
        n,
        b"string slice too long\0" as *const u8 as *const libc::c_char,
    );
    n = 0 as libc::c_int;
    se = s.offset(pose as isize);
    s = s.offset((posi - 1 as libc::c_int as libc::c_longlong) as isize);
    while s < se {
        let mut code: libc::c_int = 0;
        s = utf8_decode(s, &mut code);
        if s.is_null() {
            return luaL_error(
                L,
                b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
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
// TODO static
#[no_mangle]
unsafe extern "C" fn utf8_decode(
    mut o: *const libc::c_char,
    mut val: *mut libc::c_int,
) -> *const libc::c_char {
    static mut limits: [libc::c_uint; 4] = [
        0xff as libc::c_int as libc::c_uint,
        0x7f as libc::c_int as libc::c_uint,
        0x7ff as libc::c_int as libc::c_uint,
        0xffff as libc::c_int as libc::c_uint,
    ];
    let mut s: *const libc::c_uchar = o as *const libc::c_uchar;
    let mut c: libc::c_uint = *s.offset(0 as libc::c_int as isize) as libc::c_uint;
    let mut res: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if c < 0x80 as libc::c_int as libc::c_uint {
        res = c;
    } else {
        let mut count: libc::c_int = 0 as libc::c_int;
        while c & 0x40 as libc::c_int as libc::c_uint != 0 {
            count += 1;
            let mut cc: libc::c_int = *s.offset(count as isize) as libc::c_int;
            if cc & 0xc0 as libc::c_int != 0x80 as libc::c_int {
                return 0 as *const libc::c_char;
            }
            res = res << 6 as libc::c_int | (cc & 0x3f as libc::c_int) as libc::c_uint;
            c <<= 1 as libc::c_int;
        }
        res |= (c & 0x7f as libc::c_int as libc::c_uint) << count * 5 as libc::c_int;
        if count > 3 as libc::c_int || res > 0x10ffff as libc::c_int as libc::c_uint
            || res <= limits[count as usize]
        {
            return 0 as *const libc::c_char;
        }
        s = s.offset(count as isize);
    }
    if !val.is_null() {
        *val = res as libc::c_int;
    }
    return (s as *const libc::c_char).offset(1 as libc::c_int as isize);
}

/*
** utf8len(s [, i [, j]]) --> number of characters that start in the
** range [i,j], or nil + current position if 's' is not well formed in
** that interval
*/
// TODO static
#[no_mangle]
unsafe extern "C" fn utflen(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut posi: lua_Integer = u_posrelat(
        luaL_optinteger(L, 2 as libc::c_int, 1 as libc::c_int as lua_Integer),
        len,
    );
    let mut posj: lua_Integer = u_posrelat(
        luaL_optinteger(L, 3 as libc::c_int, -(1 as libc::c_int) as lua_Integer),
        len,
    );
    (1 as libc::c_int as libc::c_longlong <= posi
        && {
            posi -= 1;
            posi <= len as lua_Integer
        }
        || luaL_argerror(
            L,
            2 as libc::c_int,
            b"initial position out of string\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    posj -= 1;
    (posj < len as lua_Integer
        || luaL_argerror(
            L,
            3 as libc::c_int,
            b"final position out of string\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    while posi <= posj {
        let mut s1: *const libc::c_char = utf8_decode(
            s.offset(posi as isize),
            0 as *mut libc::c_int,
        );
        if s1.is_null() {
            lua_pushnil(L);
            lua_pushinteger(L, posi + 1 as libc::c_int as libc::c_longlong);
            return 2 as libc::c_int;
        }
        posi = s1.offset_from(s) as libc::c_long as lua_Integer;
        n += 1;
    }
    lua_pushinteger(L, n as lua_Integer);
    return 1 as libc::c_int;
}