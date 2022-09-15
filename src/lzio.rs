/*
** Buffered streams
*/

use std::ptr;

use libc::{c_char, c_int, c_uchar, c_void, memcpy, size_t};

use crate::lmem::luaM_realloc_;
use crate::lstate::lua_State;
use crate::types::lua_Reader;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut c_char,
    pub n: size_t,
    pub buffsize: size_t,
}

impl Mbuffer {
    pub const fn new() -> Mbuffer {
        Mbuffer {
            buffer: ptr::null_mut(),
            n: 0,
            buffsize: 0,
        }
    }
}

pub const EOZ: c_int = -1; /* end of stream */

pub unsafe fn zgetc(z: *mut Zio) -> c_int {
    let n = (*z).n;
    (*z).n = ((*z).n).wrapping_sub(1);
    return if n > 0 {
        let p = (*z).p;
        (*z).p = ((*z).p).offset(1);
        return *p as libc::c_uchar as libc::c_int // This cast through uchar is ESSENTIAL :)
    } else {
        luaZ_fill(z)
    };
}

pub unsafe fn luaZ_resizebuffer(L: *mut lua_State, buff: *mut Mbuffer, size: usize) {
    (*buff).buffer =
        luaM_realloc_(L, (*buff).buffer as *mut c_void, (*buff).buffsize, size) as *mut c_char;
    (*buff).buffsize = size;
}

pub unsafe fn luaZ_freebuffer(L: *mut lua_State, buff: *mut Mbuffer) {
    luaZ_resizebuffer(L, buff, 0)
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

#[no_mangle]
pub unsafe extern "C" fn luaZ_fill(z: *mut ZIO) -> c_int {
    let mut size: size_t = 0;
    let L = (*z).L;
    let buff = ((*z).reader).expect("non-null function pointer")(L, (*z).data, &mut size);
    if buff.is_null() || size == 0 {
        return EOZ;
    }
    (*z).n = size.wrapping_sub(1); /* discount char being returned */
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

#[inline(always)]
pub unsafe fn luaZ_buffer(buff: *mut Mbuffer) -> *mut c_char {
    return (*buff).buffer;
}

#[inline(always)]
pub unsafe fn luaZ_buffremove(buff: *mut Mbuffer, i: size_t) {
    (*buff).n -= i;
}

#[inline(always)]
pub unsafe fn luaZ_sizebuffer(buff: *mut Mbuffer) -> size_t {
    return (*buff).buffsize;
}

#[inline(always)]
pub unsafe fn luaZ_bufflen(buff: *mut Mbuffer) -> size_t {
    return (*buff).n;
}

#[inline(always)]
pub unsafe fn luaZ_resetbuffer(buff: *mut Mbuffer) {
    (*buff).n = 0;
}
