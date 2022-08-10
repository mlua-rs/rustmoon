use libc::{c_char, c_double, c_int, c_longlong, c_uchar, c_void, intptr_t, size_t};

use crate::llimits::LUA_IDSIZE;
use crate::lstate::{lua_State, CallInfo};

/* type of numbers in Lua */
pub type lua_Number = c_double;

/* type for integer functions */
pub type lua_Integer = c_longlong;

/* thread status */
pub const LUA_ERRMEM: c_int = 4;

/*
** basic types
*/
pub const LUA_NUMTAGS: usize = 9;

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
