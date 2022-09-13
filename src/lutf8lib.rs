

use crate::lstate::lua_State;
use libc::{c_int, c_char};

type size_t = libc::c_ulong;
type lua_Integer = libc::c_longlong;

extern "C" {
    pub fn utf8_decode(
        o: *const libc::c_char,
        val: *mut libc::c_int,
    ) -> *const libc::c_char;
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
