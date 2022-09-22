/*
** Standard library for UTF-8 manipulation
*/

use std::os::raw::{c_char, c_int};
use std::ptr;
use std::slice;

use crate::lapi::{
    lua_gettop, lua_pushcfunction, lua_pushfstring, lua_pushinteger, lua_pushlstring, lua_pushnil,
    lua_pushvalue, lua_setfield, lua_tointeger,
};
use crate::lauxlib::{
    luaL_Buffer, luaL_Reg, luaL_addvalue, luaL_argcheck, luaL_buffinit, luaL_checkinteger,
    luaL_checklstring, luaL_checkstack, luaL_error, luaL_newlib, luaL_optinteger, luaL_pushresult,
};
use crate::lstate::lua_State;
use crate::types::lua_Integer;

const MAXUNICODE: u32 = 0x10FFFF;

const fn iscont(p: u8) -> bool {
    p & 0xC0 == 0x80
}

/* from strlib */
/* translate a relative string position: negative means back from end */
const fn u_posrelat(pos: lua_Integer, len: usize) -> lua_Integer {
    if pos >= 0 {
        return pos;
    }
    if 0usize.wrapping_sub(pos as usize) > len {
        return 0;
    }
    return len as lua_Integer + pos + 1;
}

/// Decode one UTF-8 sequence, returning `None` if byte sequence is invalid.
fn utf8_decode2(s: &[u8]) -> Option<(char, usize)> {
    const limits: [u32; 4] = [0xff, 0x7f, 0x7ff, 0xffff];
    let mut c = s[0] as u32;
    let mut res = 0u32; // final result
    let mut count = 0; // to count number of continuation bytes
    if c < 0x80 {
        res = c; // ascii
    } else {
        while c & 0x40 != 0 {
            // still have continuation bytes?
            count += 1;
            let cc = if count == s.len() { 0 } else { s[count] as u32 }; // read next byte
            if cc & 0xc0 != 0x80 {
                // not a continuation byte?
                return None; // invalid byte sequence
            }
            res = (res << 6) | (cc & 0x3f); // add lower 6 bits from cont. byte
            c <<= 1; // to test next bit
        }
        res |= (c & 0x7f) << (count * 5); // add first byte
        if count > 3 || res > MAXUNICODE || res <= limits[count] {
            return None; // invalid byte sequence
        }
    }
    let res = unsafe { char::from_u32_unchecked(res) };
    return Some((res, count + 1)); // +1 to include first byte
}

/*
** utf8len(s [, i [, j]]) --> number of characters that start in the
** range [i,j], or nil + current position if 's' is not well formed in
** that interval
*/
unsafe extern "C" fn utflen(L: *mut lua_State) -> c_int {
    let mut len = 0;
    let s = luaL_checklstring(L, 1, &mut len);

    let mut posi = u_posrelat(luaL_optinteger(L, 2, 1), len);
    let mut posj: lua_Integer = u_posrelat(luaL_optinteger(L, 3, -1), len);
    let posi_ok = 1 <= posi && posi - 1 <= (len as lua_Integer);
    posi -= 1;
    luaL_argcheck(L, posi_ok, 2, cstr!("initial position out of string"));
    posj -= 1;
    let posj_ok = posj < (len as lua_Integer);
    luaL_argcheck(L, posj_ok, 3, cstr!("final position out of string"));

    let s = slice::from_raw_parts(s as *const u8, len);
    let mut n = 0;
    while posi <= posj {
        match utf8_decode2(&s[posi as usize..]) {
            Some((_, len)) => {
                posi += len as i64;
            }
            None => {
                // conversion error?
                lua_pushnil(L); // return nil ...
                lua_pushinteger(L, posi + 1); // ... and current position
                return 2;
            }
        }
        n += 1;
    }
    lua_pushinteger(L, n);
    1
}

/*
** codepoint(s, [i, [j]])  -> returns codepoints for all characters
** that start in the range [i,j]
*/
unsafe extern "C" fn codepoint(L: *mut lua_State) -> c_int {
    let mut len = 0;
    let s = luaL_checklstring(L, 1, &mut len);
    let posi = u_posrelat(luaL_optinteger(L, 2, 1), len);
    let pose = u_posrelat(luaL_optinteger(L, 3, posi), len);
    luaL_argcheck(L, posi >= 1, 2, cstr!("out of range"));
    luaL_argcheck(L, pose <= len as lua_Integer, 3, cstr!("out of range"));
    if posi > pose {
        return 0; // empty interval; return no values
    }
    if pose - posi >= c_int::MAX as _ {
        luaL_error(L, cstr!("string slice too long")); // overflow?
    }
    let mut n = (pose - posi) as c_int + 1;
    luaL_checkstack(L, n, cstr!("string slice too long"));

    let mut s = slice::from_raw_parts(s as *const u8, len);
    s = &s[posi as usize - 1..];
    let mut count = 0;
    while n > 0 {
        match utf8_decode2(s) {
            Some((code, idx)) => {
                s = &s[idx..];
                n -= idx as i32;
                lua_pushinteger(L, code as _);
                count += 1;
            }
            None => {
                luaL_error(L, cstr!("invalid UTF-8 code"));
            }
        }
    }
    count
}

