/*
** Tag methods
*/

use std::mem;
use std::ptr;

use libc::{c_char, c_int, c_uint};

use crate::ldebug::{luaG_concaterror, luaG_opinterror, luaG_tointerror};
use crate::ldo::{luaD_call, luaD_callnoyield, restorestack, savestack};
use crate::lgc::luaC_fix;
use crate::llimits::lu_byte;
use crate::lobject::{
    getstr, hvalue, l_isfalse, luaO_nilobject_, setobj, tsvalue, ttisfulluserdata, ttisnil,
    ttisstring, ttistable, ttnov, uvalue, StkId, TString, TValue, Table, LUA_TOTALTAGS,
};
use crate::lstate::{isLua, lua_State};
use crate::lvm::tonumber;
use crate::types::{LUA_TTABLE, LUA_TUSERDATA};

/*
* WARNING: if you change the order of this enumeration,
* grep "ORDER TM" and "ORDER OP"
*/
pub type TMS = c_uint;

pub const TM_INDEX: TMS = 0;
pub const TM_NEWINDEX: TMS = 1;
pub const TM_GC: TMS = 2;
pub const TM_MODE: TMS = 3;
pub const TM_LEN: TMS = 4;
pub const TM_EQ: TMS = 5; /* last tag method with fast access */
pub const TM_ADD: TMS = 6;
pub const TM_SUB: TMS = 7;
pub const TM_MUL: TMS = 8;
pub const TM_MOD: TMS = 9;
pub const TM_POW: TMS = 10;
pub const TM_DIV: TMS = 11;
pub const TM_IDIV: TMS = 12;
pub const TM_BAND: TMS = 13;
pub const TM_BOR: TMS = 14;
pub const TM_BXOR: TMS = 15;
pub const TM_SHL: TMS = 16;
pub const TM_SHR: TMS = 17;
pub const TM_UNM: TMS = 18;
pub const TM_BNOT: TMS = 19;
pub const TM_LT: TMS = 20;
pub const TM_LE: TMS = 21;
pub const TM_CONCAT: TMS = 22;
pub const TM_CALL: TMS = 23;
pub const TM_N: usize = 24; /* number of elements in the enum */

// #define gfasttm(g,et,e) ((et) == NULL ? NULL : \
//   ((et)->flags & (1u<<(e))) ? NULL : luaT_gettm(et, e, (g)->tmname[e]))

// #define fasttm(l,et,e)	gfasttm(G(l), et, e)

unsafe fn ttypename(i: usize) -> *const c_char {
    luaT_typenames_[i + 1]
}

extern "C" {
    fn luaS_new(L: *mut lua_State, str: *const c_char) -> *mut TString;
    fn luaH_getshortstr(t: *mut Table, key: *mut TString) -> *const TValue;
}

static mut udatatypename: [c_char; 9] =
    unsafe { *mem::transmute::<&[u8; 9], &[c_char; 9]>(b"userdata\0") };

#[no_mangle]
pub static mut luaT_typenames_: [*const c_char; LUA_TOTALTAGS] = unsafe {
    [
        b"no value\0" as *const u8 as *const c_char,
        b"nil\0" as *const u8 as *const c_char,
        b"boolean\0" as *const u8 as *const c_char,
        udatatypename.as_ptr(),
        b"number\0" as *const u8 as *const c_char,
        b"string\0" as *const u8 as *const c_char,
        b"table\0" as *const u8 as *const c_char,
        b"function\0" as *const u8 as *const c_char,
        udatatypename.as_ptr(),
        b"thread\0" as *const u8 as *const c_char,
        b"proto\0" as *const u8 as *const c_char, /* this last case is used for tests only */
    ]
};

#[no_mangle]
pub unsafe extern "C" fn luaT_init(L: *mut lua_State) {
    static mut luaT_eventname: [*const c_char; 24] = [
        /* ORDER TM */
        b"__index\0" as *const u8 as *const c_char,
        b"__newindex\0" as *const u8 as *const c_char,
        b"__gc\0" as *const u8 as *const c_char,
        b"__mode\0" as *const u8 as *const c_char,
        b"__len\0" as *const u8 as *const c_char,
        b"__eq\0" as *const u8 as *const c_char,
        b"__add\0" as *const u8 as *const c_char,
        b"__sub\0" as *const u8 as *const c_char,
        b"__mul\0" as *const u8 as *const c_char,
        b"__mod\0" as *const u8 as *const c_char,
        b"__pow\0" as *const u8 as *const c_char,
        b"__div\0" as *const u8 as *const c_char,
        b"__idiv\0" as *const u8 as *const c_char,
        b"__band\0" as *const u8 as *const c_char,
        b"__bor\0" as *const u8 as *const c_char,
        b"__bxor\0" as *const u8 as *const c_char,
        b"__shl\0" as *const u8 as *const c_char,
        b"__shr\0" as *const u8 as *const c_char,
        b"__unm\0" as *const u8 as *const c_char,
        b"__bnot\0" as *const u8 as *const c_char,
        b"__lt\0" as *const u8 as *const c_char,
        b"__le\0" as *const u8 as *const c_char,
        b"__concat\0" as *const u8 as *const c_char,
        b"__call\0" as *const u8 as *const c_char,
    ];
    let mut i = 0;
    while i < TM_N {
        (*(*L).l_G).tmname[i] = luaS_new(L, luaT_eventname[i]);
        luaC_fix(L, obj2gco!((*(*L).l_G).tmname[i])); /* never collect these names */
        i += 1;
    }
}

