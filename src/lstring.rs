/*
** String table (keep all strings handled by Lua)
*/

use std::mem::size_of;
use std::ptr;

use libc::{c_char, c_int, c_uint, memcmp, memcpy, size_t, strcmp, strlen};

use crate::lgc::{changewhite, isdead, luaC_fix, luaC_newobj};
use crate::llimits::{lu_byte, LUAI_MAXSHORTLEN, MINSTRTABSIZE, STRCACHE_M, STRCACHE_N};
use crate::lmem::luaM_reallocvector;
use crate::lobject::{
    getstr, luaO_nilobject_, setuservalue, TString, UTString, UUdata, Udata, LUA_TLNGSTR,
    LUA_TSHRSTR,
};
use crate::lstate::{gco2ts, gco2u, global_State, lua_State};
use crate::types::LUA_TUSERDATA;

pub const fn sizelstring(l: usize) -> usize {
    size_of::<UTString>() + l + 1
}

pub const fn sizeludata(l: usize) -> usize {
    size_of::<UUdata>() + l
}

pub unsafe fn sizeudata(u: *const Udata) -> usize {
    sizeludata((*u).len)
}

pub unsafe fn luaS_newliteral(L: *mut lua_State, s: *const c_char) -> *mut TString {
    luaS_newlstr(L, s, strlen(s))
}

/*
** test whether a string is a reserved word
*/
#[inline(always)]
pub unsafe fn isreserved(s: *const TString) -> bool {
    (*s).tt as c_int == LUA_TSHRSTR && (*s).extra > 0
}

/*
** equality for short strings, which are always internalized
*/
pub unsafe fn eqshrstr(a: *const TString, b: *const TString) -> bool {
    debug_assert!((*a).tt == LUA_TSHRSTR as u8 && (*b).tt == LUA_TSHRSTR as u8);
    a == b
}

pub const MEMERRMSG: *const c_char = cstr!("not enough memory");

/*
** Lua will use at most ~(2^LUAI_HASHLIMIT) bytes from a string to
** compute its hash
*/
const LUAI_HASHLIMIT: usize = 5;

/*
** equality for long strings
*/
#[no_mangle]
pub unsafe extern "C" fn luaS_eqlngstr(a: *mut TString, b: *mut TString) -> c_int {
    let len = (*a).u.lnglen;
    debug_assert!((*a).tt as c_int == LUA_TLNGSTR && (*b).tt as c_int == LUA_TLNGSTR);
    (a == b || /* same instance or... */
        len == (*b).u.lnglen /* equal length and ... */
        && memcmp(getstr(a) as _, getstr(b) as _, len) == 0) as c_int /* equal contents */
}

#[no_mangle]
pub unsafe extern "C" fn luaS_hash(str: *const c_char, mut l: size_t, seed: c_uint) -> c_uint {
    let mut h = seed ^ (l as c_uint);
    let step = (l >> LUAI_HASHLIMIT) + 1;
    while l >= step {
        h ^= (h << 5)
            .wrapping_add(h >> 2)
            .wrapping_add(*str.offset(l as isize - 1) as c_uint);
        l -= step;
    }
    return h;
}

#[no_mangle]
pub unsafe extern "C" fn luaS_hashlongstr(ts: *mut TString) -> c_uint {
    debug_assert!((*ts).tt as c_int == LUA_TLNGSTR);
    if (*ts).extra == 0 {
        /* no hash? */
        (*ts).hash = luaS_hash(getstr(ts), (*ts).u.lnglen, (*ts).hash);
        (*ts).extra = 1; /* now it has its hash */
    }
    return (*ts).hash;
}

/*
** resizes the string table
*/
#[no_mangle]
pub unsafe extern "C" fn luaS_resize(L: *mut lua_State, newsize: c_int) {
    let tb = &mut (*(*L).l_G).strt;
    if newsize > (*tb).size {
        /* grow table if needed */
        luaM_reallocvector::<*mut TString>(
            L,
            &mut (*tb).hash,
            (*tb).size as usize,
            newsize as usize,
        );
        for i in (*tb).size..newsize {
            *((*tb).hash).offset(i as isize) = ptr::null_mut();
        }
    }
    /* rehash */
    for i in 0..(*tb).size {
        let mut p = *((*tb).hash).offset(i as isize);
        *((*tb).hash).offset(i as isize) = ptr::null_mut();
        while !p.is_null() {
            /* for each node in the list */
            let hnext = (*p).u.hnext; /* save next */
            let h = lmod!((*p).hash, newsize as u32);
            (*p).u.hnext = *((*tb).hash).offset(h as isize); /* new position */
            *((*tb).hash).offset(h as isize) = p; /* chain it */
            p = hnext;
        }
    }
    if newsize < (*tb).size {
        /* shrink table if needed */
        /* vanishing slice should be empty */
        debug_assert!(
            (*(*tb).hash.offset(newsize as isize)).is_null()
                && (*(*tb).hash.offset((*tb).size as isize - 1)).is_null()
        );
        luaM_reallocvector::<*mut TString>(
            L,
            &mut (*tb).hash,
            (*tb).size as usize,
            newsize as usize,
        );
    }
    (*tb).size = newsize;
}

/*
** Clear API string cache. (Entries cannot be empty, so fill them with
** a non-collectable string.)
*/
#[no_mangle]
pub unsafe extern "C" fn luaS_clearcache(g: *mut global_State) {
    for i in 0..STRCACHE_N {
        for j in 0..STRCACHE_M {
            /* will entry be collected? */
            if iswhite!((*g).strcache[i][j]) {
                (*g).strcache[i][j] = (*g).memerrmsg; /* replace it with something fixed */
            }
        }
    }
}

