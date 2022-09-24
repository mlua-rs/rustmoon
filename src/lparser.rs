/*
** Lua Parser
*/

use std::ptr;

use libc::{c_char, c_int, c_short, c_uchar, c_uint, size_t, strlen};

use crate::lcode::{
    getinstruction, luaK_checkstack, luaK_codeABC, luaK_codeABx, luaK_codeAsBx, luaK_codek,
    luaK_concat, luaK_dischargevars, luaK_exp2RK, luaK_exp2anyreg, luaK_exp2anyregup,
    luaK_exp2nextreg, luaK_exp2val, luaK_fixline, luaK_getlabel, luaK_goiffalse, luaK_goiftrue,
    luaK_indexed, luaK_infix, luaK_intK, luaK_jump, luaK_jumpto, luaK_nil, luaK_patchclose,
    luaK_patchlist, luaK_patchtohere, luaK_posfix, luaK_prefix, luaK_reserveregs, luaK_ret,
    luaK_self, luaK_setlist, luaK_setoneret, luaK_setreturns, luaK_storevar, luaK_stringK, BinOpr,
    UnOpr, NO_JUMP, OPR_ADD, OPR_AND, OPR_BAND, OPR_BNOT, OPR_BOR, OPR_BXOR, OPR_CONCAT, OPR_DIV,
    OPR_EQ, OPR_GE, OPR_GT, OPR_IDIV, OPR_LE, OPR_LEN, OPR_LT, OPR_MINUS, OPR_MOD, OPR_MUL, OPR_NE,
    OPR_NOBINOPR, OPR_NOT, OPR_NOUNOPR, OPR_OR, OPR_POW, OPR_SHL, OPR_SHR, OPR_SUB,
};
use crate::ldo::luaD_inctop;
use crate::lfunc::{luaF_newLclosure, luaF_newproto, MAXUPVAL};
use crate::lgc::{luaC_checkGC, luaC_objbarrier};
use crate::llex::{
    luaX_lookahead, luaX_newstring, luaX_next, luaX_setinput, luaX_syntaxerror, luaX_token2str,
    LexState, SemInfo, Token, TK_BREAK, TK_DBCOLON, TK_DO, TK_ELSE, TK_ELSEIF, TK_END, TK_EOS,
    TK_FOR, TK_FUNCTION, TK_GOTO, TK_IF, TK_IN, TK_NAME, TK_REPEAT, TK_RETURN, TK_THEN, TK_UNTIL,
    TK_WHILE,
};
use crate::llimits::{lu_byte, LUAI_MAXCCALLS, MAX_INT, SHRT_MAX};
use crate::lmem::{luaM_growvector, luaM_reallocvector};
use crate::lobject::{
    getstr, luaO_int2fb, luaO_pushfstring, setclLvalue, sethvalue, GCObject, LClosure, LocVar,
    Proto, TString, Table,
};
use crate::lopcodes::{
    MAXARG_Bx, LFIELDS_PER_FLUSH, OP_CALL, OP_CLOSURE, OP_FORLOOP, OP_FORPREP, OP_GETUPVAL,
    OP_MOVE, OP_NEWTABLE, OP_SETTABLE, OP_TAILCALL, OP_TFORCALL, OP_TFORLOOP, OP_VARARG, SETARG_B,
    SETARG_C, SET_OPCODE,
};
use crate::lstate::lua_State;
use crate::lstring::{isreserved, luaS_new};
use crate::ltable::luaH_new;
use crate::lzio::{Mbuffer, ZIO};
use crate::types::{lua_Integer, lua_Number, LUA_MULTRET};

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

pub const fn vkisinreg(k: expkind) -> bool {
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
macro_rules! eqstr {
    ($a: expr, $b: expr) => {
        $a == $b
    };
}

/* semantic error */
unsafe extern "C" fn semerror(mut ls: *mut LexState, msg: *const c_char) {
    (*ls).t.token = 0;
    luaX_syntaxerror(ls, msg);
}

unsafe extern "C" fn error_expected(ls: *mut LexState, token: c_int) {
    luaX_syntaxerror(
        ls,
        luaO_pushfstring((*ls).L, cstr!("%s expected"), luaX_token2str(ls, token)),
    );
}

unsafe extern "C" fn errorlimit(fs: *mut FuncState, limit: c_int, what: *const c_char) {
    let L = (*(*fs).ls).L;
    let msg: *const c_char;
    let line = (*(*fs).f).linedefined;
    let where_0 = if line == 0 {
        cstr!("main function")
    } else {
        luaO_pushfstring(L, cstr!("function at line %d"), line)
    };
    msg = luaO_pushfstring(
        L,
        cstr!("too many %s (limit is %d) in %s"),
        what,
        limit,
        where_0,
    );
    luaX_syntaxerror((*fs).ls, msg);
}

unsafe extern "C" fn checklimit(fs: *mut FuncState, v: c_int, l: c_int, what: *const c_char) {
    if v > l {
        errorlimit(fs, l, what);
    }
}

unsafe extern "C" fn testnext(ls: *mut LexState, c: c_int) -> c_int {
    if (*ls).t.token == c {
        luaX_next(ls);
        return 1;
    } else {
        return 0;
    }
}

unsafe extern "C" fn check(ls: *mut LexState, c: c_int) {
    if (*ls).t.token != c {
        error_expected(ls, c);
    }
}

unsafe extern "C" fn checknext(ls: *mut LexState, c: c_int) {
    check(ls, c);
    luaX_next(ls);
}

unsafe extern "C" fn check_match(ls: *mut LexState, what: c_int, who: c_int, where_0: c_int) {
    if testnext(ls, what) == 0 {
        if where_0 == (*ls).linenumber {
            error_expected(ls, what);
        } else {
            luaX_syntaxerror(
                ls,
                luaO_pushfstring(
                    (*ls).L,
                    cstr!("%s expected (to close %s at line %d)"),
                    luaX_token2str(ls, what),
                    luaX_token2str(ls, who),
                    where_0,
                ),
            );
        }
    }
}

unsafe extern "C" fn str_checkname(ls: *mut LexState) -> *mut TString {
    let ts: *mut TString;
    check(ls, TK_NAME as c_int);
    ts = (*ls).t.seminfo.ts;
    luaX_next(ls);
    return ts;
}

unsafe extern "C" fn init_exp(mut e: *mut expdesc, k: expkind, i: c_int) {
    (*e).t = NO_JUMP;
    (*e).f = (*e).t;
    (*e).k = k;
    (*e).u.info = i;
}
unsafe extern "C" fn codestring(ls: *mut LexState, e: *mut expdesc, s: *mut TString) {
    init_exp(e, VK, luaK_stringK((*ls).fs, s));
}
unsafe extern "C" fn checkname(ls: *mut LexState, e: *mut expdesc) {
    codestring(ls, e, str_checkname(ls));
}

unsafe extern "C" fn registerlocalvar(ls: *mut LexState, varname: *mut TString) -> c_int {
    let mut fs = (*ls).fs;
    let f = (*fs).f;
    let mut oldsize = (*f).sizelocvars;
    luaM_growvector(
        (*ls).L,
        &mut (*f).locvars,
        (*fs).nlocvars as c_int,
        &mut (*f).sizelocvars,
        SHRT_MAX as c_int,
        cstr!("local variables"),
    );
    while oldsize < (*f).sizelocvars {
        (*((*f).locvars).offset(oldsize as isize)).varname = ptr::null_mut() as *mut TString;
        oldsize = oldsize + 1;
    }
    (*((*f).locvars).offset((*fs).nlocvars as isize)).varname = varname;
    luaC_objbarrier((*ls).L, f as *mut GCObject, varname as *mut GCObject);
    let nlocvars = (*fs).nlocvars;
    (*fs).nlocvars = (*fs).nlocvars + 1;
    return nlocvars as c_int;
}

unsafe extern "C" fn new_localvar(ls: *mut LexState, name: *mut TString) {
    let fs = (*ls).fs;
    let mut dyd = (*ls).dyd;
    let reg = registerlocalvar(ls, name);
    checklimit(
        fs,
        (*dyd).actvar.n + 1 as c_int - (*fs).firstlocal,
        MAXVARS as c_int,
        b"local variables\0" as *const u8 as *const c_char,
    );
    luaM_growvector(
        (*ls).L,
        &mut (*dyd).actvar.arr,
        (*dyd).actvar.n + 1,
        &mut (*dyd).actvar.size,
        MAX_INT,
        cstr!("local variables"),
    );
    (*((*dyd).actvar.arr).offset((*dyd).actvar.n as isize)).idx = reg as c_short;
    (*dyd).actvar.n = (*dyd).actvar.n + 1;
}

unsafe extern "C" fn new_localvarliteral(ls: *mut LexState, name: *const c_char) {
    new_localvar(
        ls,
        luaX_newstring(ls, name, strlen(name) / std::mem::size_of::<c_char>()),
    );
}

unsafe extern "C" fn getlocvar(fs: *mut FuncState, i: c_int) -> *mut LocVar {
    let idx =
        (*((*(*(*fs).ls).dyd).actvar.arr).offset(((*fs).firstlocal + i) as isize)).idx as c_int;
    return &mut *((*(*fs).f).locvars).offset(idx as isize) as *mut LocVar;
}

unsafe extern "C" fn adjustlocalvars(ls: *mut LexState, mut nvars: c_int) {
    let mut fs = (*ls).fs;
    (*fs).nactvar = (*fs).nactvar + nvars as c_uchar;
    while nvars != 0 {
        (*getlocvar(fs, (*fs).nactvar as c_int - nvars)).startpc = (*fs).pc;
        nvars -= 1;
    }
}

unsafe extern "C" fn removevars(mut fs: *mut FuncState, tolevel: c_int) {
    (*(*(*fs).ls).dyd).actvar.n -= (*fs).nactvar as c_int - tolevel;
    while (*fs).nactvar as c_int > tolevel {
        (*fs).nactvar = ((*fs).nactvar).wrapping_sub(1);
        (*getlocvar(fs, (*fs).nactvar as c_int)).endpc = (*fs).pc;
    }
}

unsafe extern "C" fn searchupvalue(fs: *mut FuncState, name: *mut TString) -> c_int {
    let mut i: c_int = 0;
    let up = (*(*fs).f).upvalues;
    while i < (*fs).nups as c_int {
        if eqstr!((*up.offset(i as isize)).name, name) {
            return i;
        }
        i += 1;
    }
    return -1; /* not found */
}

unsafe extern "C" fn newupvalue(
    mut fs: *mut FuncState,
    name: *mut TString,
    v: *mut expdesc,
) -> c_int {
    let f = (*fs).f;
    let mut oldsize = (*f).sizeupvalues;
    checklimit(
        fs,
        (*fs).nups as c_int + 1 as c_int,
        MAXUPVAL as c_int,
        cstr!("upvalues"),
    );
    luaM_growvector(
        (*(*fs).ls).L,
        &mut (*f).upvalues,
        (*fs).nups as c_int,
        &mut (*f).sizeupvalues,
        MAXUPVAL as c_int,
        cstr!("upvalues"),
    );
    while oldsize < (*f).sizeupvalues {
        (*((*f).upvalues).offset(oldsize as isize)).name = ptr::null_mut() as *mut TString;
        oldsize = oldsize + 1;
    }
    (*((*f).upvalues).offset((*fs).nups as isize)).instack =
        ((*v).k as c_uint == VLOCAL as c_uint) as lu_byte;
    (*((*f).upvalues).offset((*fs).nups as isize)).idx = (*v).u.info as lu_byte;
    (*((*f).upvalues).offset((*fs).nups as isize)).name = name;
    luaC_objbarrier((*(*fs).ls).L, f as *mut GCObject, name as *mut GCObject);
    let nups = (*fs).nups;
    (*fs).nups = nups.wrapping_add(1);
    return nups as c_int;
}

/* static int searchvar (FuncState *fs, TString *n) {
  int i;
  for (i = cast_int(fs->nactvar) - 1; i >= 0; i--) {
    if (eqstr(n, getlocvar(fs, i)->varname))
      return i;
  }
  return -1;  /* not found */
}
 */

