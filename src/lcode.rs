/*
** Code generator for Lua
*/

use libc::{abs, c_int, c_uint};

use crate::lgc::luaC_barrier;
use crate::llex::luaX_syntaxerror;
use crate::llimits::{Instruction, MAX_INT, lu_byte};
use crate::lmem::luaM_growvector;
use crate::lobject::{setfltvalue, setivalue, TValue, ttisinteger, ttype, setnilvalue, setobj, GCObject};
use crate::lopcodes::{
    luaP_opmodes, GETARG_sBx, MAXARG_sBx, OpCode, SETARG_sBx, GETARG_A, GETARG_B, GETARG_C,
    GET_OPCODE, NO_REG, OP_JMP, OP_LOADNIL, OP_RETURN, OP_TEST, OP_TESTSET, POS_A, POS_B, POS_C,
    POS_OP, SETARG_A, SETARG_B, iABC, getOpMode, getBMode, OpArgN, getCMode, MAXARG_A, MAXARG_B, MAXARG_C, iABx, iAsBx, MAXARG_Bx, CREATE_ABx, CREATE_Ax, OP_EXTRAARG, OP_LOADK, OP_LOADKX, ISK, MAXARG_Ax,
};
use crate::lparser::{expdesc, FuncState, VKFLT, VKINT, VNONRELOC};
use crate::ltable::luaH_set;
use crate::lvm::luaV_rawequalobj;

pub const MAXREGS: c_int = 255;
pub const NO_JUMP: c_int = -1;

#[inline(always)]
pub unsafe extern "C" fn hasjumps(e: *const expdesc) -> bool {
    return (*e).t != (*e).f;
}

#[inline(always)]
unsafe fn luaK_codeAsBx(fs: *mut FuncState, o: OpCode, A: c_int, sBx: c_int) -> c_int {
    return luaK_codeABx(fs, o, A, (sBx + MAXARG_sBx as c_int) as c_uint); // This integer size manipulation is absolutely necessary.
}

#[inline(always)]
unsafe fn testTMode(m: usize) -> c_int {
    return luaP_opmodes[m] as c_int & (1 << 7) as c_int;
}

#[inline(always)]
unsafe extern "C" fn CREATE_ABC(o: OpCode, a: c_int, b: c_int, c: c_int) -> u32 {
    return (o as Instruction) << POS_OP
        | (a as Instruction) << POS_A
        | (b as Instruction) << POS_B
        | (c as Instruction) << POS_C;
}

