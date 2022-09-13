use crate::lstate::lua_State;
use crate::types::lua_Integer;

extern "C" {
    fn lua_seti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer);
}

pub type IdxT = libc::c_uint;

#[no_mangle]
pub unsafe extern "C" fn set2(mut L: *mut lua_State, mut i: IdxT, mut j: IdxT) {
    lua_seti(L, 1 as libc::c_int, i as lua_Integer);
    lua_seti(L, 1 as libc::c_int, j as lua_Integer);
}
