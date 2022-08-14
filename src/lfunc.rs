/*
** Auxiliary functions to manipulate prototypes and closures
*/

use std::mem::size_of;
use std::ptr;

use libc::{c_char, c_int, size_t};

use crate::lgc::luaC_upvalbarrier;
use crate::llimits::{lu_byte, lu_mem};
use crate::lmem::{luaM_free, luaM_freearray, luaM_new};
use crate::lobject::{
    getstr, setnilvalue, setobj, CClosure, GCObject, LClosure, Proto, StkId, TValue, LUA_TCCL,
    LUA_TLCL, LUA_TPROTO,
};
use crate::lstate::{gco2ccl, gco2lcl, gco2p, lua_State};

pub const fn sizeCclosure(n: c_int) -> size_t {
    (size_of::<CClosure>() as c_int + size_of::<TValue>() as c_int * (n - 1)) as size_t
}

pub const fn sizeLclosure(n: c_int) -> size_t {
    (size_of::<LClosure>() as c_int + size_of::<*const TValue>() as c_int * (n - 1)) as size_t
}

/* test whether thread is in 'twups' list */
#[inline(always)]
pub unsafe fn isintwups(L: *mut lua_State) -> bool {
    (*L).twups != L
}

/*
** maximum number of upvalues in a closure (both C and Lua). (Value
** must fit in a VM register.)
*/
pub const MAXUPVAL: usize = 255;

/*
** Upvalues for Lua closures
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub v: *mut TValue,   /* points to stack or to its own value */
    pub refcount: lu_mem, /* reference counter */
    pub u: C2RustUnnamed_3,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub open: C2RustUnnamed_4, /* (when open) */
    pub value: TValue,         /* the value (when closed) */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub next: *mut UpVal, /* linked list */
    pub touched: c_int,   /* mark to avoid cycles with dead threads */
}

#[inline(always)]
pub unsafe fn upisopen(up: *mut UpVal) -> bool {
    (*up).v != &mut (*up).u.value as *mut _
}

extern "C" {
    pub fn luaC_newobj(L: *mut lua_State, tt: c_int, sz: size_t) -> *mut GCObject;
}

#[no_mangle]
pub unsafe extern "C" fn luaF_newCclosure(L: *mut lua_State, n: c_int) -> *mut CClosure {
    let o = luaC_newobj(L, LUA_TCCL, sizeCclosure(n));
    let mut c: *mut CClosure = gco2ccl(o);
    (*c).nupvalues = n as lu_byte;
    return c;
}

#[no_mangle]
pub unsafe extern "C" fn luaF_newLclosure(L: *mut lua_State, mut n: c_int) -> *mut LClosure {
    let o = luaC_newobj(L, LUA_TLCL, sizeLclosure(n));
    let mut c: *mut LClosure = gco2lcl(o);
    (*c).p = ptr::null_mut();
    (*c).nupvalues = n as lu_byte;
    while n != 0 {
        n -= 1;
        *((*c).upvals).as_mut_ptr().offset(n as isize) = ptr::null_mut();
    }
    return c;
}

