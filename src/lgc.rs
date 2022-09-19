/*
** Garbage Collector
*/

use std::env;
use std::mem;
use std::ptr;

use libc::{c_int, c_void, strchr};

use crate::ldo::{luaD_callnoyield, luaD_pcall, luaD_shrinkstack, luaD_throw, savestack};
use crate::lfunc::{isintwups, luaF_freeproto, sizeCclosure, sizeLclosure, upisopen, UpVal};
use crate::llimits::{lu_byte, Instruction};
use crate::lmem::{luaM_free, luaM_freemem, luaM_newobject_sz};
use crate::lobject::{
    gcvalue, getuservalue, iscollectable, luaO_pushfstring, novariant, righttt, setgcovalue,
    setnilvalue, setobj, sizenode, svalue, tsvalue, ttisdeadkey, ttisfunction, ttisnil, ttisstring,
    CClosure, GCObject, LClosure, LocVar, Node, Proto, TString, TValue, Table, Upvaldesc, LUA_TCCL,
    LUA_TDEADKEY, LUA_TLCL, LUA_TLNGSTR, LUA_TPROTO, LUA_TSHRSTR,
};
use crate::lstate::{
    gco2ccl, gco2lcl, gco2p, gco2t, gco2th, gco2ts, gco2u, gettotalbytes, global_State,
    luaE_freethread, luaE_setdebt, lua_State, CallInfo, CIST_FIN, KGC_EMERGENCY, KGC_NORMAL,
};
use crate::lstring::{luaS_clearcache, luaS_remove, luaS_resize, sizelstring, sizeudata};
use crate::ltable::{allocsizenode, gkey, gnode, gval, luaH_free};
use crate::ltm::{gfasttm, luaT_gettmbyobj, TM_GC, TM_MODE};
use crate::types::{
    LUA_ERRGCMM, LUA_ERRRUN, LUA_NUMTAGS, LUA_OK, LUA_TTABLE, LUA_TTHREAD, LUA_TUSERDATA,
};

/*
** Collectable objects may have one of three colors: white, which
** means the object is not marked; gray, which means the
** object is marked, but its references may be not marked; and
** black, which means that the object and all its references are marked.
** The main invariant of the garbage collector, while marking objects,
** is that a black object can never point to a white one. Moreover,
** any gray object must be in a "gray list" (gray, grayagain, weak,
** allweak, ephemeron) so that it can be visited again before finishing
** the collection cycle. These lists have no meaning when the invariant
** is not being enforced (e.g., sweep phase).
*/

/* how much to allocate before next GC step */
/* ~100 small strings */
pub const GCSTEPSIZE: usize = 100 * mem::size_of::<TString>();

/*
** Possible states of the Garbage Collector
*/
pub const GCSpropagate: u8 = 0;
pub const GCSatomic: u8 = 1;
pub const GCSswpallgc: u8 = 2;
pub const GCSswpfinobj: u8 = 3;
pub const GCSswptobefnz: u8 = 4;
pub const GCSswpend: u8 = 5;
pub const GCScallfin: u8 = 6;
pub const GCSpause: u8 = 7;

/*
** internal state for collector while inside the atomic phase. The
** collector should never be in this state while running regular code.
*/
pub const GCSinsideatomic: u8 = GCSpause + 1;

#[inline(always)]
unsafe fn issweepphase(g: *mut global_State) -> bool {
    GCSswpallgc <= (*g).gcstate && (*g).gcstate <= GCSswpend
}

/*
** macro to tell when main invariant (white objects cannot point to black
** ones) must be kept. During a collection, the sweep
** phase may break the invariant, as objects turned white may point to
** still-black objects. The invariant is restored when sweep ends and
** all objects are white again.
*/
#[inline(always)]
unsafe fn keepinvariant(g: *mut global_State) -> bool {
    (*g).gcstate <= GCSatomic
}

/*
** some useful bit tricks
*/
#[inline(always)]
pub fn resetbits(x: &mut lu_byte, m: lu_byte) {
    *x &= !m
}

#[inline(always)]
pub fn setbits(x: &mut lu_byte, m: lu_byte) {
    *x |= m
}

#[inline(always)]
pub const fn testbits(x: lu_byte, m: lu_byte) -> bool {
    x & m != 0
}

#[inline(always)]
pub const fn bitmask(b: lu_byte) -> lu_byte {
    1 << b
}

#[inline(always)]
pub const fn bit2mask(b1: lu_byte, b2: lu_byte) -> lu_byte {
    bitmask(b1) | bitmask(b2)
}

#[inline(always)]
pub fn l_setbit(x: &mut lu_byte, b: lu_byte) {
    setbits(x, bitmask(b))
}

#[inline(always)]
pub fn resetbit(x: &mut lu_byte, b: lu_byte) {
    resetbits(x, bitmask(b))
}

#[inline(always)]
pub(crate) const fn testbit(x: lu_byte, b: lu_byte) -> bool {
    testbits(x, bitmask(b))
}

/* Layout for bit use in 'marked' field: */
pub const WHITE0BIT: lu_byte = 0; /* object is white (type 0) */
pub const WHITE1BIT: lu_byte = 1; /* object is white (type 1) */
pub const BLACKBIT: lu_byte = 2; /* object is black */
pub const FINALIZEDBIT: lu_byte = 3; /* object has been marked for finalization */
/* bit 7 is currently used by tests (luaL_checkmemory) */

pub const WHITEBITS: lu_byte = bit2mask(WHITE0BIT, WHITE1BIT);

macro_rules! iswhite {
    ($x:expr) => {
        crate::lgc::testbits((*$x).marked, crate::lgc::WHITEBITS)
    };
}

macro_rules! isblack {
    ($x:expr) => {
        crate::lgc::testbit((*$x).marked, crate::lgc::BLACKBIT)
    };
}

macro_rules! isgray {
    ($x:expr) => {
        /* neither white nor black */
        !crate::lgc::testbits(
            (*$x).marked,
            crate::lgc::WHITEBITS | crate::lgc::bitmask(crate::lgc::BLACKBIT),
        )
    };
}

#[inline(always)]
pub unsafe fn tofinalize(x: *mut GCObject) -> bool {
    testbit((*x).marked, FINALIZEDBIT)
}

#[inline(always)]
pub unsafe fn otherwhite(g: *mut global_State) -> lu_byte {
    (*g).currentwhite ^ WHITEBITS
}

#[inline(always)]
pub const fn isdeadm(ow: lu_byte, m: lu_byte) -> bool {
    (m ^ WHITEBITS) & ow == 0
}

#[inline(always)]
pub unsafe fn isdead(g: *mut global_State, v: *mut GCObject) -> bool {
    isdeadm(otherwhite(g), (*v).marked)
}

