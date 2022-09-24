/*
** Auxiliary functions for building Lua libraries
*/

use std::mem;
use std::ptr;

use libc::{
    c_char, c_int, c_void, fclose, fdopen, feof, ferror, fflush, fopen, fprintf, fread, free,
    freopen, memcpy, size_t, strcmp, strerror, strlen, strncmp, strstr, EOF, FILE, STDERR_FILENO,
    STDIN_FILENO, STDOUT_FILENO, fwrite, realloc, c_long, c_uchar
};

// Cross platform retrieval of errno
#[cfg(target_os = "linux")]
use libc::__errno_location as errno_location;
#[cfg(target_os = "macos")]
use libc::__error as errno_location;

use crate::lapi::{
    lua_absindex, lua_atpanic, lua_call, lua_checkstack, lua_concat, lua_copy, lua_createtable,
    lua_error, lua_getallocf, lua_getfield, lua_getmetatable, lua_gettop, lua_insert,
    lua_isinteger, lua_isnil, lua_isnumber, lua_isstring, lua_istable, lua_len, lua_load,
    lua_newtable, lua_newuserdata, lua_next, lua_pop, lua_pushboolean, lua_pushcclosure,
    lua_pushcfunction, lua_pushfstring, lua_pushinteger, lua_pushliteral, lua_pushlstring,
    lua_pushnil, lua_pushstring, lua_pushvalue, lua_pushvfstring, lua_rawequal, lua_rawget,
    lua_rawgeti, lua_rawlen, lua_rawseti, lua_remove, lua_setfield, lua_setglobal,
    lua_setmetatable, lua_settop, lua_toboolean, lua_tointeger, lua_tointegerx, lua_tolstring,
    lua_tonumberx, lua_topointer, lua_tostring, lua_touserdata, lua_type, lua_typename,
    lua_version,
};
use crate::ldebug::{lua_getinfo, lua_getstack};
use crate::llimits::LUAL_BUFFERSIZE;
use crate::lstate::{lua_State, lua_newstate, CallInfo};
use crate::types::{
    lua_CFunction, lua_Debug, lua_Integer, lua_Number, LUA_ERRERR, LUA_REGISTRYINDEX, LUA_TBOOLEAN,
    LUA_TLIGHTUSERDATA, LUA_TNIL, LUA_TNONE, LUA_TNUMBER, LUA_TSTRING, LUA_TTABLE, LUA_VERSION_NUM,
};

extern "C" {
    fn getc(__stream: *mut FILE) -> c_int;
}

/* extra error code for 'luaL_loadfilex' */
pub const LUA_ERRFILE: c_int = LUA_ERRERR + 1;

/* key, in the registry, for table of loaded modules */
pub const LUA_LOADED_TABLE: *const c_char = cstr!("_LOADED");

/* key, in the registry, for table of preloaded loaders */
pub const LUA_PRELOAD_TABLE: *const c_char = cstr!("_PRELOAD");

#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: lua_CFunction,
}

impl luaL_Reg {
    pub(crate) const fn new(name: *const c_char, func: lua_CFunction) -> Self {
        luaL_Reg { name, func }
    }
}

const LUAL_NUMSIZES: usize = mem::size_of::<lua_Integer>() * 16 + mem::size_of::<lua_Number>();

pub unsafe fn luaL_checkversion(L: *mut lua_State) {
    luaL_checkversion_(L, LUA_VERSION_NUM, LUAL_NUMSIZES)
}

/* predefined references */
pub const LUA_NOREF: c_int = -2;
pub const LUA_REFNIL: c_int = -1;

pub unsafe fn luaL_newlibtable(L: *mut lua_State, l: *const luaL_Reg) {
    let len = {
        let mut l = l;
        let mut i = 0;
        while !(*l).name.is_null() {
            i += 1;
            l = l.add(1);
        }
        i
    };
    lua_createtable(L, 0, len);
}

pub unsafe fn luaL_newlib(L: *mut lua_State, l: *const luaL_Reg) {
    luaL_checkversion(L);
    luaL_newlibtable(L, l);
    luaL_setfuncs(L, l, 0);
}

#[inline(always)]
pub unsafe fn luaL_argcheck(
    L: *mut lua_State,
    cond: bool,
    arg: c_int,
    extramsg: *const c_char,
) -> bool {
    cond || luaL_argerror(L, arg, extramsg) != 0
}

pub unsafe fn luaL_loadfile(L: *mut lua_State, filename: *const c_char) -> c_int {
    luaL_loadfilex(L, filename, ptr::null())
}

pub unsafe fn luaL_checkstring(L: *mut lua_State, n: c_int) -> *const c_char {
    luaL_checklstring(L, n, ptr::null_mut())
}

pub unsafe fn luaL_optstring(L: *mut lua_State, arg: c_int, def: *const c_char) -> *const c_char {
    luaL_optlstring(L, arg, def, ptr::null_mut())
}

/*
** {==================================================================
** "Abstraction Layer" for basic report of messages and errors
** ===================================================================
*/

/* print a string */
#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn lua_writestring(s: *const c_char, l: size_t) {
    let stdout_fd = fdopen(STDOUT_FILENO, cstr!("w"));
    fwrite(
        s as *const c_void,
        std::mem::size_of::<c_char>(),
        l,
        stdout_fd,
    );
    fflush(stdout_fd);
}

/* print a newline and flush the output */
#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn lua_writeline() {
    lua_writestring(cstr!("\n"), 1);
}

/* print an error message */
#[inline(always)]
pub unsafe extern "C" fn lua_writestringerror(s: *const c_char, l: size_t) {
    let stderr_fd = fdopen(STDERR_FILENO, cstr!("w"));
    fprintf(stderr_fd, s, l);
    fflush(stderr_fd);
}

/*
** {======================================================
** Traceback
** =======================================================
*/