/*
** If expression is a numeric constant, fills 'v' with its value
** and returns 1. Otherwise, returns 0.
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn tonumeral(e: *const expdesc, v: *mut TValue) -> c_int {
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
pub unsafe extern "C" fn luaK_nil(fs: *mut FuncState, mut from: c_int, n: c_int) {
    let previous: *mut Instruction;
    let mut l = from + n - 1; /* last register to set nil */
    if (*fs).pc > (*fs).lasttarget {
        /* no jumps to current position? */
        previous = &mut *((*(*fs).f).code).offset(((*fs).pc - 1) as isize) as *mut Instruction;
        if GET_OPCODE(*previous) == OP_LOADNIL {
            /* previous is LOADNIL? */
            let pfrom = GETARG_A(*previous); /* get previous range */
            let pl = pfrom + GETARG_B(*previous);
            if pfrom <= from && from <= pl + 1 || from <= pfrom && pfrom <= l + 1 {
                /* can connect both? */
                if pfrom < from {
                    /* from = min(from, pfrom) */
                    from = pfrom;
                }
                if pl > l {
                    /* l = max(l, pl) */
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
pub unsafe extern "C" fn getjump(fs: *mut FuncState, pc: c_int) -> c_int {
    let offset = GETARG_sBx(*((*(*fs).f).code.offset(pc as isize)));
    if offset == NO_JUMP {
        /* point to itself represents end of list */
        return NO_JUMP; /* end of list */
    } else {
        return pc + 1 + offset; /* turn offset into absolute position */
    };
}

/*
** Fix jump instruction at position 'pc' to jump to 'dest'.
** (Jump addresses are relative in Lua)
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn fixjump(fs: *mut FuncState, pc: libc::c_int, dest: libc::c_int) {
    let jmp: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize) as *mut Instruction;
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
pub unsafe extern "C" fn luaK_concat(fs: *mut FuncState, l1: *mut c_int, l2: c_int) {
    if l2 == NO_JUMP {
        return; /* nothing to concatenate? */
    } else {
        if *l1 == NO_JUMP {
            /* no original list? */
            *l1 = l2; /* 'l1' points to 'l2' */
        } else {
            let mut list = *l1;
            let mut next: c_int;
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
pub unsafe extern "C" fn luaK_ret(fs: *mut FuncState, first: c_int, nret: c_int) {
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
pub unsafe extern "C" fn getjumpcontrol(fs: *mut FuncState, pc: libc::c_int) -> *mut Instruction {
    let pi: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize);
    if pc >= 1 && testTMode(GET_OPCODE(*pi.offset(-1)) as usize) != 0 {
        return pi.offset(-1);
    } else {
        return pi;
    };
}

/*
** Patch destination register for a TESTSET instruction.
** If instruction in position 'node' is not a TESTSET, return 0 ("fails").
** Otherwise, if 'reg' is not 'NO_REG', set it as the destination
** register. Otherwise, change instruction to a simple 'TEST' (produces
** no register value)
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn patchtestreg(fs: *mut FuncState, node: c_int, reg: c_int) -> c_int {
    let i = getjumpcontrol(fs, node);
    if GET_OPCODE(*i) != OP_TESTSET {
        return 0; /* cannot patch other instructions */
    }
    if reg != NO_REG as c_int && reg != GETARG_B(*i) {
        SETARG_A(i, reg);
    } else {
        /* no register to put value or register already has the value;
        change instruction to simple test */
        *i = CREATE_ABC(OP_TEST, GETARG_B(*i), 0, GETARG_C(*i));
    }
    return 1 as libc::c_int;
}

/*
** Traverse a list of tests ensuring no one produces a value
*/
/*static void removevalues (FuncState *fs, int list) {
  for (; list != NO_JUMP; list = getjump(fs, list))
      patchtestreg(fs, list, NO_REG);
}
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn removevalues(fs: *mut FuncState, mut list: c_int) {
    while list != NO_JUMP {
        patchtestreg(fs, list, NO_REG as c_int);
        list = getjump(fs, list);
    }
}

/*
** Traverse a list of tests, patching their destination address and
** registers: tests producing values jump to 'vtarget' (and put their
** values in 'reg'), other tests jump to 'dtarget'.
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn patchlistaux(
    fs: *mut FuncState,
    mut list: c_int,
    vtarget: c_int,
    reg: c_int,
    dtarget: c_int,
) {
    while list != NO_JUMP {
        let next = getjump(fs, list);
        if patchtestreg(fs, list, reg) != 0 {
            fixjump(fs, list, vtarget);
        } else {
            fixjump(fs, list, dtarget);
        }
        list = next;
    }
}

/*
** Ensure all pending jumps to current position are fixed (jumping
** to current position with no values) and reset list of pending
** jumps
*/
// FIXME static
#[no_mangle]
pub unsafe extern "C" fn dischargejpc(mut fs: *mut FuncState) {
    patchlistaux(fs, (*fs).jpc, (*fs).pc, NO_REG as c_int, (*fs).pc);
    (*fs).jpc = NO_JUMP;
}

/*
** Add elements in 'list' to list of pending jumps to "here"
** (current position)
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_patchtohere(fs: *mut FuncState, list: c_int) {
    luaK_getlabel(fs);
    luaK_concat(fs, &mut (*fs).jpc, list);
}

/*
** Path all jumps in 'list' to jump to 'target'.
** (The assert means that we cannot fix a jump to a forward address
** because we only know addresses once code is generated.)
*/

#[no_mangle]
pub unsafe extern "C" fn luaK_patchlist(fs: *mut FuncState, list: c_int, target: c_int) {
    if target == (*fs).pc {
        luaK_patchtohere(fs, list);
    } else {
        patchlistaux(fs, list, target, NO_REG as c_int, target);
    };
}

/*
** Path all jumps in 'list' to close upvalues up to given 'level'
** (The assertion checks that jumps either were closing nothing
** or were closing higher levels, from inner blocks.)
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_patchclose(fs: *mut FuncState, mut list: c_int, mut level: c_int) {
    level += 1;
    while list != NO_JUMP {
        SETARG_A((*(*fs).f).code.offset(list as isize), level);
        list = getjump(fs, list);
    }
}

/*
 ** Emit instruction 'i', checking for array sizes and saving also its
 ** line information. Return 'i' position.
 */

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn luaK_code(mut fs: *mut FuncState, i: Instruction) -> c_int {
    let f = (*fs).f;
    dischargejpc(fs); /* 'pc' will change */
    /* put new instruction in code array */
    luaM_growvector(
        (*(*fs).ls).L,
        &mut (*f).code,
        (*fs).pc,
        &mut (*f).sizecode,
        MAX_INT,
        cstr!("opcodes"),
    );
    *((*f).code).offset((*fs).pc as isize) = i;
    /* save corresponding line information */
    luaM_growvector(
        (*(*fs).ls).L,
        &mut (*f).lineinfo,
        (*fs).pc,
        &mut (*f).sizelineinfo,
        MAX_INT,
        cstr!("opcodes"),
    );

    *((*f).lineinfo).offset((*fs).pc as isize) = (*(*fs).ls).lastline;
    let oldPc = (*fs).pc;
    (*fs).pc = (*fs).pc + 1;
    return oldPc;
}

/*
** Format and emit an 'iABC' instruction. (Assertions check consistency
** of parameters versus opcode.)
*/

#[no_mangle]
pub unsafe extern "C" fn luaK_codeABC(
    fs: *mut FuncState,
    o: OpCode,
    a: c_int,
    b: c_int,
    c: c_int,
) -> c_int {
    debug_assert!(getOpMode(o) == iABC);
    debug_assert!(getBMode(o) != OpArgN || b == 0);
    debug_assert!(getCMode(o) != OpArgN || c == 0);
    debug_assert!(a <= MAXARG_A as c_int && b <= MAXARG_B as c_int && c <= MAXARG_C as c_int);
    return luaK_code(fs, CREATE_ABC(o, a, b, c));
}

/*
** Format and emit an 'iABx' instruction.
*/

#[no_mangle]
pub unsafe extern "C" fn luaK_codeABx(
    fs: *mut FuncState,
    o: OpCode,
    a: c_int,
    bc: c_uint,
) -> c_int {
    debug_assert!(getOpMode(o) == iABx || getOpMode(o) == iAsBx);
    debug_assert!(getCMode(o) == OpArgN);
    debug_assert!(a <= MAXARG_A as i32 && bc <= MAXARG_Bx);
    return luaK_code(fs, CREATE_ABx(o, a, bc));
}

/*
** Emit an "extra argument" instruction (format 'iAx')
*/
/* 
static int codeextraarg (FuncState *fs, int a) {
    lua_assert(a <= MAXARG_Ax);
    return luaK_code(fs, CREATE_Ax(OP_EXTRAARG, a));
  }
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn codeextraarg(
    mut fs: *mut FuncState,
    mut a: c_int,
) -> c_int {
    return luaK_code(fs, CREATE_Ax(OP_EXTRAARG, a));
}

/*
** Emit a "load constant" instruction, using either 'OP_LOADK'
** (if constant index 'k' fits in 18 bits) or an 'OP_LOADKX'
** instruction with "extra argument".
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_codek(
    fs: *mut FuncState,
    reg: c_int,
    k: c_int,
) -> libc::c_int {
    if k as c_uint <= MAXARG_Bx {
        return luaK_codeABx(fs, OP_LOADK, reg, k as c_uint)
    } else {
        let p = luaK_codeABx(fs, OP_LOADKX, reg, 0);
        codeextraarg(fs, k);
        return p;
    };
}

/*
** Check register-stack level, keeping track of its maximum size
** in field 'maxstacksize'
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_checkstack(mut fs: *mut FuncState, n: c_int) {
    let newstack = (*fs).freereg as c_int + n;
    if newstack > (*(*fs).f).maxstacksize as c_int {
        if newstack >= MAXREGS {
            luaX_syntaxerror((*fs).ls, cstr!("function or expression needs too many registers"));
        }
        (*(*fs).f).maxstacksize = newstack as lu_byte;
    }
}

/*
** Reserve 'n' registers in register stack
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_reserveregs(mut fs: *mut FuncState, n: c_int) {
    luaK_checkstack(fs, n);
    (*fs).freereg = (*fs).freereg + n as lu_byte;
}

/*
** Free register 'reg', if it is neither a constant index nor
** a local variable.
)
*/
// FIXME static
#[no_mangle]
pub unsafe extern "C" fn freereg(mut fs: *mut FuncState, reg: c_int) {
    if !ISK(reg as c_uint) && reg >= (*fs).nactvar as c_int {
        (*fs).freereg = ((*fs).freereg).wrapping_sub(1);
    }
}

/*
** Free register used by expression 'e' (if any)
*/
/*static void freeexp (FuncState *fs, expdesc *e) {
    if (e->k == VNONRELOC)
      freereg(fs, e->u.info);
  }*/
#[no_mangle]
pub unsafe extern "C" fn freeexp(fs: *mut FuncState, e: *mut expdesc) {
    if (*e).k as libc::c_uint == VNONRELOC as c_uint {
        freereg(fs, (*e).u.info);
    }
}

/*
** Free registers used by expressions 'e1' and 'e2' (if any) in proper
** order.
*/
#[no_mangle]
pub unsafe extern "C" fn freeexps(
    fs: *mut FuncState,
    e1: *mut expdesc,
    e2: *mut expdesc,
) {
    let r1: c_int = if (*e1).k as c_uint == VNONRELOC as c_uint { (*e1).u.info } else { -1 };
    let r2: c_int = if (*e2).k as c_uint == VNONRELOC as c_uint { (*e2).u.info } else { -1 };
    if r1 > r2 {
        freereg(fs, r1);
        freereg(fs, r2);
    } else {
        freereg(fs, r2);
        freereg(fs, r1);
    };
}

/*
** Add constant 'v' to prototype's list of constants (field 'k').
** Use scanner's table to cache position of constants in constant list
** and try to reuse constants. Because some values should not be used
** as keys (nil cannot be a key, integer keys can collapse with float
** keys), the caller must provide a useful 'key' for indexing the cache.
*/

// FIXME - static
#[no_mangle]
pub unsafe extern "C" fn addk(
    mut fs: *mut FuncState,
    key: *mut TValue,
    v: *mut TValue,
) -> c_int {
    let L = (*(*fs).ls).L;
    let f = (*fs).f;
    let idx = luaH_set(L, (*(*fs).ls).h, key); /* index scanner table */
    let mut k: libc::c_int = 0;
    let mut oldsize: c_int = 0;
    if ttisinteger(idx) {  /* is there an index there? */
        k = (*idx).value_.i as c_int;
        /* correct value? (warning: must distinguish floats from integers!) */
        if k < (*fs).nk && ttype((*f).k.offset(k as isize)) == ttype(v)
            && luaV_rawequalobj((*f).k.offset(k as isize), v) != 0
        {
            return k; /* reuse index */
        }
    }
    /* constant not found; create a new entry */
    oldsize = (*f).sizek;
    k = (*fs).nk;
    /* numerical value does not need GC barrier;
       table has no metatable, so it does not need to invalidate cache */
    setivalue(idx, k as i64);
    luaM_growvector(L, &mut (*f).k, k, &mut (*f).sizek, MAXARG_Ax as c_int, cstr!("constants"));
    while oldsize < (*f).sizek {
        setnilvalue((*f).k.offset(oldsize as isize));
        oldsize += 1;
    }
    setobj(L, (*f).k.offset(k as isize), v);
    (*fs).nk += 1;
    luaC_barrier(L, f as *mut GCObject, v);
    return k;
}