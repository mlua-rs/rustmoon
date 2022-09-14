/*
** Lua API
*/

use libc::{c_char, c_int, c_long, c_uint, c_ulong, c_void, ptrdiff_t, size_t};
use std::ptr;

use crate::ldebug::luaG_errormsg;
use crate::ldo::{
    luaD_call, luaD_callnoyield, luaD_growstack, luaD_pcall, luaD_protectedparser,
    luaD_rawrunprotected,
};
use crate::ldump::luaU_dump;
use crate::lfunc::{luaF_newCclosure, UpVal};
use crate::lgc::{luaC_barrier_, luaC_barrierback_, luaC_checkGC, luaC_fullgc, luaC_upvalbarrier_};
use crate::llimits::{l_mem, lu_byte, lu_mem};
use crate::lobject::{
    clCvalue, luaO_arith, luaO_nilobject_, luaO_pushvfstring, luaO_str2num, luaO_tostring,
    setivalue, setnilvalue, setobj, ttislcf, CClosure, GCObject, LClosure, Proto, StkId, TString,
    TValue, Table, UTString, UUdata, Udata, Value,
};
use crate::lstate::{global_State, luaE_setdebt, lua_State, CallInfo, GCUnion};
use crate::lstring::{luaS_new, luaS_newlstr, luaS_newudata};
use crate::ltable::{
    luaH_get, luaH_getint, luaH_getn, luaH_getstr, luaH_new, luaH_next, luaH_resize, luaH_set,
    luaH_setint,
};
use crate::ltm::luaT_typenames_;
use crate::lvm::{luaV_concat, luaV_equalobj, luaV_tointeger, luaV_tonumber_};
use crate::lzio::{luaZ_init, ZIO};
use crate::types::{
    lua_Alloc, lua_CFunction, lua_Integer, lua_KContext, lua_KFunction, lua_Number, lua_Reader,
    lua_Writer, LUA_MULTRET, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS, LUA_TFUNCTION, LUA_TNIL, LUA_VERSION_NUM,
};

pub(crate) unsafe fn api_incr_top(L: *mut lua_State) {
    (*L).top = (*L).top.add(1);
    debug_assert!((*L).top <= (*(*L).ci).top, "stack overflow");
}

pub(crate) unsafe fn adjustresults(L: *mut lua_State, nres: i32) {
    if nres == LUA_MULTRET && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top;
    }
}

pub(crate) unsafe fn api_checknelems(L: *mut lua_State, n: i32) {
    debug_assert!(
        (n as isize) < (*L).top.offset_from((*(*L).ci).func),
        "not enough elements in the stack"
    );
}

pub const fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_REGISTRYINDEX - i
}

pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -n - 1)
}

pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0)
}

pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0)
}

pub unsafe fn lua_pushglobaltable(L: *mut lua_State) {
    lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS);
}

pub unsafe fn lua_tostring(L: *mut lua_State, idx: c_int) -> *const c_char {
    lua_tolstring(L, idx, ptr::null_mut())
}

pub unsafe fn lua_isfunction(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TFUNCTION) as c_int
}

pub unsafe fn lua_isnil(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TNIL) as c_int
}

pub unsafe fn lua_insert(L: *mut lua_State, idx: c_int) {
    lua_rotate(L, idx, 1)
}

pub unsafe fn lua_remove(L: *mut lua_State, idx: c_int) {
    lua_rotate(L, idx, -1);
    lua_pop(L, 1);
}

pub unsafe fn lua_call(L: *mut lua_State, n: c_int, r: c_int) {
    lua_callk(L, n, r, 0, None)
}

pub unsafe fn lua_pcall(L: *mut lua_State, n: c_int, r: c_int, f: c_int) -> c_int {
    lua_pcallk(L, n, r, f, 0, None)
}

/* test for pseudo index */
#[inline(always)]
fn ispseudo(i: c_int) -> bool {
    return i <= LUA_REGISTRYINDEX;
}

unsafe extern "C" fn index2addr(
    L: *mut lua_State,
    mut idx: libc::c_int,
) -> *mut TValue {
    let ci = (*L).ci;
    if idx > 0 as libc::c_int {
        let o = ((*ci).func).offset(idx as isize);
        if o >= (*L).top {
            return &luaO_nilobject_ as *const TValue as *mut TValue
        } else {
            return o
        }
    } else if !ispseudo(idx) {
        return ((*L).top).offset(idx as isize)
    } else if idx == LUA_REGISTRYINDEX {
        return &mut (*(*L).l_G).l_registry
    } else {
        idx = LUA_REGISTRYINDEX - idx;
        if ttislcf((*ci).func) {
            return &luaO_nilobject_ as *const TValue as *mut TValue
        } else {
            let func: *mut CClosure = clCvalue((*ci).func);
            return if idx <= (*func).nupvalues as libc::c_int {
                &mut *((*func).upvalue)
                    .as_mut_ptr()
                    .offset((idx - 1 as libc::c_int) as isize) as *mut TValue
            } else {
                &luaO_nilobject_ as *const TValue as *mut TValue
            };
        }
    };
}

unsafe extern "C" fn growstack(L: *mut lua_State, ud: *mut c_void) {
    let size: c_int = *(ud as *mut c_int);
    luaD_growstack(L, size);
}

