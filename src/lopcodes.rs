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

pub type Instruction = libc::c_uint;

pub type OpMode = libc::c_uint;

// basic instruction format
pub const iABC: OpMode = 0;
pub const iABx: OpMode = 1;
pub const iAsBx: OpMode = 2;
pub const iAx: OpMode = 3;

/*
  size and position of opcode arguments.
*/
pub const SIZE_C: libc::c_int = 9 as libc::c_int;
pub const SIZE_B: libc::c_int = 9 as libc::c_int;
pub const SIZE_Bx: libc::c_int = SIZE_C + SIZE_B;
pub const SIZE_A: libc::c_int = 8 as libc::c_int;
pub const SIZE_Ax: libc::c_int = SIZE_C + SIZE_B + SIZE_A;

pub const SIZE_OP: libc::c_int = 6 as libc::c_int;

pub const POS_OP: libc::c_int = 0 as libc::c_int;
pub const POS_A: libc::c_int = POS_OP + SIZE_OP;
pub const POS_C: libc::c_int = POS_A + SIZE_A;
pub const POS_B: libc::c_int = POS_C + SIZE_C;
pub const POS_Bx: libc::c_int = POS_C;
pub const POS_Ax: libc::c_int = POS_A;

/*
  limits for opcode arguments.
  we use (signed) int to manipulate most arguments,
  so they must fit in LUAI_BITSINT-1 bits (-1 for sign)
*/
pub const MAXARG_Bx: libc::c_int = ((1 as libc::c_int) << SIZE_Bx) - 1 as libc::c_int;
pub const MAXARG_sBx: libc::c_int = MAXARG_Bx >> 1 as libc::c_int;
pub const MAXARG_Ax: libc::c_int = ((1 as libc::c_int) << SIZE_Ax) - 1 as libc::c_int;
pub const MAXARG_A: libc::c_int = ((1 as libc::c_int) << SIZE_A) - 1 as libc::c_int;
pub const MAXARG_C: libc::c_int = ((1 as libc::c_int) << SIZE_C) - 1 as libc::c_int;

/*
  the following macros help to manipulate instructions
*/

/// creates a mask with 'n' 1 bits at position 'p'
macro_rules! MASK1 {
    ($n:expr, $p:expr) => {
        (!((!(0 as Instruction)) << $n) << $p)
    };
}

/// creates a mask with 'n' 0 bits at position 'p'
macro_rules! MASK0 {
    ($n:expr, $p:expr) => {
        (!MASK1!($n, $p))
    };
}

macro_rules! GET_OPCODE {
    ($i:expr) => {
        (($i) >> POS_OP) & MASK1!(SIZE_OP, 0)
    };
}

macro_rules! SET_OPCODE {
    ($i: expr, $o: expr) => {
        $i = ($i & MASK0(SIZE_OP, POS_OP))
            | ((($o as Instruction) << POS_OP) & MASK1(SIZE_OP, POS_OP))
    };
}