unsafe fn pushutfchar(L: *mut lua_State, arg: c_int) {
    let code = luaL_checkinteger(L, arg);
    let ok = 0 <= code && code <= MAXUNICODE as _;
    luaL_argcheck(L, ok, arg, cstr!("value out of range"));
    lua_pushfstring(L, cstr!("%U"), code);
}

/*
** utfchar(n1, n2, ...)  -> char(n1)..char(n2)...
*/
unsafe extern "C" fn utfchar(L: *mut lua_State) -> c_int {
    let n = lua_gettop(L); // number of arguments
    if n == 1 {
        // optimize common case of single char
        pushutfchar(L, 1);
    } else {
        let mut b = luaL_Buffer::new();
        luaL_buffinit(L, &mut b);
        for i in 1..=n {
            pushutfchar(L, i);
            luaL_addvalue(&mut b);
        }
        luaL_pushresult(&mut b);
    }
    1
}

/*
** offset(s, n, [i]) -> index where n-th character counting from
**   position 'i' starts; 0 means character at 'i'.
*/
unsafe extern "C" fn byteoffset(L: *mut lua_State) -> c_int {
    let mut len = 0;
    let s = luaL_checklstring(L, 1, &mut len);
    let mut n = luaL_checkinteger(L, 2);
    let mut posi = if n >= 0 { 1 } else { len as lua_Integer + 1 };
    posi = u_posrelat(luaL_optinteger(L, 3, posi), len);
    luaL_argcheck(
        L,
        1 <= posi && posi - 1 <= len as lua_Integer,
        3,
        cstr!("position out of range"),
    );
    let mut posi = posi as usize - 1;
    let s = slice::from_raw_parts(s as *const u8, len);
    if n == 0 {
        // find beginning of current byte sequence
        while posi > 0 && iscont(s[posi]) {
            posi -= 1;
        }
    } else {
        if posi < s.len() && iscont(s[posi]) {
            luaL_error(L, cstr!("initial position is a continuation byte"));
        }
        if n < 0 {
            while n < 0 && posi > 0 {
                // move back
                posi -= 1;
                // find beginning of previous character
                while posi > 0 && iscont(s[posi]) {
                    posi -= 1;
                }
                n += 1;
            }
        } else {
            n -= 1; // do not move for 1st character
            while n > 0 && posi < len {
                posi += 1;
                // find beginning of next character
                while posi < s.len() && iscont(s[posi]) {
                    posi += 1;
                }
                n -= 1;
            }
        }
    }
    if n == 0 {
        // did it find given character?
        lua_pushinteger(L, posi as lua_Integer + 1);
    } else {
        // no such character
        lua_pushnil(L);
    }
    1
}

unsafe extern "C" fn iter_aux(L: *mut lua_State) -> c_int {
    let mut len = 0;
    let s = luaL_checklstring(L, 1, &mut len);
    let mut n = lua_tointeger(L, 2) - 1;
    let s = slice::from_raw_parts(s as *const u8, len);
    if n < 0 {
        // first iteration?
        n = 0; // start from here
    } else if n < len as lua_Integer {
        // skip current byte and its continuations
        n += 1;
        while (n as usize) < s.len() && iscont(s[n as usize]) {
            n += 1;
        }
    }
    if n >= len as lua_Integer {
        0 // no more codepoints
    } else {
        match utf8_decode2(&s[n as usize..]) {
            Some((code, _)) => {
                lua_pushinteger(L, n + 1);
                lua_pushinteger(L, code as lua_Integer);
            }
            None => luaL_error(L, cstr!("invalid UTF-8 code")),
        }
        2
    }
}

unsafe extern "C" fn iter_codes(L: *mut lua_State) -> c_int {
    luaL_checklstring(L, 1, ptr::null_mut());
    lua_pushcfunction(L, Some(iter_aux));
    lua_pushvalue(L, 1);
    lua_pushinteger(L, 0);
    3
}

const funcs: [luaL_Reg; 7] = [
    luaL_Reg::new(cstr!("offset"), Some(byteoffset)),
    luaL_Reg::new(cstr!("codepoint"), Some(codepoint)),
    luaL_Reg::new(cstr!("char"), Some(utfchar)),
    luaL_Reg::new(cstr!("len"), Some(utflen)),
    luaL_Reg::new(cstr!("codes"), Some(iter_codes)),
    // placeholders
    luaL_Reg::new(cstr!("charpattern"), None),
    luaL_Reg::new(ptr::null(), None),
];

// pattern to match a single UTF-8 character
const UTF8PATT: &[u8] = b"[\0-\x7F\xC2-\xF4][\x80-\xBF]*";

#[no_mangle]
pub unsafe extern "C" fn luaopen_utf8(L: *mut lua_State) -> c_int {
    luaL_newlib(L, funcs.as_ptr());
    lua_pushlstring(L, UTF8PATT.as_ptr() as *const c_char, UTF8PATT.len());
    lua_setfield(L, -2, cstr!("charpattern"));
    return 1;
}