#[no_mangle]
pub unsafe extern "C" fn lua_checkstack(L: *mut lua_State, mut n: c_int) -> c_int {
    let res: c_int;
    let ci: *mut CallInfo = (*L).ci;
    if ((*L).stack_last).offset_from((*L).top) as libc::c_long > n as libc::c_long {
        res = 1 as c_int;
    } else {
        let inuse: c_int = ((*L).top).offset_from((*L).stack) as libc::c_long as c_int + 5 as c_int;
        if inuse > 1000000 as c_int - n {
            res = 0 as c_int;
        } else {
            res = (luaD_rawrunprotected(
                L,
                Some(growstack as unsafe extern "C" fn(*mut lua_State, *mut c_void) -> ()),
                &mut n as *mut c_int as *mut c_void,
            ) == 0 as c_int) as c_int;
        }
    }
    if res != 0 && (*ci).top < ((*L).top).offset(n as isize) {
        let ref mut fresh0 = (*ci).top;
        *fresh0 = ((*L).top).offset(n as isize);
    }
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int) {
    let mut i: c_int;
    if from == to {
        return;
    }
    let ref mut fresh1 = (*from).top;
    *fresh1 = (*fresh1).offset(-(n as isize));
    i = 0 as c_int;
    while i < n {
        let io1: *mut TValue = (*to).top;
        *io1 = *((*from).top).offset(i as isize);
        let ref mut fresh2 = (*to).top;
        *fresh2 = (*fresh2).offset(1);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction {
    let old: lua_CFunction;
    old = (*(*L).l_G).panic;
    let ref mut fresh3 = (*(*L).l_G).panic;
    *fresh3 = panicf;
    return old;
}

#[no_mangle]
pub unsafe extern "C" fn lua_version(L: *mut lua_State) -> *const lua_Number {
    static mut version: lua_Number = LUA_VERSION_NUM as lua_Number;
    if L.is_null() { return &version } else { return (*(*L).l_G).version };
}

/*
** basic stack manipulation
*/

/*
** convert an acceptable stack index into an absolute index
*/
#[no_mangle]
pub unsafe extern "C" fn lua_absindex(L: *mut lua_State, idx: c_int) -> c_int {
    return if idx > 0 as c_int || idx <= -(1000000 as c_int) - 1000 as c_int {
        idx
    } else {
        ((*L).top).offset_from((*(*L).ci).func) as c_long as c_int + idx
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_gettop(L: *mut lua_State) -> c_int {
    return ((*L).top).offset_from(((*(*L).ci).func).offset(1 as c_int as isize)) as c_long
        as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_settop(L: *mut lua_State, idx: c_int) {
    let func: StkId = (*(*L).ci).func;
    if idx >= 0 {
        while (*L).top < func.offset(1 as c_int as isize).offset(idx as isize) {
            let ref mut fresh4 = (*L).top;
            let fresh5 = *fresh4;
            *fresh4 = (*fresh4).offset(1);
            (*fresh5).tt_ = 0 as c_int;
        }
        let ref mut fresh6 = (*L).top;
        *fresh6 = func.offset(1 as c_int as isize).offset(idx as isize);
    } else {
        let ref mut fresh7 = (*L).top;
        *fresh7 = (*fresh7).offset((idx + 1 as c_int) as isize);
    };
}

/*
** Reverse the stack segment from 'from' to 'to'
** (auxiliary to 'lua_rotate')
*/
unsafe extern "C" fn reverse(_L: *mut lua_State, mut from: StkId, mut to: StkId) {
    while from < to {
        let mut temp: TValue = TValue {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0,
        };
        let io1: *mut TValue = &mut temp;
        *io1 = *from;
        let io1_0: *mut TValue = from;
        *io1_0 = *to;
        let io1_1: *mut TValue = to;
        *io1_1 = temp;
        from = from.offset(1);
        to = to.offset(-1);
    }
}
/*
** Let x = AB, where A is a prefix of length 'n'. Then,
** rotate x n == BA. But BA == (A^r . B^r)^r.
*/
#[no_mangle]
pub unsafe extern "C" fn lua_rotate(L: *mut lua_State, idx: c_int, n: c_int) {
    let p: StkId;
    let t: StkId;
    let m: StkId;
    t = ((*L).top).offset(-(1 as c_int as isize));
    p = index2addr(L, idx);
    m = if n >= 0 as c_int {
        t.offset(-(n as isize))
    } else {
        p.offset(-(n as isize)).offset(-(1 as c_int as isize))
    };
    reverse(L, p, m);
    reverse(L, m.offset(1 as c_int as isize), t);
    reverse(L, p, t);
}

#[no_mangle]
pub unsafe extern "C" fn lua_copy(L: *mut lua_State, fromidx: c_int, toidx: c_int) {
    let fr: *mut TValue;
    let to: *mut TValue;
    fr = index2addr(L, fromidx);
    to = index2addr(L, toidx);
    let io1: *mut TValue = to;
    *io1 = *fr;
    if toidx < -(1000000 as c_int) - 1000 as c_int {
        if (*fr).tt_ & (1 as c_int) << 6 as c_int != 0
            && (*((*(*(*L).ci).func).value_.gc as *mut GCUnion))
                .cl
                .c
                .marked as c_int
                & (1 as c_int) << 2 as c_int
                != 0
            && (*(*fr).value_.gc).marked as c_int
                & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
                != 0
        {
            luaC_barrier_(
                L,
                &mut (*(&mut (*((*(*(*L).ci).func).value_.gc as *mut GCUnion)).cl.c
                    as *mut CClosure as *mut GCUnion))
                    .gc,
                (*fr).value_.gc,
            );
        } else {
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushvalue(L: *mut lua_State, idx: c_int) {
    setobj(L, (*L).top, index2addr(L, idx));
    api_incr_top(L);
}
/*
** access functions (stack -> C)
*/

#[no_mangle]
pub unsafe extern "C" fn lua_type(L: *mut lua_State, idx: c_int) -> c_int {
    let o: StkId = index2addr(L, idx);
    return if o != &luaO_nilobject_ as *const TValue as StkId {
        (*o).tt_ & 0xf as c_int
    } else {
        -(1 as c_int)
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_typename(_L: *mut lua_State, t: c_int) -> *const c_char {
    return luaT_typenames_[(t + 1 as c_int) as usize];
}

#[no_mangle]
pub unsafe extern "C" fn lua_iscfunction(L: *mut lua_State, idx: c_int) -> c_int {
    let o: StkId = index2addr(L, idx);
    return ((*o).tt_ == 6 as c_int | (1 as c_int) << 4 as c_int
        || (*o).tt_ == 6 as c_int | (2 as c_int) << 4 as c_int | (1 as c_int) << 6 as c_int)
        as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_isinteger(L: *mut lua_State, idx: c_int) -> c_int {
    let o: StkId = index2addr(L, idx);
    return ((*o).tt_ == 3 as c_int | (1 as c_int) << 4 as c_int) as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_isnumber(L: *mut lua_State, idx: c_int) -> c_int {
    let mut n: lua_Number = 0.;
    let o: *const TValue = index2addr(L, idx);
    return if (*o).tt_ == 3 as c_int | (0 as c_int) << 4 as c_int {
        1 as c_int
    } else {
        luaV_tonumber_(o, &mut n)
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_isstring(L: *mut lua_State, idx: c_int) -> c_int {
    let o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ & 0xf as c_int == 4 as c_int || (*o).tt_ & 0xf as c_int == 3 as c_int)
        as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_isuserdata(L: *mut lua_State, idx: c_int) -> c_int {
    let o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ == 7 as c_int | (1 as c_int) << 6 as c_int || (*o).tt_ == 2 as c_int)
        as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_rawequal(L: *mut lua_State, index1: c_int, index2: c_int) -> c_int {
    let o1: StkId = index2addr(L, index1);
    let o2: StkId = index2addr(L, index2);
    return if o1 != &luaO_nilobject_ as *const TValue as StkId
        && o2 != &luaO_nilobject_ as *const TValue as StkId
    {
        luaV_equalobj(
            0 as *mut lua_State,
            o1 as *const TValue,
            o2 as *const TValue,
        )
    } else {
        0 as c_int
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_arith(L: *mut lua_State, op: c_int) {
    if !(op != 12 as c_int && op != 13 as c_int) {
        let io1: *mut TValue = (*L).top;
        *io1 = *((*L).top).offset(-(1 as c_int as isize));
        let ref mut fresh9 = (*L).top;
        *fresh9 = (*fresh9).offset(1);
    }
    luaO_arith(
        L,
        op,
        ((*L).top).offset(-(2 as c_int as isize)) as *const TValue,
        ((*L).top).offset(-(1 as c_int as isize)) as *const TValue,
        ((*L).top).offset(-(2 as c_int as isize)),
    );
    let ref mut fresh10 = (*L).top;
    *fresh10 = (*fresh10).offset(-1);
}

#[no_mangle]
pub unsafe extern "C" fn lua_compare(
    L: *mut lua_State,
    index1: c_int,
    index2: c_int,
    op: c_int,
) -> c_int {
    let o1: StkId;
    let o2: StkId;
    let mut i: c_int = 0 as c_int;
    o1 = index2addr(L, index1);
    o2 = index2addr(L, index2);
    if o1 != &luaO_nilobject_ as *const TValue as StkId
        && o2 != &luaO_nilobject_ as *const TValue as StkId
    {
        match op {
            0 => {
                i = luaV_equalobj(L, o1 as *const TValue, o2 as *const TValue);
            }
            1 => {
                i = luaV_lessthan(L, o1 as *const TValue, o2 as *const TValue);
            }
            2 => {
                i = luaV_lessequal(L, o1 as *const TValue, o2 as *const TValue);
            }
            _ => {}
        }
    }
    return i;
}

#[no_mangle]
pub unsafe extern "C" fn lua_stringtonumber(L: *mut lua_State, s: *const c_char) -> size_t {
    let sz: size_t = luaO_str2num(s, (*L).top);
    if sz != 0 {
        let ref mut fresh11 = (*L).top;
        *fresh11 = (*fresh11).offset(1);
    }
    return sz;
}

#[no_mangle]
pub unsafe extern "C" fn lua_tonumberx(
    L: *mut lua_State,
    idx: c_int,
    pisnum: *mut c_int,
) -> lua_Number {
    let mut n: lua_Number = 0.;
    let o: *const TValue = index2addr(L, idx);
    let isnum: c_int = if (*o).tt_ == 3 as c_int | (0 as c_int) << 4 as c_int {
        n = (*o).value_.n;
        1 as c_int
    } else {
        luaV_tonumber_(o, &mut n)
    };
    if isnum == 0 {
        n = 0 as c_int as lua_Number;
    }
    if !pisnum.is_null() {
        *pisnum = isnum;
    }
    return n;
}

#[no_mangle]
pub unsafe extern "C" fn lua_tointegerx(
    L: *mut lua_State,
    idx: c_int,
    pisnum: *mut c_int,
) -> lua_Integer {
    let mut res: lua_Integer = 0;
    let o: *const TValue = index2addr(L, idx);
    let isnum: c_int = if (*o).tt_ == 3 as c_int | (1 as c_int) << 4 as c_int {
        res = (*o).value_.i;
        1 as c_int
    } else {
        luaV_tointeger(o, &mut res, 0 as c_int)
    };
    if isnum == 0 {
        res = 0 as c_int as lua_Integer;
    }
    if !pisnum.is_null() {
        *pisnum = isnum;
    }
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int {
    let o: *const TValue = index2addr(L, idx);
    return !((*o).tt_ == 0 as c_int || (*o).tt_ == 1 as c_int && (*o).value_.b == 0 as c_int)
        as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_tolstring(
    L: *mut lua_State,
    idx: c_int,
    len: *mut size_t,
) -> *const libc::c_char {
    let mut o: StkId = index2addr(L, idx);
    if !((*o).tt_ & 0xf as c_int == 4 as c_int) {
        if !((*o).tt_ & 0xf as c_int == 3 as c_int) {
            if !len.is_null() {
                *len = 0 as c_int as size_t;
            }
            return 0 as *const c_char;
        }
        luaO_tostring(L, o);
        if (*(*L).l_G).GCdebt > 0 {
            luaC_step(L);
        }
        o = index2addr(L, idx);
    }
    if !len.is_null() {
        *len = if (*((*o).value_.gc as *mut GCUnion)).ts.tt as c_int
            == 4 as c_int | (0 as c_int) << 4 as c_int
        {
            (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as size_t
        } else {
            (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen
        };
    }
    return (&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
}

#[no_mangle]
pub unsafe extern "C" fn lua_rawlen(L: *mut lua_State, idx: c_int) -> size_t {
    let o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3f as c_int {
        4 => return (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as size_t,
        20 => return (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen,
        7 => return (*((*o).value_.gc as *mut GCUnion)).u.len,
        5 => return luaH_getn(&mut (*((*o).value_.gc as *mut GCUnion)).h) as size_t,
        _ => return 0 as c_int as size_t,
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> lua_CFunction {
    let o: StkId = index2addr(L, idx);
    if (*o).tt_ == 6 as c_int | (1 as c_int) << 4 as c_int {
        return (*o).value_.f;
    } else if (*o).tt_ == 6 as c_int | (2 as c_int) << 4 as c_int | (1 as c_int) << 6 as c_int {
        return (*((*o).value_.gc as *mut GCUnion)).cl.c.f;
    } else {
        return None;
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut libc::c_void {
    let o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0xf as c_int {
        7 => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata as *mut libc::c_char)
                .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *mut libc::c_void;
        }
        2 => return (*o).value_.p,
        _ => return 0 as *mut c_void,
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_tothread(L: *mut lua_State, idx: c_int) -> *mut lua_State {
    let o: StkId = index2addr(L, idx);
    return if !((*o).tt_ == 8 as c_int | (1 as c_int) << 6 as c_int) {
        0 as *mut lua_State
    } else {
        &mut (*((*o).value_.gc as *mut GCUnion)).th
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_topointer(L: *mut lua_State, idx: c_int) -> *const c_void {
    let o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3f as c_int {
        5 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).h as *mut Table as *const c_void;
        }
        6 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.l as *mut LClosure as *const c_void;
        }
        38 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.c as *mut CClosure as *const c_void;
        }
        22 => {
            return ::std::mem::transmute::<lua_CFunction, size_t>((*o).value_.f) as *mut c_void;
        }
        8 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).th as *mut lua_State
                as *const libc::c_void;
        }
        7 => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata as *mut libc::c_char)
                .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *const libc::c_void;
        }
        2 => return (*o).value_.p,
        _ => return 0 as *const libc::c_void,
    };
}

/*
** push functions (C -> stack)
*/

#[no_mangle]
pub unsafe extern "C" fn lua_pushnil(L: *mut lua_State) {
    setnilvalue((*L).top);
    api_incr_top(L);
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushnumber(L: *mut lua_State, n: lua_Number) {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.n = n;
    (*io).tt_ = 3 as c_int | (0 as c_int) << 4 as c_int;
    let ref mut fresh13 = (*L).top;
    *fresh13 = (*fresh13).offset(1);
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) {
    setivalue((*L).top, n);
    api_incr_top(L);
}

/*
** Pushes on the stack a string with given length. Avoid using 's' when
** 'len' == 0 (as 's' can be NULL in that case), due to later use of
** 'memcmp' and 'memcpy'.
*/

/*
** Pushes on the stack a string with given length. Avoid using 's' when
** 'len' == 0 (as 's' can be NULL in that case), due to later use of
** 'memcmp' and 'memcpy'.
*/

#[no_mangle]
pub unsafe extern "C" fn lua_pushlstring(
    L: *mut lua_State,
    s: *const c_char,
    len: size_t,
) -> *const c_char {
    let ts: *mut TString;
    ts = if len == 0 {
        luaS_new(L, b"\0" as *const u8 as *const libc::c_char)
    } else {
        luaS_newlstr(L, s, len)
    };
    let mut io: *mut TValue = (*L).top;
    let x_: *mut TString = ts;
    let ref mut fresh15 = (*io).value_.gc;
    *fresh15 = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as c_int | (1 as c_int) << 6 as c_int;
    let ref mut fresh16 = (*L).top;
    *fresh16 = (*fresh16).offset(1);
    if (*(*L).l_G).GCdebt > 0 {
        luaC_step(L);
    }
    return (ts as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
) -> *const c_char {
    if s.is_null() {
        (*(*L).top).tt_ = 0 as c_int;
    } else {
        let ts: *mut TString;
        ts = luaS_new(L, s);
        let mut io: *mut TValue = (*L).top;
        let x_: *mut TString = ts;
        let ref mut fresh17 = (*io).value_.gc;
        *fresh17 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as c_int | (1 as c_int) << 6 as c_int;
        s = (ts as *mut c_char).offset(::std::mem::size_of::<UTString>() as c_ulong as isize);
    }
    let ref mut fresh18 = (*L).top;
    *fresh18 = (*fresh18).offset(1);
    if (*(*L).l_G).GCdebt > 0 {
        luaC_step(L);
    }
    return s;
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushvfstring(
    L: *mut lua_State,
    fmt: *const libc::c_char,
    mut argp: ::core::ffi::VaList,
) -> *const libc::c_char {
    let ret;
    ret = luaO_pushvfstring(L, fmt, argp.as_va_list());
    luaC_checkGC(L);
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushfstring(
    L: *mut lua_State,
    fmt: *const libc::c_char,
    args: ...
) -> *const libc::c_char {
    let ret;
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    ret = lua_pushvfstring(L, fmt, argp.as_va_list());
    luaC_checkGC(L);
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, mut n: c_int) {
    if n == 0 {
        let mut io: *mut TValue = (*L).top;
        let ref mut fresh19 = (*io).value_.f;
        *fresh19 = fn_0;
        (*io).tt_ = 6 as c_int | (1 as c_int) << 4 as c_int;
        let ref mut fresh20 = (*L).top;
        *fresh20 = (*fresh20).offset(1);
    } else {
        let cl: *mut CClosure;
        cl = luaF_newCclosure(L, n);
        let ref mut fresh21 = (*cl).f;
        *fresh21 = fn_0;
        let ref mut fresh22 = (*L).top;
        *fresh22 = (*fresh22).offset(-(n as isize));
        loop {
            let fresh23 = n;
            n = n - 1;
            if !(fresh23 != 0) {
                break;
            }
            let io1: *mut TValue =
                &mut *((*cl).upvalue).as_mut_ptr().offset(n as isize) as *mut TValue;
            *io1 = *((*L).top).offset(n as isize);
        }
        let mut io_0: *mut TValue = (*L).top;
        let x_: *mut CClosure = cl;
        let ref mut fresh24 = (*io_0).value_.gc;
        *fresh24 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io_0).tt_ = 6 as c_int | (2 as c_int) << 4 as c_int | (1 as c_int) << 6 as c_int;
        let ref mut fresh25 = (*L).top;
        *fresh25 = (*fresh25).offset(1);
        if (*(*L).l_G).GCdebt > 0 {
            luaC_step(L);
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushboolean(L: *mut lua_State, b: c_int) {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.b = (b != 0 as c_int) as c_int;
    (*io).tt_ = 1 as c_int;
    let ref mut fresh26 = (*L).top;
    *fresh26 = (*fresh26).offset(1);
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void) {
    let mut io: *mut TValue = (*L).top;
    let ref mut fresh27 = (*io).value_.p;
    *fresh27 = p;
    (*io).tt_ = 2 as c_int;
    let ref mut fresh28 = (*L).top;
    *fresh28 = (*fresh28).offset(1);
}

#[no_mangle]
pub unsafe extern "C" fn lua_pushthread(L: *mut lua_State) -> c_int {
    let mut io: *mut TValue = (*L).top;
    let x_: *mut lua_State = L;
    let ref mut fresh29 = (*io).value_.gc;
    *fresh29 = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 8 as c_int | (1 as c_int) << 6 as c_int;
    let ref mut fresh30 = (*L).top;
    *fresh30 = (*fresh30).offset(1);
    return ((*(*L).l_G).mainthread == L) as c_int;
}

/*
** get functions (Lua -> stack)
*/

unsafe extern "C" fn auxgetstr(
    L: *mut lua_State,
    t: *const TValue,
    k: *const libc::c_char,
) -> c_int {
    let slot: *const TValue;
    let str: *mut TString = luaS_new(L, k);
    if if !((*t).tt_ == 5 as c_int | (1 as c_int) << 6 as c_int) {
        slot = 0 as *const TValue;
        0 as c_int
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        !((*slot).tt_ == 0 as c_int) as c_int
    } != 0
    {
        let io1: *mut TValue = (*L).top;
        *io1 = *slot;
        let ref mut fresh31 = (*L).top;
        *fresh31 = (*fresh31).offset(1);
    } else {
        let mut io: *mut TValue = (*L).top;
        let x_: *mut TString = str;
        let ref mut fresh32 = (*io).value_.gc;
        *fresh32 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as c_int | (1 as c_int) << 6 as c_int;
        let ref mut fresh33 = (*L).top;
        *fresh33 = (*fresh33).offset(1);
        luaV_finishget(
            L,
            t,
            ((*L).top).offset(-(1 as c_int as isize)),
            ((*L).top).offset(-(1 as c_int as isize)),
            slot,
        );
    }
    return (*((*L).top).offset(-(1 as c_int as isize))).tt_ & 0xf as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_getglobal(L: *mut lua_State, name: *const libc::c_char) -> c_int {
    let reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
    return auxgetstr(L, luaH_getint(reg, 2 as c_int as lua_Integer), name);
}

#[no_mangle]
pub unsafe extern "C" fn lua_gettable(L: *mut lua_State, idx: c_int) -> c_int {
    let t: StkId;
    t = index2addr(L, idx);
    let slot: *const TValue;
    if if !((*t).tt_ == 5 as c_int | (1 as c_int) << 6 as c_int) {
        slot = 0 as *const TValue;
        0 as c_int
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            ((*L).top).offset(-(1 as c_int as isize)) as *const TValue,
        );
        !((*slot).tt_ == 0 as c_int) as c_int
    } != 0
    {
        let io1: *mut TValue = ((*L).top).offset(-(1 as c_int as isize));
        *io1 = *slot;
    } else {
        luaV_finishget(
            L,
            t as *const TValue,
            ((*L).top).offset(-(1 as c_int as isize)),
            ((*L).top).offset(-(1 as c_int as isize)),
            slot,
        );
    }
    return (*((*L).top).offset(-(1 as c_int as isize))).tt_ & 0xf as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_getfield(L: *mut lua_State, idx: c_int, k: *const c_char) -> c_int {
    return auxgetstr(L, index2addr(L, idx), k);
}

#[no_mangle]
pub unsafe extern "C" fn lua_geti(L: *mut lua_State, idx: c_int, n: lua_Integer) -> c_int {
    let t: StkId;
    let slot: *const TValue;
    t = index2addr(L, idx);
    if if !((*t).tt_ == 5 as c_int | (1 as c_int) << 6 as c_int) {
        slot = 0 as *const TValue;
        0 as c_int
    } else {
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        !((*slot).tt_ == 0 as c_int) as c_int
    } != 0
    {
        let io1: *mut TValue = (*L).top;
        *io1 = *slot;
        let ref mut fresh34 = (*L).top;
        *fresh34 = (*fresh34).offset(1);
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3 as c_int | (1 as c_int) << 4 as c_int;
        let ref mut fresh35 = (*L).top;
        *fresh35 = (*fresh35).offset(1);
        luaV_finishget(
            L,
            t as *const TValue,
            ((*L).top).offset(-(1 as c_int as isize)),
            ((*L).top).offset(-(1 as c_int as isize)),
            slot,
        );
    }
    return (*((*L).top).offset(-(1 as c_int as isize))).tt_ & 0xf as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawget(L: *mut lua_State, idx: c_int) -> c_int {
    let t: StkId;
    t = index2addr(L, idx);
    let io1: *mut TValue = ((*L).top).offset(-(1 as c_int as isize));
    *io1 = *luaH_get(
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        ((*L).top).offset(-(1 as c_int as isize)) as *const TValue,
    );
    return (*((*L).top).offset(-(1 as c_int as isize))).tt_ & 0xf as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_rawgeti(L: *mut lua_State, idx: c_int, n: lua_Integer) -> c_int {
    let t: StkId;
    t = index2addr(L, idx);
    let io1: *mut TValue = (*L).top;
    *io1 = *luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
    let ref mut fresh36 = (*L).top;
    *fresh36 = (*fresh36).offset(1);
    return (*((*L).top).offset(-(1 as c_int as isize))).tt_ & 0xf as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgetp(
    L: *mut lua_State,
    idx: c_int,
    p: *const libc::c_void,
) -> c_int {
    let t: StkId;
    let mut k: TValue = TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    t = index2addr(L, idx);
    let mut io: *mut TValue = &mut k;
    let ref mut fresh37 = (*io).value_.p;
    *fresh37 = p as *mut libc::c_void;
    (*io).tt_ = 2 as c_int;
    let io1: *mut TValue = (*L).top;
    *io1 = *luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, &mut k);
    let ref mut fresh38 = (*L).top;
    *fresh38 = (*fresh38).offset(1);
    return (*((*L).top).offset(-(1 as c_int as isize))).tt_ & 0xf as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_createtable(L: *mut lua_State, narray: c_int, nrec: c_int) {
    let t: *mut Table;
    t = luaH_new(L);
    let mut io: *mut TValue = (*L).top;
    let x_: *mut Table = t;
    let ref mut fresh39 = (*io).value_.gc;
    *fresh39 = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 5 as c_int | (1 as c_int) << 6 as c_int;
    let ref mut fresh40 = (*L).top;
    *fresh40 = (*fresh40).offset(1);
    if narray > 0 as c_int || nrec > 0 as c_int {
        luaH_resize(L, t, narray as c_uint, nrec as c_uint);
    }
    if (*(*L).l_G).GCdebt > 0 {
        luaC_step(L);
    }
}

#[no_mangle]
pub unsafe extern "C" fn lua_getmetatable(L: *mut lua_State, objindex: c_int) -> c_int {
    let obj: *const TValue;
    let mt: *mut Table;
    let mut res: c_int = 0 as c_int;
    obj = index2addr(L, objindex);
    match (*obj).tt_ & 0xf as c_int {
        5 => {
            mt = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
        }
        7 => {
            mt = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
        }
        _ => {
            mt = (*(*L).l_G).mt[((*obj).tt_ & 0xf as c_int) as usize];
        }
    }
    if !mt.is_null() {
        let mut io: *mut TValue = (*L).top;
        let x_: *mut Table = mt;
        let ref mut fresh41 = (*io).value_.gc;
        *fresh41 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = 5 as c_int | (1 as c_int) << 6 as c_int;
        let ref mut fresh42 = (*L).top;
        *fresh42 = (*fresh42).offset(1);
        res = 1 as c_int;
    }
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn lua_getuservalue(L: *mut lua_State, idx: c_int) -> c_int {
    let o: StkId;
    o = index2addr(L, idx);
    let io: *mut TValue = (*L).top;
    let iu: *const Udata = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*io).value_ = (*iu).user_;
    (*io).tt_ = (*iu).ttuv_ as c_int;
    let ref mut fresh43 = (*L).top;
    *fresh43 = (*fresh43).offset(1);
    return (*((*L).top).offset(-(1 as c_int as isize))).tt_ & 0xf as c_int;
}

/*
** set functions (stack -> Lua)
*/

/*
** t[k] = value at the top of the stack (where 'k' is a string)
*/
unsafe extern "C" fn auxsetstr(L: *mut lua_State, t: *const TValue, k: *const libc::c_char) {
    let slot: *const TValue;
    let str: *mut TString = luaS_new(L, k);
    if if !((*t).tt_ == 5 as c_int | (1 as c_int) << 6 as c_int) {
        slot = 0 as *const TValue;
        0 as c_int
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        if (*slot).tt_ == 0 as c_int {
            0 as c_int
        } else {
            if (*((*L).top).offset(-(1 as c_int as isize))).tt_ & (1 as c_int) << 6 as c_int != 0
                && (*((*t).value_.gc as *mut GCUnion)).h.marked as c_int
                    & (1 as c_int) << 2 as c_int
                    != 0
                && (*(*((*L).top).offset(-(1 as c_int as isize))).value_.gc).marked as c_int
                    & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
                    != 0
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *((*L).top).offset(-(1 as c_int as isize));
            1 as c_int
        }
    } != 0
    {
        let ref mut fresh44 = (*L).top;
        *fresh44 = (*fresh44).offset(-1);
    } else {
        let mut io: *mut TValue = (*L).top;
        let x_: *mut TString = str;
        let ref mut fresh45 = (*io).value_.gc;
        *fresh45 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as c_int | (1 as c_int) << 6 as c_int;
        let ref mut fresh46 = (*L).top;
        *fresh46 = (*fresh46).offset(1);
        luaV_finishset(
            L,
            t,
            ((*L).top).offset(-(1 as c_int as isize)),
            ((*L).top).offset(-(2 as c_int as isize)),
            slot,
        );
        let ref mut fresh47 = (*L).top;
        *fresh47 = (*fresh47).offset(-(2 as c_int as isize));
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char) {
    let reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
    auxsetstr(L, luaH_getint(reg, 2 as c_int as lua_Integer), name);
}

#[no_mangle]
pub unsafe extern "C" fn lua_settable(L: *mut lua_State, idx: c_int) {
    let t: StkId;
    t = index2addr(L, idx);
    let slot: *const TValue;
    if if !((*t).tt_ == 5 as c_int | (1 as c_int) << 6 as c_int) {
        slot = 0 as *const TValue;
        0 as c_int
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            ((*L).top).offset(-(2 as c_int as isize)) as *const TValue,
        );
        if (*slot).tt_ == 0 as c_int {
            0 as c_int
        } else {
            if (*((*L).top).offset(-(1 as c_int as isize))).tt_ & (1 as c_int) << 6 as c_int != 0
                && (*((*t).value_.gc as *mut GCUnion)).h.marked as c_int
                    & (1 as c_int) << 2 as c_int
                    != 0
                && (*(*((*L).top).offset(-(1 as c_int as isize))).value_.gc).marked as c_int
                    & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
                    != 0
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *((*L).top).offset(-(1 as c_int as isize));
            1 as c_int
        }
    } == 0
    {
        luaV_finishset(
            L,
            t as *const TValue,
            ((*L).top).offset(-(2 as c_int as isize)),
            ((*L).top).offset(-(1 as c_int as isize)),
            slot,
        );
    }
    let ref mut fresh48 = (*L).top;
    *fresh48 = (*fresh48).offset(-(2 as c_int as isize));
}

#[no_mangle]
pub unsafe extern "C" fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_char) {
    auxsetstr(L, index2addr(L, idx), k);
}

#[no_mangle]
pub unsafe extern "C" fn lua_seti(L: *mut lua_State, idx: c_int, n: lua_Integer) {
    let t: StkId;
    let slot: *const TValue;
    t = index2addr(L, idx);
    if if !((*t).tt_ == 5 as c_int | (1 as c_int) << 6 as c_int) {
        slot = 0 as *const TValue;
        0 as c_int
    } else {
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        if (*slot).tt_ == 0 as c_int {
            0 as c_int
        } else {
            if (*((*L).top).offset(-(1 as c_int as isize))).tt_ & (1 as c_int) << 6 as c_int != 0
                && (*((*t).value_.gc as *mut GCUnion)).h.marked as c_int
                    & (1 as c_int) << 2 as c_int
                    != 0
                && (*(*((*L).top).offset(-(1 as c_int as isize))).value_.gc).marked as c_int
                    & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
                    != 0
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *((*L).top).offset(-(1 as c_int as isize));
            1 as c_int
        }
    } != 0
    {
        let ref mut fresh49 = (*L).top;
        *fresh49 = (*fresh49).offset(-1);
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3 as c_int | (1 as c_int) << 4 as c_int;
        let ref mut fresh50 = (*L).top;
        *fresh50 = (*fresh50).offset(1);
        luaV_finishset(
            L,
            t as *const TValue,
            ((*L).top).offset(-(1 as c_int as isize)),
            ((*L).top).offset(-(2 as c_int as isize)),
            slot,
        );
        let ref mut fresh51 = (*L).top;
        *fresh51 = (*fresh51).offset(-(2 as c_int as isize));
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_rawset(L: *mut lua_State, idx: c_int) {
    let o: StkId;
    let slot: *mut TValue;
    o = index2addr(L, idx);
    slot = luaH_set(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        ((*L).top).offset(-(2 as c_int as isize)) as *const TValue,
    );
    *slot = *((*L).top).offset(-(1 as c_int as isize));
    (*((*o).value_.gc as *mut GCUnion)).h.flags = 0 as c_int as lu_byte;
    if (*((*L).top).offset(-(1 as c_int as isize))).tt_ & (1 as c_int) << 6 as c_int != 0
        && (*((*o).value_.gc as *mut GCUnion)).h.marked as c_int & (1 as c_int) << 2 as c_int != 0
        && (*(*((*L).top).offset(-(1 as c_int as isize))).value_.gc).marked as c_int
            & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
            != 0
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    let ref mut fresh52 = (*L).top;
    *fresh52 = (*fresh52).offset(-(2 as c_int as isize));
}

#[no_mangle]
pub unsafe extern "C" fn lua_rawseti(L: *mut lua_State, idx: c_int, n: lua_Integer) {
    let o: StkId;
    o = index2addr(L, idx);
    luaH_setint(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        n,
        ((*L).top).offset(-(1 as c_int as isize)),
    );
    if (*((*L).top).offset(-(1 as c_int as isize))).tt_ & (1 as c_int) << 6 as c_int != 0
        && (*((*o).value_.gc as *mut GCUnion)).h.marked as c_int & (1 as c_int) << 2 as c_int != 0
        && (*(*((*L).top).offset(-(1 as c_int as isize))).value_.gc).marked as c_int
            & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
            != 0
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    let ref mut fresh53 = (*L).top;
    *fresh53 = (*fresh53).offset(-1);
}

#[no_mangle]
pub unsafe extern "C" fn lua_rawsetp(L: *mut lua_State, idx: c_int, p: *const libc::c_void) {
    let o: StkId;
    let mut k: TValue = TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let slot: *mut TValue;
    o = index2addr(L, idx);
    let mut io: *mut TValue = &mut k;
    let ref mut fresh54 = (*io).value_.p;
    *fresh54 = p as *mut libc::c_void;
    (*io).tt_ = 2 as c_int;
    slot = luaH_set(L, &mut (*((*o).value_.gc as *mut GCUnion)).h, &mut k);
    *slot = *((*L).top).offset(-(1 as c_int as isize));
    if (*((*L).top).offset(-(1 as c_int as isize))).tt_ & (1 as c_int) << 6 as c_int != 0
        && (*((*o).value_.gc as *mut GCUnion)).h.marked as c_int & (1 as c_int) << 2 as c_int != 0
        && (*(*((*L).top).offset(-(1 as c_int as isize))).value_.gc).marked as c_int
            & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
            != 0
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    let ref mut fresh55 = (*L).top;
    *fresh55 = (*fresh55).offset(-1);
}

#[no_mangle]
pub unsafe extern "C" fn lua_setmetatable(L: *mut lua_State, objindex: c_int) -> c_int {
    let obj: *mut TValue;
    let mt: *mut Table;
    obj = index2addr(L, objindex);
    if (*((*L).top).offset(-(1 as c_int as isize))).tt_ == 0 as c_int {
        mt = 0 as *mut Table;
    } else {
        mt = &mut (*((*((*L).top).offset(-(1 as c_int as isize))).value_.gc as *mut GCUnion)).h;
    }
    match (*obj).tt_ & 0xf as c_int {
        5 => {
            let ref mut fresh56 = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
            *fresh56 = mt;
            if !mt.is_null() {
                if (*(*obj).value_.gc).marked as c_int & (1 as c_int) << 2 as c_int != 0
                    && (*mt).marked as c_int
                        & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
                        != 0
                {
                    luaC_barrier_(
                        L,
                        &mut (*((*obj).value_.gc as *mut GCUnion)).gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {
                };
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        7 => {
            let ref mut fresh57 = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
            *fresh57 = mt;
            if !mt.is_null() {
                if (*((*obj).value_.gc as *mut GCUnion)).u.marked as c_int
                    & (1 as c_int) << 2 as c_int
                    != 0
                    && (*mt).marked as c_int
                        & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
                        != 0
                {
                    luaC_barrier_(
                        L,
                        &mut (*(&mut (*((*obj).value_.gc as *mut GCUnion)).u as *mut Udata
                            as *mut GCUnion))
                            .gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {
                };
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        _ => {
            let ref mut fresh58 = (*(*L).l_G).mt[((*obj).tt_ & 0xf as c_int) as usize];
            *fresh58 = mt;
        }
    }
    let ref mut fresh59 = (*L).top;
    *fresh59 = (*fresh59).offset(-1);
    return 1 as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn lua_setuservalue(L: *mut lua_State, idx: c_int) {
    let o: StkId;
    o = index2addr(L, idx);
    let io: *const TValue = ((*L).top).offset(-(1 as c_int as isize)) as *const TValue;
    let mut iu: *mut Udata = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*iu).user_ = (*io).value_;
    (*iu).ttuv_ = (*io).tt_ as lu_byte;
    if (*((*L).top).offset(-(1 as c_int as isize))).tt_ & (1 as c_int) << 6 as c_int != 0
        && (*(*o).value_.gc).marked as c_int & (1 as c_int) << 2 as c_int != 0
        && (*(*((*L).top).offset(-(1 as c_int as isize))).value_.gc).marked as c_int
            & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
            != 0
    {
        luaC_barrier_(
            L,
            &mut (*((*o).value_.gc as *mut GCUnion)).gc,
            (*((*L).top).offset(-(1 as c_int as isize))).value_.gc,
        );
    } else {
    };
    let ref mut fresh60 = (*L).top;
    *fresh60 = (*fresh60).offset(-1);
}
/*
** 'load' and 'call' functions (run Lua code)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_callk(
    mut L: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    ctx: lua_KContext,
    k: lua_KFunction,
) {
    let func: StkId;
    func = ((*L).top).offset(-((nargs + 1 as c_int) as isize));
    if k.is_some() && (*L).nny as c_int == 0 as c_int {
        let ref mut fresh61 = (*(*L).ci).u.c.k;
        *fresh61 = k;
        (*(*L).ci).u.c.ctx = ctx;
        luaD_call(L, func, nresults);
    } else {
        luaD_callnoyield(L, func, nresults);
    }
    if nresults == -(1 as c_int) && (*(*L).ci).top < (*L).top {
        let ref mut fresh62 = (*(*L).ci).top;
        *fresh62 = (*L).top;
    }
}
/*
** Execute a protected call.
*/

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallS {
    pub func: StkId,
    pub nresults: c_int,
}

unsafe extern "C" fn f_call(L: *mut lua_State, ud: *mut c_void) {
    let c: *mut CallS = ud as *mut CallS;
    luaD_callnoyield(L, (*c).func, (*c).nresults);
}

#[no_mangle]
pub unsafe extern "C" fn lua_pcallk(
    mut L: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    errfunc: c_int,
    ctx: lua_KContext,
    k: lua_KFunction,
) -> c_int {
    let mut c: CallS = CallS {
        func: 0 as *mut TValue,
        nresults: 0,
    };
    let status: c_int;
    let func: ptrdiff_t;
    if errfunc == 0 as c_int {
        func = 0 as c_int as ptrdiff_t;
    } else {
        let o: StkId = index2addr(L, errfunc);
        func = (o as *mut libc::c_char).offset_from((*L).stack as *mut libc::c_char) as isize;
    }
    c.func = ((*L).top).offset(-((nargs + 1 as c_int) as isize));
    if k.is_none() || (*L).nny as c_int > 0 as c_int {
        c.nresults = nresults;
        status = luaD_pcall(
            L,
            Some(f_call as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
            &mut c as *mut CallS as *mut libc::c_void,
            (c.func as *mut libc::c_char).offset_from((*L).stack as *mut libc::c_char) as isize,
            func,
        );
    } else {
        let mut ci: *mut CallInfo = (*L).ci;
        let ref mut fresh63 = (*ci).u.c.k;
        *fresh63 = k;
        (*ci).u.c.ctx = ctx;
        (*ci).extra =
            (c.func as *mut libc::c_char).offset_from((*L).stack as *mut libc::c_char) as isize;
        (*ci).u.c.old_errfunc = (*L).errfunc;
        (*L).errfunc = func;
        (*ci).callstatus = ((*ci).callstatus as c_int & !((1 as c_int) << 0 as c_int)
            | (*L).allowhook as c_int) as libc::c_ushort;
        let ref mut fresh64 = (*ci).callstatus;
        *fresh64 = (*fresh64 as c_int | (1 as c_int) << 4 as c_int) as libc::c_ushort;
        luaD_call(L, c.func, nresults);
        let ref mut fresh65 = (*ci).callstatus;
        *fresh65 = (*fresh65 as c_int & !((1 as c_int) << 4 as c_int)) as libc::c_ushort;
        (*L).errfunc = (*ci).u.c.old_errfunc;
        status = 0 as c_int;
    }
    if nresults == -(1 as c_int) && (*(*L).ci).top < (*L).top {
        let ref mut fresh66 = (*(*L).ci).top;
        *fresh66 = (*L).top;
    }
    return status;
}

#[no_mangle]
pub unsafe extern "C" fn lua_load(
    L: *mut lua_State,
    reader: lua_Reader,
    data: *mut libc::c_void,
    mut chunkname: *const libc::c_char,
    mode: *const libc::c_char,
) -> c_int {
    let mut z: ZIO = ZIO {
        n: 0,
        p: 0 as *const libc::c_char,
        reader: None,
        data: 0 as *mut libc::c_void,
        L: 0 as *mut lua_State,
    };
    let status: c_int;
    if chunkname.is_null() {
        chunkname = b"?\0" as *const u8 as *const libc::c_char;
    }
    luaZ_init(L, &mut z, reader, data);
    status = luaD_protectedparser(L, &mut z, chunkname, mode);
    if status == 0 as c_int {
        let f: *mut LClosure = &mut (*((*((*L).top).offset(-(1 as c_int as isize))).value_.gc
            as *mut GCUnion))
            .cl
            .l;
        if (*f).nupvalues as c_int >= 1 as c_int {
            let reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
            let gt: *const TValue = luaH_getint(reg, 2 as c_int as lua_Integer);
            let io1: *mut TValue = (**((*f).upvals).as_mut_ptr().offset(0 as c_int as isize)).v;
            *io1 = *gt;
            if (*(**((*f).upvals).as_mut_ptr().offset(0 as c_int as isize)).v).tt_
                & (1 as c_int) << 6 as c_int
                != 0
                && !((**((*f).upvals).as_mut_ptr().offset(0 as c_int as isize)).v
                    != &mut (**((*f).upvals).as_mut_ptr().offset(0 as c_int as isize))
                        .u
                        .value as *mut TValue)
            {
                luaC_upvalbarrier_(L, *((*f).upvals).as_mut_ptr().offset(0 as c_int as isize));
            } else {
            };
        }
    }
    return status;
}

#[no_mangle]
pub unsafe extern "C" fn lua_dump(
    L: *mut lua_State,
    writer: lua_Writer,
    data: *mut libc::c_void,
    strip: c_int,
) -> c_int {
    let status: c_int;
    let o: *mut TValue;
    o = ((*L).top).offset(-(1 as c_int as isize));
    if (*o).tt_ == 6 as c_int | (0 as c_int) << 4 as c_int | (1 as c_int) << 6 as c_int {
        status = luaU_dump(
            L,
            (*((*o).value_.gc as *mut GCUnion)).cl.l.p,
            writer,
            data,
            strip,
        );
    } else {
        status = 1 as c_int;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_status(L: *mut lua_State) -> c_int {
    return (*L).status as c_int;
}

/*
** Garbage-collection function
*/

#[no_mangle]
pub unsafe extern "C" fn lua_gc(L: *mut lua_State, what: c_int, mut data: c_int) -> c_int {
    let mut res: c_int = 0 as c_int;
    let g: *mut global_State;
    g = (*L).l_G;
    match what {
        0 => {
            (*g).gcrunning = 0 as c_int as lu_byte;
        }
        1 => {
            luaE_setdebt(g, 0 as c_int as l_mem);
            (*g).gcrunning = 1 as c_int as lu_byte;
        }
        2 => {
            luaC_fullgc(L, 0 as c_int);
        }
        3 => {
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem >> 10 as c_int) as c_int;
        }
        4 => {
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem & 0x3ff as usize) as c_int;
        }
        5 => {
            let mut debt: l_mem = 1 as c_int as l_mem;
            let oldrunning: lu_byte = (*g).gcrunning;
            (*g).gcrunning = 1 as c_int as lu_byte;
            if data == 0 as c_int {
                luaE_setdebt(
                    g,
                    -((100 as c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<TString>() as libc::c_ulong)
                        as c_int) as l_mem,
                );
                luaC_step(L);
            } else {
                debt = data as l_mem * 1024 as isize + (*g).GCdebt;
                luaE_setdebt(g, debt);
                if (*(*L).l_G).GCdebt > 0 {
                    luaC_step(L);
                }
            }
            (*g).gcrunning = oldrunning;
            if debt > 0 && (*g).gcstate as c_int == 7 as c_int {
                res = 1 as c_int;
            }
        }
        6 => {
            res = (*g).gcpause;
            (*g).gcpause = data;
        }
        7 => {
            res = (*g).gcstepmul;
            if data < 40 as c_int {
                data = 40 as c_int;
            }
            (*g).gcstepmul = data;
        }
        9 => {
            res = (*g).gcrunning as c_int;
        }
        _ => {
            res = -(1 as c_int);
        }
    }
    return res;
}

/*
** miscellaneous functions
*/

#[no_mangle]
pub unsafe extern "C" fn lua_error(L: *mut lua_State) -> ! {
    luaG_errormsg(L);
}

#[no_mangle]
pub unsafe extern "C" fn lua_next(L: *mut lua_State, idx: c_int) -> c_int {
    let t: StkId;
    let more: c_int;
    t = index2addr(L, idx);
    more = luaH_next(
        L,
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        ((*L).top).offset(-(1 as c_int as isize)),
    );
    if more != 0 {
        let ref mut fresh67 = (*L).top;
        *fresh67 = (*fresh67).offset(1);
    } else {
        let ref mut fresh68 = (*L).top;
        *fresh68 = (*fresh68).offset(-(1 as c_int as isize));
    }
    return more;
}

#[no_mangle]
pub unsafe extern "C" fn lua_concat(L: *mut lua_State, n: c_int) {
    if n >= 2 as c_int {
        luaV_concat(L, n);
    } else if n == 0 as c_int {
        let mut io: *mut TValue = (*L).top;
        let x_: *mut TString = luaS_newlstr(
            L,
            b"\0" as *const u8 as *const libc::c_char,
            0 as c_int as size_t,
        );
        let ref mut fresh69 = (*io).value_.gc;
        *fresh69 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as c_int | (1 as c_int) << 6 as c_int;
        let ref mut fresh70 = (*L).top;
        *fresh70 = (*fresh70).offset(1);
    }
    if (*(*L).l_G).GCdebt > 0 {
        luaC_step(L);
    }
}

#[no_mangle]
pub unsafe extern "C" fn lua_len(L: *mut lua_State, idx: c_int) {
    let t: StkId;
    t = index2addr(L, idx);
    luaV_objlen(L, (*L).top, t as *const TValue);
    let ref mut fresh71 = (*L).top;
    *fresh71 = (*fresh71).offset(1);
}

#[no_mangle]
pub unsafe extern "C" fn lua_getallocf(L: *mut lua_State, ud: *mut *mut libc::c_void) -> lua_Alloc {
    let f: lua_Alloc;
    if !ud.is_null() {
        *ud = (*(*L).l_G).ud;
    }
    f = (*(*L).l_G).frealloc;
    return f;
}

#[no_mangle]
pub unsafe extern "C" fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut libc::c_void) {
    let ref mut fresh72 = (*(*L).l_G).ud;
    *fresh72 = ud;
    let ref mut fresh73 = (*(*L).l_G).frealloc;
    *fresh73 = f;
}

#[no_mangle]
pub unsafe extern "C" fn lua_newuserdata(L: *mut lua_State, size: size_t) -> *mut libc::c_void {
    let u: *mut Udata;
    u = luaS_newudata(L, size);
    let mut io: *mut TValue = (*L).top;
    let x_: *mut Udata = u;
    let ref mut fresh74 = (*io).value_.gc;
    *fresh74 = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 7 as c_int | (1 as c_int) << 6 as c_int;
    let ref mut fresh75 = (*L).top;
    *fresh75 = (*fresh75).offset(1);
    if (*(*L).l_G).GCdebt > 0 {
        luaC_step(L);
    }
    return (u as *mut libc::c_char)
        .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}

unsafe extern "C" fn aux_upvalue(
    fi: StkId,
    n: c_int,
    val: *mut *mut TValue,
    owner: *mut *mut CClosure,
    uv: *mut *mut UpVal,
) -> *const libc::c_char {
    match (*fi).tt_ & 0x3f as c_int {
        38 => {
            let f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            if !(1 as c_int <= n && n <= (*f).nupvalues as c_int) {
                return 0 as *const libc::c_char;
            }
            *val = &mut *((*f).upvalue)
                .as_mut_ptr()
                .offset((n - 1 as c_int) as isize) as *mut TValue;
            if !owner.is_null() {
                *owner = f;
            }
            return b"\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            let f_0: *mut LClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
            let name: *mut TString;
            let p: *mut Proto = (*f_0).p;
            if !(1 as c_int <= n && n <= (*p).sizeupvalues) {
                return 0 as *const libc::c_char;
            }
            *val = (**((*f_0).upvals)
                .as_mut_ptr()
                .offset((n - 1 as c_int) as isize))
            .v;
            if !uv.is_null() {
                *uv = *((*f_0).upvals)
                    .as_mut_ptr()
                    .offset((n - 1 as c_int) as isize);
            }
            name = (*((*p).upvalues).offset((n - 1 as c_int) as isize)).name;
            return if name.is_null() {
                b"(*no name)\0" as *const u8 as *const libc::c_char
            } else {
                (name as *mut libc::c_char)
                    .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                    as *const libc::c_char
            };
        }
        _ => return 0 as *const libc::c_char,
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_getupvalue(
    L: *mut lua_State,
    funcindex: c_int,
    n: c_int,
) -> *const libc::c_char {
    let name: *const c_char;
    let mut val: *mut TValue = 0 as *mut TValue;
    name = aux_upvalue(
        index2addr(L, funcindex),
        n,
        &mut val,
        0 as *mut *mut CClosure,
        0 as *mut *mut UpVal,
    );
    if !name.is_null() {
        let io1: *mut TValue = (*L).top;
        *io1 = *val;
        let ref mut fresh76 = (*L).top;
        *fresh76 = (*fresh76).offset(1);
    }
    return name;
}

#[no_mangle]
pub unsafe extern "C" fn lua_setupvalue(
    L: *mut lua_State,
    funcindex: c_int,
    n: c_int,
) -> *const libc::c_char {
    let name: *const c_char;
    let mut val: *mut TValue = 0 as *mut TValue;
    let mut owner: *mut CClosure = 0 as *mut CClosure;
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    let fi: StkId;
    fi = index2addr(L, funcindex);
    name = aux_upvalue(fi, n, &mut val, &mut owner, &mut uv);
    if !name.is_null() {
        let ref mut fresh77 = (*L).top;
        *fresh77 = (*fresh77).offset(-1);
        let io1: *mut TValue = val;
        *io1 = *(*L).top;
        if !owner.is_null() {
            if (*(*L).top).tt_ & (1 as c_int) << 6 as c_int != 0
                && (*owner).marked as c_int & (1 as c_int) << 2 as c_int != 0
                && (*(*(*L).top).value_.gc).marked as c_int
                    & ((1 as c_int) << 0 as c_int | (1 as c_int) << 1 as c_int)
                    != 0
            {
                luaC_barrier_(L, &mut (*(owner as *mut GCUnion)).gc, (*(*L).top).value_.gc);
            } else {
            };
        } else if !uv.is_null() {
            if (*(*uv).v).tt_ & (1 as c_int) << 6 as c_int != 0
                && !((*uv).v != &mut (*uv).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, uv);
            } else {
            };
        }
    }
    return name;
}

unsafe extern "C" fn getupvalref(L: *mut lua_State, fidx: c_int, n: c_int) -> *mut *mut UpVal {
    let f: *mut LClosure;
    let fi: StkId = index2addr(L, fidx);
    f = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
    return &mut *((*f).upvals).as_mut_ptr().offset((n - 1 as c_int) as isize) as *mut *mut UpVal;
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvalueid(
    L: *mut lua_State,
    fidx: c_int,
    n: c_int,
) -> *mut libc::c_void {
    let fi: StkId = index2addr(L, fidx);
    match (*fi).tt_ & 0x3f as c_int {
        6 => return *getupvalref(L, fidx, n) as *mut libc::c_void,
        38 => {
            let f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            return &mut *((*f).upvalue)
                .as_mut_ptr()
                .offset((n - 1 as c_int) as isize) as *mut TValue
                as *mut libc::c_void;
        }
        _ => return 0 as *mut libc::c_void,
    };
}

#[no_mangle]
pub unsafe extern "C" fn lua_upvaluejoin(
    L: *mut lua_State,
    fidx1: c_int,
    n1: c_int,
    fidx2: c_int,
    n2: c_int,
) {
    let mut up1: *mut *mut UpVal = getupvalref(L, fidx1, n1);
    let up2: *mut *mut UpVal = getupvalref(L, fidx2, n2);
    if *up1 == *up2 {
        return;
    }
    luaC_upvdeccount(L, *up1);
    *up1 = *up2;
    let ref mut fresh78 = (**up1).refcount;
    *fresh78 = (*fresh78).wrapping_add(1);
    if (**up1).v != &mut (**up1).u.value as *mut TValue {
        (**up1).u.open.touched = 1 as c_int;
    }
    if (*(**up1).v).tt_ & (1 as c_int) << 6 as c_int != 0
        && !((**up1).v != &mut (**up1).u.value as *mut TValue)
    {
        luaC_upvalbarrier_(L, *up1);
    } else {
    };
}

extern "C" {
    pub fn luaC_step(L: *mut lua_State);
    pub fn luaV_lessthan(L: *mut lua_State, l: *const TValue, r: *const TValue) -> c_int;
    pub fn luaV_lessequal(L: *mut lua_State, l: *const TValue, r: *const TValue) -> c_int;
    pub fn luaV_finishget(
        L: *mut lua_State,
        t: *const TValue,
        key: *mut TValue,
        val: StkId,
        slot: *const TValue,
    );
    pub fn luaC_checkfinalizer(L: *mut lua_State, o: *mut GCObject, mt: *mut Table);
    pub fn luaV_finishset(
        L: *mut lua_State,
        t: *const TValue,
        key: *mut TValue,
        val: StkId,
        slot: *const TValue,
    );
    pub fn luaC_upvdeccount(L: *mut lua_State, uv: *mut UpVal);
    pub fn luaV_objlen(L: *mut lua_State, ra: StkId, rb: *const TValue);
}