#[inline(always)]
pub unsafe fn changewhite(x: *mut GCObject) {
    (*x).marked ^= WHITEBITS;
}

#[inline(always)]
pub unsafe fn gray2black(x: *mut GCObject) {
    l_setbit(&mut (*x).marked, BLACKBIT)
}

#[inline(always)]
pub unsafe fn luaC_white(g: *mut global_State) -> lu_byte {
    (*g).currentwhite & WHITEBITS
}

/*
** Does one step of collection when debt becomes positive. 'pre'/'pos'
** allows some adjustments to be done only when needed. macro
** 'condchangemem' is used only for heavy tests (forcing a full
** GC cycle on every opportunity)
*/
#[inline(always)]
pub unsafe fn luaC_condGC(L: *mut lua_State, mut pre: impl FnMut(), mut pos: impl FnMut()) {
    if (*(*L).l_G).GCdebt > 0 {
        pre();
        luaC_step(L);
        pos();
    }
    #[cfg(debug_assertions)]
    if env::var("LUA_HARDMEMTESTS").as_deref() == Ok("1") && (*(*L).l_G).gcrunning != 0 {
        pre();
        luaC_fullgc(L, 0);
        pos();
    }
}

/* more often than not, 'pre'/'pos' are empty */
#[inline(always)]
pub unsafe fn luaC_checkGC(L: *mut lua_State) {
    luaC_condGC(L, || (), || ());
}

#[inline(always)]
pub unsafe fn luaC_barrier(L: *mut lua_State, p: *mut GCObject, v: *const TValue) {
    if iscollectable(v) && isblack!(p) && iswhite!(gcvalue(v)) {
        luaC_barrier_(L, p, gcvalue(v))
    }
}

#[inline(always)]
pub unsafe fn luaC_barrierback(L: *mut lua_State, p: *mut Table, v: *const TValue) {
    if iscollectable(v) && isblack!(p) && iswhite!(gcvalue(v)) {
        luaC_barrierback_(L, p)
    }
}

#[inline(always)]
pub unsafe fn luaC_objbarrier(L: *mut lua_State, p: *mut GCObject, o: *mut GCObject) {
    if isblack!(p) && iswhite!(o) {
        luaC_barrier_(L, p, o);
    }
}

#[inline(always)]
pub unsafe fn luaC_upvalbarrier(L: *mut lua_State, uv: *mut UpVal) {
    if iscollectable((*uv).v) && !upisopen(uv) {
        luaC_upvalbarrier_(L, uv)
    }
}

/*
** cost of sweeping one element (the size of a small object divided
** by some adjust for the sweep speed)
*/
const GCSWEEPCOST: usize = (mem::size_of::<TString>() + 4) / 4;

/* maximum number of elements to sweep in each single step */
const GCSWEEPMAX: usize = (GCSTEPSIZE + GCSWEEPCOST) / 4;

/* cost of calling one finalizer */
const GCFINALIZECOST: usize = GCSWEEPCOST;

/*
** macro to adjust 'stepmul': 'stepmul' is actually used like
** 'stepmul / STEPMULADJ' (value chosen by tests)
*/
const STEPMULADJ: usize = 200;

/*
** macro to adjust 'pause': 'pause' is actually used like
** 'pause / PAUSEADJ' (value chosen by tests)
*/
const PAUSEADJ: usize = 100;

/*
** 'makewhite' erases all color bits then sets only the current white
** bit
*/
const maskcolors: u8 = !(bitmask(BLACKBIT) | WHITEBITS);

#[inline(always)]
unsafe fn makewhite(g: *mut global_State, x: *mut GCObject) {
    (*x).marked = ((*x).marked & maskcolors) | luaC_white(g);
}

#[inline(always)]
// TODO: macro?
unsafe fn white2gray(x: *mut GCObject) {
    resetbits(&mut (*x).marked, WHITEBITS)
}

// TODO: macro?
#[inline(always)]
unsafe fn black2gray(x: *mut GCObject) {
    resetbit(&mut (*x).marked, BLACKBIT)
}

#[inline(always)]
unsafe fn valiswhite(x: *const TValue) -> bool {
    iscollectable(x) && iswhite!(gcvalue(x))
}

#[inline(always)]
unsafe fn checkdeadkey(n: *mut Node) {
    debug_assert!(!ttisdeadkey(gkey(n)) || ttisnil(gval(n)));
}

#[inline(always)]
unsafe fn checkconsistency(obj: *const TValue) {
    debug_assert!(!iscollectable(obj) || righttt(obj));
}

#[inline(always)]
unsafe fn markvalue(g: *mut global_State, o: *const TValue) {
    checkconsistency(o);
    if valiswhite(o) {
        reallymarkobject(g, gcvalue(o));
    }
}

#[inline(always)]
unsafe fn markobject(g: *mut global_State, t: *mut GCObject) {
    if iswhite!(t) {
        reallymarkobject(g, t);
    }
}

/*
** mark an object that can be NULL (either because it is really optional,
** or it was stripped as debug info, or inside an uncompleted structure)
*/
macro_rules! markobjectN {
    ($g:expr, $t:expr) => {
        if !$t.is_null() {
            markobject($g, obj2gco!($t));
        }
    };
}

/*
**
** Generic functions
**
*/

// one after last element in a hash array
#[inline(always)]
unsafe fn gnodelast(h: *const Table) -> *mut Node {
    gnode(h, sizenode(h))
}

// link collectable object 'o' into list pointed by 'p'
macro_rules! linkgclist {
    ($o:expr, $p:expr) => {{
        (*$o).gclist = $p;
        $p = obj2gco!($o);
    }};
}

/*
** If key is not marked, mark its entry as dead. This allows key to be
** collected, but keeps its entry in the table.  A dead node is needed
** when Lua looks up for a key (it may be part of a chain) and when
** traversing a weak table (key might be removed from the table during
** traversal). Other places never manipulate dead keys, because its
** associated nil value is enough to signal that the entry is logically
** empty.
*/
#[inline(always)]
unsafe fn removeentry(n: *mut Node) {
    debug_assert!(ttisnil(gval(n)));
    if valiswhite(gkey(n)) {
        (*n).i_key.nk.tt_ = LUA_TDEADKEY; /* setdeadvalue(wgkey(n)); unused and unmarked key; remove it */
    }
}

/*
** tells whether a key or value can be cleared from a weak
** table. Non-collectable objects are never removed from weak
** tables. Strings behave as 'values', so are never removed too. for
** other objects: if really collected, cannot keep them; for objects
** being finalized, keep them in keys, but not in values
*/
unsafe fn iscleared(g: *mut global_State, o: *const TValue) -> bool {
    if !iscollectable(o) {
        return false;
    }
    if ttisstring(o) {
        markobject(g, obj2gco!(tsvalue(o))); /* strings are 'values', so are never weak */
        return false;
    }
    return iswhite!(gcvalue(o));
}

