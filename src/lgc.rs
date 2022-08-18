/*
** Garbage Collector
*/

use std::env;
use std::mem;

use libc::{c_int, size_t};

use crate::lfunc::upisopen;
use crate::lfunc::UpVal;
use crate::llimits::lu_byte;
use crate::lobject::{iscollectable, GCObject, TString};
use crate::lstate::{global_State, lua_State};

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
pub const GCSTEPSIZE: c_int = 100 * mem::size_of::<TString>() as c_int;

/*
** Possible states of the Garbage Collector
*/
pub const GCSpropagate: c_int = 0;
pub const GCSatomic: c_int = 1;
pub const GCSswpallgc: c_int = 2;
pub const GCSswpfinobj: c_int = 3;
pub const GCSswptobefnz: c_int = 4;
pub const GCSswpend: c_int = 5;
pub const GCScallfin: c_int = 6;
pub const GCSpause: c_int = 7;

// #define issweepphase(g)  \
// 	(GCSswpallgc <= (g)->gcstate && (g)->gcstate <= GCSswpend)

/*
** macro to tell when main invariant (white objects cannot point to black
** ones) must be kept. During a collection, the sweep
** phase may break the invariant, as objects turned white may point to
** still-black objects. The invariant is restored when sweep ends and
** all objects are white again.
*/

// #define keepinvariant(g)	((g)->gcstate <= GCSatomic)

/*
** some useful bit tricks
*/
pub fn resetbits(x: &mut lu_byte, m: lu_byte) {
    *x &= !m
}

pub fn setbits(x: &mut lu_byte, m: lu_byte) {
    *x |= m
}

pub const fn testbits(x: lu_byte, m: lu_byte) -> lu_byte {
    x & m
}

pub const fn bitmask(b: lu_byte) -> lu_byte {
    1 << b
}

pub const fn bit2mask(b1: lu_byte, b2: lu_byte) -> lu_byte {
    bitmask(b1) | bitmask(b2)
}

pub fn l_setbit(x: &mut lu_byte, b: lu_byte) {
    setbits(x, bitmask(b))
}

pub fn resetbit(x: &mut lu_byte, b: lu_byte) {
    resetbits(x, bitmask(b))
}

pub const fn testbit(x: lu_byte, b: lu_byte) -> lu_byte {
    testbits(x, bitmask(b))
}

/* Layout for bit use in 'marked' field: */
pub const WHITE0BIT: lu_byte = 0; /* object is white (type 0) */
pub const WHITE1BIT: lu_byte = 1; /* object is white (type 1) */
pub const BLACKBIT: lu_byte = 2; /* object is black */
pub const FINALIZEDBIT: lu_byte = 3; /* object has been marked for finalization */
/* bit 7 is currently used by tests (luaL_checkmemory) */

pub const WHITEBITS: lu_byte = bit2mask(WHITE0BIT, WHITE1BIT);

pub unsafe fn iswhite(x: *mut GCObject) -> bool {
    testbits((*x).marked, WHITEBITS) != 0
}

pub unsafe fn isblack(x: *mut GCObject) -> bool {
    testbit((*x).marked, BLACKBIT) != 0
}

pub unsafe fn isgray(x: *mut GCObject) -> bool {
    /* neither white nor black */
    testbits((*x).marked, WHITEBITS | bitmask(BLACKBIT)) == 0
}

pub unsafe fn tofinalize(x: *mut GCObject) -> bool {
    testbit((*x).marked, FINALIZEDBIT) != 0
}

pub unsafe fn otherwhite(g: *mut global_State) -> lu_byte {
    (*g).currentwhite ^ WHITEBITS
}

pub const fn isdeadm(ow: lu_byte, m: lu_byte) -> bool {
    (m ^ WHITEBITS) & ow == 0
}

pub unsafe fn isdead(g: *mut global_State, v: *mut GCObject) -> bool {
    isdeadm(otherwhite(g), (*v).marked)
}

pub unsafe fn changewhite(x: *mut GCObject) {
    (*x).marked ^= WHITEBITS;
}

// #define gray2black(x)	l_setbit((x)->marked, BLACKBIT)
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

// #define luaC_condGC(L,pre,pos) \
// 	{ if (G(L)->GCdebt > 0) { pre; luaC_step(L); pos;}; \
// 	  condchangemem(L,pre,pos); }

/* more often than not, 'pre'/'pos' are empty */
pub unsafe fn luaC_checkGC(L: *mut lua_State) {
    if (*(*L).l_G).GCdebt > 0 {
        luaC_step(L);
        #[cfg(debug_assertions)]
        if env::var("LUA_HARDMEMTESTS").as_deref() == Ok("1") {
            luaC_fullgc(L, 0);
        }
    }
}

// #define luaC_barrier(L,p,v) (  \
// 	(iscollectable(v) && isblack(p) && iswhite(gcvalue(v))) ?  \
// 	luaC_barrier_(L,obj2gco(p),gcvalue(v)) : cast_void(0))

// #define luaC_barrierback(L,p,v) (  \
// 	(iscollectable(v) && isblack(p) && iswhite(gcvalue(v))) ? \
// 	luaC_barrierback_(L,p) : cast_void(0))

// #define luaC_objbarrier(L,p,o) (  \
// 	(isblack(p) && iswhite(o)) ? \
// 	luaC_barrier_(L,obj2gco(p),obj2gco(o)) : cast_void(0))

pub unsafe fn luaC_upvalbarrier(L: *mut lua_State, uv: *mut UpVal) {
    if iscollectable((*uv).v) && !upisopen(uv) {
        luaC_upvalbarrier_(L, uv)
    }
}

extern "C" {
    pub fn luaC_upvalbarrier_(L: *mut lua_State, uv: *mut UpVal);
    pub fn luaC_fix(L: *mut lua_State, o: *mut GCObject);
    pub fn luaC_newobj(L: *mut lua_State, tt: c_int, sz: size_t) -> *mut GCObject;
    pub fn luaC_step(L: *mut lua_State);
    pub fn luaC_freeallobjects(L: *mut lua_State);
    pub fn luaC_fullgc(L: *mut lua_State, isemergency: c_int);
}
