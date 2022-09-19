/*
** Code generator for Lua
*/

use libc::{c_uint, c_int};

use crate::llimits::Instruction;
use crate::lobject::{TValue, setivalue, setfltvalue};
use crate::lopcodes::{GET_OPCODE, OP_LOADNIL, GETARG_A, GETARG_B, SETARG_A, SETARG_B, OpCode, GETARG_sBx};
use crate::lparser::{expdesc, VKINT, VKFLT, FuncState};

pub const MAXREGS: c_int = 255;
pub const NO_JUMP: c_int = -1;

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

/*
** Create a OP_LOADNIL instruction, but try to optimize: if the previous
** instruction is also OP_LOADNIL and ranges are compatible, adjust
** range of previous instruction instead of emitting a new one. (For
** instance, 'local a; local b' will generate a single opcode.)
*/

#[no_mangle]
pub unsafe extern "C" fn luaK_nil(
    fs: *mut FuncState,
    mut from: c_int,
    n: c_int,
) {
    let mut previous = 0 as *mut Instruction;
    let mut l = from + n - 1; /* last register to set nil */
    if (*fs).pc > (*fs).lasttarget {  /* no jumps to current position? */
        previous = &mut *((*(*fs).f).code).offset(((*fs).pc - 1) as isize)
            as *mut Instruction;
        if GET_OPCODE(*previous) == OP_LOADNIL { /* previous is LOADNIL? */
            let pfrom = GETARG_A(*previous); /* get previous range */
            let pl = pfrom + GETARG_B(*previous);
            if pfrom <= from && from <= pl + 1 || from <= pfrom && pfrom <= l + 1 { /* can connect both? */
                if pfrom < from { /* from = min(from, pfrom) */
                    from = pfrom;
                }
                if pl > l {  /* l = max(l, pl) */
                    l = pl;
                }
                SETARG_A(previous, from);
                SETARG_B(previous, l - from);
                return;
            }
        }
    }
    luaK_codeABC(fs, OP_LOADNIL, from, n - 1, 0);
}

/*
** Gets the destination address of a jump instruction. Used to traverse
** a list of jumps.
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn getjump(
    fs: *mut FuncState,
    pc: c_int,
) -> c_int {
    let offset = GETARG_sBx(*((*(*fs).f).code.offset(pc as isize)));
    if offset == NO_JUMP { /* point to itself represents end of list */
        return NO_JUMP /* end of list */
    } else {
        return pc + 1 + offset /* turn offset into absolute position */
    };
}

extern "C" {
    pub fn luaK_codeABC(
        fs: *mut FuncState,
        o: OpCode,
        a: c_int,
        b: c_int,
        c: c_int,
    ) -> c_int;
}