/*
** fill a closure with new closed upvalues
*/
#[no_mangle]
pub unsafe extern "C" fn luaF_initupvals(L: *mut lua_State, cl: *mut LClosure) {
    let mut i = 0;
    while i < (*cl).nupvalues {
        let uv = luaM_new::<UpVal>(L);
        (*uv).refcount = 1;
        (*uv).v = &mut (*uv).u.value; /* make it closed */
        setnilvalue((*uv).v);
        *((*cl).upvals).as_mut_ptr().offset(i as isize) = uv;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaF_findupval(L: *mut lua_State, level: StkId) -> *mut UpVal {
    let mut pp: *mut *mut UpVal = &mut (*L).openupval;
    let mut p: *mut UpVal;
    debug_assert!(isintwups(L) || (*L).openupval.is_null());
    while !(*pp).is_null() && (**pp).v >= level {
        p = *pp;
        debug_assert!(upisopen(p));
        if (*p).v == level {
            /* found a corresponding upvalue? */
            return p;
        }
        pp = &mut (*p).u.open.next;
    }
    /* not found: create a new upvalue */
    let uv = luaM_new::<UpVal>(L);
    (*uv).refcount = 0;
    (*uv).u.open.next = *pp; /* link it to list of open upvalues */
    (*uv).u.open.touched = 1;
    *pp = uv;
    (*uv).v = level; /* current value lives in the stack */
    if !isintwups(L) {
        /* thread not in list of threads with upvalues? */
        (*L).twups = (*(*L).l_G).twups; /* link it to the list */
        (*(*L).l_G).twups = L;
    }
    return uv;
}

#[no_mangle]
pub unsafe extern "C" fn luaF_close(L: *mut lua_State, level: StkId) {
    let mut uv;
    while !((*L).openupval).is_null() && (*(*L).openupval).v >= level {
        uv = (*L).openupval;
        debug_assert!(upisopen(uv));
        (*L).openupval = (*uv).u.open.next; /* remove from 'open' list */
        if (*uv).refcount == 0 {
            /* no references? */
            luaM_free(L, uv); /* free upvalue */
        } else {
            setobj(L, &mut (*uv).u.value, (*uv).v); /* move value to upvalue slot */
            (*uv).v = &mut (*uv).u.value; /* now current value lives here */
            luaC_upvalbarrier(L, uv);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaF_newproto(L: *mut lua_State) -> *mut Proto {
    let o = luaC_newobj(L, LUA_TPROTO, size_of::<Proto>());
    let f: *mut Proto = gco2p(o);
    (*f).k = ptr::null_mut();
    (*f).sizek = 0;
    (*f).p = ptr::null_mut();
    (*f).sizep = 0;
    (*f).code = ptr::null_mut();
    (*f).cache = ptr::null_mut();
    (*f).sizecode = 0;
    (*f).lineinfo = ptr::null_mut();
    (*f).sizelineinfo = 0;
    (*f).upvalues = ptr::null_mut();
    (*f).sizeupvalues = 0;
    (*f).numparams = 0;
    (*f).is_vararg = 0;
    (*f).maxstacksize = 0;
    (*f).locvars = ptr::null_mut();
    (*f).sizelocvars = 0;
    (*f).linedefined = 0;
    (*f).lastlinedefined = 0;
    (*f).source = ptr::null_mut();
    return f;
}

#[no_mangle]
pub unsafe extern "C" fn luaF_freeproto(L: *mut lua_State, f: *mut Proto) {
    luaM_freearray(L, (*f).code, (*f).sizecode as usize);
    luaM_freearray(L, (*f).p, (*f).sizep as usize);
    luaM_freearray(L, (*f).k, (*f).sizek as usize);
    luaM_freearray(L, (*f).lineinfo, (*f).sizelineinfo as usize);
    luaM_freearray(L, (*f).locvars, (*f).sizelocvars as usize);
    luaM_freearray(L, (*f).upvalues, (*f).sizeupvalues as usize);
    luaM_free(L, f);
}

/*
** Look for n-th local variable at line 'line' in function 'func'.
** Returns NULL if not found.
*/

#[no_mangle]
pub unsafe extern "C" fn luaF_getlocalname(
    f: *const Proto,
    mut local_number: c_int,
    pc: c_int,
) -> *const c_char {
    let mut i = 0;
    while i < (*f).sizelocvars && (*((*f).locvars).offset(i as isize)).startpc <= pc {
        if pc < (*((*f).locvars).offset(i as isize)).endpc {
            /* is variable active? */
            local_number -= 1;
            if local_number == 0 {
                return getstr((*(*f).locvars.offset(i as isize)).varname);
            }
        }
        i += 1;
    }
    return ptr::null(); /* not found */
}
