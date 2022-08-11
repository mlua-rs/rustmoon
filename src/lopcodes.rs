/*
** Opcodes for Lua virtual machine
 */

use std::ptr;

use libc::{c_char, c_int};

use crate::llimits::lu_byte;

/*
  We assume that instructions are unsigned numbers.
  All instructions have an opcode in the first 6 bits.
  Instructions can have the following fields:
    'A' : 8 bits
    'B' : 9 bits
    'C' : 9 bits
    'Ax' : 26 bits ('A', 'B', and 'C' together)
    'Bx' : 18 bits ('B' and 'C' together)
    'sBx' : signed Bx
  A signed argument is represented in excess K; that is, the number
  value is the unsigned value minus K. K is exactly the maximum value
  for that argument (so that -max is represented by 0, and +max is
  represented by 2*max), which is half the maximum for the corresponding
  unsigned argument.
*/

// basic instruction format
pub type OpMode = lu_byte;
pub const iABC: OpMode = 0;
pub const iABx: OpMode = 1;
pub const iAsBx: OpMode = 2;
pub const iAx: OpMode = 3;

/*
  size and position of opcode arguments.
*/
pub const SIZE_C: c_int = 9;
pub const SIZE_B: c_int = 9;
pub const SIZE_Bx: c_int = SIZE_C + SIZE_B;
pub const SIZE_A: c_int = 8;
pub const SIZE_Ax: c_int = SIZE_C + SIZE_B + SIZE_A;

pub const SIZE_OP: c_int = 6;

pub const POS_OP: c_int = 0;
pub const POS_A: c_int = POS_OP + SIZE_OP;
pub const POS_C: c_int = POS_A + SIZE_A;
pub const POS_B: c_int = POS_C + SIZE_C;
pub const POS_Bx: c_int = POS_C;
pub const POS_Ax: c_int = POS_A;

/*
** limits for opcode arguments.
** we use (signed) int to manipulate most arguments,
** so they must fit in LUAI_BITSINT-1 bits (-1 for sign)
*/
pub const MAXARG_Bx: c_int = (1 << SIZE_Bx) - 1;
pub const MAXARG_sBx: c_int = MAXARG_Bx >> 1;
pub const MAXARG_Ax: c_int = (1 << SIZE_Ax) - 1;
pub const MAXARG_A: c_int = (1 << SIZE_A) - 1;
pub const MAXARG_C: c_int = (1 << SIZE_C) - 1;

/*
** the following macros help to manipulate instructions
*/

// TODO

/*
** Macros to operate RK indices
*/

// TODO

/*
** invalid register that fits in 8 bits
*/

pub const NO_REG: c_int = MAXARG_A;

/*
** R(x) - register
** Kst(x) - constant (in constant table)
** RK(x) == if ISK(x) then Kst(INDEXK(x)) else R(x)
*/

pub type OpCode = lu_byte;

pub const OP_MOVE: OpCode = 0; /* A B	R(A) := R(B) */
pub const OP_LOADK: OpCode = 1; /* A Bx	R(A) := Kst(Bx) */
pub const OP_LOADKX: OpCode = 2; /*	A 	R(A) := Kst(extra arg) */
pub const OP_LOADBOOL: OpCode = 3; /* A B C	R(A) := (Bool)B; if (C) pc++ */
pub const OP_LOADNIL: OpCode = 4; /* A B	R(A), R(A+1), ..., R(A+B) := nil */
pub const OP_GETUPVAL: OpCode = 5; /* A B	R(A) := UpValue[B] */

pub const OP_GETTABUP: OpCode = 6; /* A B C	R(A) := UpValue[B][RK(C)] */
pub const OP_GETTABLE: OpCode = 7; /* A B C	R(A) := R(B)[RK(C)] */

pub const OP_SETTABUP: OpCode = 8; /* A B C	UpValue[A][RK(B)] := RK(C) */
pub const OP_SETUPVAL: OpCode = 9; /* A B	UpValue[B] := R(A) */
pub const OP_SETTABLE: OpCode = 10; /* A B C	R(A)[RK(B)] := RK(C) */

