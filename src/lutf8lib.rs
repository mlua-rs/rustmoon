

use crate::lstate::lua_State;
use libc::{c_int};


/*
** codepoint(s, [i, [j]])  -> returns codepoints for all characters
** that start in the range [i,j]
*/
#[no_mangle]
pub unsafe extern "C" fn codepoint(mut L: *mut lua_State) -> libc::c_int {
    // TODO fill in implementation later :)
    return 1;
}
