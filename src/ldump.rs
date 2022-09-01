/*
** Save precompiled Lua chunks
*/

use std::mem::size_of;
use std::ptr;

use libc::{c_int, c_void, size_t};

use crate::llimits::Instruction;
use crate::lobject::{
    bvalue, fltvalue, getstr_c, ivalue, tsslen, tsvalue, ttype, Proto, TString, LUA_TLNGSTR,
    LUA_TNUMFLT, LUA_TNUMINT, LUA_TSHRSTR,
};
use crate::lstate::lua_State;
use crate::lundump::{LUAC_DATA, LUAC_FORMAT, LUAC_INT, LUAC_NUM, LUAC_VERSION};
use crate::types::{lua_Integer, lua_Number, lua_Writer, LUA_SIGNATURE, LUA_TBOOLEAN, LUA_TNIL};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DumpState {
    pub L: *mut lua_State,
    pub writer: lua_Writer,
    pub data: *mut c_void,
    pub strip: c_int,
    pub status: c_int,
}

unsafe fn DumpBlock(b: *const c_void, size: size_t, D: *mut DumpState) {
    if (*D).status == 0 && size > 0 {
        (*D).status = (*D).writer.expect("non-null function pointer")((*D).L, b, size, (*D).data);
    }
}

unsafe fn DumpVector<T>(v: *const T, n: usize, D: *mut DumpState) {
    DumpBlock(v as *const c_void, n * size_of::<T>(), D)
}

unsafe fn DumpLiteral(s: &[u8], D: *mut DumpState) {
    DumpBlock(s.as_ptr() as *const c_void, s.len(), D)
}

unsafe fn DumpVar<T: Copy>(x: T, D: *mut DumpState) {
    DumpVector(&x, 1, D)
}

unsafe fn DumpString(s: *const TString, D: *mut DumpState) {
    if s.is_null() {
        DumpVar::<u8>(0, D);
    } else {
        let size = tsslen(s) + 1; /* include trailing '\0' */
        let str = getstr_c(s);
        if size < 0xff {
            DumpVar::<u8>(size as u8, D);
        } else {
            DumpVar::<u8>(0xff as u8, D);
            DumpVar::<usize>(size, D);
        }
        DumpVector(str, size - 1, D); /* no need to save '\0' */
    };
}

unsafe fn DumpCode(f: *const Proto, D: *mut DumpState) {
    DumpVar::<i32>((*f).sizecode, D);
    DumpVector((*f).code, (*f).sizecode as usize, D);
}

unsafe fn DumpConstants(f: *const Proto, D: *mut DumpState) {
    let n = (*f).sizek;
    DumpVar::<i32>(n, D);
    for i in 0..n {
        let o = &mut *((*f).k).add(i as usize);
        DumpVar::<u8>(ttype(o) as u8, D);
        match ttype(o) {
            LUA_TBOOLEAN => {
                DumpVar::<u8>(bvalue(o) as u8, D);
            }
            LUA_TNUMFLT => {
                DumpVar::<lua_Number>(fltvalue(o), D);
            }
            LUA_TNUMINT => {
                DumpVar::<lua_Integer>(ivalue(o), D);
            }
            LUA_TSHRSTR | LUA_TLNGSTR => {
                DumpString(tsvalue(o), D);
            }
            LUA_TNIL | _ => {}
        }
    }
}

unsafe fn DumpProtos(f: *const Proto, D: *mut DumpState) {
    let n = (*f).sizep;
    DumpVar::<i32>(n, D);
    for i in 0..n {
        DumpFunction(*((*f).p).add(i as usize), (*f).source, D);
    }
}

unsafe fn DumpUpvalues(f: *const Proto, D: *mut DumpState) {
    let n = (*f).sizeupvalues;
    DumpVar::<i32>(n, D);
    for i in 0..n {
        DumpVar::<u8>((*((*f).upvalues).add(i as usize)).instack, D);
        DumpVar::<u8>((*((*f).upvalues).add(i as usize)).idx, D);
    }
}

unsafe fn DumpDebug(f: *const Proto, D: *mut DumpState) {
    let mut n = if (*D).strip != 0 {
        0
    } else {
        (*f).sizelineinfo
    };
    DumpVar::<i32>(n, D);
    DumpVector((*f).lineinfo, n as usize, D);
    n = if (*D).strip != 0 { 0 } else { (*f).sizelocvars };
    DumpVar::<i32>(n, D);
    for i in 0..n {
        DumpString((*((*f).locvars).add(i as usize)).varname, D);
        DumpVar::<i32>((*((*f).locvars).add(i as usize)).startpc, D);
        DumpVar::<i32>((*((*f).locvars).add(i as usize)).endpc, D);
    }
    n = if (*D).strip != 0 {
        0
    } else {
        (*f).sizeupvalues
    };
    DumpVar::<i32>(n, D);
    for i in 0..n {
        DumpString((*((*f).upvalues).add(i as usize)).name, D);
    }
}

unsafe fn DumpFunction(f: *const Proto, psource: *mut TString, D: *mut DumpState) {
    if (*D).strip != 0 || (*f).source == psource {
        DumpString(ptr::null(), D);
    } else {
        DumpString((*f).source, D);
    }
    DumpVar::<i32>((*f).linedefined, D);
    DumpVar::<i32>((*f).lastlinedefined, D);
    DumpVar::<u8>((*f).numparams, D);
    DumpVar::<u8>((*f).is_vararg, D);
    DumpVar::<u8>((*f).maxstacksize, D);
    DumpCode(f, D);
    DumpConstants(f, D);
    DumpUpvalues(f, D);
    DumpProtos(f, D);
    DumpDebug(f, D);
}

unsafe fn DumpHeader(D: *mut DumpState) {
    DumpLiteral(LUA_SIGNATURE, D);
    DumpVar::<u8>(LUAC_VERSION, D);
    DumpVar::<u8>(LUAC_FORMAT, D);
    DumpLiteral(LUAC_DATA, D);
    DumpVar::<u8>(size_of::<i32>() as u8, D);
    DumpVar::<u8>(size_of::<usize>() as u8, D);
    DumpVar::<u8>(size_of::<Instruction>() as u8, D);
    DumpVar::<u8>(size_of::<lua_Integer>() as u8, D);
    DumpVar::<u8>(size_of::<lua_Number>() as u8, D);
    DumpVar::<lua_Integer>(LUAC_INT, D);
    DumpVar::<lua_Number>(LUAC_NUM, D);
}

/*
** dump Lua function as precompiled chunk
*/
#[no_mangle]
pub unsafe extern "C" fn luaU_dump(
    L: *mut lua_State,
    f: *const Proto,
    writer: lua_Writer,
    data: *mut c_void,
    strip: c_int,
) -> c_int {
    let mut D = DumpState {
        L,
        writer,
        data,
        strip,
        status: 0,
    };
    DumpHeader(&mut D);
    // This is inconsistency in original Lua
    DumpVar::<u8>((*f).sizeupvalues as u8, &mut D);
    DumpFunction(f, ptr::null_mut(), &mut D);
    return D.status;
}
