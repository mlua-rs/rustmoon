/*
** Load precompiled Lua chunks
*/

use std::mem::{size_of, MaybeUninit};
use std::ptr;

use libc::{c_char, c_int, c_void};

use crate::ldo::{luaD_inctop, luaD_throw};
use crate::lfunc::{luaF_newLclosure, luaF_newproto};
use crate::lgc::luaC_objbarrier;
use crate::llimits::{Instruction, LUAI_MAXSHORTLEN};
use crate::lmem::luaM_newvector;
use crate::lobject::{
    getstr, luaO_pushfstring, setbvalue, setclLvalue, setfltvalue, setivalue, setnilvalue,
    setsvalue, LClosure, LocVar, Proto, TString, TValue, Upvaldesc, LUA_TLNGSTR, LUA_TNUMFLT,
    LUA_TNUMINT, LUA_TSHRSTR,
};
use crate::lstate::lua_State;
use crate::lstring::{luaS_createlngstrobj, luaS_newlstr};
use crate::lzio::{luaZ_read, ZIO};
use crate::types::{lua_Integer, lua_Number, LUA_ERRSYNTAX, LUA_SIGNATURE, LUA_TBOOLEAN, LUA_TNIL};

/* data to catch conversion errors */
pub const LUAC_DATA: &'static [u8] = b"\x19\x93\r\n\x1a\n";

pub const LUAC_INT: lua_Integer = 0x5678;
pub const LUAC_NUM: lua_Number = 370.5;

// TODO: Rewrite this
pub const LUAC_VERSION: u8 = 5 * 16 + 3;
pub const LUAC_FORMAT: u8 = 0; /* this is the official format */

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    pub L: *mut lua_State,
    pub Z: *mut ZIO,
    pub name: *const c_char,
}

unsafe fn error(S: *mut LoadState, why: *const c_char) -> ! {
    luaO_pushfstring((*S).L, cstr!("%s: %s precompiled chunk"), (*S).name, why);
    luaD_throw((*S).L, LUA_ERRSYNTAX);
}

/*
** All high-level loads go through LoadVector
*/

unsafe fn LoadBlock(S: *mut LoadState, b: *mut c_void, size: usize) {
    if luaZ_read((*S).Z, b, size) != 0 {
        error(S, cstr!("truncated"));
    }
}

unsafe fn LoadVector<T>(S: *mut LoadState, b: *mut T, n: usize) {
    LoadBlock(S, b as *mut c_void, n * size_of::<T>());
}

unsafe fn LoadVar<T: Copy + Default>(S: *mut LoadState) -> T {
    let mut x = MaybeUninit::uninit();
    LoadVector(S, x.as_mut_ptr(), 1);
    x.assume_init()
}

unsafe fn LoadString(S: *mut LoadState, p: *mut Proto) -> *mut TString {
    let L = (*S).L;
    let mut size = LoadVar::<u8>(S) as usize;
    let ts;
    if size == 0xff {
        size = LoadVar(S);
    }
    if size == 0 {
        return ptr::null_mut();
    } else {
        size -= 1;
        if size <= LUAI_MAXSHORTLEN {
            /* short string? */
            let mut buff = [0 as c_char; LUAI_MAXSHORTLEN];
            LoadVector(S, buff.as_mut_ptr(), size);
            ts = luaS_newlstr(L, buff.as_ptr(), size);
        } else {
            /* long string */
            ts = luaS_createlngstrobj(L, size);
            setsvalue(L, (*L).top, ts); /* anchor it ('loadVector' can GC) */
            luaD_inctop(L);
            LoadVector(S, getstr(ts), size); /* load directly in final place */
            (*L).top = ((*L).top).sub(1); /* pop string */
        }
    }
    luaC_objbarrier(L, obj2gco!(p), obj2gco!(ts));
    return ts;
}

unsafe fn LoadCode(S: *mut LoadState, f: *mut Proto) {
    let n = LoadVar::<i32>(S) as usize;
    (*f).code = luaM_newvector::<Instruction>((*S).L, n);
    (*f).sizecode = n as i32;
    LoadVector(S, (*f).code, n);
}

unsafe fn LoadConstants(S: *mut LoadState, mut f: *mut Proto) {
    let n = LoadVar::<i32>(S);
    (*f).k = luaM_newvector::<TValue>((*S).L, n as usize);
    (*f).sizek = n;
    for i in 0..(n as usize) {
        setnilvalue((*f).k.add(i));
    }
    for i in 0..(n as usize) {
        let o = &mut *((*f).k).add(i) as *mut TValue;
        let t = LoadVar::<u8>(S) as c_int;
        match t {
            LUA_TNIL => {
                setnilvalue(o);
            }
            LUA_TBOOLEAN => {
                let b = LoadVar::<u8>(S);
                setbvalue(o, b != 0);
            }
            LUA_TNUMFLT => {
                setfltvalue(o, LoadVar::<lua_Number>(S));
            }
            LUA_TNUMINT => {
                setivalue(o, LoadVar::<lua_Integer>(S));
            }
            LUA_TSHRSTR | LUA_TLNGSTR => {
                setsvalue((*S).L, o, LoadString(S, f));
            }
            _ => unreachable!(),
        }
    }
}

