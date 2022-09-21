/*
** Code generator for Lua
*/

use libc::{abs, c_int, c_uint, c_void, size_t};

use crate::lgc::luaC_barrier;
use crate::llex::luaX_syntaxerror;
use crate::llimits::{lu_byte, Instruction, MAX_INT};
use crate::lmem::luaM_growvector;
use crate::lobject::{
    setbvalue, setfltvalue, sethvalue, setivalue, setnilvalue, setobj, setpvalue, setsvalue,
    ttisinteger, ttype, GCObject, TString, TValue, Value,
};
use crate::lopcodes::{
    getBMode, getCMode, getOpMode, iABC, iABx, iAsBx, luaP_opmodes, CREATE_ABx, CREATE_Ax,
    GETARG_sBx, MAXARG_Ax, MAXARG_Bx, MAXARG_sBx, OpArgN, OpCode, SETARG_sBx, GETARG_A, GETARG_B,
    GETARG_C, GET_OPCODE, ISK, MAXARG_A, MAXARG_B, MAXARG_C, NO_REG, OP_EXTRAARG, OP_JMP, OP_LOADK,
    OP_LOADKX, OP_LOADNIL, OP_RETURN, OP_TEST, OP_TESTSET, POS_A, POS_B, POS_C, POS_OP, SETARG_A,
    SETARG_B, SETARG_C, OP_GETUPVAL, OP_MOVE, OP_GETTABLE, OP_GETTABUP, OP_LOADBOOL, MAXINDEXRK, RKASK, OP_SETUPVAL, OP_SETTABLE, OP_SETTABUP, OP_SELF, OP_NOT,
};
use crate::lparser::{expdesc, FuncState, VCALL, VKFLT, VKINT, VNONRELOC, VVARARG, VRELOCABLE, VLOCAL, VTRUE, VJMP, VUPVAL, VK};
use crate::ltable::luaH_set;
use crate::lvm::luaV_rawequalobj;
use crate::types::{lua_Integer, lua_Number};

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

#[inline(always)]
unsafe fn getinstruction(fs: *mut FuncState, e: *mut expdesc) -> *mut Instruction {
    return (*(*fs).f).code.offset((*e).u.info as isize);
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
pub unsafe extern "C" fn codeextraarg(fs: *mut FuncState, a: c_int) -> c_int {
    return luaK_code(fs, CREATE_Ax(OP_EXTRAARG, a));
}

/*
** Emit a "load constant" instruction, using either 'OP_LOADK'
** (if constant index 'k' fits in 18 bits) or an 'OP_LOADKX'
** instruction with "extra argument".
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_codek(fs: *mut FuncState, reg: c_int, k: c_int) -> libc::c_int {
    if k as c_uint <= MAXARG_Bx {
        return luaK_codeABx(fs, OP_LOADK, reg, k as c_uint);
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
            luaX_syntaxerror(
                (*fs).ls,
                cstr!("function or expression needs too many registers"),
            );
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
pub unsafe extern "C" fn freeexps(fs: *mut FuncState, e1: *mut expdesc, e2: *mut expdesc) {
    let r1: c_int = if (*e1).k as c_uint == VNONRELOC as c_uint {
        (*e1).u.info
    } else {
        -1
    };
    let r2: c_int = if (*e2).k as c_uint == VNONRELOC as c_uint {
        (*e2).u.info
    } else {
        -1
    };
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
pub unsafe extern "C" fn addk(mut fs: *mut FuncState, key: *mut TValue, v: *mut TValue) -> c_int {
    let L = (*(*fs).ls).L;
    let f = (*fs).f;
    let idx = luaH_set(L, (*(*fs).ls).h, key); /* index scanner table */
    let mut k: libc::c_int;
    let mut oldsize: c_int;
    if ttisinteger(idx) {
        /* is there an index there? */
        k = (*idx).value_.i as c_int;
        /* correct value? (warning: must distinguish floats from integers!) */
        if k < (*fs).nk
            && ttype((*f).k.offset(k as isize)) == ttype(v)
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
    luaM_growvector(
        L,
        &mut (*f).k,
        k,
        &mut (*f).sizek,
        MAXARG_Ax as c_int,
        cstr!("constants"),
    );
    while oldsize < (*f).sizek {
        setnilvalue((*f).k.offset(oldsize as isize));
        oldsize += 1;
    }
    setobj(L, (*f).k.offset(k as isize), v);
    (*fs).nk += 1;
    luaC_barrier(L, f as *mut GCObject, v);
    return k;
}

/*
** Add a string to list of constants and return its index.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_stringK(fs: *mut FuncState, s: *mut TString) -> libc::c_int {
    let mut o = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    setsvalue((*(*fs).ls).L, &mut o, s);
    return addk(fs, &mut o, &mut o); /* use string itself as key */
}

