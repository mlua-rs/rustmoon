/*
** Lua tables (hash)
*/

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem::{self, size_of};
use std::ptr;

use libc::{c_int, c_uint, c_void};

use crate::ldebug::luaG_runerror;
use crate::ldo::{luaD_rawrunprotected, luaD_throw};
use crate::lgc::{luaC_barrierback, luaC_newobj};
use crate::lmem::{luaM_free, luaM_freearray, luaM_newvector, luaM_reallocvector};
use crate::lobject::{
    bvalue, deadvalue, fltvalue, fvalue, gcvalue, iscollectable, ivalue, luaO_ceillog2,
    luaO_nilobject_, pvalue, setivalue, setnilvalue, setnodekey, setobj, setsvalue, sizenode,
    tsvalue, ttisdeadkey, ttisfloat, ttisinteger, ttisnil, ttisshrstring, ttype, C2RustUnnamed_6,
    Node, StkId, TKey, TString, TValue, Table, Value, LUA_TLCF, LUA_TLNGSTR, LUA_TNUMFLT,
    LUA_TNUMINT, LUA_TSHRSTR,
};
use crate::lstate::{gco2t, lua_State};
use crate::lstring::{eqshrstr, luaS_hashlongstr};
use crate::lvm::{luaV_rawequalobj, luaV_tointeger};
use crate::types::{
    lua_Integer, lua_Number, lua_Unsigned, LUA_ERRMEM, LUA_OK, LUA_TBOOLEAN, LUA_TLIGHTUSERDATA,
    LUA_TNIL, LUA_TTABLE,
};

/*
** Implementation of tables (aka arrays, objects, or hash tables).
** Tables keep its elements in two parts: an array part and a hash part.
** Non-negative integer keys are all candidates to be kept in the array
** part. The actual size of the array is the largest 'n' such that
** more than half the slots between 1 and n are in use.
** Hash uses a mix of chained scatter table with Brent's variation.
** A main invariant of these tables is that, if an element is not
** in its main position (i.e. the 'original' position that its hash gives
** to it), then the colliding element is in its own main position.
** Hence even when the load factor reaches 100%, performance remains good.
*/

pub unsafe fn gnode(t: *const Table, i: usize) -> *mut Node {
    (*t).node.add(i)
}

pub unsafe fn gval(n: *mut Node) -> *mut TValue {
    &mut (*n).i_val
}

pub unsafe fn gnext(n: *const Node) -> i32 {
    (*n).i_key.nk.next
}

pub unsafe fn setgnext(n: *mut Node, x: i32) {
    (*n).i_key.nk.next = x;
}

/* 'const' to avoid wrong writings that can mess up field 'next' */
pub unsafe fn gkey(n: *const Node) -> *const TValue {
    &(*n).i_key.tvk
}

pub unsafe fn invalidateTMcache(t: *mut Table) {
    (*t).flags = 0;
}

/* true when 't' is using 'dummynode' as its hash part */
pub unsafe fn isdummy(t: *const Table) -> bool {
    (*t).lastfree.is_null()
}

/* allocated size for hash nodes */
pub unsafe fn allocsizenode(t: *const Table) -> usize {
    if isdummy(t) {
        0
    } else {
        sizenode(t)
    }
}

/* returns the key, given the value of a table entry */
// #define keyfromval(v) \
//   (gkey(cast(Node *, cast(char *, (v)) - offsetof(Node, i_val))))

/*
** Maximum size of array part (MAXASIZE) is 2^MAXABITS. MAXABITS is
** the largest integer such that MAXASIZE fits in an unsigned int.
*/
pub const MAXABITS: u64 = (size_of::<c_int>() * 8 - 1) as u64;
pub const MAXASIZE: u64 = 1 << MAXABITS;

/*
** Maximum size of hash part is 2^MAXHBITS. MAXHBITS is the largest
** integer such that 2^MAXHBITS fits in a signed int. (Note that the
** maximum number of elements in a table, 2^MAXABITS + 2^MAXHBITS, still
** fits comfortably in an unsigned int.)
*/
pub const MAXHBITS: u64 = MAXABITS - 1;

