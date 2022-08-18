use libc::{c_char, c_double, c_int, c_longlong, c_uchar, c_void, intptr_t, size_t};

use crate::llimits::LUA_IDSIZE;
use crate::lstate::{lua_State, CallInfo};

/* thread status */
pub const LUA_OK: c_int = 0;
pub const LUA_YIELD: c_int = 1;
pub const LUA_ERRRUN: c_int = 2;
pub const LUA_ERRSYNTAX: c_int = 3;
pub const LUA_ERRMEM: c_int = 4;
pub const LUA_ERRGCMM: c_int = 5;
pub const LUA_ERRERR: c_int = 6;

/*
** basic types
*/
pub const LUA_TNONE: c_int = -1;

pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TSTRING: c_int = 4;
pub const LUA_TTABLE: c_int = 5;
pub const LUA_TFUNCTION: c_int = 6;
pub const LUA_TUSERDATA: c_int = 7;
pub const LUA_TTHREAD: c_int = 8;

pub const LUA_NUMTAGS: usize = 9;

/* minimum Lua stack available to a C function */
pub const LUA_MINSTACK: usize = 20;

/* predefined values in the registry */
pub const LUA_RIDX_MAINTHREAD: lua_Integer = 1;
pub const LUA_RIDX_GLOBALS: lua_Integer = 2;
pub const LUA_RIDX_LAST: lua_Integer = LUA_RIDX_GLOBALS;

/* type of numbers in Lua */
pub type lua_Number = c_double;

/* type for integer functions */
pub type lua_Integer = c_longlong;

/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> c_int>;

/*
** Type for continuation functions
*/
pub type lua_KContext = intptr_t;
pub type lua_KFunction = Option<unsafe extern "C" fn(*mut lua_State, c_int, lua_KContext) -> c_int>;

/*
** Type for functions that read/write blocks when loading/dumping Lua chunks
*/
pub type lua_Reader =
    Option<unsafe extern "C" fn(*mut lua_State, *mut c_void, *mut size_t) -> *const c_char>;

pub type lua_Writer =
    Option<unsafe extern "C" fn(*mut lua_State, *const c_void, size_t, *mut c_void) -> c_int>;

/*
** Type for memory-allocation functions
*/
pub type lua_Alloc =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, size_t, size_t) -> *mut c_void>;

/* Functions to be called by the debugger in specific events */
pub type lua_Hook = Option<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: c_int,
    pub name: *const c_char,
    pub namewhat: *const c_char,
    pub what: *const c_char,
    pub source: *const c_char,
    pub currentline: c_int,
    pub linedefined: c_int,
    pub lastlinedefined: c_int,
    pub nups: c_uchar,
    pub nparams: c_uchar,
    pub isvararg: c_char,
    pub istailcall: c_char,
    pub short_src: [c_char; LUA_IDSIZE],
    pub i_ci: *mut CallInfo,
}