/*
** Add an integer to list of constants and return its index.
** Integers use userdata as keys to avoid collision with floats with
** same value; conversion to 'void*' is used only for hashing, so there
** are no "precision" problems.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_intK(fs: *mut FuncState, n: lua_Integer) -> libc::c_int {
    let mut k = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut o = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    setpvalue(&mut k, n as size_t as *mut c_void);
    setivalue(&mut o, n);
    return addk(fs, &mut k, &mut o);
}
/*
** Add a float to list of constants and return its index.
*/
// FIXME static
#[no_mangle]
pub unsafe extern "C" fn luaK_numberK(fs: *mut FuncState, r: lua_Number) -> c_int {
    let mut o = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    setfltvalue(&mut o, r);
    return addk(fs, &mut o, &mut o); /* use number itself as key */
}

/*
** Add a boolean to list of constants and return its index.
*/
// FIXME static
#[no_mangle]
pub unsafe extern "C" fn boolK(fs: *mut FuncState, b: c_int) -> c_int {
    let mut o = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    setbvalue(&mut o, b != 0);
    return addk(fs, &mut o, &mut o); /* use boolean itself as key */
}

/*
** Add nil to list of constants and return its index.
*/
// FIXME static
#[no_mangle]
pub unsafe extern "C" fn nilK(fs: *mut FuncState) -> libc::c_int {
    let mut k = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut v = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    setnilvalue(&mut v);
    /* cannot use nil as key; instead use table itself to represent nil */
    sethvalue((*(*fs).ls).L, &mut k, (*(*fs).ls).h);
    return addk(fs, &mut k, &mut v);
}

/*
** Fix an expression to return the number of results 'nresults'.
** Either 'e' is a multi-ret expression (function call or vararg)
** or 'nresults' is LUA_MULTRET (as any expression can satisfy that).
*/

#[no_mangle]
pub unsafe extern "C" fn luaK_setreturns(fs: *mut FuncState, e: *mut expdesc, nresults: c_int) {
    if (*e).k as c_uint == VCALL {
        /* expression is an open function call? */
        SETARG_C(getinstruction(fs, e), nresults + 1);
    } else if (*e).k as c_uint == VVARARG {
        let pc: *mut Instruction = getinstruction(fs, e);
        SETARG_B(pc, nresults + 1);
        SETARG_A(pc, (*fs).freereg as c_int);
        luaK_reserveregs(fs, 1);
    }
}

/*
** Fix an expression to return one result.
** If expression is not a multi-ret expression (function call or
** vararg), it already returns one result, so nothing needs to be done.
** Function calls become VNONRELOC expressions (as its result comes
** fixed in the base register of the call), while vararg expressions
** become VRELOCABLE (as OP_VARARG puts its results where it wants).
** (Calls are created returning one result, so that does not need
** to be fixed.)
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_setoneret(fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k == VCALL { /* expression is an open function call? */
         /* already returns 1 value */
         debug_assert!(GETARG_C(*getinstruction(fs, e)) == 2);
        (*e).k = VNONRELOC; /* result has fixed position */
        (*e).u.info = GETARG_A(*getinstruction(fs, e));
    } else if (*e).k== VVARARG {
        SETARG_B(getinstruction(fs, e), 2);
        (*e).k = VRELOCABLE; /* can relocate its simple result */
    }
}

