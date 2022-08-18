/*
** Lua tables (hash)
*/

use libc::c_uint;

use crate::lobject::{TString, TValue, Table};
use crate::lstate::lua_State;
use crate::types::lua_Integer;

extern "C" {
    pub fn luaH_new(L: *mut lua_State) -> *mut Table;
    pub fn luaH_resize(L: *mut lua_State, t: *mut Table, nasize: c_uint, nhsize: c_uint);
    pub fn luaH_setint(L: *mut lua_State, t: *mut Table, key: lua_Integer, value: *mut TValue);
    pub fn luaH_getshortstr(t: *mut Table, key: *mut TString) -> *const TValue;
}