/*
** Initialize the string table and the string cache
*/
#[no_mangle]
pub unsafe extern "C" fn luaS_init(L: *mut lua_State) {
    let g = (*L).l_G;
    luaS_resize(L, MINSTRTABSIZE as c_int); /* initial size of string table */
    /* pre-create memory-error message */
    (*g).memerrmsg = luaS_newliteral(L, MEMERRMSG);
    luaC_fix(L, obj2gco!((*g).memerrmsg)); /* it should never be collected */
    /* fill cache with valid strings */
    for i in 0..STRCACHE_N {
        for j in 0..STRCACHE_M {
            (*g).strcache[i][j] = (*g).memerrmsg;
        }
    }
}

/*
** creates a new string object
*/
unsafe extern "C" fn createstrobj(
    L: *mut lua_State,
    l: size_t,
    tag: c_int,
    h: c_uint,
) -> *mut TString {
    let totalsize = sizelstring(l); /* total size of TString object */
    let o = luaC_newobj(L, tag, totalsize);
    let ts = gco2ts(o);
    (*ts).hash = h;
    (*ts).extra = 0;
    *getstr(ts).offset(l as isize) = '\0' as i32 as c_char; /* ending 0 */
    return ts;
}

#[no_mangle]
pub unsafe extern "C" fn luaS_createlngstrobj(L: *mut lua_State, l: size_t) -> *mut TString {
    let mut ts = createstrobj(L, l, LUA_TLNGSTR, (*(*L).l_G).seed);
    (*ts).u.lnglen = l;
    return ts;
}

#[no_mangle]
pub unsafe extern "C" fn luaS_remove(L: *mut lua_State, ts: *mut TString) {
    let tb = &mut (*(*L).l_G).strt;
    let mut p = &mut *((*tb).hash).offset(lmod!((*ts).hash, (*tb).size as u32) as isize)
        as *mut *mut TString;
    while *p != ts {
        /* find previous element */
        p = &mut (**p).u.hnext;
    }
    *p = (**p).u.hnext; /* remove element from its list */
    (*tb).nuse -= 1;
}

/*
** checks whether short string exists and reuses it or creates a new one
*/
unsafe extern "C" fn internshrstr(
    L: *mut lua_State,
    str: *const c_char,
    l: size_t,
) -> *mut TString {
    let g = (*L).l_G;
    let h = luaS_hash(str, l, (*g).seed);
    let mut list = &mut *((*g).strt.hash).offset(lmod!(h, (*g).strt.size as u32) as isize)
        as *mut *mut TString;
    debug_assert!(!str.is_null()); /* otherwise 'memcmp'/'memcpy' are undefined */
    let mut ts = *list;
    while !ts.is_null() {
        if l == (*ts).shrlen as size_t && memcmp(str as _, getstr(ts) as _, l) == 0 {
            /* found! */
            if isdead(g, obj2gco!(ts)) {
                /* dead (but not collected yet)? */
                changewhite(obj2gco!(ts)); /* resurrect it */
            }
            return ts;
        }
        ts = (*ts).u.hnext;
    }
    if (*g).strt.nuse >= (*g).strt.size && (*g).strt.size <= c_int::MAX / 2 {
        luaS_resize(L, (*g).strt.size * 2);
        /* recompute with new size */
        list = &mut *((*g).strt.hash).offset(lmod!(h, (*g).strt.size as u32) as isize);
    }
    ts = createstrobj(L, l, LUA_TSHRSTR, h);
    memcpy(getstr(ts) as _, str as _, l);
    (*ts).shrlen = l as lu_byte;
    (*ts).u.hnext = *list;
    *list = ts;
    (*g).strt.nuse += 1;
    return ts;
}

/*
** new string (with explicit length)
*/
#[no_mangle]
pub unsafe extern "C" fn luaS_newlstr(
    L: *mut lua_State,
    str: *const c_char,
    l: size_t,
) -> *mut TString {
    if l <= LUAI_MAXSHORTLEN {
        /* short string? */
        return internshrstr(L, str, l);
    } else {
        // if (l >= (MAX_SIZE - sizeof(TString))/sizeof(char))
        //     luaM_toobig(L);
        let ts = luaS_createlngstrobj(L, l);
        memcpy(getstr(ts) as _, str as _, l);
        return ts;
    };
}

/*
** Create or reuse a zero-terminated string, first checking in the
** cache (using the string address as a key). The cache can contain
** only zero-terminated strings, so it is safe to use 'strcmp' to
** check hits.
*/
#[no_mangle]
pub unsafe extern "C" fn luaS_new(L: *mut lua_State, str: *const c_char) -> *mut TString {
    let i = (str as usize % STRCACHE_N) as c_uint; /* hash */
    let p = ((*(*L).l_G).strcache[i as usize]).as_mut_ptr();
    for j in 0..STRCACHE_M {
        if strcmp(str, getstr(*p.add(j))) == 0 {
            /* hit? */
            return *p.offset(j as isize); /* that is it */
        }
    }
    /* normal route */
    let mut j = STRCACHE_M - 1;
    while j > 0 {
        *p.add(j) = *p.add(j - 1); /* move out last element */
        j -= 1;
    }
    /* new element is first in the list */
    *p = luaS_newlstr(L, str, strlen(str));
    return *p;
}

#[no_mangle]
pub unsafe extern "C" fn luaS_newudata(L: *mut lua_State, s: size_t) -> *mut Udata {
    // if (s > MAX_SIZE - sizeof(Udata))
    //     luaM_toobig(L);
    let o = luaC_newobj(L, LUA_TUSERDATA, sizeludata(s));
    let u = gco2u(o);
    (*u).len = s;
    (*u).metatable = ptr::null_mut();
    setuservalue(L, u, &luaO_nilobject_);
    return u;
}