/*
** Ensure that expression 'e' is not a variable.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_dischargevars(
    fs: *mut FuncState,
    mut e: *mut expdesc,
) {
    match (*e).k as libc::c_uint {
        8 => { // VLOCAL /* already in a register */
            (*e).k = VNONRELOC; /* becomes a non-relocatable value */
        }
        9 => { // VUPVAL /* move value to some (pending) register */
            (*e)
                .u
                .info = luaK_codeABC(
                fs,
                OP_GETUPVAL,
                0 as libc::c_int,
                (*e).u.info,
                0 as libc::c_int,
            );
            (*e).k = VRELOCABLE;
        }
        10 => { // VINDEXED
            let op;
            freereg(fs, (*e).u.ind.idx as libc::c_int);
            if (*e).u.ind.vt as libc::c_int == VLOCAL as libc::c_int { /* is 't' in a register? */
                freereg(fs, (*e).u.ind.t as libc::c_int);
                op = OP_GETTABLE;
            } else {
                op = OP_GETTABUP; /* 't' is in an upvalue */
            }
            (*e)
                .u
                .info = luaK_codeABC(
                fs,
                op,
                0,
                (*e).u.ind.t as c_int,
                (*e).u.ind.idx as c_int,
            );
            (*e).k = VRELOCABLE;
        }
        14 | 13 => { // VVARARG | VCALL
            luaK_setoneret(fs, e);
        }
        _ => {} /* there is one value available (somewhere) */
    };
  }

/*
** Ensures expression value is in register 'reg' (and therefore
** 'e' will become a non-relocatable expression).
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn discharge2reg(
    fs: *mut FuncState,
    mut e: *mut expdesc,
    reg: libc::c_int,
) {
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        1 => {
            luaK_nil(fs, reg, 1 as libc::c_int);
        }
        3 | 2 => {
            luaK_codeABC(
                fs,
                OP_LOADBOOL,
                reg,
                ((*e).k as libc::c_uint == VTRUE as libc::c_int as libc::c_uint)
                    as libc::c_int,
                0,
            );
        }
        4 => {
            luaK_codek(fs, reg, (*e).u.info);
        }
        5 => {
            luaK_codek(fs, reg, luaK_numberK(fs, (*e).u.nval));
        }
        6 => {
            luaK_codek(fs, reg, luaK_intK(fs, (*e).u.ival));
        }
        12 => {
            let pc: *mut Instruction = getinstruction(fs, e);
            SETARG_A(pc, reg); /* instruction will put result in 'reg' */
        }
        7 => {
            if reg != (*e).u.info {
                luaK_codeABC(fs, OP_MOVE, reg, (*e).u.info, 0 as libc::c_int);
            }
        }
        _ => {
            debug_assert!((*e).k == VJMP);
            /* nothing to do... */
            return;
        }
    }
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}

/*
** Ensures expression value is in any register.
*/
// FIXME static
#[no_mangle]
pub unsafe extern "C" fn discharge2anyreg(fs: *mut FuncState, e: *mut expdesc) {
    if (*e).k as libc::c_uint != VNONRELOC as c_uint { /* no fixed register yet? */
        luaK_reserveregs(fs, 1); /* get a register */
        discharge2reg(fs, e, (*fs).freereg as libc::c_int - 1); /* put value there */
    }
}

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn code_loadbool(
    fs: *mut FuncState,
    A: c_int,
    b: c_int,
    jump: c_int,
) -> libc::c_int {
    luaK_getlabel(fs); /* those instructions may be jump targets */
    return luaK_codeABC(fs, OP_LOADBOOL, A, b, jump);
}