pub const OP_NEWTABLE: OpCode = 11; /* A B C	R(A) := {} (size = B,C) */

pub const OP_SELF: OpCode = 12; /* A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)] */

pub const OP_ADD: OpCode = 13; /* A B C	R(A) := RK(B) + RK(C) */
pub const OP_SUB: OpCode = 14; /* A B C	R(A) := RK(B) - RK(C) */
pub const OP_MUL: OpCode = 15; /* A B C	R(A) := RK(B) * RK(C) */
pub const OP_MOD: OpCode = 16; /* A B C	R(A) := RK(B) % RK(C) */
pub const OP_POW: OpCode = 17; /* A B C	R(A) := RK(B) ^ RK(C) */
pub const OP_DIV: OpCode = 18; /* A B C	R(A) := RK(B) / RK(C) */
pub const OP_IDIV: OpCode = 19; /* A B C	R(A) := RK(B) // RK(C) */
pub const OP_BAND: OpCode = 20; /* A B C	R(A) := RK(B) & RK(C) */
pub const OP_BOR: OpCode = 21; /* A B C	R(A) := RK(B) | RK(C) */
pub const OP_BXOR: OpCode = 22; /* A B C	R(A) := RK(B) ~ RK(C) */
pub const OP_SHL: OpCode = 23; /* A B C	R(A) := RK(B) << RK(C) */
pub const OP_SHR: OpCode = 24; /* A B C	R(A) := RK(B) >> RK(C) */
pub const OP_UNM: OpCode = 25; /* A B	R(A) := -R(B) */
pub const OP_BNOT: OpCode = 26; /* A B	R(A) := ~R(B) */
pub const OP_NOT: OpCode = 27; /* A B	R(A) := not R(B) */
pub const OP_LEN: OpCode = 28; /* A B	R(A) := length of R(B) */

pub const OP_CONCAT: OpCode = 29; /* A B C	R(A) := R(B).. ... ..R(C) */

pub const OP_JMP: OpCode = 30; /* A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1) */
pub const OP_EQ: OpCode = 31; /* A B C	if ((RK(B) == RK(C)) ~= A) then pc++ */
pub const OP_LT: OpCode = 32; /* A B C	if ((RK(B) <  RK(C)) ~= A) then pc++ */
pub const OP_LE: OpCode = 33; /* A B C	if ((RK(B) <= RK(C)) ~= A) then pc++ */

pub const OP_TEST: OpCode = 34; /* A C	if not (R(A) <=> C) then pc++ */
pub const OP_TESTSET: OpCode = 35; /* A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/

pub const OP_CALL: OpCode = 36; /* A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
pub const OP_TAILCALL: OpCode = 37; /* A B C	return R(A)(R(A+1), ... ,R(A+B-1)) */
pub const OP_RETURN: OpCode = 38; /* A B	return R(A), ... ,R(A+B-2)	(see note) */

pub const OP_FORLOOP: OpCode = 39; /* A sBx	R(A)+=R(A+2); if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) } */
pub const OP_FORPREP: OpCode = 40; /* A sBx	R(A)-=R(A+2); pc+=sBx */

pub const OP_TFORCALL: OpCode = 41; /* A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2)); */
pub const OP_TFORLOOP: OpCode = 42; /* A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/

pub const OP_SETLIST: OpCode = 43; /* A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B */

pub const OP_CLOSURE: OpCode = 44; /* A Bx	R(A) := closure(KPROTO[Bx]) */

pub const OP_VARARG: OpCode = 45; /* A B	R(A), R(A+1), ..., R(A+B-2) = vararg */

pub const OP_EXTRAARG: OpCode = 46; /* Ax	extra (larger) argument for previous opcode	*/

pub const NUM_OPCODES: usize = 47;

/*
  Notes:
  (*) In OP_CALL, if (B == 0) then B = top. If (C == 0), then 'top' is
  set to last_result+1, so next open instruction (OP_CALL, OP_RETURN,
  OP_SETLIST) may use 'top'.

  (*) In OP_VARARG, if (B == 0) then use actual number of varargs and
  set top (like in OP_CALL with C == 0).

  (*) In OP_RETURN, if (B == 0) then return up to 'top'.

  (*) In OP_SETLIST, if (B == 0) then B = 'top'; if (C == 0) then next
  'instruction' is EXTRAARG(real C).

  (*) In OP_LOADKX, the next 'instruction' is always EXTRAARG.

  (*) For comparisons, A specifies what condition the test should accept
  (true or false).

  (*) All 'skips' (pc++) assume that next instruction is a jump.

*/

