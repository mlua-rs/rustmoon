/*
  Expression and variable descriptor.
  Code generation for variables and expressions can be delayed to allow
  optimizations; An 'expdesc' structure describes a potentially-delayed
  variable/expression. It has a description of its "main" value plus a
  list of conditional jumps that can also produce its value (generated
  by short-circuit operators 'and'/'or').
*/

// kinds of variables/expressions
pub type expkind = libc::c_uint;

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
    // patch list of 'exit when true'
    pub t: libc::c_int,
    // patch list of 'exit when false'
    pub f: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    // for VKINT
    pub ival: lua_Integer,
    // for VKFLT
    pub nval: lua_Number,
    // for generic use
    pub info: libc::c_int,
    // for indexed variables (VINDEXED)
    pub ind: C2RustUnnamed_9,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    // index (R/K)
    pub idx: libc::c_short,
    // table (register or upvalue)
    pub t: lu_byte,
    // whether 't' is register (VLOCAL) or upvalue (VUPVAL)
    pub vt: lu_byte,
}

// description of active local variable
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vardesc {
    // variable index in stack
    pub idx: libc::c_short,
}

// description of pending goto statements and label statements
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc {
    // label identifier
    pub name: *mut TString,
    // position in code
    pub pc: libc::c_int,
    // line where it appeared
    pub line: libc::c_int,
    // local level where it appears in current block
    pub nactvar: lu_byte,
}

// list of labels or gotos
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist {
    // array
    pub arr: *mut Labeldesc,
    // number of entries in use
    pub n: libc::c_int,
    // array size
    pub size: libc::c_int,
}

// dynamic structures used by the parser
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dyndata {
    // list of active local variables
    pub actvar: C2RustUnnamed_7,
    // list of pending gotos
    pub gt: Labellist,
    // list of active labels
    pub label: Labellist,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub arr: *mut Vardesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}

// nodes for block list (list of active blocks)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockCnt {
    // chain
    pub previous: *mut BlockCnt,
    // index of first label in this block
    pub firstlabel: libc::c_int,
    // index of first pending goto in this block
    pub firstgoto: libc::c_int,
    // # active locals outside the block
    pub nactvar: lu_byte,
    //  true if some variable in the block is an upvalue
    pub upval: lu_byte,
    // true if 'block' is a loop
    pub isloop: lu_byte,
}

// state needed to generate code for a given function
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    // current function header
    pub f: *mut Proto,
    // enclosing function
    pub prev: *mut FuncState,
    // lexical state
    pub ls: *mut LexState,
    // chain of current blocks
    pub bl: *mut BlockCnt,
    // next position to code (equivalent to 'ncode')
    pub pc: libc::c_int,
    // 'label' of last 'jump label'
    pub lasttarget: libc::c_int,
    // list of pending jumps to 'pc'
    pub jpc: libc::c_int,
    // number of elements in 'k'
    pub nk: libc::c_int,
    // number of elements in 'p'
    pub np: libc::c_int,
    // index of first local var (in Dyndata array)
    pub firstlocal: libc::c_int,
    // number of elements in 'f->locvars'
    pub nlocvars: libc::c_short,
    // number of active local variables
    pub nactvar: lu_byte,
    // number of upvalues
    pub nups: lu_byte,
    // first free register
    pub freereg: lu_byte,
}

// maximum number of local variables per function
// (must be smaller than 250, due to the bytecode format)
pub const MAXVARS: libc::c_int = 200 as libc::c_int;

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

// semantic error
unsafe fn semerror(mut ls: *mut LexState, mut msg: *const libc::c_char) -> ! {
    // remove "near <token>" from final message
    (*ls).t.token = 0 as libc::c_int;
    luaX_syntaxerror(ls, msg);
}

unsafe fn error_expected(mut ls: *mut LexState, mut token: libc::c_int) -> ! {
    luaX_syntaxerror(
        ls,
        luaO_pushfstring(
            (*ls).L,
            b"%s expected\0" as *const u8 as *const libc::c_char,
            luaX_token2str(ls, token),
        ),
    );
}