/*
** check whether list has any jump that do not produce a value
** or produce an inverted value
*/
/*static int need_value (FuncState *fs, int list) {
    for (; list != NO_JUMP; list = getjump(fs, list)) {
      Instruction i = *getjumpcontrol(fs, list);
      if (GET_OPCODE(i) != OP_TESTSET) return 1;
    }
    return 0;  /* not found */
  }*/
// FIXME static
#[no_mangle]
pub unsafe extern "C" fn need_value(
    fs: *mut FuncState,
    mut list: c_int,
) -> c_int {
    while list != NO_JUMP {
        let i = *getjumpcontrol(fs, list);
        if GET_OPCODE(i) as c_uint != OP_TESTSET as c_uint {
            return 1
        }
        list = getjump(fs, list);
    }
    return 0;
}

/*
** Ensures final expression result (including results from its jump
** lists) is in register 'reg'.
** If expression has jumps, need to patch these jumps either to
** its final position or to "load" instructions (for those tests
** that do not produce values).
*/
/*static void exp2reg (FuncState *fs, expdesc *e, int reg) {
    discharge2reg(fs, e, reg);
    if (e->k == VJMP)  /* expression itself is a test? */
      luaK_concat(fs, &e->t, e->u.info);  /* put this jump in 't' list */
    if (hasjumps(e)) {
      int final;  /* position after whole expression */
      int p_f = NO_JUMP;  /* position of an eventual LOAD false */
      int p_t = NO_JUMP;  /* position of an eventual LOAD true */
      if (need_value(fs, e->t) || need_value(fs, e->f)) {
        int fj = (e->k == VJMP) ? NO_JUMP : luaK_jump(fs);
        p_f = code_loadbool(fs, reg, 0, 1);
        p_t = code_loadbool(fs, reg, 1, 0);
        luaK_patchtohere(fs, fj);
      }
      final = luaK_getlabel(fs);
      patchlistaux(fs, e->f, final, reg, p_f);
      patchlistaux(fs, e->t, final, reg, p_t);
    }
    e->f = e->t = NO_JUMP;
    e->u.info = reg;
    e->k = VNONRELOC;
  }*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn exp2reg(
    fs: *mut FuncState,
    mut e: *mut expdesc,
    reg: c_int,
) {
    discharge2reg(fs, e, reg);
    if (*e).k as libc::c_uint == VJMP as libc::c_uint {
        luaK_concat(fs, &mut (*e).t, (*e).u.info);
    }
    if hasjumps(e) {
        let final_0;
        let mut p_f = NO_JUMP;
        let mut p_t = NO_JUMP;
        if need_value(fs, (*e).t) != 0 || need_value(fs, (*e).f) != 0 {
            let fj = if (*e).k as libc::c_uint == VJMP as libc::c_uint
            {
                NO_JUMP
            } else {
                luaK_jump(fs)
            };
            p_f = code_loadbool(fs, reg, 0, 1);
            p_t = code_loadbool(fs, reg, 1, 0);
            luaK_patchtohere(fs, fj);
        }
        final_0 = luaK_getlabel(fs);
        patchlistaux(fs, (*e).f, final_0, reg, p_f);
        patchlistaux(fs, (*e).t, final_0, reg, p_t);
    }
    (*e).t = NO_JUMP;
    (*e).f = (*e).t;
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}