// pub unsafe fn hashpow2(t: *mut Table, n: c_int) -> *mut Node {
//     gnode(t, lmod!(n, sizenode(t)) as usize)
// }

pub unsafe fn hashstr(t: *const Table, s: *const TString) -> *mut Node {
    gnode(t, lmod!((*s).hash, sizenode(t) as u32) as usize)
}

pub unsafe fn hashboolean(t: *const Table, b: bool) -> *mut Node {
    gnode(t, lmod!(b as u32, sizenode(t) as u32) as usize)
}

pub unsafe fn hashint(t: *const Table, i: lua_Integer) -> *mut Node {
    gnode(t, lmod!(i, sizenode(t) as lua_Integer) as usize)
}

/*
** for some types, it is better to avoid modulus by power of 2, as
** they tend to have many 2 factors.
*/
pub unsafe fn hashmod(t: *const Table, n: u32) -> *mut Node {
    gnode(t, n as usize % ((sizenode(t) - 1) | 1))
}

pub unsafe fn hashpointer(t: *const Table, p: *const c_void) -> *mut Node {
    gnode(t, p as usize % ((sizenode(t) - 1) | 1))
}

static mut dummynode_: Node = Node {
    i_val: TValue {
        value_: Value {
            gc: ptr::null_mut(),
        },
        tt_: LUA_TNIL,
    },
    i_key: TKey {
        nk: C2RustUnnamed_6 {
            value_: Value {
                gc: ptr::null_mut(),
            },
            tt_: LUA_TNIL,
            next: 0,
        },
    },
};

/*
** Hash for floating-point numbers.
*/
unsafe extern "C" fn l_hashfloat(n: lua_Number) -> u32 {
    #[derive(Hash)]
    struct Distance((u64, i16, i8));

    const fn integer_decode(val: f64) -> (u64, i16, i8) {
        let bits: u64 = unsafe { mem::transmute(val) };
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };

        exponent -= 1023 + 52;
        (mantissa, exponent, sign)
    }

    let mut s = DefaultHasher::new();
    Distance(integer_decode(n)).hash(&mut s);
    (s.finish() & u32::MAX as u64) as u32
}

/*
** returns the 'main' position of an element in a table (that is, the index
** of its hash value)
*/
unsafe extern "C" fn mainposition(t: *const Table, key: *const TValue) -> *mut Node {
    match ttype(key) {
        LUA_TNUMINT => hashint(t, ivalue(key)),
        LUA_TNUMFLT => hashmod(t, l_hashfloat(fltvalue(key))),
        LUA_TSHRSTR => hashstr(t, tsvalue(key)),
        LUA_TLNGSTR => gnode(
            t,
            lmod!(luaS_hashlongstr(tsvalue(key)), sizenode(t) as u32) as usize,
        ),
        LUA_TBOOLEAN => hashboolean(t, bvalue(key)),
        LUA_TLIGHTUSERDATA => hashpointer(t, pvalue(key)),
        LUA_TLCF => hashpointer(
            t,
            fvalue(key)
                .map(|p| p as *const c_void)
                .unwrap_or(ptr::null()),
        ),
        _ => {
            debug_assert!(!ttisdeadkey(key));
            hashpointer(t, gcvalue(key) as *const c_void)
        }
    }
}

/*
** returns the index for 'key' if 'key' is an appropriate key to live in
** the array part of the table, 0 otherwise.
*/
unsafe extern "C" fn arrayindex(key: *const TValue) -> c_uint {
    if ttisinteger(key) {
        let k = ivalue(key);
        if 0 < k && k as lua_Unsigned <= MAXASIZE {
            /* 'key' is an appropriate array index */
            return k as c_uint;
        }
    }
    /* 'key' did not match some condition */
    return 0;
}