unsafe extern "C" fn searchvar(fs: *mut FuncState, n: *mut TString) -> c_int {
    let mut i: c_int;
    i = (*fs).nactvar as c_int - 1;
    while i >= 0 as c_int {
        if eqstr!(n, (*getlocvar(fs, i)).varname) {
            return i;
        }
        i -= 1;
    }
    return -1;
}

/*
  Mark block where variable at given level was defined
  (to emit close instructions later).
*/
/*static void markupval (FuncState *fs, int level) {
  BlockCnt *bl = fs->bl;
  while (bl->nactvar > level)
    bl = bl->previous;
  bl->upval = 1;
}
*/

unsafe extern "C" fn markupval(fs: *mut FuncState, level: c_int) {
    let mut bl = (*fs).bl;
    while (*bl).nactvar as c_int > level {
        bl = (*bl).previous;
    }
    (*bl).upval = 1 as lu_byte;
}

/*
  Find variable with given name 'n'. If it is an upvalue, add this
  upvalue into all intermediate functions.
*/
unsafe extern "C" fn singlevaraux(
    fs: *mut FuncState,
    n: *mut TString,
    var: *mut expdesc,
    base: c_int,
) {
    if fs.is_null() {
        /* no more levels? */
        init_exp(var, VVOID, 0); /* default is global */
    } else {
        let v = searchvar(fs, n); /* look up locals at current level */
        if v >= 0 {
            /* found? */
            init_exp(var, VLOCAL, v); /* variable is local */
            if base == 0 {
                markupval(fs, v); /* local will be used as an upval */
            }
        } else {
            /* not found as local at current level; try upvalues */
            let mut idx = searchupvalue(fs, n); /* try existing upvalues */
            if idx < 0 {
                /* not found? */
                singlevaraux((*fs).prev, n, var, 0); /* try upper levels */
                if (*var).k as c_uint == VVOID as c_uint {
                    /* not found? */
                    return; /* it is a global */
                } /* else was LOCAL or UPVAL */
                idx = newupvalue(fs, n, var); /* will be a new upvalue */
            }
            init_exp(var, VUPVAL, idx); /* new or old upvalue */
        }
    };
}

unsafe extern "C" fn singlevar(ls: *mut LexState, var: *mut expdesc) {
    let varname = str_checkname(ls);
    let fs = (*ls).fs;
    singlevaraux(fs, varname, var, 1 as c_int);
    if (*var).k as c_uint == VVOID as c_int as c_uint {
        let mut key = expdesc {
            k: VVOID,
            u: C2RustUnnamed_8 { ival: 0 },
            t: 0,
            f: 0,
        };
        singlevaraux(fs, (*ls).envn, var, 1 as c_int);
        codestring(ls, &mut key, varname);
        luaK_indexed(fs, var, &mut key);
    }
}

/*
static void adjust_assign (LexState *ls, int nvars, int nexps, expdesc *e) {
  FuncState *fs = ls->fs;
  int extra = nvars - nexps;
  if (hasmultret(e->k)) {
    extra++;  /* includes call itself */
    if (extra < 0) extra = 0;
    luaK_setreturns(fs, e, extra);  /* last exp. provides the difference */
    if (extra > 1) luaK_reserveregs(fs, extra-1);
  }
  else {
    if (e->k != VVOID) luaK_exp2nextreg(fs, e);  /* close last expression */
    if (extra > 0) {
      int reg = fs->freereg;
      luaK_reserveregs(fs, extra);
      luaK_nil(fs, reg, extra);
    }
  }
  if (nexps > nvars)
    ls->fs->freereg -= nexps - nvars;  /* remove extra values */
}
*/

unsafe extern "C" fn adjust_assign(
    mut ls: *mut LexState,
    nvars: c_int,
    nexps: c_int,
    e: *mut expdesc,
) {
    let fs = (*ls).fs;
    let mut extra = nvars - nexps;
    if hasmultret((*e).k) {
        extra += 1;
        if extra < 0 as c_int {
            extra = 0 as c_int;
        }
        luaK_setreturns(fs, e, extra);
        if extra > 1 as c_int {
            luaK_reserveregs(fs, extra - 1 as c_int);
        }
    } else {
        if (*e).k as c_uint != VVOID as c_int as c_uint {
            luaK_exp2nextreg(fs, e);
        }
        if extra > 0 as c_int {
            let reg = (*fs).freereg as c_int;
            luaK_reserveregs(fs, extra);
            luaK_nil(fs, reg, extra);
        }
    }
    if nexps > nvars {
        (*(*ls).fs).freereg = ((*(*ls).fs).freereg as c_int - (nexps - nvars)) as lu_byte;
    }
}

unsafe extern "C" fn enterlevel(ls: *mut LexState) {
    let mut L = (*ls).L;
    (*L).nCcalls = ((*L).nCcalls).wrapping_add(1);
    checklimit(
        (*ls).fs,
        (*L).nCcalls as c_int,
        LUAI_MAXCCALLS as c_int,
        cstr!("C levels"),
    );
}

unsafe extern "C" fn leavelevel(ls: *mut LexState) {
    (*(*ls).L).nCcalls -= 1;
}

unsafe extern "C" fn closegoto(ls: *mut LexState, g: c_int, label: *mut Labeldesc) {
    let mut i: c_int;
    let fs = (*ls).fs;
    let mut gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let gt: *mut Labeldesc = &mut *((*gl).arr).offset(g as isize) as *mut Labeldesc;
    if ((*gt).nactvar as c_int) < (*label).nactvar as c_int {
        let vname = (*getlocvar(fs, (*gt).nactvar as c_int)).varname;
        let msg = luaO_pushfstring(
            (*ls).L,
            cstr!("<goto %s> at line %d jumps into the scope of local '%s'"),
            getstr((*gt).name),
            (*gt).line,
            getstr(vname),
        );
        semerror(ls, msg);
    }
    luaK_patchlist(fs, (*gt).pc, (*label).pc);
    i = g;
    /* remove goto from pending list */
    while i < (*gl).n - 1 as c_int {
        *((*gl).arr).offset(i as isize) = *((*gl).arr).offset((i + 1 as c_int) as isize);
        i += 1;
    }
    (*gl).n -= 1;
}

