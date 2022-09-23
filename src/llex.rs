/*
** Lexical Analyzer
*/

use std::ptr;

use libc::{c_char, c_int, c_ulong, isalpha, isdigit, isspace, isxdigit, size_t, INT_MAX};

use crate::ldebug::luaG_addinfo;
use crate::ldo::luaD_throw;
use crate::lgc::{luaC_checkGC, luaC_fix};
use crate::llimits::{lu_byte, LUA_MINBUFFER};
use crate::lobject::{
    fltvalue, ivalue, luaO_hexavalue, luaO_pushfstring, luaO_str2num, luaO_utf8esc, setbvalue,
    setsvalue, tsvalue, ttisfloat, ttisinteger, ttisnil, GCObject, TString, TValue, Table, Value,
    UTF8BUFFSZ,
};
use crate::lparser::{Dyndata, FuncState};
use crate::lstate::lua_State;
use crate::lstring::{isreserved, luaS_new, luaS_newliteral, luaS_newlstr};
use crate::ltable::{keyfromval, luaH_set};
use crate::lzio::{
    luaZ_buffer, luaZ_bufflen, luaZ_buffremove, luaZ_resetbuffer, luaZ_resizebuffer,
    luaZ_sizebuffer, zgetc, Mbuffer, EOZ, ZIO,
};
use crate::types::{lua_Integer, lua_Number, LUA_ERRSYNTAX};

/* maximum value for size_t */
pub const MAX_SIZET: size_t = !(0 as c_int as size_t);

/* maximum size visible for Lua (must be representable in a lua_Integer */
pub const MAX_SIZE: lua_Integer = 9223372036854775807; // FIXME
pub const __SCHAR_MAX__: c_int = 127 as c_int;
pub const UCHAR_MAX: c_int = __SCHAR_MAX__ * 2 as c_int + 1 as c_int;
pub const LUA_ENV: *const c_char = cstr!("_ENV");
pub const FIRST_RESERVED: c_int = 257;
pub const TK_WHILE: c_int = 278;
pub const NUM_RESERVED: c_int = TK_WHILE - FIRST_RESERVED + 1;
pub const TK_EOS: c_int = 289;
pub const TK_STRING: c_int = 293;
pub const TK_INT: c_int = 291;
pub const TK_FLT: c_int = 290;
pub const TK_NAME: c_int = 292;
pub const TK_LE: c_int = 284;
pub const TK_EQ: c_int = 282;
pub const TK_CONCAT: c_int = 280;
pub const TK_DOTS: c_int = 281;
pub const TK_IDIV: c_int = 279;
pub const TK_GE: c_int = 283;
pub const TK_SHR: c_int = 287;
pub const TK_SHL: c_int = 286;
pub const TK_DBCOLON: c_int = 288;
pub const TK_NE: c_int = 285;
pub const TK_RETURN: c_int = 274;
pub const TK_FUNCTION: c_int = 265;
pub const TK_END: c_int = 262;
pub const TK_GOTO: c_int = 266;
pub const TK_DO: c_int = 259;
pub const TK_UNTIL: c_int = 277;
pub const TK_REPEAT: c_int = 273;
pub const TK_IN: c_int = 268;
pub const TK_FOR: c_int = 264;
pub const TK_THEN: c_int = 275;
pub const TK_BREAK: c_int = 258;
pub const TK_ELSEIF: c_int = 261;
pub const TK_ELSE: c_int = 260;
pub const TK_IF: c_int = 267;

/* semantics information */
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    pub r: lua_Number,
    pub i: lua_Integer,
    pub ts: *mut TString,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: c_int,
    pub seminfo: SemInfo,
}

