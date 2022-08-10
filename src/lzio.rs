use std::ptr;

use libc::{c_char, c_int, c_uchar, c_void, memcpy, size_t};

use crate::lstate::lua_State;
use crate::types::lua_Reader;

///
/// Buffered streams
///

pub type ZIO = Zio;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const c_char,
    pub reader: lua_Reader,
    pub data: *mut c_void,
    pub L: *mut lua_State,
}

#[no_mangle]
pub unsafe extern "C" fn luaZ_init(
    L: *mut lua_State,
    z: *mut ZIO,
    reader: lua_Reader,
    data: *mut c_void,
) {
    (*z).L = L;
    (*z).reader = reader;
    (*z).data = data;
    (*z).n = 0;
    (*z).p = ptr::null();
}

pub const EOZ: c_int = -1; /* end of stream */

#[no_mangle]
pub unsafe extern "C" fn luaZ_fill(z: *mut ZIO) -> c_int {
    let mut size: size_t = 0;
    let L = (*z).L;
    let buff = ((*z).reader).expect("non-null function pointer")(L, (*z).data, &mut size);
    if buff.is_null() || size == 0 {
        return EOZ;
    }
    (*z).n = size - 1; /* discount char being returned */
    (*z).p = buff;
    let fresh0 = (*z).p;
    (*z).p = ((*z).p).offset(1);
    return *fresh0 as c_uchar as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn luaZ_read(z: *mut ZIO, mut b: *mut c_void, mut n: size_t) -> size_t {
    while n != 0 {
        if (*z).n == 0 {
            /* no bytes in buffer? */
            /* try to read more */
            if luaZ_fill(z) == EOZ {
                return n; /* no more input; return number of missing bytes */
            } else {
                (*z).n += 1; /* luaZ_fill consumed first byte; put it back */
                (*z).p = ((*z).p).offset(-1);
            }
        }
        let m = if n <= (*z).n { n } else { (*z).n }; /* min. between n and z->n */
        memcpy(b, (*z).p as *const c_void, m);
        (*z).n -= m;
        (*z).p = ((*z).p).offset(m as isize);
        b = (b as *mut c_char).offset(m as isize) as *mut c_void;
        n -= m;
    }
    return 0;
}
