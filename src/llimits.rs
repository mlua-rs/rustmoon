use libc::{c_int, c_uchar, c_uint, ptrdiff_t, size_t};

pub type lu_byte = c_uchar;
pub type l_mem = ptrdiff_t;
pub type lu_mem = size_t;
pub type sig_atomic_t = c_int;

/*
** type for virtual-machine instructions;
** must be an unsigned with (at least) 4 bytes (see details in lopcodes.h)
*/
pub type Instruction = c_uint;

/*
** Size of cache for strings in the API. 'N' is the number of
** sets (better be a prime) and "M" is the size of each set (M == 1
** makes a direct cache.)
*/
pub const STRCACHE_N: usize = 53;
pub const STRCACHE_M: usize = 2;

/*
@@ LUA_IDSIZE gives the maximum size for the description of the source
@@ of a function in debug information.
** CHANGE it if you want a different size.
*/
pub const LUA_IDSIZE: usize = 60;