/*
** Ensures final expression result (including results from its jump
** lists) is in next available register.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2nextreg(fs: *mut FuncState, e: *mut expdesc) {
    luaK_dischargevars(fs, e);
    freeexp(fs, e);
    luaK_reserveregs(fs, 1);
    exp2reg(fs, e, (*fs).freereg as libc::c_int - 1);
}

/*
** Ensures final expression result (including results from its jump
** lists) is in some (any) register and return that register.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2anyreg(
    fs: *mut FuncState,
    e: *mut expdesc,
) -> libc::c_int {
    luaK_dischargevars(fs, e);
    if (*e).k as libc::c_uint == VNONRELOC as libc::c_uint { /* expression already has a register? */
        if !hasjumps(e) { /* no jumps? */
            return (*e).u.info; /* result is already in a register */
        }
        if (*e).u.info >= (*fs).nactvar as libc::c_int { /* reg. is not a local? */
            exp2reg(fs, e, (*e).u.info);  /* put final result in it */
            return (*e).u.info;
        }
    }
    luaK_exp2nextreg(fs, e); /* otherwise, use next available register */
    return (*e).u.info;
}

/*
** Ensures final expression result is either in a register or in an
** upvalue.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2anyregup(fs: *mut FuncState, e: *mut expdesc) {
    if (*e).k as libc::c_uint != VUPVAL as c_uint || !hasjumps(e) {
        luaK_exp2anyreg(fs, e);
    }
}
/*
** Ensures final expression result is either in a register or it is
** a constant.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2val(fs: *mut FuncState, e: *mut expdesc) {
    if hasjumps(e) {
        luaK_exp2anyreg(fs, e);
    } else {
        luaK_dischargevars(fs, e);
    };
}
  
  
  /*
  ** Ensures final expression result is in a valid R/K index
  ** (that is, it is either in a register or in 'k' with an index
  ** in the range of R/K indices).
  ** Returns R/K index.
  */
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2RK(
    fs: *mut FuncState,
    mut e: *mut expdesc,
) -> libc::c_int {
    luaK_exp2val(fs, e);
    let mut doVk = true;
    match (*e).k as libc::c_uint {  /* move constants to 'k' */
        2 => { // VTRUE
            (*e).u.info = boolK(fs, 1);
        }
        3 => { // VFALSE
            (*e).u.info = boolK(fs, 0);
        }
        1 => { // VNIL
            (*e).u.info = nilK(fs);
        }
        6 => { // VKINT
            (*e).u.info = luaK_intK(fs, (*e).u.ival);
        }
        5 => { // VKFLT
            (*e).u.info = luaK_numberK(fs, (*e).u.nval);
        }
        4 => { // VK
        }
        _ => {
            doVk = false;
        }
    }
    if doVk {
        (*e).k = VK;
        if (*e).u.info as c_uint <= MAXINDEXRK {
            return RKASK((*e).u.info as c_uint) as c_int;
        }
    }
    return luaK_exp2anyreg(fs, e);
}

/*
** Generate code to store result of expression 'ex' into variable 'var'.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_storevar(
    fs: *mut FuncState,
    var: *mut expdesc,
    ex: *mut expdesc,
) {
    match (*var).k as libc::c_uint {
        8 => { // VLOCAL
            freeexp(fs, ex);
            exp2reg(fs, ex, (*var).u.info); /* compute 'ex' into proper place */
            return;
        }
        9 => { // VUPVAL
            let e = luaK_exp2anyreg(fs, ex);
            luaK_codeABC(fs, OP_SETUPVAL, e, (*var).u.info, 0 as libc::c_int);
        }
        10 => { // VINDEXED
            let op = (if (*var).u.ind.vt as libc::c_int == VLOCAL as libc::c_int {
                OP_SETTABLE as libc::c_int
            } else {
                OP_SETTABUP as libc::c_int
            }) as OpCode;
            let e_0 = luaK_exp2RK(fs, ex);
            luaK_codeABC(
                fs,
                op,
                (*var).u.ind.t as libc::c_int,
                (*var).u.ind.idx as libc::c_int,
                e_0,
            );
        }
        _ => {
            debug_assert!(false);  /* invalid var kind to store */
        }
    }
    freeexp(fs, ex);
}

