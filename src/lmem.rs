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

pub const MINSIZEARRAY: libc::c_int = 4 as libc::c_int;

pub unsafe fn luaM_growaux_(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut size: *mut libc::c_int,
    mut size_elems: size_t,
    mut limit: libc::c_int,
    mut what: *const libc::c_char,
) -> *mut libc::c_void {
    let mut newblock = 0 as *mut libc::c_void;
    let mut newsize: libc::c_int = 0;
    if *size >= limit / 2 as libc::c_int {
        /* cannot double it? */
        if *size >= limit {
            /* cannot grow even a little? */
            luaG_runerror(
                L,
                b"too many %s (limit is %d)\0" as *const u8 as *const libc::c_char,
                what,
                limit,
            );
        }
        newsize = limit;
    } else {
        newsize = *size * 2 as libc::c_int;
        if newsize < MINSIZEARRAY {
            /* minimum size */
            newsize = MINSIZEARRAY;
        }
    }
    newblock = luaM_realloc_(
        L,
        block,
        (*size as libc::c_ulong).checked_mul(size_elems),
        (newsize as libc::c_ulong).checked_mul(size_elems),
    );
    /* update only when everything else is OK */
    *size = newsize;
    return newblock;
}

/*
** generic allocation routine.
*/

pub unsafe fn luaM_realloc_(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut libc::c_void {
    let mut newblock = 0 as *mut libc::c_void;
    let mut g = (*L).l_G;
    let mut realosize = if !block.is_null() {
        osize
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    // TODO: HARDMEMTESTS
    newblock = (((*g).frealloc).expect("non-null function pointer"))((*g).ud, block, osize, nsize);
    if newblock.is_null() && nsize > 0 as libc::c_int as libc::c_ulong {
        /* cannot fail when shrinking a block */
        debug_assert!(nsize > realosize);
        if !((*g).version).is_null() {
            /* is state fully built? */
            /* try to free some memory... */
            luaC_fullgc(L, 1 as libc::c_int);
            /* try again */
            newblock =
                (((*g).frealloc).expect("non-null function pointer"))((*g).ud, block, osize, nsize);
        }
        if newblock.is_null() {
            luaD_throw(L, LUA_ERRMEM);
        }
    }
    debug_assert!((nsize == 0) == newblock.is_null());
    (*g).GCdebt = ((*g).GCdebt as libc::c_ulong)
        .wrapping_add(nsize)
        .wrapping_sub(realosize) as l_mem;
    return newblock;
}
