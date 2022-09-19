use libc::{c_double, c_int, c_long, c_uchar, c_uint, c_void, ptrdiff_t, size_t, INT_MAX};

use crate::types::{lua_Integer, lua_Number};

pub type lu_byte = c_uchar;
pub type l_mem = ptrdiff_t;
pub type lu_mem = size_t;
pub type sig_atomic_t = c_int;

/* type to ensure maximum alignment */
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    pub n: lua_Number,
    pub u: c_double,
    pub s: *mut c_void,
    pub i: lua_Integer,
    pub l: c_long,
}

/*
** type for virtual-machine instructions;
** must be an unsigned with (at least) 4 bytes (see details in lopcodes.h)
*/
pub type Instruction = c_uint;

/*
** Maximum length for short strings, that is, strings that are
** internalized. (Cannot be smaller than reserved words or tags for
** metamethods, as these strings must be internalized;
** #("function") = 8, #("__newindex") = 10.)
*/
pub const LUAI_MAXSHORTLEN: usize = 40;

/*
** Initial size for the string table (must be power of 2).
** The Lua core alone registers ~50 strings (reserved words +
** metaevent keys + a few others). Libraries would typically add
** a few dozens more.
*/
pub const MINSTRTABSIZE: usize = 128;

/*
** Size of cache for strings in the API. 'N' is the number of
** sets (better be a prime) and "M" is the size of each set (M == 1
** makes a direct cache.)
*/
pub const STRCACHE_N: usize = 53;
pub const STRCACHE_M: usize = 2;

/* minimum size for string buffer */
pub const LUA_MINBUFFER: usize = 32;

/*
@@ LUA_IDSIZE gives the maximum size for the description of the source
@@ of a function in debug information.
** CHANGE it if you want a different size.
*/
pub const LUA_IDSIZE: usize = 60;

/*
@@ LUAL_BUFFERSIZE is the buffer size used by the lauxlib buffer system.
** CHANGE it if it uses too much C-stack space. (For long double,
** 'string.format("%.99f", -1e4932)' needs 5034 bytes, so a
** smaller buffer would force a memory allocation for each call to
** 'string.format'.)
*/
pub const LUAL_BUFFERSIZE: usize = 8192;

/*
@@ LUAI_MAXSTACK limits the size of the Lua stack.
** CHANGE it if you need a different limit. This limit is arbitrary;
** its only purpose is to stop Lua from consuming unlimited stack
** space (and to reserve some numbers for pseudo-indices).
*/
pub const LUAI_MAXSTACK: usize = 1000000;

/*
** maximum depth for nested C calls and syntactical nested non-terminals
** in a program. (Value must fit in an unsigned short int.)
*/
pub const LUAI_MAXCCALLS: u16 = 200;

pub const MAX_INT: i32 = INT_MAX; 