/*
** function to be used with macro "fasttm": optimized for absence of
** tag methods
*/
#[no_mangle]
pub unsafe extern "C" fn luaT_gettm(
    events: *mut Table,
    event: TMS,
    ename: *mut TString,
) -> *const TValue {
    let tm = luaH_getshortstr(events, ename);
    debug_assert!(event <= TM_EQ);
    if ttisnil(tm) {
        /* no tag method? */
        (*events).flags |= (1 as lu_byte) << event as lu_byte; /* cache this fact */
        return ptr::null();
    }
    return tm;
}

#[no_mangle]
pub unsafe extern "C" fn luaT_gettmbyobj(
    L: *mut lua_State,
    o: *const TValue,
    event: TMS,
) -> *const TValue {
    let mt;
    match ttnov(o) {
        LUA_TTABLE => {
            mt = (*hvalue(o)).metatable;
        }
        LUA_TUSERDATA => {
            mt = (*uvalue(o)).metatable;
        }
        _ => {
            mt = (*(*L).l_G).mt[ttnov(o) as usize];
        }
    }
    return if !mt.is_null() {
        luaH_getshortstr(mt, (*(*L).l_G).tmname[event as usize])
    } else {
        &luaO_nilobject_
    };
}

/*
** Return the name of the type of an object. For tables and userdata
** with metatable, use their '__name' metafield, if present.
*/
#[no_mangle]
pub unsafe extern "C" fn luaT_objtypename(L: *mut lua_State, o: *const TValue) -> *const c_char {
    let mut mt = ptr::null_mut();
    if ttistable(o) {
        mt = (*hvalue(o)).metatable;
    } else if ttisfulluserdata(o) {
        mt = (*uvalue(o)).metatable;
    }
    if !mt.is_null() {
        let name = luaH_getshortstr(mt, luaS_new(L, b"__name\0" as *const u8 as *const c_char));
        if ttisstring(name) {
            /* is '__name' a string? */
            return getstr(tsvalue(name)); /* use it as type name */
        }
    }
    return ttypename(ttnov(o) as usize); /* else use standard type name */
}

#[no_mangle]
pub unsafe extern "C" fn luaT_callTM(
    L: *mut lua_State,
    f: *const TValue,
    p1: *const TValue,
    p2: *const TValue,
    mut p3: *mut TValue,
    hasres: c_int,
) {
    let result = savestack(L, p3);
    let func = (*L).top;
    setobj(L, func, f); /* push function (assume EXTRA_STACK) */
    setobj(L, func.add(1), p1); /* 1st argument */
    setobj(L, func.add(2), p2); /* 2nd argument */
    (*L).top = ((*L).top).add(3);
    if hasres == 0 {
        /* no result? 'p3' is third argument */
        setobj(L, (*L).top, p3); /* 3rd argument */
        (*L).top = (*L).top.add(1);
    }
    /* metamethod may yield only when called from Lua code */
    if isLua((*L).ci) {
        luaD_call(L, func, hasres);
    } else {
        luaD_callnoyield(L, func, hasres);
    }
    if hasres != 0 {
        /* if has result, move it to its place */
        p3 = restorestack(L, result);
        (*L).top = (*L).top.sub(1);
        setobj(L, p3, (*L).top);
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaT_callbinTM(
    L: *mut lua_State,
    p1: *const TValue,
    p2: *const TValue,
    res: StkId,
    event: TMS,
) -> c_int {
    let mut tm = luaT_gettmbyobj(L, p1, event); /* try first operand */
    if ttisnil(tm) {
        tm = luaT_gettmbyobj(L, p2, event); /* try second operand */
    }
    if ttisnil(tm) {
        return 0;
    }
    luaT_callTM(L, tm, p1, p2, res, 1);
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn luaT_trybinTM(
    L: *mut lua_State,
    p1: *const TValue,
    p2: *const TValue,
    res: StkId,
    event: TMS,
) {
    if luaT_callbinTM(L, p1, p2, res, event) == 0 {
        match event {
            TM_CONCAT => {
                luaG_concaterror(L, p1, p2);
            }
            TM_BAND | TM_BOR | TM_BXOR | TM_SHL | TM_SHR | TM_BNOT => {
                let mut dummy = 0.;
                if tonumber(p1, &mut dummy) != 0 && tonumber(p2, &mut dummy) != 0 {
                    luaG_tointerror(L, p1, p2);
                } else {
                    luaG_opinterror(
                        L,
                        p1,
                        p2,
                        b"perform bitwise operation on\0" as *const u8 as *const c_char,
                    );
                }
            }
            _ => {
                luaG_opinterror(
                    L,
                    p1,
                    p2,
                    b"perform arithmetic on\0" as *const u8 as *const c_char,
                );
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaT_callorderTM(
    L: *mut lua_State,
    p1: *const TValue,
    p2: *const TValue,
    event: TMS,
) -> c_int {
    if luaT_callbinTM(L, p1, p2, (*L).top, event) == 0 {
        return -1; /* no metamethod */
    } else {
        return !l_isfalse((*L).top) as c_int;
    };
}