pub const LEVELS1: c_int = 10; /* size of the first part of the stack */
pub const LEVELS2: c_int = 11; /* size of the second part of the stack */

/*
** search for 'objidx' in table at index -1.
** return 1 + string at top if find a good name.
*/
unsafe extern "C" fn findfield(
    L: *mut lua_State,
    objidx: c_int,
    level: c_int,
) -> c_int {
    if level == 0 || !lua_istable(L, -1) {
        return 0;
    }
    lua_pushnil(L);
    while lua_next(L, -2) != 0 {
        if lua_type(L, -2) == LUA_TSTRING {
            if lua_rawequal(L, objidx, -1) != 0 {
                lua_pop(L, 1);
                return 1;
            } else {
                if findfield(L, objidx, level - 1) != 0 {
                    lua_remove(L, -2); /* remove table (but keep name) */
                    lua_pushliteral(L, ".");
                    lua_insert(L, -2);
                    lua_concat(L, 3);
                    return 1 as c_int;
                }
            }
        }
        lua_pop(L, 1);
    }
    return 0;
}

/*
** Search for a name for a function in all loaded modules
*/

unsafe extern "C" fn pushglobalfuncname(L: *mut lua_State, ar: *mut lua_Debug) -> c_int {
    let top = lua_gettop(L);
    lua_getinfo(L, b"f\0" as *const u8 as *const c_char, ar);
    lua_getfield(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE);
    if findfield(L, top + 1, 2) != 0 {
        let name = lua_tostring(L, -1);
        if strncmp(name, b"_G.\0" as *const u8 as *const c_char, 3) == 0 as c_int {
            lua_pushstring(L, name.offset(3 as c_int as isize));
            lua_remove(L, -2);
        }
        lua_copy(L, -1, top + 1);
        lua_pop(L, 2);
        return 1;
    } else {
        lua_settop(L, top);
        return 0;
    };
}

unsafe extern "C" fn pushfuncname(L: *mut lua_State, ar: *mut lua_Debug) {
    if pushglobalfuncname(L, ar) != 0 {
        /* try first a global name */
        lua_pushfstring(
            L,
            b"function '%s'\0" as *const u8 as *const c_char,
            lua_tostring(L, -1),
        );
        lua_remove(L, -2); /* remove name */
    } else if *(*ar).namewhat as c_int != '\0' as i32 {
        /* is there a name from code? */
        lua_pushfstring(
            /* use it */
            L,
            b"%s '%s'\0" as *const u8 as *const c_char,
            (*ar).namewhat,
            (*ar).name,
        );
    } else if *(*ar).what as c_int == 'm' as i32 {
        /* main? */
        lua_pushliteral(L, "main chunk");
    } else if *(*ar).what as c_int != 'C' as i32 {
        /* for Lua functions, use <file:line> */
        lua_pushfstring(
            L,
            b"function <%s:%d>\0" as *const u8 as *const c_char,
            ((*ar).short_src).as_mut_ptr(),
            (*ar).linedefined,
        );
    } else {
        /* nothing left... */
        lua_pushliteral(L, "?");
    };
}

