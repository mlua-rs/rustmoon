/*
** Stack and Call structure of Lua
*/

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
// #define luaD_checkstackaux(L,n,pre,pos)  \
// 	if (L->stack_last - L->top <= (n)) \
// 	  { pre; luaD_growstack(L, n); pos; } else { condmovestack(L,pre,pos); }

/* In general, 'pre'/'pos' are empty (nothing to save) */
// #define luaD_checkstack(L,n)	luaD_checkstackaux(L,n,(void)0,(void)0)

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
}