/*
** try to close a goto with existing labels; this solves backward jumps
*/
unsafe extern "C" fn findlabel(ls: *mut LexState, g: c_int) -> c_int {
    let mut i: c_int;
    let bl = (*(*ls).fs).bl;
    let dyd = (*ls).dyd;
    let gt: *mut Labeldesc = &mut *((*dyd).gt.arr).offset(g as isize) as *mut Labeldesc;
    i = (*bl).firstlabel;
    /* check labels in current block for a match */
    while i < (*dyd).label.n {
        let lb: *mut Labeldesc = &mut *((*dyd).label.arr).offset(i as isize) as *mut Labeldesc;
        if eqstr!((*lb).name, (*gt).name) {
            /* correct label? */
            if (*gt).nactvar as c_int > (*lb).nactvar as c_int
                && ((*bl).upval as c_int != 0 || (*dyd).label.n > (*bl).firstlabel)
            {
                luaK_patchclose((*ls).fs, (*gt).pc, (*lb).nactvar as c_int);
            }
            closegoto(ls, g, lb); /* close it */
            return 1;
        }
        i += 1;
    }
    return 0; /* label not found; cannot close goto */
}
unsafe extern "C" fn newlabelentry(
    ls: *mut LexState,
    mut l: *mut Labellist,
    name: *mut TString,
    line: c_int,
    pc: c_int,
) -> c_int {
    let n = (*l).n;
    luaM_growvector(
        (*ls).L,
        &mut (*l).arr,
        n,
        &mut (*l).size,
        SHRT_MAX as c_int,
        cstr!("labels/gotos"),
    );
    (*((*l).arr).offset(n as isize)).name = name;
    (*((*l).arr).offset(n as isize)).line = line;
    (*((*l).arr).offset(n as isize)).nactvar = (*(*ls).fs).nactvar;
    (*((*l).arr).offset(n as isize)).pc = pc;
    (*l).n = n + 1;
    return n;
}
/*
** check whether new label 'lb' matches any pending gotos in current
** block; solves forward jumps
*/
unsafe extern "C" fn findgotos(ls: *mut LexState, lb: *mut Labeldesc) {
    let gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let mut i = (*(*(*ls).fs).bl).firstgoto;
    while i < (*gl).n {
        if eqstr!((*(*gl).arr.offset(i as isize)).name, (*lb).name) {
            closegoto(ls, i, lb);
        } else {
            i += 1;
        }
    }
}

/*
** export pending gotos to outer level, to check them against
** outer labels; if the block being exited has upvalues, and
** the goto exits the scope of any variable (which can be the
** upvalue), close those variables being exited.
*/
unsafe extern "C" fn movegotosout(fs: *mut FuncState, bl: *mut BlockCnt) {
    let mut i = (*bl).firstgoto;
    let gl: *mut Labellist = &mut (*(*(*fs).ls).dyd).gt;
    /* correct pending gotos to current block and try to close it
    with visible labels */
    while i < (*gl).n {
        let mut gt: *mut Labeldesc = &mut *((*gl).arr).offset(i as isize) as *mut Labeldesc;
        if (*gt).nactvar as c_int > (*bl).nactvar as c_int {
            if (*bl).upval != 0 {
                luaK_patchclose(fs, (*gt).pc, (*bl).nactvar as c_int);
            }
            (*gt).nactvar = (*bl).nactvar;
        }
        if findlabel((*fs).ls, i) == 0 {
            i += 1; /* move to next one */
        }
    }
}

unsafe extern "C" fn enterblock(mut fs: *mut FuncState, mut bl: *mut BlockCnt, isloop: lu_byte) {
    (*bl).isloop = isloop;
    (*bl).nactvar = (*fs).nactvar;
    (*bl).firstlabel = (*(*(*fs).ls).dyd).label.n;
    (*bl).firstgoto = (*(*(*fs).ls).dyd).gt.n;
    (*bl).upval = 0;
    (*bl).previous = (*fs).bl;
    (*fs).bl = bl;
}

/*
** create a label named 'break' to resolve break statements
*/

unsafe extern "C" fn breaklabel(ls: *mut LexState) {
    let n = luaS_new((*ls).L, cstr!("break"));
    let l = newlabelentry(ls, &mut (*(*ls).dyd).label, n, 0 as c_int, (*(*ls).fs).pc);
    findgotos(ls, &mut *((*(*ls).dyd).label.arr).offset(l as isize));
}

/*
** generates an error for an undefined 'goto'; choose appropriate
** message when label name is a reserved word (which can only be 'break')
*/

unsafe extern "C" fn undefgoto(ls: *mut LexState, gt: *mut Labeldesc) {
    let mut msg = if isreserved((*gt).name) {
        cstr!("<%s> at line %d not inside a loop")
    } else {
        cstr!("no visible label '%s' for <goto> at line %d")
    };
    msg = luaO_pushfstring((*ls).L, msg, getstr((*gt).name), (*gt).line);
    semerror(ls, msg);
}

unsafe extern "C" fn leaveblock(mut fs: *mut FuncState) {
    let bl = (*fs).bl;
    let mut ls = (*fs).ls;
    if !((*bl).previous).is_null() && (*bl).upval as c_int != 0 {
        /* create a 'jump to here' to close upvalues */
        let j = luaK_jump(fs);
        luaK_patchclose(fs, j, (*bl).nactvar as c_int);
        luaK_patchtohere(fs, j);
    }
    if (*bl).isloop != 0 {
        breaklabel(ls); /* close pending breaks */
    }
    (*fs).bl = (*bl).previous;
    removevars(fs, (*bl).nactvar as c_int);
    (*fs).freereg = (*fs).nactvar; /* free registers */
    (*(*ls).dyd).label.n = (*bl).firstlabel; /* remove local labels */
    if !((*bl).previous).is_null() {
        /* inner block? */
        movegotosout(fs, bl); /* update pending gotos to outer block */
    } else if (*bl).firstgoto < (*(*ls).dyd).gt.n {
        /* pending gotos in outer block? */
        undefgoto(
            ls,
            &mut *((*(*ls).dyd).gt.arr).offset((*bl).firstgoto as isize),
        ); /* error */
    }
}

unsafe extern "C" fn addprototype(ls: *mut LexState) -> *mut Proto {
    let L = (*ls).L;
    let mut fs = (*ls).fs;
    let f = (*fs).f;
    if (*fs).np >= (*f).sizep {
        let mut oldsize = (*f).sizep;
        luaM_growvector(
            L,
            &mut (*f).p,
            (*fs).np,
            &mut (*f).sizep,
            MAXARG_Bx as c_int,
            cstr!("functions"),
        );
        while oldsize < (*f).sizep {
            *((*f).p).offset(oldsize as isize) = ptr::null_mut() as *mut Proto;
            oldsize = oldsize + 1;
        }
    }
    let clp = luaF_newproto(L);
    *((*f).p).offset((*fs).np as isize) = clp;
    (*fs).np = (*fs).np + 1;
    luaC_objbarrier(L, f as *mut GCObject, clp as *mut GCObject);
    return clp;
}

/*
** codes instruction to create new closure in parent function.
** The OP_CLOSURE instruction must use the last available register,
** so that, if it invokes the GC, the GC knows which registers
** are in use at that time.
*/

unsafe extern "C" fn codeclosure(ls: *mut LexState, v: *mut expdesc) {
    let fs = (*(*ls).fs).prev;
    init_exp(
        v,
        VRELOCABLE,
        luaK_codeABx(fs, OP_CLOSURE, 0, ((*fs).np - 1) as c_uint),
    );
    luaK_exp2nextreg(fs, v); /* fix it at the last register */
}

unsafe extern "C" fn open_func(mut ls: *mut LexState, mut fs: *mut FuncState, bl: *mut BlockCnt) {
    let mut f: *mut Proto;
    (*fs).prev = (*ls).fs; /* linked list of funcstates */
    (*fs).ls = ls;
    (*ls).fs = fs;
    (*fs).pc = 0;
    (*fs).lasttarget = 0;
    (*fs).jpc = NO_JUMP;
    (*fs).freereg = 0;
    (*fs).nk = 0;
    (*fs).np = 0;
    (*fs).nups = 0;
    (*fs).nlocvars = 0;
    (*fs).nactvar = 0;
    (*fs).firstlocal = (*(*ls).dyd).actvar.n;
    (*fs).bl = ptr::null_mut() as *mut BlockCnt;
    f = (*fs).f;
    (*f).source = (*ls).source;
    luaC_objbarrier((*ls).L, f as *mut GCObject, (*f).source as *mut GCObject);
    (*f).maxstacksize = 2; /* registers 0/1 are always valid */
    enterblock(fs, bl, 0);
}

unsafe extern "C" fn close_func(mut ls: *mut LexState) {
    let L = (*ls).L;
    let fs = (*ls).fs;
    let mut f = (*fs).f;
    luaK_ret(fs, 0, 0); /* final return */
    leaveblock(fs);
    luaM_reallocvector(
        L,
        &mut (*f).code,
        (*f).sizecode as size_t,
        (*fs).pc as size_t,
    );
    (*f).sizecode = (*fs).pc;
    luaM_reallocvector(
        L,
        &mut (*f).lineinfo,
        (*f).sizelineinfo as size_t,
        (*fs).pc as size_t,
    );
    (*f).sizelineinfo = (*fs).pc;
    luaM_reallocvector(L, &mut (*f).k, (*f).sizek as size_t, (*fs).nk as size_t);
    (*f).sizek = (*fs).nk;
    luaM_reallocvector(L, &mut (*f).p, (*f).sizep as size_t, (*fs).np as size_t);
    (*f).sizep = (*fs).np;
    luaM_reallocvector(
        L,
        &mut (*f).locvars,
        (*f).sizelocvars as size_t,
        (*fs).nlocvars as size_t,
    );
    (*f).sizelocvars = (*fs).nlocvars as c_int;
    luaM_reallocvector(
        L,
        &mut (*f).upvalues,
        (*f).sizeupvalues as size_t,
        (*fs).nups as size_t,
    );
    (*f).sizeupvalues = (*fs).nups as c_int;
    (*ls).fs = (*fs).prev;
    luaC_checkGC(L);
}