/*
** barrier that moves collector forward, that is, mark the white object
** being pointed by a black object. (If in sweep phase, clear the black
** object to white [sweep it] to avoid other barrier calls for this
** same object.)
*/
#[no_mangle]
pub unsafe extern "C" fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject) {
    let g = (*L).l_G;
    debug_assert!(isblack!(o) && iswhite!(v) && !isdead(g, v) && !isdead(g, o));
    if keepinvariant(g) {
        /* must keep invariant? */
        reallymarkobject(g, v); /* restore invariant */
    } else {
        /* sweep phase */
        debug_assert!(issweepphase(g));
        makewhite(g, o); /* mark main obj. as white to avoid other barriers */
    }
}

/*
** barrier that moves collector backward, that is, mark the black object
** pointing to a white object as gray again.
*/
#[no_mangle]
pub unsafe extern "C" fn luaC_barrierback_(L: *mut lua_State, t: *mut Table) {
    let g = (*L).l_G;
    debug_assert!(isblack!(t) && !isdead(g, obj2gco!(t)));
    black2gray(obj2gco!(t)); /* make table gray (again) */
    linkgclist!(t, (*g).grayagain);
}

/*
** barrier for assignments to closed upvalues. Because upvalues are
** shared among closures, it is impossible to know the color of all
** closures pointing to it. So, we assume that the object being assigned
** must be marked.
*/
#[no_mangle]
pub unsafe extern "C" fn luaC_upvalbarrier_(L: *mut lua_State, uv: *mut UpVal) {
    let g = (*L).l_G;
    let o = gcvalue((*uv).v);
    debug_assert!(!upisopen(uv)); /* ensured by macro luaC_upvalbarrier */
    if keepinvariant(g) {
        markobject(g, o);
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaC_fix(L: *mut lua_State, o: *mut GCObject) {
    let g = (*L).l_G;
    debug_assert!((*g).allgc == o); /* object must be 1st in 'allgc' list! */
    white2gray(o); /* they will be gray forever */
    (*g).allgc = (*o).next; /* remove object from 'allgc' list */
    (*o).next = (*g).fixedgc; /* link it to 'fixedgc' list */
    (*g).fixedgc = o;
}

/*
** create a new collectable object (with given type and size) and link
** it to 'allgc' list.
*/
#[no_mangle]
pub unsafe extern "C" fn luaC_newobj(L: *mut lua_State, tt: c_int, sz: usize) -> *mut GCObject {
    let g = (*L).l_G;
    let o = luaM_newobject_sz(L, novariant(tt) as u8, sz) as *mut GCObject;
    (*o).marked = luaC_white(g);
    (*o).tt = tt as u8;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    return o;
}

/*
**
** Mark functions
**
*/

/*
** mark an object. Userdata, strings, and closed upvalues are visited
** and turned black here. Other objects are marked gray and added
** to appropriate list to be visited (and turned black) later. (Open
** upvalues are already linked in 'headuv' list.)
*/
unsafe fn reallymarkobject(g: *mut global_State, o: *mut GCObject) {
    white2gray(o);
    match (*o).tt as c_int {
        LUA_TSHRSTR => {
            gray2black(o);
            (*g).GCmemtrav += sizelstring((*gco2ts(o)).shrlen as usize);
        }
        LUA_TLNGSTR => {
            gray2black(o);
            (*g).GCmemtrav += sizelstring((*gco2ts(o)).u.lnglen);
        }
        LUA_TUSERDATA => {
            let mut uvalue = TValue::new();
            markobjectN!(g, (*gco2u(o)).metatable); /* mark its metatable */
            gray2black(o);
            (*g).GCmemtrav += sizeudata(gco2u(o));
            getuservalue((*g).mainthread, gco2u(o), &mut uvalue);
            if valiswhite(&uvalue) {
                /* markvalue(g, &uvalue); */
                return reallymarkobject(g, gcvalue(&uvalue)); // Tail call, should be optimized
            }
        }
        LUA_TLCL => {
            linkgclist!(gco2lcl(o), (*g).gray);
        }
        LUA_TCCL => {
            linkgclist!(gco2ccl(o), (*g).gray);
        }
        LUA_TTABLE => {
            linkgclist!(gco2t(o), (*g).gray);
        }
        LUA_TTHREAD => {
            linkgclist!(gco2th(o), (*g).gray);
        }
        LUA_TPROTO => {
            linkgclist!(gco2p(o), (*g).gray);
        }
        _ => unreachable!(),
    }
}

/*
** mark metamethods for basic types
*/
#[inline(always)]
unsafe fn markmt(g: *mut global_State) {
    for i in 0..LUA_NUMTAGS {
        markobjectN!(g, (*g).mt[i]);
    }
}

/*
** mark all objects in list of being-finalized
*/
#[inline(always)]
unsafe fn markbeingfnz(g: *mut global_State) {
    let mut o = (*g).tobefnz;
    while !o.is_null() {
        markobject(g, o);
        o = (*o).next;
    }
}

/*
** Mark all values stored in marked open upvalues from non-marked threads.
** (Values from marked threads were already marked when traversing the
** thread.) Remove from the list threads that no longer have upvalues and
** not-marked threads.
*/
unsafe fn remarkupvals(g: *mut global_State) {
    let mut p: *mut *mut lua_State = &mut (*g).twups;
    let mut thread;
    while {
        thread = *p;
        !thread.is_null()
    } {
        debug_assert!(!isblack!(thread)); /* threads are never black */
        if isgray!(thread) && !(*thread).openupval.is_null() {
            p = &mut (*thread).twups; /* keep marked thread with upvalues in the list */
        } else {
            /* thread is not marked or without upvalues */
            *p = (*thread).twups; /* remove thread from the list */
            (*thread).twups = thread; /* mark that it is out of list */
            let mut uv = (*thread).openupval;
            while !uv.is_null() {
                if (*uv).u.open.touched != 0 {
                    markvalue(g, (*uv).v); /* remark upvalue's value */
                    (*uv).u.open.touched = 0;
                }
                uv = (*uv).u.open.next;
            }
        }
    }
}

/*
** mark root set and reset all gray lists, to start a new collection
*/
unsafe fn restartcollection(g: *mut global_State) {
    (*g).gray = ptr::null_mut();
    (*g).grayagain = ptr::null_mut();
    (*g).weak = ptr::null_mut();
    (*g).allweak = ptr::null_mut();
    (*g).ephemeron = ptr::null_mut();
    markobject(g, obj2gco!((*g).mainthread));
    markvalue(g, &mut (*g).l_registry);
    markmt(g);
    markbeingfnz(g); /* mark any finalizing object left from previous cycle */
}

/*
** Traverse functions
*/

/*
** Traverse a table with weak values and link it to proper list. During
** propagate phase, keep it in 'grayagain' list, to be revisited in the
** atomic phase. In the atomic phase, if table has any white value,
** put it in 'weak' list, to be cleared.
*/
unsafe fn traverseweakvalue(g: *mut global_State, h: *mut Table) {
    /* if there is array part, assume it may have white values (it is not
    worth traversing it now just to check) */
    let mut hasclears = (*h).sizearray > 0;
    let mut n = gnode(h, 0);
    let limit = gnodelast(h);
    while n < limit {
        /* traverse hash part */
        checkdeadkey(n);
        if ttisnil(gval(n)) {
            /* entry is empty? */
            removeentry(n); /* remove it */
        } else {
            debug_assert!(!ttisnil(gkey(n)));
            markvalue(g, gkey(n)); /* mark key */
            if !hasclears && iscleared(g, gval(n)) {
                /* is there a white value? */
                hasclears = true; /* table will have to be cleared */
            }
        }
        n = n.add(1);
    }
    if (*g).gcstate == GCSpropagate {
        linkgclist!(h, (*g).grayagain); /* must retraverse it in atomic phase */
    } else if hasclears {
        linkgclist!(h, (*g).weak); /* has to be cleared later */
    }
}

/*
** Traverse an ephemeron table and link it to proper list. Returns true
** iff any object was marked during this traversal (which implies that
** convergence has to continue). During propagation phase, keep table
** in 'grayagain' list, to be visited again in the atomic phase. In
** the atomic phase, if table has any white->white entry, it has to
** be revisited during ephemeron convergence (as that key may turn
** black). Otherwise, if it has any white key, table has to be cleared
** (in the atomic phase).
*/
unsafe fn traverseephemeron(g: *mut global_State, h: *mut Table) -> bool {
    let mut marked = false; /* true if an object is marked in this traversal */
    let mut hasclears = false; /* true if table has white keys */
    let mut hasww = false; /* true if table has entry "white-key -> white-value" */
    /* traverse array part */
    for i in 0..(*h).sizearray {
        if valiswhite((*h).array.add(i as usize)) {
            marked = true;
            reallymarkobject(g, gcvalue((*h).array.add(i as usize)));
        }
    }
    /* traverse hash part */
    let mut n = gnode(h, 0);
    let limit = gnodelast(h);
    while n < limit {
        checkdeadkey(n);
        if ttisnil(gval(n)) {
            /* entry is empty? */
            removeentry(n); /* remove it */
        } else if iscleared(g, gkey(n)) {
            /* key is not marked (yet)? */
            hasclears = true; /* table must be cleared */
            if valiswhite(gval(n)) {
                /* value not marked yet? */
                hasww = true; /* white-white entry */
            }
        } else if valiswhite(gval(n)) {
            /* value not marked yet? */
            marked = true;
            reallymarkobject(g, gcvalue(gval(n))); /* mark it now */
        }
        n = n.add(1);
    }
    /* link table into proper list */
    if (*g).gcstate == GCSpropagate {
        linkgclist!(h, (*g).grayagain); /* must retraverse it in atomic phase */
    } else if hasww {
        /* table has white->white entries? */
        linkgclist!(h, (*g).ephemeron); /* have to propagate again */
    } else if hasclears {
        /* table has white keys? */
        linkgclist!(h, (*g).allweak); /* may have to clean white keys */
    }
    return marked;
}

unsafe fn traversestrongtable(g: *mut global_State, h: *mut Table) {
    /* traverse array part */
    for i in 0..(*h).sizearray {
        markvalue(g, (*h).array.add(i as usize));
    }
    /* traverse hash part */
    let mut n = gnode(h, 0);
    let limit = gnodelast(h);
    while n < limit {
        checkdeadkey(n);
        if ttisnil(gval(n)) {
            /* entry is empty? */
            removeentry(n); /* remove it */
        } else {
            debug_assert!(!ttisnil(gkey(n)));
            markvalue(g, gkey(n)); /* mark key */
            markvalue(g, gval(n)); /* mark value */
        }
        n = n.add(1);
    }
}

unsafe fn traversetable(g: *mut global_State, h: *mut Table) -> usize {
    let (mut weakkey, mut weakvalue) = (ptr::null(), ptr::null());
    let mode = gfasttm(g, (*h).metatable, TM_MODE);
    markobjectN!(g, (*h).metatable);
    if !mode.is_null() && ttisstring(mode) && {
        /* is there a weak mode? */
        weakkey = strchr(svalue(mode), b'k' as i32);
        weakvalue = strchr(svalue(mode), b'v' as i32);
        /* is really weak? */
        !weakkey.is_null() || !weakvalue.is_null()
    } {
        black2gray(obj2gco!(h)); /* keep table gray */
        if weakkey.is_null() {
            /* strong keys? */
            traverseweakvalue(g, h);
        } else if weakvalue.is_null() {
            /* strong values? */
            traverseephemeron(g, h);
        } else {
            /* all weak */
            linkgclist!(h, (*g).allweak); /* nothing to traverse now */
        }
    } else {
        /* not weak */
        traversestrongtable(g, h);
    }
    return mem::size_of::<Table>()
        + (mem::size_of::<TValue>() * (*h).sizearray as usize)
        + (mem::size_of::<Node>() * allocsizenode(h));
}

/*
** Traverse a prototype. (While a prototype is being build, its
** arrays can be larger than needed; the extra slots are filled with
** NULL, so the use of 'markobjectN')
*/
unsafe fn traverseproto(g: *mut global_State, f: *mut Proto) -> usize {
    if !(*f).cache.is_null() && iswhite!((*f).cache) {
        (*f).cache = ptr::null_mut(); /* allow cache to be collected */
    }
    markobjectN!(g, (*f).source);
    for i in 0..(*f).sizek {
        /* mark literals */
        markvalue(g, (*f).k.add(i as usize));
    }
    for i in 0..(*f).sizeupvalues {
        /* mark upvalue names */
        markobjectN!(g, (*(*f).upvalues.add(i as usize)).name);
    }
    for i in 0..(*f).sizep {
        /* mark nested protos */
        markobjectN!(g, *(*f).p.add(i as usize));
    }
    for i in 0..(*f).sizelocvars {
        /* mark local-variable names */
        markobjectN!(g, (*(*f).locvars.add(i as usize)).varname);
    }
    return mem::size_of::<Proto>()
        + (mem::size_of::<Instruction>() * (*f).sizecode as usize)
        + (mem::size_of::<*const Proto>() * (*f).sizep as usize)
        + (mem::size_of::<TValue>() * (*f).sizek as usize)
        + (mem::size_of::<c_int>() * (*f).sizelineinfo as usize)
        + (mem::size_of::<LocVar>() * (*f).sizelocvars as usize)
        + (mem::size_of::<Upvaldesc>() * (*f).sizeupvalues as usize);
}

unsafe fn traverseCclosure(g: *mut global_State, cl: *mut CClosure) -> usize {
    for i in 0..(*cl).nupvalues {
        /* mark its upvalues */
        markvalue(g, (*cl).upvalue.as_mut_ptr().add(i as usize));
    }
    return sizeCclosure((*cl).nupvalues as c_int);
}

/*
** open upvalues point to values in a thread, so those values should
** be marked when the thread is traversed except in the atomic phase
** (because then the value cannot be changed by the thread and the
** thread may not be traversed again)
*/
unsafe fn traverseLclosure(g: *mut global_State, cl: *mut LClosure) -> usize {
    markobjectN!(g, (*cl).p); /* mark its prototype */
    for i in 0..(*cl).nupvalues as usize {
        /* mark its upvalues */
        let mut uv = *((*cl).upvals).as_mut_ptr().add(i);
        if !uv.is_null() {
            if upisopen(uv) && (*g).gcstate != GCSinsideatomic {
                (*uv).u.open.touched = 1; /* can be marked in 'remarkupvals' */
            } else {
                markvalue(g, (*uv).v);
            }
        }
    }
    return sizeLclosure((*cl).nupvalues as c_int);
}

unsafe fn traversethread(g: *mut global_State, th: *mut lua_State) -> usize {
    let mut o = (*th).stack;
    if o.is_null() {
        return 1; /* stack not completely built yet */
    }
    debug_assert!((*g).gcstate == GCSinsideatomic || (*th).openupval.is_null() || isintwups(th));
    while o < (*th).top {
        /* mark live elements in the stack */
        markvalue(g, o);
        o = o.add(1);
    }
    if (*g).gcstate == GCSinsideatomic {
        /* final traversal? */
        let limit = (*th).stack.add((*th).stacksize as usize); /* real end of stack */
        while o < limit {
            /* clear not-marked stack slice */
            setnilvalue(o);
            o = o.add(1);
        }
        /* 'remarkupvals' may have removed thread from 'twups' list */
        if !isintwups(th) && !(*th).openupval.is_null() {
            (*th).twups = (*g).twups; /* link it back to the list */
            (*g).twups = th;
        }
    } else if (*g).gckind != KGC_EMERGENCY {
        luaD_shrinkstack(th); /* do not change stack in emergency cycle */
    }
    return mem::size_of::<lua_State>()
        + (mem::size_of::<TValue>() * (*th).stacksize as usize)
        + (mem::size_of::<CallInfo>() * (*th).nci as usize);
}

/*
** traverse one gray object, turning it to black (except for threads,
** which are always gray).
*/
unsafe fn propagatemark(g: *mut global_State) {
    let o = (*g).gray;
    debug_assert!(isgray!(o));
    gray2black(o);
    let size = match (*o).tt as c_int {
        LUA_TTABLE => {
            let h = gco2t(o);
            (*g).gray = (*h).gclist; /* remove from 'gray' list */
            traversetable(g, h)
        }
        LUA_TLCL => {
            let cl = gco2lcl(o);
            (*g).gray = (*cl).gclist; /* remove from 'gray' list */
            traverseLclosure(g, cl)
        }
        LUA_TCCL => {
            let cl = gco2ccl(o);
            (*g).gray = (*cl).gclist; /* remove from 'gray' list */
            traverseCclosure(g, cl)
        }
        LUA_TTHREAD => {
            let th = gco2th(o);
            (*g).gray = (*th).gclist; /* remove from 'gray' list */
            linkgclist!(th, (*g).grayagain); /* insert into 'grayagain' list */
            black2gray(o);
            traversethread(g, th)
        }
        LUA_TPROTO => {
            let p = gco2p(o);
            (*g).gray = (*p).gclist; /* remove from 'gray' list */
            traverseproto(g, p)
        }
        _ => unreachable!(),
    };
    (*g).GCmemtrav += size;
}

#[inline(always)]
unsafe fn propagateall(g: *mut global_State) {
    while !(*g).gray.is_null() {
        propagatemark(g);
    }
}

unsafe fn convergeephemerons(g: *mut global_State) {
    let mut changed = true;
    while changed {
        let mut next = (*g).ephemeron; /* get ephemeron list */
        (*g).ephemeron = ptr::null_mut(); /* tables may return to this list when traversed */
        changed = false;
        let mut w;
        while {
            w = next;
            !w.is_null()
        } {
            next = (*gco2t(w)).gclist;
            if traverseephemeron(g, gco2t(w)) {
                /* traverse marked some value? */
                propagateall(g); /* propagate changes */
                changed = true; /* will have to revisit all ephemeron tables */
            }
        }
    }
}

/*
**
** Sweep Functions
**
*/

/*
** clear entries with unmarked keys from all weaktables in list 'l' up
** to element 'f'
*/
unsafe fn clearkeys(g: *mut global_State, mut l: *mut GCObject, f: *mut GCObject) {
    while l != f {
        let h = gco2t(l);
        let mut n = gnode(h, 0);
        let limit = gnodelast(h);
        while n < limit {
            if !ttisnil(gval(n)) && iscleared(g, gkey(n)) {
                setnilvalue(gval(n)); /* remove value ... */
            }
            if ttisnil(gval(n)) {
                /* is entry empty? */
                removeentry(n); /* remove entry from table */
            }
            n = n.add(1);
        }
        l = (*gco2t(l)).gclist;
    }
}

/*
** clear entries with unmarked values from all weaktables in list 'l' up
** to element 'f'
*/
unsafe fn clearvalues(g: *mut global_State, mut l: *mut GCObject, f: *mut GCObject) {
    while l != f {
        let h = gco2t(l);
        for i in 0..(*h).sizearray as usize {
            let o = (*h).array.add(i);
            if iscleared(g, o) {
                /* value was collected? */
                setnilvalue(o); /* remove value */
            }
        }
        let mut n = gnode(h, 0);
        let limit = gnodelast(h);
        while n < limit {
            if !ttisnil(gval(n)) && iscleared(g, gval(n)) {
                setnilvalue(gval(n)); /* remove value ... */
                removeentry(n); /* and remove entry from table */
            }
            n = n.add(1);
        }
        l = (*gco2t(l)).gclist;
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaC_upvdeccount(L: *mut lua_State, uv: *mut UpVal) {
    debug_assert!((*uv).refcount > 0);
    (*uv).refcount -= 1;
    if (*uv).refcount == 0 && !upisopen(uv) {
        luaM_free(L, uv);
    }
}

unsafe fn freeLclosure(L: *mut lua_State, cl: *mut LClosure) {
    for i in 0..(*cl).nupvalues as usize {
        let uv = *(*cl).upvals.as_mut_ptr().add(i);
        if !uv.is_null() {
            luaC_upvdeccount(L, uv);
        }
    }
    luaM_freemem(L, cl as *mut c_void, sizeLclosure((*cl).nupvalues as c_int));
}

unsafe fn freeobj(L: *mut lua_State, o: *mut GCObject) {
    match (*o).tt as c_int {
        LUA_TPROTO => {
            luaF_freeproto(L, gco2p(o));
        }
        LUA_TLCL => {
            freeLclosure(L, gco2lcl(o));
        }
        LUA_TCCL => {
            luaM_freemem(
                L,
                o as *mut c_void,
                sizeCclosure((*gco2ccl(o)).nupvalues as c_int),
            );
        }
        LUA_TTABLE => {
            luaH_free(L, gco2t(o));
        }
        LUA_TTHREAD => {
            luaE_freethread(L, gco2th(o));
        }
        LUA_TUSERDATA => {
            luaM_freemem(L, o as *mut c_void, sizeudata(gco2u(o)));
        }
        LUA_TSHRSTR => {
            luaS_remove(L, gco2ts(o)); /* remove it from hash table */
            luaM_freemem(
                L,
                o as *mut c_void,
                sizelstring((*gco2ts(o)).shrlen as usize),
            );
        }
        LUA_TLNGSTR => {
            luaM_freemem(L, o as *mut c_void, sizelstring((*gco2ts(o)).u.lnglen));
        }
        _ => unreachable!(),
    };
}

#[inline(always)]
unsafe fn sweepwholelist(L: *mut lua_State, p: *mut *mut GCObject) {
    sweeplist(L, p, usize::MAX);
}

/*
** sweep at most 'count' elements from a list of GCObjects erasing dead
** objects, where a dead object is one marked with the old (non current)
** white; change all non-dead objects back to white, preparing for next
** collection cycle. Return where to continue the traversal or NULL if
** list is finished.
*/
unsafe fn sweeplist(
    L: *mut lua_State,
    mut p: *mut *mut GCObject,
    mut count: usize,
) -> *mut *mut GCObject {
    let g = (*L).l_G;
    let ow = otherwhite(g);
    let white = luaC_white(g); /* current white */
    while !(*p).is_null() && count > 0 {
        count -= 1;
        let curr = *p;
        let marked = (*curr).marked;
        if isdeadm(ow, marked) {
            /* is 'curr' dead? */
            *p = (*curr).next; /* remove 'curr' from list */
            freeobj(L, curr); /* erase 'curr' */
        } else {
            /* change mark to 'white' */
            (*curr).marked = (marked & maskcolors) | white;
            p = &mut (*curr).next; /* go to next element */
        }
    }
    return if (*p).is_null() { ptr::null_mut() } else { p };
}

/*
** sweep a list until a live object (or end of list)
*/
unsafe fn sweeptolive(L: *mut lua_State, mut p: *mut *mut GCObject) -> *mut *mut GCObject {
    let old = p;
    while p == old {
        p = sweeplist(L, p, 1);
    }
    return p;
}

/*
**
** Finalization
**
*/

/*
** If possible, shrink string table
*/
unsafe fn checkSizes(L: *mut lua_State, g: *mut global_State) {
    if (*g).gckind != KGC_EMERGENCY {
        let olddebt = (*g).GCdebt;
        if (*g).strt.nuse < (*g).strt.size / 4 {
            /* string table too big? */
            luaS_resize(L, (*g).strt.size / 2); /* shrink it a little */
        }
        /* update estimate */
        (*g).GCestimate = ((*g).GCestimate as isize + (*g).GCdebt - olddebt) as usize;
    }
}

unsafe fn udata2finalize(g: *mut global_State) -> *mut GCObject {
    let mut o = (*g).tobefnz; /* get first element */
    debug_assert!(tofinalize(o));
    (*g).tobefnz = (*o).next; /* remove it from 'tobefnz' list */
    (*o).next = (*g).allgc; /* return it to 'allgc' list */
    (*g).allgc = o;
    resetbit(&mut (*o).marked, FINALIZEDBIT); /* object is "normal" again */
    if issweepphase(g) {
        makewhite(g, o); /* "sweep" object */
    }
    return o;
}

#[inline(always)]
unsafe extern "C" fn dothecall(L: *mut lua_State, _ud: *mut c_void) {
    luaD_callnoyield(L, ((*L).top).sub(2), 0);
}

unsafe fn GCTM(L: *mut lua_State, propagateerrors: c_int) {
    let g = (*L).l_G;
    let mut v = TValue::new();
    setgcovalue(&mut v, udata2finalize(g));
    let tm = luaT_gettmbyobj(L, &v, TM_GC);
    if !tm.is_null() && ttisfunction(tm) {
        /* is there a finalizer? */
        let oldah = (*L).allowhook;
        let running = (*g).gcrunning;
        (*L).allowhook = 0; /* stop debug hooks during GC metamethod */
        (*g).gcrunning = 0; /* avoid GC steps */
        setobj(L, (*L).top, tm); /* push finalizer... */
        setobj(L, (*L).top.add(1), &v); /* ... and its argument */
        (*L).top = (*L).top.add(2); /* and (next line) call the finalizer */
        (*(*L).ci).callstatus |= CIST_FIN; /* will run a finalizer */
        let mut status = luaD_pcall(
            L,
            Some(dothecall),
            ptr::null_mut(),
            savestack(L, (*L).top.sub(2)),
            0,
        );
        (*(*L).ci).callstatus &= !CIST_FIN; /* not running a finalizer anymore */
        (*L).allowhook = oldah; /* restore hooks */
        (*g).gcrunning = running; /* restore state */
        if status != LUA_OK && propagateerrors != 0 {
            /* error while running __gc? */
            if status == LUA_ERRRUN {
                /* is there an error object? */
                let msg = if ttisstring((*L).top.sub(1)) {
                    svalue((*L).top.sub(1))
                } else {
                    cstr!("no message")
                };
                luaO_pushfstring(L, cstr!("error in __gc metamethod (%s)"), msg);
                status = LUA_ERRGCMM; /* error in __gc metamethod */
            }
            luaD_throw(L, status); /* re-throw error */
        }
    }
}

/*
** call a few (up to 'g->gcfinnum') finalizers
*/
unsafe fn runafewfinalizers(L: *mut lua_State) -> c_int {
    let g = (*L).l_G;
    debug_assert!((*g).tobefnz.is_null() || (*g).gcfinnum > 0);
    let mut i = 0;
    while !(*g).tobefnz.is_null() && i < (*g).gcfinnum {
        GCTM(L, 1); /* call one finalizer */
        i += 1;
    }
    (*g).gcfinnum = if (*g).tobefnz.is_null() {
        0 /* nothing more to finalize? */
    } else {
        (*g).gcfinnum * 2 /* else call a few more next time */
    };
    return i as c_int;
}

/*
** call all pending finalizers
*/
#[inline(always)]
unsafe fn callallpendingfinalizers(L: *mut lua_State) {
    let g = (*L).l_G;
    while !(*g).tobefnz.is_null() {
        GCTM(L, 0);
    }
}

/*
** find last 'next' field in list 'p' list (to add elements in its end)
*/
#[inline(always)]
unsafe fn findlast(mut p: *mut *mut GCObject) -> *mut *mut GCObject {
    while !(*p).is_null() {
        p = &mut (**p).next;
    }
    return p;
}

/*
** move all unreachable objects (or 'all' objects) that need
** finalization from list 'finobj' to list 'tobefnz' (to be finalized)
*/
unsafe fn separatetobefnz(g: *mut global_State, all: c_int) {
    let mut p = &mut (*g).finobj;
    let mut lastnext = findlast(&mut (*g).tobefnz);
    let mut curr;
    while {
        curr = *p;
        !curr.is_null()
    } {
        /* traverse all finalizable objects */
        debug_assert!(tofinalize(curr));
        if !(iswhite!(curr) || all != 0) {
            /* not being collected? */
            p = &mut (*curr).next; /* don't bother with it */
        } else {
            *p = (*curr).next; /* remove 'curr' from 'finobj' list */
            (*curr).next = *lastnext; /* link at the end of 'tobefnz' list */
            *lastnext = curr;
            lastnext = &mut (*curr).next;
        }
    }
}

/*
** if object 'o' has a finalizer, remove it from 'allgc' list (must
** search the list to find it) and link it in 'finobj' list.
*/
#[no_mangle]
pub unsafe extern "C" fn luaC_checkfinalizer(L: *mut lua_State, o: *mut GCObject, mt: *mut Table) {
    let g = (*L).l_G;
    if tofinalize(o) || gfasttm(g, mt, TM_GC).is_null() {
        /* obj. is already marked... or  or has no finalizer? */
        return; /* nothing to be done */
    }
    /* move 'o' to 'finobj' list */
    if issweepphase(g) {
        makewhite(g, o); /* "sweep" object 'o' */
        if (*g).sweepgc == &mut (*o).next {
            /* should not remove 'sweepgc' object */
            (*g).sweepgc = sweeptolive(L, (*g).sweepgc); /* change 'sweepgc' */
        }
    }
    /* search for pointer pointing to 'o' */
    let mut p = &mut (*g).allgc;
    while *p != o {
        p = &mut (**p).next;
    }
    *p = (*o).next; /* remove 'o' from 'allgc' list */
    (*o).next = (*g).finobj; /* link it in 'finobj' list */
    (*g).finobj = o;
    l_setbit(&mut (*o).marked, FINALIZEDBIT); /* mark it as such */
}

/*
**
** GC control
**
*/

/*
** Set a reasonable "time" to wait before starting a new GC cycle; cycle
** will start when memory use hits threshold. (Division by 'estimate'
** should be OK: it cannot be zero (because Lua cannot even start with
** less than PAUSEADJ bytes).
*/
unsafe fn setpause(g: *mut global_State) {
    let estimate = ((*g).GCestimate / PAUSEADJ) as isize; /* adjust 'estimate' */
    debug_assert!(estimate > 0);
    /* overflow? */
    let threshold = if ((*g).gcpause as isize) < (isize::MAX / estimate) {
        estimate * (*g).gcpause as isize /* no overflow */
    } else {
        isize::MAX /* overflow; truncate to maximum */
    };
    let debt = gettotalbytes(g) as isize - threshold;
    luaE_setdebt(g, debt);
}

/*
** Enter first sweep phase.
** The call to 'sweeplist' tries to make pointer point to an object
** inside the list (instead of to the header), so that the real sweep do
** not need to skip objects created between "now" and the start of the
** real sweep.
*/
unsafe fn entersweep(L: *mut lua_State) {
    let g = (*L).l_G;
    (*g).gcstate = GCSswpallgc;
    debug_assert!((*g).sweepgc.is_null());
    (*g).sweepgc = sweeplist(L, &mut (*g).allgc, 1);
}

#[no_mangle]
pub unsafe extern "C" fn luaC_freeallobjects(L: *mut lua_State) {
    let g = (*L).l_G;
    separatetobefnz(g, 1); /* separate all objects with finalizers */
    debug_assert!((*g).finobj.is_null());
    callallpendingfinalizers(L);
    debug_assert!((*g).tobefnz.is_null());
    (*g).currentwhite = WHITEBITS; /* this "white" makes all objects look dead */
    (*g).gckind = KGC_NORMAL;
    sweepwholelist(L, &mut (*g).finobj);
    sweepwholelist(L, &mut (*g).allgc);
    sweepwholelist(L, &mut (*g).fixedgc); /* collect fixed objects */
    debug_assert!((*g).strt.nuse == 0);
}

unsafe fn atomic(L: *mut lua_State) -> usize {
    let g = (*L).l_G;
    let grayagain = (*g).grayagain; /* save original list */
    debug_assert!((*g).ephemeron.is_null() && (*g).weak.is_null());
    debug_assert!(!iswhite!((*g).mainthread));
    (*g).gcstate = GCSinsideatomic;
    (*g).GCmemtrav = 0; /* start counting work */
    markobject(g, obj2gco!(L)); /* mark running thread */
    /* registry and global metatables may be changed by API */
    markvalue(g, &(*g).l_registry);
    markmt(g); /* mark global metatables */
    /* remark occasional upvalues of (maybe) dead threads */
    remarkupvals(g);
    propagateall(g); /* propagate changes */
    let mut work = (*g).GCmemtrav; /* stop counting (do not recount 'grayagain') */
    (*g).gray = grayagain;
    propagateall(g); /* traverse 'grayagain' list */
    (*g).GCmemtrav = 0; /* restart counting */
    convergeephemerons(g);
    /* at this point, all strongly accessible objects are marked. */
    /* Clear values from weak tables, before checking finalizers */
    clearvalues(g, (*g).weak, ptr::null_mut());
    clearvalues(g, (*g).allweak, ptr::null_mut());
    let origweak = (*g).weak;
    let origall = (*g).allweak;
    work += (*g).GCmemtrav; /* stop counting (objects being finalized) */
    separatetobefnz(g, 0); /* separate objects to be finalized */
    (*g).gcfinnum = 1; /* there may be objects to be finalized */
    markbeingfnz(g); /* mark objects that will be finalized */
    propagateall(g); /* remark, to propagate 'resurrection' */
    (*g).GCmemtrav = 0; /* restart counting */
    convergeephemerons(g);
    /* at this point, all resurrected objects are marked. */
    /* remove dead objects from weak tables */
    clearkeys(g, (*g).ephemeron, ptr::null_mut()); /* clear keys from all ephemeron tables */
    clearkeys(g, (*g).allweak, ptr::null_mut()); /* clear keys from all 'allweak' tables */
    /* clear values from resurrected weak tables */
    clearvalues(g, (*g).weak, origweak);
    clearvalues(g, (*g).allweak, origall);
    luaS_clearcache(g);
    (*g).currentwhite = otherwhite(g); /* flip current white */
    work += (*g).GCmemtrav; /* complete counting */
    return work; /* estimate of memory marked by 'atomic' */
}

unsafe fn sweepstep(
    L: *mut lua_State,
    g: *mut global_State,
    nextstate: u8,
    nextlist: *mut *mut GCObject,
) -> usize {
    if !(*g).sweepgc.is_null() {
        let olddebt = (*g).GCdebt;
        (*g).sweepgc = sweeplist(L, (*g).sweepgc, GCSWEEPMAX);
        (*g).GCestimate = ((*g).GCestimate as isize + (*g).GCdebt - olddebt) as usize; /* update estimate */
        if !(*g).sweepgc.is_null() {
            /* is there still something to sweep? */
            return GCSWEEPMAX * GCSWEEPCOST;
        }
    }
    /* else enter next state */
    (*g).gcstate = nextstate;
    (*g).sweepgc = nextlist;
    return 0;
}

unsafe fn singlestep(L: *mut lua_State) -> usize {
    let g = (*L).l_G;
    match (*g).gcstate {
        GCSpause => {
            (*g).GCmemtrav = (*g).strt.size as usize * mem::size_of::<*const GCObject>();
            restartcollection(g);
            (*g).gcstate = GCSpropagate;
            return (*g).GCmemtrav;
        }
        GCSpropagate => {
            (*g).GCmemtrav = 0;
            debug_assert!(!(*g).gray.is_null());
            propagatemark(g);
            if (*g).gray.is_null() {
                /* no more gray objects? */
                (*g).gcstate = GCSatomic; /* finish propagate phase */
            }
            return (*g).GCmemtrav; /* memory traversed in this step */
        }
        GCSatomic => {
            propagateall(g); /* make sure gray list is empty */
            let work = atomic(L); /* work is what was traversed by 'atomic' */
            entersweep(L);
            (*g).GCestimate = gettotalbytes(g); /* first estimate */
            return work;
        }
        GCSswpallgc => return sweepstep(L, g, GCSswpfinobj, &mut (*g).finobj), /* sweep "regular" objects */
        GCSswpfinobj => return sweepstep(L, g, GCSswptobefnz, &mut (*g).tobefnz), /* sweep objects with finalizers */
        GCSswptobefnz => return sweepstep(L, g, GCSswpend, ptr::null_mut()), /* sweep objects to be finalized */
        GCSswpend => {
            /* finish sweeps */
            makewhite(g, obj2gco!((*g).mainthread)); /* sweep main thread */
            checkSizes(L, g);
            (*g).gcstate = GCScallfin;
            return 0;
        }
        GCScallfin => {
            /* call remaining finalizers */
            if !(*g).tobefnz.is_null() && (*g).gckind != KGC_EMERGENCY {
                let n = runafewfinalizers(L) as usize;
                return n * GCFINALIZECOST;
            } else {
                /* emergency mode or no more finalizers */
                (*g).gcstate = GCSpause; /* finish collection */
                return 0;
            }
        }
        _ => unreachable!(),
    }
}

/*
** advances the garbage collector until it reaches a state allowed
** by 'statemask'
*/
#[no_mangle]
pub unsafe extern "C" fn luaC_runtilstate(L: *mut lua_State, statesmask: c_int) {
    let g = (*L).l_G;
    while !testbit(statesmask as u8, (*g).gcstate) {
        singlestep(L);
    }
}

/*
** get GC debt and convert it from Kb to 'work units' (avoid zero debt
** and overflows)
*/
unsafe fn getdebt(g: *mut global_State) -> isize {
    let debt = (*g).GCdebt;
    let stepmul = (*g).gcstepmul;
    if debt <= 0 {
        return 0; /* minimal debt */
    }
    let mut debt = debt as usize;
    debt = (debt / STEPMULADJ) + 1;
    debt = if debt < isize::MAX as usize / stepmul as usize {
        debt * stepmul as usize
    } else {
        isize::MAX as usize
    };
    return debt as isize;
}

/*
** performs a basic GC step when collector is running
*/
#[no_mangle]
pub unsafe extern "C" fn luaC_step(L: *mut lua_State) {
    let g = (*L).l_G;
    let mut debt = getdebt(g); /* GC deficit (be paid now) */
    if (*g).gcrunning == 0 {
        /* not running? */
        luaE_setdebt(g, -(GCSTEPSIZE as isize * 10)); /* avoid being called too often */
        return;
    }
    loop {
        /* repeat until pause or enough "credit" (negative debt) */
        let work = singlestep(L); /* perform one single step */
        debt -= work as isize;
        if debt > -(GCSTEPSIZE as isize) && (*g).gcstate != GCSpause {
            continue;
        }
        break;
    }
    if (*g).gcstate == GCSpause {
        setpause(g); /* pause until next cycle */
    } else {
        debt = (debt / (*g).gcstepmul as isize) * (STEPMULADJ as isize); /* convert 'work units' to Kb */
        luaE_setdebt(g, debt);
        runafewfinalizers(L);
    }
}

/*
** Performs a full GC cycle; if 'isemergency', set a flag to avoid
** some operations which could change the interpreter state in some
** unexpected ways (running finalizers and shrinking some structures).
** Before running the collection, check 'keepinvariant'; if it is true,
** there may be some objects marked as black, so the collector has
** to sweep all objects to turn them back to white (as white has not
** changed, nothing will be collected).
*/
#[no_mangle]
pub unsafe extern "C" fn luaC_fullgc(L: *mut lua_State, isemergency: c_int) {
    let g = (*L).l_G;
    debug_assert!((*g).gckind == KGC_NORMAL);
    if isemergency != 0 {
        (*g).gckind = KGC_EMERGENCY; /* set flag */
    }
    if keepinvariant(g) {
        /* black objects? */
        entersweep(L); /* sweep everything to turn them back to white */
    }
    /* finish any pending sweep phase to start a new cycle */
    luaC_runtilstate(L, bitmask(GCSpause) as c_int);
    luaC_runtilstate(L, !bitmask(GCSpause) as c_int); /* start new collection */
    luaC_runtilstate(L, bitmask(GCScallfin) as c_int); /* run up to finalizers */
    /* estimate must be correct after a full GC cycle */
    debug_assert!((*g).GCestimate == gettotalbytes(g));
    luaC_runtilstate(L, bitmask(GCSpause) as c_int); /* finish collection */
    (*g).gckind = KGC_NORMAL;
    setpause(g);
}
