#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, register_tool)]
extern "C" {
    pub type lua_longjmp;
    fn luaO_pushvfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        argp: va_list,
    ) -> *const libc::c_char;
    static luaO_nilobject_: TValue;
    static luaT_typenames_: [*const libc::c_char; 11];
    fn luaZ_init(
        L: *mut lua_State,
        z: *mut ZIO,
        reader: lua_Reader,
        data: *mut libc::c_void,
    );
    fn luaO_arith(
        L: *mut lua_State,
        op: libc::c_int,
        p1: *const TValue,
        p2: *const TValue,
        res: *mut TValue,
    );
    fn luaE_setdebt(g: *mut global_State, debt: l_mem);
    fn luaO_str2num(s: *const libc::c_char, o: *mut TValue) -> size_t;
    fn luaO_tostring(L: *mut lua_State, obj: StkId);
    fn luaG_errormsg(L: *mut lua_State) -> !;
    fn luaD_protectedparser(
        L: *mut lua_State,
        z: *mut ZIO,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaD_pcall(
        L: *mut lua_State,
        func: Pfunc,
        u: *mut libc::c_void,
        oldtop: ptrdiff_t,
        ef: ptrdiff_t,
    ) -> libc::c_int;
    fn luaD_growstack(L: *mut lua_State, n: libc::c_int);
    fn luaD_rawrunprotected(
        L: *mut lua_State,
        f: Pfunc,
        ud: *mut libc::c_void,
    ) -> libc::c_int;
    fn luaF_newCclosure(L: *mut lua_State, nelems: libc::c_int) -> *mut CClosure;
    fn luaC_step(L: *mut lua_State);
    fn luaC_fullgc(L: *mut lua_State, isemergency: libc::c_int);
    fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject);
    fn luaC_barrierback_(L: *mut lua_State, o: *mut Table);
    fn luaC_upvalbarrier_(L: *mut lua_State, uv: *mut UpVal);
    fn luaC_checkfinalizer(L: *mut lua_State, o: *mut GCObject, mt: *mut Table);
    fn luaC_upvdeccount(L: *mut lua_State, uv: *mut UpVal);
    fn luaS_newudata(L: *mut lua_State, s: size_t) -> *mut Udata;
    fn luaS_newlstr(
        L: *mut lua_State,
        str: *const libc::c_char,
        l: size_t,
    ) -> *mut TString;
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    fn luaH_getint(t: *mut Table, key: lua_Integer) -> *const TValue;
    fn luaH_setint(
        L: *mut lua_State,
        t: *mut Table,
        key: lua_Integer,
        value: *mut TValue,
    );
    fn luaH_getstr(t: *mut Table, key: *mut TString) -> *const TValue;
    fn luaH_get(t: *mut Table, key: *const TValue) -> *const TValue;
    fn luaH_set(L: *mut lua_State, t: *mut Table, key: *const TValue) -> *mut TValue;
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    fn luaH_resize(
        L: *mut lua_State,
        t: *mut Table,
        nasize: libc::c_uint,
        nhsize: libc::c_uint,
    );
    fn luaH_next(L: *mut lua_State, t: *mut Table, key: StkId) -> libc::c_int;
    fn luaH_getn(t: *mut Table) -> lua_Unsigned;
    fn luaU_dump(
        L: *mut lua_State,
        f: *const Proto,
        w: lua_Writer,
        data: *mut libc::c_void,
        strip: libc::c_int,
    ) -> libc::c_int;
    fn luaV_equalobj(
        L: *mut lua_State,
        t1: *const TValue,
        t2: *const TValue,
    ) -> libc::c_int;
    fn luaV_lessthan(
        L: *mut lua_State,
        l: *const TValue,
        r: *const TValue,
    ) -> libc::c_int;
    fn luaV_lessequal(
        L: *mut lua_State,
        l: *const TValue,
        r: *const TValue,
    ) -> libc::c_int;
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> libc::c_int;
    fn luaV_tointeger(
        obj: *const TValue,
        p: *mut lua_Integer,
        mode: libc::c_int,
    ) -> libc::c_int;
    fn luaV_finishget(
        L: *mut lua_State,
        t: *const TValue,
        key: *mut TValue,
        val: StkId,
        slot: *const TValue,
    );
    fn luaV_finishset(
        L: *mut lua_State,
        t: *const TValue,
        key: *mut TValue,
        val: StkId,
        slot: *const TValue,
    );
    fn luaV_concat(L: *mut lua_State, total: libc::c_int);
    fn luaV_objlen(L: *mut lua_State, ra: StkId, rb: *const TValue);
}
pub type __builtin_va_list = *mut libc::c_char;
pub type va_list = __builtin_va_list;
pub type __darwin_intptr_t = libc::c_long;
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type intptr_t = __darwin_intptr_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_State {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nci: libc::c_ushort,
    pub status: lu_byte,
    pub top: StkId,
    pub l_G: *mut global_State,
    pub ci: *mut CallInfo,
    pub oldpc: *const Instruction,
    pub stack_last: StkId,
    pub stack: StkId,
    pub openupval: *mut UpVal,
    pub gclist: *mut GCObject,
    pub twups: *mut lua_State,
    pub errorJmp: *mut lua_longjmp,
    pub base_ci: CallInfo,
    pub hook: lua_Hook,
    pub errfunc: ptrdiff_t,
    pub stacksize: libc::c_int,
    pub basehookcount: libc::c_int,
    pub hookcount: libc::c_int,
    pub nny: libc::c_ushort,
    pub nCcalls: libc::c_ushort,
    pub hookmask: sig_atomic_t,
    pub allowhook: lu_byte,
}
pub type lu_byte = libc::c_uchar;
pub type sig_atomic_t = libc::c_int;
pub type lua_Hook = Option::<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub currentline: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub nups: libc::c_uchar,
    pub nparams: libc::c_uchar,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub short_src: [libc::c_char; 60],
    pub i_ci: *mut CallInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: StkId,
    pub top: StkId,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: C2RustUnnamed,
    pub extra: ptrdiff_t,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub l: C2RustUnnamed_1,
    pub c: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub k: lua_KFunction,
    pub old_errfunc: ptrdiff_t,
    pub ctx: lua_KContext,
}
pub type lua_KContext = intptr_t;
pub type lua_KFunction = Option::<
    unsafe extern "C" fn(*mut lua_State, libc::c_int, lua_KContext) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub base: StkId,
    pub savedpc: *const Instruction,
}
pub type Instruction = libc::c_uint;
pub type StkId = *mut TValue;
pub type TValue = lua_TValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_TValue {
    pub value_: Value,
    pub tt_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub gc: *mut GCObject,
    pub p: *mut libc::c_void,
    pub b: libc::c_int,
    pub f: lua_CFunction,
    pub i: lua_Integer,
    pub n: lua_Number,
}
pub type lua_Number = libc::c_double;
pub type lua_Integer = libc::c_longlong;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCObject {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub v: *mut TValue,
    pub refcount: lu_mem,
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub open: C2RustUnnamed_3,
    pub value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub next: *mut UpVal,
    pub touched: libc::c_int,
}
pub type lu_mem = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_State {
    pub frealloc: lua_Alloc,
    pub ud: *mut libc::c_void,
    pub totalbytes: l_mem,
    pub GCdebt: l_mem,
    pub GCmemtrav: lu_mem,
    pub GCestimate: lu_mem,
    pub strt: stringtable,
    pub l_registry: TValue,
    pub seed: libc::c_uint,
    pub currentwhite: lu_byte,
    pub gcstate: lu_byte,
    pub gckind: lu_byte,
    pub gcrunning: lu_byte,
    pub allgc: *mut GCObject,
    pub sweepgc: *mut *mut GCObject,
    pub finobj: *mut GCObject,
    pub gray: *mut GCObject,
    pub grayagain: *mut GCObject,
    pub weak: *mut GCObject,
    pub ephemeron: *mut GCObject,
    pub allweak: *mut GCObject,
    pub tobefnz: *mut GCObject,
    pub fixedgc: *mut GCObject,
    pub twups: *mut lua_State,
    pub gcfinnum: libc::c_uint,
    pub gcpause: libc::c_int,
    pub gcstepmul: libc::c_int,
    pub panic: lua_CFunction,
    pub mainthread: *mut lua_State,
    pub version: *const lua_Number,
    pub memerrmsg: *mut TString,
    pub tmname: [*mut TString; 24],
    pub mt: [*mut Table; 9],
    pub strcache: [[*mut TString; 2]; 53],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub extra: lu_byte,
    pub shrlen: lu_byte,
    pub hash: libc::c_uint,
    pub u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub lnglen: size_t,
    pub hnext: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub flags: lu_byte,
    pub lsizenode: lu_byte,
    pub sizearray: libc::c_uint,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table,
    pub gclist: *mut GCObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub i_val: TValue,
    pub i_key: TKey,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union TKey {
    pub nk: C2RustUnnamed_5,
    pub tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub value_: Value,
    pub tt_: libc::c_int,
    pub next: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub hash: *mut *mut TString,
    pub nuse: libc::c_int,
    pub size: libc::c_int,
}
pub type l_mem = ptrdiff_t;
pub type lua_Alloc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        size_t,
        size_t,
    ) -> *mut libc::c_void,