/* state of the lexer plus state of the parser when shared by all functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: c_int,     // current character (charint)
    pub linenumber: c_int,  // input line counterp
    pub lastline: c_int,    // line of last token 'consumed'
    pub t: Token,           // current token
    pub lookahead: Token,   // look ahead token
    pub fs: *mut FuncState, // current function (parser)
    pub L: *mut lua_State,
    pub z: *mut ZIO,          // input stream
    pub buff: *mut Mbuffer,   // buffer for tokens
    pub h: *mut Table,        // to avoid collection/reuse strings
    pub dyd: *mut Dyndata,    // dynamic structures used by the parser
    pub source: *mut TString, // current source name
    pub envn: *mut TString,   // environment variable name
}

/* ORDER RESERVED */
static mut luaX_tokens: [*const c_char; 37] = [
    cstr!("and"),
    cstr!("break"),
    cstr!("do"),
    cstr!("else"),
    cstr!("elseif"),
    cstr!("end"),
    cstr!("false"),
    cstr!("for"),
    cstr!("function"),
    cstr!("goto"),
    cstr!("if"),
    cstr!("in"),
    cstr!("local"),
    cstr!("nil"),
    cstr!("not"),
    cstr!("or"),
    cstr!("repeat"),
    cstr!("return"),
    cstr!("then"),
    cstr!("true"),
    cstr!("until"),
    cstr!("while"),
    cstr!("//"),
    cstr!(".."),
    cstr!("..."),
    cstr!("=="),
    cstr!(">="),
    cstr!("<="),
    cstr!("~="),
    cstr!("<<"),
    cstr!(">>"),
    cstr!("::"),
    cstr!("<eof>"),
    cstr!("<number>"),
    cstr!("<integer>"),
    cstr!("<name>"),
    cstr!("<string>"),
];

#[inline(always)]
unsafe fn next(ls: *mut LexState) {
    (*ls).current = zgetc((*ls).z);
}

#[inline(always)]
unsafe fn save_and_next(ls: *mut LexState) {
    save(ls, (*ls).current);
    next(ls);
}

unsafe fn save(ls: *mut LexState, c: c_int) {
    let b = (*ls).buff;
    let currentSize: size_t = luaZ_sizebuffer(b);
    let currentLen: size_t = luaZ_bufflen(b);
    if currentLen.wrapping_add(1) > currentSize {
        if currentSize >= (MAX_SIZE as size_t / 2) {
            lexerror(ls, cstr!("lexical element too long"), 0);
        }
        luaZ_resizebuffer((*ls).L, b, currentSize.wrapping_mul(2));
    }
    *((*b).buffer).offset(currentLen as isize) = c as c_char;
    (*b).n = (*b).n + 1;
}

#[no_mangle]
pub unsafe extern "C" fn luaX_init(L: *mut lua_State) {
    let mut i: c_int = 0;
    let e = luaS_newliteral(L, LUA_ENV);
    luaC_fix(L, obj2gco!(e));
    while i < NUM_RESERVED {
        let mut ts = luaS_new(L, luaX_tokens[i as usize]);
        luaC_fix(L, obj2gco!(ts));
        (*ts).extra = i as lu_byte + 1;
        i += 1;
    }
}

/*
** creates a new string and anchors it in scanner's table so that
** it will not be collected until the end of the compilation
** (by that time it should be anchored somewhere)
*/
#[no_mangle]
pub unsafe extern "C" fn luaX_newstring(
    ls: *mut LexState,
    str: *const c_char,
    l: size_t,
) -> *mut TString {
    let mut L = (*ls).L;
    let mut ts = luaS_newlstr(L, str, l); /* create new string */
    setsvalue(L, (*L).top, ts); /* temporarily anchor it in stack */
    (*L).top = (*L).top.offset(1);
    let o = luaH_set(/* entry for 'str' */ L, (*ls).h, (*L).top.offset(-1));
    if ttisnil(o) {
        /* not in use yet? */
        /* boolean value does not need GC barrier;
        table has no metatable, so it does not need to invalidate cache */
        setbvalue(o, true); /* t[string] = true */
        luaC_checkGC(L);
    } else {
        /* string already present */
        ts = tsvalue(keyfromval(o)); /* re-use value previously stored */
    }
    (*L).top = (*L).top.offset(-1); /* remove string from stack */
    return ts;
}