pub static luaP_opnames: [*const libc::c_char; 48] = [
    b"MOVE\0" as *const u8 as *const libc::c_char,
    b"LOADK\0" as *const u8 as *const libc::c_char,
    b"LOADKX\0" as *const u8 as *const libc::c_char,
    b"LOADBOOL\0" as *const u8 as *const libc::c_char,
    b"LOADNIL\0" as *const u8 as *const libc::c_char,
    b"GETUPVAL\0" as *const u8 as *const libc::c_char,
    b"GETTABUP\0" as *const u8 as *const libc::c_char,
    b"GETTABLE\0" as *const u8 as *const libc::c_char,
    b"SETTABUP\0" as *const u8 as *const libc::c_char,
    b"SETUPVAL\0" as *const u8 as *const libc::c_char,
    b"SETTABLE\0" as *const u8 as *const libc::c_char,
    b"NEWTABLE\0" as *const u8 as *const libc::c_char,
    b"SELF\0" as *const u8 as *const libc::c_char,
    b"ADD\0" as *const u8 as *const libc::c_char,
    b"SUB\0" as *const u8 as *const libc::c_char,
    b"MUL\0" as *const u8 as *const libc::c_char,
    b"MOD\0" as *const u8 as *const libc::c_char,
    b"POW\0" as *const u8 as *const libc::c_char,
    b"DIV\0" as *const u8 as *const libc::c_char,
    b"IDIV\0" as *const u8 as *const libc::c_char,
    b"BAND\0" as *const u8 as *const libc::c_char,
    b"BOR\0" as *const u8 as *const libc::c_char,
    b"BXOR\0" as *const u8 as *const libc::c_char,
    b"SHL\0" as *const u8 as *const libc::c_char,
    b"SHR\0" as *const u8 as *const libc::c_char,
    b"UNM\0" as *const u8 as *const libc::c_char,
    b"BNOT\0" as *const u8 as *const libc::c_char,
    b"NOT\0" as *const u8 as *const libc::c_char,
    b"LEN\0" as *const u8 as *const libc::c_char,
    b"CONCAT\0" as *const u8 as *const libc::c_char,
    b"JMP\0" as *const u8 as *const libc::c_char,
    b"EQ\0" as *const u8 as *const libc::c_char,
    b"LT\0" as *const u8 as *const libc::c_char,
    b"LE\0" as *const u8 as *const libc::c_char,
    b"TEST\0" as *const u8 as *const libc::c_char,
    b"TESTSET\0" as *const u8 as *const libc::c_char,
    b"CALL\0" as *const u8 as *const libc::c_char,
    b"TAILCALL\0" as *const u8 as *const libc::c_char,
    b"RETURN\0" as *const u8 as *const libc::c_char,
    b"FORLOOP\0" as *const u8 as *const libc::c_char,
    b"FORPREP\0" as *const u8 as *const libc::c_char,
    b"TFORCALL\0" as *const u8 as *const libc::c_char,
    b"TFORLOOP\0" as *const u8 as *const libc::c_char,
    b"SETLIST\0" as *const u8 as *const libc::c_char,
    b"CLOSURE\0" as *const u8 as *const libc::c_char,
    b"VARARG\0" as *const u8 as *const libc::c_char,
    b"EXTRAARG\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

macro_rules! opmode {
    ($t:expr, $a:expr, $b:expr, $c:expr, $m:expr) => {
        ($t << 7) | ($a << 6) | ($b << 4) | ($c << 2) | $m
    };
}

pub static mut luaP_opmodes: [lu_byte; 47] = [
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgK, OpArgN, iABx),
    opmode!(0, 1, OpArgN, OpArgN, iABx),
    opmode!(0, 1, OpArgU, OpArgU, iABC),
    opmode!(0, 1, OpArgU, OpArgN, iABC),
    opmode!(0, 1, OpArgU, OpArgN, iABC),
    opmode!(0, 1, OpArgU, OpArgK, iABC),
    opmode!(0, 1, OpArgR, OpArgK, iABC),
    opmode!(0, 0, OpArgK, OpArgK, iABC),
    opmode!(0, 0, OpArgU, OpArgN, iABC),
    opmode!(0, 0, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgU, OpArgU, iABC),
    opmode!(0, 1, OpArgR, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgR, iABC),
    opmode!(0, 0, OpArgR, OpArgN, iAsBx),
    opmode!(1, 0, OpArgK, OpArgK, iABC),
    opmode!(1, 0, OpArgK, OpArgK, iABC),
    opmode!(1, 0, OpArgK, OpArgK, iABC),
    opmode!(1, 0, OpArgN, OpArgU, iABC),
    opmode!(1, 1, OpArgR, OpArgU, iABC),
    opmode!(0, 1, OpArgU, OpArgU, iABC),
    opmode!(0, 1, OpArgU, OpArgU, iABC),
    opmode!(0, 0, OpArgU, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iAsBx),
    opmode!(0, 1, OpArgR, OpArgN, iAsBx),
    opmode!(0, 0, OpArgN, OpArgU, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iAsBx),
    opmode!(0, 0, OpArgU, OpArgU, iABC),
    opmode!(0, 1, OpArgU, OpArgN, iABx),
    opmode!(0, 1, OpArgU, OpArgN, iABC),
    opmode!(0, 0, OpArgU, OpArgU, iAx),
];
