/*
** Lua Parser
*/

use std::ptr;

use libc::{c_char, c_int, c_short, c_uint};

use crate::llex::LexState;
use crate::lobject::{LClosure, Proto, TString};
use crate::lstate::lua_State;
use crate::lzio::{Mbuffer, ZIO};
use crate::types::{lua_Integer, lua_Number};

/*
  Expression and variable descriptor.
  Code generation for variables and expressions can be delayed to allow
  optimizations; An 'expdesc' structure describes a potentially-delayed
  variable/expression. It has a description of its "main" value plus a
  list of conditional jumps that can also produce its value (generated
  by short-circuit operators 'and'/'or').
*/

// kinds of variables/expressions
pub type expkind = c_uint;

// when 'expdesc' describes the last expression a list, this kind means an empty list (so, no expression)
pub const VVOID: expkind = 0;

// constant nil
pub const VNIL: expkind = 1;

// constant true
pub const VTRUE: expkind = 2;

// constant false
pub const VFALSE: expkind = 3;

// constant in 'k'; info = index of constant in 'k'
pub const VK: expkind = 4;

// floating constant; nval = numerical float value
pub const VKFLT: expkind = 5;

// integer constant; nval = numerical integer value
pub const VKINT: expkind = 6;

// expression has its value in a fixed register; info = result register
pub const VNONRELOC: expkind = 7;

// local variable; info = local register
pub const VLOCAL: expkind = 8;

// upvalue variable; info = index of upvalue in 'upvalues'
pub const VUPVAL: expkind = 9;

// indexed variable;
// ind.vt = whether 't' is register or upvalue;
// ind.t = table register or upvalue;
// ind.idx = key's R/K index
pub const VINDEXED: expkind = 10;

// expression is a test/comparison; info = pc of corresponding jump instruction
pub const VJMP: expkind = 11;

// expression can put result in any register; info = instruction pc
pub const VRELOCABLE: expkind = 12;

// expression is a function call; info = instruction pc
pub const VCALL: expkind = 13;

// vararg expression; info = instruction pc
pub const VVARARG: expkind = 14;

const fn vkisvar(k: expkind) -> bool {
    VLOCAL <= k && k <= VINDEXED
}

const fn vkisinreg(k: expkind) -> bool {
    k == VNONRELOC || k == VLOCAL
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct expdesc {
    pub k: expkind,
    pub u: C2RustUnnamed_8,
    pub t: c_int, // patch list of 'exit when true'
    pub f: c_int, // patch list of 'exit when false'
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ival: lua_Integer,    // for VKINT
    pub nval: lua_Number,     // for VKFLT
    pub info: c_int,          // for generic use
    pub ind: C2RustUnnamed_9, // for indexed variables (VINDEXED)
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub idx: c_short, // index (R/K)
    pub t: u8,        // table (register or upvalue)
    pub vt: u8,       // whether 't' is register (VLOCAL) or upvalue (VUPVAL)
}

// description of active local variable
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vardesc {
    pub idx: c_short, // variable index in stack
}

// description of pending goto statements and label statements
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc {
    pub name: *mut TString, // label identifier
    pub pc: c_int,          // position in code
    pub line: c_int,        // line where it appeared
    pub nactvar: u8,        // local level where it appears in current block
}

// list of labels or gotos
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist {
    pub arr: *mut Labeldesc, // array
    pub n: c_int,            // number of entries in use
    pub size: c_int,         // array size
}

// dynamic structures used by the parser
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dyndata {
    pub actvar: C2RustUnnamed_7, // list of active local variables
    pub gt: Labellist,           // list of pending gotos
    pub label: Labellist,        // list of active labels
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub arr: *mut Vardesc,
    pub n: c_int,
    pub size: c_int,
}

impl Dyndata {
    pub(crate) const fn new() -> Self {
        Dyndata {
            actvar: C2RustUnnamed_7 {
                arr: ptr::null_mut(),
                n: 0,
                size: 0,
            },
            gt: Labellist {
                arr: ptr::null_mut(),
                n: 0,
                size: 0,
            },
            label: Labellist {
                arr: ptr::null_mut(),
                n: 0,
                size: 0,
            },
        }
    }
}

// nodes for block list (list of active blocks)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockCnt {
    pub previous: *mut BlockCnt, // chain
    pub firstlabel: c_int,       // index of first label in this block
    pub firstgoto: c_int,        // index of first pending goto in this block
    pub nactvar: u8,             // # active locals outside the block
    pub upval: u8,               // true if some variable in the block is an upvalue
    pub isloop: u8,              // true if 'block' is a loop
}

// state needed to generate code for a given function
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto,        // current function header
    pub prev: *mut FuncState, // enclosing function
    pub ls: *mut LexState,    // lexical state
    pub bl: *mut BlockCnt,    // chain of current blocks
    pub pc: c_int,            // next position to code (equivalent to 'ncode')
    pub lasttarget: c_int,    // 'label' of last 'jump label'
    pub jpc: c_int,           // list of pending jumps to 'pc'
    pub nk: c_int,            // number of elements in 'k'
    pub np: c_int,            // number of elements in 'p'
    pub firstlocal: c_int,    // index of first local var (in Dyndata array)
    pub nlocvars: c_short,    // number of elements in 'f->locvars'
    pub nactvar: u8,          // number of active local variables
    pub nups: u8,             // number of upvalues
    pub freereg: u8,          // first free register
}

// maximum number of local variables per function
// (must be smaller than 250, due to the bytecode format)
pub const MAXVARS: usize = 200;

const fn hasmultret(k: expkind) -> bool {
    k == VCALL || k == VVARARG
}

// because all strings are unified by the scanner, the parser
// can use pointer equality for string equality
// macro_rules! eqstr {
//     ($a: expr, $b: expr) => {
//         $a == $b
//     };
// }

/* semantic error */
/* static l_noret semerror (LexState *ls, const char *msg) {
  ls->t.token = 0;  /* remove "near <token>" from final message */
  luaX_syntaxerror(ls, msg);
}*/

/*
  static l_noret error_expected (LexState *ls, int token) {
    luaX_syntaxerror(ls,
        luaO_pushfstring(ls->L, "%s expected", luaX_token2str(ls, token)));
  }
*/
/*
pub unsafe extern "C" fn error_expected(mut ls: *mut LexState, mut token: libc::c_int) -> ! {
    luaX_syntaxerror(
        ls,
        luaO_pushfstring(
            (*ls).L,
            b"%s expected\0" as *const u8 as *const libc::c_char,
            luaX_token2str(ls, token),
        ),
    );
}*/

extern "C" {
    pub fn luaY_parser(
        L: *mut lua_State,
        z: *mut ZIO,
        buff: *mut Mbuffer,
        dyd: *mut Dyndata,
        name: *const c_char,
        firstchar: c_int,
    ) -> *mut LClosure;
}