/*============================================================*/
/* GRAMMAR RULES */
/*============================================================*/

/*
** check whether current token is in the follow set of a block.
** 'until' closes syntactical blocks, but do not close scope,
** so it is handled in separate.
*/

unsafe extern "C" fn block_follow(ls: *mut LexState, withuntil: c_int) -> c_int {
    match (*ls).t.token {
        // TK_ELSE |  TK_ELSEIF | TK_END | TK_EOS
        260 | 261 | 262 | 289 => return 1,
        277 => return withuntil, // TK_UNTIL
        _ => return 0,
    };
}

/*
static void statlist (LexState *ls) {
  /* statlist -> { stat [';'] } */
  while (!block_follow(ls, 1)) {
    if (ls->t.token == TK_RETURN) {
      statement(ls);
      return;  /* 'return' must be last statement */
    }
    statement(ls);
  }
}
 */

unsafe extern "C" fn statlist(ls: *mut LexState) {
    /* statlist -> { stat [';'] } */
    while block_follow(ls, 1) == 0 {
        if (*ls).t.token == TK_RETURN {
            statement(ls);
            return; /* 'return' must be last statement */
        }
        statement(ls);
    }
}

unsafe extern "C" fn fieldsel(ls: *mut LexState, v: *mut expdesc) {
    /* fieldsel -> ['.' | ':'] NAME */
    let fs = (*ls).fs;
    let mut key = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    luaK_exp2anyregup(fs, v);
    luaX_next(ls); /* skip the dot or colon */
    checkname(ls, &mut key);
    luaK_indexed(fs, v, &mut key);
}
unsafe extern "C" fn yindex(ls: *mut LexState, v: *mut expdesc) {
    /* index -> '[' expr ']' */
    luaX_next(ls); /* skip the '[' */
    expr(ls, v);
    luaK_exp2val((*ls).fs, v);
    checknext(ls, ']' as i32);
}

/*
** {======================================================================
** Rules for Constructors
** =======================================================================
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConsControl {
    pub v: expdesc,      /* last list item read */
    pub t: *mut expdesc, /* table descriptor */
    pub nh: c_int,       /* total number of 'record' elements */
    pub na: c_int,       /* total number of array elements */
    pub tostore: c_int,  /* number of array elements pending to be stored */
}

unsafe extern "C" fn recfield(ls: *mut LexState, mut cc: *mut ConsControl) {
    /* recfield -> (NAME | '['exp1']') = exp1 */
    let mut fs = (*ls).fs;
    let reg = (*(*ls).fs).freereg as c_int;
    let mut key = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut val = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let rkkey: c_int;
    if (*ls).t.token == TK_NAME as c_int {
        checklimit(fs, (*cc).nh, MAX_INT, cstr!("items in a constructor"));
        checkname(ls, &mut key);
    } else {
        /* ls->t.token == '[' */
        yindex(ls, &mut key);
    }
    (*cc).nh += 1;
    checknext(ls, '=' as i32);
    rkkey = luaK_exp2RK(fs, &mut key);
    expr(ls, &mut val);
    luaK_codeABC(
        fs,
        OP_SETTABLE,
        (*(*cc).t).u.info,
        rkkey,
        luaK_exp2RK(fs, &mut val),
    );
    (*fs).freereg = reg as lu_byte; /* free registers */
}

unsafe extern "C" fn closelistfield(fs: *mut FuncState, mut cc: *mut ConsControl) {
    if (*cc).v.k == VVOID {
        return; /* there is no list item */
    }
    luaK_exp2nextreg(fs, &mut (*cc).v);
    (*cc).v.k = VVOID;
    if (*cc).tostore == LFIELDS_PER_FLUSH as c_int {
        luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore); /* flush */
        (*cc).tostore = 0; /* no more items pending */
    }
}

unsafe extern "C" fn lastlistfield(fs: *mut FuncState, mut cc: *mut ConsControl) {
    if (*cc).tostore == 0 as c_int {
        return;
    }
    if hasmultret((*cc).v.k) {
        luaK_setmultret(fs, &mut (*cc).v);
        luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, LUA_MULTRET);
        (*cc).na -= 1; /* do not count last expression (unknown number of elements) */
    } else {
        if (*cc).v.k != VVOID {
            luaK_exp2nextreg(fs, &mut (*cc).v);
        }
        luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
    };
}

#[inline(always)]
unsafe extern "C" fn luaK_setmultret(fs: *mut FuncState, e: *mut expdesc) {
    luaK_setreturns(fs, e, LUA_MULTRET)
}

unsafe extern "C" fn listfield(ls: *mut LexState, mut cc: *mut ConsControl) {
    /* listfield -> exp */
    expr(ls, &mut (*cc).v);
    checklimit((*ls).fs, (*cc).na, MAX_INT, cstr!("items in a constructor"));
    (*cc).na += 1;
    (*cc).tostore += 1;
}

unsafe extern "C" fn field(ls: *mut LexState, cc: *mut ConsControl) {
    /* field -> listfield | recfield */
    match (*ls).t.token {
        292 => {
            // TK_NAME: may be 'listfield' or 'recfield'
            if luaX_lookahead(ls) != '=' as i32 {
                /* expression? */
                listfield(ls, cc);
            } else {
                recfield(ls, cc);
            }
        }
        91 => {
            // '['
            recfield(ls, cc);
        }
        _ => {
            listfield(ls, cc);
        }
    };
}

unsafe extern "C" fn constructor(ls: *mut LexState, t: *mut expdesc) {
    /* constructor -> '{' [ field { sep field } [sep] ] '}'
    sep -> ',' | ';' */
    let fs = (*ls).fs;
    let line = (*ls).linenumber;
    let pc = luaK_codeABC(fs, OP_NEWTABLE, 0, 0, 0);
    let mut cc = ConsControl {
        v: expdesc {
            k: VVOID,
            u: C2RustUnnamed_8 { ival: 0 },
            t: 0,
            f: 0,
        },
        t: 0 as *mut expdesc,
        nh: 0,
        na: 0,
        tostore: 0,
    };
    cc.tostore = 0 as c_int;
    cc.nh = cc.tostore;
    cc.na = cc.nh;
    cc.t = t;
    init_exp(t, VRELOCABLE, pc);
    init_exp(&mut cc.v, VVOID, 0); /* no value (yet) */
    luaK_exp2nextreg((*ls).fs, t); /* fix it at stack top */
    checknext(ls, '{' as i32);
    while !((*ls).t.token == '}' as i32) {
        closelistfield(fs, &mut cc);
        field(ls, &mut cc);
        if !(testnext(ls, ',' as i32) != 0 || testnext(ls, ';' as i32) != 0) {
            break;
        }
    }
    check_match(ls, '}' as i32, '{' as i32, line);
    lastlistfield(fs, &mut cc);
    SETARG_B(
        (*(*fs).f).code.offset(pc as isize),
        luaO_int2fb(cc.na as c_uint),
    ); /* set initial array size */
    SETARG_C(
        (*(*fs).f).code.offset(pc as isize),
        luaO_int2fb(cc.nh as c_uint),
    ); /* set initial table size */
}

/* }====================================================================== */

unsafe extern "C" fn parlist(ls: *mut LexState) {
    /* parlist -> [ param { ',' param } ] */
    let fs = (*ls).fs;
    let mut f = (*fs).f;
    let mut nparams = 0 as c_int;
    (*f).is_vararg = 0;
    if (*ls).t.token != ')' as i32 {
        /* is 'parlist' not empty? */
        loop {
            match (*ls).t.token {
                292 => {
                    // TK_NAME: param -> NAME
                    new_localvar(ls, str_checkname(ls));
                    nparams += 1;
                }
                281 => {
                    // TK_DOTS: param -> '...'
                    luaX_next(ls);
                    (*f).is_vararg = 1;
                }
                _ => {
                    luaX_syntaxerror(ls, cstr!("<name> or '...' expected"));
                }
            }
            if !((*f).is_vararg == 0 && testnext(ls, ',' as i32) != 0) {
                break;
            }
        }
    }
    adjustlocalvars(ls, nparams);
    (*f).numparams = (*fs).nactvar as lu_byte;
    luaK_reserveregs(fs, (*fs).nactvar as c_int); /* reserve register for parameters */
}

unsafe extern "C" fn body(ls: *mut LexState, e: *mut expdesc, ismethod: c_int, line: c_int) {
    let mut new_fs = FuncState {
        f: 0 as *mut Proto,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        jpc: 0,
        nk: 0,
        np: 0,
        firstlocal: 0,
        nlocvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
    };
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    new_fs.f = addprototype(ls);
    (*new_fs.f).linedefined = line;
    open_func(ls, &mut new_fs, &mut bl);
    checknext(ls, '(' as i32);
    if ismethod != 0 {
        new_localvarliteral(ls, cstr!("self")); /* create 'self' parameter */
        adjustlocalvars(ls, 1);
    }
    parlist(ls);
    checknext(ls, ')' as i32);
    statlist(ls);
    (*new_fs.f).lastlinedefined = (*ls).linenumber;
    check_match(ls, TK_END, TK_FUNCTION, line);
    codeclosure(ls, e);
    close_func(ls);
}

