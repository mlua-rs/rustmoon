/*
** Lua API
*/

use std::ptr;

use libc::{c_char, c_int, c_void, size_t};

use crate::lstate::lua_State;
use crate::types::{
    lua_CFunction, lua_Integer, lua_KContext, lua_KFunction, lua_Number, LUA_MULTRET,
    LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS, LUA_TFUNCTION, LUA_TNIL,
};

pub(crate) unsafe fn api_incr_top(L: *mut lua_State) {
    (*L).top = (*L).top.add(1);
    debug_assert!((*L).top <= (*(*L).ci).top, "stack overflow");
}

pub(crate) unsafe fn adjustresults(L: *mut lua_State, nres: i32) {
    if nres == LUA_MULTRET && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top;
    }
}

pub(crate) unsafe fn api_checknelems(L: *mut lua_State, n: i32) {
    debug_assert!(
        (n as isize) < (*L).top.offset_from((*(*L).ci).func),
        "not enough elements in the stack"
    );
}

pub const fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_REGISTRYINDEX - i
}

pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -n - 1)
}

pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0)
}

pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0)
}

pub unsafe fn lua_pushglobaltable(L: *mut lua_State) {
    lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS);
}

pub unsafe fn lua_tostring(L: *mut lua_State, idx: c_int) -> *const c_char {
    lua_tolstring(L, idx, ptr::null_mut())
}

pub unsafe fn lua_isfunction(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TFUNCTION) as c_int
}

pub unsafe fn lua_isnil(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TNIL) as c_int
}

pub unsafe fn lua_insert(L: *mut lua_State, idx: c_int) {
    lua_rotate(L, idx, 1)
}

pub unsafe fn lua_remove(L: *mut lua_State, idx: c_int) {
    lua_rotate(L, idx, -1);
    lua_pop(L, 1);
}

pub unsafe fn lua_call(L: *mut lua_State, n: c_int, r: c_int) {
    lua_callk(L, n, r, 0, None)
}

pub unsafe fn lua_pcall(L: *mut lua_State, n: c_int, r: c_int, f: c_int) -> c_int {
    lua_pcallk(L, n, r, f, 0, None)
}

extern "C" {
    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;
    pub fn lua_absindex(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_gettop(L: *mut lua_State) -> c_int;
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
    pub fn lua_pushvalue(L: *mut lua_State, idx: c_int);
    pub fn lua_rotate(L: *mut lua_State, idx: c_int, n: c_int);
    pub fn lua_copy(L: *mut lua_State, fromidx: c_int, toidx: c_int);
    pub fn lua_checkstack(L: *mut lua_State, n: c_int) -> c_int;
    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);
    pub fn lua_isnumber(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isstring(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_iscfunction(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isinteger(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isuserdata(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_typename(L: *mut lua_State, t: c_int) -> *const c_char;
    pub fn lua_tonumberx(L: *mut lua_State, idx: c_int, pisnum: *mut c_int) -> lua_Number;
    pub fn lua_tointegerx(L: *mut lua_State, idx: c_int, pisnum: *mut c_int) -> lua_Integer;
    pub fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) -> *const c_char;
    pub fn lua_rawlen(L: *mut lua_State, idx: c_int) -> size_t;
    pub fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> lua_CFunction;
    pub fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
    pub fn lua_tothread(L: *mut lua_State, idx: c_int) -> *mut lua_State;
    pub fn lua_topointer(L: *mut lua_State, idx: c_int) -> *const c_void;
    pub fn lua_createtable(L: *mut lua_State, narray: c_int, nrec: c_int);
    pub fn lua_getfield(L: *mut lua_State, idx: c_int, k: *const c_char) -> c_int;
    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_pushboolean(L: *mut lua_State, b: c_int);
    pub fn lua_pushcclosure(L: *mut lua_State, r#fn: lua_CFunction, n: c_int);
    pub fn lua_pushlstring(L: *mut lua_State, s: *const c_char, len: size_t) -> *const c_char;
    pub fn lua_pushstring(L: *mut lua_State, s: *const c_char) -> *const c_char;
    pub fn lua_pushfstring(L: *mut lua_State, fmt: *const c_char, args: ...) -> *const c_char;
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
    pub fn lua_rawgeti(L: *mut lua_State, idx: c_int, n: lua_Integer) -> c_int;
    pub fn lua_rawgetp(L: *mut lua_State, idx: c_int, p: *const c_void) -> c_int;
    pub fn lua_rawseti(L: *mut lua_State, idx: c_int, n: lua_Integer);
    pub fn lua_rawsetp(L: *mut lua_State, idx: c_int, p: *const c_void);
    pub fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    pub fn lua_setmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    pub fn lua_pcallk(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        errfunc: c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> c_int;
    pub fn lua_callk(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    );
}
