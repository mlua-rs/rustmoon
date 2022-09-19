/*
** Code generator for Lua
*/

use libc::{c_uint, c_int, abs, c_char, c_uchar};

use crate::llex::luaX_syntaxerror;
use crate::llimits::{Instruction, lu_byte};
use crate::lobject::{TValue, setivalue, setfltvalue};
use crate::lopcodes::{GET_OPCODE, OP_LOADNIL, GETARG_A, GETARG_B, SETARG_A, SETARG_B, OpCode, GETARG_sBx, MAXARG_sBx, SETARG_sBx, OP_JMP, OP_RETURN, luaP_opmodes};
use crate::lparser::{expdesc, VKINT, VKFLT, FuncState};

pub const MAXREGS: c_int = 255;
pub const NO_JUMP: c_int = -1;

#[inline(always)]
pub unsafe extern "C" fn hasjumps (e: *const expdesc) -> bool {
    return (*e).t != (*e).f;
}

#[inline(always)]
unsafe fn luaK_codeAsBx(fs: *mut FuncState, o: OpCode, A: c_int, sBx: c_int) -> c_int {
    return luaK_codeABx(fs,o,A,(sBx+MAXARG_sBx as c_int) as c_uint); // This integer size manipulation is absolutely necessary.
}

#[inline(always)]
unsafe fn testTMode(m: usize) -> c_int {
    return luaP_opmodes[m] as c_int & (1 << 7) as c_int;
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

/*
** Fix jump instruction at position 'pc' to jump to 'dest'.
** (Jump addresses are relative in Lua)
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn fixjump(
    fs: *mut FuncState,
    pc: libc::c_int,
    dest: libc::c_int,
) {
    let jmp: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize)
        as *mut Instruction;
    let offset = dest - (pc + 1 as libc::c_int);
    if abs(offset) as c_uint > MAXARG_sBx {
        luaX_syntaxerror((*fs).ls, cstr!("control structure too long"));
    }
    SETARG_sBx(jmp, offset);
}

/*
** Concatenate jump-list 'l2' into jump-list 'l1'
*/

#[no_mangle]
pub unsafe extern "C" fn luaK_concat(
      fs: *mut FuncState,
      l1: *mut c_int,
      l2: c_int,
) {
    if l2 == NO_JUMP {
        return /* nothing to concatenate? */
    } else {
        if *l1 == NO_JUMP { /* no original list? */
            *l1 = l2; /* 'l1' points to 'l2' */
        } else {
            let mut list = *l1;
            let mut next: c_int = 0;
            loop {
                next = getjump(fs, list); /* find last element */
                if !(next != NO_JUMP) {
                    break;
                }
                list = next;
            }
            fixjump(fs, list, l2); /* last element links to 'l2' */
        }
    };
}

/*
** Create a jump instruction and return its position, so its destination
** can be fixed later (with 'fixjump'). If there are jumps to
** this position (kept in 'jpc'), link them all together so that
** 'patchlistaux' will fix all them directly to the final destination.
*/

#[no_mangle]
pub unsafe extern "C" fn luaK_jump(mut fs: *mut FuncState) -> c_int {
    let jpc = (*fs).jpc; /* save list of jumps to here */
    (*fs).jpc = NO_JUMP; /* no more jumps to here */
    let mut j: c_int = luaK_codeAsBx(fs, OP_JMP, 0, NO_JUMP);
    luaK_concat(fs, &mut j, jpc); /* keep them on hold */
    return j;
}

/*
** Code a 'return' instruction
*/

#[no_mangle]
pub unsafe extern "C" fn luaK_ret(
    fs: *mut FuncState,
    first: c_int,
    nret: c_int,
) {
    luaK_codeABC(fs, OP_RETURN, first, nret + 1, 0);
}

/*
** Code a "conditional jump", that is, a test or comparison opcode
** followed by a jump. Return jump position.
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn condjump(
    fs: *mut FuncState,
    op: OpCode,
    A: c_int,
    B: c_int,
    C: c_int,
) -> c_int {
    luaK_codeABC(fs, op, A, B, C);
    return luaK_jump(fs);
}
  
/*
** returns current 'pc' and marks it as a jump target (to avoid wrong
** optimizations with consecutive instructions not in the same basic block).
*/

#[no_mangle]
pub unsafe extern "C" fn luaK_getlabel(mut fs: *mut FuncState) -> libc::c_int {
    (*fs).lasttarget = (*fs).pc;
    return (*fs).pc;
}

/*
** Returns the position of the instruction "controlling" a given
** jump (that is, its condition), or the jump itself if it is
** unconditional.
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn getjumpcontrol(
    fs: *mut FuncState,
    pc: libc::c_int,
) -> *mut Instruction {
    let pi: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize);
    if pc >= 1 && testTMode(GET_OPCODE(*pi.offset(-1)) as usize) != 0 {
        return pi.offset(-1)
    } else {
        return pi
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
    pub fn luaK_codeABx(
        fs: *mut FuncState,
        o: OpCode,
        a: c_int,
        bc: c_uint,
    ) -> c_int;
}