unsafe fn LoadProtos(S: *mut LoadState, f: *mut Proto) {
    let n = LoadVar::<i32>(S);
    (*f).p = luaM_newvector::<*mut Proto>((*S).L, n as usize);
    (*f).sizep = n;
    for i in 0..(n as usize) {
        *(*f).p.add(i) = ptr::null_mut();
    }
    for i in 0..(n as usize) {
        *(*f).p.add(i) = luaF_newproto((*S).L);
        luaC_objbarrier((*S).L, obj2gco!(f), obj2gco!(*(*f).p.add(i)));
        LoadFunction(S, *((*f).p).add(i), (*f).source);
    }
}

unsafe fn LoadUpvalues(S: *mut LoadState, f: *mut Proto) {
    let n = LoadVar::<i32>(S);
    (*f).upvalues = luaM_newvector::<Upvaldesc>((*S).L, n as usize);
    (*f).sizeupvalues = n;
    for i in 0..(n as usize) {
        (*(*f).upvalues.add(i)).name = ptr::null_mut();
    }
    for i in 0..(n as usize) {
        (*(*f).upvalues.add(i)).instack = LoadVar::<u8>(S);
        (*(*f).upvalues.add(i)).idx = LoadVar::<u8>(S);
    }
}

unsafe fn LoadDebug(S: *mut LoadState, f: *mut Proto) {
    let n = LoadVar::<i32>(S);
    (*f).lineinfo = luaM_newvector::<c_int>((*S).L, n as usize);
    (*f).sizelineinfo = n;
    LoadVector(S, (*f).lineinfo, n as usize);
    let n = LoadVar::<i32>(S);
    (*f).locvars = luaM_newvector::<LocVar>((*S).L, n as usize);
    (*f).sizelocvars = n;
    for i in 0..(n as usize) {
        (*(*f).locvars.add(i)).varname = ptr::null_mut();
    }
    for i in 0..(n as usize) {
        (*(*f).locvars.add(i)).varname = LoadString(S, f);
        (*(*f).locvars.add(i)).startpc = LoadVar::<i32>(S);
        (*(*f).locvars.add(i)).endpc = LoadVar::<i32>(S);
    }
    let n = LoadVar::<i32>(S);
    for i in 0..(n as usize) {
        (*(*f).upvalues.add(i)).name = LoadString(S, f);
    }
}

unsafe fn LoadFunction(S: *mut LoadState, f: *mut Proto, psource: *mut TString) {
    (*f).source = LoadString(S, f);
    if (*f).source.is_null() {
        /* no source in dump? */
        (*f).source = psource; /* reuse parent's source */
    }
    (*f).linedefined = LoadVar::<i32>(S);
    (*f).lastlinedefined = LoadVar::<i32>(S);
    (*f).numparams = LoadVar::<u8>(S);
    (*f).is_vararg = LoadVar::<u8>(S);
    (*f).maxstacksize = LoadVar::<u8>(S);
    LoadCode(S, f);
    LoadConstants(S, f);
    LoadUpvalues(S, f);
    LoadProtos(S, f);
    LoadDebug(S, f);
}

unsafe fn checkliteral(S: *mut LoadState, s: &[u8], msg: *const c_char) {
    let mut buff = [0u8; LUA_SIGNATURE.len() + LUAC_DATA.len()];
    LoadVector(S, buff.as_mut_ptr(), s.len());
    if s != &buff[..s.len()] {
        error(S, msg);
    }
}

unsafe fn checksize<T>(S: *mut LoadState, tname: *const c_char) {
    if LoadVar::<u8>(S) as usize != size_of::<T>() {
        error(
            S,
            luaO_pushfstring((*S).L, cstr!("%s size mismatch in"), tname),
        );
    }
}

unsafe fn checkHeader(S: *mut LoadState) {
    /* 1st char already checked */
    checkliteral(S, &LUA_SIGNATURE[1..], cstr!("not a"));
    if LoadVar::<u8>(S) != LUAC_VERSION {
        error(S, cstr!("version mismatch in"));
    }
    if LoadVar::<u8>(S) != LUAC_FORMAT {
        error(S, cstr!("format mismatch in"));
    }
    checkliteral(S, LUAC_DATA, cstr!("corrupted"));
    checksize::<i32>(S, cstr!("int"));
    checksize::<usize>(S, cstr!("size_t"));
    checksize::<Instruction>(S, cstr!("Instruction"));
    checksize::<lua_Integer>(S, cstr!("lua_Integer"));
    checksize::<lua_Number>(S, cstr!("lua_Number"));
    if LoadVar::<lua_Integer>(S) != LUAC_INT {
        error(S, cstr!("endianness mismatch in"));
    }
    if LoadVar::<lua_Number>(S) != LUAC_NUM {
        error(S, cstr!("float format mismatch in\0"));
    }
}

/*
** load precompiled chunk
*/
#[no_mangle]
pub unsafe extern "C" fn luaU_undump(
    L: *mut lua_State,
    Z: *mut ZIO,
    name: *const c_char,
) -> *mut LClosure {
    let mut S = LoadState { L, Z, name };
    if *name == b'@' as c_char || *name == b'=' as c_char {
        S.name = name.add(1);
    } else if *name == LUA_SIGNATURE[0] as c_char {
        S.name = cstr!("binary string");
    }
    checkHeader(&mut S);
    let cl = luaF_newLclosure(L, LoadVar::<u8>(&mut S) as c_int);
    setclLvalue(L, (*L).top, cl);
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    luaC_objbarrier(L, obj2gco!(cl), obj2gco!((*cl).p));
    LoadFunction(&mut S, (*cl).p, ptr::null_mut());
    debug_assert!((*cl).nupvalues as c_int == (*(*cl).p).sizeupvalues);
    return cl;
}
