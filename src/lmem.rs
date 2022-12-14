use std::mem::size_of;
use std::ptr;

use libc::{c_char, c_int, c_void, size_t};

use crate::ldebug::luaG_runerror;
use crate::ldo::luaD_throw;
use crate::lgc::luaC_fullgc;
use crate::llimits::l_mem;
use crate::lstate::lua_State;
use crate::types::LUA_ERRMEM;

/*
** Arrays of chars do not need any test
*/

#[inline(always)]
pub unsafe fn luaM_freemem(L: *mut lua_State, b: *mut c_void, s: usize) {
    luaM_realloc_(L, b, s, 0);
}

#[inline(always)]
pub unsafe fn luaM_free<T>(L: *mut lua_State, b: *mut T) {
    luaM_realloc_(L, b as *mut c_void, size_of::<T>(), 0);
}

#[inline(always)]
pub unsafe fn luaM_freearray<T>(L: *mut lua_State, b: *mut T, n: usize) {
    luaM_realloc_(L, b as *mut c_void, n * size_of::<T>(), 0);
}

#[inline(always)]
pub unsafe fn luaM_new<T>(L: *mut lua_State) -> *mut T {
    luaM_realloc_(L, ptr::null_mut(), 0, size_of::<T>()) as *mut T
}

#[inline(always)]
pub unsafe fn luaM_newvector<T>(L: *mut lua_State, n: usize) -> *mut T {
    luaM_realloc_(L, ptr::null_mut(), 0, n * size_of::<T>()) as *mut T
}

#[inline(always)]
pub unsafe fn luaM_newobject<T>(L: *mut lua_State, tag: u8) -> *mut T {
    luaM_realloc_(L, ptr::null_mut(), tag as usize, size_of::<T>()) as *mut T
}

pub unsafe fn luaM_growvector<T>(
    L: *mut lua_State,
    v: *mut *mut T,
    nelems: c_int,
    size: *mut i32,
    limit: c_int,
    what: *const c_char,
) {
    if nelems + 1 as libc::c_int > (*size) {
        *v = luaM_growaux_(
            L,
            (*v) as *mut c_void,
            size,
            ::std::mem::size_of::<T>() as usize,
            limit,
            what,
        ) as *mut T;
    }
}

#[inline(always)]
pub unsafe fn luaM_newobject_sz(L: *mut lua_State, tag: u8, sz: usize) -> *mut c_void {
    luaM_realloc_(L, ptr::null_mut(), tag as usize, sz)
}

#[inline(always)]
pub unsafe fn luaM_reallocvector<T>(L: *mut lua_State, v: &mut *mut T, oldn: usize, n: usize) {
    *v = luaM_realloc_(
        L,
        *v as *mut c_void,
        oldn * size_of::<T>(),
        n * size_of::<T>(),
    ) as *mut T;
}

/*
** About the realloc function:
** void * frealloc (void *ud, void *ptr, size_t osize, size_t nsize);
** ('osize' is the old size, 'nsize' is the new size)
**
** * frealloc(ud, NULL, x, s) creates a new block of size 's' (no
** matter 'x').
**
** * frealloc(ud, p, x, 0) frees the block 'p'
** (in this specific case, frealloc must return NULL);
** particularly, frealloc(ud, NULL, 0, 0) does nothing
** (which is equivalent to free(NULL) in ISO C)
**
** frealloc returns NULL if it cannot create or reallocate the area
** (any reallocation to an equal or smaller size cannot fail!)
*/

pub const MINSIZEARRAY: c_int = 4;

#[no_mangle]
pub unsafe extern "C" fn luaM_growaux_(
    L: *mut lua_State,
    block: *mut c_void,
    size: *mut c_int,
    size_elems: size_t,
    limit: c_int,
    what: *const c_char,
) -> *mut c_void {
    let mut newsize;
    if *size >= limit / 2 {
        /* cannot double it? */
        if *size >= limit {
            /* cannot grow even a little? */
            luaG_runerror(L, cstr!("too many %s (limit is %d)"), what, limit);
        }
        newsize = limit; /* still have at least one free place */
    } else {
        newsize = *size * 2;
        if newsize < MINSIZEARRAY {
            newsize = MINSIZEARRAY; /* minimum size */
        }
    }
    let newblock = luaM_realloc_(
        L,
        block,
        (*size as size_t) * size_elems,
        (newsize as size_t) * size_elems,
    );
    *size = newsize; /* update only when everything else is OK */
    return newblock;
}

#[no_mangle]
pub unsafe extern "C" fn luaM_toobig(L: *mut lua_State) -> ! {
    luaG_runerror(L, cstr!("memory allocation error: block too big"));
}

/*
** generic allocation routine.
*/
#[no_mangle]
pub unsafe extern "C" fn luaM_realloc_(
    L: *mut lua_State,
    block: *mut c_void,
    osize: size_t,
    nsize: size_t,
) -> *mut c_void {
    let g = (*L).l_G;
    let realosize = if !block.is_null() { osize } else { 0 };
    debug_assert!((realosize == 0) == block.is_null());
    // TODO: HARDMEMTESTS
    let mut newblock =
        (((*g).frealloc).expect("non-null function pointer"))((*g).ud, block, osize, nsize);
    if newblock.is_null() && nsize > 0 {
        debug_assert!(nsize > realosize); /* cannot fail when shrinking a block */
        if !((*g).version).is_null() {
            /* is state fully built? */
            /* try to free some memory... */
            luaC_fullgc(L, 1);
            /* try again */
            newblock =
                (((*g).frealloc).expect("non-null function pointer"))((*g).ud, block, osize, nsize);
        }
        if newblock.is_null() {
            luaD_throw(L, LUA_ERRMEM);
        }
    }
    debug_assert!((nsize == 0) == newblock.is_null());
    (*g).GCdebt = ((*g).GCdebt + nsize as l_mem) - realosize as l_mem;
    return newblock;
}