>;
pub type lua_Unsigned = libc::c_ulonglong;
pub type lua_Reader = Option::<
    unsafe extern "C" fn(
        *mut lua_State,
        *mut libc::c_void,
        *mut size_t,
    ) -> *const libc::c_char,
>;
pub type lua_Writer = Option::<
    unsafe extern "C" fn(
        *mut lua_State,
        *const libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proto {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub numparams: lu_byte,
    pub is_vararg: lu_byte,
    pub maxstacksize: lu_byte,
    pub sizeupvalues: libc::c_int,
    pub sizek: libc::c_int,
    pub sizecode: libc::c_int,
    pub sizelineinfo: libc::c_int,
    pub sizep: libc::c_int,
    pub sizelocvars: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub k: *mut TValue,
    pub code: *mut Instruction,
    pub p: *mut *mut Proto,
    pub lineinfo: *mut libc::c_int,
    pub locvars: *mut LocVar,
    pub upvalues: *mut Upvaldesc,
    pub cache: *mut LClosure,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    pub gc: GCObject,
    pub ts: TString,
    pub u: Udata,
    pub cl: Closure,
    pub h: Table,
    pub p: Proto,
    pub th: lua_State,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table,
    pub len: size_t,
    pub user_: Value,
}
pub type Pfunc = Option::<unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString {
    pub dummy: L_Umaxalign,
    pub tsv: TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    pub n: lua_Number,
    pub u: libc::c_double,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
    pub l: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UUdata {
    pub dummy: L_Umaxalign,
    pub uv: Udata,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallS {
    pub func: StkId,
    pub nresults: libc::c_int,
}
pub type ZIO = Zio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State,
}
#[no_mangle]
pub static mut lua_ident: [libc::c_char; 129] = unsafe {
    *::std::mem::transmute::<
        &[u8; 129],
        &[libc::c_char; 129],
    >(
        b"$LuaVersion: Lua 5.3.6  Copyright (C) 1994-2020 Lua.org, PUC-Rio $$LuaAuthors: R. Ierusalimschy, L. H. de Figueiredo, W. Celes $\0",
    )
};
unsafe extern "C" fn index2addr(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *mut TValue {
    let mut ci: *mut CallInfo = (*L).ci;
    if idx > 0 as libc::c_int {
        let mut o: *mut TValue = ((*ci).func).offset(idx as isize);
        if o >= (*L).top {
            return &luaO_nilobject_ as *const TValue as *mut TValue
        } else {
            return o
        }
    } else if !(idx <= -(1000000 as libc::c_int) - 1000 as libc::c_int) {
        return ((*L).top).offset(idx as isize)
    } else if idx == -(1000000 as libc::c_int) - 1000 as libc::c_int {
        return &mut (*(*L).l_G).l_registry
    } else {
        idx = -(1000000 as libc::c_int) - 1000 as libc::c_int - idx;
        if (*(*ci).func).tt_ == 6 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            return &luaO_nilobject_ as *const TValue as *mut TValue
        } else {
            let mut func: *mut CClosure = &mut (*((*(*ci).func).value_.gc
                as *mut GCUnion))
                .cl
                .c;
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
unsafe extern "C" fn growstack(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut size: libc::c_int = *(ud as *mut libc::c_int);
    luaD_growstack(L, size);
}
#[no_mangle]
pub unsafe extern "C" fn lua_checkstack(
    mut L: *mut lua_State,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut ci: *mut CallInfo = (*L).ci;
    if ((*L).stack_last).offset_from((*L).top) as libc::c_long > n as libc::c_long {
        res = 1 as libc::c_int;
    } else {
        let mut inuse: libc::c_int = ((*L).top).offset_from((*L).stack) as libc::c_long
            as libc::c_int + 5 as libc::c_int;
        if inuse > 1000000 as libc::c_int - n {
            res = 0 as libc::c_int;
        } else {
            res = (luaD_rawrunprotected(
                L,
                Some(
                    growstack
                        as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
                ),
                &mut n as *mut libc::c_int as *mut libc::c_void,
            ) == 0 as libc::c_int) as libc::c_int;
        }
    }
    if res != 0 && (*ci).top < ((*L).top).offset(n as isize) {
        let ref mut fresh0 = (*ci).top;
        *fresh0 = ((*L).top).offset(n as isize);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_xmove(
    mut from: *mut lua_State,
    mut to: *mut lua_State,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if from == to {
        return;
    }
    let ref mut fresh1 = (*from).top;
    *fresh1 = (*fresh1).offset(-(n as isize));
    i = 0 as libc::c_int;
    while i < n {
        let mut io1: *mut TValue = (*to).top;
        *io1 = *((*from).top).offset(i as isize);
        let ref mut fresh2 = (*to).top;
        *fresh2 = (*fresh2).offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_atpanic(
    mut L: *mut lua_State,
    mut panicf: lua_CFunction,
) -> lua_CFunction {
    let mut old: lua_CFunction = None;
    old = (*(*L).l_G).panic;
    let ref mut fresh3 = (*(*L).l_G).panic;
    *fresh3 = panicf;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn lua_version(mut L: *mut lua_State) -> *const lua_Number {
    static mut version: lua_Number = 503 as libc::c_int as lua_Number;
    if L.is_null() { return &version } else { return (*(*L).l_G).version };
}
#[no_mangle]
pub unsafe extern "C" fn lua_absindex(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    return if idx > 0 as libc::c_int
        || idx <= -(1000000 as libc::c_int) - 1000 as libc::c_int
    {
        idx
    } else {
        ((*L).top).offset_from((*(*L).ci).func) as libc::c_long as libc::c_int + idx
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettop(mut L: *mut lua_State) -> libc::c_int {
    return ((*L).top).offset_from(((*(*L).ci).func).offset(1 as libc::c_int as isize))
        as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_settop(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut func: StkId = (*(*L).ci).func;
    if idx >= 0 as libc::c_int {
        while (*L).top < func.offset(1 as libc::c_int as isize).offset(idx as isize) {
            let ref mut fresh4 = (*L).top;
            let fresh5 = *fresh4;
            *fresh4 = (*fresh4).offset(1);
            (*fresh5).tt_ = 0 as libc::c_int;
        }
        let ref mut fresh6 = (*L).top;
        *fresh6 = func.offset(1 as libc::c_int as isize).offset(idx as isize);
    } else {
        let ref mut fresh7 = (*L).top;
        *fresh7 = (*fresh7).offset((idx + 1 as libc::c_int) as isize);
    };
}
unsafe extern "C" fn reverse(mut L: *mut lua_State, mut from: StkId, mut to: StkId) {
    while from < to {
        let mut temp: TValue = TValue {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io1: *mut TValue = &mut temp;
        *io1 = *from;
        let mut io1_0: *mut TValue = from;
        *io1_0 = *to;
        let mut io1_1: *mut TValue = to;
        *io1_1 = temp;
        from = from.offset(1);
        to = to.offset(-1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_rotate(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: libc::c_int,
) {
    let mut p: StkId = 0 as *mut TValue;
    let mut t: StkId = 0 as *mut TValue;
    let mut m: StkId = 0 as *mut TValue;
    t = ((*L).top).offset(-(1 as libc::c_int as isize));
    p = index2addr(L, idx);
    m = if n >= 0 as libc::c_int {
        t.offset(-(n as isize))
    } else {
        p.offset(-(n as isize)).offset(-(1 as libc::c_int as isize))
    };
    reverse(L, p, m);
    reverse(L, m.offset(1 as libc::c_int as isize), t);
    reverse(L, p, t);
}
#[no_mangle]
pub unsafe extern "C" fn lua_copy(
    mut L: *mut lua_State,
    mut fromidx: libc::c_int,
    mut toidx: libc::c_int,
) {
    let mut fr: *mut TValue = 0 as *mut TValue;
    let mut to: *mut TValue = 0 as *mut TValue;
    fr = index2addr(L, fromidx);
    to = index2addr(L, toidx);
    let mut io1: *mut TValue = to;
    *io1 = *fr;
    if toidx < -(1000000 as libc::c_int) - 1000 as libc::c_int {
        if (*fr).tt_ & (1 as libc::c_int) << 6 as libc::c_int != 0
            && (*((*(*(*L).ci).func).value_.gc as *mut GCUnion)).cl.c.marked
                as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0
            && (*(*fr).value_.gc).marked as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            luaC_barrier_(
                L,
                &mut (*(&mut (*((*(*(*L).ci).func).value_.gc as *mut GCUnion)).cl.c
                    as *mut CClosure as *mut GCUnion))
                    .gc,
                (*fr).value_.gc,
            );
        } else {};
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvalue(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut io1: *mut TValue = (*L).top;
    *io1 = *index2addr(L, idx);
    let ref mut fresh8 = (*L).top;
    *fresh8 = (*fresh8).offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_type(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: StkId = index2addr(L, idx);
    return if o != &luaO_nilobject_ as *const TValue as StkId {
        (*o).tt_ & 0xf as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_typename(
    mut L: *mut lua_State,
    mut t: libc::c_int,
) -> *const libc::c_char {
    return luaT_typenames_[(t + 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn lua_iscfunction(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: StkId = index2addr(L, idx);
    return ((*o).tt_ == 6 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        || (*o).tt_
            == 6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isinteger(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: StkId = index2addr(L, idx);
    return ((*o).tt_ == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isnumber(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2addr(L, idx);
    return if (*o).tt_ == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int {
        n = (*o).value_.n;
        1 as libc::c_int
    } else {
        luaV_tonumber_(o, &mut n)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_isstring(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ & 0xf as libc::c_int == 4 as libc::c_int
        || (*o).tt_ & 0xf as libc::c_int == 3 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isuserdata(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ == 7 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int
        || (*o).tt_ == 2 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawequal(
    mut L: *mut lua_State,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
) -> libc::c_int {
    let mut o1: StkId = index2addr(L, index1);
    let mut o2: StkId = index2addr(L, index2);
    return if o1 != &luaO_nilobject_ as *const TValue as StkId
        && o2 != &luaO_nilobject_ as *const TValue as StkId
    {
        luaV_equalobj(0 as *mut lua_State, o1 as *const TValue, o2 as *const TValue)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_arith(mut L: *mut lua_State, mut op: libc::c_int) {
    if !(op != 12 as libc::c_int && op != 13 as libc::c_int) {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *((*L).top).offset(-(1 as libc::c_int as isize));
        let ref mut fresh9 = (*L).top;
        *fresh9 = (*fresh9).offset(1);
    }
    luaO_arith(
        L,
        op,
        ((*L).top).offset(-(2 as libc::c_int as isize)) as *const TValue,
        ((*L).top).offset(-(1 as libc::c_int as isize)) as *const TValue,
        ((*L).top).offset(-(2 as libc::c_int as isize)),
    );
    let ref mut fresh10 = (*L).top;
    *fresh10 = (*fresh10).offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_compare(
    mut L: *mut lua_State,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut o1: StkId = 0 as *mut TValue;
    let mut o2: StkId = 0 as *mut TValue;
    let mut i: libc::c_int = 0 as libc::c_int;
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
pub unsafe extern "C" fn lua_stringtonumber(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
) -> size_t {
    let mut sz: size_t = luaO_str2num(s, (*L).top);
    if sz != 0 as libc::c_int as libc::c_ulong {
        let ref mut fresh11 = (*L).top;
        *fresh11 = (*fresh11).offset(1);
    }
    return sz;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tonumberx(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut pisnum: *mut libc::c_int,
) -> lua_Number {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2addr(L, idx);
    let mut isnum: libc::c_int = if (*o).tt_
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        n = (*o).value_.n;
        1 as libc::c_int
    } else {
        luaV_tonumber_(o, &mut n)
    };
    if isnum == 0 {
        n = 0 as libc::c_int as lua_Number;
    }
    if !pisnum.is_null() {
        *pisnum = isnum;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tointegerx(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut pisnum: *mut libc::c_int,
) -> lua_Integer {
    let mut res: lua_Integer = 0;
    let mut o: *const TValue = index2addr(L, idx);
    let mut isnum: libc::c_int = if (*o).tt_
        == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
    {
        res = (*o).value_.i;
        1 as libc::c_int
    } else {
        luaV_tointeger(o, &mut res, 0 as libc::c_int)
    };
    if isnum == 0 {
        res = 0 as libc::c_int as lua_Integer;
    }
    if !pisnum.is_null() {
        *pisnum = isnum;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_toboolean(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return !((*o).tt_ == 0 as libc::c_int
        || (*o).tt_ == 1 as libc::c_int && (*o).value_.b == 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tolstring(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut len: *mut size_t,
) -> *const libc::c_char {
    let mut o: StkId = index2addr(L, idx);
    if !((*o).tt_ & 0xf as libc::c_int == 4 as libc::c_int) {
        if !((*o).tt_ & 0xf as libc::c_int == 3 as libc::c_int) {
            if !len.is_null() {
                *len = 0 as libc::c_int as size_t;
            }
            return 0 as *const libc::c_char;
        }
        luaO_tostring(L, o);
        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
            luaC_step(L);
        }
        o = index2addr(L, idx);
    }
    if !len.is_null() {
        *len = if (*((*o).value_.gc as *mut GCUnion)).ts.tt as libc::c_int
            == 4 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        {
            (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as libc::c_ulong
        } else {
            (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen
        };
    }
    return (&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString
        as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawlen(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> size_t {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3f as libc::c_int {
        4 => return (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as size_t,
        20 => return (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen,
        7 => return (*((*o).value_.gc as *mut GCUnion)).u.len,
        5 => return luaH_getn(&mut (*((*o).value_.gc as *mut GCUnion)).h) as size_t,
        _ => return 0 as libc::c_int as size_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tocfunction(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> lua_CFunction {
    let mut o: StkId = index2addr(L, idx);
    if (*o).tt_ == 6 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int {
        return (*o).value_.f
    } else if (*o).tt_
        == 6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
    {
        return (*((*o).value_.gc as *mut GCUnion)).cl.c.f
    } else {
        return None
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_touserdata(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *mut libc::c_void {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0xf as libc::c_int {
        7 => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata
                as *mut libc::c_char)
                .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *mut libc::c_void;
        }
        2 => return (*o).value_.p,
        _ => return 0 as *mut libc::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tothread(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *mut lua_State {
    let mut o: StkId = index2addr(L, idx);
    return if !((*o).tt_ == 8 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) {
        0 as *mut lua_State
    } else {
        &mut (*((*o).value_.gc as *mut GCUnion)).th
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_topointer(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *const libc::c_void {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3f as libc::c_int {
        5 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).h as *mut Table
                as *const libc::c_void;
        }
        6 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.l as *mut LClosure
                as *const libc::c_void;
        }
        38 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.c as *mut CClosure
                as *const libc::c_void;
        }
        22 => {
            return ::std::mem::transmute::<lua_CFunction, size_t>((*o).value_.f)
                as *mut libc::c_void;
        }
        8 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).th as *mut lua_State
                as *const libc::c_void;
        }
        7 => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata
                as *mut libc::c_char)
                .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *const libc::c_void;
        }
        2 => return (*o).value_.p,
        _ => return 0 as *const libc::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushnil(mut L: *mut lua_State) {
    (*(*L).top).tt_ = 0 as libc::c_int;
    let ref mut fresh12 = (*L).top;
    *fresh12 = (*fresh12).offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushnumber(mut L: *mut lua_State, mut n: lua_Number) {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.n = n;
    (*io).tt_ = 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int;
    let ref mut fresh13 = (*L).top;
    *fresh13 = (*fresh13).offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushinteger(mut L: *mut lua_State, mut n: lua_Integer) {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.i = n;
    (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
    let ref mut fresh14 = (*L).top;
    *fresh14 = (*fresh14).offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    let mut ts: *mut TString = 0 as *mut TString;
    ts = if len == 0 as libc::c_int as libc::c_ulong {
        luaS_new(L, b"\0" as *const u8 as *const libc::c_char)
    } else {
        luaS_newlstr(L, s, len)
    };
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut TString = ts;
    let ref mut fresh15 = (*io).value_.gc;
    *fresh15 = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    let ref mut fresh16 = (*L).top;
    *fresh16 = (*fresh16).offset(1);
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return (ts as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
) -> *const libc::c_char {
    if s.is_null() {
        (*(*L).top).tt_ = 0 as libc::c_int;
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        ts = luaS_new(L, s);
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = ts;
        let ref mut fresh17 = (*io).value_.gc;
        *fresh17 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        s = (ts as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
    }
    let ref mut fresh18 = (*L).top;
    *fresh18 = (*fresh18).offset(1);
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut argp: va_list,
) -> *const libc::c_char {
    let mut ret: *const libc::c_char = 0 as *const libc::c_char;
    ret = luaO_pushvfstring(L, fmt, argp);
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *const libc::c_char {
    let mut ret: *const libc::c_char = 0 as *const libc::c_char;
    let mut argp: va_list = 0 as *mut libc::c_char;
    argp = args.clone();
    ret = luaO_pushvfstring(L, fmt, argp);
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushcclosure(
    mut L: *mut lua_State,
    mut fn_0: lua_CFunction,
    mut n: libc::c_int,
) {
    if n == 0 as libc::c_int {
        let mut io: *mut TValue = (*L).top;
        let ref mut fresh19 = (*io).value_.f;
        *fresh19 = fn_0;
        (*io).tt_ = 6 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
        let ref mut fresh20 = (*L).top;
        *fresh20 = (*fresh20).offset(1);
    } else {
        let mut cl: *mut CClosure = 0 as *mut CClosure;
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
            let mut io1: *mut TValue = &mut *((*cl).upvalue)
                .as_mut_ptr()
                .offset(n as isize) as *mut TValue;
            *io1 = *((*L).top).offset(n as isize);
        }
        let mut io_0: *mut TValue = (*L).top;
        let mut x_: *mut CClosure = cl;
        let ref mut fresh24 = (*io_0).value_.gc;
        *fresh24 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io_0)
            .tt_ = 6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int;
        let ref mut fresh25 = (*L).top;
        *fresh25 = (*fresh25).offset(1);
        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
            luaC_step(L);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushboolean(mut L: *mut lua_State, mut b: libc::c_int) {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.b = (b != 0 as libc::c_int) as libc::c_int;
    (*io).tt_ = 1 as libc::c_int;
    let ref mut fresh26 = (*L).top;
    *fresh26 = (*fresh26).offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlightuserdata(
    mut L: *mut lua_State,
    mut p: *mut libc::c_void,
) {
    let mut io: *mut TValue = (*L).top;
    let ref mut fresh27 = (*io).value_.p;
    *fresh27 = p;
    (*io).tt_ = 2 as libc::c_int;
    let ref mut fresh28 = (*L).top;
    *fresh28 = (*fresh28).offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushthread(mut L: *mut lua_State) -> libc::c_int {
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut lua_State = L;
    let ref mut fresh29 = (*io).value_.gc;
    *fresh29 = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 8 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    let ref mut fresh30 = (*L).top;
    *fresh30 = (*fresh30).offset(1);
    return ((*(*L).l_G).mainthread == L) as libc::c_int;
}
unsafe extern "C" fn auxgetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) -> libc::c_int {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if if !((*t).tt_ == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        !((*slot).tt_ == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *slot;
        let ref mut fresh31 = (*L).top;
        *fresh31 = (*fresh31).offset(1);
    } else {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = str;
        let ref mut fresh32 = (*io).value_.gc;
        *fresh32 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        let ref mut fresh33 = (*L).top;
        *fresh33 = (*fresh33).offset(1);
        luaV_finishget(
            L,
            t,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            slot,
        );
    }
    return (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_ & 0xf as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getglobal(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion))
        .h;
    return auxgetstr(L, luaH_getint(reg, 2 as libc::c_int as lua_Integer), name);
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettable(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut slot: *const TValue = 0 as *const TValue;
    if if !((*t).tt_ == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            ((*L).top).offset(-(1 as libc::c_int as isize)) as *const TValue,
        );
        !((*slot).tt_ == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        let mut io1: *mut TValue = ((*L).top).offset(-(1 as libc::c_int as isize));
        *io1 = *slot;
    } else {
        luaV_finishget(
            L,
            t as *const TValue,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            slot,
        );
    }
    return (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_ & 0xf as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getfield(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut k: *const libc::c_char,
) -> libc::c_int {
    return auxgetstr(L, index2addr(L, idx), k);
}
#[no_mangle]
pub unsafe extern "C" fn lua_geti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    t = index2addr(L, idx);
    if if !((*t).tt_ == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        !((*slot).tt_ == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *slot;
        let ref mut fresh34 = (*L).top;
        *fresh34 = (*fresh34).offset(1);
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
        let ref mut fresh35 = (*L).top;
        *fresh35 = (*fresh35).offset(1);
        luaV_finishget(
            L,
            t as *const TValue,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            slot,
        );
    }
    return (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_ & 0xf as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawget(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut io1: *mut TValue = ((*L).top).offset(-(1 as libc::c_int as isize));
    *io1 = *luaH_get(
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        ((*L).top).offset(-(1 as libc::c_int as isize)) as *const TValue,
    );
    return (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_ & 0xf as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgeti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut io1: *mut TValue = (*L).top;
    *io1 = *luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
    let ref mut fresh36 = (*L).top;
    *fresh36 = (*fresh36).offset(1);
    return (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_ & 0xf as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgetp(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut p: *const libc::c_void,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
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
    (*io).tt_ = 2 as libc::c_int;
    let mut io1: *mut TValue = (*L).top;
    *io1 = *luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, &mut k);
    let ref mut fresh38 = (*L).top;
    *fresh38 = (*fresh38).offset(1);
    return (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_ & 0xf as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_createtable(
    mut L: *mut lua_State,
    mut narray: libc::c_int,
    mut nrec: libc::c_int,
) {
    let mut t: *mut Table = 0 as *mut Table;
    t = luaH_new(L);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut Table = t;
    let ref mut fresh39 = (*io).value_.gc;
    *fresh39 = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    let ref mut fresh40 = (*L).top;
    *fresh40 = (*fresh40).offset(1);
    if narray > 0 as libc::c_int || nrec > 0 as libc::c_int {
        luaH_resize(L, t, narray as libc::c_uint, nrec as libc::c_uint);
    }
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_getmetatable(
    mut L: *mut lua_State,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj: *const TValue = 0 as *const TValue;
    let mut mt: *mut Table = 0 as *mut Table;
    let mut res: libc::c_int = 0 as libc::c_int;
    obj = index2addr(L, objindex);
    match (*obj).tt_ & 0xf as libc::c_int {
        5 => {
            mt = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
        }
        7 => {
            mt = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
        }
        _ => {
            mt = (*(*L).l_G).mt[((*obj).tt_ & 0xf as libc::c_int) as usize];
        }
    }
    if !mt.is_null() {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut Table = mt;
        let ref mut fresh41 = (*io).value_.gc;
        *fresh41 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        let ref mut fresh42 = (*L).top;
        *fresh42 = (*fresh42).offset(1);
        res = 1 as libc::c_int;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getuservalue(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: StkId = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *mut TValue = (*L).top;
    let mut iu: *const Udata = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*io).value_ = (*iu).user_;
    (*io).tt_ = (*iu).ttuv_ as libc::c_int;
    let ref mut fresh43 = (*L).top;
    *fresh43 = (*fresh43).offset(1);
    return (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_ & 0xf as libc::c_int;
}
unsafe extern "C" fn auxsetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if if !((*t).tt_ == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        (if (*slot).tt_ == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_
                & (1 as libc::c_int) << 6 as libc::c_int != 0
                && (*((*t).value_.gc as *mut GCUnion)).h.marked as libc::c_int
                    & (1 as libc::c_int) << 2 as libc::c_int != 0
                && (*(*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc).marked
                    as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {};
            *(slot as *mut TValue) = *((*L).top).offset(-(1 as libc::c_int as isize));
            1 as libc::c_int
        })
    } != 0
    {
        let ref mut fresh44 = (*L).top;
        *fresh44 = (*fresh44).offset(-1);
    } else {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = str;
        let ref mut fresh45 = (*io).value_.gc;
        *fresh45 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        let ref mut fresh46 = (*L).top;
        *fresh46 = (*fresh46).offset(1);
        luaV_finishset(
            L,
            t,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(2 as libc::c_int as isize)),
            slot,
        );
        let ref mut fresh47 = (*L).top;
        *fresh47 = (*fresh47).offset(-(2 as libc::c_int as isize));
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setglobal(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) {
    let mut reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion))
        .h;
    auxsetstr(L, luaH_getint(reg, 2 as libc::c_int as lua_Integer), name);
}
#[no_mangle]
pub unsafe extern "C" fn lua_settable(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut slot: *const TValue = 0 as *const TValue;
    if if !((*t).tt_ == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            ((*L).top).offset(-(2 as libc::c_int as isize)) as *const TValue,
        );
        (if (*slot).tt_ == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_
                & (1 as libc::c_int) << 6 as libc::c_int != 0
                && (*((*t).value_.gc as *mut GCUnion)).h.marked as libc::c_int
                    & (1 as libc::c_int) << 2 as libc::c_int != 0
                && (*(*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc).marked
                    as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {};
            *(slot as *mut TValue) = *((*L).top).offset(-(1 as libc::c_int as isize));
            1 as libc::c_int
        })
    } == 0
    {
        luaV_finishset(
            L,
            t as *const TValue,
            ((*L).top).offset(-(2 as libc::c_int as isize)),
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            slot,
        );
    }
    let ref mut fresh48 = (*L).top;
    *fresh48 = (*fresh48).offset(-(2 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn lua_setfield(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut k: *const libc::c_char,
) {
    auxsetstr(L, index2addr(L, idx), k);
}
#[no_mangle]
pub unsafe extern "C" fn lua_seti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) {
    let mut t: StkId = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    t = index2addr(L, idx);
    if if !((*t).tt_ == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        (if (*slot).tt_ == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_
                & (1 as libc::c_int) << 6 as libc::c_int != 0
                && (*((*t).value_.gc as *mut GCUnion)).h.marked as libc::c_int
                    & (1 as libc::c_int) << 2 as libc::c_int != 0
                && (*(*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc).marked
                    as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {};
            *(slot as *mut TValue) = *((*L).top).offset(-(1 as libc::c_int as isize));
            1 as libc::c_int
        })
    } != 0
    {
        let ref mut fresh49 = (*L).top;
        *fresh49 = (*fresh49).offset(-1);
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
        let ref mut fresh50 = (*L).top;
        *fresh50 = (*fresh50).offset(1);
        luaV_finishset(
            L,
            t as *const TValue,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(2 as libc::c_int as isize)),
            slot,
        );
        let ref mut fresh51 = (*L).top;
        *fresh51 = (*fresh51).offset(-(2 as libc::c_int as isize));
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawset(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut o: StkId = 0 as *mut TValue;
    let mut slot: *mut TValue = 0 as *mut TValue;
    o = index2addr(L, idx);
    slot = luaH_set(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        ((*L).top).offset(-(2 as libc::c_int as isize)) as *const TValue,
    );
    *slot = *((*L).top).offset(-(1 as libc::c_int as isize));
    (*((*o).value_.gc as *mut GCUnion)).h.flags = 0 as libc::c_int as lu_byte;
    if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_
        & (1 as libc::c_int) << 6 as libc::c_int != 0
        && (*((*o).value_.gc as *mut GCUnion)).h.marked as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int != 0
        && (*(*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc).marked
            as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {};
    let ref mut fresh52 = (*L).top;
    *fresh52 = (*fresh52).offset(-(2 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawseti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) {
    let mut o: StkId = 0 as *mut TValue;
    o = index2addr(L, idx);
    luaH_setint(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        n,
        ((*L).top).offset(-(1 as libc::c_int as isize)),
    );
    if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_
        & (1 as libc::c_int) << 6 as libc::c_int != 0
        && (*((*o).value_.gc as *mut GCUnion)).h.marked as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int != 0
        && (*(*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc).marked
            as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {};
    let ref mut fresh53 = (*L).top;
    *fresh53 = (*fresh53).offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawsetp(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut p: *const libc::c_void,
) {
    let mut o: StkId = 0 as *mut TValue;
    let mut k: TValue = TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut slot: *mut TValue = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *mut TValue = &mut k;
    let ref mut fresh54 = (*io).value_.p;
    *fresh54 = p as *mut libc::c_void;
    (*io).tt_ = 2 as libc::c_int;
    slot = luaH_set(L, &mut (*((*o).value_.gc as *mut GCUnion)).h, &mut k);
    *slot = *((*L).top).offset(-(1 as libc::c_int as isize));
    if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_
        & (1 as libc::c_int) << 6 as libc::c_int != 0
        && (*((*o).value_.gc as *mut GCUnion)).h.marked as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int != 0
        && (*(*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc).marked
            as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {};
    let ref mut fresh55 = (*L).top;
    *fresh55 = (*fresh55).offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_setmetatable(
    mut L: *mut lua_State,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj: *mut TValue = 0 as *mut TValue;
    let mut mt: *mut Table = 0 as *mut Table;
    obj = index2addr(L, objindex);
    if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_ == 0 as libc::c_int {
        mt = 0 as *mut Table;
    } else {
        mt = &mut (*((*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc
            as *mut GCUnion))
            .h;
    }
    match (*obj).tt_ & 0xf as libc::c_int {
        5 => {
            let ref mut fresh56 = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
            *fresh56 = mt;
            if !mt.is_null() {
                if (*(*obj).value_.gc).marked as libc::c_int
                    & (1 as libc::c_int) << 2 as libc::c_int != 0
                    && (*mt).marked as libc::c_int
                        & ((1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int) != 0
                {
                    luaC_barrier_(
                        L,
                        &mut (*((*obj).value_.gc as *mut GCUnion)).gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {};
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        7 => {
            let ref mut fresh57 = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
            *fresh57 = mt;
            if !mt.is_null() {
                if (*((*obj).value_.gc as *mut GCUnion)).u.marked as libc::c_int
                    & (1 as libc::c_int) << 2 as libc::c_int != 0
                    && (*mt).marked as libc::c_int
                        & ((1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int) != 0
                {
                    luaC_barrier_(
                        L,
                        &mut (*(&mut (*((*obj).value_.gc as *mut GCUnion)).u
                            as *mut Udata as *mut GCUnion))
                            .gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {};
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        _ => {
            let ref mut fresh58 = (*(*L).l_G)
                .mt[((*obj).tt_ & 0xf as libc::c_int) as usize];
            *fresh58 = mt;
        }
    }
    let ref mut fresh59 = (*L).top;
    *fresh59 = (*fresh59).offset(-1);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setuservalue(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut o: StkId = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *const TValue = ((*L).top).offset(-(1 as libc::c_int as isize))
        as *const TValue;
    let mut iu: *mut Udata = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*iu).user_ = (*io).value_;
    (*iu).ttuv_ = (*io).tt_ as lu_byte;
    if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_
        & (1 as libc::c_int) << 6 as libc::c_int != 0
        && (*(*o).value_.gc).marked as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int != 0
        && (*(*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc).marked
            as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        luaC_barrier_(
            L,
            &mut (*((*o).value_.gc as *mut GCUnion)).gc,
            (*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc,
        );
    } else {};
    let ref mut fresh60 = (*L).top;
    *fresh60 = (*fresh60).offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_callk(
    mut L: *mut lua_State,
    mut nargs: libc::c_int,
    mut nresults: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) {
    let mut func: StkId = 0 as *mut TValue;
    func = ((*L).top).offset(-((nargs + 1 as libc::c_int) as isize));
    if k.is_some() && (*L).nny as libc::c_int == 0 as libc::c_int {
        let ref mut fresh61 = (*(*L).ci).u.c.k;
        *fresh61 = k;
        (*(*L).ci).u.c.ctx = ctx;
        luaD_call(L, func, nresults);
    } else {
        luaD_callnoyield(L, func, nresults);
    }
    if nresults == -(1 as libc::c_int) && (*(*L).ci).top < (*L).top {
        let ref mut fresh62 = (*(*L).ci).top;
        *fresh62 = (*L).top;
    }
}
unsafe extern "C" fn f_call(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut c: *mut CallS = ud as *mut CallS;
    luaD_callnoyield(L, (*c).func, (*c).nresults);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pcallk(
    mut L: *mut lua_State,
    mut nargs: libc::c_int,
    mut nresults: libc::c_int,
    mut errfunc: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> libc::c_int {
    let mut c: CallS = CallS {
        func: 0 as *mut TValue,
        nresults: 0,
    };
    let mut status: libc::c_int = 0;
    let mut func: ptrdiff_t = 0;
    if errfunc == 0 as libc::c_int {
        func = 0 as libc::c_int as ptrdiff_t;
    } else {
        let mut o: StkId = index2addr(L, errfunc);
        func = (o as *mut libc::c_char).offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long;
    }
    c.func = ((*L).top).offset(-((nargs + 1 as libc::c_int) as isize));
    if k.is_none() || (*L).nny as libc::c_int > 0 as libc::c_int {
        c.nresults = nresults;
        status = luaD_pcall(
            L,
            Some(
                f_call as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
            ),
            &mut c as *mut CallS as *mut libc::c_void,
            (c.func as *mut libc::c_char).offset_from((*L).stack as *mut libc::c_char)
                as libc::c_long,
            func,
        );
    } else {
        let mut ci: *mut CallInfo = (*L).ci;
        let ref mut fresh63 = (*ci).u.c.k;
        *fresh63 = k;
        (*ci).u.c.ctx = ctx;
        (*ci)
            .extra = (c.func as *mut libc::c_char)
            .offset_from((*L).stack as *mut libc::c_char) as libc::c_long;
        (*ci).u.c.old_errfunc = (*L).errfunc;
        (*L).errfunc = func;
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int
            & !((1 as libc::c_int) << 0 as libc::c_int) | (*L).allowhook as libc::c_int)
            as libc::c_ushort;
        let ref mut fresh64 = (*ci).callstatus;
        *fresh64 = (*fresh64 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as libc::c_ushort;
        luaD_call(L, c.func, nresults);
        let ref mut fresh65 = (*ci).callstatus;
        *fresh65 = (*fresh65 as libc::c_int & !((1 as libc::c_int) << 4 as libc::c_int))
            as libc::c_ushort;
        (*L).errfunc = (*ci).u.c.old_errfunc;
        status = 0 as libc::c_int;
    }
    if nresults == -(1 as libc::c_int) && (*(*L).ci).top < (*L).top {
        let ref mut fresh66 = (*(*L).ci).top;
        *fresh66 = (*L).top;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_load(
    mut L: *mut lua_State,
    mut reader: lua_Reader,
    mut data: *mut libc::c_void,
    mut chunkname: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut z: ZIO = ZIO {
        n: 0,
        p: 0 as *const libc::c_char,
        reader: None,
        data: 0 as *mut libc::c_void,
        L: 0 as *mut lua_State,
    };
    let mut status: libc::c_int = 0;
    if chunkname.is_null() {
        chunkname = b"?\0" as *const u8 as *const libc::c_char;
    }
    luaZ_init(L, &mut z, reader, data);
    status = luaD_protectedparser(L, &mut z, chunkname, mode);
    if status == 0 as libc::c_int {
        let mut f: *mut LClosure = &mut (*((*((*L).top)
            .offset(-(1 as libc::c_int as isize)))
            .value_
            .gc as *mut GCUnion))
            .cl
            .l;
        if (*f).nupvalues as libc::c_int >= 1 as libc::c_int {
            let mut reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc
                as *mut GCUnion))
                .h;
            let mut gt: *const TValue = luaH_getint(
                reg,
                2 as libc::c_int as lua_Integer,
            );
            let mut io1: *mut TValue = (**((*f).upvals)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .v;
            *io1 = *gt;
            if (*(**((*f).upvals).as_mut_ptr().offset(0 as libc::c_int as isize)).v).tt_
                & (1 as libc::c_int) << 6 as libc::c_int != 0
                && !((**((*f).upvals).as_mut_ptr().offset(0 as libc::c_int as isize)).v
                    != &mut (**((*f).upvals)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .u
                        .value as *mut TValue)
            {
                luaC_upvalbarrier_(
                    L,
                    *((*f).upvals).as_mut_ptr().offset(0 as libc::c_int as isize),
                );
            } else {};
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_dump(
    mut L: *mut lua_State,
    mut writer: lua_Writer,
    mut data: *mut libc::c_void,
    mut strip: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut o: *mut TValue = 0 as *mut TValue;
    o = ((*L).top).offset(-(1 as libc::c_int as isize));
    if (*o).tt_
        == 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
    {
        status = luaU_dump(
            L,
            (*((*o).value_.gc as *mut GCUnion)).cl.l.p,
            writer,
            data,
            strip,
        );
    } else {
        status = 1 as libc::c_int;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_status(mut L: *mut lua_State) -> libc::c_int {
    return (*L).status as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gc(
    mut L: *mut lua_State,
    mut what: libc::c_int,
    mut data: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut g: *mut global_State = 0 as *mut global_State;
    g = (*L).l_G;
    match what {
        0 => {
            (*g).gcrunning = 0 as libc::c_int as lu_byte;
        }
        1 => {
            luaE_setdebt(g, 0 as libc::c_int as l_mem);
            (*g).gcrunning = 1 as libc::c_int as lu_byte;
        }
        2 => {
            luaC_fullgc(L, 0 as libc::c_int);
        }
        3 => {
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem >> 10 as libc::c_int)
                as libc::c_int;
        }
        4 => {
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem
                & 0x3ff as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        5 => {
            let mut debt: l_mem = 1 as libc::c_int as l_mem;
            let mut oldrunning: lu_byte = (*g).gcrunning;
            (*g).gcrunning = 1 as libc::c_int as lu_byte;
            if data == 0 as libc::c_int {
                luaE_setdebt(
                    g,
                    -((100 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<TString>() as libc::c_ulong)
                        as libc::c_int) as l_mem,
                );
                luaC_step(L);
            } else {
                debt = data as l_mem * 1024 as libc::c_int as libc::c_long + (*g).GCdebt;
                luaE_setdebt(g, debt);
                if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                    luaC_step(L);
                }
            }
            (*g).gcrunning = oldrunning;
            if debt > 0 as libc::c_int as libc::c_long
                && (*g).gcstate as libc::c_int == 7 as libc::c_int
            {
                res = 1 as libc::c_int;
            }
        }
        6 => {
            res = (*g).gcpause;
            (*g).gcpause = data;
        }
        7 => {
            res = (*g).gcstepmul;
            if data < 40 as libc::c_int {
                data = 40 as libc::c_int;
            }
            (*g).gcstepmul = data;
        }
        9 => {
            res = (*g).gcrunning as libc::c_int;
        }
        _ => {
            res = -(1 as libc::c_int);
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_error(mut L: *mut lua_State) -> libc::c_int {
    luaG_errormsg(L);
}
#[no_mangle]
pub unsafe extern "C" fn lua_next(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut more: libc::c_int = 0;
    t = index2addr(L, idx);
    more = luaH_next(
        L,
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        ((*L).top).offset(-(1 as libc::c_int as isize)),
    );
    if more != 0 {
        let ref mut fresh67 = (*L).top;
        *fresh67 = (*fresh67).offset(1);
    } else {
        let ref mut fresh68 = (*L).top;
        *fresh68 = (*fresh68).offset(-(1 as libc::c_int as isize));
    }
    return more;
}
#[no_mangle]
pub unsafe extern "C" fn lua_concat(mut L: *mut lua_State, mut n: libc::c_int) {
    if n >= 2 as libc::c_int {
        luaV_concat(L, n);
    } else if n == 0 as libc::c_int {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = luaS_newlstr(
            L,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        let ref mut fresh69 = (*io).value_.gc;
        *fresh69 = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        let ref mut fresh70 = (*L).top;
        *fresh70 = (*fresh70).offset(1);
    }
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_len(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    luaV_objlen(L, (*L).top, t as *const TValue);
    let ref mut fresh71 = (*L).top;
    *fresh71 = (*fresh71).offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_getallocf(
    mut L: *mut lua_State,
    mut ud: *mut *mut libc::c_void,
) -> lua_Alloc {
    let mut f: lua_Alloc = None;
    if !ud.is_null() {
        *ud = (*(*L).l_G).ud;
    }
    f = (*(*L).l_G).frealloc;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setallocf(
    mut L: *mut lua_State,
    mut f: lua_Alloc,
    mut ud: *mut libc::c_void,
) {
    let ref mut fresh72 = (*(*L).l_G).ud;
    *fresh72 = ud;
    let ref mut fresh73 = (*(*L).l_G).frealloc;
    *fresh73 = f;
}
#[no_mangle]
pub unsafe extern "C" fn lua_newuserdata(
    mut L: *mut lua_State,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut u: *mut Udata = 0 as *mut Udata;
    u = luaS_newudata(L, size);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut Udata = u;
    let ref mut fresh74 = (*io).value_.gc;
    *fresh74 = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 7 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    let ref mut fresh75 = (*L).top;
    *fresh75 = (*fresh75).offset(1);
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return (u as *mut libc::c_char)
        .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
unsafe extern "C" fn aux_upvalue(
    mut fi: StkId,
    mut n: libc::c_int,
    mut val: *mut *mut TValue,
    mut owner: *mut *mut CClosure,
    mut uv: *mut *mut UpVal,
) -> *const libc::c_char {
    match (*fi).tt_ & 0x3f as libc::c_int {
        38 => {
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            if !(1 as libc::c_int <= n && n <= (*f).nupvalues as libc::c_int) {
                return 0 as *const libc::c_char;
            }
            *val = &mut *((*f).upvalue)
                .as_mut_ptr()
                .offset((n - 1 as libc::c_int) as isize) as *mut TValue;
            if !owner.is_null() {
                *owner = f;
            }
            return b"\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            let mut f_0: *mut LClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
            let mut name: *mut TString = 0 as *mut TString;
            let mut p: *mut Proto = (*f_0).p;
            if !(1 as libc::c_int <= n && n <= (*p).sizeupvalues) {
                return 0 as *const libc::c_char;
            }
            *val = (**((*f_0).upvals)
                .as_mut_ptr()
                .offset((n - 1 as libc::c_int) as isize))
                .v;
            if !uv.is_null() {
                *uv = *((*f_0).upvals)
                    .as_mut_ptr()
                    .offset((n - 1 as libc::c_int) as isize);
            }
            name = (*((*p).upvalues).offset((n - 1 as libc::c_int) as isize)).name;
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
    mut L: *mut lua_State,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *mut TValue = 0 as *mut TValue;
    name = aux_upvalue(
        index2addr(L, funcindex),
        n,
        &mut val,
        0 as *mut *mut CClosure,
        0 as *mut *mut UpVal,
    );
    if !name.is_null() {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *val;
        let ref mut fresh76 = (*L).top;
        *fresh76 = (*fresh76).offset(1);
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setupvalue(
    mut L: *mut lua_State,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *mut TValue = 0 as *mut TValue;
    let mut owner: *mut CClosure = 0 as *mut CClosure;
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    let mut fi: StkId = 0 as *mut TValue;
    fi = index2addr(L, funcindex);
    name = aux_upvalue(fi, n, &mut val, &mut owner, &mut uv);
    if !name.is_null() {
        let ref mut fresh77 = (*L).top;
        *fresh77 = (*fresh77).offset(-1);
        let mut io1: *mut TValue = val;
        *io1 = *(*L).top;
        if !owner.is_null() {
            if (*(*L).top).tt_ & (1 as libc::c_int) << 6 as libc::c_int != 0
                && (*owner).marked as libc::c_int
                    & (1 as libc::c_int) << 2 as libc::c_int != 0
                && (*(*(*L).top).value_.gc).marked as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                luaC_barrier_(
                    L,
                    &mut (*(owner as *mut GCUnion)).gc,
                    (*(*L).top).value_.gc,
                );
            } else {};
        } else if !uv.is_null() {
            if (*(*uv).v).tt_ & (1 as libc::c_int) << 6 as libc::c_int != 0
                && !((*uv).v != &mut (*uv).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, uv);
            } else {};
        }
    }
    return name;
}
unsafe extern "C" fn getupvalref(
    mut L: *mut lua_State,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
) -> *mut *mut UpVal {
    let mut f: *mut LClosure = 0 as *mut LClosure;
    let mut fi: StkId = index2addr(L, fidx);
    f = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
    return &mut *((*f).upvals).as_mut_ptr().offset((n - 1 as libc::c_int) as isize)
        as *mut *mut UpVal;
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvalueid(
    mut L: *mut lua_State,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
) -> *mut libc::c_void {
    let mut fi: StkId = index2addr(L, fidx);
    match (*fi).tt_ & 0x3f as libc::c_int {
        6 => return *getupvalref(L, fidx, n) as *mut libc::c_void,
        38 => {
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            return &mut *((*f).upvalue)
                .as_mut_ptr()
                .offset((n - 1 as libc::c_int) as isize) as *mut TValue
                as *mut libc::c_void;
        }
        _ => return 0 as *mut libc::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvaluejoin(
    mut L: *mut lua_State,
    mut fidx1: libc::c_int,
    mut n1: libc::c_int,
    mut fidx2: libc::c_int,
    mut n2: libc::c_int,
) {
    let mut up1: *mut *mut UpVal = getupvalref(L, fidx1, n1);
    let mut up2: *mut *mut UpVal = getupvalref(L, fidx2, n2);
    if *up1 == *up2 {
        return;
    }
    luaC_upvdeccount(L, *up1);
    *up1 = *up2;
    let ref mut fresh78 = (**up1).refcount;
    *fresh78 = (*fresh78).wrapping_add(1);
    if (**up1).v != &mut (**up1).u.value as *mut TValue {
        (**up1).u.open.touched = 1 as libc::c_int;
    }
    if (*(**up1).v).tt_ & (1 as libc::c_int) << 6 as libc::c_int != 0
        && !((**up1).v != &mut (**up1).u.value as *mut TValue)
    {
        luaC_upvalbarrier_(L, *up1);
    } else {};
}