unsafe extern "C" fn explist(ls: *mut LexState, v: *mut expdesc) -> c_int {
    /* explist -> expr { ',' expr } */
    let mut n: c_int = 1; /* at least one expression */
    expr(ls, v);
    while testnext(ls, ',' as i32) != 0 {
        luaK_exp2nextreg((*ls).fs, v);
        expr(ls, v);
        n += 1;
    }
    return n;
}

unsafe extern "C" fn funcargs(ls: *mut LexState, f: *mut expdesc, line: c_int) {
    let mut fs = (*ls).fs;
    let mut args = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let base: c_int;
    let nparams: c_int;
    match (*ls).t.token {
        40 => {
            /* '(': funcargs -> '(' [ explist ] ')' */
            luaX_next(ls);
            if (*ls).t.token == ')' as i32 {
                args.k = VVOID;
            } else {
                explist(ls, &mut args);
                luaK_setmultret(fs, &mut args);
            }
            check_match(ls, ')' as i32, '(' as i32, line);
        }
        123 => {
            /* {  funcargs -> constructor */
            constructor(ls, &mut args);
        }
        293 => {
            // TK_STRING funcargs -> STRING
            codestring(ls, &mut args, (*ls).t.seminfo.ts);
            luaX_next(ls); /* must use 'seminfo' before 'next' */
        }
        _ => {
            luaX_syntaxerror(ls, cstr!("function arguments expected"));
        }
    }
    base = (*f).u.info; /* base register for call */
    if hasmultret(args.k) {
        nparams = LUA_MULTRET; /* open call */
    } else {
        if args.k != VVOID {
            luaK_exp2nextreg(fs, &mut args); /* close last argument */
        }
        nparams = (*fs).freereg as c_int - (base + 1);
    }
    init_exp(f, VCALL, luaK_codeABC(fs, OP_CALL, base, nparams + 1, 2));
    luaK_fixline(fs, line);
    (*fs).freereg = (base + 1) as lu_byte; /* call remove function and arguments and leaves
                                           (unless changed) one result */
}

/*
** {======================================================================
** Expression parsing
** =======================================================================
*/

unsafe extern "C" fn primaryexp(ls: *mut LexState, v: *mut expdesc) {
    /* primaryexp -> NAME | '(' expr ')' */
    match (*ls).t.token {
        40 => {
            // ('
            let line = (*ls).linenumber;
            luaX_next(ls);
            expr(ls, v);
            check_match(ls, ')' as i32, '(' as i32, line);
            luaK_dischargevars((*ls).fs, v);
            return;
        }
        292 => {
            // TK_NAME
            singlevar(ls, v);
            return;
        }
        _ => {
            luaX_syntaxerror(ls, cstr!("unexpected symbol"));
        }
    };
}

