/*
** Lexical Analyzer
*/

use libc::{c_char, c_int};

use crate::lobject::{TString, Table};
use crate::lparser::{Dyndata, FuncState};
use crate::lstate::lua_State;
use crate::lzio::{Mbuffer, ZIO};
use crate::types::{lua_Integer, lua_Number};

pub const LUA_ENV: *const c_char = cstr!("_ENV");

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

extern "C" {
    pub fn luaX_init(L: *mut lua_State);
}