unsafe fn errorlimit(
    mut fs: *mut FuncState,
    mut limit: libc::c_int,
    mut what: *const libc::c_char,
) -> ! {
    let mut L = (*(*fs).ls).L;
    let mut msg = 0 as *const libc::c_char;
    let mut line = (*(*fs).f).linedefined;
    let mut where_0 = if line == 0 as libc::c_int {
        b"main function\0" as *const u8 as *const libc::c_char
    } else {
        luaO_pushfstring(
            L,
            b"function at line %d\0" as *const u8 as *const libc::c_char,
            line,
        )
    };
    msg = luaO_pushfstring(
        L,
        b"too many %s (limit is %d) in %s\0" as *const u8 as *const libc::c_char,
        what,
        limit,
        where_0,
    );
    luaX_syntaxerror((*fs).ls, msg);
}

unsafe fn checklimit(
    mut fs: *mut FuncState,
    mut v: libc::c_int,
    mut l: libc::c_int,
    mut what: *const libc::c_char,
) {
    if v > l {
        errorlimit(fs, l, what);
    }
}

unsafe fn testnext(mut ls: *mut LexState, mut c: libc::c_int) -> libc::c_int {
    if (*ls).t.token == c {
        luaX_next(ls);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}

unsafe fn check(mut ls: *mut LexState, mut c: libc::c_int) {
    if (*ls).t.token != c {
        error_expected(ls, c);
    }
}

unsafe fn checknext(mut ls: *mut LexState, mut c: libc::c_int) {
    check(ls, c);
    luaX_next(ls);
}

#[inline]
unsafe fn check_condition(ls: *mut LexState, c: libc::c_int, msg: *const libc::c_char) {
    if c == 0 {
        luaX_syntaxerror(ls, msg);
    }
}

unsafe fn check_match(
    mut ls: *mut LexState,
    mut what: libc::c_int,
    mut who: libc::c_int,
    mut where_0: libc::c_int,
) {
    if testnext(ls, what) == 0 {
        if where_0 == (*ls).linenumber {
            error_expected(ls, what);
        } else {
            luaX_syntaxerror(
                ls,
                luaO_pushfstring(
                    (*ls).L,
                    b"%s expected (to close %s at line %d)\0" as *const u8 as *const libc::c_char,
                    luaX_token2str(ls, what),
                    luaX_token2str(ls, who),
                    where_0,
                ),
            );
        }
    }
}

unsafe fn str_checkname(mut ls: *mut LexState) -> *mut TString {
    let mut ts = 0 as *mut TString;
    check(ls, TK_NAME as libc::c_int);
    ts = (*ls).t.seminfo.ts;
    luaX_next(ls);
    return ts;
}

unsafe fn init_exp(mut e: *mut expdesc, mut k: expkind, mut i: libc::c_int) {
    (*e).t = NO_JUMP;
    (*e).f = (*e).t;
    (*e).k = k;
    (*e).u.info = i;
}

unsafe fn codestring(mut ls: *mut LexState, mut e: *mut expdesc, mut s: *mut TString) {
    init_exp(e, VK, luaK_stringK((*ls).fs, s));
}

unsafe fn checkname(mut ls: *mut LexState, mut e: *mut expdesc) {
    codestring(ls, e, str_checkname(ls));
}

unsafe fn registerlocalvar(mut ls: *mut LexState, mut varname: *mut TString) -> libc::c_int {
    let mut fs = (*ls).fs;
    let mut f = (*fs).f;
    let mut oldsize = (*f).sizelocvars;
    // luaM_growvector
    if (*fs).nlocvars as libc::c_int + 1 as libc::c_int > (*f).sizelocvars {
        (*f).locvars = luaM_growaux_(
            (*ls).L,
            (*f).locvars as *mut libc::c_void,
            &mut (*f).sizelocvars,
            ::core::mem::size_of::<LocVar>() as libc::c_ulong,
            libc::c_short::MAX as libc::c_int,
            b"local variables\0" as *const u8 as *const libc::c_char,
        ) as *mut LocVar;
    }
    while oldsize < (*f).sizelocvars {
        let fresh96 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh97 = (*((*f).locvars).offset(fresh96 as isize)).varname;
        *fresh97 = NULL as *mut TString;
    }
    let ref mut fresh98 = (*((*f).locvars).offset((*fs).nlocvars as isize)).varname;
    *fresh98 = varname;
    if (*f).marked as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0
        && (*varname).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
            != 0
    {
        luaC_barrier_(
            (*ls).L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*(varname as *mut GCUnion)).gc,
        );
    } else {
    };
    let fresh99 = (*fs).nlocvars;
    (*fs).nlocvars = (*fs).nlocvars + 1;
    return fresh99 as libc::c_int;
}