/*
** masks for instruction properties. The format is:
** bits 0-1: op mode
** bits 2-3: C arg mode
** bits 4-5: B arg mode
** bit 6: instruction set register A
** bit 7: operator is a test (next instruction must be a jump)
*/

pub type OpArgMask = lu_byte;
pub const OpArgN: OpArgMask = 0; /* argument is not used */
pub const OpArgU: OpArgMask = 1; /* argument is used */
pub const OpArgR: OpArgMask = 2; /* argument is a register or a jump offset */
pub const OpArgK: OpArgMask = 3; /* argument is a constant or register/constant */

macro_rules! opmode {
    ($t:expr, $a:expr, $b:expr, $c:expr, $m:expr) => {
        ($t << 7) | ($a << 6) | ($b << 4) | ($c << 2) | $m
    };
}

#[no_mangle]
pub static mut luaP_opmodes: [lu_byte; NUM_OPCODES] = [
    /*      T  A    B       C     mode		opcode	*/
    opmode!(0, 1, OpArgR, OpArgN, iABC),  /* OP_MOVE */
    opmode!(0, 1, OpArgK, OpArgN, iABx),  /* OP_LOADK */
    opmode!(0, 1, OpArgN, OpArgN, iABx),  /* OP_LOADKX */
    opmode!(0, 1, OpArgU, OpArgU, iABC),  /* OP_LOADBOOL */
    opmode!(0, 1, OpArgU, OpArgN, iABC),  /* OP_LOADNIL */
    opmode!(0, 1, OpArgU, OpArgN, iABC),  /* OP_GETUPVAL */
    opmode!(0, 1, OpArgU, OpArgK, iABC),  /* OP_GETTABUP */
    opmode!(0, 1, OpArgR, OpArgK, iABC),  /* OP_GETTABLE */
    opmode!(0, 0, OpArgK, OpArgK, iABC),  /* OP_SETTABUP */
    opmode!(0, 0, OpArgU, OpArgN, iABC),  /* OP_SETUPVAL */
    opmode!(0, 0, OpArgK, OpArgK, iABC),  /* OP_SETTABLE */
    opmode!(0, 1, OpArgU, OpArgU, iABC),  /* OP_NEWTABLE */
    opmode!(0, 1, OpArgR, OpArgK, iABC),  /* OP_SELF */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_ADD */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_SUB */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_MUL */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_MOD */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_POW */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_DIV */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_IDIV */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_BAND */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_BOR */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_BXOR */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_SHL */
    opmode!(0, 1, OpArgK, OpArgK, iABC),  /* OP_SHR */
    opmode!(0, 1, OpArgR, OpArgN, iABC),  /* OP_UNM */
    opmode!(0, 1, OpArgR, OpArgN, iABC),  /* OP_BNOT */
    opmode!(0, 1, OpArgR, OpArgN, iABC),  /* OP_NOT */
    opmode!(0, 1, OpArgR, OpArgN, iABC),  /* OP_LEN */
    opmode!(0, 1, OpArgR, OpArgR, iABC),  /* OP_CONCAT */
    opmode!(0, 0, OpArgR, OpArgN, iAsBx), /* OP_JMP */
    opmode!(1, 0, OpArgK, OpArgK, iABC),  /* OP_EQ */
    opmode!(1, 0, OpArgK, OpArgK, iABC),  /* OP_LT */
    opmode!(1, 0, OpArgK, OpArgK, iABC),  /* OP_LE */
    opmode!(1, 0, OpArgN, OpArgU, iABC),  /* OP_TEST */
    opmode!(1, 1, OpArgR, OpArgU, iABC),  /* OP_TESTSET */
    opmode!(0, 1, OpArgU, OpArgU, iABC),  /* OP_CALL */
    opmode!(0, 1, OpArgU, OpArgU, iABC),  /* OP_TAILCALL */
    opmode!(0, 0, OpArgU, OpArgN, iABC),  /* OP_RETURN */
    opmode!(0, 1, OpArgR, OpArgN, iAsBx), /* OP_FORLOOP */
    opmode!(0, 1, OpArgR, OpArgN, iAsBx), /* OP_FORPREP */
    opmode!(0, 0, OpArgN, OpArgU, iABC),  /* OP_TFORCALL */
    opmode!(0, 1, OpArgR, OpArgN, iAsBx), /* OP_TFORLOOP */
    opmode!(0, 0, OpArgU, OpArgU, iABC),  /* OP_SETLIST */
    opmode!(0, 1, OpArgU, OpArgN, iABx),  /* OP_CLOSURE */
    opmode!(0, 1, OpArgU, OpArgN, iABC),  /* OP_VARARG */
    opmode!(0, 0, OpArgU, OpArgU, iAx),   /* OP_EXTRAARG */
];