/*
** Emit SELF instruction (convert expression 'e' into 'e:key(e,').
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_self(
    fs: *mut FuncState,
    mut e: *mut expdesc,
    key: *mut expdesc,
) {
    let ereg: libc::c_int;
    luaK_exp2anyreg(fs, e);
    ereg = (*e).u.info; /* register where 'e' was placed */
    freeexp(fs, e);
    (*e).u.info = (*fs).freereg as libc::c_int; /* base register for op_self */
    (*e).k = VNONRELOC; /* self expression has a fixed register */
    luaK_reserveregs(fs, 2 as libc::c_int);  /* function and 'self' produced by op_self */
    luaK_codeABC(fs, OP_SELF, (*e).u.info, ereg, luaK_exp2RK(fs, key));
    freeexp(fs, key);
}

/*
** Negate condition 'e' (where 'e' is a comparison).
*/

// FIXME static
#[no_mangle]
pub unsafe extern "C" fn negatecondition(fs: *mut FuncState, e: *mut expdesc) {
    let pc = getjumpcontrol(fs, (*e).u.info);
    debug_assert!(testTMode(GET_OPCODE(*pc) as size_t) != 0 && GET_OPCODE(*pc) != OP_TESTSET &&
                                             GET_OPCODE(*pc) != OP_TEST);
    SETARG_A(pc, (GETARG_A(*pc) == 0) as c_int);
}


/*
** Emit instruction to jump if 'e' is 'cond' (that is, if 'cond'
** is true, code will jump if 'e' is true.) Return jump position.
** Optimize when 'e' is 'not' something, inverting the condition
** and removing the 'not'.
*/
// FIXME static
#[no_mangle]
pub unsafe extern "C" fn jumponcond(
    mut fs: *mut FuncState,
    e: *mut expdesc,
    cond_0: libc::c_int,
) -> libc::c_int {
    if (*e).k as libc::c_uint == VRELOCABLE as libc::c_int as libc::c_uint {
        let ie = getinstruction(fs, e);
        if GET_OPCODE(*ie) == OP_NOT {
            (*fs).pc -= 1; /* remove previous OP_NOT */
            return condjump(
                fs,
                OP_TEST,
                GETARG_B(*ie),
                0 as libc::c_int,
                (cond_0 == 0) as libc::c_int,
            );
        }
        /* else go through */
    }
    discharge2anyreg(fs, e);
    freeexp(fs, e);
    return condjump(fs, OP_TESTSET, NO_REG as c_int, (*e).u.info, cond_0);
}

/*
** Emit code to go through if 'e' is true, jump otherwise.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_goiftrue(fs: *mut FuncState, mut e: *mut expdesc) {
    let pc: libc::c_int; /* pc of new jump */
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        11 => { // VJMP condition?
            negatecondition(fs, e);
            pc = (*e).u.info;
        }
        4 | 5 | 6 | 2 => { // VK VKFLT VKINT VTRUE
            pc = NO_JUMP; /* always true; do nothing */
        }
        _ => {
            pc = jumponcond(fs, e, 0); /* jump when false */
        }
    }
    luaK_concat(fs, &mut (*e).f, pc); /* insert new jump in false list */
    luaK_patchtohere(fs, (*e).t); /* true list jumps to here (to go through) */
    (*e).t = NO_JUMP;
}
/*
** Emit code to go through if 'e' is false, jump otherwise.
*/
/*void luaK_goiffalse (FuncState *fs, expdesc *e) {
    int pc;  /* pc of new jump */
    luaK_dischargevars(fs, e);
    switch (e->k) {
      case VJMP: {
        pc = e->u.info;  /* already jump if true */
        break;
      }
      case VNIL: case VFALSE: {
        pc = NO_JUMP;  /* always false; do nothing */
        break;
      }
      default: {
        pc = jumponcond(fs, e, 1);  /* jump if true */
        break;
      }
    }
    luaK_concat(fs, &e->t, pc);  /* insert new jump in 't' list */
    luaK_patchtohere(fs, e->f);  /* false list jumps to here (to go through) */
    e->f = NO_JUMP;
  }*/