unsafe extern "C" fn suffixedexp(ls: *mut LexState, v: *mut expdesc) {
    /* suffixedexp ->
    primaryexp { '.' NAME | '[' exp ']' | ':' NAME funcargs | funcargs } */
    let fs = (*ls).fs;
    let line = (*ls).linenumber;
    primaryexp(ls, v);
    loop {
        match (*ls).t.token {
            46 => {
                // '.':  fieldsel
                fieldsel(ls, v);
            }
            91 => {
                // '[': '[' exp1 ']'
                let mut key = expdesc {
                    k: VVOID,
                    u: C2RustUnnamed_8 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaK_exp2anyregup(fs, v);
                yindex(ls, &mut key);
                luaK_indexed(fs, v, &mut key);
            }
            58 => {
                // ':': ':' NAME funcargs
                let mut key_0 = expdesc {
                    k: VVOID,
                    u: C2RustUnnamed_8 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaX_next(ls);
                checkname(ls, &mut key_0);
                luaK_self(fs, v, &mut key_0);
                funcargs(ls, v, line);
            }
            40 | 293 | 123 => {
                // '(' | TK_STRING | '{': funcargs
                luaK_exp2nextreg(fs, v);
                funcargs(ls, v, line);
            }
            _ => return,
        }
    }
}

unsafe extern "C" fn simpleexp(ls: *mut LexState, mut v: *mut expdesc) {
    /* simpleexp -> FLT | INT | STRING | NIL | TRUE | FALSE | ... |
    constructor | FUNCTION body | suffixedexp */
    match (*ls).t.token {
        290 => {
            // TK_FLT:
            init_exp(v, VKFLT, 0 as c_int);
            (*v).u.nval = (*ls).t.seminfo.r;
        }
        291 => {
            // TK_INT
            init_exp(v, VKINT, 0 as c_int);
            (*v).u.ival = (*ls).t.seminfo.i;
        }
        293 => {
            // TK_STRING
            codestring(ls, v, (*ls).t.seminfo.ts);
        }
        270 => {
            // TK_NIL
            init_exp(v, VNIL, 0);
        }
        276 => {
            // TK_TRUE
            init_exp(v, VTRUE, 0);
        }
        263 => {
            // TK_FALSE
            init_exp(v, VFALSE, 0);
        }
        281 => {
            // TK_DOTS: vararg
            let fs = (*ls).fs;
            check_condition(
                ls,
                (*(*fs).f).is_vararg != 0,
                cstr!("cannot use '...' outside a vararg function"),
            );
            init_exp(v, VVARARG, luaK_codeABC(fs, OP_VARARG, 0, 1, 0));
        }
        123 => {
            // '{':  constructor
            constructor(ls, v);
            return;
        }
        265 => {
            // TK_FUNCTION
            luaX_next(ls);
            body(ls, v, 0, (*ls).linenumber);
            return;
        }
        _ => {
            suffixedexp(ls, v);
            return;
        }
    }
    luaX_next(ls);
}

#[inline(always)]
unsafe fn check_condition(ls: *mut LexState, c: bool, msg: *const c_char) {
    if !c {
        luaX_syntaxerror(ls, msg);
    }
}

unsafe extern "C" fn getunopr(op: c_int) -> UnOpr {
    match op {
        271 => return OPR_NOT,  // TK_NOT
        45 => return OPR_MINUS, // -
        126 => return OPR_BNOT, // ~
        35 => return OPR_LEN,   // #
        _ => return OPR_NOUNOPR,
    };
}

unsafe extern "C" fn getbinopr(op: c_int) -> BinOpr {
    match op {
        43 => return OPR_ADD,     // '+'
        45 => return OPR_SUB,     // '-'
        42 => return OPR_MUL,     // '*'
        37 => return OPR_MOD,     // '%'
        94 => return OPR_POW,     // '^'
        47 => return OPR_DIV,     // '/'
        279 => return OPR_IDIV,   // TK_IDIV
        38 => return OPR_BAND,    // '&'
        124 => return OPR_BOR,    // '|'
        126 => return OPR_BXOR,   // '~'
        286 => return OPR_SHL,    // TK_SHL
        287 => return OPR_SHR,    // TK_SHR
        280 => return OPR_CONCAT, // TK_CONCAT
        285 => return OPR_NE,     // TK_NE
        282 => return OPR_EQ,     // TK_EQ
        60 => return OPR_LT,      // '<'
        284 => return OPR_LE,     // TK_LE
        62 => return OPR_GT,      // '>'
        283 => return OPR_GE,     // TK_GE
        257 => return OPR_AND,    // TK_AND
        272 => return OPR_OR,     // TK_OR
        _ => return OPR_NOBINOPR,
    };
}

pub struct OprPriority {
    pub left: lu_byte,  /* left priority for each binary operator */
    pub right: lu_byte, /* right priority */
}
static mut priority: [OprPriority; 21] = [
    /* ORDER OPR */
    {
        let init = OprPriority {
            // '+'
            left: 10,
            right: 10,
        };
        init
    },
    {
        let init = OprPriority {
            // '-'
            left: 10,
            right: 10,
        };
        init
    },
    {
        let init = OprPriority {
            // '*'
            left: 11,
            right: 11,
        };
        init
    },
    {
        let init = OprPriority {
            // '%'
            left: 11,
            right: 11,
        };
        init
    },
    {
        let init = OprPriority {
            // '^' (right associative)
            left: 14,
            right: 13,
        };
        init
    },
    {
        let init = OprPriority {
            // '/'
            left: 11,
            right: 11,
        };
        init
    },
    {
        let init = OprPriority {
            // '//'
            left: 11,
            right: 11,
        };
        init
    },
    {
        let init = OprPriority {
            // '&'
            left: 6,
            right: 6,
        };
        init
    },
    {
        let init = OprPriority {
            // '|'
            left: 4,
            right: 4,
        };
        init
    },
    {
        let init = OprPriority {
            // '~'
            left: 5,
            right: 5,
        };
        init
    },
    {
        let init = OprPriority {
            // '<<'
            left: 7,
            right: 7,
        };
        init
    },
    {
        let init = OprPriority {
            // '>>'
            left: 7,
            right: 7,
        };
        init
    },
    {
        let init = OprPriority {
            // '..' (right associative)
            left: 9,
            right: 8,
        };
        init
    },
    {
        let init = OprPriority {
            // ==
            left: 3,
            right: 3,
        };
        init
    },
    {
        let init = OprPriority {
            // <
            left: 3,
            right: 3,
        };
        init
    },
    {
        let init = OprPriority {
            // <=
            left: 3,
            right: 3,
        };
        init
    },
    {
        let init = OprPriority {
            // ~=
            left: 3,
            right: 3,
        };
        init
    },
    {
        let init = OprPriority {
            // >
            left: 3,
            right: 3,
        };
        init
    },
    {
        let init = OprPriority {
            // >=
            left: 3,
            right: 3,
        };
        init
    },
    {
        let init = OprPriority {
            // and
            left: 2,
            right: 2,
        };
        init
    },
    {
        let init = OprPriority {
            // or
            left: 1,
            right: 1,
        };
        init
    },
];

pub const UNARY_PRIORITY: c_int = 12; /* priority for unary operators */

/*
** subexpr -> (simpleexp | unop subexpr) { binop subexpr }
** where 'binop' is any binary operator with a priority higher than 'limit'
*/
unsafe extern "C" fn subexpr(ls: *mut LexState, v: *mut expdesc, limit: c_int) -> BinOpr {
    let mut op: c_uint;
    let uop: c_uint;
    enterlevel(ls);
    /* expand while operators have priorities higher than 'limit' */
    uop = getunopr((*ls).t.token);
    if uop as c_uint != OPR_NOUNOPR as c_int as c_uint {
        let line = (*ls).linenumber;
        luaX_next(ls);
        /* read sub-expression with higher priority */
        subexpr(ls, v, UNARY_PRIORITY);
        luaK_prefix((*ls).fs, uop, v, line);
    } else {
        simpleexp(ls, v);
    }
    op = getbinopr((*ls).t.token);
    while op as c_uint != OPR_NOBINOPR as c_int as c_uint
        && priority[op as usize].left as c_int > limit
    {
        let mut v2 = expdesc {
            k: VVOID,
            u: C2RustUnnamed_8 { ival: 0 },
            t: 0,
            f: 0,
        };
        let nextop: c_uint;
        let line_0 = (*ls).linenumber;
        luaX_next(ls);
        luaK_infix((*ls).fs, op, v);
        nextop = subexpr(ls, &mut v2, priority[op as usize].right as c_int);
        luaK_posfix((*ls).fs, op, v, &mut v2, line_0);
        op = nextop;
    }
    leavelevel(ls);
    return op; /* return first untreated operator */
}

unsafe extern "C" fn expr(ls: *mut LexState, v: *mut expdesc) {
    subexpr(ls, v, 0 as c_int);
}

/* }==================================================================== */

/*
** {======================================================================
** Rules for Statements
** =======================================================================
*/

unsafe extern "C" fn block(ls: *mut LexState) {
    /* block -> statlist */
    let fs = (*ls).fs;
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    enterblock(fs, &mut bl, 0);
    statlist(ls);
    leaveblock(fs);
}

/*
** structure to chain all variables in the left-hand side of an
** assignment
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LHS_assign {
    pub prev: *mut LHS_assign,
    pub v: expdesc, /* variable (global, local, upvalue, or indexed) */
}

/*
** check whether, in an assignment to an upvalue/local variable, the
** upvalue/local variable is begin used in a previous assignment to a
** table. If so, save original upvalue/local value in a safe place and
** use this safe copy in the previous assignment.
*/

unsafe extern "C" fn check_conflict(ls: *mut LexState, mut lh: *mut LHS_assign, v: *mut expdesc) {
    let fs = (*ls).fs;
    let extra = (*fs).freereg as c_int; /* eventual position to save local variable */
    let mut conflict: bool = false;
    while !lh.is_null() {
        /* check all previous assignments */
        if (*lh).v.k == VINDEXED {
            /* assigning to a table? */
            /* table is the upvalue/local being assigned now? */
            if (*lh).v.u.ind.vt as c_uint == (*v).k && (*lh).v.u.ind.t as c_int == (*v).u.info {
                conflict = true;
                (*lh).v.u.ind.vt = VLOCAL as c_int as lu_byte;
                (*lh).v.u.ind.t = extra as lu_byte; /* previous assignment will use safe copy */
            }
            /* index is the local being assigned? (index cannot be upvalue) */
            if (*v).k == VLOCAL && (*lh).v.u.ind.idx as c_int == (*v).u.info {
                conflict = true;
                (*lh).v.u.ind.idx = extra as c_short; /* previous assignment will use safe copy */
            }
        }
        lh = (*lh).prev;
    }
    if conflict {
        /* copy upvalue/local value to a temporary (in position 'extra') */
        let op = if (*v).k == VLOCAL {
            OP_MOVE
        } else {
            OP_GETUPVAL
        };
        luaK_codeABC(fs, op, extra, (*v).u.info, 0 as c_int);
        luaK_reserveregs(fs, 1 as c_int);
    }
}

unsafe extern "C" fn assignment(ls: *mut LexState, lh: *mut LHS_assign, nvars: c_int) {
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    check_condition(ls, vkisvar((*lh).v.k), cstr!("syntax error"));
    if testnext(ls, ',' as i32) != 0 {
        /* assignment -> ',' suffixedexp assignment */
        let mut nv = LHS_assign {
            prev: 0 as *mut LHS_assign,
            v: expdesc {
                k: VVOID,
                u: C2RustUnnamed_8 { ival: 0 },
                t: 0,
                f: 0,
            },
        };
        nv.prev = lh;
        suffixedexp(ls, &mut nv.v);
        if nv.v.k as c_uint != VINDEXED as c_int as c_uint {
            check_conflict(ls, lh, &mut nv.v);
        }
        checklimit(
            (*ls).fs,
            nvars + (*(*ls).L).nCcalls as c_int,
            LUAI_MAXCCALLS as c_int,
            cstr!("C levels\0"),
        );
        assignment(ls, &mut nv, nvars + 1 as c_int);
    } else {
        /* assignment -> '=' explist */
        checknext(ls, '=' as i32);
        let nexps = explist(ls, &mut e);
        if nexps != nvars {
            adjust_assign(ls, nvars, nexps, &mut e);
        } else {
            luaK_setoneret((*ls).fs, &mut e); /* close last expression */
            luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
            return; /* avoid default */
        }
    }
    init_exp(&mut e, VNONRELOC, (*(*ls).fs).freereg as c_int - 1); /* default assignment */
    luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
}

unsafe extern "C" fn cond(ls: *mut LexState) -> c_int {
    /* cond -> exp */
    let mut v = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    expr(ls, &mut v); /* read condition */
    if v.k as c_uint == VNIL as c_int as c_uint {
        v.k = VFALSE; /* 'falses' are all equal here */
    }
    luaK_goiftrue((*ls).fs, &mut v);
    return v.f;
}

unsafe extern "C" fn gotostat(ls: *mut LexState, pc: c_int) {
    let line = (*ls).linenumber;
    let label: *mut TString;
    let g: c_int;
    if testnext(ls, TK_GOTO as c_int) != 0 {
        label = str_checkname(ls);
    } else {
        luaX_next(ls); /* skip break */
        label = luaS_new((*ls).L, b"break\0" as *const u8 as *const c_char);
    }
    g = newlabelentry(ls, &mut (*(*ls).dyd).gt, label, line, pc);
    findlabel(ls, g); /* close it if label already defined */
}

/* check for repeated labels on the same block */
unsafe extern "C" fn checkrepeated(fs: *mut FuncState, ll: *mut Labellist, label: *mut TString) {
    let mut i: c_int;
    i = (*(*fs).bl).firstlabel;
    while i < (*ll).n {
        if eqstr!(label, (*(*ll).arr.offset(i as isize)).name) {
            let msg = luaO_pushfstring(
                (*(*fs).ls).L,
                cstr!("label '%s' already defined on line %d"),
                getstr(label),
                (*((*ll).arr).offset(i as isize)).line,
            );
            semerror((*fs).ls, msg);
        }
        i += 1;
    }
}

/* skip no-op statements */
unsafe extern "C" fn skipnoopstat(ls: *mut LexState) {
    while (*ls).t.token == ';' as i32 || (*ls).t.token == TK_DBCOLON as c_int {
        statement(ls);
    }
}

unsafe extern "C" fn labelstat(ls: *mut LexState, label: *mut TString, line: c_int) {
    /* label -> '::' NAME '::' */
    let fs = (*ls).fs;
    let ll: *mut Labellist = &mut (*(*ls).dyd).label;
    let l: c_int; /* index of new label being created */
    checkrepeated(fs, ll, label); /* check for repeated labels */
    checknext(ls, TK_DBCOLON); /* skip double colon */
    /* create new entry for this label */
    l = newlabelentry(ls, ll, label, line, luaK_getlabel(fs));
    skipnoopstat(ls); /* skip other no-op statements */
    if block_follow(ls, 0 as c_int) != 0 {
        /* label is last no-op statement in the block? */
        /* assume that locals are already out of scope */
        (*((*ll).arr).offset(l as isize)).nactvar = (*(*fs).bl).nactvar;
    }
    findgotos(ls, &mut *((*ll).arr).offset(l as isize));
}

unsafe extern "C" fn whilestat(ls: *mut LexState, line: c_int) {
    /* whilestat -> WHILE cond DO block END */
    let fs = (*ls).fs;
    let whileinit: c_int;
    let condexit: c_int;
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    luaX_next(ls); /* skip WHILE */
    whileinit = luaK_getlabel(fs);
    condexit = cond(ls);
    enterblock(fs, &mut bl, 1 as c_int as lu_byte);
    checknext(ls, TK_DO as c_int);
    block(ls);
    luaK_jumpto(fs, whileinit);
    check_match(ls, TK_END as c_int, TK_WHILE as c_int, line);
    leaveblock(fs);
    luaK_patchtohere(fs, condexit); /* false conditions finish the loop */
}

/* static void repeatstat (LexState *ls, int line) {
  /* repeatstat -> REPEAT block UNTIL cond */
  int condexit;
  FuncState *fs = ls->fs;
  int repeat_init = luaK_getlabel(fs);
  BlockCnt bl1, bl2;
  enterblock(fs, &bl1, 1);  /* loop block */
  enterblock(fs, &bl2, 0);  /* scope block */
  luaX_next(ls);  /* skip REPEAT */
  statlist(ls);
  check_match(ls, TK_UNTIL, TK_REPEAT, line);
  condexit = cond(ls);  /* read condition (inside scope block) */
  if (bl2.upval)  /* upvalues? */
    luaK_patchclose(fs, condexit, bl2.nactvar);
  leaveblock(fs);  /* finish scope */
  luaK_patchlist(fs, condexit, repeat_init);  /* close the loop */
  leaveblock(fs);  /* finish loop */
}
 */