/* opcode names */
pub const luaP_opnames: [*const c_char; NUM_OPCODES + 1] = [
    b"MOVE\0" as *const u8 as *const c_char,
    b"LOADK\0" as *const u8 as *const c_char,
    b"LOADKX\0" as *const u8 as *const c_char,
    b"LOADBOOL\0" as *const u8 as *const c_char,
    b"LOADNIL\0" as *const u8 as *const c_char,
    b"GETUPVAL\0" as *const u8 as *const c_char,
    b"GETTABUP\0" as *const u8 as *const c_char,
    b"GETTABLE\0" as *const u8 as *const c_char,
    b"SETTABUP\0" as *const u8 as *const c_char,
    b"SETUPVAL\0" as *const u8 as *const c_char,
    b"SETTABLE\0" as *const u8 as *const c_char,
    b"NEWTABLE\0" as *const u8 as *const c_char,
    b"SELF\0" as *const u8 as *const c_char,
    b"ADD\0" as *const u8 as *const c_char,
    b"SUB\0" as *const u8 as *const c_char,
    b"MUL\0" as *const u8 as *const c_char,
    b"MOD\0" as *const u8 as *const c_char,
    b"POW\0" as *const u8 as *const c_char,
    b"DIV\0" as *const u8 as *const c_char,
    b"IDIV\0" as *const u8 as *const c_char,
    b"BAND\0" as *const u8 as *const c_char,
    b"BOR\0" as *const u8 as *const c_char,
    b"BXOR\0" as *const u8 as *const c_char,
    b"SHL\0" as *const u8 as *const c_char,
    b"SHR\0" as *const u8 as *const c_char,
    b"UNM\0" as *const u8 as *const c_char,
    b"BNOT\0" as *const u8 as *const c_char,
    b"NOT\0" as *const u8 as *const c_char,
    b"LEN\0" as *const u8 as *const c_char,
    b"CONCAT\0" as *const u8 as *const c_char,
    b"JMP\0" as *const u8 as *const c_char,
    b"EQ\0" as *const u8 as *const c_char,
    b"LT\0" as *const u8 as *const c_char,
    b"LE\0" as *const u8 as *const c_char,
    b"TEST\0" as *const u8 as *const c_char,
    b"TESTSET\0" as *const u8 as *const c_char,
    b"CALL\0" as *const u8 as *const c_char,
    b"TAILCALL\0" as *const u8 as *const c_char,
    b"RETURN\0" as *const u8 as *const c_char,
    b"FORLOOP\0" as *const u8 as *const c_char,
    b"FORPREP\0" as *const u8 as *const c_char,
    b"TFORCALL\0" as *const u8 as *const c_char,
    b"TFORLOOP\0" as *const u8 as *const c_char,
    b"SETLIST\0" as *const u8 as *const c_char,
    b"CLOSURE\0" as *const u8 as *const c_char,
    b"VARARG\0" as *const u8 as *const c_char,
    b"EXTRAARG\0" as *const u8 as *const c_char,
    ptr::null(),
];

/* number of list items to accumulate before a SETLIST instruction */
pub const LFIELDS_PER_FLUSH: usize = 50;