unsafe fn new_localvar(mut ls: *mut LexState, mut name: *mut TString) {
    let mut fs = (*ls).fs;
    let mut dyd = (*ls).dyd;
    let mut reg = registerlocalvar(ls, name);
    checklimit(
        fs,
        (*dyd).actvar.n + 1 as libc::c_int - (*fs).firstlocal,
        MAXVARS,
        b"local variables\0" as *const u8 as *const libc::c_char,
    );
    luaM_growvector!(
        (*ls).L,
        (*dyd).actvar.arr,
        (*dyd).actvar.n + 1,
        (*dyd).actvar.size,
        Vardesc,
        MAX_INT,
        "local variables"
    );
    let fresh158 = (*dyd).actvar.n;
    (*dyd).actvar.n = (*dyd).actvar.n + 1;
    (*((*dyd).actvar.arr).offset(fresh158 as isize)).idx = reg as libc::c_short;
}

unsafe fn new_localvarliteral_(
    mut ls: *mut LexState,
    mut name: *const libc::c_char,
    mut sz: size_t,
) {
    new_localvar(ls, luaX_newstring(ls, name, sz));
}

// TODO new_localvarliteral

unsafe fn getlocvar(mut fs: *mut FuncState, mut i: libc::c_int) -> *mut LocVar {
    let mut idx = (*((*(*(*fs).ls).dyd).actvar.arr).offset(((*fs).firstlocal + i) as isize)).idx
        as libc::c_int;
    assert_debug!(idx < (*fs).nlocvars);
    return &mut *((*(*fs).f).locvars).offset(idx as isize) as *mut LocVar;
}

unsafe fn adjustlocalvars(mut ls: *mut LexState, mut nvars: libc::c_int) {
    let mut fs = (*ls).fs;
    (*fs).nactvar = (*fs).nactvar + (nvars as lu_byte);
    while nvars != 0 {
        (*getlocvar(fs, (*fs).nactvar as libc::c_int - nvars)).startpc = (*fs).pc;
        nvars -= 1;
    }
}

unsafe fn removevars(mut fs: *mut FuncState, mut tolevel: libc::c_int) {
    (*(*(*fs).ls).dyd).actvar.n -= (*fs).nactvar as libc::c_int - tolevel;
    while (*fs).nactvar as libc::c_int > tolevel {
        (*fs).nactvar = ((*fs).nactvar).wrapping_sub(1);
        (*getlocvar(fs, (*fs).nactvar as libc::c_int)).endpc = (*fs).pc;
    }
}

unsafe fn searchupvalue(mut fs: *mut FuncState, mut name: *mut TString) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut up = (*(*fs).f).upvalues;
    while i < (*fs).nups as libc::c_int {
        if eqstr!(up[i].name, name) != 0 {
            return i;
        }
        i += 1;
    }
    // not found
    return -(1 as libc::c_int);
}

unsafe fn newupvalue(
    mut fs: *mut FuncState,
    mut name: *mut TString,
    mut v: *mut expdesc,
) -> libc::c_int {
    let mut f = (*fs).f;
    let mut oldsize = (*f).sizeupvalues;
    checklimit(
        fs,
        (*fs).nups as libc::c_int + 1 as libc::c_int,
        MAXUPVAL,
        b"upvalues\0" as *const u8 as *const libc::c_char,
    );
    luaM_growvector!(
        fs -> ls -> L, f -> upvalues, fs -> nups, f -> sizeupvalues, Upvaldesc, MAXUPVAL,
        "upvalues"
    );
    while oldsize < (*f).sizeupvalues {
        let fresh160 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh161 = (*((*f).upvalues).offset(fresh160 as isize)).name;
        *fresh161 = NULL as *mut TString;
    }
    (*((*f).upvalues).offset((*fs).nups as isize)).instack =
        ((*v).k as libc::c_uint == VLOCAL as libc::c_int as libc::c_uint) as libc::c_int as lu_byte;
    (*((*f).upvalues).offset((*fs).nups as isize)).idx = cast_byte!(v -> u.info);
    let ref mut fresh162 = (*((*f).upvalues).offset((*fs).nups as isize)).name;
    *fresh162 = name;
    if luaC_objbarrier!(fs -> ls -> L, f, name) != 0 {
    } else {
    };
    let fresh163 = (*fs).nups;
    (*fs).nups = ((*fs).nups).wrapping_add(1);
    return fresh163 as libc::c_int;
}