unsafe extern "C" fn lastlevel(L: *mut lua_State) -> c_int {
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const c_char,
        namewhat: 0 as *const c_char,
        what: 0 as *const c_char,
        source: 0 as *const c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut li = 1 as c_int;
    let mut le = 1 as c_int;
    while lua_getstack(L, le, &mut ar) != 0 {
        li = le;
        le *= 2 as c_int;
    }
    while li < le {
        let m = (li + le) / 2 as c_int;
        if lua_getstack(L, m, &mut ar) != 0 {
            li = m + 1 as c_int;
        } else {
            le = m;
        }
    }
    return le - 1 as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_traceback(
    L: *mut lua_State,
    L1: *mut lua_State,
    msg: *const c_char,
    mut level: c_int,
) {
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const c_char,
        namewhat: 0 as *const c_char,
        what: 0 as *const c_char,
        source: 0 as *const c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let top = lua_gettop(L);
    let last = lastlevel(L1);
    let mut n1 = if last - level > LEVELS1 + LEVELS2 {
        LEVELS1
    } else {
        -(1 as c_int)
    };
    if !msg.is_null() {
        lua_pushfstring(L, b"%s\n\0" as *const u8 as *const c_char, msg);
    }
    luaL_checkstack(L, 10 as c_int, ptr::null_mut() as *const c_char);
    lua_pushliteral(L, "stack traceback:");
    loop {
        let fresh314 = level;
        level = level + 1;
        if !(lua_getstack(L1, fresh314, &mut ar) != 0) {
            break;
        }
        let fresh315 = n1;
        n1 = n1 - 1;
        if fresh315 == 0 as c_int {
            lua_pushliteral(L, "\n\t...");
            level = last - LEVELS2 + 1 as c_int;
        } else {
            lua_getinfo(L1, b"Slnt\0" as *const u8 as *const c_char, &mut ar);
            lua_pushfstring(
                L,
                b"\n\t%s:\0" as *const u8 as *const c_char,
                (ar.short_src).as_mut_ptr(),
            );
            if ar.currentline > 0 as c_int {
                lua_pushfstring(
                    L,
                    b"%d:\0" as *const u8 as *const c_char,
                    ar.currentline,
                );
            }
            lua_pushliteral(L, " in ");
            pushfuncname(L, &mut ar);
            if ar.istailcall != 0 {
                lua_pushliteral(L, "\n\t(...tail calls...)");
            }
            lua_concat(L, lua_gettop(L) - top);
        }
    }
    lua_concat(L, lua_gettop(L) - top);
}

/*
** {======================================================
** Error-report functions
** =======================================================
*/

#[no_mangle]
pub unsafe extern "C" fn luaL_argerror(
    L: *mut lua_State,
    mut arg: c_int,
    extramsg: *const c_char,
) -> c_int {
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const c_char,
        namewhat: 0 as *const c_char,
        what: 0 as *const c_char,
        source: 0 as *const c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    if lua_getstack(L, 0 as c_int, &mut ar) == 0 {
        luaL_error(
            L,
            b"bad argument #%d (%s)\0" as *const u8 as *const c_char,
            arg,
            extramsg,
        );
    }
    lua_getinfo(L, b"n\0" as *const u8 as *const c_char, &mut ar);
    if strcmp(ar.namewhat, b"method\0" as *const u8 as *const c_char) == 0 as c_int {
        arg -= 1;
        if arg == 0 as c_int {
            luaL_error(
                L,
                b"calling '%s' on bad self (%s)\0" as *const u8 as *const c_char,
                ar.name,
                extramsg,
            );
        }
    }
    if (ar.name).is_null() {
        ar.name = if pushglobalfuncname(L, &mut ar) != 0 {
            lua_tostring(L, -1)
        } else {
            b"?\0" as *const u8 as *const c_char
        };
    }
    luaL_error(
        L,
        b"bad argument #%d to '%s' (%s)\0" as *const u8 as *const c_char,
        arg,
        ar.name,
        extramsg,
    );
}

unsafe fn luaL_typename(L: *mut lua_State, i: c_int) -> *const i8 {
    return lua_typename(L, lua_type(L, i));
}

unsafe extern "C" fn typeerror(
    L: *mut lua_State,
    arg: c_int,
    tname: *const c_char,
) -> c_int {
    let msg;
    let typearg;
    if luaL_getmetafield(L, arg, b"__name\0" as *const u8 as *const c_char) == LUA_TSTRING {
        typearg = lua_tostring(L, -1);
    } else if lua_type(L, arg) == LUA_TLIGHTUSERDATA {
        typearg = b"light userdata\0" as *const u8 as *const c_char;
    } else {
        typearg = luaL_typename(L, arg);
    }
    msg = lua_pushfstring(
        L,
        b"%s expected, got %s\0" as *const u8 as *const c_char,
        tname,
        typearg,
    );
    return luaL_argerror(L, arg, msg);
}

unsafe extern "C" fn tag_error(L: *mut lua_State, arg: c_int, tag: c_int) {
    typeerror(L, arg, lua_typename(L, tag));
}

/*
** The use of 'lua_pushfstring' ensures this function does not
** need reserved stack space when called.
*/
#[no_mangle]
pub unsafe extern "C" fn luaL_where(L: *mut lua_State, level: c_int) {
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const c_char,
        namewhat: 0 as *const c_char,
        what: 0 as *const c_char,
        source: 0 as *const c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    if lua_getstack(L, level, &mut ar) != 0 {
        lua_getinfo(L, b"Sl\0" as *const u8 as *const c_char, &mut ar);
        if ar.currentline > 0 as c_int {
            lua_pushfstring(
                L,
                b"%s:%d: \0" as *const u8 as *const c_char,
                (ar.short_src).as_mut_ptr(),
                ar.currentline,
            );
            return;
        }
    }
    lua_pushfstring(L, b"\0" as *const u8 as *const c_char);
}

/*
** Again, the use of 'lua_pushvfstring' ensures this function does
** not need reserved stack space when called. (At worst, it generates
** an error with "stack overflow" instead of the given message.)
*/
#[no_mangle]
pub unsafe extern "C" fn luaL_error(L: *mut lua_State, fmt: *const c_char, args: ...) -> ! {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    luaL_where(L, 1 as c_int);
    lua_pushvfstring(L, fmt, argp.as_va_list());
    lua_concat(L, 2 as c_int);
    lua_error(L);
}

#[no_mangle]
pub unsafe extern "C" fn luaL_fileresult(
    L: *mut lua_State,
    stat: c_int,
    fname: *const c_char,
) -> c_int {
    let en: c_int = *errno_location();
    if stat != 0 {
        lua_pushboolean(L, 1 as c_int);
        return 1 as c_int;
    } else {
        lua_pushnil(L);
        if !fname.is_null() {
            lua_pushfstring(
                L,
                b"%s: %s\0" as *const u8 as *const c_char,
                fname,
                strerror(en),
            );
        } else {
            lua_pushstring(L, strerror(en));
        }
        lua_pushinteger(L, en as lua_Integer);
        return 3 as c_int;
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaL_execresult(L: *mut lua_State, stat: c_int) -> c_int {
    let what = b"exit\0" as *const u8 as *const c_char;
    if stat == -(1 as c_int) {
        return luaL_fileresult(L, 0 as c_int, ptr::null_mut() as *const c_char);
    } else {
        if *what as c_int == 'e' as i32 && stat == 0 as c_int {
            lua_pushboolean(L, 1 as c_int);
        } else {
            lua_pushnil(L);
        }
        lua_pushstring(L, what);
        lua_pushinteger(L, stat as lua_Integer);
        return 3 as c_int;
    };
}

/*
** {======================================================
** Userdata's metatable manipulation
** =======================================================
*/

unsafe fn luaL_getmetatable(L: *mut lua_State, n: *const c_char) -> c_int {
    lua_getfield(L, LUA_REGISTRYINDEX, n)
}

#[no_mangle]
pub unsafe extern "C" fn luaL_newmetatable(
    L: *mut lua_State,
    tname: *const c_char,
) -> c_int {
    if luaL_getmetatable(L, tname) != LUA_TNIL {
        return 0 as c_int;
    }
    lua_pop(L, 1);
    lua_createtable(L, 0 as c_int, 2 as c_int);
    lua_pushstring(L, tname);
    lua_setfield(
        L,
        -(2 as c_int),
        b"__name\0" as *const u8 as *const c_char,
    );
    lua_pushvalue(L, -(1 as c_int));
    lua_setfield(L, LUA_REGISTRYINDEX, tname);
    return 1 as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_setmetatable(L: *mut lua_State, tname: *const c_char) {
    luaL_getmetatable(L, tname);
    lua_setmetatable(L, -2);
}

#[no_mangle]
pub unsafe extern "C" fn luaL_testudata(
    L: *mut lua_State,
    ud: c_int,
    tname: *const c_char,
) -> *mut c_void {
    let mut p = lua_touserdata(L, ud);
    if !p.is_null() {
        if lua_getmetatable(L, ud) != 0 {
            luaL_getmetatable(L, tname);
            if lua_rawequal(L, -1, -2) == 0 {
                p = ptr::null_mut() as *mut c_void;
            }
            lua_pop(L, 2);
            return p;
        }
    }
    return ptr::null_mut() as *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_checkudata(
    L: *mut lua_State,
    ud: c_int,
    tname: *const c_char,
) -> *mut c_void {
    let p = luaL_testudata(L, ud, tname);
    if p.is_null() {
        typeerror(L, ud, tname);
    }
    return p;
}

/*
** {======================================================
** Argument check functions
** =======================================================
*/

#[no_mangle]
pub unsafe extern "C" fn luaL_checkoption(
    L: *mut lua_State,
    arg: c_int,
    def: *const c_char,
    lst: *const *const c_char,
) -> c_int {
    let name = if !def.is_null() {
        luaL_optstring(L, arg, def)
    } else {
        luaL_checkstring(L, arg)
    };
    let mut i: c_int = 0;
    while !(*lst.offset(i as isize)).is_null() {
        if strcmp(*lst.offset(i as isize), name) == 0 as c_int {
            return i;
        }
        i += 1;
    }
    return luaL_argerror(
        L,
        arg,
        lua_pushfstring(
            L,
            b"invalid option '%s'\0" as *const u8 as *const c_char,
            name,
        ),
    );
}

/*
** Ensures the stack has at least 'space' extra slots, raising an error
** if it cannot fulfill the request. (The error handling needs a few
** extra slots to format the error message. In case of an error without
** this extra space, Lua will generate the same 'stack overflow' error,
** but without 'msg'.)
*/

#[no_mangle]
pub unsafe extern "C" fn luaL_checkstack(
    L: *mut lua_State,
    space: c_int,
    msg: *const c_char,
) {
    if lua_checkstack(L, space) == 0 {
        if !msg.is_null() {
            luaL_error(
                L,
                b"stack overflow (%s)\0" as *const u8 as *const c_char,
                msg,
            );
        } else {
            luaL_error(L, b"stack overflow\0" as *const u8 as *const c_char);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaL_checktype(L: *mut lua_State, arg: c_int, t: c_int) {
    if lua_type(L, arg) != t {
        tag_error(L, arg, t);
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaL_checkany(L: *mut lua_State, arg: c_int) {
    if lua_type(L, arg) == LUA_TNONE {
        luaL_argerror(
            L,
            arg,
            b"value expected\0" as *const u8 as *const c_char,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaL_checklstring(
    L: *mut lua_State,
    arg: c_int,
    len: *mut size_t,
) -> *const c_char {
    let s = lua_tolstring(L, arg, len);
    if s.is_null() {
        tag_error(L, arg, LUA_TSTRING);
    }
    return s;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_optlstring(
    L: *mut lua_State,
    arg: c_int,
    def: *const c_char,
    len: *mut size_t,
) -> *const c_char {
    if lua_isnoneornil(L, arg) {
        if !len.is_null() {
            *len = if !def.is_null() { strlen(def) } else { 0 };
        }
        return def;
    } else {
        return luaL_checklstring(L, arg, len);
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaL_checknumber(L: *mut lua_State, arg: c_int) -> lua_Number {
    let mut isnum: c_int = 0;
    let d = lua_tonumberx(L, arg, &mut isnum);
    if isnum == 0 {
        tag_error(L, arg, LUA_TNUMBER);
    }
    return d;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_optnumber(
    L: *mut lua_State,
    arg: c_int,
    def: lua_Number,
) -> lua_Number {
    return luaL_opt(L, luaL_checknumber, arg, def);
}

unsafe extern "C" fn interror(L: *mut lua_State, arg: c_int) {
    if lua_isnumber(L, arg) != 0 {
        luaL_argerror(
            L,
            arg,
            b"number has no integer representation\0" as *const u8 as *const c_char,
        );
    } else {
        tag_error(L, arg, LUA_TNUMBER);
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaL_checkinteger(L: *mut lua_State, arg: c_int) -> lua_Integer {
    let mut isnum: c_int = 0;
    let d = lua_tointegerx(L, arg, &mut isnum);
    if isnum == 0 {
        interror(L, arg);
    }
    return d;
}

#[inline(always)]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, n: c_int) -> bool {
    return lua_type(L, n) <= 0;
}

#[inline(always)]
pub unsafe fn lua_isnone(L: *mut lua_State, n: c_int) -> bool {
    return lua_type(L, n) == LUA_TNONE;
}

#[inline(always)]
pub unsafe extern "C" fn lua_replace(L: *mut lua_State, idx: c_int) {
    lua_copy(L, -1, idx);
    lua_pop(L, 1);
}

pub unsafe extern "C" fn luaL_opt<T>(
    L: *mut lua_State,
    f: unsafe extern "C" fn(L: *mut lua_State, n: c_int) -> T,
    n: c_int,
    d: T,
) -> T {
    if lua_isnoneornil(L, n) {
        return d;
    }
    return f(L, n);
}

#[no_mangle]
pub unsafe extern "C" fn luaL_optinteger(
    L: *mut lua_State,
    arg: c_int,
    def: lua_Integer,
) -> lua_Integer {
    return luaL_opt(L, luaL_checkinteger, arg, def);
}

/*
** {======================================================
** Generic Buffer manipulation
** =======================================================
*/

/* userdata to box arbitrary data */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UBox {
    pub box_0: *mut c_void,
    pub bsize: size_t,
}

unsafe extern "C" fn resizebox(
    L: *mut lua_State,
    idx: c_int,
    newsize: size_t,
) -> *mut c_void {
    let mut ud = 0 as *mut c_void;
    let allocf = lua_getallocf(L, &mut ud);
    let mut box_0 = lua_touserdata(L, idx) as *mut UBox;
    let temp =
        allocf.expect("non-null function pointer")(ud, (*box_0).box_0, (*box_0).bsize, newsize);
    if temp.is_null() && newsize > 0 {
        resizebox(L, idx, 0 as c_int as size_t);
        luaL_error(
            L,
            b"not enough memory for buffer allocation\0" as *const u8 as *const c_char,
        );
    }
    (*box_0).box_0 = temp;
    (*box_0).bsize = newsize;
    return temp;
}

unsafe extern "C" fn boxgc(L: *mut lua_State) -> c_int {
    resizebox(L, 1, 0);
    return 0;
}

unsafe extern "C" fn newbox(L: *mut lua_State, newsize: size_t) -> *mut c_void {
    let mut box_0 = lua_newuserdata(L, ::core::mem::size_of::<UBox>() as size_t) as *mut UBox;
    (*box_0).box_0 = ptr::null_mut() as *mut c_void;
    (*box_0).bsize = 0 as c_int as size_t;
    if luaL_newmetatable(L, b"LUABOX\0" as *const u8 as *const c_char) != 0 {
        lua_pushcfunction(L, Some(boxgc));
        lua_setfield(
            L,
            -(2 as c_int),
            b"__gc\0" as *const u8 as *const c_char,
        );
    }
    lua_setmetatable(L, -(2 as c_int));
    return resizebox(L, -(1 as c_int), newsize);
}

/*
** check whether buffer is using a userdata on the stack as a temporary
** buffer
*/
#[inline(always)]
unsafe fn buffonstack(B: *mut luaL_Buffer) -> bool {
    return (*B).b != (*B).initb.as_mut_ptr();
}

/*
** returns a pointer to a free area with at least 'sz' bytes
*/

#[no_mangle]
pub unsafe extern "C" fn luaL_prepbuffsize(
    mut B: *mut luaL_Buffer,
    sz: size_t,
) -> *mut c_char {
    let L = (*B).L;
    if ((*B).size).wrapping_sub((*B).n) < sz {
        let newbuff;
        let mut newsize = ((*B).size).wrapping_mul(2 as size_t);
        if newsize.wrapping_sub((*B).n) < sz {
            newsize = ((*B).n).wrapping_add(sz);
        }
        if newsize < (*B).n || newsize.wrapping_sub((*B).n) < sz {
            luaL_error(L, b"buffer too large\0" as *const u8 as *const c_char);
        }
        if buffonstack(B) {
            newbuff = resizebox(L, -(1 as c_int), newsize) as *mut c_char;
        } else {
            newbuff = newbox(L, newsize) as *mut c_char;
            memcpy(
                newbuff as *mut c_void,
                (*B).b as *const c_void,
                ((*B).n).wrapping_mul(::core::mem::size_of::<c_char>() as size_t),
            );
        }
        (*B).b = newbuff;
        (*B).size = newsize;
    }
    return &mut *((*B).b).offset((*B).n as isize) as *mut c_char;
}

pub unsafe fn luaL_addsize(mut B: *mut luaL_Buffer, s: size_t) -> size_t {
    (*B).n += s;
    return (*B).n;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_addlstring(B: *mut luaL_Buffer, s: *const c_char, l: size_t) {
    if l > 0 {
        let b = luaL_prepbuffsize(B, l);
        memcpy(
            b as *mut c_void,
            s as *const c_void,
            l.wrapping_mul(::core::mem::size_of::<c_char>() as usize),
        );
        luaL_addsize(B, l);
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaL_addstring(B: *mut luaL_Buffer, s: *const c_char) {
    luaL_addlstring(B, s, strlen(s));
}

#[no_mangle]
pub unsafe extern "C" fn luaL_pushresult(B: *mut luaL_Buffer) {
    let L = (*B).L;
    lua_pushlstring(L, (*B).b, (*B).n);
    if buffonstack(B) {
        resizebox(L, -2, 0);
        lua_remove(L, -2);
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaL_pushresultsize(B: *mut luaL_Buffer, sz: size_t) {
    luaL_addsize(B, sz);
    luaL_pushresult(B);
}

#[no_mangle]
pub unsafe extern "C" fn luaL_addvalue(B: *mut luaL_Buffer) {
    let L = (*B).L;
    let mut l: size_t = 0;
    let s = lua_tolstring(L, -(1 as c_int), &mut l);
    if buffonstack(B) {
        lua_insert(L, -2); /* put value below buffer */
    }
    luaL_addlstring(B, s, l);
    if buffonstack(B) {
        lua_remove(L, -2)
    } else {
        lua_remove(L, -1)
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaL_buffinit(L: *mut lua_State, mut B: *mut luaL_Buffer) {
    (*B).L = L;
    (*B).b = ((*B).initb).as_mut_ptr();
    (*B).n = 0 as c_int as size_t;
    (*B).size = LUAL_BUFFERSIZE;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_buffinitsize(
    L: *mut lua_State,
    B: *mut luaL_Buffer,
    sz: size_t,
) -> *mut c_char {
    luaL_buffinit(L, B);
    return luaL_prepbuffsize(B, sz);
}

/*
** {======================================================
** Reference system
** =======================================================
*/

/* index of free-list header */
pub const freelist: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn luaL_ref(L: *mut lua_State, mut t: c_int) -> c_int {
    let mut ref_0;
    if lua_isnil(L, -1) != 0 {
        lua_pop(L, 1);
        return LUA_REFNIL;
    }
    t = lua_absindex(L, t);
    lua_rawgeti(L, t, freelist as lua_Integer);
    ref_0 = lua_tointeger(L, -1) as c_int;
    lua_pop(L, 1);
    if ref_0 != 0 as c_int {
        lua_rawgeti(L, t, ref_0 as lua_Integer);
        lua_rawseti(L, t, freelist as lua_Integer);
    } else {
        ref_0 = lua_rawlen(L, t) as c_int + 1;
    }
    lua_rawseti(L, t, ref_0 as lua_Integer);
    return ref_0;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_unref(L: *mut lua_State, mut t: c_int, ref_0: c_int) {
    if ref_0 >= 0 {
        t = lua_absindex(L, t);
        lua_rawgeti(L, t, freelist as lua_Integer);
        lua_rawseti(L, t, ref_0 as lua_Integer);
        lua_pushinteger(L, ref_0 as lua_Integer);
        lua_rawseti(L, t, freelist as lua_Integer);
    }
}

/*
** {======================================================
** Load functions
** =======================================================
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadF {
    pub n: c_int,             /* number of pre-read characters */
    pub f: *mut FILE,               /* file being read */
    pub buff: [c_char; 8192], /* area for reading file */
}

unsafe extern "C" fn getF(
    _L: *mut lua_State,
    ud: *mut c_void,
    size: *mut size_t,
) -> *const c_char {
    let mut lf = ud as *mut LoadF;
    if (*lf).n > 0 as c_int {
        /* are there pre-read characters to be read? */
        *size = (*lf).n as size_t; /* return them (chars already in buffer) */
        (*lf).n = 0 as c_int; /* no more pre-read characters */
    } else {
        /* read a block from file */
        /* 'fread' can return > 0 *and* set the EOF flag. If next call to
        'getF' called 'fread', it might still wait for user input.
        The next check avoids this problem. */
        if feof((*lf).f) != 0 {
            return ptr::null_mut() as *const c_char;
        }
        // FIXME - this reads 1 byte at a time, we should read more than this, but seeing 8192 bytes (the buffer size)
        //         just causes it to block...
        // *size = fread(lf->buff, 1, sizeof(lf->buff), lf->f);  /* read block */
        *size = fread(
            ((*lf).buff).as_mut_ptr() as *mut c_void,
            1 as size_t,
            1,
            //::core::mem::size_of::<[c_char; 8192]>() as size_t,
            (*lf).f, /* read block */
        );
    }
    return ((*lf).buff).as_mut_ptr();
}

unsafe extern "C" fn errfile(
    L: *mut lua_State,
    what: *const c_char,
    fnameindex: c_int,
) -> c_int {
    let errno: c_int = *errno_location();
    let serr: *const c_char = strerror(errno);
    let filename = lua_tostring(L, fnameindex).offset(1 as c_int as isize);
    lua_pushfstring(
        L,
        b"cannot %s %s: %s\0" as *const u8 as *const c_char,
        what,
        filename,
        serr,
    );
    lua_settop(L, -(1 as c_int) - 1);
    return LUA_ERRFILE;
}

unsafe extern "C" fn skipBOM(mut lf: *mut LoadF) -> c_int {
    let mut p = b"\xEF\xBB\xBF\0" as *const u8 as *const c_char;
    let mut c: c_int;
    (*lf).n = 0 as c_int;
    loop {
        c = getc((*lf).f);
        if c == EOF || {
            let fresh319 = p;
            p = p.offset(1);
            c != *(fresh319 as *const c_uchar) as c_int
        } {
            return c;
        }
        let fresh320 = (*lf).n;
        (*lf).n = (*lf).n + 1;
        (*lf).buff[fresh320 as usize] = c as c_char;
        if !(*p as c_int != '\0' as i32) {
            break;
        }
    }
    (*lf).n = 0 as c_int;
    return getc((*lf).f);
}

/*
** reads the first character of file 'f' and skips an optional BOM mark
** in its beginning plus its first line if it starts with '#'. Returns
** true if it skipped the first line.  In any case, '*cp' has the
** first "valid" character of the file (after the optional BOM and
** a first-line comment).
*/
unsafe extern "C" fn skipcomment(lf: *mut LoadF, cp: *mut c_int) -> c_int {
    *cp = skipBOM(lf);
    let mut c = *cp;
    if c == '#' as i32 {
        loop {
            c = getc((*lf).f);
            if !(c != EOF && c != '\n' as i32) {
                break;
            }
        }
        *cp = getc((*lf).f);
        return 1 as c_int;
    } else {
        return 0 as c_int;
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaL_loadfilex(
    L: *mut lua_State,
    filename: *const c_char,
    mode: *const c_char,
) -> c_int {
    let mut lf = LoadF {
        n: 0,
        f: 0 as *mut FILE,
        buff: [0; 8192],
    };
    let status: c_int;
    let readstatus: c_int;
    let mut c: c_int = 0;
    let fnameindex = lua_gettop(L) + 1 as c_int;
    if filename.is_null() {
        lua_pushliteral(L, "=stdin");
        lf.f = fdopen(STDIN_FILENO, b"r\0" as *const u8 as *const c_char)
    } else {
        lua_pushfstring(L, b"@%s\0" as *const u8 as *const c_char, filename);
        lf.f = fopen(filename, b"r\0" as *const u8 as *const c_char);
        if (lf.f).is_null() {
            return errfile(L, b"open\0" as *const u8 as *const c_char, fnameindex);
        }
    }
    if skipcomment(&mut lf, &mut c) != 0 {
        lf.buff[lf.n as usize] = '\n' as i8;
        lf.n = lf.n + 1;
    }
    if c == (*::core::mem::transmute::<&[u8; 5], &[c_char; 5]>(b"\x1BLua\0"))[0 as usize]
        as c_int
        && !filename.is_null()
    {
        lf.f = freopen(filename, b"rb\0" as *const u8 as *const c_char, lf.f);
        if (lf.f).is_null() {
            return errfile(
                L,
                b"reopen\0" as *const u8 as *const c_char,
                fnameindex,
            );
        }
        skipcomment(&mut lf, &mut c);
    }
    if c != EOF {
        lf.buff[lf.n as usize] = c as c_char;
        lf.n = lf.n + 1;
    }
    status = lua_load(
        L,
        Some(getF),
        &mut lf as *mut LoadF as *mut c_void,
        lua_tostring(L, -1),
        mode,
    );
    readstatus = ferror(lf.f);
    if !filename.is_null() {
        fclose(lf.f);
    }
    if readstatus != 0 {
        lua_settop(L, fnameindex);
        return errfile(L, b"read\0" as *const u8 as *const c_char, fnameindex);
    }
    lua_remove(L, fnameindex);
    return status;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadS {
    pub s: *const c_char,
    pub size: size_t,
}

unsafe extern "C" fn getS(
    _L: *mut lua_State,
    ud: *mut c_void,
    size: *mut size_t,
) -> *const c_char {
    let mut ls = ud as *mut LoadS;
    if (*ls).size == 0 as size_t {
        return ptr::null_mut() as *const c_char;
    }
    *size = (*ls).size;
    (*ls).size = 0 as c_int as size_t;
    return (*ls).s;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_loadbufferx(
    L: *mut lua_State,
    buff: *const c_char,
    size: size_t,
    name: *const c_char,
    mode: *const c_char,
) -> c_int {
    let mut ls = LoadS {
        s: 0 as *const c_char,
        size: 0,
    };
    ls.s = buff;
    ls.size = size;
    return lua_load(
        L,
        Some(getS),
        &mut ls as *mut LoadS as *mut c_void,
        name,
        mode,
    );
}

#[no_mangle]
pub unsafe extern "C" fn luaL_loadstring(L: *mut lua_State, s: *const c_char) -> c_int {
    return luaL_loadbufferx(L, s, strlen(s), s, ptr::null_mut());
}

#[no_mangle]
pub unsafe fn luaL_loadbuffer(
    L: *mut lua_State,
    buff: *const c_char,
    size: size_t,
    name: *const c_char,
) -> c_int {
    return luaL_loadbufferx(L, buff, size, name, 0 as *const i8);
}

/* }====================================================== */

#[no_mangle]
pub unsafe extern "C" fn luaL_getmetafield(
    L: *mut lua_State,
    obj: c_int,
    event: *const c_char,
) -> c_int {
    if lua_getmetatable(L, obj) == 0 {
        return LUA_TNIL;
    } else {
        let tt: c_int;
        lua_pushstring(L, event);
        tt = lua_rawget(L, -(2 as c_int));
        if tt == LUA_TNIL {
            lua_pop(L, 2);
        } else {
            lua_remove(L, -2);
        }
        return tt;
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaL_callmeta(
    L: *mut lua_State,
    mut obj: c_int,
    event: *const c_char,
) -> c_int {
    obj = lua_absindex(L, obj);
    if luaL_getmetafield(L, obj, event) == LUA_TNIL {
        return 0 as c_int;
    }
    lua_pushvalue(L, obj);
    lua_call(L, 1, 1);
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_len(L: *mut lua_State, idx: c_int) -> lua_Integer {
    let l: lua_Integer;
    let mut isnum: c_int = 0;
    lua_len(L, idx);
    l = lua_tointegerx(L, -(1 as c_int), &mut isnum);
    if isnum == 0 {
        luaL_error(
            L,
            b"object length is not an integer\0" as *const u8 as *const c_char,
        );
    }
    lua_pop(L, 1);
    return l;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_tolstring(
    L: *mut lua_State,
    idx: c_int,
    len: *mut size_t,
) -> *const c_char {
    if luaL_callmeta(L, idx, b"__tostring\0" as *const u8 as *const c_char) != 0 {
        if lua_isstring(L, -1) == 0 {
            luaL_error(
                L,
                b"'__tostring' must return a string\0" as *const u8 as *const c_char,
            );
        }
    } else {
        match lua_type(L, idx) {
            LUA_TNUMBER => {
                if lua_isinteger(L, idx) != 0 {
                    lua_pushfstring(
                        L,
                        b"%I\0" as *const u8 as *const c_char,
                        lua_tointeger(L, idx),
                    );
                } else {
                    lua_pushfstring(
                        L,
                        b"%f\0" as *const u8 as *const c_char,
                        lua_tonumberx(L, idx, ptr::null_mut()),
                    );
                }
            }
            LUA_TSTRING => {
                lua_pushvalue(L, idx);
            }
            LUA_TBOOLEAN => {
                lua_pushstring(
                    L,
                    if lua_toboolean(L, idx) != 0 {
                        b"true\0" as *const u8 as *const c_char
                    } else {
                        b"false\0" as *const u8 as *const c_char
                    },
                );
            }
            LUA_TNIL => {
                lua_pushliteral(L, "nil");
            }
            _ => {
                let tt = luaL_getmetafield(L, idx, b"__name\0" as *const u8 as *const c_char);
                let kind = if tt == LUA_TSTRING {
                    lua_tostring(L, -1)
                } else {
                    luaL_typename(L, idx)
                };
                lua_pushfstring(
                    L,
                    b"%s: %p\0" as *const u8 as *const c_char,
                    kind,
                    lua_topointer(L, idx),
                );
                if tt != LUA_TNIL {
                    lua_remove(L, -2); /* remove '__name' */
                }
            }
        }
    }
    return lua_tolstring(L, -1, len);
}

/* }====================================================== */

/*
** set functions from list 'l' into table at top - 'nup'; each
** function gets the 'nup' elements at the top as upvalues.
** Returns with only the table at the stack.
*/

#[no_mangle]
pub unsafe extern "C" fn luaL_setfuncs(
    L: *mut lua_State,
    mut l: *const luaL_Reg,
    nup: c_int,
) {
    luaL_checkstack(
        L,
        nup,
        b"too many upvalues\0" as *const u8 as *const c_char,
    );
    while !((*l).name).is_null() {
        let mut i: c_int = 0;
        while i < nup {
            lua_pushvalue(L, -nup);
            i += 1;
        }
        lua_pushcclosure(L, (*l).func, nup);
        lua_setfield(L, -(nup + 2 as c_int), (*l).name);
        l = l.offset(1);
    }
    lua_pop(L, nup);
}
/*
** ensure that stack[idx][fname] has a table and push that table
** into the stack
*/

#[no_mangle]
pub unsafe extern "C" fn luaL_getsubtable(
    L: *mut lua_State,
    mut idx: c_int,
    fname: *const c_char,
) -> c_int {
    if lua_getfield(L, idx, fname) == LUA_TTABLE {
        return 1 as c_int;
    } else {
        lua_pop(L, 1);
        idx = lua_absindex(L, idx);
        lua_newtable(L);
        lua_pushvalue(L, -(1 as c_int));
        lua_setfield(L, idx, fname);
        return 0 as c_int;
    };
}

/*
** Stripped-down 'require': After checking "loaded" table, calls 'openf'
** to open a module, registers the result in 'package.loaded' table and,
** if 'glb' is true, also registers the result in the global table.
** Leaves resulting module on the top.
*/

#[no_mangle]
pub unsafe extern "C" fn luaL_requiref(
    L: *mut lua_State,
    modname: *const c_char,
    openf: lua_CFunction,
    glb: c_int,
) {
    luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE);
    lua_getfield(L, -1, modname); /* LOADED[modname] */
    if lua_toboolean(L, -1) == 0 {
        /* package not already loaded? */
        lua_pop(L, 1); /* remove field */
        lua_pushcfunction(L, openf);
        lua_pushstring(L, modname); /* argument to open function */
        lua_call(L, 1, 1); /* call 'openf' to open module */
        lua_pushvalue(L, -1); /* make copy of module (call result) */
        lua_setfield(L, -3, modname); /* LOADED[modname] = module */
    }
    lua_remove(L, -2); /* remove LOADED table */
    if glb != 0 {
        lua_pushvalue(L, -1); /* copy of module */
        lua_setglobal(L, modname); /* _G[modname] = module */
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaL_gsub(
    L: *mut lua_State,
    mut s: *const c_char,
    p: *const c_char,
    r: *const c_char,
) -> *const c_char {
    let mut wild: *const c_char;
    let l = strlen(p);
    let mut b = luaL_Buffer {
        b: 0 as *mut c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    loop {
        wild = strstr(s, p);
        if wild.is_null() {
            break;
        }
        luaL_addlstring(&mut b, s, wild.offset_from(s) as c_long as size_t);
        luaL_addstring(&mut b, r);
        s = wild.offset(l as isize);
    }
    luaL_addstring(&mut b, s);
    luaL_pushresult(&mut b);
    return lua_tostring(L, -1);
}

unsafe extern "C" fn l_alloc(
    _ud: *mut c_void,
    ptr: *mut c_void,
    osize: size_t,
    nsize: size_t,
) -> *mut c_void {
    if nsize == 0 {
        free(ptr);
        return ptr::null_mut() as *mut c_void;
    } else {
        let newptr = realloc(ptr, nsize);
        if newptr.is_null() && !ptr.is_null() && nsize <= osize {
            return ptr;
        } else {
            return newptr;
        }
    };
}

unsafe extern "C" fn panic(L: *mut lua_State) -> c_int {
    let stderr_fd = fdopen(STDERR_FILENO, b"w\0" as *const u8 as *const c_char);
    fprintf(
        stderr_fd,
        b"PANIC: unprotected error in call to Lua API (%s)\n\0" as *const u8 as *const c_char,
        lua_tolstring(L, -(1 as c_int), 0 as *mut size_t),
    );
    fflush(stderr_fd);
    return 0 as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_newstate() -> *mut lua_State {
    let L = lua_newstate(Some(l_alloc), ptr::null_mut());
    if !L.is_null() {
        lua_atpanic(L, Some(panic));
    }
    return L;
}

#[no_mangle]
pub unsafe extern "C" fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) {
    let v = lua_version(L);
    if sz != LUAL_NUMSIZES {
        luaL_error(
            L,
            b"core and library have incompatible numeric types\0" as *const u8
                as *const c_char,
        );
    }
    let n: *mut lua_State = ptr::null_mut();
    if v != lua_version(n) {
        luaL_error(
            L,
            b"multiple Lua VMs detected\0" as *const u8 as *const c_char,
        );
    } else if *v != ver {
        luaL_error(
            L,
            b"version mismatch: app. needs %f, Lua core provides %f\0" as *const u8
                as *const c_char,
            ver,
            *v,
        );
    }
}

/*
** Generic Buffer manipulation
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut c_char, /* buffer address */
    pub size: size_t,   /* buffer size */
    pub n: size_t,      /* number of characters in buffer */
    pub L: *mut lua_State,
    pub initb: [c_char; LUAL_BUFFERSIZE], /* initial buffer */
}

impl luaL_Buffer {
    pub const fn new() -> Self {
        luaL_Buffer {
            b: ptr::null_mut(),
            size: 0,
            n: 0,
            L: ptr::null_mut(),
            initb: [0; LUAL_BUFFERSIZE],
        }
    }
}