unsafe extern "C" fn repeatstat(ls: *mut LexState, line: c_int) {
    /* repeatstat -> REPEAT block UNTIL cond */
    let condexit: c_int;
    let fs = (*ls).fs;
    let repeat_init = luaK_getlabel(fs);
    let mut bl1 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut bl2 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    enterblock(fs, &mut bl1, 1 as c_int as lu_byte); /* loop block */
    enterblock(fs, &mut bl2, 0 as c_int as lu_byte); /* scope block */
    luaX_next(ls); /* skip REPEAT */
    statlist(ls);
    check_match(ls, TK_UNTIL as c_int, TK_REPEAT as c_int, line);
    condexit = cond(ls); /* read condition (inside scope block) */
    if bl2.upval != 0 {
        /* upvalues? */
        luaK_patchclose(fs, condexit, bl2.nactvar as c_int);
    }
    leaveblock(fs); /* finish scope */
    luaK_patchlist(fs, condexit, repeat_init); /* close the loop */
    leaveblock(fs); /* finish loop */
}

unsafe extern "C" fn exp1(ls: *mut LexState) -> c_int {
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    expr(ls, &mut e);
    luaK_exp2nextreg((*ls).fs, &mut e);
    return e.u.info;
}

unsafe extern "C" fn forbody(
    ls: *mut LexState,
    base: c_int,
    line: c_int,
    nvars: c_int,
    isnum: c_int,
) {
    /* forbody -> DO block */
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let fs = (*ls).fs;
    let prep: c_int;
    let endfor: c_int;
    adjustlocalvars(ls, 3); /* control variables */
    checknext(ls, TK_DO as c_int);
    prep = if isnum != 0 {
        luaK_codeAsBx(fs, OP_FORPREP, base, NO_JUMP)
    } else {
        luaK_jump(fs)
    };
    enterblock(fs, &mut bl, 0); /* scope for declared variables */
    adjustlocalvars(ls, nvars);
    luaK_reserveregs(fs, nvars);
    block(ls);
    leaveblock(fs); /* end of scope for declared variables */
    luaK_patchtohere(fs, prep);
    if isnum != 0 {
        /* numeric for? */
        endfor = luaK_codeAsBx(fs, OP_FORLOOP, base, NO_JUMP);
    } else {
        /* generic for */
        luaK_codeABC(fs, OP_TFORCALL, base, 0 as c_int, nvars);
        luaK_fixline(fs, line);
        endfor = luaK_codeAsBx(fs, OP_TFORLOOP, base + 2, NO_JUMP);
    }
    luaK_patchlist(fs, endfor, prep + 1 as c_int);
    luaK_fixline(fs, line);
}

unsafe extern "C" fn fornum(ls: *mut LexState, varname: *mut TString, line: c_int) {
    /* fornum -> NAME = exp1,exp1[,exp1] forbody */
    let fs = (*ls).fs;
    let base = (*fs).freereg as c_int;
    new_localvarliteral(ls, cstr!("(for index)"));
    new_localvarliteral(ls, cstr!("(for limit)"));
    new_localvarliteral(ls, cstr!("(for step)"));
    new_localvar(ls, varname);
    checknext(ls, '=' as i32);
    exp1(ls); /* initial value */
    checknext(ls, ',' as i32);
    exp1(ls); /* limit */
    if testnext(ls, ',' as i32) != 0 {
        exp1(ls); /* optional step */
    } else {
        /* default step = 1 */
        luaK_codek(fs, (*fs).freereg as c_int, luaK_intK(fs, 1));
        luaK_reserveregs(fs, 1);
    }
    forbody(ls, base, line, 1, 1);
}

unsafe extern "C" fn forlist(ls: *mut LexState, indexname: *mut TString) {
    let fs = (*ls).fs;
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut nvars = 4 as c_int;
    let line: c_int;
    let base = (*fs).freereg as c_int;
    new_localvarliteral(ls, cstr!("(for generator)"));
    new_localvarliteral(ls, cstr!("(for state)"));
    new_localvarliteral(ls, cstr!("(for control)"));
    new_localvar(ls, indexname);
    while testnext(ls, ',' as i32) != 0 {
        new_localvar(ls, str_checkname(ls));
        nvars += 1;
    }
    checknext(ls, TK_IN as c_int);
    line = (*ls).linenumber;
    adjust_assign(ls, 3 as c_int, explist(ls, &mut e), &mut e);
    luaK_checkstack(fs, 3 as c_int);
    forbody(ls, base, line, nvars - 3 as c_int, 0 as c_int);
}

unsafe extern "C" fn forstat(ls: *mut LexState, line: c_int) {
    /* forstat -> FOR (fornum | forlist) END */
    let fs = (*ls).fs;
    let varname: *mut TString;
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    enterblock(fs, &mut bl, 1); /* scope for loop and control variables */
    luaX_next(ls); /* skip 'for' */
    varname = str_checkname(ls); /* first variable name */
    match (*ls).t.token {
        61 => {
            // '='
            fornum(ls, varname, line);
        }
        44 | 268 => {
            // ',' | TK_IN
            forlist(ls, varname);
        }
        _ => {
            luaX_syntaxerror(ls, cstr!("'=' or 'in' expected"));
        }
    }
    check_match(ls, TK_END as c_int, TK_FOR as c_int, line);
    leaveblock(fs); /* loop scope ('break' jumps to this point) */
}

unsafe extern "C" fn test_then_block(ls: *mut LexState, escapelist: *mut c_int) {
    /* test_then_block -> [IF | ELSEIF] cond THEN block */
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let fs = (*ls).fs;
    let mut v = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let jf: c_int; /* instruction to skip 'then' code (if condition is false) */
    luaX_next(ls); /* skip IF or ELSEIF */
    expr(ls, &mut v); /* read condition */
    checknext(ls, TK_THEN as c_int);
    if (*ls).t.token == TK_GOTO as c_int || (*ls).t.token == TK_BREAK as c_int {
        luaK_goiffalse((*ls).fs, &mut v); /* will jump to label if condition is true */
        enterblock(fs, &mut bl, 0); /* must enter block before 'goto' */
        gotostat(ls, v.t); /* handle goto/break */
        while testnext(ls, ';' as i32) != 0 {} /* skip colons */
        if block_follow(ls, 0) != 0 {
            /* 'goto' is the entire block? */
            leaveblock(fs);
            return; /* and that is it */
        } else {
            /* must skip over 'then' part if condition is false */
            jf = luaK_jump(fs);
        }
    } else {
        /* regular case (not goto/break) */
        luaK_goiftrue((*ls).fs, &mut v); /* skip over block if condition is false */
        enterblock(fs, &mut bl, 0);
        jf = v.f;
    }
    statlist(ls); /* 'then' part */
    leaveblock(fs);
    if (*ls).t.token == TK_ELSE || (*ls).t.token == TK_ELSEIF {
        /* followed by 'else'/'elseif'? */
        luaK_concat(fs, escapelist, luaK_jump(fs)); /* must jump over it */
    }
    luaK_patchtohere(fs, jf);
}

unsafe extern "C" fn ifstat(ls: *mut LexState, line: c_int) {
    /* ifstat -> IF cond THEN block {ELSEIF cond THEN block} [ELSE block] END */
    let fs = (*ls).fs;
    let mut escapelist = NO_JUMP; /* exit list for finished parts */
    test_then_block(ls, &mut escapelist); /* IF cond THEN block */
    while (*ls).t.token == TK_ELSEIF {
        test_then_block(ls, &mut escapelist); /* ELSEIF cond THEN block */
    }
    if testnext(ls, TK_ELSE) != 0 {
        block(ls); /* 'else' part */
    }
    check_match(ls, TK_END, TK_IF, line);
    luaK_patchtohere(fs, escapelist); /* patch escape list to 'if' end */
}