#[inline(always)]
pub unsafe fn currIsNewline(ls: *mut LexState) -> bool {
    if (*ls).current == '\n' as i32 || (*ls).current == '\r' as i32 {
        return true;
    }
    return false;
}
/*
** increment line number and skips newline sequence (any of
** \n, \r, \n\r, or \r\n)
*/
unsafe fn inclinenumber(mut ls: *mut LexState) {
    let old = (*ls).current;
    debug_assert!(currIsNewline(ls));
    next(ls); /* skip '\n' or '\r' */
    if currIsNewline(ls) && (*ls).current != old {
        next(ls); /* skip '\n\r' or '\r\n' */
    }
    (*ls).linenumber = (*ls).linenumber + 1;
    if (*ls).linenumber >= INT_MAX {
        lexerror(ls, cstr!("chunk has too many lines"), 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaX_setinput(
    L: *mut lua_State,
    mut ls: *mut LexState,
    z: *mut ZIO,
    source: *mut TString,
    firstchar: c_int,
) {
    (*ls).t.token = 0;
    (*ls).L = L;
    (*ls).current = firstchar;
    (*ls).lookahead.token = TK_EOS;
    (*ls).z = z;
    (*ls).fs = ptr::null_mut();
    (*ls).linenumber = 1;
    (*ls).lastline = 1;
    (*ls).source = source;
    (*ls).envn = luaS_newliteral(L, LUA_ENV); /* get env name */
    luaZ_resizebuffer((*ls).L, (*ls).buff, LUA_MINBUFFER); /* initialize buffer */
}

/*
** =======================================================
** LEXICAL ANALYZER
** =======================================================
*/

unsafe fn check_next1(ls: *mut LexState, c: c_int) -> c_int {
    if (*ls).current == c {
        next(ls);
        return 1;
    }
    return 0;
}

/*
** Check whether current char is in set 'set' (with two chars) and
** saves it
*/
unsafe fn check_next2(ls: *mut LexState, set: *const c_char) -> c_int {
    if (*ls).current == *set.offset(0) as c_int || (*ls).current == *set.offset(1) as c_int {
        save_and_next(ls);
        return 1;
    }
    return 0;
}

/* LUA_NUMBER */
/*
** this function is quite liberal in what it accepts, as 'luaO_str2num'
** will reject ill-formed numerals.
*/

unsafe fn read_numeral(ls: *mut LexState, mut seminfo: *mut SemInfo) -> c_int {
    let mut obj = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut expo = cstr!("Ee");
    let first = (*ls).current;
    debug_assert!(isdigit((*ls).current) != 0);
    save_and_next(ls);
    if first == '0' as i32 && check_next2(ls, cstr!("xX")) != 0 {
        expo = cstr!("Pp");
    }
    loop {
        if check_next2(ls, expo) != 0 {
            /* exponent part? */
            check_next2(ls, cstr!("-+")); /* optional exponent sign */
        }
        if isxdigit((*ls).current) != 0 {
            save_and_next(ls);
        } else {
            if (*ls).current != '.' as i32 {
                break;
            }
            save_and_next(ls);
        }
    }
    save(ls, '\0' as i32);
    if luaO_str2num(luaZ_buffer((*ls).buff), &mut obj) == 0 {
        lexerror(ls, cstr!("malformed number"), TK_FLT);
    }
    if ttisinteger(&obj) {
        (*seminfo).i = ivalue(&obj);
        return TK_INT;
    } else {
        debug_assert!(ttisfloat(&obj));
        (*seminfo).r = fltvalue(&obj);
        return TK_FLT;
    };
}

/*
** reads a sequence '[=*[' or ']=*]', leaving the last bracket.
** If sequence is well formed, return its number of '='s + 2; otherwise,
** return 1 if there is no '='s or 0 otherwise (an unfinished '[==...').
*/

unsafe fn skip_sep(ls: *mut LexState) -> size_t {
    let mut count: size_t = 0;
    let s = (*ls).current;
    debug_assert!(s == '[' as i32 || s == ']' as i32);
    save_and_next(ls);
    while (*ls).current == '=' as i32 {
        save_and_next(ls);
        count = count.wrapping_add(1);
    }
    return if (*ls).current == s {
        count.wrapping_add(2)
    } else {
        if count == 0 {
            1
        } else {
            0
        }
    };
}

unsafe fn read_long_string(ls: *mut LexState, mut seminfo: *mut SemInfo, sep: size_t) {
    let line = (*ls).linenumber; /* initial line (for error message) */
    save_and_next(ls); /* skip 2nd '[' */
    if currIsNewline(ls) {
        /* string starts with a newline? */
        inclinenumber(ls); /* skip it */
    }
    loop {
        match (*ls).current {
            EOZ => {
                /* error */
                let what = if !seminfo.is_null() {
                    cstr!("string")
                } else {
                    cstr!("comment")
                };
                let msg = luaO_pushfstring(
                    (*ls).L,
                    cstr!("unfinished long %s (starting at line %d)"),
                    what,
                    line,
                );
                lexerror(ls, msg, TK_EOS);
            }
            93 => {
                // ]
                let ss = skip_sep(ls);
                if ss == sep {
                    save_and_next(ls); /* skip 2nd ']' */
                    break;
                }
            }
            10 | 13 => {
                // '\n' | '\r\
                save(ls, '\n' as i32);
                inclinenumber(ls);
                if seminfo.is_null() {
                    luaZ_resetbuffer((*ls).buff); /* avoid wasting space */
                }
            }
            _ => {
                if !seminfo.is_null() {
                    save_and_next(ls);
                } else {
                    next(ls);
                }
            }
        }
    }
    if !seminfo.is_null() {
        (*seminfo).ts = luaX_newstring(
            ls,
            luaZ_buffer((*ls).buff).offset(sep as isize),
            luaZ_bufflen((*ls).buff).wrapping_sub(2 * sep),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaX_token2str(ls: *mut LexState, token: c_int) -> *const c_char {
    if token < FIRST_RESERVED {
        return luaO_pushfstring((*ls).L, cstr!("'%c'"), token);
    } else {
        let s = luaX_tokens[(token - FIRST_RESERVED) as usize];
        if token < TK_EOS as c_int {
            return luaO_pushfstring((*ls).L, cstr!("'%s'"), s);
        } else {
            return s;
        }
    };
}

unsafe fn txtToken(ls: *mut LexState, token: c_int) -> *const c_char {
    match token {
        292 | 293 | 290 | 291 => {
            save(ls, '\0' as i32);
            return luaO_pushfstring((*ls).L, cstr!("'%s'"), luaZ_buffer((*ls).buff));
        }
        _ => return luaX_token2str(ls, token),
    };
}

unsafe fn lexerror(ls: *mut LexState, mut msg: *const c_char, token: c_int) {
    msg = luaG_addinfo((*ls).L, msg, (*ls).source, (*ls).linenumber);
    if token != 0 {
        luaO_pushfstring((*ls).L, cstr!("%s near %s"), msg, txtToken(ls, token));
    }
    luaD_throw((*ls).L, LUA_ERRSYNTAX);
}

#[no_mangle]
pub unsafe extern "C" fn luaX_syntaxerror(ls: *mut LexState, msg: *const c_char) {
    lexerror(ls, msg, (*ls).t.token);
}

unsafe fn esccheck(ls: *mut LexState, c: c_int, msg: *const c_char) {
    if c == 0 {
        if (*ls).current != EOZ {
            save_and_next(ls); /* add current to buffer for error message */
        }
        lexerror(ls, msg, TK_STRING as c_int);
    }
}

unsafe fn gethexa(ls: *mut LexState) -> c_int {
    save_and_next(ls);
    esccheck(
        ls,
        isxdigit((*ls).current),
        cstr!("hexadecimal digit expected"),
    );
    return luaO_hexavalue((*ls).current);
}

unsafe fn readhexaesc(ls: *mut LexState) -> c_int {
    let mut r = gethexa(ls);
    r = (r << 4 as c_int) + gethexa(ls);
    luaZ_buffremove((*ls).buff, 2);
    return r;
}

unsafe fn readutf8esc(ls: *mut LexState) -> c_ulong {
    let mut r: c_ulong;
    let mut i: c_int = 4; /* chars to be removed: '\', 'u', '{', and first digit */
    save_and_next(ls); /* skip 'u' */
    esccheck(
        ls,
        ((*ls).current == '{' as i32) as c_int,
        cstr!("missing '{'"),
    );
    r = gethexa(ls) as c_ulong; /* must have at least one digit */
    save_and_next(ls);
    while isxdigit((*ls).current) != 0 {
        i += 1;
        r = (r << 4).wrapping_add(luaO_hexavalue((*ls).current) as c_ulong);
        esccheck(
            ls,
            (r <= 0x10ffff as c_ulong) as c_int,
            cstr!("UTF-8 value too large"),
        );
        save_and_next(ls);
    }
    esccheck(
        ls,
        ((*ls).current == '}' as i32) as c_int,
        cstr!("missing '}'"),
    );
    next(ls); /* skip '}' */
    luaZ_buffremove((*ls).buff, i as size_t); /* remove saved chars from buffer */
    return r;
}

unsafe fn utf8esc(ls: *mut LexState) {
    let mut buff: [c_char; 8] = [0; 8];
    let mut n: c_int = luaO_utf8esc(buff.as_mut_ptr(), readutf8esc(ls));
    while n > 0 {
        save(ls, buff[(UTF8BUFFSZ - n as usize) as usize] as c_int);
        n -= 1;
    }
}

unsafe fn readdecesc(ls: *mut LexState) -> c_int {
    let mut i: c_int;
    let mut r = 0 as c_int;
    i = 0 as c_int;
    while i < 3 as c_int && isdigit((*ls).current) != 0 {
        r = 10 as c_int * r + (*ls).current - '0' as i32;
        save_and_next(ls);
        i += 1;
    }
    esccheck(
        ls,
        (r <= UCHAR_MAX) as c_int,
        cstr!("decimal escape too large"),
    );
    luaZ_buffremove((*ls).buff, i as size_t); /* remove read digits from buffer */
    return r;
}

unsafe fn read_string(ls: *mut LexState, del: c_int, mut seminfo: *mut SemInfo) {
    save_and_next(ls); /* keep delimiter (for error messages) */
    while (*ls).current != del {
        let mut read_save: bool = false;
        match (*ls).current {
            EOZ => {
                lexerror(ls, cstr!("unfinished string"), TK_EOS as c_int);
                break;
            }
            10 | 13 => {
                // \n \r
                lexerror(ls, cstr!("unfinished string"), TK_STRING as c_int);
                break;
            }
            92 => {
                /* \ - escape sequences */
                let c: c_int;
                save_and_next(ls); /* keep '\\' for error messages */
                match (*ls).current {
                    97 => {
                        // \a
                        c = '\u{7}' as i32;
                        read_save = true;
                    }
                    98 => {
                        // \b
                        c = '\u{8}' as i32;
                        read_save = true;
                    }
                    102 => {
                        // \f
                        c = '\u{c}' as i32;
                        read_save = true;
                    }
                    110 => {
                        // \n
                        c = '\n' as i32;
                        read_save = true;
                    }
                    114 => {
                        // \r
                        c = '\r' as i32;
                        read_save = true;
                    }
                    116 => {
                        // \t
                        c = '\t' as i32;
                        read_save = true;
                    }
                    118 => {
                        // \v
                        c = '\u{b}' as i32;
                        read_save = true;
                    }
                    120 => {
                        // \x
                        c = readhexaesc(ls);
                        read_save = true;
                    }
                    117 => {
                        // \u
                        utf8esc(ls);
                        continue;
                    }
                    10 | 13 => {
                        // \n \r
                        inclinenumber(ls);
                        c = '\n' as i32;
                    }
                    92 | 34 | 39 => {
                        // \\ \" \'
                        c = (*ls).current;
                        read_save = true;
                    }
                    EOZ => {
                        continue;
                    }
                    122 => {
                        // \z - zap following span of spaces
                        luaZ_buffremove((*ls).buff, 1); /* remove '\\' */
                        next(ls); /* skip the 'z' */
                        while isspace((*ls).current) != 0 {
                            if currIsNewline(ls) {
                                inclinenumber(ls);
                            } else {
                                next(ls);
                            }
                        }
                        continue;
                    }
                    _ => {
                        esccheck(
                            ls,
                            isdigit((*ls).current),
                            b"invalid escape sequence\0" as *const u8 as *const c_char,
                        );
                        c = readdecesc(ls); /* digital escape '\ddd' */
                    }
                }
                if read_save {
                    next(ls);
                }
                luaZ_buffremove((*ls).buff, 1); /* remove '\\' */
                save(ls, c);
            }
            _ => {
                save_and_next(ls);
            }
        }
    }
    save_and_next(ls); /* skip delimiter */
    (*seminfo).ts = luaX_newstring(
        ls,
        luaZ_buffer((*ls).buff).offset(1),
        luaZ_bufflen((*ls).buff).wrapping_sub(2),
    );
}

unsafe fn lislalpha(c: c_int) -> bool {
    if isalpha(c) != 0 {
        return true;
    }
    if c == '_' as i32 {
        return true;
    }
    return false;
}

unsafe fn llex(ls: *mut LexState, mut seminfo: *mut SemInfo) -> c_int {
    luaZ_resetbuffer((*ls).buff);
    loop {
        match (*ls).current {
            10 | 13 => {
                /* line breaks \n and \r */
                inclinenumber(ls);
            }
            32 | 12 | 9 | 11 => {
                // ' ' \f \t \v - spaces
                next(ls);
            }
            45 => {
                /* '-' or '--' (comment) */
                next(ls);
                if (*ls).current != '-' as i32 {
                    return '-' as i32;
                }
                next(ls);
                let mut isLongComment = false;
                if (*ls).current == '[' as i32 {
                    let sep = skip_sep(ls);
                    luaZ_resetbuffer((*ls).buff); /* 'skip_sep' may dirty the buffer */
                    if sep >= 2 {
                        read_long_string(ls, ptr::null_mut() as *mut SemInfo, sep); /* skip long comment */
                        luaZ_resetbuffer((*ls).buff); /* previous call may dirty the buff. */
                        isLongComment = true;
                    }
                }
                /* else short comment */
                if !isLongComment {
                    while !currIsNewline(ls) && (*ls).current != EOZ {
                        next(ls); /* skip until end of line (or end of file) */
                    }
                }
            }
            91 => {
                /* [ - long string or simply '[' */
                let sep_0 = skip_sep(ls);
                if sep_0 >= 2 {
                    read_long_string(ls, seminfo, sep_0);
                    return TK_STRING as c_int;
                } else {
                    if sep_0 == 0 {
                        lexerror(ls, cstr!("invalid long string delimiter"), TK_STRING);
                    }
                }
                return '[' as i32;
            }
            61 => {
                // =
                next(ls);
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_EQ as c_int;
                } else {
                    return '=' as i32;
                }
            }
            60 => {
                // <
                next(ls);
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_LE as c_int;
                } else if check_next1(ls, '<' as i32) != 0 {
                    return TK_SHL as c_int;
                } else {
                    return '<' as i32;
                }
            }
            62 => {
                // >
                next(ls);
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_GE as c_int;
                } else if check_next1(ls, '>' as i32) != 0 {
                    return TK_SHR as c_int;
                } else {
                    return '>' as i32;
                }
            }
            47 => {
                // /
                next(ls);
                if check_next1(ls, '/' as i32) != 0 {
                    return TK_IDIV as c_int;
                } else {
                    return '/' as i32;
                }
            }
            126 => {
                // ~
                next(ls);
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_NE as c_int;
                } else {
                    return '~' as i32;
                }
            }
            58 => {
                // :
                next(ls);
                if check_next1(ls, ':' as i32) != 0 {
                    return TK_DBCOLON as c_int;
                } else {
                    return ':' as i32;
                }
            }
            34 | 39 => {
                // " or ' - short literal strings
                read_string(ls, (*ls).current, seminfo);
                return TK_STRING as c_int;
            }
            46 => {
                /* . - '.', '..', '...', or number */
                save_and_next(ls);
                if check_next1(ls, '.' as i32) != 0 {
                    if check_next1(ls, '.' as i32) != 0 {
                        return TK_DOTS as c_int;
                    } else {
                        return TK_CONCAT as c_int;
                    }
                } else if isdigit((*ls).current) == 0 {
                    return '.' as i32;
                } else {
                    return read_numeral(ls, seminfo);
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                // 0-9
                return read_numeral(ls, seminfo);
            }
            EOZ => return TK_EOS as c_int,
            _ => {
                if lislalpha((*ls).current) {
                    /* identifier or reserved word? */
                    let ts;
                    loop {
                        save_and_next(ls);
                        if isdigit((*ls).current) == 0 && !lislalpha((*ls).current) {
                            break;
                        }
                    }
                    ts = luaX_newstring(ls, luaZ_buffer((*ls).buff), luaZ_bufflen((*ls).buff));
                    (*seminfo).ts = ts;
                    if isreserved(ts) {
                        /* reserved word? */
                        return (*ts).extra as c_int - 1 as c_int + FIRST_RESERVED;
                    } else {
                        return TK_NAME as c_int;
                    }
                } else {
                    /* single-char tokens (+ - / ...) */
                    let c = (*ls).current;
                    next(ls);
                    return c;
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaX_next(mut ls: *mut LexState) {
    (*ls).lastline = (*ls).linenumber;
    if (*ls).lookahead.token != TK_EOS as c_int {
        (*ls).t = (*ls).lookahead;
        (*ls).lookahead.token = TK_EOS as c_int;
    } else {
        (*ls).t.token = llex(ls, &mut (*ls).t.seminfo);
    };
}

#[no_mangle]
pub unsafe extern "C" fn luaX_lookahead(mut ls: *mut LexState) -> c_int {
    (*ls).lookahead.token = llex(ls, &mut (*ls).lookahead.seminfo);
    return (*ls).lookahead.token;
}
