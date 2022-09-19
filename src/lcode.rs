/*
** Code generator for Lua
*/

use libc::{c_uint, c_int};

use crate::lobject::{TValue, setivalue, setfltvalue};
use crate::lparser::{expdesc, VKINT, VKFLT};
use crate::types::lua_Integer;

pub const MAXREGS: libc::c_int = 255 as libc::c_int;

#[inline(always)]
pub unsafe extern "C" fn hasjumps (mut e: *const expdesc) -> bool {
    return (*e).t != (*e).f;
}

/*
** If expression is a numeric constant, fills 'v' with its value
** and returns 1. Otherwise, returns 0.
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn tonumeral(
    e: *const expdesc,
    v: *mut TValue,
) -> c_int {
    if hasjumps(e) {
        return 0;
    }
    match (*e).k as c_uint {
        VKINT => {
            if !v.is_null() {
                setivalue(v, (*e).u.ival);
            }
            return 1;
        }
        VKFLT => {
            if !v.is_null() {
                setfltvalue(v, (*e).u.nval);
            }
            return 1;
        }
        _ => return 0,
    };
}
