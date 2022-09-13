
/*use std::mem::{size_of, MaybeUninit};
use std::ptr;

use libc::{c_char, c_int, c_void, c_longlong};

use crate::ldo::{luaD_inctop, luaD_throw};
use crate::lfunc::{luaF_newLclosure, luaF_newproto};
use crate::lgc::luaC_objbarrier;
use crate::llimits::{Instruction, LUAI_MAXSHORTLEN};
use crate::lmem::luaM_newvector;
use crate::lobject::{
    getstr, luaO_pushfstring, setbvalue, setclLvalue, setfltvalue, setivalue, setnilvalue,
    setsvalue, LClosure, LocVar, Proto, TString, TValue, Upvaldesc, LUA_TLNGSTR, LUA_TNUMFLT,
    LUA_TNUMINT, LUA_TSHRSTR,
};
use crate::lstate::lua_State;
use crate::lstring::{luaS_createlngstrobj, luaS_newlstr};
use crate::lzio::{luaZ_read, ZIO};
use crate::types::{lua_Integer, lua_Number, LUA_ERRSYNTAX, LUA_SIGNATURE, LUA_TBOOLEAN, LUA_TNIL};
*/

use crate::lstate::lua_State;
use libc::{c_int};


/*
** codepoint(s, [i, [j]])  -> returns codepoints for all characters
** that start in the range [i,j]
*/
#[no_mangle]
pub unsafe extern "C" fn codepoint(mut L: *mut lua_State) -> libc::c_int {
    return 1;
    /*let mut len: size_t = 0;
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
    return n;*/
}