/*
** returns the index of a 'key' for table traversals. First goes all
** elements in the array part, then elements in the hash part. The
** beginning of a traversal is signaled by 0.
*/
unsafe extern "C" fn findindex(L: *mut lua_State, t: *mut Table, key: StkId) -> c_uint {
    if ttisnil(key) {
        /* first iteration */
        return 0;
    }

    let mut i = arrayindex(key as *const TValue);
    /* is 'key' inside array part? */
    if i != 0 && i <= (*t).sizearray {
        /* yes; that's the index */
        return i;
    }

    let mut nx;
    let mut n = mainposition(t, key);
    loop {
        /* check whether 'key' is somewhere in the chain */
        /* key may be dead already, but it is ok to use it in 'next' */
        if luaV_rawequalobj(gkey(n), key) != 0
            || (ttisdeadkey(gkey(n))
                && iscollectable(key)
                && deadvalue(gkey(n)) == gcvalue(key) as *mut c_void)
        {
            let offset = n.offset_from(gnode(t, 0));
            debug_assert!(offset >= 0); // extra check
            i = offset as c_uint; /* key index in hash table */
            /* hash elements are numbered after array ones */
            return (i + 1) + (*t).sizearray;
        }
        nx = gnext(n);
        if nx == 0 {
            /* key not found */
            luaG_runerror(L, cstr!("invalid key to 'next'"));
        } else {
            n = n.offset(nx as isize);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaH_next(L: *mut lua_State, t: *mut Table, key: StkId) -> c_int {
    /* find original element */
    let mut i = findindex(L, t, key);
    /* try first array part */
    while i < (*t).sizearray {
        /* a non-nil value? */
        if !ttisnil((*t).array.add(i as usize)) {
            setivalue(key, (i + 1) as lua_Integer);
            setobj(L, key.add(1), (*t).array.add(i as usize));
            return 1;
        }
        i += 1;
    }
    /* hash part */
    i -= (*t).sizearray;
    while (i as c_int as usize) < sizenode(t) {
        /* a non-nil value? */
        if !ttisnil(gval(gnode(t, i as usize))) {
            setobj(L, key, gkey(gnode(t, i as usize)));
            setobj(L, key.add(1), gval(gnode(t, i as usize)));
            return 1;
        }
        i += 1;
    }
    /* no more elements */
    return 0;
}

/*
**
** Rehash
**
*/

/*
** Compute the optimal size for the array part of table 't'. 'nums' is a
** "count array" where 'nums[i]' is the number of integers in the table
** between 2^(i - 1) + 1 and 2^i. 'pna' enters with the total number of
** integer keys in the table and leaves with the number of keys that
** will go to the array part; return the optimal size.
*/

unsafe extern "C" fn computesizes(nums: *mut c_uint, pna: *mut c_uint) -> c_uint {
    let mut a: c_uint = 0; /* number of elements smaller than 2^i */
    let mut na: c_uint = 0; /* number of elements to go to array part */
    let mut optimal: c_uint = 0; /* optimal size for array part */
    /* loop while keys can fill more than half of total size */
    let mut i = 0;
    let mut twotoi = 1; /* 2^i (candidate for optimal size) */
    while twotoi > 0 && *pna > twotoi / 2 {
        if *nums.add(i) > 0 {
            a += *nums.add(i);
            /* more than half elements present? */
            if a > twotoi / 2 {
                /* optimal size (till now) */
                optimal = twotoi;
                /* all elements up to 'optimal' will go to array part */
                na = a;
            }
        }
        i += 1;
        twotoi *= 2;
    }
    debug_assert!((optimal == 0 || optimal / 2 < na) && na <= optimal);
    *pna = na;
    return optimal;
}

unsafe extern "C" fn countint(key: *const TValue, nums: *mut c_uint) -> c_int {
    let k = arrayindex(key);
    /* is 'key' an appropriate array index? */
    if k != 0 {
        *nums.add(luaO_ceillog2(k) as usize) += 1; /* count as such */
        return 1;
    } else {
        return 0;
    }
}

/*
** Count keys in array part of table 't': Fill 'nums[i]' with
** number of keys that will go into corresponding slice and return
** total number of non-nil keys.
*/
unsafe extern "C" fn numusearray(t: *const Table, nums: *mut c_uint) -> c_uint {
    let mut lg = 0;
    let mut ttlg: c_uint = 1; /* 2^lg */
    let mut ause: c_uint = 0; /* summation of 'nums' */
    let mut i: c_uint = 1; /* count to traverse all array keys */
    while lg <= MAXABITS {
        let mut lc = 0; /* counter */
        let mut lim = ttlg;
        if lim > (*t).sizearray {
            lim = (*t).sizearray; /* adjust upper limit */
            if i > lim {
                break; /* no more elements to count */
            }
        }
        /* count elements in range (2^(lg - 1), 2^lg] */
        while i <= lim {
            if !ttisnil((*t).array.add((i - 1) as usize)) {
                lc += 1;
            }
            i += 1;
        }
        *nums.add(lg as usize) += lc;
        ause += lc;
        lg += 1;
        ttlg *= 2;
    }
    return ause;
}

unsafe extern "C" fn numusehash(t: *const Table, nums: *mut c_uint, pna: *mut c_uint) -> c_int {
    let mut totaluse = 0; /* total number of elements */
    let mut ause = 0; /* elements added to 'nums' (can go to array part) */
    let mut i = sizenode(t);
    while i != 0 {
        i -= 1;
        let n: *mut Node = ((*t).node).add(i);
        if !ttisnil(gval(n)) {
            ause += countint(gkey(n), nums);
            totaluse += 1;
        }
    }
    *pna += ause as c_uint;
    return totaluse;
}

unsafe extern "C" fn setarrayvector(L: *mut lua_State, t: *mut Table, size: c_uint) {
    luaM_reallocvector::<TValue>(L, &mut (*t).array, (*t).sizearray as usize, size as usize);
    for i in (*t).sizearray..size {
        setnilvalue((*t).array.add(i as usize));
    }
    (*t).sizearray = size;
}

unsafe extern "C" fn setnodevector(L: *mut lua_State, t: *mut Table, mut size: c_uint) {
    if size == 0 {
        /* no elements to hash part? */
        (*t).node = &mut dummynode_; /* use common 'dummynode' */
        (*t).lsizenode = 0;
        (*t).lastfree = ptr::null_mut(); /* signal that it is using dummy node */
    } else {
        let lsize = luaO_ceillog2(size);
        if lsize as u64 > MAXHBITS {
            luaG_runerror(L, cstr!("table overflow"));
        }
        size = 1 << lsize; // 2^lsize
        (*t).node = luaM_newvector::<Node>(L, size as usize);
        for i in 0..size as usize {
            let n: *mut Node = gnode(t, i);
            setgnext(n, 0);
            (*n).i_key.nk.tt_ = LUA_TNIL; // setnilvalue(wgkey(n));
            setnilvalue(gval(n));
        }
        (*t).lsizenode = lsize as u8;
        (*t).lastfree = gnode(t, size as usize); /* all positions are free */
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuxsetnodeT {
    pub t: *mut Table,
    pub nhsize: c_uint,
}

unsafe extern "C" fn auxsetnode(L: *mut lua_State, ud: *mut c_void) {
    let asn = ud as *mut AuxsetnodeT;
    setnodevector(L, (*asn).t, (*asn).nhsize);
}

#[no_mangle]
pub unsafe extern "C" fn luaH_resize(
    L: *mut lua_State,
    t: *mut Table,
    nasize: c_uint,
    nhsize: c_uint,
) {
    let mut asn = AuxsetnodeT {
        t: ptr::null_mut(),
        nhsize: 0,
    };
    let oldasize = (*t).sizearray;
    let oldhsize = allocsizenode(t);
    let nold = (*t).node; /* save old hash ... */
    if nasize > oldasize {
        /* array part must grow? */
        setarrayvector(L, t, nasize);
    }
    /* create new hash part with appropriate size */
    asn.t = t;
    asn.nhsize = nhsize;
    if luaD_rawrunprotected(
        L,
        Some(auxsetnode),
        &mut asn as *mut AuxsetnodeT as *mut c_void,
    ) != LUA_OK
    {
        /* mem. error? */
        setarrayvector(L, t, oldasize); /* array back to its original size */
        luaD_throw(L, LUA_ERRMEM); /* rethrow memory error */
    }
    if nasize < oldasize {
        /* array part must shrink? */
        (*t).sizearray = nasize;
        /* re-insert elements from vanishing slice */
        for i in nasize..oldasize {
            if !ttisnil((*t).array.add(i as usize)) {
                luaH_setint(
                    L,
                    t,
                    i as lua_Integer + 1,
                    &mut *((*t).array).add(i as usize),
                );
            }
        }
        /* shrink array */
        luaM_reallocvector::<TValue>(L, &mut (*t).array, oldasize as usize, nasize as usize);
    }
    /* re-insert elements from hash part */
    let mut j = oldhsize as c_int - 1;
    while j >= 0 {
        let old = nold.add(j as usize);
        if !ttisnil(gval(old)) {
            /* doesn't need barrier/invalidate cache, as entry was already present in the table */
            setobj(L, luaH_set(L, t, gkey(old)), gval(old));
        }
        j -= 1;
    }
    if oldhsize > 0 {
        /* not the dummy node? */
        luaM_freearray(L, nold, oldhsize as usize); /* free old hash */
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaH_resizearray(L: *mut lua_State, t: *mut Table, nasize: c_uint) {
    let nsize = allocsizenode(t);
    luaH_resize(L, t, nasize, nsize as c_uint);
}

/*
** nums[i] = number of keys 'k' where 2^(i - 1) < k <= 2^i
*/
unsafe extern "C" fn rehash(L: *mut lua_State, t: *mut Table, ek: *const TValue) {
    let asize: c_uint; /* optimal size for array part */
    let mut na: c_uint; /* number of keys in the array part */
    let mut nums = [0 as c_uint; (MAXABITS + 1) as usize];
    /* reset counts */
    for i in 0..=MAXABITS {
        nums[i as usize] = 0;
    }
    na = numusearray(t, nums.as_mut_ptr()); /* count keys in array part */
    /* all those keys are integer keys */
    let mut totaluse = na;
    /* count keys in hash part */
    totaluse += numusehash(t, nums.as_mut_ptr(), &mut na) as u32;
    /* count extra key */
    na += countint(ek, nums.as_mut_ptr()) as u32;
    totaluse += 1;
    /* compute new size for array part */
    asize = computesizes(nums.as_mut_ptr(), &mut na);
    /* resize the table to new computed sizes */
    luaH_resize(L, t, asize, totaluse - na);
}

#[no_mangle]
pub unsafe extern "C" fn luaH_new(L: *mut lua_State) -> *mut Table {
    let o = luaC_newobj(L, LUA_TTABLE, size_of::<Table>());
    let mut t: *mut Table = gco2t(o);
    (*t).metatable = ptr::null_mut();
    (*t).flags = !0;
    (*t).array = ptr::null_mut();
    (*t).sizearray = 0;
    setnodevector(L, t, 0);
    return t;
}

#[no_mangle]
pub unsafe extern "C" fn luaH_free(L: *mut lua_State, t: *mut Table) {
    if !isdummy(t) {
        luaM_freearray(L, (*t).node, sizenode(t));
    }
    luaM_freearray(L, (*t).array, (*t).sizearray as usize);
    luaM_free(L, t);
}

unsafe extern "C" fn getfreepos(t: *mut Table) -> *mut Node {
    if !isdummy(t) {
        while (*t).lastfree > (*t).node {
            (*t).lastfree = ((*t).lastfree).offset(-1);
            if ttisnil(gkey((*t).lastfree)) {
                return (*t).lastfree;
            }
        }
    }
    /* could not find a free place */
    return ptr::null_mut();
}

/*
** inserts a new key into a hash table; first, check whether key's main
** position is free. If not, check whether colliding node is in its main
** position or not: if it is not, move colliding node to an empty place and
** put new key in its main position; otherwise (colliding node is in its main
** position), new key goes to an empty position.
*/
#[no_mangle]
pub unsafe extern "C" fn luaH_newkey(
    L: *mut lua_State,
    t: *mut Table,
    mut key: *const TValue,
) -> *mut TValue {
    // let mut mp = 0 as *mut Node;
    let mut aux = TValue {
        value_: Value {
            gc: ptr::null_mut(),
        },
        tt_: 0,
    };
    if ttisnil(key) {
        luaG_runerror(L, cstr!("table index is nil"));
    } else if ttisfloat(key) {
        let mut k: lua_Integer = 0;
        if luaV_tointeger(key, &mut k, 0) != 0 {
            /* does index fit in an integer? */
            setivalue(&mut aux, k);
            /* insert it as an integer */
            key = &mut aux;
        } else if fltvalue(key).is_nan() {
            luaG_runerror(L, cstr!("table index is NaN"));
        }
    }
    let mut mp = mainposition(t, key);
    if !ttisnil(gval(mp)) || isdummy(t) {
        /* main position is taken? */
        let f = getfreepos(t); /* get a free place */
        if f.is_null() {
            /* cannot find a free place? */
            /* grow table */
            rehash(L, t, key);
            /* whatever called 'newkey' takes care of TM cache */
            /* insert key into grown table */
            return luaH_set(L, t, key);
        }
        debug_assert!(!isdummy(t));
        let mut othern = mainposition(t, gkey(mp));
        if othern != mp {
            /* is colliding node out of its main position? */
            /* yes; move colliding node into free position */
            while othern.offset(gnext(othern) as isize) != mp {
                /* find previous */
                othern = othern.offset(gnext(othern) as isize);
            }
            /* rechain to point to 'f' */
            setgnext(othern, f.offset_from(othern) as i32);
            /* copy colliding node into free pos. (mp->next also goes) */
            *f = *mp;
            if gnext(mp) != 0 {
                /* correct 'next' */
                setgnext(f, gnext(f) + mp.offset_from(f) as i32);
                /* now 'mp' is free */
                setgnext(mp, 0);
            }
            setnilvalue(gval(mp));
        } else {
            /* colliding node is in its own main position */
            /* new node will go into free position */
            if gnext(mp) != 0 {
                /* chain new position */
                setgnext(f, mp.add(gnext(mp) as usize).offset_from(f) as i32);
            } else {
                debug_assert!(gnext(f) == 0);
            }
            setgnext(mp, f.offset_from(mp) as i32);
            mp = f;
        }
    }
    setnodekey(L, &mut (*mp).i_key, key);
    luaC_barrierback(L, t, key);
    debug_assert!(ttisnil(gval(mp)));
    return gval(mp);
}

/*
** search function for integers
*/
#[no_mangle]
pub unsafe extern "C" fn luaH_getint(t: *mut Table, key: lua_Integer) -> *const TValue {
    /* (1 <= key && key <= t->sizearray) */
    if key > 0 && key - 1 < (*t).sizearray as lua_Integer {
        return &*((*t).array).add((key - 1) as usize);
    } else {
        let mut n = hashint(t, key);
        loop {
            /* check whether 'key' is somewhere in the chain */
            if ttisinteger(gkey(n)) && ivalue(gkey(n)) == key {
                /* that's it */
                return gval(n);
            } else {
                let nx = gnext(n);
                if nx == 0 {
                    break;
                }
                n = n.add(nx as usize);
            }
        }
        return &luaO_nilobject_;
    };
}

/*
** search function for short strings
*/
#[no_mangle]
pub unsafe extern "C" fn luaH_getshortstr(t: *mut Table, key: *mut TString) -> *const TValue {
    let mut n = hashstr(t, key);
    debug_assert!((*key).tt == LUA_TSHRSTR as u8);
    loop {
        /* check whether 'key' is somewhere in the chain */
        let k = gkey(n);
        if ttisshrstring(k) && eqshrstr(tsvalue(k), key) {
            /* that's it */
            return gval(n);
        } else {
            let nx = gnext(n);
            if nx == 0 {
                /* not found */
                return &luaO_nilobject_;
            }
            n = n.add(nx as usize);
        }
    }
}

/*
** "Generic" get version. (Not that generic: not valid for integers,
** which may be in array part, nor for floats with integral values.)
*/
unsafe extern "C" fn getgeneric(t: *mut Table, key: *const TValue) -> *const TValue {
    let mut n = mainposition(t, key);
    loop {
        /* check whether 'key' is somewhere in the chain */
        if luaV_rawequalobj(gkey(n), key) != 0 {
            /* that's it */
            return gval(n);
        } else {
            let nx = gnext(n);
            if nx == 0 {
                /* not found */
                return &luaO_nilobject_;
            }
            n = n.add(nx as usize);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaH_getstr(t: *mut Table, key: *mut TString) -> *const TValue {
    if (*key).tt == LUA_TSHRSTR as u8 {
        return luaH_getshortstr(t, key);
    } else {
        /* for long strings, use generic case */
        let mut ko = TValue {
            value_: Value {
                gc: ptr::null_mut(),
            },
            tt_: 0,
        };
        setsvalue(ptr::null_mut(), &mut ko, key);
        return getgeneric(t, &ko);
    };
}

/*
** main search function
*/
#[no_mangle]
pub unsafe extern "C" fn luaH_get(t: *mut Table, key: *const TValue) -> *const TValue {
    match ttype(key) {
        LUA_TSHRSTR => luaH_getshortstr(t, tsvalue(key)),
        LUA_TNUMINT => luaH_getint(t, ivalue(key)),
        LUA_TNIL => &luaO_nilobject_,
        LUA_TNUMFLT => {
            let mut k = 0;
            if luaV_tointeger(key, &mut k, 0) != 0 {
                /* index is int? */
                /* use specialized version */
                luaH_getint(t, k)
            } else {
                getgeneric(t, key)
            }
        }
        _ => getgeneric(t, key),
    }
}

/*
** beware: when using this function you probably need to check a GC
** barrier and invalidate the TM cache.
*/
#[no_mangle]
pub unsafe extern "C" fn luaH_set(
    L: *mut lua_State,
    t: *mut Table,
    key: *const TValue,
) -> *mut TValue {
    let p = luaH_get(t, key);
    if p != &luaO_nilobject_ {
        p as *mut TValue
    } else {
        luaH_newkey(L, t, key)
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaH_setint(
    L: *mut lua_State,
    t: *mut Table,
    key: lua_Integer,
    value: *mut TValue,
) {
    let p = luaH_getint(t, key);
    let cell;
    if p != &luaO_nilobject_ {
        cell = p as *mut TValue;
    } else {
        let mut k = TValue {
            value_: Value {
                gc: ptr::null_mut(),
            },
            tt_: 0,
        };
        setivalue(&mut k, key);
        cell = luaH_newkey(L, t, &mut k);
    };
    setobj(L, cell, value);
}

unsafe extern "C" fn unbound_search(t: *mut Table, mut j: lua_Unsigned) -> lua_Unsigned {
    let mut i = j; /* i is zero or a present index */
    j += 1;
    /* find 'i' and 'j' such that i is present and j is not */
    while !ttisnil(luaH_getint(t, j as lua_Integer)) {
        i = j;
        if j > (lua_Integer::MAX / 2) as lua_Unsigned {
            /* overflow? */
            /* table was built with bad purposes: resort to linear search */
            i = 1;
            while !ttisnil(luaH_getint(t, i as lua_Integer)) {
                i += 1;
            }
            return i - 1;
        }
        j *= 2;
    }
    /* now do a binary search between them */
    while j - i > 1 {
        let m = (i + j) / 2;
        if ttisnil(luaH_getint(t, m as lua_Integer)) {
            j = m;
        } else {
            i = m;
        }
    }
    return i;
}

/*
** Try to find a boundary in table 't'. A 'boundary' is an integer index
** such that t[i] is non-nil and t[i+1] is nil (and 0 if t[1] is nil).
*/
#[no_mangle]
pub unsafe extern "C" fn luaH_getn(t: *mut Table) -> lua_Unsigned {
    let mut j = (*t).sizearray;
    if j > 0 && ttisnil((*t).array.add((j - 1) as usize)) {
        /* there is a boundary in the array part: (binary) search for it */
        let mut i = 0;
        while (j - i) > 1 {
            let m = (i + j) / 2;
            if ttisnil((*t).array.add((m - 1) as usize)) {
                j = m;
            } else {
                i = m;
            }
        }
        return i as lua_Unsigned;

        /* else must find a boundary in hash part */
    } else if isdummy(t) {
        /* hash part is empty? */
        /* that is easy... */
        return j as lua_Unsigned;
    } else {
        return unbound_search(t, j as lua_Unsigned);
    };
}