unsafe extern "C" fn localfunc(ls: *mut LexState) {
    let mut b = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let fs = (*ls).fs;
    new_localvar(ls, str_checkname(ls)); /* new local variable */
    adjustlocalvars(ls, 1); /* enter its scope */
    body(ls, &mut b, 0, (*ls).linenumber); /* function created in next register */
    /* debug information will only see the variable after this point! */
    (*getlocvar(fs, b.u.info)).startpc = (*fs).pc;
}

unsafe extern "C" fn localstat(ls: *mut LexState) {
    /* stat -> LOCAL NAME {',' NAME} ['=' explist] */
    let mut nvars = 0 as c_int;
    let nexps: c_int;
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    loop {
        new_localvar(ls, str_checkname(ls));
        nvars += 1;
        if !(testnext(ls, ',' as i32) != 0) {
            break;
        }
    }
    if testnext(ls, '=' as i32) != 0 {
        nexps = explist(ls, &mut e);
    } else {
        e.k = VVOID;
        nexps = 0 as c_int;
    }
    adjust_assign(ls, nvars, nexps, &mut e);
    adjustlocalvars(ls, nvars);
}

unsafe extern "C" fn funcname(ls: *mut LexState, v: *mut expdesc) -> c_int {
    /* funcname -> NAME {fieldsel} [':' NAME] */
    let mut ismethod = 0;
    singlevar(ls, v);
    while (*ls).t.token == '.' as i32 {
        fieldsel(ls, v);
    }
    if (*ls).t.token == ':' as i32 {
        ismethod = 1 as c_int;
        fieldsel(ls, v);
    }
    return ismethod;
}

unsafe extern "C" fn funcstat(ls: *mut LexState, line: c_int) {
    /* funcstat -> FUNCTION funcname body */
    let ismethod: c_int;
    let mut v = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut b = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    luaX_next(ls); /* skip FUNCTION */
    ismethod = funcname(ls, &mut v);
    body(ls, &mut b, ismethod, line);
    luaK_storevar((*ls).fs, &mut v, &mut b);
    luaK_fixline((*ls).fs, line); /* definition "happens" in the first line */
}

unsafe extern "C" fn exprstat(ls: *mut LexState) {
    /* stat -> func | assignment */
    let fs = (*ls).fs;
    let mut v = LHS_assign {
        prev: 0 as *mut LHS_assign,
        v: expdesc {
            k: VVOID,
            u: C2RustUnnamed_8 { ival: 0 },
            t: 0,
            f: 0,
        },
    };
    suffixedexp(ls, &mut v.v);
    if (*ls).t.token == '=' as i32 || (*ls).t.token == ',' as i32 {
        /* stat -> assignment ? */
        v.prev = ptr::null_mut() as *mut LHS_assign;
        assignment(ls, &mut v, 1 as c_int);
    } else {
        check_condition(ls, v.v.k == VCALL, cstr!("syntax error"));
        SETARG_C(getinstruction(fs, &mut v.v), 1); /* call statement uses no results */
    };
}

unsafe extern "C" fn retstat(ls: *mut LexState) {
    /* stat -> RETURN [explist] [';'] */
    let fs = (*ls).fs;
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let first: c_int; /* registers with returned values */
    let mut nret: c_int;
    if block_follow(ls, 1 as c_int) != 0 || (*ls).t.token == ';' as i32 {
        nret = 0;
        first = nret; /* return no values */
    } else {
        nret = explist(ls, &mut e); /* optional return values */
        if hasmultret(e.k) {
            luaK_setmultret(fs, &mut e);
            if e.k == VCALL && nret == 1 {
                /* tail call? */
                SET_OPCODE(&mut *getinstruction(fs, &mut e), OP_TAILCALL);
            }
            first = (*fs).nactvar as c_int;
            nret = LUA_MULTRET; /* return all values */
        } else if nret == 1 {
            /* only one single value? */
            first = luaK_exp2anyreg(fs, &mut e);
        } else {
            luaK_exp2nextreg(fs, &mut e); /* values must go to the stack */
            first = (*fs).nactvar as c_int; /* return all active values */
        }
    }
    luaK_ret(fs, first, nret);
    testnext(ls, ';' as i32); /* skip optional semicolon */
}

unsafe extern "C" fn statement(mut ls: *mut LexState) {
    let line = (*ls).linenumber;
    enterlevel(ls);
    match (*ls).t.token {
        59 => {
            // ';' stat -> ';' (empty statement)
            luaX_next(ls); /* skip ';' */
        }
        267 => {
            // TK_IF: stat -> ifstat
            ifstat(ls, line);
        }
        278 => {
            // TK_WHILE: stat -> whilestat
            whilestat(ls, line);
        }
        259 => {
            // TK_DO: stat -> DO block END
            luaX_next(ls); /* skip DO */
            block(ls);
            check_match(ls, TK_END, TK_DO, line);
        }
        264 => {
            // TK_FOR: stat -> forstat
            forstat(ls, line);
        }
        273 => {
            // TK_REPEAT: stat -> repeatstat
            repeatstat(ls, line);
        }
        265 => {
            // TK_FUNCTION: stat -> funcstat
            funcstat(ls, line);
        }
        269 => {
            // TK_LOCAL: stat -> localstat
            luaX_next(ls); /* skip LOCAL */
            if testnext(ls, TK_FUNCTION) != 0 {
                /* local function? */
                localfunc(ls);
            } else {
                localstat(ls);
            }
        }
        288 => {
            // TK_DBCOLON: stat -> label
            luaX_next(ls); /* skip double colon */
            labelstat(ls, str_checkname(ls), line);
        }
        274 => {
            // TK_RETURN: stat -> retstat
            luaX_next(ls); /* skip RETURN */
            retstat(ls);
        }
        258 | 266 => {
            // TK_BREAK | TK_GOTO:   stat -> breakstat  | 'goto' NAME
            gotostat(ls, luaK_jump((*ls).fs));
        }
        _ => {
            /* stat -> func | assignment */
            exprstat(ls);
        }
    }
    debug_assert!(
        (*(*(*ls).fs).f).maxstacksize >= (*(*ls).fs).freereg
            && (*(*ls).fs).freereg >= (*(*ls).fs).nactvar
    );
    (*(*ls).fs).freereg = (*(*ls).fs).nactvar; /* free registers */
    leavelevel(ls);
}

/* }====================================================================== */

/*
** compiles the main function, which is a regular vararg function with an
** upvalue named LUA_ENV
*/
unsafe extern "C" fn mainfunc(ls: *mut LexState, mut fs: *mut FuncState) {
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut v = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    open_func(ls, fs, &mut bl);
    (*(*fs).f).is_vararg = 1; /* main function is always declared vararg */
    init_exp(&mut v, VLOCAL, 0); /* create and... */
    newupvalue(fs, (*ls).envn, &mut v); /* ...set environment upvalue */
    luaC_objbarrier(
        (*ls).L,
        (*fs).f as *mut GCObject,
        (*ls).envn as *mut GCObject,
    );
    luaX_next(ls); /* read first token */
    statlist(ls); /* parse main body */
    check(ls, TK_EOS);
    close_func(ls);
}

#[no_mangle]
pub unsafe extern "C" fn luaY_parser(
    mut L: *mut lua_State,
    z: *mut ZIO,
    buff: *mut Mbuffer,
    mut dyd: *mut Dyndata,
    name: *const c_char,
    firstchar: c_int,
) -> *mut LClosure {
    let mut lexstate = LexState {
        current: 0,
        linenumber: 0,
        lastline: 0,
        t: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        lookahead: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        fs: 0 as *mut FuncState,
        L: 0 as *mut lua_State,
        z: 0 as *mut ZIO,
        buff: 0 as *mut Mbuffer,
        h: 0 as *mut Table,
        dyd: 0 as *mut Dyndata,
        source: 0 as *mut TString,
        envn: 0 as *mut TString,
    };
    let mut funcstate = FuncState {
        f: 0 as *mut Proto,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        jpc: 0,
        nk: 0,
        np: 0,
        firstlocal: 0,
        nlocvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
    };
    let mut cl = luaF_newLclosure(L, 1 as c_int); /* create main closure */
    setclLvalue(L, (*L).top, cl); /* anchor it (to avoid being collected) */
    luaD_inctop(L);
    lexstate.h = luaH_new(L); /* create table for scanner */
    sethvalue(L, (*L).top, lexstate.h); /* anchor it */
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    funcstate.f = (*cl).p;
    luaC_objbarrier(L, cl as *mut GCObject, (*cl).p as *mut GCObject);
    (*funcstate.f).source = luaS_new(L, name); /* create and anchor TString */
    // lua_assert(iswhite(funcstate.f));  /* do not need barrier here */
    lexstate.buff = buff;
    lexstate.dyd = dyd;
    (*dyd).label.n = 0 as c_int;
    (*dyd).gt.n = (*dyd).label.n;
    (*dyd).actvar.n = (*dyd).gt.n;
    luaX_setinput(L, &mut lexstate, z, (*funcstate.f).source, firstchar);
    mainfunc(&mut lexstate, &mut funcstate);
    //lua_assert(!funcstate.prev && funcstate.nups == 1 && !lexstate.fs);
    /* all scopes should be correctly finished */
    (*L).top = ((*L).top).offset(-1); /* remove scanner's table */
    return cl; /* closure is on the stack, too */
}
