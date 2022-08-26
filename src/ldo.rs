/*
** Stack and Call structure of Lua
*/

use std::env;

use libc::{c_char, c_int, c_void, ptrdiff_t};

use crate::lobject::{StkId, TValue};
use crate::lstate::lua_State;

/*
** Macro to check stack size and grow stack if needed.  Parameters
** 'pre'/'pos' allow the macro to preserve a pointer into the
** stack across reallocations, doing the work only when needed.
** 'condmovestack' is used in heavy tests to force a stack reallocation
** at every check.
*/
pub unsafe fn luaD_checkstackaux(L: *mut lua_State, n: i32, mut pre: impl FnMut(), mut pos: impl FnMut()) {
    if (*L).stack_last.offset_from((*L).top) <= n as isize {
        pre();
        luaD_growstack(L, n);
        pos();
    } else {
        #[cfg(debug_assertions)]
        if env::var("LUA_HARDSTACKTESTS").as_deref() == Ok("1") {
            let sz = (*L).stacksize;
            pre();
            // realloc stack keeping its size
            luaD_reallocstack(L, sz);
            pos();
        }
    }
}

/* In general, 'pre'/'pos' are empty (nothing to save) */
pub unsafe fn luaD_checkstack(L: *mut lua_State, n: i32) {
    luaD_checkstackaux(L, n, || (), || ());
}

pub unsafe fn savestack(L: *mut lua_State, p: *const TValue) -> ptrdiff_t {
    (p as *const c_char).offset_from((*L).stack as *const c_char)
}

pub unsafe fn restorestack(L: *mut lua_State, n: ptrdiff_t) -> *mut TValue {
    ((*L).stack as *mut c_char).offset(n) as *mut TValue
}

/* type of protected functions, to be ran by 'runprotected' */
pub type Pfunc = Option<unsafe extern "C" fn(*mut lua_State, *mut c_void)>;

pub type jmp_buf = [libc::c_int; 37];

/* chain list of long jump buffers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_longjmp {
    pub previous: *mut lua_longjmp,
    pub b: jmp_buf,
    pub status: c_int,
}

extern "C" {
    pub fn luaD_call(L: *mut lua_State, func: StkId, nResults: c_int);
    pub fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: c_int);
    pub fn luaD_rawrunprotected(L: *mut lua_State, f: Pfunc, ud: *mut c_void) -> c_int;
    pub fn luaD_throw(L: *mut lua_State, errcode: c_int) -> !;
    pub fn luaD_hook(L: *mut lua_State, event: c_int, line: c_int);
    pub fn luaD_inctop(L: *mut lua_State);
    pub fn luaD_growstack(L: *mut lua_State, n: c_int);
    pub fn luaD_reallocstack(L: *mut lua_State, newsize: c_int);
}
