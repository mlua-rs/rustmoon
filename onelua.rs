#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn localeconv() -> *mut lconv;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn log2(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn tmpfile() -> *mut FILE;
    fn tmpnam(_: *mut libc::c_char) -> *mut libc::c_char;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn clearerr(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn clock() -> clock_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type sig_atomic_t = __sig_atomic_t;
pub type time_t = __time_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
pub type intptr_t = libc::c_long;
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
    pub u: C2RustUnnamed_0,
    pub extra: ptrdiff_t,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub l: C2RustUnnamed_2,
    pub c: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct C2RustUnnamed_2 {
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
pub struct lua_longjmp {
    pub previous: *mut lua_longjmp,
    pub b: jmp_buf,
    pub status: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub v: *mut TValue,
    pub refcount: lu_mem,
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub open: C2RustUnnamed_4,
    pub value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub u: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
    pub nk: C2RustUnnamed_6,
    pub tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct LG {
    pub l: LX,
    pub g: global_State,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LX {
    pub extra_: [lu_byte; 8],
    pub l: lua_State,
}
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
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
}
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
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table,
    pub len: size_t,
    pub user_: Value,
}
pub const OP_EXTRAARG: OpCode = 46;
pub const OP_VARARG: OpCode = 45;
pub type TMS = libc::c_uint;
pub const TM_N: TMS = 24;
pub const TM_CALL: TMS = 23;
pub const TM_CONCAT: TMS = 22;
pub const TM_LE: TMS = 21;
pub const TM_LT: TMS = 20;
pub const TM_BNOT: TMS = 19;
pub const TM_UNM: TMS = 18;
pub const TM_SHR: TMS = 17;
pub const TM_SHL: TMS = 16;
pub const TM_BXOR: TMS = 15;
pub const TM_BOR: TMS = 14;
pub const TM_BAND: TMS = 13;
pub const TM_IDIV: TMS = 12;
pub const TM_DIV: TMS = 11;
pub const TM_POW: TMS = 10;
pub const TM_MOD: TMS = 9;
pub const TM_MUL: TMS = 8;
pub const TM_SUB: TMS = 7;
pub const TM_ADD: TMS = 6;
pub const TM_EQ: TMS = 5;
pub const TM_LEN: TMS = 4;
pub const TM_MODE: TMS = 3;
pub const TM_GC: TMS = 2;
pub const TM_NEWINDEX: TMS = 1;
pub const TM_INDEX: TMS = 0;
pub const OP_SELF: OpCode = 12;
pub const OP_LOADK: OpCode = 1;
pub type OpCode = libc::c_uint;
pub const OP_CLOSURE: OpCode = 44;
pub const OP_SETLIST: OpCode = 43;
pub const OP_TFORLOOP: OpCode = 42;
pub const OP_TFORCALL: OpCode = 41;
pub const OP_FORPREP: OpCode = 40;
pub const OP_FORLOOP: OpCode = 39;
pub const OP_RETURN: OpCode = 38;
pub const OP_TAILCALL: OpCode = 37;
pub const OP_CALL: OpCode = 36;
pub const OP_TESTSET: OpCode = 35;
pub const OP_TEST: OpCode = 34;
pub const OP_LE: OpCode = 33;
pub const OP_LT: OpCode = 32;
pub const OP_EQ: OpCode = 31;
pub const OP_JMP: OpCode = 30;
pub const OP_CONCAT: OpCode = 29;
pub const OP_LEN: OpCode = 28;
pub const OP_NOT: OpCode = 27;
pub const OP_BNOT: OpCode = 26;
pub const OP_UNM: OpCode = 25;
pub const OP_SHR: OpCode = 24;
pub const OP_SHL: OpCode = 23;
pub const OP_BXOR: OpCode = 22;
pub const OP_BOR: OpCode = 21;
pub const OP_BAND: OpCode = 20;
pub const OP_IDIV: OpCode = 19;
pub const OP_DIV: OpCode = 18;
pub const OP_POW: OpCode = 17;
pub const OP_MOD: OpCode = 16;
pub const OP_MUL: OpCode = 15;
pub const OP_SUB: OpCode = 14;
pub const OP_ADD: OpCode = 13;
pub const OP_NEWTABLE: OpCode = 11;
pub const OP_SETTABLE: OpCode = 10;
pub const OP_SETUPVAL: OpCode = 9;
pub const OP_SETTABUP: OpCode = 8;
pub const OP_GETTABLE: OpCode = 7;
pub const OP_GETTABUP: OpCode = 6;
pub const OP_GETUPVAL: OpCode = 5;
pub const OP_LOADNIL: OpCode = 4;
pub const OP_LOADBOOL: OpCode = 3;
pub const OP_LOADKX: OpCode = 2;
pub const OP_MOVE: OpCode = 0;
pub const iAx: OpMode = 3;
pub const OpArgU: OpArgMask = 1;
pub const iABC: OpMode = 0;
pub const OpArgN: OpArgMask = 0;
pub const iABx: OpMode = 1;
pub const iAsBx: OpMode = 2;
pub const OpArgR: OpArgMask = 2;
pub const OpArgK: OpArgMask = 3;
pub type l_uacNumber = libc::c_double;
pub type l_uacInt = libc::c_longlong;
pub type Pfunc = Option::<unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UUdata {
    pub dummy: L_Umaxalign,
    pub uv: Udata,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuxsetnodeT {
    pub t: *mut Table,
    pub nhsize: libc::c_uint,
}
pub const TK_WHILE: RESERVED = 278;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: libc::c_int,
    pub line: libc::c_int,
    pub nactvar: lu_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist {
    pub arr: *mut Labeldesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dyndata {
    pub actvar: C2RustUnnamed_7,
    pub gt: Labellist,
    pub label: Labellist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub arr: *mut Vardesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vardesc {
    pub idx: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SParser {
    pub z: *mut ZIO,
    pub buff: Mbuffer,
    pub dyd: Dyndata,
    pub mode: *const libc::c_char,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut libc::c_char,
    pub n: size_t,
    pub buffsize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto,
    pub prev: *mut FuncState,
    pub ls: *mut LexState,
    pub bl: *mut BlockCnt,
    pub pc: libc::c_int,
    pub lasttarget: libc::c_int,
    pub jpc: libc::c_int,
    pub nk: libc::c_int,
    pub np: libc::c_int,
    pub firstlocal: libc::c_int,
    pub nlocvars: libc::c_short,
    pub nactvar: lu_byte,
    pub nups: lu_byte,
    pub freereg: lu_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockCnt {
    pub previous: *mut BlockCnt,
    pub firstlabel: libc::c_int,
    pub firstgoto: libc::c_int,
    pub nactvar: lu_byte,
    pub upval: lu_byte,
    pub isloop: lu_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: libc::c_int,
    pub linenumber: libc::c_int,
    pub lastline: libc::c_int,
    pub t: Token,
    pub lookahead: Token,
    pub fs: *mut FuncState,
    pub L: *mut lua_State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer,
    pub h: *mut Table,
    pub dyd: *mut Dyndata,
    pub source: *mut TString,
    pub envn: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: libc::c_int,
    pub seminfo: SemInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    pub r: lua_Number,
    pub i: lua_Integer,
    pub ts: *mut TString,
}
pub const TK_EOS: RESERVED = 289;
pub const TK_INT: RESERVED = 291;
pub const TK_FLT: RESERVED = 290;
pub const TK_STRING: RESERVED = 293;
pub const TK_NAME: RESERVED = 292;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ival: lua_Integer,
    pub nval: lua_Number,
    pub info: libc::c_int,
    pub ind: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub idx: libc::c_short,
    pub t: lu_byte,
    pub vt: lu_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expdesc {
    pub k: expkind,
    pub u: C2RustUnnamed_8,
    pub t: libc::c_int,
    pub f: libc::c_int,
}
pub type expkind = libc::c_uint;
pub const VVARARG: expkind = 14;
pub const VCALL: expkind = 13;
pub const VRELOCABLE: expkind = 12;
pub const VJMP: expkind = 11;
pub const VINDEXED: expkind = 10;
pub const VUPVAL: expkind = 9;
pub const VLOCAL: expkind = 8;
pub const VNONRELOC: expkind = 7;
pub const VKINT: expkind = 6;
pub const VKFLT: expkind = 5;
pub const VK: expkind = 4;
pub const VFALSE: expkind = 3;
pub const VTRUE: expkind = 2;
pub const VNIL: expkind = 1;
pub const VVOID: expkind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LHS_assign {
    pub prev: *mut LHS_assign,
    pub v: expdesc,
}
pub type BinOpr = libc::c_uint;
pub const OPR_NOBINOPR: BinOpr = 21;
pub const OPR_OR: BinOpr = 20;
pub const OPR_AND: BinOpr = 19;
pub const OPR_GE: BinOpr = 18;
pub const OPR_GT: BinOpr = 17;
pub const OPR_NE: BinOpr = 16;
pub const OPR_LE: BinOpr = 15;
pub const OPR_LT: BinOpr = 14;
pub const OPR_EQ: BinOpr = 13;
pub const OPR_CONCAT: BinOpr = 12;
pub const OPR_SHR: BinOpr = 11;
pub const OPR_SHL: BinOpr = 10;
pub const OPR_BXOR: BinOpr = 9;
pub const OPR_BOR: BinOpr = 8;
pub const OPR_BAND: BinOpr = 7;
pub const OPR_IDIV: BinOpr = 6;
pub const OPR_DIV: BinOpr = 5;
pub const OPR_POW: BinOpr = 4;
pub const OPR_MOD: BinOpr = 3;
pub const OPR_MUL: BinOpr = 2;
pub const OPR_SUB: BinOpr = 1;
pub const OPR_ADD: BinOpr = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub left: lu_byte,
    pub right: lu_byte,
}
pub const TK_CONCAT: RESERVED = 280;
pub const TK_DOTS: RESERVED = 281;
pub const TK_DBCOLON: RESERVED = 288;
pub const TK_NE: RESERVED = 285;
pub const TK_IDIV: RESERVED = 279;
pub const TK_SHR: RESERVED = 287;
pub const TK_GE: RESERVED = 283;
pub const TK_SHL: RESERVED = 286;
pub const TK_LE: RESERVED = 284;
pub const TK_EQ: RESERVED = 282;
pub const TK_OR: RESERVED = 272;
pub const TK_AND: RESERVED = 257;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConsControl {
    pub v: expdesc,
    pub t: *mut expdesc,
    pub nh: libc::c_int,
    pub na: libc::c_int,
    pub tostore: libc::c_int,
}
pub const TK_FUNCTION: RESERVED = 265;
pub const TK_END: RESERVED = 262;
pub const TK_FALSE: RESERVED = 263;
pub const TK_TRUE: RESERVED = 276;
pub const TK_NIL: RESERVED = 270;
pub type UnOpr = libc::c_uint;
pub const OPR_NOUNOPR: UnOpr = 4;
pub const OPR_LEN: UnOpr = 3;
pub const OPR_NOT: UnOpr = 2;
pub const OPR_BNOT: UnOpr = 1;
pub const OPR_MINUS: UnOpr = 0;
pub const TK_NOT: RESERVED = 271;
pub const TK_GOTO: RESERVED = 266;
pub const TK_BREAK: RESERVED = 258;
pub const TK_UNTIL: RESERVED = 277;
pub const TK_ELSEIF: RESERVED = 261;
pub const TK_ELSE: RESERVED = 260;
pub const TK_RETURN: RESERVED = 274;
pub const TK_LOCAL: RESERVED = 269;
pub const TK_REPEAT: RESERVED = 273;
pub const TK_FOR: RESERVED = 264;
pub const TK_DO: RESERVED = 259;
pub const TK_IN: RESERVED = 268;
pub const TK_IF: RESERVED = 267;
pub const TK_THEN: RESERVED = 275;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    pub L: *mut lua_State,
    pub Z: *mut ZIO,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DumpState {
    pub L: *mut lua_State,
    pub writer: lua_Writer,
    pub data: *mut libc::c_void,
    pub strip: libc::c_int,
    pub status: libc::c_int,
}
pub type OpMode = libc::c_uint;
pub type OpArgMask = libc::c_uint;
pub type RESERVED = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadF {
    pub n: libc::c_int,
    pub f: *mut FILE,
    pub buff: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadS {
    pub s: *const libc::c_char,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State,
    pub initb: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UBox {
    pub box_0: *mut libc::c_void,
    pub bsize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Stream {
    pub f: *mut FILE,
    pub closef: lua_CFunction,
}
pub type IdxT = libc::c_uint;
pub type LStream = luaL_Stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RN {
    pub f: *mut FILE,
    pub c: libc::c_int,
    pub n: libc::c_int,
    pub buff: [libc::c_char; 201],
}
pub const Knop: KOption = 8;
pub const Kpadding: KOption = 6;
pub const Kpaddalign: KOption = 7;
pub const Kzstr: KOption = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Header {
    pub L: *mut lua_State,
    pub islittle: libc::c_int,
    pub maxalign: libc::c_int,
}
pub const Kstring: KOption = 4;
pub const Kchar: KOption = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Ftypes {
    pub f: libc::c_float,
    pub d: libc::c_double,
    pub n: lua_Number,
    pub buff: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub dummy: libc::c_int,
    pub little: libc::c_char,
}
pub const Kfloat: KOption = 2;
pub const Kint: KOption = 0;
pub type KOption = libc::c_uint;
pub const Kuint: KOption = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchState {
    pub src_init: *const libc::c_char,
    pub src_end: *const libc::c_char,
    pub p_end: *const libc::c_char,
    pub L: *mut lua_State,
    pub matchdepth: libc::c_int,
    pub level: libc::c_uchar,
    pub capture: [C2RustUnnamed_12; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub init: *const libc::c_char,
    pub len: ptrdiff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMatchState {
    pub src: *const libc::c_char,
    pub p: *const libc::c_char,
    pub lastmatch: *const libc::c_char,
    pub ms: MatchState,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub const errno: libc::c_int = *__errno_location();
pub const INT_MAX: libc::c_int = __INT_MAX__;
pub const UCHAR_MAX: libc::c_int = __SCHAR_MAX__ * 2 as libc::c_int + 1 as libc::c_int;
pub const UINT_MAX: libc::c_uint = (__INT_MAX__ as libc::c_uint)
    .wrapping_mul(2 as libc::c_uint)
    .wrapping_add(1 as libc::c_uint);
pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
pub const LLONG_MAX: libc::c_longlong = __LONG_LONG_MAX__;
pub const LLONG_MIN: libc::c_longlong = -__LONG_LONG_MAX__ - 1 as libc::c_longlong;
pub const LC_CTYPE: libc::c_int = __LC_CTYPE;
pub const __LC_MONETARY: libc::c_int = 4 as libc::c_int;
pub const LC_MONETARY: libc::c_int = __LC_MONETARY;
pub const __LC_CTYPE: libc::c_int = 0 as libc::c_int;
pub const __LC_COLLATE: libc::c_int = 3 as libc::c_int;
pub const LC_COLLATE: libc::c_int = __LC_COLLATE;
pub const __LC_ALL: libc::c_int = 6 as libc::c_int;
pub const LC_TIME: libc::c_int = __LC_TIME;
pub const LC_ALL: libc::c_int = __LC_ALL;
pub const __LC_NUMERIC: libc::c_int = 1 as libc::c_int;
pub const __LC_TIME: libc::c_int = 2 as libc::c_int;
pub const LC_NUMERIC: libc::c_int = __LC_NUMERIC;
pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
pub const HUGE_VAL: libc::c_double = ::core::f64::INFINITY;
pub const SIG_DFL: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const SIGINT: libc::c_int = 2 as libc::c_int;
pub const _IOFBF: libc::c_int = 0 as libc::c_int;
pub const _IOLBF: libc::c_int = 1 as libc::c_int;
pub const _IONBF: libc::c_int = 2 as libc::c_int;
pub const EOF: libc::c_int = -(1 as libc::c_int);
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
pub const RAND_MAX: libc::c_int = 2147483647 as libc::c_int;
pub const EXIT_FAILURE: libc::c_int = 1 as libc::c_int;
pub const EXIT_SUCCESS: libc::c_int = 0 as libc::c_int;
pub const __SCHAR_MAX__: libc::c_int = 127 as libc::c_int;
pub const CLOCKS_PER_SEC: libc::c_int = 1000000 as libc::c_int;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
pub const __LONG_LONG_MAX__: libc::c_longlong = 9223372036854775807 as libc::c_longlong;
pub const LUA_IDSIZE: libc::c_int = 60 as libc::c_int;
pub const LUA_INTEGER_FMT: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"%lld\0")
};
pub const LUA_NUMBER_FMT: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"%.14g\0")
};
pub const LUA_MININTEGER: libc::c_longlong = LLONG_MIN;
pub const LUA_MAXINTEGER: libc::c_longlong = LLONG_MAX;
pub const LUA_PATH_SEP: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b";\0")
};
pub const LUA_EXTRASPACE: libc::c_ulong = ::core::mem::size_of::<*mut libc::c_void>()
    as libc::c_ulong;
pub const LUAI_MAXSTACK: libc::c_int = 1000000 as libc::c_int;
pub const LUA_PATH_MARK: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"?\0")
};
pub const LUA_DIRSEP: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"/\0")
};
pub const LUA_INTEGER_FRMLEN: [libc::c_char; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"ll\0")
};
pub const LUA_NUMBER_FRMLEN: [libc::c_char; 1] = unsafe {
    *::core::mem::transmute::<&[u8; 1], &[libc::c_char; 1]>(b"\0")
};
pub const LUAL_BUFFERSIZE: libc::c_ulong = (0x80 as libc::c_int as libc::c_ulong)
    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
    .wrapping_mul(::core::mem::size_of::<lua_Integer>() as libc::c_ulong);
pub const LUA_TPROTO: libc::c_int = LUA_NUMTAGS;
pub const LUA_TLCL: libc::c_int = 6;
pub const LUA_TLCF: libc::c_int = 22;
pub const LUA_TCCL: libc::c_int = LUA_TFUNCTION | (2 as libc::c_int) << 4 as libc::c_int;
pub const LUA_TSHRSTR: libc::c_int = 4;
pub const LUA_TLNGSTR: libc::c_int = 20;
pub const LUA_TNUMFLT: libc::c_int = 3;
pub const LUA_TNUMINT: libc::c_int = 19;
pub const BIT_ISCOLLECTABLE: libc::c_int = (1 as libc::c_int) << 6 as libc::c_int;
pub const UTF8BUFFSZ: libc::c_int = 8 as libc::c_int;
pub const BASIC_STACK_SIZE: libc::c_int = 2 as libc::c_int * LUA_MINSTACK;
pub const LUA_RIDX_LAST: libc::c_int = 2 as libc::c_int;
pub const LUA_RIDX_MAINTHREAD: libc::c_int = 1 as libc::c_int;
pub const LUA_RIDX_GLOBALS: libc::c_int = 2 as libc::c_int;
pub const MINSTRTABSIZE: libc::c_int = 128 as libc::c_int;
pub const LUA_VERSION_NUM: libc::c_int = 503 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn luaZ_read(
    mut z: *mut ZIO,
    mut b: *mut libc::c_void,
    mut n: size_t,
) -> size_t {
    while n != 0 {
        let mut m: size_t = 0;
        if (*z).n == 0 as libc::c_int as libc::c_ulong {
            if luaZ_fill(z) == EOZ {
                return n
            } else {
                (*z).n = ((*z).n).wrapping_add(1);
                (*z).p = ((*z).p).offset(-1);
            }
        }
        m = if n <= (*z).n { n } else { (*z).n };
        memcpy(b, (*z).p as *const libc::c_void, m);
        (*z).n = ((*z).n as libc::c_ulong).wrapping_sub(m) as size_t as size_t;
        (*z).p = ((*z).p).offset(m as isize);
        b = (b as *mut libc::c_char).offset(m as isize) as *mut libc::c_void;
        n = (n as libc::c_ulong).wrapping_sub(m) as size_t as size_t;
    }
    return 0 as libc::c_int as size_t;
}
pub const KGC_NORMAL: libc::c_int = 0 as libc::c_int;
pub const LUA_SIGNATURE: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\x1BLua\0")
};
pub const MAX_INT: libc::c_int = INT_MAX;
#[no_mangle]
pub unsafe extern "C" fn luaZ_init(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut reader: lua_Reader,
    mut data: *mut libc::c_void,
) {
    (*z).L = L;
    (*z).reader = reader;
    (*z).data = data;
    (*z).n = 0 as libc::c_int as size_t;
    (*z).p = NULL as *const libc::c_char;
}
pub const LUAI_MAXCCALLS: libc::c_int = 200 as libc::c_int;
pub const CIST_HOOKYIELD: libc::c_int = (1 as libc::c_int) << 6 as libc::c_int;
pub const LUA_YIELD: libc::c_int = 1 as libc::c_int;
pub const LUA_ERRRUN: libc::c_int = 2 as libc::c_int;
pub const STRCACHE_N: libc::c_int = 53 as libc::c_int;
pub const STRCACHE_M: libc::c_int = 2 as libc::c_int;
pub const LUA_TNIL: libc::c_int = 0 as libc::c_int;
pub const LUA_ERRMEM: libc::c_int = 4;
pub const LUA_TSTRING: libc::c_int = 4 as libc::c_int;
pub const LUA_TUSERDATA: libc::c_int = 7 as libc::c_int;
pub const LUA_TTABLE: libc::c_int = 5;
pub const LUA_ERRGCMM: libc::c_int = 5 as libc::c_int;
pub const LUA_ERRERR: libc::c_int = 6 as libc::c_int;
pub const CIST_LEQ: libc::c_int = (1 as libc::c_int) << 7 as libc::c_int;
pub const LUA_REGISTRYINDEX: libc::c_int = -LUAI_MAXSTACK - 1000 as libc::c_int;
pub const CIST_TAIL: libc::c_int = (1 as libc::c_int) << 5 as libc::c_int;
pub const LUA_OK: libc::c_int = 0 as libc::c_int;
pub const CIST_FIN: libc::c_int = (1 as libc::c_int) << 8 as libc::c_int;
pub const LUA_MINSTACK: libc::c_int = 20 as libc::c_int;
pub const CIST_HOOKED: libc::c_int = (1 as libc::c_int) << 2 as libc::c_int;
pub const LUA_MULTRET: libc::c_int = -(1 as libc::c_int);
pub const CIST_FRESH: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
pub const LUA_NUMTAGS: libc::c_int = 9;
pub const LUA_ERRSYNTAX: libc::c_int = 3 as libc::c_int;
pub const LUA_TTHREAD: libc::c_int = 8 as libc::c_int;
pub const LUA_TFUNCTION: libc::c_int = 6 as libc::c_int;
pub const MAX_LUMEM: lu_mem = !(0 as libc::c_int as lu_mem);
pub const MAX_LMEM: lu_mem = MAX_LUMEM >> 1 as libc::c_int;
pub const LUAI_MAXSHORTLEN: libc::c_int = 40 as libc::c_int;
pub const MAX_SIZET: size_t = !(0 as libc::c_int as size_t);
pub const KGC_EMERGENCY: libc::c_int = 1 as libc::c_int;
pub const EOZ: libc::c_int = -1;
#[no_mangle]
pub unsafe extern "C" fn luaZ_fill(mut z: *mut ZIO) -> libc::c_int {
    let mut size: size_t = 0;
    let mut L = (*z).L;
    let mut buff = 0 as *const libc::c_char;
    buff = ((*z).reader).expect("non-null function pointer")(L, (*z).data, &mut size);
    if buff.is_null() || size == 0 as libc::c_int as libc::c_ulong {
        return EOZ;
    }
    (*z).n = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*z).p = buff;
    let fresh0 = (*z).p;
    (*z).p = ((*z).p).offset(1);
    return *fresh0 as libc::c_uchar as libc::c_int;
}
pub const EXTRA_STACK: libc::c_int = 5 as libc::c_int;
pub const LUA_TNUMBER: libc::c_int = 3 as libc::c_int;
pub const LUA_TBOOLEAN: libc::c_int = 1;
pub const LUA_TLIGHTUSERDATA: libc::c_int = 2;
pub const LUA_TNONE: libc::c_int = -(1 as libc::c_int);
pub const CIST_LUA: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
pub const CIST_OAH: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub const CIST_YPCALL: libc::c_int = (1 as libc::c_int) << 4 as libc::c_int;
pub const LUA_OPSUB: libc::c_int = 1;
pub const LUA_OPADD: libc::c_int = 0 as libc::c_int;
pub const LUA_OPMUL: libc::c_int = 2;
pub const LUA_OPLE: libc::c_int = 2;
pub const LUA_OPBNOT: libc::c_int = 13 as libc::c_int;
pub const LUA_OPSHR: libc::c_int = 11;
pub const LUA_OPSHL: libc::c_int = 10;
pub const LUA_OPBXOR: libc::c_int = 9;
pub const LUA_OPBOR: libc::c_int = 8;
pub const LUA_OPBAND: libc::c_int = 7;
pub const LUA_OPLT: libc::c_int = 1 as libc::c_int;
pub const LUA_OPEQ: libc::c_int = 0;
pub const LUA_OPMOD: libc::c_int = 3;
pub const LUA_OPUNM: libc::c_int = 12 as libc::c_int;
pub const LUA_OPIDIV: libc::c_int = 6;
pub const LUA_OPPOW: libc::c_int = 4;
pub const LUA_GCISRUNNING: libc::c_int = 9 as libc::c_int;
pub const LUA_GCSETSTEPMUL: libc::c_int = 7 as libc::c_int;
pub const LUA_GCSETPAUSE: libc::c_int = 6 as libc::c_int;
pub const LUA_GCSTEP: libc::c_int = 5;
pub const LUA_GCCOUNTB: libc::c_int = 4 as libc::c_int;
pub const LUA_GCCOUNT: libc::c_int = 3;
pub const LUA_GCCOLLECT: libc::c_int = 2;
pub const LUA_MASKCOUNT: libc::c_int = (1 as libc::c_int) << LUA_HOOKCOUNT;
pub const LUA_HOOKCOUNT: libc::c_int = 3 as libc::c_int;
pub const LUA_HOOKRET: libc::c_int = 1 as libc::c_int;
pub const LUA_GCRESTART: libc::c_int = 1;
pub const LUA_MASKRET: libc::c_int = (1 as libc::c_int) << LUA_HOOKRET;
pub const LUA_HOOKLINE: libc::c_int = 2 as libc::c_int;
pub const LUA_MASKLINE: libc::c_int = (1 as libc::c_int) << LUA_HOOKLINE;
pub const LUA_GCSTOP: libc::c_int = 0;
pub const LUA_OPDIV: libc::c_int = 5;
pub const LUA_HOOKCALL: libc::c_int = 0 as libc::c_int;
pub const LUA_HOOKTAILCALL: libc::c_int = 4 as libc::c_int;
pub const LUA_MASKCALL: libc::c_int = (1 as libc::c_int) << LUA_HOOKCALL;
pub const SIZE_C: libc::c_int = 9 as libc::c_int;
pub const SIZE_B: libc::c_int = 9 as libc::c_int;
pub const SIZE_Bx: libc::c_int = SIZE_C + SIZE_B;
pub const SIZE_A: libc::c_int = 8 as libc::c_int;
pub const SIZE_Ax: libc::c_int = SIZE_C + SIZE_B + SIZE_A;
pub const SIZE_OP: libc::c_int = 6 as libc::c_int;
pub const POS_OP: libc::c_int = 0 as libc::c_int;
pub const POS_A: libc::c_int = POS_OP + SIZE_OP;
pub const POS_C: libc::c_int = POS_A + SIZE_A;
pub const POS_B: libc::c_int = POS_C + SIZE_C;
pub const POS_Bx: libc::c_int = POS_C;
pub const POS_Ax: libc::c_int = POS_A;
pub const MAXARG_Bx: libc::c_int = ((1 as libc::c_int) << SIZE_Bx) - 1 as libc::c_int;
pub const MAXARG_sBx: libc::c_int = MAXARG_Bx >> 1 as libc::c_int;
pub const MAXARG_Ax: libc::c_int = ((1 as libc::c_int) << SIZE_Ax) - 1 as libc::c_int;
pub const MAXARG_A: libc::c_int = ((1 as libc::c_int) << SIZE_A) - 1 as libc::c_int;
pub const MAXARG_C: libc::c_int = ((1 as libc::c_int) << SIZE_C) - 1 as libc::c_int;
pub const BITRK: libc::c_int = (1 as libc::c_int) << SIZE_B - 1 as libc::c_int;
pub const MAXINDEXRK: libc::c_int = BITRK - 1 as libc::c_int;
pub const NO_REG: libc::c_int = MAXARG_A;
#[no_mangle]
pub static mut luaP_opmodes: [lu_byte; 47] = [
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgK, OpArgN, iABx),
    opmode!(0, 1, OpArgN, OpArgN, iABx),
    opmode!(0, 1, OpArgU, OpArgU, iABC),
    opmode!(0, 1, OpArgU, OpArgN, iABC),
    opmode!(0, 1, OpArgU, OpArgN, iABC),
    opmode!(0, 1, OpArgU, OpArgK, iABC),
    opmode!(0, 1, OpArgR, OpArgK, iABC),
    opmode!(0, 0, OpArgK, OpArgK, iABC),
    opmode!(0, 0, OpArgU, OpArgN, iABC),
    opmode!(0, 0, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgU, OpArgU, iABC),
    opmode!(0, 1, OpArgR, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgK, OpArgK, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgR, iABC),
    opmode!(0, 0, OpArgR, OpArgN, iAsBx),
    opmode!(1, 0, OpArgK, OpArgK, iABC),
    opmode!(1, 0, OpArgK, OpArgK, iABC),
    opmode!(1, 0, OpArgK, OpArgK, iABC),
    opmode!(1, 0, OpArgN, OpArgU, iABC),
    opmode!(1, 1, OpArgR, OpArgU, iABC),
    opmode!(0, 1, OpArgU, OpArgU, iABC),
    opmode!(0, 1, OpArgU, OpArgU, iABC),
    opmode!(0, 0, OpArgU, OpArgN, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iAsBx),
    opmode!(0, 1, OpArgR, OpArgN, iAsBx),
    opmode!(0, 0, OpArgN, OpArgU, iABC),
    opmode!(0, 1, OpArgR, OpArgN, iAsBx),
    opmode!(0, 0, OpArgU, OpArgU, iABC),
    opmode!(0, 1, OpArgU, OpArgN, iABx),
    opmode!(0, 1, OpArgU, OpArgN, iABC),
    opmode!(0, 0, OpArgU, OpArgU, iAx),
];
pub const LFIELDS_PER_FLUSH: libc::c_int = 50 as libc::c_int;
#[no_mangle]
pub static mut luaP_opnames: [*const libc::c_char; 48] = [
    b"MOVE\0" as *const u8 as *const libc::c_char,
    b"LOADK\0" as *const u8 as *const libc::c_char,
    b"LOADKX\0" as *const u8 as *const libc::c_char,
    b"LOADBOOL\0" as *const u8 as *const libc::c_char,
    b"LOADNIL\0" as *const u8 as *const libc::c_char,
    b"GETUPVAL\0" as *const u8 as *const libc::c_char,
    b"GETTABUP\0" as *const u8 as *const libc::c_char,
    b"GETTABLE\0" as *const u8 as *const libc::c_char,
    b"SETTABUP\0" as *const u8 as *const libc::c_char,
    b"SETUPVAL\0" as *const u8 as *const libc::c_char,
    b"SETTABLE\0" as *const u8 as *const libc::c_char,
    b"NEWTABLE\0" as *const u8 as *const libc::c_char,
    b"SELF\0" as *const u8 as *const libc::c_char,
    b"ADD\0" as *const u8 as *const libc::c_char,
    b"SUB\0" as *const u8 as *const libc::c_char,
    b"MUL\0" as *const u8 as *const libc::c_char,
    b"MOD\0" as *const u8 as *const libc::c_char,
    b"POW\0" as *const u8 as *const libc::c_char,
    b"DIV\0" as *const u8 as *const libc::c_char,
    b"IDIV\0" as *const u8 as *const libc::c_char,
    b"BAND\0" as *const u8 as *const libc::c_char,
    b"BOR\0" as *const u8 as *const libc::c_char,
    b"BXOR\0" as *const u8 as *const libc::c_char,
    b"SHL\0" as *const u8 as *const libc::c_char,
    b"SHR\0" as *const u8 as *const libc::c_char,
    b"UNM\0" as *const u8 as *const libc::c_char,
    b"BNOT\0" as *const u8 as *const libc::c_char,
    b"NOT\0" as *const u8 as *const libc::c_char,
    b"LEN\0" as *const u8 as *const libc::c_char,
    b"CONCAT\0" as *const u8 as *const libc::c_char,
    b"JMP\0" as *const u8 as *const libc::c_char,
    b"EQ\0" as *const u8 as *const libc::c_char,
    b"LT\0" as *const u8 as *const libc::c_char,
    b"LE\0" as *const u8 as *const libc::c_char,
    b"TEST\0" as *const u8 as *const libc::c_char,
    b"TESTSET\0" as *const u8 as *const libc::c_char,
    b"CALL\0" as *const u8 as *const libc::c_char,
    b"TAILCALL\0" as *const u8 as *const libc::c_char,
    b"RETURN\0" as *const u8 as *const libc::c_char,
    b"FORLOOP\0" as *const u8 as *const libc::c_char,
    b"FORPREP\0" as *const u8 as *const libc::c_char,
    b"TFORCALL\0" as *const u8 as *const libc::c_char,
    b"TFORLOOP\0" as *const u8 as *const libc::c_char,
    b"SETLIST\0" as *const u8 as *const libc::c_char,
    b"CLOSURE\0" as *const u8 as *const libc::c_char,
    b"VARARG\0" as *const u8 as *const libc::c_char,
    b"EXTRAARG\0" as *const u8 as *const libc::c_char,
    NULL as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn luaM_growaux_(
    mut L: *mut lua_State,
    mut block_0: *mut libc::c_void,
    mut size: *mut libc::c_int,
    mut size_elems: size_t,
    mut limit: libc::c_int,
    mut what: *const libc::c_char,
) -> *mut libc::c_void {
    let mut newblock = 0 as *mut libc::c_void;
    let mut newsize: libc::c_int = 0;
    if *size >= limit / 2 as libc::c_int {
        if *size >= limit {
            luaG_runerror(
                L,
                b"too many %s (limit is %d)\0" as *const u8 as *const libc::c_char,
                what,
                limit,
            );
        }
        newsize = limit;
    } else {
        newsize = *size * 2 as libc::c_int;
        if newsize < MINSIZEARRAY {
            newsize = MINSIZEARRAY;
        }
    }
    newblock = luaM_reallocv!(L, block, * size, newsize, size_elems);
    *size = newsize;
    return newblock;
}
pub const GCSatomic: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn luaM_toobig(mut L: *mut lua_State) -> ! {
    luaG_runerror(
        L,
        b"memory allocation error: block too big\0" as *const u8 as *const libc::c_char,
    );
}
pub const MINSIZEARRAY: libc::c_int = 4 as libc::c_int;
pub const GCSpropagate: libc::c_int = 0 as libc::c_int;
pub const GCSswpend: libc::c_int = 5 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn luaM_realloc_(
    mut L: *mut lua_State,
    mut block_0: *mut libc::c_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut libc::c_void {
    let mut newblock = 0 as *mut libc::c_void;
    let mut g = G!(L);
    let mut realosize = if !block_0.is_null() {
        osize
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    newblock = (Some(((*g).frealloc).expect("non-null function pointer")))
        .expect("non-null function pointer")((*g).ud, block_0, osize, nsize);
    if newblock.is_null() && nsize > 0 as libc::c_int as libc::c_ulong {
        if !((*g).version).is_null() {
            luaC_fullgc(L, 1 as libc::c_int);
            newblock = (Some(((*g).frealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")((*g).ud, block_0, osize, nsize);
        }
        if newblock.is_null() {
            luaD_throw(L, LUA_ERRMEM);
        }
    }
    (*g)
        .GCdebt = ((*g).GCdebt as libc::c_ulong)
        .wrapping_add(nsize)
        .wrapping_sub(realosize) as l_mem;
    return newblock;
}
pub const GCSswpallgc: libc::c_int = 2 as libc::c_int;
pub const GCSswpfinobj: libc::c_int = 3;
pub const GCSswptobefnz: libc::c_int = 4;
pub const WHITEBITS: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int
    | (1 as libc::c_int) << 1 as libc::c_int;
pub const GCScallfin: libc::c_int = 6 as libc::c_int;
pub const GCSTEPSIZE: libc::c_ulong = (100 as libc::c_int as libc::c_ulong)
    .wrapping_mul(::core::mem::size_of::<TString>() as libc::c_ulong);
pub const GCSpause: libc::c_int = 7 as libc::c_int;
pub const MAXUPVAL: libc::c_int = 255 as libc::c_int;
unsafe extern "C" fn LoadBlock(
    mut S: *mut LoadState,
    mut b: *mut libc::c_void,
    mut size: size_t,
) {
    if luaZ_read((*S).Z, b, size) != 0 as libc::c_int as libc::c_ulong {
        error(S, b"truncated\0" as *const u8 as *const libc::c_char);
    }
}
pub const LUAC_VERSION: libc::c_int = unsafe {
    ((*::core::mem::transmute::<
        &[u8; 2],
        &[libc::c_char; 2],
    >(b"5\0"))[0 as libc::c_int as usize] as libc::c_int - '0' as i32)
        * 16 as libc::c_int
        + ((*::core::mem::transmute::<
            &[u8; 2],
            &[libc::c_char; 2],
        >(b"3\0"))[0 as libc::c_int as usize] as libc::c_int - '0' as i32)
};
pub const LUAC_FORMAT: libc::c_int = 0 as libc::c_int;
pub const LUAC_DATA: [libc::c_char; 7] = unsafe {
    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"\x19\x93\r\n\x1A\n\0")
};
pub const LUAC_INT: libc::c_int = 0x5678 as libc::c_int;
pub const LUAC_NUM: libc::c_double = 370.5f64;
unsafe extern "C" fn error(mut S: *mut LoadState, mut why: *const libc::c_char) -> ! {
    luaO_pushfstring(
        (*S).L,
        b"%s: %s precompiled chunk\0" as *const u8 as *const libc::c_char,
        (*S).name,
        why,
    );
    luaD_throw((*S).L, LUA_ERRSYNTAX);
}
unsafe extern "C" fn LoadProtos(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut i: libc::c_int = 0;
    let mut n = LoadInt(S);
    (*f).p = luaM_newvector!(S -> L, n, Proto *);
    (*f).sizep = n;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh1 = *((*f).p).offset(i as isize);
        *fresh1 = NULL as *mut Proto;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh2 = *((*f).p).offset(i as isize);
        *fresh2 = luaF_newproto((*S).L);
        if luaC_objbarrier!(S -> L, f, f -> p[i]) != 0 {} else {};
        LoadFunction(S, *((*f).p).offset(i as isize), (*f).source);
        i += 1;
    }
}
unsafe extern "C" fn LoadByte(mut S: *mut LoadState) -> lu_byte {
    let mut x: lu_byte = 0;
    LoadVar!(S, x)(S, &mut x as *mut lu_byte as *mut libc::c_void, LoadVar!(S, x));
    return x;
}
unsafe extern "C" fn LoadCode(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut n = LoadInt(S);
    (*f).code = luaM_newvector!(S -> L, n, Instruction);
    (*f).sizecode = n;
    LoadVector!(
        S, f -> code, n
    )(S, (*f).code as *mut libc::c_void, LoadVector!(S, f -> code, n));
}
unsafe extern "C" fn LoadNumber(mut S: *mut LoadState) -> lua_Number {
    let mut x: lua_Number = 0.;
    LoadVar!(S, x)(S, &mut x as *mut lua_Number as *mut libc::c_void, LoadVar!(S, x));
    return x;
}
unsafe extern "C" fn LoadInt(mut S: *mut LoadState) -> libc::c_int {
    let mut x: libc::c_int = 0;
    LoadVar!(S, x)(S, &mut x as *mut libc::c_int as *mut libc::c_void, LoadVar!(S, x));
    return x;
}
unsafe extern "C" fn LoadInteger(mut S: *mut LoadState) -> lua_Integer {
    let mut x: lua_Integer = 0;
    LoadVar!(S, x)(S, &mut x as *mut lua_Integer as *mut libc::c_void, LoadVar!(S, x));
    return x;
}
unsafe extern "C" fn LoadConstants(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut i: libc::c_int = 0;
    let mut n = LoadInt(S);
    (*f).k = luaM_newvector!(S -> L, n, TValue);
    (*f).sizek = n;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh3 = setnilvalue!(& f -> k[i]);
        *fresh3 = setnilvalue!(& f -> k[i]);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        let mut o: *mut TValue = &mut *((*f).k).offset(i as isize) as *mut TValue;
        let mut t = LoadByte(S) as libc::c_int;
        match t {
            LUA_TNIL => {
                let ref mut fresh4 = setnilvalue!(o);
                *fresh4 = setnilvalue!(o);
            }
            LUA_TBOOLEAN => {
                let mut io = o;
                (*io).value_.b = LoadByte(S) as libc::c_int;
                (*io).tt_ = 1 as libc::c_int;
            }
            LUA_TNUMFLT => {
                let mut io_0 = o;
                (*io_0).value_.n = LoadNumber(S);
                (*io_0).tt_ = 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int;
            }
            LUA_TNUMINT => {
                let mut io_1 = o;
                (*io_1).value_.i = LoadInteger(S);
                (*io_1).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
            }
            LUA_TSHRSTR | LUA_TLNGSTR => {
                let mut io_2 = o;
                let mut x_ = LoadString(S, f);
                (*io_2).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
                (*io_2)
                    .tt_ = (*x_).tt as libc::c_int
                    | (1 as libc::c_int) << 6 as libc::c_int;
            }
            _ => {}
        }
        i += 1;
    }
}
unsafe extern "C" fn LoadString(
    mut S: *mut LoadState,
    mut p: *mut Proto,
) -> *mut TString {
    let mut L = (*S).L;
    let mut size = LoadByte(S) as size_t;
    let mut ts = 0 as *mut TString;
    if size == 0xff as libc::c_int as libc::c_ulong {
        LoadVar!(
            S, size
        )(S, &mut size as *mut size_t as *mut libc::c_void, LoadVar!(S, size));
    }
    if size == 0 as libc::c_int as libc::c_ulong {
        return NULL as *mut TString
    } else {
        size = size.wrapping_sub(1);
        if size <= LUAI_MAXSHORTLEN as libc::c_ulong {
            let mut buff: [libc::c_char; 40] = [0; 40];
            LoadVector!(
                S, buff, size
            )(S, buff.as_mut_ptr() as *mut libc::c_void, LoadVector!(S, buff, size));
            ts = luaS_newlstr(L, buff.as_mut_ptr(), size);
        } else {
            ts = luaS_createlngstrobj(L, size);
            let mut io = setsvalue2s!(L, L -> top, ts);
            let mut x_ = setsvalue2s!(L, L -> top, ts);
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
            luaD_inctop(L);
            LoadVector!(
                S, getstr(ts), size
            )(
                S,
                (ts as *mut libc::c_char)
                    .offset(::core::mem::size_of::<UTString>() as libc::c_ulong as isize)
                    as *mut libc::c_void,
                LoadVector!(S, getstr(ts), size),
            );
            (*L).top = ((*L).top).offset(-1);
        }
    }
    if luaC_objbarrier!(L, p, ts) != 0 {} else {};
    return ts;
}
unsafe extern "C" fn LoadUpvalues(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = LoadInt(S);
    (*f).upvalues = luaM_newvector!(S -> L, n, Upvaldesc);
    (*f).sizeupvalues = n;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh5 = (*((*f).upvalues).offset(i as isize)).name;
        *fresh5 = NULL as *mut TString;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        (*((*f).upvalues).offset(i as isize)).instack = LoadByte(S);
        (*((*f).upvalues).offset(i as isize)).idx = LoadByte(S);
        i += 1;
    }
}
unsafe extern "C" fn LoadDebug(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = LoadInt(S);
    (*f).lineinfo = luaM_newvector!(S -> L, n, int);
    (*f).sizelineinfo = n;
    LoadVector!(
        S, f -> lineinfo, n
    )(S, (*f).lineinfo as *mut libc::c_void, LoadVector!(S, f -> lineinfo, n));
    n = LoadInt(S);
    (*f).locvars = luaM_newvector!(S -> L, n, LocVar);
    (*f).sizelocvars = n;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh6 = (*((*f).locvars).offset(i as isize)).varname;
        *fresh6 = NULL as *mut TString;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh7 = (*((*f).locvars).offset(i as isize)).varname;
        *fresh7 = LoadString(S, f);
        (*((*f).locvars).offset(i as isize)).startpc = LoadInt(S);
        (*((*f).locvars).offset(i as isize)).endpc = LoadInt(S);
        i += 1;
    }
    n = LoadInt(S);
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh8 = (*((*f).upvalues).offset(i as isize)).name;
        *fresh8 = LoadString(S, f);
        i += 1;
    }
}
unsafe extern "C" fn LoadFunction(
    mut S: *mut LoadState,
    mut f: *mut Proto,
    mut psource: *mut TString,
) {
    (*f).source = LoadString(S, f);
    if ((*f).source).is_null() {
        (*f).source = psource;
    }
    (*f).linedefined = LoadInt(S);
    (*f).lastlinedefined = LoadInt(S);
    (*f).numparams = LoadByte(S);
    (*f).is_vararg = LoadByte(S);
    (*f).maxstacksize = LoadByte(S);
    LoadCode(S, f);
    LoadConstants(S, f);
    LoadUpvalues(S, f);
    LoadProtos(S, f);
    LoadDebug(S, f);
}
unsafe extern "C" fn checkliteral(
    mut S: *mut LoadState,
    mut s: *const libc::c_char,
    mut msg: *const libc::c_char,
) {
    let mut buff: [libc::c_char; 12] = [0; 12];
    let mut len = strlen(s);
    LoadVector!(
        S, buff, len
    )(S, buff.as_mut_ptr() as *mut libc::c_void, LoadVector!(S, buff, len));
    if memcmp(s as *const libc::c_void, buff.as_mut_ptr() as *const libc::c_void, len)
        != 0 as libc::c_int
    {
        error(S, msg);
    }
}
unsafe extern "C" fn fchecksize(
    mut S: *mut LoadState,
    mut size: size_t,
    mut tname: *const libc::c_char,
) {
    if LoadByte(S) as libc::c_ulong != size {
        error(
            S,
            luaO_pushfstring(
                (*S).L,
                b"%s size mismatch in\0" as *const u8 as *const libc::c_char,
                tname,
            ),
        );
    }
}
unsafe extern "C" fn checkHeader(mut S: *mut LoadState) {
    checkliteral(
        S,
        LUA_SIGNATURE.as_ptr().offset(1 as libc::c_int as isize),
        b"not a\0" as *const u8 as *const libc::c_char,
    );
    if LoadByte(S) as libc::c_int != LUAC_VERSION {
        error(S, b"version mismatch in\0" as *const u8 as *const libc::c_char);
    }
    if LoadByte(S) as libc::c_int != LUAC_FORMAT {
        error(S, b"format mismatch in\0" as *const u8 as *const libc::c_char);
    }
    checkliteral(
        S,
        LUAC_DATA.as_ptr(),
        b"corrupted\0" as *const u8 as *const libc::c_char,
    );
    checksize!(
        S, int
    )(S, checksize!(S, int), b"int\0" as *const u8 as *const libc::c_char);
    checksize!(
        S, size_t
    )(S, checksize!(S, size_t), b"size_t\0" as *const u8 as *const libc::c_char);
    checksize!(
        S, Instruction
    )(
        S,
        checksize!(S, Instruction),
        b"Instruction\0" as *const u8 as *const libc::c_char,
    );
    checksize!(
        S, lua_Integer
    )(
        S,
        checksize!(S, lua_Integer),
        b"lua_Integer\0" as *const u8 as *const libc::c_char,
    );
    checksize!(
        S, lua_Number
    )(S, checksize!(S, lua_Number), b"lua_Number\0" as *const u8 as *const libc::c_char);
    if LoadInteger(S) != LUAC_INT as libc::c_longlong {
        error(S, b"endianness mismatch in\0" as *const u8 as *const libc::c_char);
    }
    if LoadNumber(S) != LUAC_NUM {
        error(S, b"float format mismatch in\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaU_undump(
    mut L: *mut lua_State,
    mut Z: *mut ZIO,
    mut name: *const libc::c_char,
) -> *mut LClosure {
    let mut S = LoadState {
        L: 0 as *mut lua_State,
        Z: 0 as *mut ZIO,
        name: 0 as *const libc::c_char,
    };
    let mut cl = 0 as *mut LClosure;
    if *name as libc::c_int == '@' as i32 || *name as libc::c_int == '=' as i32 {
        S.name = name.offset(1 as libc::c_int as isize);
    } else if *name as libc::c_int
        == (*::core::mem::transmute::<
            &[u8; 5],
            &[libc::c_char; 5],
        >(b"\x1BLua\0"))[0 as libc::c_int as usize] as libc::c_int
    {
        S.name = b"binary string\0" as *const u8 as *const libc::c_char;
    } else {
        S.name = name;
    }
    S.L = L;
    S.Z = Z;
    checkHeader(&mut S);
    cl = luaF_newLclosure(L, LoadByte(&mut S) as libc::c_int);
    let mut io = setclLvalue!(L, L -> top, cl);
    let mut x_ = setclLvalue!(L, L -> top, cl);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int;
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    if luaC_objbarrier!(L, cl, cl -> p) != 0 {} else {};
    LoadFunction(&mut S, (*cl).p, NULL as *mut TString);
    return cl;
}
unsafe extern "C" fn DumpBlock(
    mut b: *const libc::c_void,
    mut size: size_t,
    mut D: *mut DumpState,
) {
    if (*D).status == 0 as libc::c_int && size > 0 as libc::c_int as libc::c_ulong {
        (*D)
            .status = (Some(((*D).writer).expect("non-null function pointer")))
            .expect("non-null function pointer")((*D).L, b, size, (*D).data);
    }
}
unsafe extern "C" fn DumpByte(mut y: libc::c_int, mut D: *mut DumpState) {
    let mut x = y as lu_byte;
    DumpVar!(x, D)(&mut x as *mut lu_byte as *const libc::c_void, DumpVar!(x, D), D);
}
unsafe extern "C" fn DumpInt(mut x: libc::c_int, mut D: *mut DumpState) {
    DumpVar!(x, D)(&mut x as *mut libc::c_int as *const libc::c_void, DumpVar!(x, D), D);
}
unsafe extern "C" fn DumpNumber(mut x: lua_Number, mut D: *mut DumpState) {
    DumpVar!(x, D)(&mut x as *mut lua_Number as *const libc::c_void, DumpVar!(x, D), D);
}
unsafe extern "C" fn DumpInteger(mut x: lua_Integer, mut D: *mut DumpState) {
    DumpVar!(x, D)(&mut x as *mut lua_Integer as *const libc::c_void, DumpVar!(x, D), D);
}
unsafe extern "C" fn DumpString(mut s: *const TString, mut D: *mut DumpState) {
    if s.is_null() {
        DumpByte(0 as libc::c_int, D);
    } else {
        let mut size = tsslen!(s).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut str: *const libc::c_char = getstr!(s);
        if size < 0xff as libc::c_int as libc::c_ulong {
            DumpByte(cast_int!(size), D);
        } else {
            DumpByte(0xff as libc::c_int, D);
            DumpVar!(
                size, D
            )(&mut size as *mut size_t as *const libc::c_void, DumpVar!(size, D), D);
        }
        DumpVector!(
            str, size - 1, D
        )(str as *const libc::c_void, DumpVector!(str, size - 1, D), D);
    };
}
unsafe extern "C" fn DumpCode(mut f: *const Proto, mut D: *mut DumpState) {
    DumpInt((*f).sizecode, D);
    DumpVector!(
        f -> code, f -> sizecode, D
    )((*f).code as *const libc::c_void, DumpVector!(f -> code, f -> sizecode, D), D);
}
unsafe extern "C" fn DumpConstants(mut f: *const Proto, mut D: *mut DumpState) {
    let mut i: libc::c_int = 0;
    let mut n = (*f).sizek;
    DumpInt(n, D);
    i = 0 as libc::c_int;
    while i < n {
        let mut o: *const TValue = &mut *((*f).k).offset(i as isize) as *mut TValue;
        DumpByte(ttype!(o), D);
        match ttype!(o) {
            LUA_TBOOLEAN => {
                DumpByte(bvalue!(o), D);
            }
            LUA_TNUMFLT => {
                DumpNumber(fltvalue!(o), D);
            }
            LUA_TNUMINT => {
                DumpInteger(ivalue!(o), D);
            }
            LUA_TSHRSTR | LUA_TLNGSTR => {
                DumpString(tsvalue!(o), D);
            }
            LUA_TNIL | _ => {}
        }
        i += 1;
    }
}
unsafe extern "C" fn DumpProtos(mut f: *const Proto, mut D: *mut DumpState) {
    let mut i: libc::c_int = 0;
    let mut n = (*f).sizep;
    DumpInt(n, D);
    i = 0 as libc::c_int;
    while i < n {
        DumpFunction(*((*f).p).offset(i as isize), (*f).source, D);
        i += 1;
    }
}
unsafe extern "C" fn DumpUpvalues(mut f: *const Proto, mut D: *mut DumpState) {
    let mut i: libc::c_int = 0;
    let mut n = (*f).sizeupvalues;
    DumpInt(n, D);
    i = 0 as libc::c_int;
    while i < n {
        DumpByte((*((*f).upvalues).offset(i as isize)).instack as libc::c_int, D);
        DumpByte((*((*f).upvalues).offset(i as isize)).idx as libc::c_int, D);
        i += 1;
    }
}
unsafe extern "C" fn DumpDebug(mut f: *const Proto, mut D: *mut DumpState) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = if (*D).strip != 0 { 0 as libc::c_int } else { (*f).sizelineinfo };
    DumpInt(n, D);
    DumpVector!(
        f -> lineinfo, n, D
    )((*f).lineinfo as *const libc::c_void, DumpVector!(f -> lineinfo, n, D), D);
    n = if (*D).strip != 0 { 0 as libc::c_int } else { (*f).sizelocvars };
    DumpInt(n, D);
    i = 0 as libc::c_int;
    while i < n {
        DumpString((*((*f).locvars).offset(i as isize)).varname, D);
        DumpInt((*((*f).locvars).offset(i as isize)).startpc, D);
        DumpInt((*((*f).locvars).offset(i as isize)).endpc, D);
        i += 1;
    }
    n = if (*D).strip != 0 { 0 as libc::c_int } else { (*f).sizeupvalues };
    DumpInt(n, D);
    i = 0 as libc::c_int;
    while i < n {
        DumpString((*((*f).upvalues).offset(i as isize)).name, D);
        i += 1;
    }
}
unsafe extern "C" fn DumpFunction(
    mut f: *const Proto,
    mut psource: *mut TString,
    mut D: *mut DumpState,
) {
    if (*D).strip != 0 || (*f).source == psource {
        DumpString(NULL as *const TString, D);
    } else {
        DumpString((*f).source, D);
    }
    DumpInt((*f).linedefined, D);
    DumpInt((*f).lastlinedefined, D);
    DumpByte((*f).numparams as libc::c_int, D);
    DumpByte((*f).is_vararg as libc::c_int, D);
    DumpByte((*f).maxstacksize as libc::c_int, D);
    DumpCode(f, D);
    DumpConstants(f, D);
    DumpUpvalues(f, D);
    DumpProtos(f, D);
    DumpDebug(f, D);
}
unsafe extern "C" fn DumpHeader(mut D: *mut DumpState) {
    DumpLiteral!(
        LUA_SIGNATURE, D
    )(
        b"\x1BLua\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        DumpLiteral!(LUA_SIGNATURE, D),
        D,
    );
    DumpByte(LUAC_VERSION, D);
    DumpByte(LUAC_FORMAT, D);
    DumpLiteral!(
        LUAC_DATA, D
    )(
        b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        DumpLiteral!(LUAC_DATA, D),
        D,
    );
    DumpByte(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int, D);
    DumpByte(::core::mem::size_of::<size_t>() as libc::c_ulong as libc::c_int, D);
    DumpByte(::core::mem::size_of::<Instruction>() as libc::c_ulong as libc::c_int, D);
    DumpByte(::core::mem::size_of::<lua_Integer>() as libc::c_ulong as libc::c_int, D);
    DumpByte(::core::mem::size_of::<lua_Number>() as libc::c_ulong as libc::c_int, D);
    DumpInteger(LUAC_INT as lua_Integer, D);
    DumpNumber(LUAC_NUM, D);
}
#[no_mangle]
pub unsafe extern "C" fn luaU_dump(
    mut L: *mut lua_State,
    mut f: *const Proto,
    mut w: lua_Writer,
    mut data: *mut libc::c_void,
    mut strip: libc::c_int,
) -> libc::c_int {
    let mut D = DumpState {
        L: 0 as *mut lua_State,
        writer: None,
        data: 0 as *mut libc::c_void,
        strip: 0,
        status: 0,
    };
    D.L = L;
    D.writer = w;
    D.data = data;
    D.strip = strip;
    D.status = 0 as libc::c_int;
    DumpHeader(&mut D);
    DumpByte((*f).sizeupvalues, &mut D);
    DumpFunction(f, NULL as *mut TString, &mut D);
    return D.status;
}
pub const LUAI_GCPAUSE: libc::c_int = 200 as libc::c_int;
pub const LUAI_GCMUL: libc::c_int = 200 as libc::c_int;
unsafe extern "C" fn makeseed(mut L: *mut lua_State) -> libc::c_uint {
    let mut buff: [libc::c_char; 32] = [0; 32];
    let mut h = luai_makeseed!();
    let mut p = 0 as libc::c_int;
    let mut t = L as size_t;
    addbuff!(
        buff, p, L
    )(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        addbuff!(buff, p, L),
        addbuff!(buff, p, L),
    );
    p = (p as libc::c_ulong).wrapping_add(addbuff!(buff, p, L)) as libc::c_int
        as libc::c_int;
    let mut t_0 = &mut h as *mut libc::c_uint as size_t;
    addbuff!(
        buff, p, & h
    )(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        addbuff!(buff, p, & h),
        addbuff!(buff, p, & h),
    );
    p = (p as libc::c_ulong).wrapping_add(addbuff!(buff, p, & h)) as libc::c_int
        as libc::c_int;
    let mut t_1 = &luaO_nilobject_ as *const TValue as size_t;
    addbuff!(
        buff, p, luaO_nilobject
    )(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        addbuff!(buff, p, luaO_nilobject),
        addbuff!(buff, p, luaO_nilobject),
    );
    p = (p as libc::c_ulong).wrapping_add(addbuff!(buff, p, luaO_nilobject))
        as libc::c_int as libc::c_int;
    let mut t_2 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(lua_Alloc, *mut libc::c_void) -> *mut lua_State>,
        size_t,
    >(
        Some(
            lua_newstate
                as unsafe extern "C" fn(lua_Alloc, *mut libc::c_void) -> *mut lua_State,
        ),
    );
    addbuff!(
        buff, p, & lua_newstate
    )(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        addbuff!(buff, p, & lua_newstate),
        addbuff!(buff, p, & lua_newstate),
    );
    p = (p as libc::c_ulong).wrapping_add(addbuff!(buff, p, & lua_newstate))
        as libc::c_int as libc::c_int;
    return luaS_hash(buff.as_mut_ptr(), p as size_t, h);
}
#[no_mangle]
pub unsafe extern "C" fn luaE_setdebt(mut g: *mut global_State, mut debt: l_mem) {
    let mut tb = gettotalbytes!(g);
    if debt < tb - MAX_LMEM as l_mem {
        debt = tb - MAX_LMEM as l_mem;
    }
    (*g).totalbytes = tb - debt;
    (*g).GCdebt = debt;
}
#[no_mangle]
pub unsafe extern "C" fn luaE_extendCI(mut L: *mut lua_State) -> *mut CallInfo {
    let mut ci = luaM_new!(L, CallInfo);
    (*(*L).ci).next = ci;
    (*ci).previous = (*L).ci;
    (*ci).next = NULL as *mut CallInfo;
    (*L).nci = ((*L).nci).wrapping_add(1);
    return ci;
}
#[no_mangle]
pub unsafe extern "C" fn luaE_freeCI(mut L: *mut lua_State) {
    let mut ci = (*L).ci;
    let mut next = (*ci).next;
    (*ci).next = NULL as *mut CallInfo;
    loop {
        ci = next;
        if ci.is_null() {
            break;
        }
        next = (*ci).next;
        luaM_free!(L, ci)(L, luaM_free!(L, ci), luaM_free!(L, ci), luaM_free!(L, ci));
        (*L).nci = ((*L).nci).wrapping_sub(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaE_shrinkCI(mut L: *mut lua_State) {
    let mut ci = (*L).ci;
    let mut next2 = 0 as *mut CallInfo;
    while !((*ci).next).is_null()
        && {
            next2 = (*(*ci).next).next;
            !next2.is_null()
        }
    {
        luaM_free!(
            L, ci -> next
        )(
            L,
            luaM_free!(L, ci -> next),
            luaM_free!(L, ci -> next),
            luaM_free!(L, ci -> next),
        );
        (*L).nci = ((*L).nci).wrapping_sub(1);
        (*ci).next = next2;
        (*next2).previous = ci;
        ci = next2;
    }
}
unsafe extern "C" fn stack_init(mut L1: *mut lua_State, mut L: *mut lua_State) {
    let mut i: libc::c_int = 0;
    let mut ci = 0 as *mut CallInfo;
    (*L1).stack = luaM_newvector!(L, BASIC_STACK_SIZE, TValue);
    (*L1).stacksize = BASIC_STACK_SIZE;
    i = 0 as libc::c_int;
    while i < BASIC_STACK_SIZE {
        let ref mut fresh9 = setnilvalue!(L1 -> stack + i);
        *fresh9 = setnilvalue!(L1 -> stack + i);
        i += 1;
    }
    (*L1).top = (*L1).stack;
    (*L1)
        .stack_last = ((*L1).stack)
        .offset((*L1).stacksize as isize)
        .offset(-(EXTRA_STACK as isize));
    ci = &mut (*L1).base_ci;
    (*ci).previous = NULL as *mut CallInfo;
    (*ci).next = (*ci).previous;
    (*ci).callstatus = 0 as libc::c_int as libc::c_ushort;
    (*ci).func = (*L1).top;
    let ref mut fresh10 = setnilvalue!(L1 -> top ++);
    *fresh10 = setnilvalue!(L1 -> top ++);
    (*ci).top = ((*L1).top).offset(LUA_MINSTACK as isize);
    (*L1).ci = ci;
}
unsafe extern "C" fn freestack(mut L: *mut lua_State) {
    if ((*L).stack).is_null() {
        return;
    }
    (*L).ci = &mut (*L).base_ci;
    luaE_freeCI(L);
    luaM_freearray!(
        L, L -> stack, L -> stacksize
    )(
        L,
        luaM_freearray!(L, L -> stack, L -> stacksize),
        luaM_freearray!(L, L -> stack, L -> stacksize),
        luaM_freearray!(L, L -> stack, L -> stacksize),
    );
}
unsafe extern "C" fn init_registry(mut L: *mut lua_State, mut g: *mut global_State) {
    let mut temp = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut registry = luaH_new(L);
    let mut io: *mut TValue = sethvalue!(L, & g -> l_registry, registry);
    let mut x_ = sethvalue!(L, & g -> l_registry, registry);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    luaH_resize(
        L,
        registry,
        LUA_RIDX_LAST as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    let mut io_0: *mut TValue = setthvalue!(L, & temp, L);
    let mut x__0 = setthvalue!(L, & temp, L);
    (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
    (*io_0).tt_ = 8 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    luaH_setint(L, registry, LUA_RIDX_MAINTHREAD as lua_Integer, &mut temp);
    let mut io_1: *mut TValue = &mut temp;
    let mut x__1 = luaH_new(L);
    (*io_1).value_.gc = &mut (*(x__1 as *mut GCUnion)).gc;
    (*io_1).tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    luaH_setint(L, registry, LUA_RIDX_GLOBALS as lua_Integer, &mut temp);
}
unsafe extern "C" fn f_luaopen(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut g = G!(L);
    stack_init(L, L);
    init_registry(L, g);
    luaS_init(L);
    luaT_init(L);
    luaX_init(L);
    (*g).gcrunning = 1 as libc::c_int as lu_byte;
    (*g).version = lua_version(NULL as *mut lua_State);
}
unsafe extern "C" fn preinit_thread(mut L: *mut lua_State, mut g: *mut global_State) {
    let ref mut fresh11 = G!(L);
    *fresh11 = g;
    (*L).stack = NULL as StkId;
    (*L).ci = NULL as *mut CallInfo;
    (*L).nci = 0 as libc::c_int as libc::c_ushort;
    (*L).stacksize = 0 as libc::c_int;
    (*L).twups = L;
    (*L).errorJmp = NULL as *mut lua_longjmp;
    (*L).nCcalls = 0 as libc::c_int as libc::c_ushort;
    ::core::ptr::write_volatile(
        &mut (*L).hook as *mut lua_Hook,
        ::core::mem::transmute::<libc::intptr_t, lua_Hook>(NULL as libc::intptr_t),
    );
    (*L).hookmask = 0 as libc::c_int;
    (*L).basehookcount = 0 as libc::c_int;
    (*L).allowhook = 1 as libc::c_int as lu_byte;
    (*L).hookcount = (*L).basehookcount;
    (*L).openupval = NULL as *mut UpVal;
    (*L).nny = 1 as libc::c_int as libc::c_ushort;
    (*L).status = LUA_OK as lu_byte;
    (*L).errfunc = 0 as libc::c_int as ptrdiff_t;
}
unsafe extern "C" fn close_state(mut L: *mut lua_State) {
    let mut g = G!(L);
    luaF_close(L, (*L).stack);
    luaC_freeallobjects(L);
    !((*g).version).is_null();
    luaM_freearray!(
        L, G(L) -> strt.hash, G(L) -> strt.size
    )(
        L,
        luaM_freearray!(L, G(L) -> strt.hash, G(L) -> strt.size),
        luaM_freearray!(L, G(L) -> strt.hash, G(L) -> strt.size),
        luaM_freearray!(L, G(L) -> strt.hash, G(L) -> strt.size),
    );
    freestack(L);
    (Some(((*g).frealloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*g).ud,
        fromstate!(L),
        ::core::mem::size_of::<LG>() as libc::c_ulong,
        0 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lua_newthread(mut L: *mut lua_State) -> *mut lua_State {
    let mut g = G!(L);
    let mut L1 = 0 as *mut lua_State;
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
    }
    L1 = &mut (*((luaM_realloc_
        as unsafe extern "C" fn(
            *mut lua_State,
            *mut libc::c_void,
            size_t,
            size_t,
        ) -> *mut libc::c_void)(
        L,
        0 as *mut libc::c_void,
        8 as libc::c_int as size_t,
        ::core::mem::size_of::<LX>() as libc::c_ulong,
    ) as *mut LX))
        .l;
    (*L1).marked = luaC_white!(g);
    (*L1).tt = LUA_TTHREAD as lu_byte;
    (*L1).next = (*g).allgc;
    (*g).allgc = obj2gco!(L1);
    let mut io = setthvalue!(L, L -> top, L1);
    let mut x_ = setthvalue!(L, L -> top, L1);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 8 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lstate.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"lua_State *lua_newthread(lua_State *)\0"))
                .as_ptr(),
        );
    }
    preinit_thread(L1, g);
    (*L1).hookmask = (*L).hookmask;
    (*L1).basehookcount = (*L).basehookcount;
    ::core::ptr::write_volatile(&mut (*L1).hook as *mut lua_Hook, (*L).hook);
    (*L1).hookcount = (*L1).basehookcount;
    memcpy(lua_getextraspace!(L1), lua_getextraspace!(g -> mainthread), LUA_EXTRASPACE);
    stack_init(L1, L);
    return L1;
}
#[no_mangle]
pub unsafe extern "C" fn luaE_freethread(mut L: *mut lua_State, mut L1: *mut lua_State) {
    let mut l = fromstate!(L1);
    luaF_close(L1, (*L1).stack);
    freestack(L1);
    luaM_free!(L, l)(L, luaM_free!(L, l), luaM_free!(L, l), luaM_free!(L, l));
}
#[no_mangle]
pub unsafe extern "C" fn lua_newstate(
    mut f: lua_Alloc,
    mut ud: *mut libc::c_void,
) -> *mut lua_State {
    let mut i: libc::c_int = 0;
    let mut L = 0 as *mut lua_State;
    let mut g = 0 as *mut global_State;
    let mut l = (Some(f.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ud,
        0 as *mut libc::c_void,
        8 as libc::c_int as size_t,
        ::core::mem::size_of::<LG>() as libc::c_ulong,
    ) as *mut LG;
    if l.is_null() {
        return NULL as *mut lua_State;
    }
    L = &mut (*l).l.l;
    g = &mut (*l).g;
    (*L).next = NULL as *mut GCObject;
    (*L).tt = LUA_TTHREAD as lu_byte;
    (*g).currentwhite = bitmask!(WHITE0BIT);
    (*L).marked = luaC_white!(g);
    preinit_thread(L, g);
    (*g).frealloc = f;
    (*g).ud = ud;
    (*g).mainthread = L;
    (*g).seed = makeseed(L);
    (*g).gcrunning = 0 as libc::c_int as lu_byte;
    (*g).GCestimate = 0 as libc::c_int as lu_mem;
    (*g).strt.nuse = 0 as libc::c_int;
    (*g).strt.size = (*g).strt.nuse;
    (*g).strt.hash = NULL as *mut *mut TString;
    let ref mut fresh12 = setnilvalue!(& g -> l_registry);
    *fresh12 = setnilvalue!(& g -> l_registry);
    (*g)
        .panic = ::core::mem::transmute::<
        libc::intptr_t,
        lua_CFunction,
    >(NULL as libc::intptr_t);
    (*g).version = NULL as *const lua_Number;
    (*g).gcstate = GCSpause as lu_byte;
    (*g).gckind = KGC_NORMAL as lu_byte;
    (*g).fixedgc = NULL as *mut GCObject;
    (*g).tobefnz = (*g).fixedgc;
    (*g).finobj = (*g).tobefnz;
    (*g).allgc = (*g).finobj;
    (*g).sweepgc = NULL as *mut *mut GCObject;
    (*g).grayagain = NULL as *mut GCObject;
    (*g).gray = (*g).grayagain;
    (*g).allweak = NULL as *mut GCObject;
    (*g).ephemeron = (*g).allweak;
    (*g).weak = (*g).ephemeron;
    (*g).twups = NULL as *mut lua_State;
    (*g).totalbytes = ::core::mem::size_of::<LG>() as libc::c_ulong as l_mem;
    (*g).GCdebt = 0 as libc::c_int as l_mem;
    (*g).gcfinnum = 0 as libc::c_int as libc::c_uint;
    (*g).gcpause = LUAI_GCPAUSE;
    (*g).gcstepmul = LUAI_GCMUL;
    i = 0 as libc::c_int;
    while i < LUA_NUMTAGS {
        (*g).mt[i as usize] = NULL as *mut Table;
        i += 1;
    }
    if luaD_rawrunprotected(
        L,
        Some(f_luaopen as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
        NULL as *mut libc::c_void,
    ) != LUA_OK
    {
        close_state(L);
        L = NULL as *mut lua_State;
    }
    return L;
}
pub const NUM_RESERVED: libc::c_int = TK_WHILE as libc::c_int - 257 as libc::c_int
    + 1 as libc::c_int;
pub const FIRST_RESERVED: libc::c_int = 257 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn lua_close(mut L: *mut lua_State) {
    L = (*G!(L)).mainthread;
    close_state(L);
}
pub const LUA_ENV: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"_ENV\0")
};
pub const GCSinsideatomic: libc::c_int = GCSpause + 1 as libc::c_int;
pub const GCSWEEPCOST: libc::c_ulong = (::core::mem::size_of::<TString>()
    as libc::c_ulong)
    .wrapping_add(4 as libc::c_int as libc::c_ulong)
    .wrapping_div(4 as libc::c_int as libc::c_ulong);
pub const GCSWEEPMAX: libc::c_ulong = ((100 as libc::c_int as libc::c_ulong)
    .wrapping_mul(::core::mem::size_of::<TString>() as libc::c_ulong) as libc::c_int
    as libc::c_ulong)
    .wrapping_div(
        (::core::mem::size_of::<TString>() as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong),
    )
    .wrapping_div(4 as libc::c_int as libc::c_ulong);
pub const GCFINALIZECOST: libc::c_ulong = GCSWEEPCOST;
pub const STEPMULADJ: libc::c_int = 200 as libc::c_int;
pub const PAUSEADJ: libc::c_int = 100 as libc::c_int;
unsafe extern "C" fn removeentry(mut n: *mut Node) {
    if (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & BIT_ISCOLLECTABLE
        != 0
        && (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).value_.gc).marked
            as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        (*n).i_key.nk.tt_ = 9 as libc::c_int + 1 as libc::c_int;
    }
}
unsafe extern "C" fn iscleared(
    mut g: *mut global_State,
    mut o: *const TValue,
) -> libc::c_int {
    if iscollectable!(o) == 0 {
        return 0 as libc::c_int
    } else if ttisstring!(o) != 0 {
        if (*((*o).value_.gc as *mut GCUnion)).ts.marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            reallymarkobject(
                g,
                &mut (*(&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString
                    as *mut GCUnion))
                    .gc,
            );
        }
        return 0 as libc::c_int;
    } else {
        return (*(*o).value_.gc).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_barrier_(
    mut L: *mut lua_State,
    mut o: *mut GCObject,
    mut v: *mut GCObject,
) {
    let mut g = G!(L);
    if keepinvariant!(g) != 0 {
        reallymarkobject(g, v);
    } else {
        (*o)
            .marked = ((*o).marked as libc::c_int
            & !((1 as libc::c_int) << 2 as libc::c_int
                | ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int))
            | ((*g).currentwhite as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int)) as lu_byte as libc::c_int)
            as lu_byte;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_barrierback_(mut L: *mut lua_State, mut t: *mut Table) {
    let mut g = G!(L);
    let ref mut fresh13 = black2gray!(t);
    *fresh13 = (*fresh13 as libc::c_int
        & !((1 as libc::c_int) << 2 as libc::c_int) as lu_byte as libc::c_int)
        as lu_byte;
    let ref mut fresh14 = linkgclist!(t, g -> grayagain);
    *fresh14 = &mut (*(t as *mut GCUnion)).gc;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_upvalbarrier_(mut L: *mut lua_State, mut uv: *mut UpVal) {
    let mut g = G!(L);
    let mut o = gcvalue!(uv -> v);
    if keepinvariant!(g) != 0 {
        if (*o).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            markobject!(g, o)(g, &mut (*(o as *mut GCUnion)).gc);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaC_fix(mut L: *mut lua_State, mut o: *mut GCObject) {
    let mut g = G!(L);
    let ref mut fresh15 = white2gray!(o);
    *fresh15 = (*fresh15 as libc::c_int
        & !((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) as lu_byte as libc::c_int)
        as lu_byte;
    (*g).allgc = (*o).next;
    (*o).next = (*g).fixedgc;
    (*g).fixedgc = o;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_newobj(
    mut L: *mut lua_State,
    mut tt: libc::c_int,
    mut sz: size_t,
) -> *mut GCObject {
    let mut g = G!(L);
    let mut o = luaM_realloc_(
        L,
        0 as *mut libc::c_void,
        (tt & 0xf as libc::c_int) as size_t,
        sz,
    ) as *mut GCObject;
    (*o).marked = luaC_white!(g);
    (*o).tt = tt as lu_byte;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    return o;
}
unsafe extern "C" fn reallymarkobject(mut g: *mut global_State, mut o: *mut GCObject) {
    loop {
        let ref mut fresh16 = white2gray!(o);
        *fresh16 = (*fresh16 as libc::c_int
            & !((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) as lu_byte as libc::c_int)
            as lu_byte;
        match (*o).tt as libc::c_int {
            LUA_TSHRSTR => {
                let ref mut fresh17 = gray2black!(o);
                *fresh17 = (*fresh17 as libc::c_int | gray2black!(o)) as lu_byte;
                (*g)
                    .GCmemtrav = ((*g).GCmemtrav as libc::c_ulong)
                    .wrapping_add(sizelstring!(gco2ts(o) -> shrlen)) as lu_mem as lu_mem;
                break;
            }
            LUA_TLNGSTR => {
                let ref mut fresh18 = gray2black!(o);
                *fresh18 = (*fresh18 as libc::c_int | gray2black!(o)) as lu_byte;
                (*g)
                    .GCmemtrav = ((*g).GCmemtrav as libc::c_ulong)
                    .wrapping_add(sizelstring!(gco2ts(o) -> u.lnglen)) as lu_mem
                    as lu_mem;
                break;
            }
            LUA_TUSERDATA => {
                let mut uvalue = TValue {
                    value_: Value { gc: 0 as *mut GCObject },
                    tt_: 0,
                };
                if !((*(o as *mut GCUnion)).u.metatable).is_null() {
                    if (*(*(o as *mut GCUnion)).u.metatable).marked as libc::c_int
                        & ((1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int) != 0
                    {
                        reallymarkobject(
                            g,
                            &mut (*((*(o as *mut GCUnion)).u.metatable as *mut GCUnion))
                                .gc,
                        );
                    }
                }
                let ref mut fresh19 = gray2black!(o);
                *fresh19 = (*fresh19 as libc::c_int | gray2black!(o)) as lu_byte;
                (*g)
                    .GCmemtrav = ((*g).GCmemtrav as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<UUdata>() as libc::c_ulong)
                            .wrapping_add((*(o as *mut GCUnion)).u.len),
                    ) as lu_mem as lu_mem;
                let mut io: *mut TValue = getuservalue!(
                    g -> mainthread, gco2u(o), & uvalue
                );
                let mut iu: *const Udata = getuservalue!(
                    g -> mainthread, gco2u(o), & uvalue
                );
                let ref mut fresh20 = getuservalue!(g -> mainthread, gco2u(o), & uvalue);
                *fresh20 = getuservalue!(g -> mainthread, gco2u(o), & uvalue);
                (*io).tt_ = (*iu).ttuv_ as libc::c_int;
                if !(valiswhite!(& uvalue) != 0) {
                    break;
                }
                o = gcvalue!(& uvalue);
            }
            LUA_TLCL => {
                let ref mut fresh21 = linkgclist!(gco2lcl(o), g -> gray);
                *fresh21 = &mut (*(&mut (*(o as *mut GCUnion)).cl.l as *mut LClosure
                    as *mut GCUnion))
                    .gc;
                break;
            }
            LUA_TCCL => {
                let ref mut fresh22 = linkgclist!(gco2ccl(o), g -> gray);
                *fresh22 = &mut (*(&mut (*(o as *mut GCUnion)).cl.c as *mut CClosure
                    as *mut GCUnion))
                    .gc;
                break;
            }
            LUA_TTABLE => {
                let ref mut fresh23 = linkgclist!(gco2t(o), g -> gray);
                *fresh23 = &mut (*(&mut (*(o as *mut GCUnion)).h as *mut Table
                    as *mut GCUnion))
                    .gc;
                break;
            }
            LUA_TTHREAD => {
                let ref mut fresh24 = linkgclist!(gco2th(o), g -> gray);
                *fresh24 = &mut (*(&mut (*(o as *mut GCUnion)).th as *mut lua_State
                    as *mut GCUnion))
                    .gc;
                break;
            }
            LUA_TPROTO => {
                let ref mut fresh25 = linkgclist!(gco2p(o), g -> gray);
                *fresh25 = &mut (*(&mut (*(o as *mut GCUnion)).p as *mut Proto
                    as *mut GCUnion))
                    .gc;
                break;
            }
            _ => {
                break;
            }
        }
    };
}
unsafe extern "C" fn markmt(mut g: *mut global_State) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < LUA_NUMTAGS {
        if !((*g).mt[i as usize]).is_null() {
            if (*(*g).mt[i as usize]).marked as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                reallymarkobject(
                    g,
                    &mut (*(*((*g).mt).as_mut_ptr().offset(i as isize) as *mut GCUnion))
                        .gc,
                );
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn markbeingfnz(mut g: *mut global_State) {
    let mut o = 0 as *mut GCObject;
    o = (*g).tobefnz;
    while !o.is_null() {
        if (*o).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            markobject!(g, o)(g, &mut (*(o as *mut GCUnion)).gc);
        }
        o = (*o).next;
    }
}
unsafe extern "C" fn remarkupvals(mut g: *mut global_State) {
    let mut thread = 0 as *mut lua_State;
    let mut p: *mut *mut lua_State = &mut (*g).twups;
    loop {
        thread = *p;
        if thread.is_null() {
            break;
        }
        if (*thread).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int) == 0
            && !((*thread).openupval).is_null()
        {
            p = &mut (*thread).twups;
        } else {
            let mut uv = 0 as *mut UpVal;
            *p = (*thread).twups;
            (*thread).twups = thread;
            uv = (*thread).openupval;
            while !uv.is_null() {
                if (*uv).u.open.touched != 0 {
                    if (*(*uv).v).tt_ & BIT_ISCOLLECTABLE != 0
                        && (*(*(*uv).v).value_.gc).marked as libc::c_int
                            & ((1 as libc::c_int) << 0 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int) != 0
                    {
                        markvalue!(g, uv -> v)(g, (*(*uv).v).value_.gc);
                    }
                    (*uv).u.open.touched = 0 as libc::c_int;
                }
                uv = (*uv).u.open.next;
            }
        }
    };
}
unsafe extern "C" fn restartcollection(mut g: *mut global_State) {
    (*g).grayagain = NULL as *mut GCObject;
    (*g).gray = (*g).grayagain;
    (*g).ephemeron = NULL as *mut GCObject;
    (*g).allweak = (*g).ephemeron;
    (*g).weak = (*g).allweak;
    if (*(*g).mainthread).marked as libc::c_int
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        markobject!(g, g -> mainthread)(g, &mut (*((*g).mainthread as *mut GCUnion)).gc);
    }
    if (*g).l_registry.tt_ & BIT_ISCOLLECTABLE != 0
        && (*(*g).l_registry.value_.gc).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        markvalue!(g, & g -> l_registry)(g, (*g).l_registry.value_.gc);
    }
    markmt(g);
    markbeingfnz(g);
}
unsafe extern "C" fn traverseweakvalue(mut g: *mut global_State, mut h: *mut Table) {
    let mut n = 0 as *mut Node;
    let mut limit: *mut Node = gnodelast!(h);
    let mut hasclears = ((*h).sizearray > 0 as libc::c_int as libc::c_uint)
        as libc::c_int;
    n = gnode!(h, 0);
    while n < limit {
        if (*n).i_val.tt_ == 0 as libc::c_int {
            removeentry(n);
        } else {
            if (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_
                & BIT_ISCOLLECTABLE != 0
                && (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).value_.gc)
                    .marked as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                reallymarkobject(
                    g,
                    (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).value_.gc,
                );
            }
            if hasclears == 0 && iscleared(g, gval!(n)) != 0 {
                hasclears = 1 as libc::c_int;
            }
        }
        n = n.offset(1);
    }
    if (*g).gcstate as libc::c_int == GCSpropagate {
        let ref mut fresh26 = linkgclist!(h, g -> grayagain);
        *fresh26 = &mut (*(h as *mut GCUnion)).gc;
    } else if hasclears != 0 {
        let ref mut fresh27 = linkgclist!(h, g -> weak);
        *fresh27 = &mut (*(h as *mut GCUnion)).gc;
    }
}
unsafe extern "C" fn traverseephemeron(
    mut g: *mut global_State,
    mut h: *mut Table,
) -> libc::c_int {
    let mut marked = 0 as libc::c_int;
    let mut hasclears = 0 as libc::c_int;
    let mut hasww = 0 as libc::c_int;
    let mut n = 0 as *mut Node;
    let mut limit: *mut Node = gnodelast!(h);
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*h).sizearray {
        if valiswhite!(& h -> array[i]) != 0 {
            marked = 1 as libc::c_int;
            reallymarkobject(g, gcvalue!(& h -> array[i]));
        }
        i = i.wrapping_add(1);
    }
    n = gnode!(h, 0);
    while n < limit {
        if (*n).i_val.tt_ == 0 as libc::c_int {
            removeentry(n);
        } else if iscleared(g, gkey!(n)) != 0 {
            hasclears = 1 as libc::c_int;
            if (*n).i_val.tt_ & BIT_ISCOLLECTABLE != 0
                && (*(*n).i_val.value_.gc).marked as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                hasww = 1 as libc::c_int;
            }
        } else if (*n).i_val.tt_ & BIT_ISCOLLECTABLE != 0
            && (*(*n).i_val.value_.gc).marked as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            marked = 1 as libc::c_int;
            reallymarkobject(g, (*n).i_val.value_.gc);
        }
        n = n.offset(1);
    }
    if (*g).gcstate as libc::c_int == GCSpropagate {
        let ref mut fresh28 = linkgclist!(h, g -> grayagain);
        *fresh28 = &mut (*(h as *mut GCUnion)).gc;
    } else if hasww != 0 {
        let ref mut fresh29 = linkgclist!(h, g -> ephemeron);
        *fresh29 = &mut (*(h as *mut GCUnion)).gc;
    } else if hasclears != 0 {
        let ref mut fresh30 = linkgclist!(h, g -> allweak);
        *fresh30 = &mut (*(h as *mut GCUnion)).gc;
    }
    return marked;
}
unsafe extern "C" fn traversestrongtable(mut g: *mut global_State, mut h: *mut Table) {
    let mut n = 0 as *mut Node;
    let mut limit: *mut Node = gnodelast!(h);
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*h).sizearray {
        if (*((*h).array).offset(i as isize)).tt_ & BIT_ISCOLLECTABLE != 0
            && (*(*((*h).array).offset(i as isize)).value_.gc).marked as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            markvalue!(
                g, & h -> array[i]
            )(g, (*((*h).array).offset(i as isize)).value_.gc);
        }
        i = i.wrapping_add(1);
    }
    n = gnode!(h, 0);
    while n < limit {
        if (*n).i_val.tt_ == 0 as libc::c_int {
            removeentry(n);
        } else {
            if (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_
                & BIT_ISCOLLECTABLE != 0
                && (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).value_.gc)
                    .marked as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                reallymarkobject(
                    g,
                    (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).value_.gc,
                );
            }
            if (*n).i_val.tt_ & BIT_ISCOLLECTABLE != 0
                && (*(*n).i_val.value_.gc).marked as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                reallymarkobject(g, (*n).i_val.value_.gc);
            }
        }
        n = n.offset(1);
    }
}
unsafe extern "C" fn traversetable(
    mut g: *mut global_State,
    mut h: *mut Table,
) -> lu_mem {
    let mut weakkey = 0 as *const libc::c_char;
    let mut weakvalue = 0 as *const libc::c_char;
    let mut mode = gfasttm!(g, h -> metatable, TM_MODE);
    if !((*h).metatable).is_null() {
        if (*(*h).metatable).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            reallymarkobject(g, &mut (*((*h).metatable as *mut GCUnion)).gc);
        }
    }
    if !mode.is_null() && ttisstring!(mode) != 0
        && {
            weakkey = strchr(svalue!(mode), 'k' as i32);
            weakvalue = strchr(svalue!(mode), 'v' as i32);
            !weakkey.is_null() || !weakvalue.is_null()
        }
    {
        let ref mut fresh31 = black2gray!(h);
        *fresh31 = (*fresh31 as libc::c_int
            & !((1 as libc::c_int) << 2 as libc::c_int) as lu_byte as libc::c_int)
            as lu_byte;
        if weakkey.is_null() {
            traverseweakvalue(g, h);
        } else if weakvalue.is_null() {
            traverseephemeron(g, h);
        } else {
            let ref mut fresh32 = linkgclist!(h, g -> allweak);
            *fresh32 = &mut (*(h as *mut GCUnion)).gc;
        }
    } else {
        traversestrongtable(g, h);
    }
    return (::core::mem::size_of::<Table>() as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<TValue>() as libc::c_ulong)
                .wrapping_mul((*h).sizearray as libc::c_ulong),
        )
        .wrapping_add(
            (::core::mem::size_of::<Node>() as libc::c_ulong)
                .wrapping_mul(
                    (if ((*h).lastfree).is_null() {
                        0 as libc::c_int
                    } else {
                        (1 as libc::c_int) << (*h).lsizenode as libc::c_int
                    }) as size_t,
                ),
        );
}
unsafe extern "C" fn traverseproto(
    mut g: *mut global_State,
    mut f: *mut Proto,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if !((*f).cache).is_null() && iswhite!(f -> cache) != 0 {
        (*f).cache = NULL as *mut LClosure;
    }
    if !((*f).source).is_null() {
        if (*(*f).source).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            reallymarkobject(g, &mut (*((*f).source as *mut GCUnion)).gc);
        }
    }
    i = 0 as libc::c_int;
    while i < (*f).sizek {
        if (*((*f).k).offset(i as isize)).tt_ & BIT_ISCOLLECTABLE != 0
            && (*(*((*f).k).offset(i as isize)).value_.gc).marked as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            markvalue!(g, & f -> k[i])(g, (*((*f).k).offset(i as isize)).value_.gc);
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*f).sizeupvalues {
        if !((*((*f).upvalues).offset(i as isize)).name).is_null() {
            if (*(*((*f).upvalues).offset(i as isize)).name).marked as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                reallymarkobject(
                    g,
                    &mut (*((*((*f).upvalues).offset(i as isize)).name as *mut GCUnion))
                        .gc,
                );
            }
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*f).sizep {
        if !(*((*f).p).offset(i as isize)).is_null() {
            if (**((*f).p).offset(i as isize)).marked as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                reallymarkobject(
                    g,
                    &mut (*(*((*f).p).offset(i as isize) as *mut GCUnion)).gc,
                );
            }
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*f).sizelocvars {
        if !((*((*f).locvars).offset(i as isize)).varname).is_null() {
            if (*(*((*f).locvars).offset(i as isize)).varname).marked as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                reallymarkobject(
                    g,
                    &mut (*((*((*f).locvars).offset(i as isize)).varname
                        as *mut GCUnion))
                        .gc,
                );
            }
        }
        i += 1;
    }
    return (::core::mem::size_of::<Proto>() as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<Instruction>() as libc::c_ulong)
                .wrapping_mul((*f).sizecode as libc::c_ulong),
        )
        .wrapping_add(
            (::core::mem::size_of::<*mut Proto>() as libc::c_ulong)
                .wrapping_mul((*f).sizep as libc::c_ulong),
        )
        .wrapping_add(
            (::core::mem::size_of::<TValue>() as libc::c_ulong)
                .wrapping_mul((*f).sizek as libc::c_ulong),
        )
        .wrapping_add(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul((*f).sizelineinfo as libc::c_ulong),
        )
        .wrapping_add(
            (::core::mem::size_of::<LocVar>() as libc::c_ulong)
                .wrapping_mul((*f).sizelocvars as libc::c_ulong),
        )
        .wrapping_add(
            (::core::mem::size_of::<Upvaldesc>() as libc::c_ulong)
                .wrapping_mul((*f).sizeupvalues as libc::c_ulong),
        ) as libc::c_int;
}
unsafe extern "C" fn traverseCclosure(
    mut g: *mut global_State,
    mut cl: *mut CClosure,
) -> lu_mem {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cl).nupvalues as libc::c_int {
        if (*((*cl).upvalue).as_mut_ptr().offset(i as isize)).tt_ & BIT_ISCOLLECTABLE
            != 0
            && (*(*((*cl).upvalue).as_mut_ptr().offset(i as isize)).value_.gc).marked
                as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            markvalue!(
                g, & cl -> upvalue[i]
            )(g, (*((*cl).upvalue).as_mut_ptr().offset(i as isize)).value_.gc);
        }
        i += 1;
    }
    return sizeCclosure!(cl -> nupvalues);
}
unsafe extern "C" fn traverseLclosure(
    mut g: *mut global_State,
    mut cl: *mut LClosure,
) -> lu_mem {
    let mut i: libc::c_int = 0;
    if !((*cl).p).is_null() {
        if (*(*cl).p).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            reallymarkobject(g, &mut (*((*cl).p as *mut GCUnion)).gc);
        }
    }
    i = 0 as libc::c_int;
    while i < (*cl).nupvalues as libc::c_int {
        let mut uv = *((*cl).upvals).as_mut_ptr().offset(i as isize);
        if !uv.is_null() {
            if upisopen!(uv) != 0 && (*g).gcstate as libc::c_int != GCSinsideatomic {
                (*uv).u.open.touched = 1 as libc::c_int;
            } else if (*(*uv).v).tt_ & BIT_ISCOLLECTABLE != 0
                && (*(*(*uv).v).value_.gc).marked as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
            {
                markvalue!(g, uv -> v)(g, (*(*uv).v).value_.gc);
            }
        }
        i += 1;
    }
    return sizeLclosure!(cl -> nupvalues);
}
unsafe extern "C" fn traversethread(
    mut g: *mut global_State,
    mut th: *mut lua_State,
) -> lu_mem {
    let mut o = (*th).stack;
    if o.is_null() {
        return 1 as libc::c_int as lu_mem;
    }
    while o < (*th).top {
        if (*o).tt_ & BIT_ISCOLLECTABLE != 0
            && (*(*o).value_.gc).marked as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
        {
            markvalue!(g, o)(g, (*o).value_.gc);
        }
        o = o.offset(1);
    }
    if (*g).gcstate as libc::c_int == GCSinsideatomic {
        let mut lim = ((*th).stack).offset((*th).stacksize as isize);
        while o < lim {
            let ref mut fresh33 = setnilvalue!(o);
            *fresh33 = setnilvalue!(o);
            o = o.offset(1);
        }
        if isintwups!(th) == 0 && !((*th).openupval).is_null() {
            (*th).twups = (*g).twups;
            (*g).twups = th;
        }
    } else if (*g).gckind as libc::c_int != KGC_EMERGENCY {
        luaD_shrinkstack(th);
    }
    return (::core::mem::size_of::<lua_State>() as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<TValue>() as libc::c_ulong)
                .wrapping_mul((*th).stacksize as libc::c_ulong),
        )
        .wrapping_add(
            (::core::mem::size_of::<CallInfo>() as libc::c_ulong)
                .wrapping_mul((*th).nci as libc::c_ulong),
        );
}
unsafe extern "C" fn propagatemark(mut g: *mut global_State) {
    let mut size: lu_mem = 0;
    let mut o = (*g).gray;
    let ref mut fresh34 = gray2black!(o);
    *fresh34 = (*fresh34 as libc::c_int | gray2black!(o)) as lu_byte;
    match (*o).tt as libc::c_int {
        LUA_TTABLE => {
            let mut h: *mut Table = gco2t!(o);
            (*g).gray = (*h).gclist;
            size = traversetable(g, h);
        }
        LUA_TLCL => {
            let mut cl: *mut LClosure = gco2lcl!(o);
            (*g).gray = (*cl).gclist;
            size = traverseLclosure(g, cl);
        }
        LUA_TCCL => {
            let mut cl_0: *mut CClosure = gco2ccl!(o);
            (*g).gray = (*cl_0).gclist;
            size = traverseCclosure(g, cl_0);
        }
        LUA_TTHREAD => {
            let mut th: *mut lua_State = gco2th!(o);
            (*g).gray = (*th).gclist;
            let ref mut fresh35 = linkgclist!(th, g -> grayagain);
            *fresh35 = &mut (*(th as *mut GCUnion)).gc;
            let ref mut fresh36 = black2gray!(o);
            *fresh36 = (*fresh36 as libc::c_int
                & !((1 as libc::c_int) << 2 as libc::c_int) as lu_byte as libc::c_int)
                as lu_byte;
            size = traversethread(g, th);
        }
        LUA_TPROTO => {
            let mut p: *mut Proto = gco2p!(o);
            (*g).gray = (*p).gclist;
            size = traverseproto(g, p) as lu_mem;
        }
        _ => return,
    }
    (*g)
        .GCmemtrav = ((*g).GCmemtrav as libc::c_ulong).wrapping_add(size) as lu_mem
        as lu_mem;
}
unsafe extern "C" fn propagateall(mut g: *mut global_State) {
    while !((*g).gray).is_null() {
        propagatemark(g);
    }
}
unsafe extern "C" fn convergeephemerons(mut g: *mut global_State) {
    let mut changed: libc::c_int = 0;
    loop {
        let mut w = 0 as *mut GCObject;
        let mut next = (*g).ephemeron;
        (*g).ephemeron = NULL as *mut GCObject;
        changed = 0 as libc::c_int;
        loop {
            w = next;
            if w.is_null() {
                break;
            }
            next = (*(w as *mut GCUnion)).h.gclist;
            if traverseephemeron(g, gco2t!(w)) != 0 {
                propagateall(g);
                changed = 1 as libc::c_int;
            }
        }
        if !(changed != 0) {
            break;
        }
    };
}
unsafe extern "C" fn clearkeys(
    mut g: *mut global_State,
    mut l: *mut GCObject,
    mut f: *mut GCObject,
) {
    while l != f {
        let mut h: *mut Table = gco2t!(l);
        let mut n = 0 as *mut Node;
        let mut limit: *mut Node = gnodelast!(h);
        n = gnode!(h, 0);
        while n < limit {
            if !((*n).i_val.tt_ == 0 as libc::c_int) && iscleared(g, gkey!(n)) != 0 {
                (*n).i_val.tt_ = 0 as libc::c_int;
            }
            if (*n).i_val.tt_ == 0 as libc::c_int {
                removeentry(n);
            }
            n = n.offset(1);
        }
        l = (*(l as *mut GCUnion)).h.gclist;
    }
}
unsafe extern "C" fn clearvalues(
    mut g: *mut global_State,
    mut l: *mut GCObject,
    mut f: *mut GCObject,
) {
    while l != f {
        let mut h: *mut Table = gco2t!(l);
        let mut n = 0 as *mut Node;
        let mut limit: *mut Node = gnodelast!(h);
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*h).sizearray {
            let mut o: *mut TValue = &mut *((*h).array).offset(i as isize)
                as *mut TValue;
            if iscleared(g, o) != 0 {
                let ref mut fresh37 = setnilvalue!(o);
                *fresh37 = setnilvalue!(o);
            }
            i = i.wrapping_add(1);
        }
        n = gnode!(h, 0);
        while n < limit {
            if !((*n).i_val.tt_ == 0 as libc::c_int) && iscleared(g, gval!(n)) != 0 {
                (*n).i_val.tt_ = 0 as libc::c_int;
                removeentry(n);
            }
            n = n.offset(1);
        }
        l = (*(l as *mut GCUnion)).h.gclist;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaC_upvdeccount(mut L: *mut lua_State, mut uv: *mut UpVal) {
    (*uv).refcount = ((*uv).refcount).wrapping_sub(1);
    if (*uv).refcount == 0 as libc::c_int as libc::c_ulong && upisopen!(uv) == 0 {
        luaM_free!(L, uv)(L, luaM_free!(L, uv), luaM_free!(L, uv), luaM_free!(L, uv));
    }
}
unsafe extern "C" fn freeLclosure(mut L: *mut lua_State, mut cl: *mut LClosure) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cl).nupvalues as libc::c_int {
        let mut uv = *((*cl).upvals).as_mut_ptr().offset(i as isize);
        if !uv.is_null() {
            luaC_upvdeccount(L, uv);
        }
        i += 1;
    }
    luaM_realloc_(
        L,
        cl as *mut libc::c_void,
        (::core::mem::size_of::<LClosure>() as libc::c_ulong as libc::c_int
            + (::core::mem::size_of::<*mut TValue>() as libc::c_ulong)
                .wrapping_mul(
                    ((*cl).nupvalues as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                ) as libc::c_int) as size_t,
        0 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn freeobj(mut L: *mut lua_State, mut o: *mut GCObject) {
    match (*o).tt as libc::c_int {
        LUA_TPROTO => {
            luaF_freeproto(L, gco2p!(o));
        }
        LUA_TLCL => {
            freeLclosure(L, gco2lcl!(o));
        }
        LUA_TCCL => {
            luaM_realloc_(
                L,
                o as *mut libc::c_void,
                (::core::mem::size_of::<CClosure>() as libc::c_ulong as libc::c_int
                    + (::core::mem::size_of::<TValue>() as libc::c_ulong)
                        .wrapping_mul(
                            ((*(o as *mut GCUnion)).cl.c.nupvalues as libc::c_int
                                - 1 as libc::c_int) as libc::c_ulong,
                        ) as libc::c_int) as size_t,
                0 as libc::c_int as size_t,
            );
        }
        LUA_TTABLE => {
            luaH_free(L, gco2t!(o));
        }
        LUA_TTHREAD => {
            luaE_freethread(L, gco2th!(o));
        }
        LUA_TUSERDATA => {
            luaM_realloc_(
                L,
                o as *mut libc::c_void,
                (::core::mem::size_of::<UUdata>() as libc::c_ulong)
                    .wrapping_add((*(o as *mut GCUnion)).u.len),
                0 as libc::c_int as size_t,
            );
        }
        LUA_TSHRSTR => {
            luaS_remove(L, gco2ts!(o));
            luaM_realloc_(
                L,
                o as *mut libc::c_void,
                (::core::mem::size_of::<UTString>() as libc::c_ulong)
                    .wrapping_add(
                        (((*(o as *mut GCUnion)).ts.shrlen as libc::c_int
                            + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
                0 as libc::c_int as size_t,
            );
        }
        LUA_TLNGSTR => {
            luaM_realloc_(
                L,
                o as *mut libc::c_void,
                (::core::mem::size_of::<UTString>() as libc::c_ulong)
                    .wrapping_add(
                        ((*(o as *mut GCUnion)).ts.u.lnglen)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
                0 as libc::c_int as size_t,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn sweeplist(
    mut L: *mut lua_State,
    mut p: *mut *mut GCObject,
    mut count: lu_mem,
) -> *mut *mut GCObject {
    let mut g = G!(L);
    let mut ow = otherwhite!(g);
    let mut white = luaC_white!(g);
    while !(*p).is_null()
        && {
            let fresh38 = count;
            count = count.wrapping_sub(1);
            fresh38 > 0 as libc::c_int as libc::c_ulong
        }
    {
        let mut curr = *p;
        let mut marked = (*curr).marked as libc::c_int;
        if isdeadm!(ow, marked) == 0 {
            *p = (*curr).next;
            freeobj(L, curr);
        } else {
            (*curr).marked = cast_byte!((marked & maskcolors) | white);
            p = &mut (*curr).next;
        }
    }
    return if (*p).is_null() { NULL as *mut *mut GCObject } else { p };
}
unsafe extern "C" fn sweeptolive(
    mut L: *mut lua_State,
    mut p: *mut *mut GCObject,
) -> *mut *mut GCObject {
    let mut old = p;
    loop {
        p = sweeplist(L, p, 1 as libc::c_int as lu_mem);
        if !(p == old) {
            break;
        }
    }
    return p;
}
unsafe extern "C" fn checkSizes(mut L: *mut lua_State, mut g: *mut global_State) {
    if (*g).gckind as libc::c_int != KGC_EMERGENCY {
        let mut olddebt = (*g).GCdebt;
        if (*g).strt.nuse < (*g).strt.size / 4 as libc::c_int {
            luaS_resize(L, (*g).strt.size / 2 as libc::c_int);
        }
        (*g)
            .GCestimate = ((*g).GCestimate as libc::c_ulong)
            .wrapping_add(((*g).GCdebt - olddebt) as libc::c_ulong) as lu_mem as lu_mem;
    }
}
unsafe extern "C" fn udata2finalize(mut g: *mut global_State) -> *mut GCObject {
    let mut o = (*g).tobefnz;
    (*g).tobefnz = (*o).next;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    let ref mut fresh39 = resetbit!(o -> marked, FINALIZEDBIT);
    *fresh39 = (*fresh39 as libc::c_int
        & !((1 as libc::c_int) << 3 as libc::c_int) as lu_byte as libc::c_int)
        as lu_byte;
    if issweepphase!(g) != 0 {
        (*o)
            .marked = ((*o).marked as libc::c_int
            & !((1 as libc::c_int) << 2 as libc::c_int
                | ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int))
            | ((*g).currentwhite as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int)) as lu_byte as libc::c_int)
            as lu_byte;
    }
    return o;
}
unsafe extern "C" fn dothecall(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    luaD_callnoyield(
        L,
        ((*L).top).offset(-(2 as libc::c_int as isize)),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn GCTM(mut L: *mut lua_State, mut propagateerrors: libc::c_int) {
    let mut g = G!(L);
    let mut tm = 0 as *const TValue;
    let mut v = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut v;
    let mut i_g = udata2finalize(g);
    (*io).value_.gc = i_g;
    (*io).tt_ = (*i_g).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    tm = luaT_gettmbyobj(L, &mut v, TM_GC);
    if !tm.is_null() && ttisfunction!(tm) != 0 {
        let mut status: libc::c_int = 0;
        let mut oldah = (*L).allowhook;
        let mut running = (*g).gcrunning as libc::c_int;
        (*L).allowhook = 0 as libc::c_int as lu_byte;
        (*g).gcrunning = 0 as libc::c_int as lu_byte;
        let mut io1 = setobj2s!(L, L -> top, tm);
        let ref mut fresh40 = setobj2s!(L, L -> top, tm);
        *fresh40 = setobj2s!(L, L -> top, tm);
        let mut io1_0 = setobj2s!(L, L -> top + 1, & v);
        let ref mut fresh41 = setobj2s!(L, L -> top + 1, & v);
        *fresh41 = setobj2s!(L, L -> top + 1, & v);
        (*L).top = ((*L).top).offset(2 as libc::c_int as isize);
        (*(*L).ci)
            .callstatus = ((*(*L).ci).callstatus as libc::c_int | CIST_FIN)
            as libc::c_ushort;
        status = luaD_pcall(
            L,
            Some(
                dothecall
                    as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
            ),
            NULL as *mut libc::c_void,
            savestack!(L, L -> top - 2),
            0 as libc::c_int as ptrdiff_t,
        );
        (*(*L).ci)
            .callstatus = ((*(*L).ci).callstatus as libc::c_int & !CIST_FIN)
            as libc::c_ushort;
        (*L).allowhook = oldah;
        (*g).gcrunning = running as lu_byte;
        if status != LUA_OK && propagateerrors != 0 {
            if status == LUA_ERRRUN {
                let mut msg = if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_
                    & 0xf as libc::c_int == ttisstring!(L -> top - 1)
                {
                    svalue!(L -> top - 1)
                } else {
                    b"no message\0" as *const u8 as *const libc::c_char
                };
                luaO_pushfstring(
                    L,
                    b"error in __gc metamethod (%s)\0" as *const u8
                        as *const libc::c_char,
                    msg,
                );
                status = LUA_ERRGCMM;
            }
            luaD_throw(L, status);
        }
    }
}
unsafe extern "C" fn runafewfinalizers(mut L: *mut lua_State) -> libc::c_int {
    let mut g = G!(L);
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while !((*g).tobefnz).is_null() && i < (*g).gcfinnum {
        GCTM(L, 1 as libc::c_int);
        i = i.wrapping_add(1);
    }
    (*g)
        .gcfinnum = if ((*g).tobefnz).is_null() {
        0 as libc::c_int as libc::c_uint
    } else {
        ((*g).gcfinnum).wrapping_mul(2 as libc::c_int as libc::c_uint)
    };
    return i as libc::c_int;
}
unsafe extern "C" fn callallpendingfinalizers(mut L: *mut lua_State) {
    let mut g = G!(L);
    while !((*g).tobefnz).is_null() {
        GCTM(L, 0 as libc::c_int);
    }
}
unsafe extern "C" fn findlast(mut p: *mut *mut GCObject) -> *mut *mut GCObject {
    while !(*p).is_null() {
        p = &mut (**p).next;
    }
    return p;
}
unsafe extern "C" fn separatetobefnz(mut g: *mut global_State, mut all: libc::c_int) {
    let mut curr = 0 as *mut GCObject;
    let mut p: *mut *mut GCObject = &mut (*g).finobj;
    let mut lastnext = findlast(&mut (*g).tobefnz);
    loop {
        curr = *p;
        if curr.is_null() {
            break;
        }
        if !(iswhite!(curr) != 0 || all != 0) {
            p = &mut (*curr).next;
        } else {
            *p = (*curr).next;
            (*curr).next = *lastnext;
            *lastnext = curr;
            lastnext = &mut (*curr).next;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_checkfinalizer(
    mut L: *mut lua_State,
    mut o: *mut GCObject,
    mut mt: *mut Table,
) {
    let mut g = G!(L);
    if tofinalize!(o) != 0 || gfasttm!(g, mt, TM_GC).is_null() {
        return
    } else {
        let mut p = 0 as *mut *mut GCObject;
        if issweepphase!(g) != 0 {
            (*o)
                .marked = ((*o).marked as libc::c_int
                & !((1 as libc::c_int) << 2 as libc::c_int
                    | ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int))
                | ((*g).currentwhite as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int)) as lu_byte
                    as libc::c_int) as lu_byte;
            if (*g).sweepgc == &mut (*o).next as *mut *mut GCObject {
                (*g).sweepgc = sweeptolive(L, (*g).sweepgc);
            }
        }
        p = &mut (*g).allgc;
        while *p != o {
            p = &mut (**p).next;
        }
        *p = (*o).next;
        (*o).next = (*g).finobj;
        (*g).finobj = o;
        let ref mut fresh42 = l_setbit!(o -> marked, FINALIZEDBIT);
        *fresh42 = (*fresh42 as libc::c_int | l_setbit!(o -> marked, FINALIZEDBIT))
            as lu_byte;
    };
}
unsafe extern "C" fn setpause(mut g: *mut global_State) {
    let mut threshold: l_mem = 0;
    let mut debt: l_mem = 0;
    let mut estimate = ((*g).GCestimate).wrapping_div(PAUSEADJ as libc::c_ulong)
        as l_mem;
    threshold = if ((*g).gcpause as libc::c_long) < MAX_LMEM as l_mem / estimate {
        estimate * (*g).gcpause as libc::c_long
    } else {
        MAX_LMEM as l_mem
    };
    debt = gettotalbytes!(g).wrapping_sub(threshold as libc::c_ulong) as l_mem;
    luaE_setdebt(g, debt);
}
unsafe extern "C" fn entersweep(mut L: *mut lua_State) {
    let mut g = G!(L);
    (*g).gcstate = GCSswpallgc as lu_byte;
    (*g).sweepgc = sweeplist(L, &mut (*g).allgc, 1 as libc::c_int as lu_mem);
}
#[no_mangle]
pub unsafe extern "C" fn luaC_freeallobjects(mut L: *mut lua_State) {
    let mut g = G!(L);
    separatetobefnz(g, 1 as libc::c_int);
    callallpendingfinalizers(L);
    (*g).currentwhite = WHITEBITS as lu_byte;
    (*g).gckind = KGC_NORMAL as lu_byte;
    sweepwholelist!(L, & g -> finobj)(L, &mut (*g).finobj, MAX_LUMEM);
    sweepwholelist!(L, & g -> allgc)(L, &mut (*g).allgc, MAX_LUMEM);
    sweepwholelist!(L, & g -> fixedgc)(L, &mut (*g).fixedgc, MAX_LUMEM);
}
unsafe extern "C" fn atomic(mut L: *mut lua_State) -> l_mem {
    let mut g = G!(L);
    let mut work: l_mem = 0;
    let mut origweak = 0 as *mut GCObject;
    let mut origall = 0 as *mut GCObject;
    let mut grayagain = (*g).grayagain;
    (*g).gcstate = GCSinsideatomic as lu_byte;
    (*g).GCmemtrav = 0 as libc::c_int as lu_mem;
    if (*L).marked as libc::c_int
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        markobject!(g, L)(g, &mut (*(L as *mut GCUnion)).gc);
    }
    if (*g).l_registry.tt_ & BIT_ISCOLLECTABLE != 0
        && (*(*g).l_registry.value_.gc).marked as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        markvalue!(g, & g -> l_registry)(g, (*g).l_registry.value_.gc);
    }
    markmt(g);
    remarkupvals(g);
    propagateall(g);
    work = (*g).GCmemtrav as l_mem;
    (*g).gray = grayagain;
    propagateall(g);
    (*g).GCmemtrav = 0 as libc::c_int as lu_mem;
    convergeephemerons(g);
    clearvalues(g, (*g).weak, NULL as *mut GCObject);
    clearvalues(g, (*g).allweak, NULL as *mut GCObject);
    origweak = (*g).weak;
    origall = (*g).allweak;
    work = (work as libc::c_ulong).wrapping_add((*g).GCmemtrav) as l_mem as l_mem;
    separatetobefnz(g, 0 as libc::c_int);
    (*g).gcfinnum = 1 as libc::c_int as libc::c_uint;
    markbeingfnz(g);
    propagateall(g);
    (*g).GCmemtrav = 0 as libc::c_int as lu_mem;
    convergeephemerons(g);
    clearkeys(g, (*g).ephemeron, NULL as *mut GCObject);
    clearkeys(g, (*g).allweak, NULL as *mut GCObject);
    clearvalues(g, (*g).weak, origweak);
    clearvalues(g, (*g).allweak, origall);
    luaS_clearcache(g);
    (*g)
        .currentwhite = ((*g).currentwhite as libc::c_int
        ^ ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int)) as lu_byte;
    work = (work as libc::c_ulong).wrapping_add((*g).GCmemtrav) as l_mem as l_mem;
    return work;
}
unsafe extern "C" fn sweepstep(
    mut L: *mut lua_State,
    mut g: *mut global_State,
    mut nextstate: libc::c_int,
    mut nextlist: *mut *mut GCObject,
) -> lu_mem {
    if !((*g).sweepgc).is_null() {
        let mut olddebt = (*g).GCdebt;
        (*g).sweepgc = sweeplist(L, (*g).sweepgc, GCSWEEPMAX);
        (*g)
            .GCestimate = ((*g).GCestimate as libc::c_ulong)
            .wrapping_add(((*g).GCdebt - olddebt) as libc::c_ulong) as lu_mem as lu_mem;
        if !((*g).sweepgc).is_null() {
            return GCSWEEPMAX.wrapping_mul(GCSWEEPCOST);
        }
    }
    (*g).gcstate = nextstate as lu_byte;
    (*g).sweepgc = nextlist;
    return 0 as libc::c_int as lu_mem;
}
unsafe extern "C" fn singlestep(mut L: *mut lua_State) -> lu_mem {
    let mut g = G!(L);
    match (*g).gcstate as libc::c_int {
        GCSpause => {
            (*g)
                .GCmemtrav = ((*g).strt.size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut GCObject>() as libc::c_ulong);
            restartcollection(g);
            (*g).gcstate = GCSpropagate as lu_byte;
            return (*g).GCmemtrav;
        }
        GCSpropagate => {
            (*g).GCmemtrav = 0 as libc::c_int as lu_mem;
            propagatemark(g);
            if ((*g).gray).is_null() {
                (*g).gcstate = GCSatomic as lu_byte;
            }
            return (*g).GCmemtrav;
        }
        GCSatomic => {
            let mut work: lu_mem = 0;
            propagateall(g);
            work = atomic(L) as lu_mem;
            entersweep(L);
            (*g).GCestimate = gettotalbytes!(g);
            return work;
        }
        GCSswpallgc => return sweepstep(L, g, GCSswpfinobj, &mut (*g).finobj),
        GCSswpfinobj => return sweepstep(L, g, GCSswptobefnz, &mut (*g).tobefnz),
        GCSswptobefnz => return sweepstep(L, g, GCSswpend, NULL as *mut *mut GCObject),
        GCSswpend => {
            (*(*g).mainthread)
                .marked = ((*(*g).mainthread).marked as libc::c_int
                & !((1 as libc::c_int) << 2 as libc::c_int
                    | ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int))
                | ((*g).currentwhite as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int)) as lu_byte
                    as libc::c_int) as lu_byte;
            checkSizes(L, g);
            (*g).gcstate = GCScallfin as lu_byte;
            return 0 as libc::c_int as lu_mem;
        }
        GCScallfin => {
            if !((*g).tobefnz).is_null() && (*g).gckind as libc::c_int != KGC_EMERGENCY {
                let mut n = runafewfinalizers(L);
                return (n as libc::c_ulong).wrapping_mul(GCFINALIZECOST);
            } else {
                (*g).gcstate = GCSpause as lu_byte;
                return 0 as libc::c_int as lu_mem;
            }
        }
        _ => return 0 as libc::c_int as lu_mem,
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_runtilstate(
    mut L: *mut lua_State,
    mut statesmask: libc::c_int,
) {
    let mut g = G!(L);
    while testbit!(statesmask, g -> gcstate) == 0 {
        singlestep(L);
    }
}
unsafe extern "C" fn getdebt(mut g: *mut global_State) -> l_mem {
    let mut debt = (*g).GCdebt;
    let mut stepmul = (*g).gcstepmul;
    if debt <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as l_mem
    } else {
        debt = debt / STEPMULADJ as libc::c_long + 1 as libc::c_int as libc::c_long;
        debt = if debt < MAX_LMEM as l_mem / stepmul as libc::c_long {
            debt * stepmul as libc::c_long
        } else {
            MAX_LMEM as l_mem
        };
        return debt;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_step(mut L: *mut lua_State) {
    let mut g = G!(L);
    let mut debt = getdebt(g);
    if (*g).gcrunning == 0 {
        luaE_setdebt(g, (-(GCSTEPSIZE as libc::c_int) * 10 as libc::c_int) as l_mem);
        return;
    }
    loop {
        let mut work = singlestep(L);
        debt = (debt as libc::c_ulong).wrapping_sub(work) as l_mem as l_mem;
        if !(debt > -(GCSTEPSIZE as libc::c_int) as libc::c_long
            && (*g).gcstate as libc::c_int != GCSpause)
        {
            break;
        }
    }
    if (*g).gcstate as libc::c_int == GCSpause {
        setpause(g);
    } else {
        debt = debt / (*g).gcstepmul as libc::c_long * STEPMULADJ as libc::c_long;
        luaE_setdebt(g, debt);
        runafewfinalizers(L);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_fullgc(
    mut L: *mut lua_State,
    mut isemergency: libc::c_int,
) {
    let mut g = G!(L);
    if isemergency != 0 {
        (*g).gckind = KGC_EMERGENCY as lu_byte;
    }
    if keepinvariant!(g) != 0 {
        entersweep(L);
    }
    luaC_runtilstate(L, bitmask!(GCSpause));
    luaC_runtilstate(L, !bitmask!(GCSpause));
    luaC_runtilstate(L, bitmask!(GCScallfin));
    luaC_runtilstate(L, bitmask!(GCSpause));
    (*g).gckind = KGC_NORMAL as lu_byte;
    setpause(g);
}
static mut luaX_tokens: [*const libc::c_char; 37] = [
    b"and\0" as *const u8 as *const libc::c_char,
    b"break\0" as *const u8 as *const libc::c_char,
    b"do\0" as *const u8 as *const libc::c_char,
    b"else\0" as *const u8 as *const libc::c_char,
    b"elseif\0" as *const u8 as *const libc::c_char,
    b"end\0" as *const u8 as *const libc::c_char,
    b"false\0" as *const u8 as *const libc::c_char,
    b"for\0" as *const u8 as *const libc::c_char,
    b"function\0" as *const u8 as *const libc::c_char,
    b"goto\0" as *const u8 as *const libc::c_char,
    b"if\0" as *const u8 as *const libc::c_char,
    b"in\0" as *const u8 as *const libc::c_char,
    b"local\0" as *const u8 as *const libc::c_char,
    b"nil\0" as *const u8 as *const libc::c_char,
    b"not\0" as *const u8 as *const libc::c_char,
    b"or\0" as *const u8 as *const libc::c_char,
    b"repeat\0" as *const u8 as *const libc::c_char,
    b"return\0" as *const u8 as *const libc::c_char,
    b"then\0" as *const u8 as *const libc::c_char,
    b"true\0" as *const u8 as *const libc::c_char,
    b"until\0" as *const u8 as *const libc::c_char,
    b"while\0" as *const u8 as *const libc::c_char,
    b"//\0" as *const u8 as *const libc::c_char,
    b"..\0" as *const u8 as *const libc::c_char,
    b"...\0" as *const u8 as *const libc::c_char,
    b"==\0" as *const u8 as *const libc::c_char,
    b">=\0" as *const u8 as *const libc::c_char,
    b"<=\0" as *const u8 as *const libc::c_char,
    b"~=\0" as *const u8 as *const libc::c_char,
    b"<<\0" as *const u8 as *const libc::c_char,
    b">>\0" as *const u8 as *const libc::c_char,
    b"::\0" as *const u8 as *const libc::c_char,
    b"<eof>\0" as *const u8 as *const libc::c_char,
    b"<number>\0" as *const u8 as *const libc::c_char,
    b"<integer>\0" as *const u8 as *const libc::c_char,
    b"<name>\0" as *const u8 as *const libc::c_char,
    b"<string>\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn save(mut ls: *mut LexState, mut c: libc::c_int) {
    let mut b = (*ls).buff;
    if luaZ_bufflen!(b).wrapping_add(1 as libc::c_int as libc::c_ulong)
        > luaZ_sizebuffer!(b)
    {
        let mut newsize: size_t = 0;
        if luaZ_sizebuffer!(b)
            >= (if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
            {
                MAX_SIZET
            } else {
                9223372036854775807 as libc::c_longlong as size_t
            })
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            lexerror(
                ls,
                b"lexical element too long\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        newsize = luaZ_sizebuffer!(b).wrapping_mul(2 as libc::c_int as libc::c_ulong);
        let ref mut fresh43 = luaZ_resizebuffer!(ls -> L, b, newsize);
        *fresh43 = luaM_realloc_(
            (*ls).L,
            (*b).buffer as *mut libc::c_void,
            ((*b).buffsize)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            newsize.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        let ref mut fresh44 = luaZ_resizebuffer!(ls -> L, b, newsize);
        *fresh44 = newsize;
    }
    let ref mut fresh45 = luaZ_bufflen!(b);
    let fresh46 = *fresh45;
    *fresh45 = (*fresh45).wrapping_add(1);
    *((*b).buffer).offset(fresh46 as isize) = cast!(char, c);
}
#[no_mangle]
pub unsafe extern "C" fn luaX_init(mut L: *mut lua_State) {
    let mut i: libc::c_int = 0;
    let mut e = luaS_newliteral!(L, LUA_ENV);
    luaC_fix(L, obj2gco!(e));
    i = 0 as libc::c_int;
    while i < NUM_RESERVED {
        let mut ts = luaS_new(L, luaX_tokens[i as usize]);
        luaC_fix(L, obj2gco!(ts));
        (*ts).extra = cast_byte!(i + 1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaX_token2str(
    mut ls: *mut LexState,
    mut token: libc::c_int,
) -> *const libc::c_char {
    if token < FIRST_RESERVED {
        return luaO_pushfstring(
            (*ls).L,
            b"'%c'\0" as *const u8 as *const libc::c_char,
            token,
        )
    } else {
        let mut s = luaX_tokens[(token - FIRST_RESERVED) as usize];
        if token < TK_EOS as libc::c_int {
            return luaO_pushfstring(
                (*ls).L,
                b"'%s'\0" as *const u8 as *const libc::c_char,
                s,
            )
        } else {
            return s
        }
    };
}
unsafe extern "C" fn txtToken(
    mut ls: *mut LexState,
    mut token: libc::c_int,
) -> *const libc::c_char {
    match token {
        292 | 293 | 290 | 291 => {
            save(ls, '\0' as i32);
            return luaO_pushfstring(
                (*ls).L,
                b"'%s'\0" as *const u8 as *const libc::c_char,
                luaZ_buffer!(ls -> buff),
            );
        }
        _ => return luaX_token2str(ls, token),
    };
}
unsafe extern "C" fn lexerror(
    mut ls: *mut LexState,
    mut msg: *const libc::c_char,
    mut token: libc::c_int,
) -> ! {
    msg = luaG_addinfo((*ls).L, msg, (*ls).source, (*ls).linenumber);
    if token != 0 {
        luaO_pushfstring(
            (*ls).L,
            b"%s near %s\0" as *const u8 as *const libc::c_char,
            msg,
            txtToken(ls, token),
        );
    }
    luaD_throw((*ls).L, LUA_ERRSYNTAX);
}
#[no_mangle]
pub unsafe extern "C" fn luaX_syntaxerror(
    mut ls: *mut LexState,
    mut msg: *const libc::c_char,
) -> ! {
    lexerror(ls, msg, (*ls).t.token);
}
#[no_mangle]
pub unsafe extern "C" fn luaX_newstring(
    mut ls: *mut LexState,
    mut str: *const libc::c_char,
    mut l: size_t,
) -> *mut TString {
    let mut L = (*ls).L;
    let mut o = 0 as *mut TValue;
    let mut ts = luaS_newlstr(L, str, l);
    let mut io = setsvalue2s!(L, L -> top ++, ts);
    let mut x_ = setsvalue2s!(L, L -> top ++, ts);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    o = luaH_set(
        L,
        (*ls).h,
        ((*L).top).offset(-(1 as libc::c_int as isize)) as *const TValue,
    );
    if ttisnil!(o) != 0 {
        let mut io_0 = setbvalue!(o, 1);
        (*io_0).value_.b = setbvalue!(o, 1);
        (*io_0).tt_ = 1 as libc::c_int;
        if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
            luaC_checkGC!(L)(L);
        }
    } else {
        ts = &mut (*((*(&mut (*((o as *mut libc::c_char)
            .offset(-(0 as libc::c_ulong as isize)) as *mut Node))
            .i_key
            .tvk as *mut TValue as *const TValue))
            .value_
            .gc as *mut GCUnion))
            .ts;
    }
    (*L).top = ((*L).top).offset(-1);
    return ts;
}
unsafe extern "C" fn inclinenumber(mut ls: *mut LexState) {
    let mut old = (*ls).current;
    let fresh47 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls)
        .current = if fresh47 > 0 as libc::c_int as libc::c_ulong {
        let fresh48 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh48 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    if currIsNewline!(ls) != 0 && (*ls).current != old {
        let fresh49 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls)
            .current = if fresh49 > 0 as libc::c_int as libc::c_ulong {
            let fresh50 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh50 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        };
    }
    (*ls).linenumber += 1;
    if (*ls).linenumber >= MAX_INT {
        lexerror(
            ls,
            b"chunk has too many lines\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaX_setinput(
    mut L: *mut lua_State,
    mut ls: *mut LexState,
    mut z: *mut ZIO,
    mut source: *mut TString,
    mut firstchar: libc::c_int,
) {
    (*ls).t.token = 0 as libc::c_int;
    (*ls).L = L;
    (*ls).current = firstchar;
    (*ls).lookahead.token = TK_EOS as libc::c_int;
    (*ls).z = z;
    (*ls).fs = NULL as *mut FuncState;
    (*ls).linenumber = 1 as libc::c_int;
    (*ls).lastline = 1 as libc::c_int;
    (*ls).source = source;
    (*ls).envn = luaS_newliteral!(L, LUA_ENV);
    let ref mut fresh51 = luaZ_resizebuffer!(ls -> L, ls -> buff, LUA_MINBUFFER);
    *fresh51 = luaM_realloc_(
        (*ls).L,
        (*(*ls).buff).buffer as *mut libc::c_void,
        ((*(*ls).buff).buffsize)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let ref mut fresh52 = luaZ_resizebuffer!(ls -> L, ls -> buff, LUA_MINBUFFER);
    *fresh52 = 32 as libc::c_int as size_t;
}
unsafe extern "C" fn check_next1(
    mut ls: *mut LexState,
    mut c: libc::c_int,
) -> libc::c_int {
    if (*ls).current == c {
        let fresh53 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls)
            .current = if fresh53 > 0 as libc::c_int as libc::c_ulong {
            let fresh54 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh54 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        };
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn check_next2(
    mut ls: *mut LexState,
    mut set: *const libc::c_char,
) -> libc::c_int {
    if (*ls).current == *set.offset(0 as libc::c_int as isize) as libc::c_int
        || (*ls).current == *set.offset(1 as libc::c_int as isize) as libc::c_int
    {
        let fresh55 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls)
            .current = (if fresh55 > 0 as libc::c_int as libc::c_ulong {
            let fresh56 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh56 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        });
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn read_numeral(
    mut ls: *mut LexState,
    mut seminfo: *mut SemInfo,
) -> libc::c_int {
    let mut obj = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut expo = b"Ee\0" as *const u8 as *const libc::c_char;
    let mut first = (*ls).current;
    let fresh57 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls)
        .current = (if fresh57 > 0 as libc::c_int as libc::c_ulong {
        let fresh58 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh58 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    });
    if first == '0' as i32
        && check_next2(ls, b"xX\0" as *const u8 as *const libc::c_char) != 0
    {
        expo = b"Pp\0" as *const u8 as *const libc::c_char;
    }
    loop {
        if check_next2(ls, expo) != 0 {
            check_next2(ls, b"-+\0" as *const u8 as *const libc::c_char);
        }
        if lisxdigit!(ls -> current) != 0 {
            let fresh59 = (*(*ls).z).n;
            (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
            (*ls)
                .current = (if fresh59 > 0 as libc::c_int as libc::c_ulong {
                let fresh60 = (*(*ls).z).p;
                (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                *fresh60 as libc::c_uchar as libc::c_int
            } else {
                luaZ_fill((*ls).z)
            });
        } else {
            if !((*ls).current == '.' as i32) {
                break;
            }
            let fresh61 = (*(*ls).z).n;
            (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
            (*ls)
                .current = (if fresh61 > 0 as libc::c_int as libc::c_ulong {
                let fresh62 = (*(*ls).z).p;
                (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                *fresh62 as libc::c_uchar as libc::c_int
            } else {
                luaZ_fill((*ls).z)
            });
        }
    }
    save(ls, '\0' as i32);
    if luaO_str2num(luaZ_buffer!(ls -> buff), &mut obj)
        == 0 as libc::c_int as libc::c_ulong
    {
        lexerror(
            ls,
            b"malformed number\0" as *const u8 as *const libc::c_char,
            TK_FLT as libc::c_int,
        );
    }
    if ttisinteger!(& obj) != 0 {
        (*seminfo).i = ivalue!(& obj);
        return TK_INT as libc::c_int;
    } else {
        (*seminfo).r = fltvalue!(& obj);
        return TK_FLT as libc::c_int;
    };
}
unsafe extern "C" fn llex(
    mut ls: *mut LexState,
    mut seminfo: *mut SemInfo,
) -> libc::c_int {
    let ref mut fresh63 = luaZ_resetbuffer!(ls -> buff);
    *fresh63 = luaZ_resetbuffer!(ls -> buff);
    loop {
        let mut current_block_85: u64;
        match (*ls).current {
            10 | 13 => {
                inclinenumber(ls);
            }
            32 | 12 | 9 | 11 => {
                let fresh64 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = if fresh64 > 0 as libc::c_int as libc::c_ulong {
                    let fresh65 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh65 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
            }
            45 => {
                let fresh66 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = if fresh66 > 0 as libc::c_int as libc::c_ulong {
                    let fresh67 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh67 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if (*ls).current != '-' as i32 {
                    return '-' as i32;
                }
                let fresh68 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = if fresh68 > 0 as libc::c_int as libc::c_ulong {
                    let fresh69 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh69 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if (*ls).current == '[' as i32 {
                    let mut sep = skip_sep(ls);
                    let ref mut fresh70 = luaZ_resetbuffer!(ls -> buff);
                    *fresh70 = luaZ_resetbuffer!(ls -> buff);
                    if sep >= 2 as libc::c_int as libc::c_ulong {
                        read_long_string(ls, NULL as *mut SemInfo, sep);
                        let ref mut fresh71 = luaZ_resetbuffer!(ls -> buff);
                        *fresh71 = luaZ_resetbuffer!(ls -> buff);
                        current_block_85 = 10512632378975961025;
                    } else {
                        current_block_85 = 3512920355445576850;
                    }
                } else {
                    current_block_85 = 3512920355445576850;
                }
                match current_block_85 {
                    10512632378975961025 => {}
                    _ => {
                        while currIsNewline!(ls) == 0 && (*ls).current != EOZ {
                            let fresh72 = (*(*ls).z).n;
                            (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                            (*ls)
                                .current = if fresh72 > 0 as libc::c_int as libc::c_ulong {
                                let fresh73 = (*(*ls).z).p;
                                (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                                *fresh73 as libc::c_uchar as libc::c_int
                            } else {
                                luaZ_fill((*ls).z)
                            };
                        }
                    }
                }
            }
            91 => {
                let mut sep_0 = skip_sep(ls);
                if sep_0 >= 2 as libc::c_int as libc::c_ulong {
                    read_long_string(ls, seminfo, sep_0);
                    return TK_STRING as libc::c_int;
                } else {
                    if sep_0 == 0 as libc::c_int as libc::c_ulong {
                        lexerror(
                            ls,
                            b"invalid long string delimiter\0" as *const u8
                                as *const libc::c_char,
                            TK_STRING as libc::c_int,
                        );
                    }
                }
                return '[' as i32;
            }
            61 => {
                let fresh74 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = if fresh74 > 0 as libc::c_int as libc::c_ulong {
                    let fresh75 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh75 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_EQ as libc::c_int
                } else {
                    return '=' as i32
                }
            }
            60 => {
                let fresh76 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = if fresh76 > 0 as libc::c_int as libc::c_ulong {
                    let fresh77 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh77 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_LE as libc::c_int
                } else if check_next1(ls, '<' as i32) != 0 {
                    return TK_SHL as libc::c_int
                } else {
                    return '<' as i32
                }
            }
            62 => {
                let fresh78 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = if fresh78 > 0 as libc::c_int as libc::c_ulong {
                    let fresh79 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh79 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_GE as libc::c_int
                } else if check_next1(ls, '>' as i32) != 0 {
                    return TK_SHR as libc::c_int
                } else {
                    return '>' as i32
                }
            }
            47 => {
                let fresh80 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = if fresh80 > 0 as libc::c_int as libc::c_ulong {
                    let fresh81 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh81 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '/' as i32) != 0 {
                    return TK_IDIV as libc::c_int
                } else {
                    return '/' as i32
                }
            }
            126 => {
                let fresh82 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = if fresh82 > 0 as libc::c_int as libc::c_ulong {
                    let fresh83 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh83 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_NE as libc::c_int
                } else {
                    return '~' as i32
                }
            }
            58 => {
                let fresh84 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = if fresh84 > 0 as libc::c_int as libc::c_ulong {
                    let fresh85 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh85 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, ':' as i32) != 0 {
                    return TK_DBCOLON as libc::c_int
                } else {
                    return ':' as i32
                }
            }
            34 | 39 => {
                read_string(ls, (*ls).current, seminfo);
                return TK_STRING as libc::c_int;
            }
            46 => {
                let fresh86 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = (if fresh86 > 0 as libc::c_int as libc::c_ulong {
                    let fresh87 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh87 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                });
                if check_next1(ls, '.' as i32) != 0 {
                    if check_next1(ls, '.' as i32) != 0 {
                        return TK_DOTS as libc::c_int
                    } else {
                        return TK_CONCAT as libc::c_int
                    }
                } else if lisdigit!(ls -> current) == 0 {
                    return '.' as i32
                } else {
                    return read_numeral(ls, seminfo)
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                return read_numeral(ls, seminfo);
            }
            EOZ => return TK_EOS as libc::c_int,
            _ => {
                if lislalpha!(ls -> current) != 0 {
                    let mut ts = 0 as *mut TString;
                    loop {
                        let fresh88 = (*(*ls).z).n;
                        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                        (*ls)
                            .current = (if fresh88 > 0 as libc::c_int as libc::c_ulong {
                            let fresh89 = (*(*ls).z).p;
                            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                            *fresh89 as libc::c_uchar as libc::c_int
                        } else {
                            luaZ_fill((*ls).z)
                        });
                        if !(lislalnum!(ls -> current) != 0) {
                            break;
                        }
                    }
                    ts = luaX_newstring(
                        ls,
                        luaZ_buffer!(ls -> buff),
                        luaZ_bufflen!(ls -> buff),
                    );
                    (*seminfo).ts = ts;
                    if isreserved!(ts) != 0 {
                        return (*ts).extra as libc::c_int - 1 as libc::c_int
                            + FIRST_RESERVED
                    } else {
                        return TK_NAME as libc::c_int
                    }
                } else {
                    let mut c = (*ls).current;
                    let fresh90 = (*(*ls).z).n;
                    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                    (*ls)
                        .current = if fresh90 > 0 as libc::c_int as libc::c_ulong {
                        let fresh91 = (*(*ls).z).p;
                        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                        *fresh91 as libc::c_uchar as libc::c_int
                    } else {
                        luaZ_fill((*ls).z)
                    };
                    return c;
                }
            }
        }
    };
}
unsafe extern "C" fn gethexa(mut ls: *mut LexState) -> libc::c_int {
    let fresh92 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls)
        .current = (if fresh92 > 0 as libc::c_int as libc::c_ulong {
        let fresh93 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh93 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    });
    esccheck(
        ls,
        lisxdigit!(ls -> current),
        b"hexadecimal digit expected\0" as *const u8 as *const libc::c_char,
    );
    return luaO_hexavalue((*ls).current);
}
unsafe extern "C" fn skip_sep(mut ls: *mut LexState) -> size_t {
    let mut count = 0 as libc::c_int as size_t;
    let mut s = (*ls).current;
    let fresh94 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls)
        .current = (if fresh94 > 0 as libc::c_int as libc::c_ulong {
        let fresh95 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh95 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    });
    while (*ls).current == '=' as i32 {
        let fresh96 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls)
            .current = (if fresh96 > 0 as libc::c_int as libc::c_ulong {
            let fresh97 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh97 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        });
        count = count.wrapping_add(1);
    }
    return if (*ls).current == s {
        count.wrapping_add(2 as libc::c_int as libc::c_ulong)
    } else {
        (if count == 0 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_ulong
    };
}
unsafe extern "C" fn read_long_string(
    mut ls: *mut LexState,
    mut seminfo: *mut SemInfo,
    mut sep: size_t,
) {
    let mut line = (*ls).linenumber;
    let fresh98 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls)
        .current = (if fresh98 > 0 as libc::c_int as libc::c_ulong {
        let fresh99 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh99 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    });
    if currIsNewline!(ls) != 0 {
        inclinenumber(ls);
    }
    loop {
        match (*ls).current {
            EOZ => {
                let mut what = if !seminfo.is_null() {
                    b"string\0" as *const u8 as *const libc::c_char
                } else {
                    b"comment\0" as *const u8 as *const libc::c_char
                };
                let mut msg = luaO_pushfstring(
                    (*ls).L,
                    b"unfinished long %s (starting at line %d)\0" as *const u8
                        as *const libc::c_char,
                    what,
                    line,
                );
                lexerror(ls, msg, TK_EOS as libc::c_int);
            }
            93 => {
                if !(skip_sep(ls) == sep) {
                    continue;
                }
                let fresh100 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = (if fresh100 > 0 as libc::c_int as libc::c_ulong {
                    let fresh101 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh101 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                });
                break;
            }
            10 | 13 => {
                save(ls, '\n' as i32);
                inclinenumber(ls);
                if seminfo.is_null() {
                    let ref mut fresh102 = luaZ_resetbuffer!(ls -> buff);
                    *fresh102 = luaZ_resetbuffer!(ls -> buff);
                }
            }
            _ => {
                if !seminfo.is_null() {
                    let fresh103 = (*(*ls).z).n;
                    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                    (*ls)
                        .current = (if fresh103 > 0 as libc::c_int as libc::c_ulong {
                        let fresh104 = (*(*ls).z).p;
                        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                        *fresh104 as libc::c_uchar as libc::c_int
                    } else {
                        luaZ_fill((*ls).z)
                    });
                } else {
                    let fresh105 = (*(*ls).z).n;
                    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                    (*ls)
                        .current = if fresh105 > 0 as libc::c_int as libc::c_ulong {
                        let fresh106 = (*(*ls).z).p;
                        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                        *fresh106 as libc::c_uchar as libc::c_int
                    } else {
                        luaZ_fill((*ls).z)
                    };
                }
            }
        }
    }
    if !seminfo.is_null() {
        (*seminfo)
            .ts = luaX_newstring(
            ls,
            luaZ_buffer!(ls -> buff).offset(sep as isize),
            luaZ_bufflen!(ls -> buff)
                .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(sep)),
        );
    }
}
unsafe extern "C" fn utf8esc(mut ls: *mut LexState) {
    let mut buff: [libc::c_char; 8] = [0; 8];
    let mut n = luaO_utf8esc(buff.as_mut_ptr(), readutf8esc(ls));
    while n > 0 as libc::c_int {
        save(ls, buff[(UTF8BUFFSZ - n) as usize] as libc::c_int);
        n -= 1;
    }
}
unsafe extern "C" fn readhexaesc(mut ls: *mut LexState) -> libc::c_int {
    let mut r = gethexa(ls);
    r = (r << 4 as libc::c_int) + gethexa(ls);
    let ref mut fresh107 = luaZ_buffremove!(ls -> buff, 2);
    *fresh107 = (*fresh107 as libc::c_ulong)
        .wrapping_sub(luaZ_buffremove!(ls -> buff, 2)) as size_t as size_t;
    return r;
}
unsafe extern "C" fn esccheck(
    mut ls: *mut LexState,
    mut c: libc::c_int,
    mut msg: *const libc::c_char,
) {
    if c == 0 {
        if (*ls).current != EOZ {
            let fresh108 = (*(*ls).z).n;
            (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
            (*ls)
                .current = (if fresh108 > 0 as libc::c_int as libc::c_ulong {
                let fresh109 = (*(*ls).z).p;
                (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                *fresh109 as libc::c_uchar as libc::c_int
            } else {
                luaZ_fill((*ls).z)
            });
        }
        lexerror(ls, msg, TK_STRING as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaX_lookahead(mut ls: *mut LexState) -> libc::c_int {
    (*ls).lookahead.token = llex(ls, &mut (*ls).lookahead.seminfo);
    return (*ls).lookahead.token;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_next(mut ls: *mut LexState) {
    (*ls).lastline = (*ls).linenumber;
    if (*ls).lookahead.token != TK_EOS as libc::c_int {
        (*ls).t = (*ls).lookahead;
        (*ls).lookahead.token = TK_EOS as libc::c_int;
    } else {
        (*ls).t.token = llex(ls, &mut (*ls).t.seminfo);
    };
}
unsafe extern "C" fn readutf8esc(mut ls: *mut LexState) -> libc::c_ulong {
    let mut r: libc::c_ulong = 0;
    let mut i = 4 as libc::c_int;
    let fresh110 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls)
        .current = (if fresh110 > 0 as libc::c_int as libc::c_ulong {
        let fresh111 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh111 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    });
    esccheck(
        ls,
        ((*ls).current == '{' as i32) as libc::c_int,
        b"missing '{'\0" as *const u8 as *const libc::c_char,
    );
    r = gethexa(ls) as libc::c_ulong;
    while lisxdigit!(ls -> current) != 0 {
        i += 1;
        r = (r << 4 as libc::c_int)
            .wrapping_add(luaO_hexavalue((*ls).current) as libc::c_ulong);
        esccheck(
            ls,
            (r <= 0x10ffff as libc::c_int as libc::c_ulong) as libc::c_int,
            b"UTF-8 value too large\0" as *const u8 as *const libc::c_char,
        );
    }
    esccheck(
        ls,
        ((*ls).current == '}' as i32) as libc::c_int,
        b"missing '}'\0" as *const u8 as *const libc::c_char,
    );
    let fresh112 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls)
        .current = if fresh112 > 0 as libc::c_int as libc::c_ulong {
        let fresh113 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh113 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    let ref mut fresh114 = luaZ_buffremove!(ls -> buff, i);
    *fresh114 = (*fresh114 as libc::c_ulong)
        .wrapping_sub(luaZ_buffremove!(ls -> buff, i)) as size_t as size_t;
    return r;
}
unsafe extern "C" fn read_string(
    mut ls: *mut LexState,
    mut del: libc::c_int,
    mut seminfo: *mut SemInfo,
) {
    let mut current_block: u64;
    let fresh115 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls)
        .current = (if fresh115 > 0 as libc::c_int as libc::c_ulong {
        let fresh116 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh116 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    });
    while (*ls).current != del {
        match (*ls).current {
            EOZ => {
                lexerror(
                    ls,
                    b"unfinished string\0" as *const u8 as *const libc::c_char,
                    TK_EOS as libc::c_int,
                );
            }
            10 | 13 => {
                lexerror(
                    ls,
                    b"unfinished string\0" as *const u8 as *const libc::c_char,
                    TK_STRING as libc::c_int,
                );
            }
            92 => {
                let mut c: libc::c_int = 0;
                let fresh117 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = (if fresh117 > 0 as libc::c_int as libc::c_ulong {
                    let fresh118 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh118 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                });
                match (*ls).current {
                    97 => {
                        c = '\u{7}' as i32;
                        current_block = 4379362052238274123;
                    }
                    98 => {
                        c = '\u{8}' as i32;
                        current_block = 4379362052238274123;
                    }
                    102 => {
                        c = '\u{c}' as i32;
                        current_block = 4379362052238274123;
                    }
                    110 => {
                        c = '\n' as i32;
                        current_block = 4379362052238274123;
                    }
                    114 => {
                        c = '\r' as i32;
                        current_block = 4379362052238274123;
                    }
                    116 => {
                        c = '\t' as i32;
                        current_block = 4379362052238274123;
                    }
                    118 => {
                        c = '\u{b}' as i32;
                        current_block = 4379362052238274123;
                    }
                    120 => {
                        c = readhexaesc(ls);
                        current_block = 4379362052238274123;
                    }
                    117 => {
                        utf8esc(ls);
                        continue;
                    }
                    10 | 13 => {
                        inclinenumber(ls);
                        c = '\n' as i32;
                        current_block = 8352143818600766666;
                    }
                    92 | 34 | 39 => {
                        c = (*ls).current;
                        current_block = 4379362052238274123;
                    }
                    EOZ => {
                        continue;
                    }
                    122 => {
                        let ref mut fresh119 = luaZ_buffremove!(ls -> buff, 1);
                        *fresh119 = (*fresh119 as libc::c_ulong)
                            .wrapping_sub(luaZ_buffremove!(ls -> buff, 1)) as size_t
                            as size_t;
                        let fresh120 = (*(*ls).z).n;
                        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                        (*ls)
                            .current = if fresh120 > 0 as libc::c_int as libc::c_ulong {
                            let fresh121 = (*(*ls).z).p;
                            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                            *fresh121 as libc::c_uchar as libc::c_int
                        } else {
                            luaZ_fill((*ls).z)
                        };
                        while lisspace!(ls -> current) != 0 {
                            if currIsNewline!(ls) != 0 {
                                inclinenumber(ls);
                            } else {
                                let fresh122 = (*(*ls).z).n;
                                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                                (*ls)
                                    .current = if fresh122 > 0 as libc::c_int as libc::c_ulong {
                                    let fresh123 = (*(*ls).z).p;
                                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                                    *fresh123 as libc::c_uchar as libc::c_int
                                } else {
                                    luaZ_fill((*ls).z)
                                };
                            }
                        }
                        continue;
                    }
                    _ => {
                        esccheck(
                            ls,
                            lisdigit!(ls -> current),
                            b"invalid escape sequence\0" as *const u8
                                as *const libc::c_char,
                        );
                        c = readdecesc(ls);
                        current_block = 8352143818600766666;
                    }
                }
                match current_block {
                    4379362052238274123 => {
                        let fresh124 = (*(*ls).z).n;
                        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                        (*ls)
                            .current = if fresh124 > 0 as libc::c_int as libc::c_ulong {
                            let fresh125 = (*(*ls).z).p;
                            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                            *fresh125 as libc::c_uchar as libc::c_int
                        } else {
                            luaZ_fill((*ls).z)
                        };
                    }
                    _ => {}
                }
                let ref mut fresh126 = luaZ_buffremove!(ls -> buff, 1);
                *fresh126 = (*fresh126 as libc::c_ulong)
                    .wrapping_sub(luaZ_buffremove!(ls -> buff, 1)) as size_t as size_t;
                save(ls, c);
            }
            _ => {
                let fresh127 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls)
                    .current = (if fresh127 > 0 as libc::c_int as libc::c_ulong {
                    let fresh128 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh128 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                });
            }
        }
    }
    let fresh129 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls)
        .current = (if fresh129 > 0 as libc::c_int as libc::c_ulong {
        let fresh130 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh130 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    });
    (*seminfo)
        .ts = luaX_newstring(
        ls,
        luaZ_buffer!(ls -> buff).offset(1 as libc::c_int as isize),
        luaZ_bufflen!(ls -> buff).wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn readdecesc(mut ls: *mut LexState) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int && lisdigit!(ls -> current) != 0 {
        r = 10 as libc::c_int * r + (*ls).current - '0' as i32;
        let fresh131 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls)
            .current = (if fresh131 > 0 as libc::c_int as libc::c_ulong {
            let fresh132 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh132 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        });
        i += 1;
    }
    esccheck(
        ls,
        (r <= UCHAR_MAX) as libc::c_int,
        b"decimal escape too large\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh133 = luaZ_buffremove!(ls -> buff, i);
    *fresh133 = (*fresh133 as libc::c_ulong)
        .wrapping_sub(luaZ_buffremove!(ls -> buff, i)) as size_t as size_t;
    return r;
}
pub const NO_JUMP: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub unsafe extern "C" fn luaK_setreturns(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut nresults: libc::c_int,
) {
    if (*e).k as libc::c_uint == VCALL as libc::c_int as libc::c_uint {
        let ref mut fresh134 = SETARG_C!(getinstruction(fs, e), nresults + 1);
        *fresh134 = SETARG_C!(getinstruction(fs, e), nresults + 1);
    } else if (*e).k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint {
        let mut pc: *mut Instruction = &mut getinstruction!(fs, e) as *mut Instruction;
        let ref mut fresh135 = SETARG_B!(* pc, nresults + 1);
        *fresh135 = SETARG_B!(* pc, nresults + 1);
        let ref mut fresh136 = SETARG_A!(* pc, fs -> freereg);
        *fresh136 = SETARG_A!(* pc, fs -> freereg);
        luaK_reserveregs(fs, 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaK_posfix(
    mut fs: *mut FuncState,
    mut op: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut line: libc::c_int,
) {
    match op as libc::c_uint {
        19 => {
            luaK_dischargevars(fs, e2);
            luaK_concat(fs, &mut (*e2).f, (*e1).f);
            *e1 = *e2;
        }
        20 => {
            luaK_dischargevars(fs, e2);
            luaK_concat(fs, &mut (*e2).t, (*e1).t);
            *e1 = *e2;
        }
        12 => {
            luaK_exp2val(fs, e2);
            if (*e2).k as libc::c_uint == VRELOCABLE as libc::c_int as libc::c_uint
                && (*((*(*fs).f).code).offset((*e2).u.info as isize) >> 0 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 6 as libc::c_int)
                        << 0 as libc::c_int) as OpCode as libc::c_uint
                    == OP_CONCAT as libc::c_int as libc::c_uint
            {
                freeexp(fs, e1);
                let ref mut fresh137 = SETARG_B!(getinstruction(fs, e2), e1 -> u.info);
                *fresh137 = SETARG_B!(getinstruction(fs, e2), e1 -> u.info);
                (*e1).k = VRELOCABLE;
                (*e1).u.info = (*e2).u.info;
            } else {
                luaK_exp2nextreg(fs, e2);
                codebinexpval(fs, OP_CONCAT, e1, e2, line);
            }
        }
        0 | 1 | 2 | 5 | 6 | 3 | 4 | 7 | 8 | 9 | 10 | 11 => {
            if constfolding(
                fs,
                (op as libc::c_uint).wrapping_add(LUA_OPADD as libc::c_uint)
                    as libc::c_int,
                e1,
                e2,
            ) == 0
            {
                codebinexpval(fs, cast!(OpCode, op + OP_ADD), e1, e2, line);
            }
        }
        13 | 14 | 15 | 16 | 17 | 18 => {
            codecomp(fs, op, e1, e2);
        }
        _ => {}
    };
}
unsafe extern "C" fn codecomp(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
) {
    let mut rk1 = if (*e1).k as libc::c_uint == VK as libc::c_int as libc::c_uint {
        RKASK!(e1 -> u.info)
    } else {
        check_exp!(e1 -> k == VNONRELOC, e1 -> u.info)
    };
    let mut rk2 = luaK_exp2RK(fs, e2);
    freeexps(fs, e1, e2);
    match opr as libc::c_uint {
        16 => {
            (*e1).u.info = condjump(fs, OP_EQ, 0 as libc::c_int, rk1, rk2);
        }
        17 | 18 => {
            let mut op = cast!(OpCode, (opr - OPR_NE) + OP_EQ);
            (*e1).u.info = condjump(fs, op, 1 as libc::c_int, rk2, rk1);
        }
        _ => {
            let mut op_0 = cast!(OpCode, (opr - OPR_EQ) + OP_EQ);
            (*e1).u.info = condjump(fs, op_0, 1 as libc::c_int, rk1, rk2);
        }
    }
    (*e1).k = VJMP;
}
unsafe extern "C" fn condjump(
    mut fs: *mut FuncState,
    mut op: OpCode,
    mut A: libc::c_int,
    mut B: libc::c_int,
    mut C: libc::c_int,
) -> libc::c_int {
    luaK_codeABC(fs, op, A, B, C);
    return luaK_jump(fs);
}
unsafe extern "C" fn freeexps(
    mut fs: *mut FuncState,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
) {
    let mut r1 = if (*e1).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        (*e1).u.info
    } else {
        -(1 as libc::c_int)
    };
    let mut r2 = if (*e2).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        (*e2).u.info
    } else {
        -(1 as libc::c_int)
    };
    if r1 > r2 {
        freereg(fs, r1);
        freereg(fs, r2);
    } else {
        freereg(fs, r2);
        freereg(fs, r1);
    };
}
unsafe extern "C" fn codebinexpval(
    mut fs: *mut FuncState,
    mut op: OpCode,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut line: libc::c_int,
) {
    let mut rk2 = luaK_exp2RK(fs, e2);
    let mut rk1 = luaK_exp2RK(fs, e1);
    freeexps(fs, e1, e2);
    (*e1).u.info = luaK_codeABC(fs, op, 0 as libc::c_int, rk1, rk2);
    (*e1).k = VRELOCABLE;
    luaK_fixline(fs, line);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_fixline(mut fs: *mut FuncState, mut line: libc::c_int) {
    *((*(*fs).f).lineinfo).offset(((*fs).pc - 1 as libc::c_int) as isize) = line;
}
unsafe extern "C" fn constfolding(
    mut fs: *mut FuncState,
    mut op: libc::c_int,
    mut e1: *mut expdesc,
    mut e2: *const expdesc,
) -> libc::c_int {
    let mut v1 = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut v2 = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut res = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    if tonumeral(e1, &mut v1) == 0 || tonumeral(e2, &mut v2) == 0
        || validop(op, &mut v1, &mut v2) == 0
    {
        return 0 as libc::c_int;
    }
    luaO_arith((*(*fs).ls).L, op, &mut v1, &mut v2, &mut res);
    if ttisinteger!(& res) != 0 {
        (*e1).k = VKINT;
        (*e1).u.ival = ivalue!(& res);
    } else {
        let mut n = fltvalue!(& res);
        if !(n == n) || n == 0 as libc::c_int as libc::c_double {
            return 0 as libc::c_int;
        }
        (*e1).k = VKFLT;
        (*e1).u.nval = n;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn validop(
    mut op: libc::c_int,
    mut v1: *mut TValue,
    mut v2: *mut TValue,
) -> libc::c_int {
    match op {
        LUA_OPBAND | LUA_OPBOR | LUA_OPBXOR | LUA_OPSHL | LUA_OPSHR | LUA_OPBNOT => {
            let mut i: lua_Integer = 0;
            return (tointeger!(v1, & i) != 0 && tointeger!(v2, & i) != 0) as libc::c_int;
        }
        LUA_OPDIV | LUA_OPIDIV | LUA_OPMOD => {
            return (nvalue!(v2) != 0 as libc::c_int as libc::c_double) as libc::c_int;
        }
        _ => return 1 as libc::c_int,
    };
}
unsafe extern "C" fn tonumeral(
    mut e: *const expdesc,
    mut v: *mut TValue,
) -> libc::c_int {
    if hasjumps!(e) != 0 {
        return 0 as libc::c_int;
    }
    match (*e).k as libc::c_uint {
        6 => {
            if !v.is_null() {
                let mut io = setivalue!(v, e -> u.ival);
                (*io).value_.i = setivalue!(v, e -> u.ival);
                (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        5 => {
            if !v.is_null() {
                let mut io_0 = setfltvalue!(v, e -> u.nval);
                (*io_0).value_.n = setfltvalue!(v, e -> u.nval);
                (*io_0).tt_ = 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_infix(
    mut fs: *mut FuncState,
    mut op: BinOpr,
    mut v: *mut expdesc,
) {
    match op as libc::c_uint {
        19 => {
            luaK_goiftrue(fs, v);
        }
        20 => {
            luaK_goiffalse(fs, v);
        }
        12 => {
            luaK_exp2nextreg(fs, v);
        }
        0 | 1 | 2 | 5 | 6 | 3 | 4 | 7 | 8 | 9 | 10 | 11 => {
            if tonumeral(v, NULL as *mut TValue) == 0 {
                luaK_exp2RK(fs, v);
            }
        }
        _ => {
            luaK_exp2RK(fs, v);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_goiffalse(mut fs: *mut FuncState, mut e: *mut expdesc) {
    let mut pc: libc::c_int = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        11 => {
            pc = (*e).u.info;
        }
        1 | 3 => {
            pc = NO_JUMP;
        }
        _ => {
            pc = jumponcond(fs, e, 1 as libc::c_int);
        }
    }
    luaK_concat(fs, &mut (*e).t, pc);
    luaK_patchtohere(fs, (*e).f);
    (*e).f = NO_JUMP;
}
unsafe extern "C" fn jumponcond(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut cond_0: libc::c_int,
) -> libc::c_int {
    if (*e).k as libc::c_uint == VRELOCABLE as libc::c_int as libc::c_uint {
        let mut ie = getinstruction!(fs, e);
        if GET_OPCODE!(ie) == OP_NOT as libc::c_int as libc::c_uint {
            (*fs).pc -= 1;
            return condjump(
                fs,
                OP_TEST,
                GETARG_B!(ie),
                0 as libc::c_int,
                (cond_0 == 0) as libc::c_int,
            );
        }
    }
    discharge2anyreg(fs, e);
    freeexp(fs, e);
    return condjump(fs, OP_TESTSET, NO_REG, (*e).u.info, cond_0);
}
unsafe extern "C" fn discharge2anyreg(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as libc::c_uint != VNONRELOC as libc::c_int as libc::c_uint {
        luaK_reserveregs(fs, 1 as libc::c_int);
        discharge2reg(fs, e, (*fs).freereg as libc::c_int - 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaK_goiftrue(mut fs: *mut FuncState, mut e: *mut expdesc) {
    let mut pc: libc::c_int = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        11 => {
            negatecondition(fs, e);
            pc = (*e).u.info;
        }
        4 | 5 | 6 | 2 => {
            pc = NO_JUMP;
        }
        _ => {
            pc = jumponcond(fs, e, 0 as libc::c_int);
        }
    }
    luaK_concat(fs, &mut (*e).f, pc);
    luaK_patchtohere(fs, (*e).t);
    (*e).t = NO_JUMP;
}
unsafe extern "C" fn negatecondition(mut fs: *mut FuncState, mut e: *mut expdesc) {
    let mut pc = getjumpcontrol(fs, (*e).u.info);
    *pc = *pc
        & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
            << 0 as libc::c_int + 6 as libc::c_int)
        | (((*pc >> 0 as libc::c_int + 6 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int) as libc::c_int == 0) as libc::c_int as Instruction)
            << 0 as libc::c_int + 6 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int + 6 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_stringK(
    mut fs: *mut FuncState,
    mut s: *mut TString,
) -> libc::c_int {
    let mut o = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = setsvalue!(fs -> ls -> L, & o, s);
    let mut x_ = setsvalue!(fs -> ls -> L, & o, s);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    return addk(fs, &mut o, &mut o);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_setlist(
    mut fs: *mut FuncState,
    mut base: libc::c_int,
    mut nelems: libc::c_int,
    mut tostore: libc::c_int,
) {
    let mut c = (nelems - 1 as libc::c_int) / LFIELDS_PER_FLUSH + 1 as libc::c_int;
    let mut b = if tostore == LUA_MULTRET { 0 as libc::c_int } else { tostore };
    if c <= MAXARG_C {
        luaK_codeABC(fs, OP_SETLIST, base, b, c);
    } else if c <= MAXARG_Ax {
        luaK_codeABC(fs, OP_SETLIST, base, b, 0 as libc::c_int);
        codeextraarg(fs, c);
    } else {
        luaX_syntaxerror(
            (*fs).ls,
            b"constructor too long\0" as *const u8 as *const libc::c_char,
        );
    }
    (*fs).freereg = (base + 1 as libc::c_int) as lu_byte;
}
pub const LUA_FLOORN2I: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2val(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if hasjumps!(e) != 0 {
        luaK_exp2anyreg(fs, e);
    } else {
        luaK_dischargevars(fs, e);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_self(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut key: *mut expdesc,
) {
    let mut ereg: libc::c_int = 0;
    luaK_exp2anyreg(fs, e);
    ereg = (*e).u.info;
    freeexp(fs, e);
    (*e).u.info = (*fs).freereg as libc::c_int;
    (*e).k = VNONRELOC;
    luaK_reserveregs(fs, 2 as libc::c_int);
    luaK_codeABC(fs, OP_SELF, (*e).u.info, ereg, luaK_exp2RK(fs, key));
    freeexp(fs, key);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_indexed(
    mut fs: *mut FuncState,
    mut t: *mut expdesc,
    mut k: *mut expdesc,
) {
    (*t).u.ind.t = (*t).u.info as lu_byte;
    (*t).u.ind.idx = luaK_exp2RK(fs, k) as libc::c_short;
    (*t)
        .u
        .ind
        .vt = (if (*t).k as libc::c_uint == VUPVAL as libc::c_int as libc::c_uint {
        VUPVAL as libc::c_int
    } else {
        VLOCAL as libc::c_int
    }) as lu_byte;
    (*t).k = VINDEXED;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2anyregup(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as libc::c_uint != VUPVAL as libc::c_int as libc::c_uint
        || hasjumps!(e) != 0
    {
        luaK_exp2anyreg(fs, e);
    }
}
unsafe extern "C" fn boolK(mut fs: *mut FuncState, mut b: libc::c_int) -> libc::c_int {
    let mut o = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = setbvalue!(& o, b);
    (*io).value_.b = setbvalue!(& o, b);
    (*io).tt_ = 1 as libc::c_int;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn nilK(mut fs: *mut FuncState) -> libc::c_int {
    let mut k = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut v = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let ref mut fresh138 = setnilvalue!(& v);
    *fresh138 = setnilvalue!(& v);
    let mut io: *mut TValue = sethvalue!(fs -> ls -> L, & k, fs -> ls -> h);
    let mut x_ = sethvalue!(fs -> ls -> L, & k, fs -> ls -> h);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    return addk(fs, &mut k, &mut v);
}
pub const MAXREGS: libc::c_int = 255 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn luaK_checkstack(mut fs: *mut FuncState, mut n: libc::c_int) {
    let mut newstack = (*fs).freereg as libc::c_int + n;
    if newstack > (*(*fs).f).maxstacksize as libc::c_int {
        if newstack >= MAXREGS {
            luaX_syntaxerror(
                (*fs).ls,
                b"function or expression needs too many registers\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*(*fs).f).maxstacksize = cast_byte!(newstack);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaK_reserveregs(mut fs: *mut FuncState, mut n: libc::c_int) {
    luaK_checkstack(fs, n);
    (*fs).freereg = ((*fs).freereg as libc::c_int + n) as lu_byte;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_setoneret(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as libc::c_uint == VCALL as libc::c_int as libc::c_uint {
        (*e).k = VNONRELOC;
        (*e)
            .u
            .info = (*((*(*fs).f).code).offset((*e).u.info as isize)
            >> 0 as libc::c_int + 6 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int) as libc::c_int;
    } else if (*e).k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint {
        let ref mut fresh139 = SETARG_B!(getinstruction(fs, e), 2);
        *fresh139 = SETARG_B!(getinstruction(fs, e), 2);
        (*e).k = VRELOCABLE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaK_dischargevars(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
) {
    match (*e).k as libc::c_uint {
        8 => {
            (*e).k = VNONRELOC;
        }
        9 => {
            (*e)
                .u
                .info = luaK_codeABC(
                fs,
                OP_GETUPVAL,
                0 as libc::c_int,
                (*e).u.info,
                0 as libc::c_int,
            );
            (*e).k = VRELOCABLE;
        }
        10 => {
            let mut op = OP_MOVE;
            freereg(fs, (*e).u.ind.idx as libc::c_int);
            if (*e).u.ind.vt as libc::c_int == VLOCAL as libc::c_int {
                freereg(fs, (*e).u.ind.t as libc::c_int);
                op = OP_GETTABLE;
            } else {
                op = OP_GETTABUP;
            }
            (*e)
                .u
                .info = luaK_codeABC(
                fs,
                op,
                0 as libc::c_int,
                (*e).u.ind.t as libc::c_int,
                (*e).u.ind.idx as libc::c_int,
            );
            (*e).k = VRELOCABLE;
        }
        14 | 13 => {
            luaK_setoneret(fs, e);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_nil(
    mut fs: *mut FuncState,
    mut from: libc::c_int,
    mut n: libc::c_int,
) {
    let mut previous = 0 as *mut Instruction;
    let mut l = from + n - 1 as libc::c_int;
    if (*fs).pc > (*fs).lasttarget {
        previous = &mut *((*(*fs).f).code).offset(((*fs).pc - 1 as libc::c_int) as isize)
            as *mut Instruction;
        if GET_OPCODE!(* previous) == OP_LOADNIL as libc::c_int as libc::c_uint {
            let mut pfrom = GETARG_A!(* previous);
            let mut pl = pfrom + GETARG_B!(* previous);
            if pfrom <= from && from <= pl + 1 as libc::c_int
                || from <= pfrom && pfrom <= l + 1 as libc::c_int
            {
                if pfrom < from {
                    from = pfrom;
                }
                if pl > l {
                    l = pl;
                }
                let ref mut fresh140 = SETARG_A!(* previous, from);
                *fresh140 = SETARG_A!(* previous, from);
                let ref mut fresh141 = SETARG_B!(* previous, l - from);
                *fresh141 = SETARG_B!(* previous, l - from);
                return;
            }
        }
    }
    luaK_codeABC(fs, OP_LOADNIL, from, n - 1 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn luaK_numberK(
    mut fs: *mut FuncState,
    mut r: lua_Number,
) -> libc::c_int {
    let mut o = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = setfltvalue!(& o, r);
    (*io).value_.n = setfltvalue!(& o, r);
    (*io).tt_ = 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn codeextraarg(
    mut fs: *mut FuncState,
    mut a: libc::c_int,
) -> libc::c_int {
    return luaK_code(fs, CREATE_Ax!(OP_EXTRAARG, a));
}
#[no_mangle]
pub unsafe extern "C" fn luaK_codek(
    mut fs: *mut FuncState,
    mut reg: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    if k <= MAXARG_Bx {
        return luaK_codeABx(fs, OP_LOADK, reg, k as libc::c_uint)
    } else {
        let mut p = luaK_codeABx(fs, OP_LOADKX, reg, 0 as libc::c_int as libc::c_uint);
        codeextraarg(fs, k);
        return p;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_prefix(
    mut fs: *mut FuncState,
    mut op: UnOpr,
    mut e: *mut expdesc,
    mut line: libc::c_int,
) {
    static mut ef: expdesc = {
        let mut init = expdesc {
            k: VKINT,
            u: C2RustUnnamed_8 {
                ival: 0 as libc::c_int as lua_Integer,
            },
            t: NO_JUMP,
            f: NO_JUMP,
        };
        init
    };
    let mut current_block_2: u64;
    match op as libc::c_uint {
        0 | 1 => {
            if constfolding(
                fs,
                (op as libc::c_uint).wrapping_add(LUA_OPUNM as libc::c_uint)
                    as libc::c_int,
                e,
                &ef,
            ) != 0
            {
                current_block_2 = 10879442775620481940;
            } else {
                current_block_2 = 17400725798413161764;
            }
        }
        3 => {
            current_block_2 = 17400725798413161764;
        }
        2 => {
            codenot(fs, e);
            current_block_2 = 10879442775620481940;
        }
        _ => {
            current_block_2 = 10879442775620481940;
        }
    }
    match current_block_2 {
        17400725798413161764 => {
            codeunexpval(fs, cast!(OpCode, op + OP_UNM), e, line);
        }
        _ => {}
    };
}
unsafe extern "C" fn codenot(mut fs: *mut FuncState, mut e: *mut expdesc) {
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        1 | 3 => {
            (*e).k = VTRUE;
        }
        4 | 5 | 6 | 2 => {
            (*e).k = VFALSE;
        }
        11 => {
            negatecondition(fs, e);
        }
        12 | 7 => {
            discharge2anyreg(fs, e);
            freeexp(fs, e);
            (*e)
                .u
                .info = luaK_codeABC(
                fs,
                OP_NOT,
                0 as libc::c_int,
                (*e).u.info,
                0 as libc::c_int,
            );
            (*e).k = VRELOCABLE;
        }
        _ => {}
    }
    let mut temp = (*e).f;
    (*e).f = (*e).t;
    (*e).t = temp;
    removevalues(fs, (*e).f);
    removevalues(fs, (*e).t);
}
unsafe extern "C" fn removevalues(mut fs: *mut FuncState, mut list: libc::c_int) {
    while list != NO_JUMP {
        patchtestreg(fs, list, NO_REG);
        list = getjump(fs, list);
    }
}
unsafe extern "C" fn codeunexpval(
    mut fs: *mut FuncState,
    mut op: OpCode,
    mut e: *mut expdesc,
    mut line: libc::c_int,
) {
    let mut r = luaK_exp2anyreg(fs, e);
    freeexp(fs, e);
    (*e).u.info = luaK_codeABC(fs, op, 0 as libc::c_int, r, 0 as libc::c_int);
    (*e).k = VRELOCABLE;
    luaK_fixline(fs, line);
}
unsafe extern "C" fn addk(
    mut fs: *mut FuncState,
    mut key: *mut TValue,
    mut v: *mut TValue,
) -> libc::c_int {
    let mut L = (*(*fs).ls).L;
    let mut f = (*fs).f;
    let mut idx = luaH_set(L, (*(*fs).ls).h, key);
    let mut k: libc::c_int = 0;
    let mut oldsize: libc::c_int = 0;
    if ttisinteger!(idx) != 0 {
        k = (*idx).value_.i as libc::c_int;
        if k < (*fs).nk && ttype!(& f -> k[k]) == ttype!(v)
            && luaV_rawequalobj!(& f -> k[k], v) != 0
        {
            return k;
        }
    }
    oldsize = (*f).sizek;
    k = (*fs).nk;
    let mut io = setivalue!(idx, k);
    (*io).value_.i = setivalue!(idx, k);
    (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
    if luaM_growvector!(L, f -> k, k, f -> sizek, TValue, MAXARG_Ax, "constants") != 0 {
        let ref mut fresh142 = luaM_growvector!(
            L, f -> k, k, f -> sizek, TValue, MAXARG_Ax, "constants"
        );
        *fresh142 = luaM_growaux_(
            L,
            (*f).k as *mut libc::c_void,
            &mut (*f).sizek,
            ::core::mem::size_of::<TValue>() as libc::c_ulong,
            ((1 as libc::c_int)
                << 9 as libc::c_int + 9 as libc::c_int + 8 as libc::c_int)
                - 1 as libc::c_int,
            b"constants\0" as *const u8 as *const libc::c_char,
        ) as *mut TValue;
    }
    while oldsize < (*f).sizek {
        let ref mut fresh143 = setnilvalue!(& f -> k[oldsize ++]);
        *fresh143 = setnilvalue!(& f -> k[oldsize ++]);
    }
    let mut io1: *mut TValue = setobj!(L, & f -> k[k], v);
    let ref mut fresh144 = setobj!(L, & f -> k[k], v);
    *fresh144 = setobj!(L, & f -> k[k], v);
    (*fs).nk += 1;
    if luaC_barrier!(L, f, v) != 0 {} else {};
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_intK(
    mut fs: *mut FuncState,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut k = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut o = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = n as size_t as *mut libc::c_void;
    (*io).tt_ = 2 as libc::c_int;
    let mut io_0: *mut TValue = setivalue!(& o, n);
    (*io_0).value_.i = setivalue!(& o, n);
    (*io_0).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
    return addk(fs, &mut k, &mut o);
}
unsafe extern "C" fn discharge2reg(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut reg: libc::c_int,
) {
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        1 => {
            luaK_nil(fs, reg, 1 as libc::c_int);
        }
        3 | 2 => {
            luaK_codeABC(
                fs,
                OP_LOADBOOL,
                reg,
                ((*e).k as libc::c_uint == VTRUE as libc::c_int as libc::c_uint)
                    as libc::c_int,
                0 as libc::c_int,
            );
        }
        4 => {
            luaK_codek(fs, reg, (*e).u.info);
        }
        5 => {
            luaK_codek(fs, reg, luaK_numberK(fs, (*e).u.nval));
        }
        6 => {
            luaK_codek(fs, reg, luaK_intK(fs, (*e).u.ival));
        }
        12 => {
            let mut pc: *mut Instruction = &mut getinstruction!(fs, e)
                as *mut Instruction;
            let ref mut fresh145 = SETARG_A!(* pc, reg);
            *fresh145 = SETARG_A!(* pc, reg);
        }
        7 => {
            if reg != (*e).u.info {
                luaK_codeABC(fs, OP_MOVE, reg, (*e).u.info, 0 as libc::c_int);
            }
        }
        _ => return,
    }
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}
unsafe extern "C" fn need_value(
    mut fs: *mut FuncState,
    mut list: libc::c_int,
) -> libc::c_int {
    while list != NO_JUMP {
        let mut i = *getjumpcontrol(fs, list);
        if GET_OPCODE!(i) != OP_TESTSET as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        }
        list = getjump(fs, list);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn code_loadbool(
    mut fs: *mut FuncState,
    mut A: libc::c_int,
    mut b: libc::c_int,
    mut jump: libc::c_int,
) -> libc::c_int {
    luaK_getlabel(fs);
    return luaK_codeABC(fs, OP_LOADBOOL, A, b, jump);
}
unsafe extern "C" fn exp2reg(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut reg: libc::c_int,
) {
    discharge2reg(fs, e, reg);
    if (*e).k as libc::c_uint == VJMP as libc::c_int as libc::c_uint {
        luaK_concat(fs, &mut (*e).t, (*e).u.info);
    }
    if hasjumps!(e) != 0 {
        let mut final_0: libc::c_int = 0;
        let mut p_f = NO_JUMP;
        let mut p_t = NO_JUMP;
        if need_value(fs, (*e).t) != 0 || need_value(fs, (*e).f) != 0 {
            let mut fj = if (*e).k as libc::c_uint == VJMP as libc::c_int as libc::c_uint
            {
                NO_JUMP
            } else {
                luaK_jump(fs)
            };
            p_f = code_loadbool(fs, reg, 0 as libc::c_int, 1 as libc::c_int);
            p_t = code_loadbool(fs, reg, 1 as libc::c_int, 0 as libc::c_int);
            luaK_patchtohere(fs, fj);
        }
        final_0 = luaK_getlabel(fs);
        patchlistaux(fs, (*e).f, final_0, reg, p_f);
        patchlistaux(fs, (*e).t, final_0, reg, p_t);
    }
    (*e).t = NO_JUMP;
    (*e).f = (*e).t;
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2nextreg(mut fs: *mut FuncState, mut e: *mut expdesc) {
    luaK_dischargevars(fs, e);
    freeexp(fs, e);
    luaK_reserveregs(fs, 1 as libc::c_int);
    exp2reg(fs, e, (*fs).freereg as libc::c_int - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2anyreg(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
) -> libc::c_int {
    luaK_dischargevars(fs, e);
    if (*e).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        if hasjumps!(e) == 0 {
            return (*e).u.info;
        }
        if (*e).u.info >= (*fs).nactvar as libc::c_int {
            exp2reg(fs, e, (*e).u.info);
            return (*e).u.info;
        }
    }
    luaK_exp2nextreg(fs, e);
    return (*e).u.info;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2RK(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
) -> libc::c_int {
    let mut current_block: u64;
    luaK_exp2val(fs, e);
    match (*e).k as libc::c_uint {
        2 => {
            (*e).u.info = boolK(fs, 1 as libc::c_int);
            current_block = 1121874722387553368;
        }
        3 => {
            (*e).u.info = boolK(fs, 0 as libc::c_int);
            current_block = 1121874722387553368;
        }
        1 => {
            (*e).u.info = nilK(fs);
            current_block = 1121874722387553368;
        }
        6 => {
            (*e).u.info = luaK_intK(fs, (*e).u.ival);
            current_block = 1121874722387553368;
        }
        5 => {
            (*e).u.info = luaK_numberK(fs, (*e).u.nval);
            current_block = 1121874722387553368;
        }
        4 => {
            current_block = 1121874722387553368;
        }
        _ => {
            current_block = 7651349459974463963;
        }
    }
    match current_block {
        1121874722387553368 => {
            (*e).k = VK;
            if (*e).u.info <= MAXINDEXRK {
                return RKASK!(e -> u.info);
            }
        }
        _ => {}
    }
    return luaK_exp2anyreg(fs, e);
}
unsafe extern "C" fn freereg(mut fs: *mut FuncState, mut reg: libc::c_int) {
    if ISK!(reg) == 0 && reg >= (*fs).nactvar as libc::c_int {
        (*fs).freereg = ((*fs).freereg).wrapping_sub(1);
    }
}
unsafe extern "C" fn freeexp(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        freereg(fs, (*e).u.info);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaK_storevar(
    mut fs: *mut FuncState,
    mut var: *mut expdesc,
    mut ex: *mut expdesc,
) {
    match (*var).k as libc::c_uint {
        8 => {
            freeexp(fs, ex);
            exp2reg(fs, ex, (*var).u.info);
            return;
        }
        9 => {
            let mut e = luaK_exp2anyreg(fs, ex);
            luaK_codeABC(fs, OP_SETUPVAL, e, (*var).u.info, 0 as libc::c_int);
        }
        10 => {
            let mut op = (if (*var).u.ind.vt as libc::c_int == VLOCAL as libc::c_int {
                OP_SETTABLE as libc::c_int
            } else {
                OP_SETTABUP as libc::c_int
            }) as OpCode;
            let mut e_0 = luaK_exp2RK(fs, ex);
            luaK_codeABC(
                fs,
                op,
                (*var).u.ind.t as libc::c_int,
                (*var).u.ind.idx as libc::c_int,
                e_0,
            );
        }
        _ => {}
    }
    freeexp(fs, ex);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchlist(
    mut fs: *mut FuncState,
    mut list: libc::c_int,
    mut target: libc::c_int,
) {
    if target == (*fs).pc {
        luaK_patchtohere(fs, list);
    } else {
        patchlistaux(fs, list, target, NO_REG, target);
    };
}
unsafe extern "C" fn patchlistaux(
    mut fs: *mut FuncState,
    mut list: libc::c_int,
    mut vtarget: libc::c_int,
    mut reg: libc::c_int,
    mut dtarget: libc::c_int,
) {
    while list != NO_JUMP {
        let mut next = getjump(fs, list);
        if patchtestreg(fs, list, reg) != 0 {
            fixjump(fs, list, vtarget);
        } else {
            fixjump(fs, list, dtarget);
        }
        list = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaK_codeABC(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    return luaK_code(fs, CREATE_ABC!(o, a, b, c));
}
#[no_mangle]
pub unsafe extern "C" fn luaK_ret(
    mut fs: *mut FuncState,
    mut first: libc::c_int,
    mut nret: libc::c_int,
) {
    luaK_codeABC(fs, OP_RETURN, first, nret + 1 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn dischargejpc(mut fs: *mut FuncState) {
    patchlistaux(fs, (*fs).jpc, (*fs).pc, NO_REG, (*fs).pc);
    (*fs).jpc = NO_JUMP;
}
unsafe extern "C" fn luaK_code(
    mut fs: *mut FuncState,
    mut i: Instruction,
) -> libc::c_int {
    let mut f = (*fs).f;
    dischargejpc(fs);
    if luaM_growvector!(
        fs -> ls -> L, f -> code, fs -> pc, f -> sizecode, Instruction, MAX_INT,
        "opcodes"
    ) != 0
    {
        let ref mut fresh146 = luaM_growvector!(
            fs -> ls -> L, f -> code, fs -> pc, f -> sizecode, Instruction, MAX_INT,
            "opcodes"
        );
        *fresh146 = luaM_growaux_(
            (*(*fs).ls).L,
            (*f).code as *mut libc::c_void,
            &mut (*f).sizecode,
            ::core::mem::size_of::<Instruction>() as libc::c_ulong,
            2147483647 as libc::c_int,
            b"opcodes\0" as *const u8 as *const libc::c_char,
        ) as *mut Instruction;
    }
    *((*f).code).offset((*fs).pc as isize) = i;
    if luaM_growvector!(
        fs -> ls -> L, f -> lineinfo, fs -> pc, f -> sizelineinfo, int, MAX_INT,
        "opcodes"
    ) != 0
    {
        let ref mut fresh147 = luaM_growvector!(
            fs -> ls -> L, f -> lineinfo, fs -> pc, f -> sizelineinfo, int, MAX_INT,
            "opcodes"
        );
        *fresh147 = luaM_growaux_(
            (*(*fs).ls).L,
            (*f).lineinfo as *mut libc::c_void,
            &mut (*f).sizelineinfo,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            2147483647 as libc::c_int,
            b"opcodes\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_int;
    }
    *((*f).lineinfo).offset((*fs).pc as isize) = (*(*fs).ls).lastline;
    let fresh148 = (*fs).pc;
    (*fs).pc = (*fs).pc + 1;
    return fresh148;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_codeABx(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: libc::c_int,
    mut bc: libc::c_uint,
) -> libc::c_int {
    return luaK_code(fs, CREATE_ABx!(o, a, bc));
}
#[no_mangle]
pub unsafe extern "C" fn luaK_jump(mut fs: *mut FuncState) -> libc::c_int {
    let mut jpc = (*fs).jpc;
    let mut j: libc::c_int = 0;
    (*fs).jpc = NO_JUMP;
    j = luaK_codeAsBx!(fs, OP_JMP, 0, NO_JUMP);
    luaK_concat(fs, &mut j, jpc);
    return j;
}
unsafe extern "C" fn getjump(
    mut fs: *mut FuncState,
    mut pc: libc::c_int,
) -> libc::c_int {
    let mut offset = GETARG_sBx!(fs -> f -> code[pc]);
    if offset == NO_JUMP {
        return NO_JUMP
    } else {
        return pc + 1 as libc::c_int + offset
    };
}
unsafe extern "C" fn fixjump(
    mut fs: *mut FuncState,
    mut pc: libc::c_int,
    mut dest: libc::c_int,
) {
    let mut jmp: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize)
        as *mut Instruction;
    let mut offset = dest - (pc + 1 as libc::c_int);
    if abs(offset) > MAXARG_sBx {
        luaX_syntaxerror(
            (*fs).ls,
            b"control structure too long\0" as *const u8 as *const libc::c_char,
        );
    }
    let ref mut fresh149 = SETARG_sBx!(* jmp, offset);
    *fresh149 = SETARG_sBx!(* jmp, offset);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchclose(
    mut fs: *mut FuncState,
    mut list: libc::c_int,
    mut level: libc::c_int,
) {
    level += 1;
    while list != NO_JUMP {
        let ref mut fresh150 = SETARG_A!(fs -> f -> code[list], level);
        *fresh150 = SETARG_A!(fs -> f -> code[list], level);
        list = getjump(fs, list);
    }
}
unsafe extern "C" fn patchtestreg(
    mut fs: *mut FuncState,
    mut node: libc::c_int,
    mut reg: libc::c_int,
) -> libc::c_int {
    let mut i = getjumpcontrol(fs, node);
    if GET_OPCODE!(* i) != OP_TESTSET as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if reg != NO_REG && reg != GETARG_B!(* i) {
        let ref mut fresh151 = SETARG_A!(* i, reg);
        *fresh151 = SETARG_A!(* i, reg);
    } else {
        *i = (OP_TEST as libc::c_int as Instruction) << POS_OP
            | ((*i
                >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                    + 9 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_int as Instruction) << POS_A
            | (0 as libc::c_int as Instruction) << POS_B
            | ((*i >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_int as Instruction) << POS_C;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_getlabel(mut fs: *mut FuncState) -> libc::c_int {
    (*fs).lasttarget = (*fs).pc;
    return (*fs).pc;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_concat(
    mut fs: *mut FuncState,
    mut l1: *mut libc::c_int,
    mut l2: libc::c_int,
) {
    if l2 == NO_JUMP {
        return
    } else {
        if *l1 == NO_JUMP {
            *l1 = l2;
        } else {
            let mut list = *l1;
            let mut next: libc::c_int = 0;
            loop {
                next = getjump(fs, list);
                if !(next != NO_JUMP) {
                    break;
                }
                list = next;
            }
            fixjump(fs, list, l2);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchtohere(
    mut fs: *mut FuncState,
    mut list: libc::c_int,
) {
    luaK_getlabel(fs);
    luaK_concat(fs, &mut (*fs).jpc, list);
}
unsafe extern "C" fn getjumpcontrol(
    mut fs: *mut FuncState,
    mut pc: libc::c_int,
) -> *mut Instruction {
    let mut pi: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize)
        as *mut Instruction;
    if pc >= 1 as libc::c_int
        && luaP_opmodes[(*pi.offset(-(1 as libc::c_int as isize)) >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 6 as libc::c_int)
                << 0 as libc::c_int) as OpCode as usize] as libc::c_int
            & (1 as libc::c_int) << 7 as libc::c_int != 0
    {
        return pi.offset(-(1 as libc::c_int as isize))
    } else {
        return pi
    };
}
pub const MAXVARS: libc::c_int = 200 as libc::c_int;
unsafe extern "C" fn semerror(mut ls: *mut LexState, mut msg: *const libc::c_char) -> ! {
    (*ls).t.token = 0 as libc::c_int;
    luaX_syntaxerror(ls, msg);
}
unsafe extern "C" fn error_expected(mut ls: *mut LexState, mut token: libc::c_int) -> ! {
    luaX_syntaxerror(
        ls,
        luaO_pushfstring(
            (*ls).L,
            b"%s expected\0" as *const u8 as *const libc::c_char,
            luaX_token2str(ls, token),
        ),
    );
}
unsafe extern "C" fn errorlimit(
    mut fs: *mut FuncState,
    mut limit: libc::c_int,
    mut what: *const libc::c_char,
) -> ! {
    let mut L = (*(*fs).ls).L;
    let mut msg = 0 as *const libc::c_char;
    let mut line = (*(*fs).f).linedefined;
    let mut where_0 = if line == 0 as libc::c_int {
        b"main function\0" as *const u8 as *const libc::c_char
    } else {
        luaO_pushfstring(
            L,
            b"function at line %d\0" as *const u8 as *const libc::c_char,
            line,
        )
    };
    msg = luaO_pushfstring(
        L,
        b"too many %s (limit is %d) in %s\0" as *const u8 as *const libc::c_char,
        what,
        limit,
        where_0,
    );
    luaX_syntaxerror((*fs).ls, msg);
}
unsafe extern "C" fn checklimit(
    mut fs: *mut FuncState,
    mut v: libc::c_int,
    mut l: libc::c_int,
    mut what: *const libc::c_char,
) {
    if v > l {
        errorlimit(fs, l, what);
    }
}
unsafe extern "C" fn testnext(mut ls: *mut LexState, mut c: libc::c_int) -> libc::c_int {
    if (*ls).t.token == c {
        luaX_next(ls);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn check(mut ls: *mut LexState, mut c: libc::c_int) {
    if (*ls).t.token != c {
        error_expected(ls, c);
    }
}
unsafe extern "C" fn checknext(mut ls: *mut LexState, mut c: libc::c_int) {
    check(ls, c);
    luaX_next(ls);
}
unsafe extern "C" fn check_match(
    mut ls: *mut LexState,
    mut what: libc::c_int,
    mut who: libc::c_int,
    mut where_0: libc::c_int,
) {
    if testnext(ls, what) == 0 {
        if where_0 == (*ls).linenumber {
            error_expected(ls, what);
        } else {
            luaX_syntaxerror(
                ls,
                luaO_pushfstring(
                    (*ls).L,
                    b"%s expected (to close %s at line %d)\0" as *const u8
                        as *const libc::c_char,
                    luaX_token2str(ls, what),
                    luaX_token2str(ls, who),
                    where_0,
                ),
            );
        }
    }
}
unsafe extern "C" fn str_checkname(mut ls: *mut LexState) -> *mut TString {
    let mut ts = 0 as *mut TString;
    check(ls, TK_NAME as libc::c_int);
    ts = (*ls).t.seminfo.ts;
    luaX_next(ls);
    return ts;
}
unsafe extern "C" fn init_exp(mut e: *mut expdesc, mut k: expkind, mut i: libc::c_int) {
    (*e).t = NO_JUMP;
    (*e).f = (*e).t;
    (*e).k = k;
    (*e).u.info = i;
}
unsafe extern "C" fn codestring(
    mut ls: *mut LexState,
    mut e: *mut expdesc,
    mut s: *mut TString,
) {
    init_exp(e, VK, luaK_stringK((*ls).fs, s));
}
unsafe extern "C" fn checkname(mut ls: *mut LexState, mut e: *mut expdesc) {
    codestring(ls, e, str_checkname(ls));
}
unsafe extern "C" fn registerlocalvar(
    mut ls: *mut LexState,
    mut varname: *mut TString,
) -> libc::c_int {
    let mut fs = (*ls).fs;
    let mut f = (*fs).f;
    let mut oldsize = (*f).sizelocvars;
    if luaM_growvector!(
        ls -> L, f -> locvars, fs -> nlocvars, f -> sizelocvars, LocVar, SHRT_MAX,
        "local variables"
    ) != 0
    {
        let ref mut fresh152 = luaM_growvector!(
            ls -> L, f -> locvars, fs -> nlocvars, f -> sizelocvars, LocVar, SHRT_MAX,
            "local variables"
        );
        *fresh152 = luaM_growaux_(
            (*ls).L,
            (*f).locvars as *mut libc::c_void,
            &mut (*f).sizelocvars,
            ::core::mem::size_of::<LocVar>() as libc::c_ulong,
            32767 as libc::c_int,
            b"local variables\0" as *const u8 as *const libc::c_char,
        ) as *mut LocVar;
    }
    while oldsize < (*f).sizelocvars {
        let fresh153 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh154 = (*((*f).locvars).offset(fresh153 as isize)).varname;
        *fresh154 = NULL as *mut TString;
    }
    let ref mut fresh155 = (*((*f).locvars).offset((*fs).nlocvars as isize)).varname;
    *fresh155 = varname;
    if luaC_objbarrier!(ls -> L, f, varname) != 0 {} else {};
    let fresh156 = (*fs).nlocvars;
    (*fs).nlocvars = (*fs).nlocvars + 1;
    return fresh156 as libc::c_int;
}
unsafe extern "C" fn new_localvar(mut ls: *mut LexState, mut name: *mut TString) {
    let mut fs = (*ls).fs;
    let mut dyd = (*ls).dyd;
    let mut reg = registerlocalvar(ls, name);
    checklimit(
        fs,
        (*dyd).actvar.n + 1 as libc::c_int - (*fs).firstlocal,
        MAXVARS,
        b"local variables\0" as *const u8 as *const libc::c_char,
    );
    if luaM_growvector!(
        ls -> L, dyd -> actvar.arr, dyd -> actvar.n + 1, dyd -> actvar.size, Vardesc,
        MAX_INT, "local variables"
    ) != 0
    {
        let ref mut fresh157 = luaM_growvector!(
            ls -> L, dyd -> actvar.arr, dyd -> actvar.n + 1, dyd -> actvar.size, Vardesc,
            MAX_INT, "local variables"
        );
        *fresh157 = luaM_growaux_(
            (*ls).L,
            (*dyd).actvar.arr as *mut libc::c_void,
            &mut (*dyd).actvar.size,
            ::core::mem::size_of::<Vardesc>() as libc::c_ulong,
            2147483647 as libc::c_int,
            b"local variables\0" as *const u8 as *const libc::c_char,
        ) as *mut Vardesc;
    }
    let fresh158 = (*dyd).actvar.n;
    (*dyd).actvar.n = (*dyd).actvar.n + 1;
    (*((*dyd).actvar.arr).offset(fresh158 as isize)).idx = cast!(short, reg);
}
unsafe extern "C" fn new_localvarliteral_(
    mut ls: *mut LexState,
    mut name: *const libc::c_char,
    mut sz: size_t,
) {
    new_localvar(ls, luaX_newstring(ls, name, sz));
}
unsafe extern "C" fn getlocvar(
    mut fs: *mut FuncState,
    mut i: libc::c_int,
) -> *mut LocVar {
    let mut idx = (*((*(*(*fs).ls).dyd).actvar.arr)
        .offset(((*fs).firstlocal + i) as isize))
        .idx as libc::c_int;
    return &mut *((*(*fs).f).locvars).offset(idx as isize) as *mut LocVar;
}
unsafe extern "C" fn adjustlocalvars(mut ls: *mut LexState, mut nvars: libc::c_int) {
    let mut fs = (*ls).fs;
    (*fs).nactvar = cast_byte!(fs -> nactvar + nvars);
    while nvars != 0 {
        (*getlocvar(fs, (*fs).nactvar as libc::c_int - nvars)).startpc = (*fs).pc;
        nvars -= 1;
    }
}
unsafe extern "C" fn removevars(mut fs: *mut FuncState, mut tolevel: libc::c_int) {
    (*(*(*fs).ls).dyd).actvar.n -= (*fs).nactvar as libc::c_int - tolevel;
    while (*fs).nactvar as libc::c_int > tolevel {
        (*fs).nactvar = ((*fs).nactvar).wrapping_sub(1);
        (*getlocvar(fs, (*fs).nactvar as libc::c_int)).endpc = (*fs).pc;
    }
}
unsafe extern "C" fn searchupvalue(
    mut fs: *mut FuncState,
    mut name: *mut TString,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut up = (*(*fs).f).upvalues;
    i = 0 as libc::c_int;
    while i < (*fs).nups as libc::c_int {
        if eqstr!(up[i].name, name) != 0 {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn newupvalue(
    mut fs: *mut FuncState,
    mut name: *mut TString,
    mut v: *mut expdesc,
) -> libc::c_int {
    let mut f = (*fs).f;
    let mut oldsize = (*f).sizeupvalues;
    checklimit(
        fs,
        (*fs).nups as libc::c_int + 1 as libc::c_int,
        MAXUPVAL,
        b"upvalues\0" as *const u8 as *const libc::c_char,
    );
    if luaM_growvector!(
        fs -> ls -> L, f -> upvalues, fs -> nups, f -> sizeupvalues, Upvaldesc, MAXUPVAL,
        "upvalues"
    ) != 0
    {
        let ref mut fresh159 = luaM_growvector!(
            fs -> ls -> L, f -> upvalues, fs -> nups, f -> sizeupvalues, Upvaldesc,
            MAXUPVAL, "upvalues"
        );
        *fresh159 = luaM_growaux_(
            (*(*fs).ls).L,
            (*f).upvalues as *mut libc::c_void,
            &mut (*f).sizeupvalues,
            ::core::mem::size_of::<Upvaldesc>() as libc::c_ulong,
            255 as libc::c_int,
            b"upvalues\0" as *const u8 as *const libc::c_char,
        ) as *mut Upvaldesc;
    }
    while oldsize < (*f).sizeupvalues {
        let fresh160 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh161 = (*((*f).upvalues).offset(fresh160 as isize)).name;
        *fresh161 = NULL as *mut TString;
    }
    (*((*f).upvalues).offset((*fs).nups as isize))
        .instack = ((*v).k as libc::c_uint == VLOCAL as libc::c_int as libc::c_uint)
        as libc::c_int as lu_byte;
    (*((*f).upvalues).offset((*fs).nups as isize)).idx = cast_byte!(v -> u.info);
    let ref mut fresh162 = (*((*f).upvalues).offset((*fs).nups as isize)).name;
    *fresh162 = name;
    if luaC_objbarrier!(fs -> ls -> L, f, name) != 0 {} else {};
    let fresh163 = (*fs).nups;
    (*fs).nups = ((*fs).nups).wrapping_add(1);
    return fresh163 as libc::c_int;
}
unsafe extern "C" fn searchvar(
    mut fs: *mut FuncState,
    mut n: *mut TString,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = cast_int!(fs -> nactvar) - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if eqstr!(n, getlocvar(fs, i) -> varname) != 0 {
            return i;
        }
        i -= 1;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn markupval(mut fs: *mut FuncState, mut level: libc::c_int) {
    let mut bl = (*fs).bl;
    while (*bl).nactvar as libc::c_int > level {
        bl = (*bl).previous;
    }
    (*bl).upval = 1 as libc::c_int as lu_byte;
}
unsafe extern "C" fn singlevaraux(
    mut fs: *mut FuncState,
    mut n: *mut TString,
    mut var: *mut expdesc,
    mut base: libc::c_int,
) {
    if fs.is_null() {
        init_exp(var, VVOID, 0 as libc::c_int);
    } else {
        let mut v = searchvar(fs, n);
        if v >= 0 as libc::c_int {
            init_exp(var, VLOCAL, v);
            if base == 0 {
                markupval(fs, v);
            }
        } else {
            let mut idx = searchupvalue(fs, n);
            if idx < 0 as libc::c_int {
                singlevaraux((*fs).prev, n, var, 0 as libc::c_int);
                if (*var).k as libc::c_uint == VVOID as libc::c_int as libc::c_uint {
                    return;
                }
                idx = newupvalue(fs, n, var);
            }
            init_exp(var, VUPVAL, idx);
        }
    };
}
unsafe extern "C" fn singlevar(mut ls: *mut LexState, mut var: *mut expdesc) {
    let mut varname = str_checkname(ls);
    let mut fs = (*ls).fs;
    singlevaraux(fs, varname, var, 1 as libc::c_int);
    if (*var).k as libc::c_uint == VVOID as libc::c_int as libc::c_uint {
        let mut key = expdesc {
            k: VVOID,
            u: C2RustUnnamed_8 { ival: 0 },
            t: 0,
            f: 0,
        };
        singlevaraux(fs, (*ls).envn, var, 1 as libc::c_int);
        codestring(ls, &mut key, varname);
        luaK_indexed(fs, var, &mut key);
    }
}
unsafe extern "C" fn adjust_assign(
    mut ls: *mut LexState,
    mut nvars: libc::c_int,
    mut nexps: libc::c_int,
    mut e: *mut expdesc,
) {
    let mut fs = (*ls).fs;
    let mut extra = nvars - nexps;
    if hasmultret!(e -> k) != 0 {
        extra += 1;
        if extra < 0 as libc::c_int {
            extra = 0 as libc::c_int;
        }
        luaK_setreturns(fs, e, extra);
        if extra > 1 as libc::c_int {
            luaK_reserveregs(fs, extra - 1 as libc::c_int);
        }
    } else {
        if (*e).k as libc::c_uint != VVOID as libc::c_int as libc::c_uint {
            luaK_exp2nextreg(fs, e);
        }
        if extra > 0 as libc::c_int {
            let mut reg = (*fs).freereg as libc::c_int;
            luaK_reserveregs(fs, extra);
            luaK_nil(fs, reg, extra);
        }
    }
    if nexps > nvars {
        (*(*ls).fs)
            .freereg = ((*(*ls).fs).freereg as libc::c_int - (nexps - nvars)) as lu_byte;
    }
}
unsafe extern "C" fn enterlevel(mut ls: *mut LexState) {
    let mut L = (*ls).L;
    (*L).nCcalls = ((*L).nCcalls).wrapping_add(1);
    checklimit(
        (*ls).fs,
        (*L).nCcalls as libc::c_int,
        LUAI_MAXCCALLS,
        b"C levels\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn closegoto(
    mut ls: *mut LexState,
    mut g: libc::c_int,
    mut label: *mut Labeldesc,
) {
    let mut i: libc::c_int = 0;
    let mut fs = (*ls).fs;
    let mut gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let mut gt: *mut Labeldesc = &mut *((*gl).arr).offset(g as isize) as *mut Labeldesc;
    if ((*gt).nactvar as libc::c_int) < (*label).nactvar as libc::c_int {
        let mut vname = (*getlocvar(fs, (*gt).nactvar as libc::c_int)).varname;
        let mut msg = luaO_pushfstring(
            (*ls).L,
            b"<goto %s> at line %d jumps into the scope of local '%s'\0" as *const u8
                as *const libc::c_char,
            getstr!(gt -> name),
            (*gt).line,
            getstr!(vname),
        );
        semerror(ls, msg);
    }
    luaK_patchlist(fs, (*gt).pc, (*label).pc);
    i = g;
    while i < (*gl).n - 1 as libc::c_int {
        *((*gl).arr)
            .offset(i as isize) = *((*gl).arr).offset((i + 1 as libc::c_int) as isize);
        i += 1;
    }
    (*gl).n -= 1;
}
unsafe extern "C" fn findlabel(
    mut ls: *mut LexState,
    mut g: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut bl = (*(*ls).fs).bl;
    let mut dyd = (*ls).dyd;
    let mut gt: *mut Labeldesc = &mut *((*dyd).gt.arr).offset(g as isize)
        as *mut Labeldesc;
    i = (*bl).firstlabel;
    while i < (*dyd).label.n {
        let mut lb: *mut Labeldesc = &mut *((*dyd).label.arr).offset(i as isize)
            as *mut Labeldesc;
        if eqstr!(lb -> name, gt -> name) != 0 {
            if (*gt).nactvar as libc::c_int > (*lb).nactvar as libc::c_int
                && ((*bl).upval as libc::c_int != 0 || (*dyd).label.n > (*bl).firstlabel)
            {
                luaK_patchclose((*ls).fs, (*gt).pc, (*lb).nactvar as libc::c_int);
            }
            closegoto(ls, g, lb);
            return 1 as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn newlabelentry(
    mut ls: *mut LexState,
    mut l: *mut Labellist,
    mut name: *mut TString,
    mut line: libc::c_int,
    mut pc: libc::c_int,
) -> libc::c_int {
    let mut n = (*l).n;
    if luaM_growvector!(
        ls -> L, l -> arr, n, l -> size, Labeldesc, SHRT_MAX, "labels/gotos"
    ) != 0
    {
        let ref mut fresh164 = luaM_growvector!(
            ls -> L, l -> arr, n, l -> size, Labeldesc, SHRT_MAX, "labels/gotos"
        );
        *fresh164 = luaM_growaux_(
            (*ls).L,
            (*l).arr as *mut libc::c_void,
            &mut (*l).size,
            ::core::mem::size_of::<Labeldesc>() as libc::c_ulong,
            32767 as libc::c_int,
            b"labels/gotos\0" as *const u8 as *const libc::c_char,
        ) as *mut Labeldesc;
    }
    let ref mut fresh165 = (*((*l).arr).offset(n as isize)).name;
    *fresh165 = name;
    (*((*l).arr).offset(n as isize)).line = line;
    (*((*l).arr).offset(n as isize)).nactvar = (*(*ls).fs).nactvar;
    (*((*l).arr).offset(n as isize)).pc = pc;
    (*l).n = n + 1 as libc::c_int;
    return n;
}
unsafe extern "C" fn findgotos(mut ls: *mut LexState, mut lb: *mut Labeldesc) {
    let mut gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let mut i = (*(*(*ls).fs).bl).firstgoto;
    while i < (*gl).n {
        if eqstr!(gl -> arr[i].name, lb -> name) != 0 {
            closegoto(ls, i, lb);
        } else {
            i += 1;
        }
    }
}
unsafe extern "C" fn movegotosout(mut fs: *mut FuncState, mut bl: *mut BlockCnt) {
    let mut i = (*bl).firstgoto;
    let mut gl: *mut Labellist = &mut (*(*(*fs).ls).dyd).gt;
    while i < (*gl).n {
        let mut gt: *mut Labeldesc = &mut *((*gl).arr).offset(i as isize)
            as *mut Labeldesc;
        if (*gt).nactvar as libc::c_int > (*bl).nactvar as libc::c_int {
            if (*bl).upval != 0 {
                luaK_patchclose(fs, (*gt).pc, (*bl).nactvar as libc::c_int);
            }
            (*gt).nactvar = (*bl).nactvar;
        }
        if findlabel((*fs).ls, i) == 0 {
            i += 1;
        }
    }
}
unsafe extern "C" fn enterblock(
    mut fs: *mut FuncState,
    mut bl: *mut BlockCnt,
    mut isloop: lu_byte,
) {
    (*bl).isloop = isloop;
    (*bl).nactvar = (*fs).nactvar;
    (*bl).firstlabel = (*(*(*fs).ls).dyd).label.n;
    (*bl).firstgoto = (*(*(*fs).ls).dyd).gt.n;
    (*bl).upval = 0 as libc::c_int as lu_byte;
    (*bl).previous = (*fs).bl;
    (*fs).bl = bl;
}
unsafe extern "C" fn breaklabel(mut ls: *mut LexState) {
    let mut n = luaS_new((*ls).L, b"break\0" as *const u8 as *const libc::c_char);
    let mut l = newlabelentry(
        ls,
        &mut (*(*ls).dyd).label,
        n,
        0 as libc::c_int,
        (*(*ls).fs).pc,
    );
    findgotos(ls, &mut *((*(*ls).dyd).label.arr).offset(l as isize));
}
unsafe extern "C" fn undefgoto(mut ls: *mut LexState, mut gt: *mut Labeldesc) -> ! {
    let mut msg = if isreserved!(gt -> name) != 0 {
        b"<%s> at line %d not inside a loop\0" as *const u8 as *const libc::c_char
    } else {
        b"no visible label '%s' for <goto> at line %d\0" as *const u8
            as *const libc::c_char
    };
    msg = luaO_pushfstring((*ls).L, msg, getstr!(gt -> name), (*gt).line);
    semerror(ls, msg);
}
unsafe extern "C" fn leaveblock(mut fs: *mut FuncState) {
    let mut bl = (*fs).bl;
    let mut ls = (*fs).ls;
    if !((*bl).previous).is_null() && (*bl).upval as libc::c_int != 0 {
        let mut j = luaK_jump(fs);
        luaK_patchclose(fs, j, (*bl).nactvar as libc::c_int);
        luaK_patchtohere(fs, j);
    }
    if (*bl).isloop != 0 {
        breaklabel(ls);
    }
    (*fs).bl = (*bl).previous;
    removevars(fs, (*bl).nactvar as libc::c_int);
    (*fs).freereg = (*fs).nactvar;
    (*(*ls).dyd).label.n = (*bl).firstlabel;
    if !((*bl).previous).is_null() {
        movegotosout(fs, bl);
    } else if (*bl).firstgoto < (*(*ls).dyd).gt.n {
        undefgoto(ls, &mut *((*(*ls).dyd).gt.arr).offset((*bl).firstgoto as isize));
    }
}
unsafe extern "C" fn addprototype(mut ls: *mut LexState) -> *mut Proto {
    let mut clp = 0 as *mut Proto;
    let mut L = (*ls).L;
    let mut fs = (*ls).fs;
    let mut f = (*fs).f;
    if (*fs).np >= (*f).sizep {
        let mut oldsize = (*f).sizep;
        if luaM_growvector!(
            L, f -> p, fs -> np, f -> sizep, Proto *, MAXARG_Bx, "functions"
        ) != 0
        {
            let ref mut fresh166 = luaM_growvector!(
                L, f -> p, fs -> np, f -> sizep, Proto *, MAXARG_Bx, "functions"
            );
            *fresh166 = luaM_growaux_(
                L,
                (*f).p as *mut libc::c_void,
                &mut (*f).sizep,
                ::core::mem::size_of::<*mut Proto>() as libc::c_ulong,
                ((1 as libc::c_int) << 9 as libc::c_int + 9 as libc::c_int)
                    - 1 as libc::c_int,
                b"functions\0" as *const u8 as *const libc::c_char,
            ) as *mut *mut Proto;
        }
        while oldsize < (*f).sizep {
            let fresh167 = oldsize;
            oldsize = oldsize + 1;
            let ref mut fresh168 = *((*f).p).offset(fresh167 as isize);
            *fresh168 = NULL as *mut Proto;
        }
    }
    clp = luaF_newproto(L);
    let fresh169 = (*fs).np;
    (*fs).np = (*fs).np + 1;
    let ref mut fresh170 = *((*f).p).offset(fresh169 as isize);
    *fresh170 = clp;
    if luaC_objbarrier!(L, f, clp) != 0 {} else {};
    return clp;
}
unsafe extern "C" fn codeclosure(mut ls: *mut LexState, mut v: *mut expdesc) {
    let mut fs = (*(*ls).fs).prev;
    init_exp(
        v,
        VRELOCABLE,
        luaK_codeABx(
            fs,
            OP_CLOSURE,
            0 as libc::c_int,
            ((*fs).np - 1 as libc::c_int) as libc::c_uint,
        ),
    );
    luaK_exp2nextreg(fs, v);
}
unsafe extern "C" fn open_func(
    mut ls: *mut LexState,
    mut fs: *mut FuncState,
    mut bl: *mut BlockCnt,
) {
    let mut f = 0 as *mut Proto;
    (*fs).prev = (*ls).fs;
    (*fs).ls = ls;
    (*ls).fs = fs;
    (*fs).pc = 0 as libc::c_int;
    (*fs).lasttarget = 0 as libc::c_int;
    (*fs).jpc = NO_JUMP;
    (*fs).freereg = 0 as libc::c_int as lu_byte;
    (*fs).nk = 0 as libc::c_int;
    (*fs).np = 0 as libc::c_int;
    (*fs).nups = 0 as libc::c_int as lu_byte;
    (*fs).nlocvars = 0 as libc::c_int as libc::c_short;
    (*fs).nactvar = 0 as libc::c_int as lu_byte;
    (*fs).firstlocal = (*(*ls).dyd).actvar.n;
    (*fs).bl = NULL as *mut BlockCnt;
    f = (*fs).f;
    (*f).source = (*ls).source;
    if luaC_objbarrier!(ls -> L, f, f -> source) != 0 {} else {};
    (*f).maxstacksize = 2 as libc::c_int as lu_byte;
    enterblock(fs, bl, 0 as libc::c_int as lu_byte);
}
unsafe extern "C" fn close_func(mut ls: *mut LexState) {
    let mut L = (*ls).L;
    let mut fs = (*ls).fs;
    let mut f = (*fs).f;
    luaK_ret(fs, 0 as libc::c_int, 0 as libc::c_int);
    leaveblock(fs);
    if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).pc as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
            > (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<Instruction>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {};
    let ref mut fresh171 = luaM_reallocvector!(
        L, f -> code, f -> sizecode, fs -> pc, Instruction
    );
    *fresh171 = luaM_realloc_(
        L,
        (*f).code as *mut libc::c_void,
        ((*f).sizecode as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Instruction>() as libc::c_ulong),
        ((*fs).pc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Instruction>() as libc::c_ulong),
    ) as *mut Instruction;
    (*f).sizecode = (*fs).pc;
    if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).pc as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
            > (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {};
    let ref mut fresh172 = luaM_reallocvector!(
        L, f -> lineinfo, f -> sizelineinfo, fs -> pc, int
    );
    *fresh172 = luaM_realloc_(
        L,
        (*f).lineinfo as *mut libc::c_void,
        ((*f).sizelineinfo as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ((*fs).pc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*f).sizelineinfo = (*fs).pc;
    if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).nk as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
            > (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {};
    let ref mut fresh173 = luaM_reallocvector!(L, f -> k, f -> sizek, fs -> nk, TValue);
    *fresh173 = luaM_realloc_(
        L,
        (*f).k as *mut libc::c_void,
        ((*f).sizek as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
        ((*fs).nk as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    (*f).sizek = (*fs).nk;
    if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).np as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
            > (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<*mut Proto>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {};
    let ref mut fresh174 = luaM_reallocvector!(L, f -> p, f -> sizep, fs -> np, Proto *);
    *fresh174 = luaM_realloc_(
        L,
        (*f).p as *mut libc::c_void,
        ((*f).sizep as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Proto>() as libc::c_ulong),
        ((*fs).np as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Proto>() as libc::c_ulong),
    ) as *mut *mut Proto;
    (*f).sizep = (*fs).np;
    if ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).nlocvars as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
            > (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<LocVar>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {};
    let ref mut fresh175 = luaM_reallocvector!(
        L, f -> locvars, f -> sizelocvars, fs -> nlocvars, LocVar
    );
    *fresh175 = luaM_realloc_(
        L,
        (*f).locvars as *mut libc::c_void,
        ((*f).sizelocvars as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LocVar>() as libc::c_ulong),
        ((*fs).nlocvars as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LocVar>() as libc::c_ulong),
    ) as *mut LocVar;
    (*f).sizelocvars = (*fs).nlocvars as libc::c_int;
    if ::core::mem::size_of::<lu_byte>() as libc::c_ulong
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).nups as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
            > (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<Upvaldesc>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {};
    let ref mut fresh176 = luaM_reallocvector!(
        L, f -> upvalues, f -> sizeupvalues, fs -> nups, Upvaldesc
    );
    *fresh176 = luaM_realloc_(
        L,
        (*f).upvalues as *mut libc::c_void,
        ((*f).sizeupvalues as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Upvaldesc>() as libc::c_ulong),
        ((*fs).nups as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Upvaldesc>() as libc::c_ulong),
    ) as *mut Upvaldesc;
    (*f).sizeupvalues = (*fs).nups as libc::c_int;
    (*ls).fs = (*fs).prev;
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
    }
}
unsafe extern "C" fn block_follow(
    mut ls: *mut LexState,
    mut withuntil: libc::c_int,
) -> libc::c_int {
    match (*ls).t.token {
        260 | 261 | 262 | 289 => return 1 as libc::c_int,
        277 => return withuntil,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn statlist(mut ls: *mut LexState) {
    while block_follow(ls, 1 as libc::c_int) == 0 {
        if (*ls).t.token == TK_RETURN as libc::c_int {
            statement(ls);
            return;
        }
        statement(ls);
    }
}
unsafe extern "C" fn fieldsel(mut ls: *mut LexState, mut v: *mut expdesc) {
    let mut fs = (*ls).fs;
    let mut key = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    luaK_exp2anyregup(fs, v);
    luaX_next(ls);
    checkname(ls, &mut key);
    luaK_indexed(fs, v, &mut key);
}
unsafe extern "C" fn yindex(mut ls: *mut LexState, mut v: *mut expdesc) {
    luaX_next(ls);
    expr(ls, v);
    luaK_exp2val((*ls).fs, v);
    checknext(ls, ']' as i32);
}
unsafe extern "C" fn recfield(mut ls: *mut LexState, mut cc: *mut ConsControl) {
    let mut fs = (*ls).fs;
    let mut reg = (*(*ls).fs).freereg as libc::c_int;
    let mut key = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut val = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut rkkey: libc::c_int = 0;
    if (*ls).t.token == TK_NAME as libc::c_int {
        checklimit(
            fs,
            (*cc).nh,
            MAX_INT,
            b"items in a constructor\0" as *const u8 as *const libc::c_char,
        );
        checkname(ls, &mut key);
    } else {
        yindex(ls, &mut key);
    }
    (*cc).nh += 1;
    checknext(ls, '=' as i32);
    rkkey = luaK_exp2RK(fs, &mut key);
    expr(ls, &mut val);
    luaK_codeABC(fs, OP_SETTABLE, (*(*cc).t).u.info, rkkey, luaK_exp2RK(fs, &mut val));
    (*fs).freereg = reg as lu_byte;
}
unsafe extern "C" fn closelistfield(mut fs: *mut FuncState, mut cc: *mut ConsControl) {
    if (*cc).v.k as libc::c_uint == VVOID as libc::c_int as libc::c_uint {
        return;
    }
    luaK_exp2nextreg(fs, &mut (*cc).v);
    (*cc).v.k = VVOID;
    if (*cc).tostore == LFIELDS_PER_FLUSH {
        luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
        (*cc).tostore = 0 as libc::c_int;
    }
}
unsafe extern "C" fn lastlistfield(mut fs: *mut FuncState, mut cc: *mut ConsControl) {
    if (*cc).tostore == 0 as libc::c_int {
        return;
    }
    if hasmultret!(cc -> v.k) != 0 {
        luaK_setmultret!(fs, & cc -> v)(fs, &mut (*cc).v, LUA_MULTRET);
        luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, LUA_MULTRET);
        (*cc).na -= 1;
    } else {
        if (*cc).v.k as libc::c_uint != VVOID as libc::c_int as libc::c_uint {
            luaK_exp2nextreg(fs, &mut (*cc).v);
        }
        luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
    };
}
unsafe extern "C" fn listfield(mut ls: *mut LexState, mut cc: *mut ConsControl) {
    expr(ls, &mut (*cc).v);
    checklimit(
        (*ls).fs,
        (*cc).na,
        MAX_INT,
        b"items in a constructor\0" as *const u8 as *const libc::c_char,
    );
    (*cc).na += 1;
    (*cc).tostore += 1;
}
unsafe extern "C" fn field(mut ls: *mut LexState, mut cc: *mut ConsControl) {
    match (*ls).t.token {
        292 => {
            if luaX_lookahead(ls) != '=' as i32 {
                listfield(ls, cc);
            } else {
                recfield(ls, cc);
            }
        }
        91 => {
            recfield(ls, cc);
        }
        _ => {
            listfield(ls, cc);
        }
    };
}
unsafe extern "C" fn constructor(mut ls: *mut LexState, mut t: *mut expdesc) {
    let mut fs = (*ls).fs;
    let mut line = (*ls).linenumber;
    let mut pc = luaK_codeABC(
        fs,
        OP_NEWTABLE,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    let mut cc = ConsControl {
        v: expdesc {
            k: VVOID,
            u: C2RustUnnamed_8 { ival: 0 },
            t: 0,
            f: 0,
        },
        t: 0 as *mut expdesc,
        nh: 0,
        na: 0,
        tostore: 0,
    };
    cc.tostore = 0 as libc::c_int;
    cc.nh = cc.tostore;
    cc.na = cc.nh;
    cc.t = t;
    init_exp(t, VRELOCABLE, pc);
    init_exp(&mut cc.v, VVOID, 0 as libc::c_int);
    luaK_exp2nextreg((*ls).fs, t);
    checknext(ls, '{' as i32);
    while !((*ls).t.token == '}' as i32) {
        closelistfield(fs, &mut cc);
        field(ls, &mut cc);
        if !(testnext(ls, ',' as i32) != 0 || testnext(ls, ';' as i32) != 0) {
            break;
        }
    }
    check_match(ls, '}' as i32, '{' as i32, line);
    lastlistfield(fs, &mut cc);
    *((*(*fs).f).code)
        .offset(
            pc as isize,
        ) = *((*(*fs).f).code).offset(pc as isize)
        & !(!(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
            << 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int + 9 as libc::c_int)
        | (luaO_int2fb(cc.na as libc::c_uint) as Instruction)
            << 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int + 9 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                << 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                    + 9 as libc::c_int;
    *((*(*fs).f).code)
        .offset(
            pc as isize,
        ) = *((*(*fs).f).code).offset(pc as isize)
        & !(!(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
            << 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int)
        | (luaO_int2fb(cc.nh as libc::c_uint) as Instruction)
            << 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                << 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int;
}
unsafe extern "C" fn parlist(mut ls: *mut LexState) {
    let mut fs = (*ls).fs;
    let mut f = (*fs).f;
    let mut nparams = 0 as libc::c_int;
    (*f).is_vararg = 0 as libc::c_int as lu_byte;
    if (*ls).t.token != ')' as i32 {
        loop {
            match (*ls).t.token {
                292 => {
                    new_localvar(ls, str_checkname(ls));
                    nparams += 1;
                }
                281 => {
                    luaX_next(ls);
                    (*f).is_vararg = 1 as libc::c_int as lu_byte;
                }
                _ => {
                    luaX_syntaxerror(
                        ls,
                        b"<name> or '...' expected\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            if !((*f).is_vararg == 0 && testnext(ls, ',' as i32) != 0) {
                break;
            }
        }
    }
    adjustlocalvars(ls, nparams);
    (*f).numparams = cast_byte!(fs -> nactvar);
    luaK_reserveregs(fs, (*fs).nactvar as libc::c_int);
}
unsafe extern "C" fn body(
    mut ls: *mut LexState,
    mut e: *mut expdesc,
    mut ismethod: libc::c_int,
    mut line: libc::c_int,
) {
    let mut new_fs = FuncState {
        f: 0 as *mut Proto,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        jpc: 0,
        nk: 0,
        np: 0,
        firstlocal: 0,
        nlocvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
    };
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    new_fs.f = addprototype(ls);
    (*new_fs.f).linedefined = line;
    open_func(ls, &mut new_fs, &mut bl);
    checknext(ls, '(' as i32);
    if ismethod != 0 {
        new_localvarliteral!(
            ls, "self"
        )(
            ls,
            b"self\0" as *const u8 as *const libc::c_char,
            new_localvarliteral!(ls, "self"),
        );
        adjustlocalvars(ls, 1 as libc::c_int);
    }
    parlist(ls);
    checknext(ls, ')' as i32);
    statlist(ls);
    (*new_fs.f).lastlinedefined = (*ls).linenumber;
    check_match(ls, TK_END as libc::c_int, TK_FUNCTION as libc::c_int, line);
    codeclosure(ls, e);
    close_func(ls);
}
unsafe extern "C" fn explist(mut ls: *mut LexState, mut v: *mut expdesc) -> libc::c_int {
    let mut n = 1 as libc::c_int;
    expr(ls, v);
    while testnext(ls, ',' as i32) != 0 {
        luaK_exp2nextreg((*ls).fs, v);
        expr(ls, v);
        n += 1;
    }
    return n;
}
unsafe extern "C" fn funcargs(
    mut ls: *mut LexState,
    mut f: *mut expdesc,
    mut line: libc::c_int,
) {
    let mut fs = (*ls).fs;
    let mut args = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut base: libc::c_int = 0;
    let mut nparams: libc::c_int = 0;
    match (*ls).t.token {
        40 => {
            luaX_next(ls);
            if (*ls).t.token == ')' as i32 {
                args.k = VVOID;
            } else {
                explist(ls, &mut args);
                luaK_setmultret!(fs, & args)(fs, &mut args, LUA_MULTRET);
            }
            check_match(ls, ')' as i32, '(' as i32, line);
        }
        123 => {
            constructor(ls, &mut args);
        }
        293 => {
            codestring(ls, &mut args, (*ls).t.seminfo.ts);
            luaX_next(ls);
        }
        _ => {
            luaX_syntaxerror(
                ls,
                b"function arguments expected\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    base = (*f).u.info;
    if hasmultret!(args.k) != 0 {
        nparams = LUA_MULTRET;
    } else {
        if args.k as libc::c_uint != VVOID as libc::c_int as libc::c_uint {
            luaK_exp2nextreg(fs, &mut args);
        }
        nparams = (*fs).freereg as libc::c_int - (base + 1 as libc::c_int);
    }
    init_exp(
        f,
        VCALL,
        luaK_codeABC(fs, OP_CALL, base, nparams + 1 as libc::c_int, 2 as libc::c_int),
    );
    luaK_fixline(fs, line);
    (*fs).freereg = (base + 1 as libc::c_int) as lu_byte;
}
unsafe extern "C" fn primaryexp(mut ls: *mut LexState, mut v: *mut expdesc) {
    match (*ls).t.token {
        40 => {
            let mut line = (*ls).linenumber;
            luaX_next(ls);
            expr(ls, v);
            check_match(ls, ')' as i32, '(' as i32, line);
            luaK_dischargevars((*ls).fs, v);
            return;
        }
        292 => {
            singlevar(ls, v);
            return;
        }
        _ => {
            luaX_syntaxerror(
                ls,
                b"unexpected symbol\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn suffixedexp(mut ls: *mut LexState, mut v: *mut expdesc) {
    let mut fs = (*ls).fs;
    let mut line = (*ls).linenumber;
    primaryexp(ls, v);
    loop {
        match (*ls).t.token {
            46 => {
                fieldsel(ls, v);
            }
            91 => {
                let mut key = expdesc {
                    k: VVOID,
                    u: C2RustUnnamed_8 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaK_exp2anyregup(fs, v);
                yindex(ls, &mut key);
                luaK_indexed(fs, v, &mut key);
            }
            58 => {
                let mut key_0 = expdesc {
                    k: VVOID,
                    u: C2RustUnnamed_8 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaX_next(ls);
                checkname(ls, &mut key_0);
                luaK_self(fs, v, &mut key_0);
                funcargs(ls, v, line);
            }
            40 | 293 | 123 => {
                luaK_exp2nextreg(fs, v);
                funcargs(ls, v, line);
            }
            _ => return,
        }
    };
}
unsafe extern "C" fn simpleexp(mut ls: *mut LexState, mut v: *mut expdesc) {
    match (*ls).t.token {
        290 => {
            init_exp(v, VKFLT, 0 as libc::c_int);
            (*v).u.nval = (*ls).t.seminfo.r;
        }
        291 => {
            init_exp(v, VKINT, 0 as libc::c_int);
            (*v).u.ival = (*ls).t.seminfo.i;
        }
        293 => {
            codestring(ls, v, (*ls).t.seminfo.ts);
        }
        270 => {
            init_exp(v, VNIL, 0 as libc::c_int);
        }
        276 => {
            init_exp(v, VTRUE, 0 as libc::c_int);
        }
        263 => {
            init_exp(v, VFALSE, 0 as libc::c_int);
        }
        281 => {
            let mut fs = (*ls).fs;
            if check_condition!(
                ls, fs -> f -> is_vararg, "cannot use '...' outside a vararg function"
            ) == 0
            {
                check_condition!(
                    ls, fs -> f -> is_vararg,
                    "cannot use '...' outside a vararg function"
                )(
                    ls,
                    b"cannot use '...' outside a vararg function\0" as *const u8
                        as *const libc::c_char,
                );
            }
            init_exp(
                v,
                VVARARG,
                luaK_codeABC(
                    fs,
                    OP_VARARG,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                ),
            );
        }
        123 => {
            constructor(ls, v);
            return;
        }
        265 => {
            luaX_next(ls);
            body(ls, v, 0 as libc::c_int, (*ls).linenumber);
            return;
        }
        _ => {
            suffixedexp(ls, v);
            return;
        }
    }
    luaX_next(ls);
}
unsafe extern "C" fn getunopr(mut op: libc::c_int) -> UnOpr {
    match op {
        271 => return OPR_NOT,
        45 => return OPR_MINUS,
        126 => return OPR_BNOT,
        35 => return OPR_LEN,
        _ => return OPR_NOUNOPR,
    };
}
unsafe extern "C" fn getbinopr(mut op: libc::c_int) -> BinOpr {
    match op {
        43 => return OPR_ADD,
        45 => return OPR_SUB,
        42 => return OPR_MUL,
        37 => return OPR_MOD,
        94 => return OPR_POW,
        47 => return OPR_DIV,
        279 => return OPR_IDIV,
        38 => return OPR_BAND,
        124 => return OPR_BOR,
        126 => return OPR_BXOR,
        286 => return OPR_SHL,
        287 => return OPR_SHR,
        280 => return OPR_CONCAT,
        285 => return OPR_NE,
        282 => return OPR_EQ,
        60 => return OPR_LT,
        284 => return OPR_LE,
        62 => return OPR_GT,
        283 => return OPR_GE,
        257 => return OPR_AND,
        272 => return OPR_OR,
        _ => return OPR_NOBINOPR,
    };
}
static mut priority: [C2RustUnnamed_10; 21] = [
    {
        let mut init = C2RustUnnamed_10 {
            left: 10 as libc::c_int as lu_byte,
            right: 10 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 10 as libc::c_int as lu_byte,
            right: 10 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 11 as libc::c_int as lu_byte,
            right: 11 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 11 as libc::c_int as lu_byte,
            right: 11 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 14 as libc::c_int as lu_byte,
            right: 13 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 11 as libc::c_int as lu_byte,
            right: 11 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 11 as libc::c_int as lu_byte,
            right: 11 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 6 as libc::c_int as lu_byte,
            right: 6 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 4 as libc::c_int as lu_byte,
            right: 4 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 5 as libc::c_int as lu_byte,
            right: 5 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 7 as libc::c_int as lu_byte,
            right: 7 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 7 as libc::c_int as lu_byte,
            right: 7 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 9 as libc::c_int as lu_byte,
            right: 8 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 3 as libc::c_int as lu_byte,
            right: 3 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 3 as libc::c_int as lu_byte,
            right: 3 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 3 as libc::c_int as lu_byte,
            right: 3 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 3 as libc::c_int as lu_byte,
            right: 3 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 3 as libc::c_int as lu_byte,
            right: 3 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 3 as libc::c_int as lu_byte,
            right: 3 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 2 as libc::c_int as lu_byte,
            right: 2 as libc::c_int as lu_byte,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            left: 1 as libc::c_int as lu_byte,
            right: 1 as libc::c_int as lu_byte,
        };
        init
    },
];
pub const UNARY_PRIORITY: libc::c_int = 12 as libc::c_int;
unsafe extern "C" fn subexpr(
    mut ls: *mut LexState,
    mut v: *mut expdesc,
    mut limit: libc::c_int,
) -> BinOpr {
    let mut op = OPR_ADD;
    let mut uop = OPR_MINUS;
    enterlevel(ls);
    uop = getunopr((*ls).t.token);
    if uop as libc::c_uint != OPR_NOUNOPR as libc::c_int as libc::c_uint {
        let mut line = (*ls).linenumber;
        luaX_next(ls);
        subexpr(ls, v, UNARY_PRIORITY);
        luaK_prefix((*ls).fs, uop, v, line);
    } else {
        simpleexp(ls, v);
    }
    op = getbinopr((*ls).t.token);
    while op as libc::c_uint != OPR_NOBINOPR as libc::c_int as libc::c_uint
        && priority[op as usize].left as libc::c_int > limit
    {
        let mut v2 = expdesc {
            k: VVOID,
            u: C2RustUnnamed_8 { ival: 0 },
            t: 0,
            f: 0,
        };
        let mut nextop = OPR_ADD;
        let mut line_0 = (*ls).linenumber;
        luaX_next(ls);
        luaK_infix((*ls).fs, op, v);
        nextop = subexpr(ls, &mut v2, priority[op as usize].right as libc::c_int);
        luaK_posfix((*ls).fs, op, v, &mut v2, line_0);
        op = nextop;
    }
    let ref mut fresh177 = leavelevel!(ls);
    *fresh177 = (*fresh177).wrapping_sub(1);
    return op;
}
unsafe extern "C" fn expr(mut ls: *mut LexState, mut v: *mut expdesc) {
    subexpr(ls, v, 0 as libc::c_int);
}
unsafe extern "C" fn block(mut ls: *mut LexState) {
    let mut fs = (*ls).fs;
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    enterblock(fs, &mut bl, 0 as libc::c_int as lu_byte);
    statlist(ls);
    leaveblock(fs);
}
unsafe extern "C" fn check_conflict(
    mut ls: *mut LexState,
    mut lh: *mut LHS_assign,
    mut v: *mut expdesc,
) {
    let mut fs = (*ls).fs;
    let mut extra = (*fs).freereg as libc::c_int;
    let mut conflict = 0 as libc::c_int;
    while !lh.is_null() {
        if (*lh).v.k as libc::c_uint == VINDEXED as libc::c_int as libc::c_uint {
            if (*lh).v.u.ind.vt as libc::c_uint == (*v).k as libc::c_uint
                && (*lh).v.u.ind.t as libc::c_int == (*v).u.info
            {
                conflict = 1 as libc::c_int;
                (*lh).v.u.ind.vt = VLOCAL as libc::c_int as lu_byte;
                (*lh).v.u.ind.t = extra as lu_byte;
            }
            if (*v).k as libc::c_uint == VLOCAL as libc::c_int as libc::c_uint
                && (*lh).v.u.ind.idx as libc::c_int == (*v).u.info
            {
                conflict = 1 as libc::c_int;
                (*lh).v.u.ind.idx = extra as libc::c_short;
            }
        }
        lh = (*lh).prev;
    }
    if conflict != 0 {
        let mut op = (if (*v).k as libc::c_uint == VLOCAL as libc::c_int as libc::c_uint
        {
            OP_MOVE as libc::c_int
        } else {
            OP_GETUPVAL as libc::c_int
        }) as OpCode;
        luaK_codeABC(fs, op, extra, (*v).u.info, 0 as libc::c_int);
        luaK_reserveregs(fs, 1 as libc::c_int);
    }
}
unsafe extern "C" fn assignment(
    mut ls: *mut LexState,
    mut lh: *mut LHS_assign,
    mut nvars: libc::c_int,
) {
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    if check_condition!(ls, vkisvar(lh -> v.k), "syntax error") == 0 {
        check_condition!(
            ls, vkisvar(lh -> v.k), "syntax error"
        )(ls, b"syntax error\0" as *const u8 as *const libc::c_char);
    }
    if testnext(ls, ',' as i32) != 0 {
        let mut nv = LHS_assign {
            prev: 0 as *mut LHS_assign,
            v: expdesc {
                k: VVOID,
                u: C2RustUnnamed_8 { ival: 0 },
                t: 0,
                f: 0,
            },
        };
        nv.prev = lh;
        suffixedexp(ls, &mut nv.v);
        if nv.v.k as libc::c_uint != VINDEXED as libc::c_int as libc::c_uint {
            check_conflict(ls, lh, &mut nv.v);
        }
        checklimit(
            (*ls).fs,
            nvars + (*(*ls).L).nCcalls as libc::c_int,
            LUAI_MAXCCALLS,
            b"C levels\0" as *const u8 as *const libc::c_char,
        );
        assignment(ls, &mut nv, nvars + 1 as libc::c_int);
    } else {
        let mut nexps: libc::c_int = 0;
        checknext(ls, '=' as i32);
        nexps = explist(ls, &mut e);
        if nexps != nvars {
            adjust_assign(ls, nvars, nexps, &mut e);
        } else {
            luaK_setoneret((*ls).fs, &mut e);
            luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
            return;
        }
    }
    init_exp(&mut e, VNONRELOC, (*(*ls).fs).freereg as libc::c_int - 1 as libc::c_int);
    luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
}
unsafe extern "C" fn cond(mut ls: *mut LexState) -> libc::c_int {
    let mut v = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    expr(ls, &mut v);
    if v.k as libc::c_uint == VNIL as libc::c_int as libc::c_uint {
        v.k = VFALSE;
    }
    luaK_goiftrue((*ls).fs, &mut v);
    return v.f;
}
unsafe extern "C" fn gotostat(mut ls: *mut LexState, mut pc: libc::c_int) {
    let mut line = (*ls).linenumber;
    let mut label = 0 as *mut TString;
    let mut g: libc::c_int = 0;
    if testnext(ls, TK_GOTO as libc::c_int) != 0 {
        label = str_checkname(ls);
    } else {
        luaX_next(ls);
        label = luaS_new((*ls).L, b"break\0" as *const u8 as *const libc::c_char);
    }
    g = newlabelentry(ls, &mut (*(*ls).dyd).gt, label, line, pc);
    findlabel(ls, g);
}
unsafe extern "C" fn checkrepeated(
    mut fs: *mut FuncState,
    mut ll: *mut Labellist,
    mut label: *mut TString,
) {
    let mut i: libc::c_int = 0;
    i = (*(*fs).bl).firstlabel;
    while i < (*ll).n {
        if eqstr!(label, ll -> arr[i].name) != 0 {
            let mut msg = luaO_pushfstring(
                (*(*fs).ls).L,
                b"label '%s' already defined on line %d\0" as *const u8
                    as *const libc::c_char,
                getstr!(label),
                (*((*ll).arr).offset(i as isize)).line,
            );
            semerror((*fs).ls, msg);
        }
        i += 1;
    }
}
unsafe extern "C" fn skipnoopstat(mut ls: *mut LexState) {
    while (*ls).t.token == ';' as i32 || (*ls).t.token == TK_DBCOLON as libc::c_int {
        statement(ls);
    }
}
unsafe extern "C" fn labelstat(
    mut ls: *mut LexState,
    mut label: *mut TString,
    mut line: libc::c_int,
) {
    let mut fs = (*ls).fs;
    let mut ll: *mut Labellist = &mut (*(*ls).dyd).label;
    let mut l: libc::c_int = 0;
    checkrepeated(fs, ll, label);
    checknext(ls, TK_DBCOLON as libc::c_int);
    l = newlabelentry(ls, ll, label, line, luaK_getlabel(fs));
    skipnoopstat(ls);
    if block_follow(ls, 0 as libc::c_int) != 0 {
        (*((*ll).arr).offset(l as isize)).nactvar = (*(*fs).bl).nactvar;
    }
    findgotos(ls, &mut *((*ll).arr).offset(l as isize));
}
unsafe extern "C" fn whilestat(mut ls: *mut LexState, mut line: libc::c_int) {
    let mut fs = (*ls).fs;
    let mut whileinit: libc::c_int = 0;
    let mut condexit: libc::c_int = 0;
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    luaX_next(ls);
    whileinit = luaK_getlabel(fs);
    condexit = cond(ls);
    enterblock(fs, &mut bl, 1 as libc::c_int as lu_byte);
    checknext(ls, TK_DO as libc::c_int);
    block(ls);
    luaK_jumpto!(fs, whileinit)(fs, luaK_jumpto!(fs, whileinit), whileinit);
    check_match(ls, TK_END as libc::c_int, TK_WHILE as libc::c_int, line);
    leaveblock(fs);
    luaK_patchtohere(fs, condexit);
}
unsafe extern "C" fn repeatstat(mut ls: *mut LexState, mut line: libc::c_int) {
    let mut condexit: libc::c_int = 0;
    let mut fs = (*ls).fs;
    let mut repeat_init = luaK_getlabel(fs);
    let mut bl1 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut bl2 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    enterblock(fs, &mut bl1, 1 as libc::c_int as lu_byte);
    enterblock(fs, &mut bl2, 0 as libc::c_int as lu_byte);
    luaX_next(ls);
    statlist(ls);
    check_match(ls, TK_UNTIL as libc::c_int, TK_REPEAT as libc::c_int, line);
    condexit = cond(ls);
    if bl2.upval != 0 {
        luaK_patchclose(fs, condexit, bl2.nactvar as libc::c_int);
    }
    leaveblock(fs);
    luaK_patchlist(fs, condexit, repeat_init);
    leaveblock(fs);
}
unsafe extern "C" fn exp1(mut ls: *mut LexState) -> libc::c_int {
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut reg: libc::c_int = 0;
    expr(ls, &mut e);
    luaK_exp2nextreg((*ls).fs, &mut e);
    reg = e.u.info;
    return reg;
}
unsafe extern "C" fn forbody(
    mut ls: *mut LexState,
    mut base: libc::c_int,
    mut line: libc::c_int,
    mut nvars: libc::c_int,
    mut isnum: libc::c_int,
) {
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut fs = (*ls).fs;
    let mut prep: libc::c_int = 0;
    let mut endfor: libc::c_int = 0;
    adjustlocalvars(ls, 3 as libc::c_int);
    checknext(ls, TK_DO as libc::c_int);
    prep = if isnum != 0 {
        luaK_codeAsBx!(fs, OP_FORPREP, base, NO_JUMP)
    } else {
        luaK_jump(fs)
    };
    enterblock(fs, &mut bl, 0 as libc::c_int as lu_byte);
    adjustlocalvars(ls, nvars);
    luaK_reserveregs(fs, nvars);
    block(ls);
    leaveblock(fs);
    luaK_patchtohere(fs, prep);
    if isnum != 0 {
        endfor = luaK_codeAsBx!(fs, OP_FORLOOP, base, NO_JUMP);
    } else {
        luaK_codeABC(fs, OP_TFORCALL, base, 0 as libc::c_int, nvars);
        luaK_fixline(fs, line);
        endfor = luaK_codeAsBx!(fs, OP_TFORLOOP, base + 2, NO_JUMP);
    }
    luaK_patchlist(fs, endfor, prep + 1 as libc::c_int);
    luaK_fixline(fs, line);
}
unsafe extern "C" fn fornum(
    mut ls: *mut LexState,
    mut varname: *mut TString,
    mut line: libc::c_int,
) {
    let mut fs = (*ls).fs;
    let mut base = (*fs).freereg as libc::c_int;
    new_localvarliteral!(
        ls, "(for index)"
    )(
        ls,
        b"(for index)\0" as *const u8 as *const libc::c_char,
        new_localvarliteral!(ls, "(for index)"),
    );
    new_localvarliteral!(
        ls, "(for limit)"
    )(
        ls,
        b"(for limit)\0" as *const u8 as *const libc::c_char,
        new_localvarliteral!(ls, "(for limit)"),
    );
    new_localvarliteral!(
        ls, "(for step)"
    )(
        ls,
        b"(for step)\0" as *const u8 as *const libc::c_char,
        new_localvarliteral!(ls, "(for step)"),
    );
    new_localvar(ls, varname);
    checknext(ls, '=' as i32);
    exp1(ls);
    checknext(ls, ',' as i32);
    exp1(ls);
    if testnext(ls, ',' as i32) != 0 {
        exp1(ls);
    } else {
        luaK_codek(
            fs,
            (*fs).freereg as libc::c_int,
            luaK_intK(fs, 1 as libc::c_int as lua_Integer),
        );
        luaK_reserveregs(fs, 1 as libc::c_int);
    }
    forbody(ls, base, line, 1 as libc::c_int, 1 as libc::c_int);
}
unsafe extern "C" fn forlist(mut ls: *mut LexState, mut indexname: *mut TString) {
    let mut fs = (*ls).fs;
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut nvars = 4 as libc::c_int;
    let mut line: libc::c_int = 0;
    let mut base = (*fs).freereg as libc::c_int;
    new_localvarliteral!(
        ls, "(for generator)"
    )(
        ls,
        b"(for generator)\0" as *const u8 as *const libc::c_char,
        new_localvarliteral!(ls, "(for generator)"),
    );
    new_localvarliteral!(
        ls, "(for state)"
    )(
        ls,
        b"(for state)\0" as *const u8 as *const libc::c_char,
        new_localvarliteral!(ls, "(for state)"),
    );
    new_localvarliteral!(
        ls, "(for control)"
    )(
        ls,
        b"(for control)\0" as *const u8 as *const libc::c_char,
        new_localvarliteral!(ls, "(for control)"),
    );
    new_localvar(ls, indexname);
    while testnext(ls, ',' as i32) != 0 {
        new_localvar(ls, str_checkname(ls));
        nvars += 1;
    }
    checknext(ls, TK_IN as libc::c_int);
    line = (*ls).linenumber;
    adjust_assign(ls, 3 as libc::c_int, explist(ls, &mut e), &mut e);
    luaK_checkstack(fs, 3 as libc::c_int);
    forbody(ls, base, line, nvars - 3 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn forstat(mut ls: *mut LexState, mut line: libc::c_int) {
    let mut fs = (*ls).fs;
    let mut varname = 0 as *mut TString;
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    enterblock(fs, &mut bl, 1 as libc::c_int as lu_byte);
    luaX_next(ls);
    varname = str_checkname(ls);
    match (*ls).t.token {
        61 => {
            fornum(ls, varname, line);
        }
        44 | 268 => {
            forlist(ls, varname);
        }
        _ => {
            luaX_syntaxerror(
                ls,
                b"'=' or 'in' expected\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    check_match(ls, TK_END as libc::c_int, TK_FOR as libc::c_int, line);
    leaveblock(fs);
}
unsafe extern "C" fn test_then_block(
    mut ls: *mut LexState,
    mut escapelist: *mut libc::c_int,
) {
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut fs = (*ls).fs;
    let mut v = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut jf: libc::c_int = 0;
    luaX_next(ls);
    expr(ls, &mut v);
    checknext(ls, TK_THEN as libc::c_int);
    if (*ls).t.token == TK_GOTO as libc::c_int
        || (*ls).t.token == TK_BREAK as libc::c_int
    {
        luaK_goiffalse((*ls).fs, &mut v);
        enterblock(fs, &mut bl, 0 as libc::c_int as lu_byte);
        gotostat(ls, v.t);
        while testnext(ls, ';' as i32) != 0 {}
        if block_follow(ls, 0 as libc::c_int) != 0 {
            leaveblock(fs);
            return;
        } else {
            jf = luaK_jump(fs);
        }
    } else {
        luaK_goiftrue((*ls).fs, &mut v);
        enterblock(fs, &mut bl, 0 as libc::c_int as lu_byte);
        jf = v.f;
    }
    statlist(ls);
    leaveblock(fs);
    if (*ls).t.token == TK_ELSE as libc::c_int
        || (*ls).t.token == TK_ELSEIF as libc::c_int
    {
        luaK_concat(fs, escapelist, luaK_jump(fs));
    }
    luaK_patchtohere(fs, jf);
}
unsafe extern "C" fn ifstat(mut ls: *mut LexState, mut line: libc::c_int) {
    let mut fs = (*ls).fs;
    let mut escapelist = NO_JUMP;
    test_then_block(ls, &mut escapelist);
    while (*ls).t.token == TK_ELSEIF as libc::c_int {
        test_then_block(ls, &mut escapelist);
    }
    if testnext(ls, TK_ELSE as libc::c_int) != 0 {
        block(ls);
    }
    check_match(ls, TK_END as libc::c_int, TK_IF as libc::c_int, line);
    luaK_patchtohere(fs, escapelist);
}
unsafe extern "C" fn localfunc(mut ls: *mut LexState) {
    let mut b = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut fs = (*ls).fs;
    new_localvar(ls, str_checkname(ls));
    adjustlocalvars(ls, 1 as libc::c_int);
    body(ls, &mut b, 0 as libc::c_int, (*ls).linenumber);
    (*getlocvar(fs, b.u.info)).startpc = (*fs).pc;
}
unsafe extern "C" fn localstat(mut ls: *mut LexState) {
    let mut nvars = 0 as libc::c_int;
    let mut nexps: libc::c_int = 0;
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    loop {
        new_localvar(ls, str_checkname(ls));
        nvars += 1;
        if !(testnext(ls, ',' as i32) != 0) {
            break;
        }
    }
    if testnext(ls, '=' as i32) != 0 {
        nexps = explist(ls, &mut e);
    } else {
        e.k = VVOID;
        nexps = 0 as libc::c_int;
    }
    adjust_assign(ls, nvars, nexps, &mut e);
    adjustlocalvars(ls, nvars);
}
unsafe extern "C" fn funcname(
    mut ls: *mut LexState,
    mut v: *mut expdesc,
) -> libc::c_int {
    let mut ismethod = 0 as libc::c_int;
    singlevar(ls, v);
    while (*ls).t.token == '.' as i32 {
        fieldsel(ls, v);
    }
    if (*ls).t.token == ':' as i32 {
        ismethod = 1 as libc::c_int;
        fieldsel(ls, v);
    }
    return ismethod;
}
unsafe extern "C" fn funcstat(mut ls: *mut LexState, mut line: libc::c_int) {
    let mut ismethod: libc::c_int = 0;
    let mut v = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut b = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    luaX_next(ls);
    ismethod = funcname(ls, &mut v);
    body(ls, &mut b, ismethod, line);
    luaK_storevar((*ls).fs, &mut v, &mut b);
    luaK_fixline((*ls).fs, line);
}
unsafe extern "C" fn exprstat(mut ls: *mut LexState) {
    let mut fs = (*ls).fs;
    let mut v = LHS_assign {
        prev: 0 as *mut LHS_assign,
        v: expdesc {
            k: VVOID,
            u: C2RustUnnamed_8 { ival: 0 },
            t: 0,
            f: 0,
        },
    };
    suffixedexp(ls, &mut v.v);
    if (*ls).t.token == '=' as i32 || (*ls).t.token == ',' as i32 {
        v.prev = NULL as *mut LHS_assign;
        assignment(ls, &mut v, 1 as libc::c_int);
    } else {
        if check_condition!(ls, v.v.k == VCALL, "syntax error") == 0 {
            check_condition!(
                ls, v.v.k == VCALL, "syntax error"
            )(ls, b"syntax error\0" as *const u8 as *const libc::c_char);
        }
        let ref mut fresh178 = SETARG_C!(getinstruction(fs, & v.v), 1);
        *fresh178 = SETARG_C!(getinstruction(fs, & v.v), 1);
    };
}
unsafe extern "C" fn retstat(mut ls: *mut LexState) {
    let mut fs = (*ls).fs;
    let mut e = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut first: libc::c_int = 0;
    let mut nret: libc::c_int = 0;
    if block_follow(ls, 1 as libc::c_int) != 0 || (*ls).t.token == ';' as i32 {
        nret = 0 as libc::c_int;
        first = nret;
    } else {
        nret = explist(ls, &mut e);
        if hasmultret!(e.k) != 0 {
            luaK_setmultret!(fs, & e)(fs, &mut e, LUA_MULTRET);
            if e.k as libc::c_uint == VCALL as libc::c_int as libc::c_uint
                && nret == 1 as libc::c_int
            {
                let ref mut fresh179 = SET_OPCODE!(getinstruction(fs,& e), OP_TAILCALL);
                *fresh179 = SET_OPCODE!(getinstruction(fs,& e), OP_TAILCALL);
            }
            first = (*fs).nactvar as libc::c_int;
            nret = LUA_MULTRET;
        } else if nret == 1 as libc::c_int {
            first = luaK_exp2anyreg(fs, &mut e);
        } else {
            luaK_exp2nextreg(fs, &mut e);
            first = (*fs).nactvar as libc::c_int;
        }
    }
    luaK_ret(fs, first, nret);
    testnext(ls, ';' as i32);
}
unsafe extern "C" fn statement(mut ls: *mut LexState) {
    let mut line = (*ls).linenumber;
    enterlevel(ls);
    match (*ls).t.token {
        59 => {
            luaX_next(ls);
        }
        267 => {
            ifstat(ls, line);
        }
        278 => {
            whilestat(ls, line);
        }
        259 => {
            luaX_next(ls);
            block(ls);
            check_match(ls, TK_END as libc::c_int, TK_DO as libc::c_int, line);
        }
        264 => {
            forstat(ls, line);
        }
        273 => {
            repeatstat(ls, line);
        }
        265 => {
            funcstat(ls, line);
        }
        269 => {
            luaX_next(ls);
            if testnext(ls, TK_FUNCTION as libc::c_int) != 0 {
                localfunc(ls);
            } else {
                localstat(ls);
            }
        }
        288 => {
            luaX_next(ls);
            labelstat(ls, str_checkname(ls), line);
        }
        274 => {
            luaX_next(ls);
            retstat(ls);
        }
        258 | 266 => {
            gotostat(ls, luaK_jump((*ls).fs));
        }
        _ => {
            exprstat(ls);
        }
    }
    (*(*ls).fs).freereg = (*(*ls).fs).nactvar;
    let ref mut fresh180 = leavelevel!(ls);
    *fresh180 = (*fresh180).wrapping_sub(1);
}
unsafe extern "C" fn mainfunc(mut ls: *mut LexState, mut fs: *mut FuncState) {
    let mut bl = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut v = expdesc {
        k: VVOID,
        u: C2RustUnnamed_8 { ival: 0 },
        t: 0,
        f: 0,
    };
    open_func(ls, fs, &mut bl);
    (*(*fs).f).is_vararg = 1 as libc::c_int as lu_byte;
    init_exp(&mut v, VLOCAL, 0 as libc::c_int);
    newupvalue(fs, (*ls).envn, &mut v);
    if luaC_objbarrier!(ls -> L, fs -> f, ls -> envn) != 0 {} else {};
    luaX_next(ls);
    statlist(ls);
    check(ls, TK_EOS as libc::c_int);
    close_func(ls);
}
#[no_mangle]
pub unsafe extern "C" fn luaY_parser(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut buff: *mut Mbuffer,
    mut dyd: *mut Dyndata,
    mut name: *const libc::c_char,
    mut firstchar: libc::c_int,
) -> *mut LClosure {
    let mut lexstate = LexState {
        current: 0,
        linenumber: 0,
        lastline: 0,
        t: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        lookahead: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        fs: 0 as *mut FuncState,
        L: 0 as *mut lua_State,
        z: 0 as *mut ZIO,
        buff: 0 as *mut Mbuffer,
        h: 0 as *mut Table,
        dyd: 0 as *mut Dyndata,
        source: 0 as *mut TString,
        envn: 0 as *mut TString,
    };
    let mut funcstate = FuncState {
        f: 0 as *mut Proto,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        jpc: 0,
        nk: 0,
        np: 0,
        firstlocal: 0,
        nlocvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
    };
    let mut cl = luaF_newLclosure(L, 1 as libc::c_int);
    let mut io = setclLvalue!(L, L -> top, cl);
    let mut x_ = setclLvalue!(L, L -> top, cl);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int;
    luaD_inctop(L);
    lexstate.h = luaH_new(L);
    let mut io_0 = sethvalue!(L, L -> top, lexstate.h);
    let mut x__0 = sethvalue!(L, L -> top, lexstate.h);
    (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
    (*io_0).tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    funcstate.f = (*cl).p;
    if luaC_objbarrier!(L, cl, cl -> p) != 0 {} else {};
    (*funcstate.f).source = luaS_new(L, name);
    lexstate.buff = buff;
    lexstate.dyd = dyd;
    (*dyd).label.n = 0 as libc::c_int;
    (*dyd).gt.n = (*dyd).label.n;
    (*dyd).actvar.n = (*dyd).gt.n;
    luaX_setinput(L, &mut lexstate, z, (*funcstate.f).source, firstchar);
    mainfunc(&mut lexstate, &mut funcstate);
    (*L).top = ((*L).top).offset(-1);
    return cl;
}
unsafe extern "C" fn currentpc(mut ci: *mut CallInfo) -> libc::c_int {
    return pcRel!(ci -> u.l.savedpc, ci_func(ci) -> p);
}
unsafe extern "C" fn currentline(mut ci: *mut CallInfo) -> libc::c_int {
    return if !((*(*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p).lineinfo)
        .is_null()
    {
        *((*(*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p).lineinfo)
            .offset(currentpc(ci) as isize)
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn swapextra(mut L: *mut lua_State) {
    if (*L).status as libc::c_int == LUA_YIELD {
        let mut ci = (*L).ci;
        let mut temp = (*ci).func;
        (*ci).func = restorestack!(L, ci -> extra);
        (*ci).extra = savestack!(L, temp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_sethook(
    mut L: *mut lua_State,
    mut func: lua_Hook,
    mut mask: libc::c_int,
    mut count: libc::c_int,
) {
    if func.is_none() || mask == 0 as libc::c_int {
        mask = 0 as libc::c_int;
        func = ::core::mem::transmute::<
            libc::intptr_t,
            lua_Hook,
        >(NULL as libc::intptr_t);
    }
    if isLua!(L -> ci) != 0 {
        (*L).oldpc = (*(*L).ci).u.l.savedpc;
    }
    ::core::ptr::write_volatile(&mut (*L).hook as *mut lua_Hook, func);
    (*L).basehookcount = count;
    (*L).hookcount = (*L).basehookcount;
    (*L).hookmask = cast_byte!(mask);
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethook(mut L: *mut lua_State) -> lua_Hook {
    return (*L).hook;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethookmask(mut L: *mut lua_State) -> libc::c_int {
    return (*L).hookmask;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethookcount(mut L: *mut lua_State) -> libc::c_int {
    return (*L).basehookcount;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getstack(
    mut L: *mut lua_State,
    mut level: libc::c_int,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut ci = 0 as *mut CallInfo;
    if level < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ci = (*L).ci;
    while level > 0 as libc::c_int && ci != &mut (*L).base_ci as *mut CallInfo {
        level -= 1;
        ci = (*ci).previous;
    }
    if level == 0 as libc::c_int && ci != &mut (*L).base_ci as *mut CallInfo {
        status = 1 as libc::c_int;
        (*ar).i_ci = ci;
    } else {
        status = 0 as libc::c_int;
    }
    return status;
}
unsafe extern "C" fn upvalname(
    mut p: *mut Proto,
    mut uv: libc::c_int,
) -> *const libc::c_char {
    let mut s = check_exp!(uv < p -> sizeupvalues, p -> upvalues[uv].name);
    if s.is_null() {
        return b"?\0" as *const u8 as *const libc::c_char
    } else {
        return getstr!(s)
    };
}
unsafe extern "C" fn findvararg(
    mut ci: *mut CallInfo,
    mut n: libc::c_int,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    let mut nparams = (*(*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p).numparams
        as libc::c_int;
    let mut nvararg = cast_int!(ci -> u.l.base - ci -> func) - nparams;
    if n <= -nvararg {
        return NULL as *const libc::c_char
    } else {
        *pos = ((*ci).func).offset(nparams as isize).offset(-(n as isize));
        return b"(*vararg)\0" as *const u8 as *const libc::c_char;
    };
}
unsafe extern "C" fn findlocal(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut n: libc::c_int,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    let mut name = NULL as *const libc::c_char;
    let mut base = 0 as *mut TValue;
    if isLua!(ci) != 0 {
        if n < 0 as libc::c_int {
            return findvararg(ci, n, pos)
        } else {
            base = (*ci).u.l.base;
            name = luaF_getlocalname(
                (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p,
                n,
                currentpc(ci),
            );
        }
    } else {
        base = ((*ci).func).offset(1 as libc::c_int as isize);
    }
    if name.is_null() {
        let mut limit = if ci == (*L).ci { (*L).top } else { (*(*ci).next).func };
        if limit.offset_from(base) as libc::c_long >= n as libc::c_long
            && n > 0 as libc::c_int
        {
            name = b"(*temporary)\0" as *const u8 as *const libc::c_char;
        } else {
            return NULL as *const libc::c_char
        }
    }
    *pos = base.offset((n - 1 as libc::c_int) as isize);
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getlocal(
    mut L: *mut lua_State,
    mut ar: *const lua_Debug,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name = 0 as *const libc::c_char;
    swapextra(L);
    if ar.is_null() {
        if isLfunction!(L -> top - 1) == 0 {
            name = NULL as *const libc::c_char;
        } else {
            name = luaF_getlocalname(
                (*((*((*L).top).offset(-(1 as libc::c_int as isize))).value_.gc
                    as *mut GCUnion))
                    .cl
                    .l
                    .p,
                n,
                0 as libc::c_int,
            );
        }
    } else {
        let mut pos = NULL as StkId;
        name = findlocal(L, (*ar).i_ci, n, &mut pos);
        if !name.is_null() {
            let mut io1 = setobj2s!(L, L -> top, pos);
            let ref mut fresh181 = setobj2s!(L, L -> top, pos);
            *fresh181 = setobj2s!(L, L -> top, pos);
            (*L).top = ((*L).top).offset(1);
            if (*L).top <= (*(*L).ci).top
                && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
            {} else {
                __assert_fail(
                    b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                        as *const libc::c_char,
                    b"./ldebug.c\0" as *const u8 as *const libc::c_char,
                    187 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    swapextra(L);
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setlocal(
    mut L: *mut lua_State,
    mut ar: *const lua_Debug,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut pos = NULL as StkId;
    let mut name = 0 as *const libc::c_char;
    swapextra(L);
    name = findlocal(L, (*ar).i_ci, n, &mut pos);
    if !name.is_null() {
        let mut io1 = setobjs2s!(L, pos, L -> top - 1);
        let ref mut fresh182 = setobjs2s!(L, pos, L -> top - 1);
        *fresh182 = setobjs2s!(L, pos, L -> top - 1);
        (*L).top = ((*L).top).offset(-1);
    }
    swapextra(L);
    return name;
}
unsafe extern "C" fn funcinfo(mut ar: *mut lua_Debug, mut cl: *mut Closure) {
    if noLuaClosure!(cl) != 0 {
        (*ar).source = b"=[C]\0" as *const u8 as *const libc::c_char;
        (*ar).linedefined = -(1 as libc::c_int);
        (*ar).lastlinedefined = -(1 as libc::c_int);
        (*ar).what = b"C\0" as *const u8 as *const libc::c_char;
    } else {
        let mut p = (*cl).l.p;
        (*ar)
            .source = if !((*p).source).is_null() {
            getstr!(p -> source)
        } else {
            b"=?\0" as *const u8 as *const libc::c_char
        };
        (*ar).linedefined = (*p).linedefined;
        (*ar).lastlinedefined = (*p).lastlinedefined;
        (*ar)
            .what = if (*ar).linedefined == 0 as libc::c_int {
            b"main\0" as *const u8 as *const libc::c_char
        } else {
            b"Lua\0" as *const u8 as *const libc::c_char
        };
    }
    luaO_chunkid(((*ar).short_src).as_mut_ptr(), (*ar).source, LUA_IDSIZE as size_t);
}
unsafe extern "C" fn collectvalidlines(mut L: *mut lua_State, mut f: *mut Closure) {
    if noLuaClosure!(f) != 0 {
        let ref mut fresh183 = setnilvalue!(L -> top);
        *fresh183 = setnilvalue!(L -> top);
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./ldebug.c\0" as *const u8 as *const libc::c_char,
                233 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"void collectvalidlines(lua_State *, Closure *)\0"))
                    .as_ptr(),
            );
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut v = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut lineinfo = (*(*f).l.p).lineinfo;
        let mut t = luaH_new(L);
        let mut io = sethvalue!(L, L -> top, t);
        let mut x_ = sethvalue!(L, L -> top, t);
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./ldebug.c\0" as *const u8 as *const libc::c_char,
                241 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"void collectvalidlines(lua_State *, Closure *)\0"))
                    .as_ptr(),
            );
        }
        let mut io_0: *mut TValue = setbvalue!(& v, 1);
        (*io_0).value_.b = setbvalue!(& v, 1);
        (*io_0).tt_ = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*(*f).l.p).sizelineinfo {
            luaH_setint(L, t, *lineinfo.offset(i as isize) as lua_Integer, &mut v);
            i += 1;
        }
    };
}
unsafe extern "C" fn getfuncname(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    if ci.is_null() {
        return NULL as *const libc::c_char
    } else if (*ci).callstatus as libc::c_int & CIST_FIN != 0 {
        *name = b"__gc\0" as *const u8 as *const libc::c_char;
        return b"metamethod\0" as *const u8 as *const libc::c_char;
    } else if (*ci).callstatus as libc::c_int & CIST_TAIL == 0
        && isLua!(ci -> previous) != 0
    {
        return funcnamefromcode(L, (*ci).previous, name)
    } else {
        return NULL as *const libc::c_char
    };
}
unsafe extern "C" fn auxgetinfo(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
    mut f: *mut Closure,
    mut ci: *mut CallInfo,
) -> libc::c_int {
    let mut status = 1 as libc::c_int;
    while *what != 0 {
        match *what as libc::c_int {
            83 => {
                funcinfo(ar, f);
            }
            108 => {
                (*ar)
                    .currentline = if !ci.is_null() && isLua!(ci) != 0 {
                    currentline(ci)
                } else {
                    -(1 as libc::c_int)
                };
            }
            117 => {
                (*ar)
                    .nups = (if f.is_null() {
                    0 as libc::c_int
                } else {
                    (*f).c.nupvalues as libc::c_int
                }) as libc::c_uchar;
                if noLuaClosure!(f) != 0 {
                    (*ar).isvararg = 1 as libc::c_int as libc::c_char;
                    (*ar).nparams = 0 as libc::c_int as libc::c_uchar;
                } else {
                    (*ar).isvararg = (*(*f).l.p).is_vararg as libc::c_char;
                    (*ar).nparams = (*(*f).l.p).numparams;
                }
            }
            116 => {
                (*ar)
                    .istailcall = (if !ci.is_null() {
                    (*ci).callstatus as libc::c_int & CIST_TAIL
                } else {
                    0 as libc::c_int
                }) as libc::c_char;
            }
            110 => {
                (*ar).namewhat = getfuncname(L, ci, &mut (*ar).name);
                if ((*ar).namewhat).is_null() {
                    (*ar).namewhat = b"\0" as *const u8 as *const libc::c_char;
                    (*ar).name = NULL as *const libc::c_char;
                }
            }
            76 | 102 => {}
            _ => {
                status = 0 as libc::c_int;
            }
        }
        what = what.offset(1);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getinfo(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut cl = 0 as *mut Closure;
    let mut ci = 0 as *mut CallInfo;
    let mut func = 0 as *mut TValue;
    swapextra(L);
    if *what as libc::c_int == '>' as i32 {
        ci = NULL as *mut CallInfo;
        func = ((*L).top).offset(-(1 as libc::c_int as isize));
        what = what.offset(1);
        (*L).top = ((*L).top).offset(-1);
    } else {
        ci = (*ar).i_ci;
        func = (*ci).func;
    }
    cl = if ttisclosure!(func) != 0 { clvalue!(func) } else { NULL as *mut Closure };
    status = auxgetinfo(L, what, ar, cl, ci);
    if !(strchr(what, 'f' as i32)).is_null() {
        let mut io1 = setobjs2s!(L, L -> top, func);
        let ref mut fresh184 = setobjs2s!(L, L -> top, func);
        *fresh184 = setobjs2s!(L, L -> top, func);
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./ldebug.c\0" as *const u8 as *const libc::c_char,
                333 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\0"))
                    .as_ptr(),
            );
        }
    }
    swapextra(L);
    if !(strchr(what, 'L' as i32)).is_null() {
        collectvalidlines(L, cl);
    }
    return status;
}
unsafe extern "C" fn kname(
    mut p: *mut Proto,
    mut pc: libc::c_int,
    mut c: libc::c_int,
    mut name: *mut *const libc::c_char,
) {
    if ISK!(c) != 0 {
        let mut kvalue: *mut TValue = &mut *((*p).k).offset(INDEXK!(c) as isize)
            as *mut TValue;
        if ttisstring!(kvalue) != 0 {
            *name = svalue!(kvalue);
            return;
        }
    } else {
        let mut what = getobjname(p, pc, c, name);
        if !what.is_null() && *what as libc::c_int == 'c' as i32 {
            return;
        }
    }
    *name = b"?\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn filterpc(
    mut pc: libc::c_int,
    mut jmptarget: libc::c_int,
) -> libc::c_int {
    if pc < jmptarget { return -(1 as libc::c_int) } else { return pc };
}
unsafe extern "C" fn findsetreg(
    mut p: *mut Proto,
    mut lastpc: libc::c_int,
    mut reg: libc::c_int,
) -> libc::c_int {
    let mut pc: libc::c_int = 0;
    let mut setreg = -(1 as libc::c_int);
    let mut jmptarget = 0 as libc::c_int;
    pc = 0 as libc::c_int;
    while pc < lastpc {
        let mut i = *((*p).code).offset(pc as isize);
        let mut op = GET_OPCODE!(i);
        let mut a = GETARG_A!(i);
        match op as libc::c_uint {
            4 => {
                let mut b = GETARG_B!(i);
                if a <= reg && reg <= a + b {
                    setreg = filterpc(pc, jmptarget);
                }
            }
            41 => {
                if reg >= a + 2 as libc::c_int {
                    setreg = filterpc(pc, jmptarget);
                }
            }
            36 | 37 => {
                if reg >= a {
                    setreg = filterpc(pc, jmptarget);
                }
            }
            30 => {
                let mut b_0 = GETARG_sBx!(i);
                let mut dest = pc + 1 as libc::c_int + b_0;
                if pc < dest && dest <= lastpc {
                    if dest > jmptarget {
                        jmptarget = dest;
                    }
                }
            }
            _ => {
                if testAMode!(op) != 0 && reg == a {
                    setreg = filterpc(pc, jmptarget);
                }
            }
        }
        pc += 1;
    }
    return setreg;
}
unsafe extern "C" fn getobjname(
    mut p: *mut Proto,
    mut lastpc: libc::c_int,
    mut reg: libc::c_int,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut pc: libc::c_int = 0;
    *name = luaF_getlocalname(p, reg + 1 as libc::c_int, lastpc);
    if !(*name).is_null() {
        return b"local\0" as *const u8 as *const libc::c_char;
    }
    pc = findsetreg(p, lastpc, reg);
    if pc != -(1 as libc::c_int) {
        let mut i = *((*p).code).offset(pc as isize);
        let mut op = GET_OPCODE!(i);
        match op as libc::c_uint {
            0 => {
                let mut b = GETARG_B!(i);
                if b < GETARG_A!(i) {
                    return getobjname(p, pc, b, name);
                }
            }
            6 | 7 => {
                let mut k = GETARG_C!(i);
                let mut t = GETARG_B!(i);
                let mut vn = if op as libc::c_uint
                    == OP_GETTABLE as libc::c_int as libc::c_uint
                {
                    luaF_getlocalname(p, t + 1 as libc::c_int, pc)
                } else {
                    upvalname(p, t)
                };
                kname(p, pc, k, name);
                return if !vn.is_null()
                    && strcmp(vn, LUA_ENV.as_ptr()) == 0 as libc::c_int
                {
                    b"global\0" as *const u8 as *const libc::c_char
                } else {
                    b"field\0" as *const u8 as *const libc::c_char
                };
            }
            5 => {
                *name = upvalname(p, GETARG_B!(i));
                return b"upvalue\0" as *const u8 as *const libc::c_char;
            }
            1 | 2 => {
                let mut b_0 = if op as libc::c_uint
                    == OP_LOADK as libc::c_int as libc::c_uint
                {
                    GETARG_Bx!(i)
                } else {
                    GETARG_Ax!(p -> code[pc + 1])
                };
                if ttisstring!(& p -> k[b]) != 0 {
                    *name = svalue!(& p -> k[b]);
                    return b"constant\0" as *const u8 as *const libc::c_char;
                }
            }
            12 => {
                let mut k_0 = GETARG_C!(i);
                kname(p, pc, k_0, name);
                return b"method\0" as *const u8 as *const libc::c_char;
            }
            _ => {}
        }
    }
    return NULL as *const libc::c_char;
}
unsafe extern "C" fn funcnamefromcode(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut tm = TM_INDEX;
    let mut p = (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p;
    let mut pc = currentpc(ci);
    let mut i = *((*p).code).offset(pc as isize);
    if (*ci).callstatus as libc::c_int & CIST_HOOKED != 0 {
        *name = b"?\0" as *const u8 as *const libc::c_char;
        return b"hook\0" as *const u8 as *const libc::c_char;
    }
    match GET_OPCODE!(i) {
        36 | 37 => return getobjname(p, pc, GETARG_A!(i), name),
        41 => {
            *name = b"for iterator\0" as *const u8 as *const libc::c_char;
            return b"for iterator\0" as *const u8 as *const libc::c_char;
        }
        12 | 6 | 7 => {
            tm = TM_INDEX;
        }
        8 | 10 => {
            tm = TM_NEWINDEX;
        }
        13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => {
            let mut offset = (i >> 0 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 6 as libc::c_int)
                    << 0 as libc::c_int) as OpCode as libc::c_int - cast_int!(OP_ADD);
            tm = (offset + TM_ADD as libc::c_int) as TMS;
        }
        25 => {
            tm = TM_UNM;
        }
        26 => {
            tm = TM_BNOT;
        }
        28 => {
            tm = TM_LEN;
        }
        29 => {
            tm = TM_CONCAT;
        }
        31 => {
            tm = TM_EQ;
        }
        32 => {
            tm = TM_LT;
        }
        33 => {
            tm = TM_LE;
        }
        _ => return NULL as *const libc::c_char,
    }
    *name = getstr!(G(L) -> tmname[tm]);
    return b"metamethod\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn isinstack(
    mut ci: *mut CallInfo,
    mut o: *const TValue,
) -> libc::c_int {
    let mut i = o.offset_from((*ci).u.l.base) as libc::c_long;
    return (0 as libc::c_int as libc::c_long <= i
        && i < ((*ci).top).offset_from((*ci).u.l.base) as libc::c_long
        && ((*ci).u.l.base).offset(i as isize) == o as StkId) as libc::c_int;
}
unsafe extern "C" fn getupvalname(
    mut ci: *mut CallInfo,
    mut o: *const TValue,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut c: *mut LClosure = ci_func!(ci);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*c).nupvalues as libc::c_int {
        if (**((*c).upvals).as_mut_ptr().offset(i as isize)).v == o as *mut TValue {
            *name = upvalname((*c).p, i);
            return b"upvalue\0" as *const u8 as *const libc::c_char;
        }
        i += 1;
    }
    return NULL as *const libc::c_char;
}
unsafe extern "C" fn varinfo(
    mut L: *mut lua_State,
    mut o: *const TValue,
) -> *const libc::c_char {
    let mut name = NULL as *const libc::c_char;
    let mut ci = (*L).ci;
    let mut kind = NULL as *const libc::c_char;
    if isLua!(ci) != 0 {
        kind = getupvalname(ci, o, &mut name);
        if kind.is_null() && isinstack(ci, o) != 0 {
            kind = getobjname(
                (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p,
                currentpc(ci),
                cast_int!(o - ci -> u.l.base),
                &mut name,
            );
        }
    }
    return if !kind.is_null() {
        luaO_pushfstring(
            L,
            b" (%s '%s')\0" as *const u8 as *const libc::c_char,
            kind,
            name,
        )
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaG_typeerror(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut op: *const libc::c_char,
) -> ! {
    let mut t = luaT_objtypename(L, o);
    luaG_runerror(
        L,
        b"attempt to %s a %s value%s\0" as *const u8 as *const libc::c_char,
        op,
        t,
        varinfo(L, o),
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaG_concaterror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    if ttisstring!(p1) != 0 || cvt2str!(p1) != 0 {
        p1 = p2;
    }
    luaG_typeerror(L, p1, b"concatenate\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_opinterror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut msg: *const libc::c_char,
) -> ! {
    let mut temp: lua_Number = 0.;
    if tonumber!(p1, & temp) == 0 {
        p2 = p1;
    }
    luaG_typeerror(L, p2, msg);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_tointerror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut temp: lua_Integer = 0;
    if tointeger!(p1, & temp) == 0 {
        p2 = p1;
    }
    luaG_runerror(
        L,
        b"number%s has no integer representation\0" as *const u8 as *const libc::c_char,
        varinfo(L, p2),
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaG_ordererror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut t1 = luaT_objtypename(L, p1);
    let mut t2 = luaT_objtypename(L, p2);
    if strcmp(t1, t2) == 0 as libc::c_int {
        luaG_runerror(
            L,
            b"attempt to compare two %s values\0" as *const u8 as *const libc::c_char,
            t1,
        );
    } else {
        luaG_runerror(
            L,
            b"attempt to compare %s with %s\0" as *const u8 as *const libc::c_char,
            t1,
            t2,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaG_addinfo(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut src: *mut TString,
    mut line: libc::c_int,
) -> *const libc::c_char {
    let mut buff: [libc::c_char; 60] = [0; 60];
    if !src.is_null() {
        luaO_chunkid(buff.as_mut_ptr(), getstr!(src), LUA_IDSIZE as size_t);
    } else {
        buff[0 as libc::c_int as usize] = '?' as i32 as libc::c_char;
        buff[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    return luaO_pushfstring(
        L,
        b"%s:%d: %s\0" as *const u8 as *const libc::c_char,
        buff.as_mut_ptr(),
        line,
        msg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaG_errormsg(mut L: *mut lua_State) -> ! {
    if (*L).errfunc != 0 as libc::c_int as libc::c_long {
        let mut errfunc = restorestack!(L, L -> errfunc);
        let mut io1 = setobjs2s!(L, L -> top, L -> top - 1);
        let ref mut fresh185 = setobjs2s!(L, L -> top, L -> top - 1);
        *fresh185 = setobjs2s!(L, L -> top, L -> top - 1);
        let mut io1_0 = setobjs2s!(L, L -> top - 1, errfunc);
        let ref mut fresh186 = setobjs2s!(L, L -> top - 1, errfunc);
        *fresh186 = setobjs2s!(L, L -> top - 1, errfunc);
        (*L).top = ((*L).top).offset(1);
        luaD_callnoyield(
            L,
            ((*L).top).offset(-(2 as libc::c_int as isize)),
            1 as libc::c_int,
        );
    }
    luaD_throw(L, LUA_ERRRUN);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_runerror(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ci = (*L).ci;
    let mut msg = 0 as *const libc::c_char;
    let mut argp: ::core::ffi::VaListImpl;
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
    }
    argp = args.clone();
    msg = luaO_pushvfstring(L, fmt, argp.as_va_list());
    if isLua!(ci) != 0 {
        luaG_addinfo(
            L,
            msg,
            (*(*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p).source,
            currentline(ci),
        );
    }
    luaG_errormsg(L);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_traceexec(mut L: *mut lua_State) {
    let mut ci = (*L).ci;
    let mut mask = (*L).hookmask as lu_byte;
    (*L).hookcount -= 1;
    let mut counthook = ((*L).hookcount == 0 as libc::c_int
        && mask as libc::c_int & LUA_MASKCOUNT != 0) as libc::c_int;
    if counthook != 0 {
        (*L).hookcount = (*L).basehookcount;
    } else if mask as libc::c_int & LUA_MASKLINE == 0 {
        return
    }
    if (*ci).callstatus as libc::c_int & CIST_HOOKYIELD != 0 {
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int & !CIST_HOOKYIELD)
            as libc::c_ushort;
        return;
    }
    if counthook != 0 {
        luaD_hook(L, LUA_HOOKCOUNT, -(1 as libc::c_int));
    }
    if mask as libc::c_int & LUA_MASKLINE != 0 {
        let mut p = (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p;
        let mut npc = pcRel!(ci -> u.l.savedpc, p);
        let mut newline = getfuncline!(p, npc);
        if npc == 0 as libc::c_int || (*ci).u.l.savedpc <= (*L).oldpc
            || newline
                != (if !((*p).lineinfo).is_null() {
                    *((*p).lineinfo)
                        .offset(
                            (((*L).oldpc).offset_from((*p).code) as libc::c_long
                                as libc::c_int - 1 as libc::c_int) as isize,
                        )
                } else {
                    -(1 as libc::c_int)
                })
        {
            luaD_hook(L, LUA_HOOKLINE, newline);
        }
    }
    (*L).oldpc = (*ci).u.l.savedpc;
    if (*L).status as libc::c_int == LUA_YIELD {
        if counthook != 0 {
            (*L).hookcount = 1 as libc::c_int;
        }
        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(-1);
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int | CIST_HOOKYIELD)
            as libc::c_ushort;
        (*ci).func = ((*L).top).offset(-(1 as libc::c_int as isize));
        luaD_throw(L, LUA_YIELD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newCclosure(
    mut L: *mut lua_State,
    mut n: libc::c_int,
) -> *mut CClosure {
    let mut o = luaC_newobj(L, LUA_TCCL, sizeCclosure!(n));
    let mut c: *mut CClosure = gco2ccl!(o);
    (*c).nupvalues = cast_byte!(n);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newLclosure(
    mut L: *mut lua_State,
    mut n: libc::c_int,
) -> *mut LClosure {
    let mut o = luaC_newobj(L, LUA_TLCL, sizeLclosure!(n));
    let mut c: *mut LClosure = gco2lcl!(o);
    (*c).p = NULL as *mut Proto;
    (*c).nupvalues = cast_byte!(n);
    loop {
        let fresh187 = n;
        n = n - 1;
        if !(fresh187 != 0) {
            break;
        }
        let ref mut fresh188 = *((*c).upvals).as_mut_ptr().offset(n as isize);
        *fresh188 = NULL as *mut UpVal;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_initupvals(mut L: *mut lua_State, mut cl: *mut LClosure) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cl).nupvalues as libc::c_int {
        let mut uv = luaM_new!(L, UpVal);
        (*uv).refcount = 1 as libc::c_int as lu_mem;
        (*uv).v = &mut (*uv).u.value;
        let ref mut fresh189 = setnilvalue!(uv -> v);
        *fresh189 = setnilvalue!(uv -> v);
        let ref mut fresh190 = *((*cl).upvals).as_mut_ptr().offset(i as isize);
        *fresh190 = uv;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaF_findupval(
    mut L: *mut lua_State,
    mut level: StkId,
) -> *mut UpVal {
    let mut pp: *mut *mut UpVal = &mut (*L).openupval;
    let mut p = 0 as *mut UpVal;
    let mut uv = 0 as *mut UpVal;
    while !(*pp).is_null()
        && {
            p = *pp;
            (*p).v >= level
        }
    {
        if (*p).v == level {
            return p;
        }
        pp = &mut (*p).u.open.next;
    }
    uv = luaM_new!(L, UpVal);
    (*uv).refcount = 0 as libc::c_int as lu_mem;
    (*uv).u.open.next = *pp;
    (*uv).u.open.touched = 1 as libc::c_int;
    *pp = uv;
    (*uv).v = level;
    if isintwups!(L) == 0 {
        (*L).twups = (*G!(L)).twups;
        let ref mut fresh191 = (*G!(L)).twups;
        *fresh191 = L;
    }
    return uv;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_close(mut L: *mut lua_State, mut level: StkId) {
    let mut uv = 0 as *mut UpVal;
    while !((*L).openupval).is_null()
        && {
            uv = (*L).openupval;
            (*uv).v >= level
        }
    {
        (*L).openupval = (*uv).u.open.next;
        if (*uv).refcount == 0 as libc::c_int as libc::c_ulong {
            luaM_free!(
                L, uv
            )(L, luaM_free!(L, uv), luaM_free!(L, uv), luaM_free!(L, uv));
        } else {
            let mut io1: *mut TValue = setobj!(L, & uv -> u.value, uv -> v);
            let ref mut fresh192 = setobj!(L, & uv -> u.value, uv -> v);
            *fresh192 = setobj!(L, & uv -> u.value, uv -> v);
            (*uv).v = &mut (*uv).u.value;
            if luaC_upvalbarrier!(L, uv) != 0 {} else {};
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newproto(mut L: *mut lua_State) -> *mut Proto {
    let mut o = luaC_newobj(
        L,
        LUA_TPROTO,
        ::core::mem::size_of::<Proto>() as libc::c_ulong,
    );
    let mut f: *mut Proto = gco2p!(o);
    (*f).k = NULL as *mut TValue;
    (*f).sizek = 0 as libc::c_int;
    (*f).p = NULL as *mut *mut Proto;
    (*f).sizep = 0 as libc::c_int;
    (*f).code = NULL as *mut Instruction;
    (*f).cache = NULL as *mut LClosure;
    (*f).sizecode = 0 as libc::c_int;
    (*f).lineinfo = NULL as *mut libc::c_int;
    (*f).sizelineinfo = 0 as libc::c_int;
    (*f).upvalues = NULL as *mut Upvaldesc;
    (*f).sizeupvalues = 0 as libc::c_int;
    (*f).numparams = 0 as libc::c_int as lu_byte;
    (*f).is_vararg = 0 as libc::c_int as lu_byte;
    (*f).maxstacksize = 0 as libc::c_int as lu_byte;
    (*f).locvars = NULL as *mut LocVar;
    (*f).sizelocvars = 0 as libc::c_int;
    (*f).linedefined = 0 as libc::c_int;
    (*f).lastlinedefined = 0 as libc::c_int;
    (*f).source = NULL as *mut TString;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_freeproto(mut L: *mut lua_State, mut f: *mut Proto) {
    luaM_freearray!(
        L, f -> code, f -> sizecode
    )(
        L,
        luaM_freearray!(L, f -> code, f -> sizecode),
        luaM_freearray!(L, f -> code, f -> sizecode),
        luaM_freearray!(L, f -> code, f -> sizecode),
    );
    luaM_freearray!(
        L, f -> p, f -> sizep
    )(
        L,
        luaM_freearray!(L, f -> p, f -> sizep),
        luaM_freearray!(L, f -> p, f -> sizep),
        luaM_freearray!(L, f -> p, f -> sizep),
    );
    luaM_freearray!(
        L, f -> k, f -> sizek
    )(
        L,
        luaM_freearray!(L, f -> k, f -> sizek),
        luaM_freearray!(L, f -> k, f -> sizek),
        luaM_freearray!(L, f -> k, f -> sizek),
    );
    luaM_freearray!(
        L, f -> lineinfo, f -> sizelineinfo
    )(
        L,
        luaM_freearray!(L, f -> lineinfo, f -> sizelineinfo),
        luaM_freearray!(L, f -> lineinfo, f -> sizelineinfo),
        luaM_freearray!(L, f -> lineinfo, f -> sizelineinfo),
    );
    luaM_freearray!(
        L, f -> locvars, f -> sizelocvars
    )(
        L,
        luaM_freearray!(L, f -> locvars, f -> sizelocvars),
        luaM_freearray!(L, f -> locvars, f -> sizelocvars),
        luaM_freearray!(L, f -> locvars, f -> sizelocvars),
    );
    luaM_freearray!(
        L, f -> upvalues, f -> sizeupvalues
    )(
        L,
        luaM_freearray!(L, f -> upvalues, f -> sizeupvalues),
        luaM_freearray!(L, f -> upvalues, f -> sizeupvalues),
        luaM_freearray!(L, f -> upvalues, f -> sizeupvalues),
    );
    luaM_free!(L, f)(L, luaM_free!(L, f), luaM_free!(L, f), luaM_free!(L, f));
}
#[no_mangle]
pub unsafe extern "C" fn luaF_getlocalname(
    mut f: *const Proto,
    mut local_number: libc::c_int,
    mut pc: libc::c_int,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*f).sizelocvars && (*((*f).locvars).offset(i as isize)).startpc <= pc {
        if pc < (*((*f).locvars).offset(i as isize)).endpc {
            local_number -= 1;
            if local_number == 0 as libc::c_int {
                return getstr!(f -> locvars[i].varname);
            }
        }
        i += 1;
    }
    return NULL as *const libc::c_char;
}
#[no_mangle]
pub static mut luaO_nilobject_: TValue = {
    let mut init = lua_TValue {
        value_: Value { gc: NULL as *mut GCObject },
        tt_: LUA_TNIL,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn luaO_int2fb(mut x: libc::c_uint) -> libc::c_int {
    let mut e = 0 as libc::c_int;
    if x < 8 as libc::c_int as libc::c_uint {
        return x as libc::c_int;
    }
    while x >= ((8 as libc::c_int) << 4 as libc::c_int) as libc::c_uint {
        x = x.wrapping_add(0xf as libc::c_int as libc::c_uint) >> 4 as libc::c_int;
        e += 4 as libc::c_int;
    }
    while x >= ((8 as libc::c_int) << 1 as libc::c_int) as libc::c_uint {
        x = x.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int;
        e += 1;
    }
    return (e + 1 as libc::c_int) << 3 as libc::c_int | cast_int!(x) - 8 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_fb2int(mut x: libc::c_int) -> libc::c_int {
    return if x < 8 as libc::c_int {
        x
    } else {
        ((x & 7 as libc::c_int) + 8 as libc::c_int)
            << (x >> 3 as libc::c_int) - 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_ceillog2(mut x: libc::c_uint) -> libc::c_int {
    static mut log_2: [lu_byte; 256] = [
        0 as libc::c_int as lu_byte,
        1 as libc::c_int as lu_byte,
        2 as libc::c_int as lu_byte,
        2 as libc::c_int as lu_byte,
        3 as libc::c_int as lu_byte,
        3 as libc::c_int as lu_byte,
        3 as libc::c_int as lu_byte,
        3 as libc::c_int as lu_byte,
        4 as libc::c_int as lu_byte,
        4 as libc::c_int as lu_byte,
        4 as libc::c_int as lu_byte,
        4 as libc::c_int as lu_byte,
        4 as libc::c_int as lu_byte,
        4 as libc::c_int as lu_byte,
        4 as libc::c_int as lu_byte,
        4 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        5 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        6 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        7 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
        8 as libc::c_int as lu_byte,
    ];
    let mut l = 0 as libc::c_int;
    x = x.wrapping_sub(1);
    while x >= 256 as libc::c_int as libc::c_uint {
        l += 8 as libc::c_int;
        x >>= 8 as libc::c_int;
    }
    return l + log_2[x as usize] as libc::c_int;
}
unsafe extern "C" fn intarith(
    mut L: *mut lua_State,
    mut op: libc::c_int,
    mut v1: lua_Integer,
    mut v2: lua_Integer,
) -> lua_Integer {
    match op {
        LUA_OPADD => return intop!(+, v1, v2),
        LUA_OPSUB => return intop!(-, v1, v2),
        LUA_OPMUL => return intop!(*, v1, v2),
        LUA_OPMOD => return luaV_mod(L, v1, v2),
        LUA_OPIDIV => return luaV_div(L, v1, v2),
        LUA_OPBAND => return intop!(&, v1, v2),
        LUA_OPBOR => return intop!(|, v1, v2),
        LUA_OPBXOR => return intop!(^, v1, v2),
        LUA_OPSHL => return luaV_shiftl(v1, v2),
        LUA_OPSHR => return luaV_shiftl(v1, -v2),
        LUA_OPUNM => return intop!(-, 0, v1),
        LUA_OPBNOT => return intop!(^, ~ l_castS2U(0), v1),
        _ => return 0 as libc::c_int as lua_Integer,
    };
}
unsafe extern "C" fn numarith(
    mut L: *mut lua_State,
    mut op: libc::c_int,
    mut v1: lua_Number,
    mut v2: lua_Number,
) -> lua_Number {
    match op {
        LUA_OPADD => return luai_numadd!(L, v1, v2),
        LUA_OPSUB => return luai_numsub!(L, v1, v2),
        LUA_OPMUL => return luai_nummul!(L, v1, v2),
        LUA_OPDIV => return luai_numdiv!(L, v1, v2),
        LUA_OPPOW => return luai_numpow!(L, v1, v2),
        LUA_OPIDIV => return luai_numidiv!(L, v1, v2),
        LUA_OPUNM => return luai_numunm!(L, v1),
        LUA_OPMOD => {
            let mut m: lua_Number = 0.;
            let ref mut fresh193 = luai_nummod!(L, v1, v2, m);
            *fresh193 = fmod(v1, v2);
            if luai_nummod!(L, v1, v2, m) != 0 {
                let ref mut fresh194 = luai_nummod!(L, v1, v2, m);
                *fresh194 += luai_nummod!(L, v1, v2, m);
            }
            return m;
        }
        _ => return 0 as libc::c_int as lua_Number,
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_arith(
    mut L: *mut lua_State,
    mut op: libc::c_int,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: *mut TValue,
) {
    match op {
        LUA_OPBAND | LUA_OPBOR | LUA_OPBXOR | LUA_OPSHL | LUA_OPSHR | LUA_OPBNOT => {
            let mut i1: lua_Integer = 0;
            let mut i2: lua_Integer = 0;
            if tointeger!(p1, & i1) != 0 && tointeger!(p2, & i2) != 0 {
                let mut io = res;
                (*io).value_.i = intarith(L, op, i1, i2);
                (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
                return;
            }
        }
        LUA_OPDIV | LUA_OPPOW => {
            let mut n1: lua_Number = 0.;
            let mut n2: lua_Number = 0.;
            if tonumber!(p1, & n1) != 0 && tonumber!(p2, & n2) != 0 {
                let mut io_0 = res;
                (*io_0).value_.n = numarith(L, op, n1, n2);
                (*io_0).tt_ = 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int;
                return;
            }
        }
        _ => {
            let mut n1_0: lua_Number = 0.;
            let mut n2_0: lua_Number = 0.;
            if ttisinteger!(p1) != 0 && ttisinteger!(p2) != 0 {
                let mut io_1 = res;
                (*io_1).value_.i = intarith(L, op, (*p1).value_.i, (*p2).value_.i);
                (*io_1).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
                return;
            } else if tonumber!(p1, & n1) != 0 && tonumber!(p2, & n2) != 0 {
                let mut io_2 = res;
                (*io_2).value_.n = numarith(L, op, n1_0, n2_0);
                (*io_2).tt_ = 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int;
                return;
            }
        }
    }
    luaT_trybinTM(L, p1, p2, res, cast!(TMS, (op - LUA_OPADD) + TM_ADD));
}
#[no_mangle]
pub unsafe extern "C" fn luaO_hexavalue(mut c: libc::c_int) -> libc::c_int {
    if lisdigit!(c) != 0 {
        return c - '0' as i32
    } else {
        return ltolower!(c) - 'a' as i32 + 10 as libc::c_int
    };
}
unsafe extern "C" fn isneg(mut s: *mut *const libc::c_char) -> libc::c_int {
    if **s as libc::c_int == '-' as i32 {
        *s = (*s).offset(1);
        return 1 as libc::c_int;
    } else {
        if **s as libc::c_int == '+' as i32 {
            *s = (*s).offset(1);
        }
    }
    return 0 as libc::c_int;
}
pub const L_MAXLENNUM: libc::c_int = 200 as libc::c_int;
unsafe extern "C" fn l_str2dloc(
    mut s: *const libc::c_char,
    mut result: *mut lua_Number,
    mut mode: libc::c_int,
) -> *const libc::c_char {
    let mut endptr = 0 as *mut libc::c_char;
    *result = if mode == 'x' as i32 {
        lua_strx2number!(s, & endptr)
    } else {
        lua_str2number!(s, & endptr)
    };
    if endptr == s as *mut libc::c_char {
        return NULL as *const libc::c_char;
    }
    while *(*__ctype_b_loc()).offset(*endptr as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        endptr = endptr.offset(1);
    }
    return if *endptr as libc::c_int == '\0' as i32 {
        endptr
    } else {
        NULL as *mut libc::c_char
    };
}
unsafe extern "C" fn l_str2d(
    mut s: *const libc::c_char,
    mut result: *mut lua_Number,
) -> *const libc::c_char {
    let mut endptr = 0 as *const libc::c_char;
    let mut pmode: *const libc::c_char = strpbrk(
        s,
        b".xXnN\0" as *const u8 as *const libc::c_char,
    );
    let mut mode = if !pmode.is_null() {
        ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c = *pmode as libc::c_uchar as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*pmode as libc::c_uchar as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*pmode as libc::c_uchar as libc::c_int as isize);
            }
            __res
        })
    } else {
        0 as libc::c_int
    };
    if mode == 'n' as i32 {
        return NULL as *const libc::c_char;
    }
    endptr = l_str2dloc(s, result, mode);
    if endptr.is_null() {
        let mut buff: [libc::c_char; 201] = [0; 201];
        let mut pdot: *const libc::c_char = strchr(s, '.' as i32);
        if strlen(s) > L_MAXLENNUM as libc::c_ulong || pdot.is_null() {
            return NULL as *const libc::c_char;
        }
        strcpy(buff.as_mut_ptr(), s);
        buff[pdot.offset_from(s) as libc::c_long as usize] = lua_getlocaledecpoint!();
        endptr = l_str2dloc(buff.as_mut_ptr(), result, mode);
        if !endptr.is_null() {
            endptr = s
                .offset(endptr.offset_from(buff.as_mut_ptr()) as libc::c_long as isize);
        }
    }
    return endptr;
}
pub const MAXBY10: libc::c_longlong = 9223372036854775807 as libc::c_longlong
    / 10 as libc::c_int as libc::c_longlong;
pub const MAXLASTD: libc::c_longlong = 9223372036854775807 as libc::c_longlong
    % 10 as libc::c_int as libc::c_longlong;
unsafe extern "C" fn l_str2int(
    mut s: *const libc::c_char,
    mut result: *mut lua_Integer,
) -> *const libc::c_char {
    let mut a = 0 as libc::c_int as lua_Unsigned;
    let mut empty = 1 as libc::c_int;
    let mut neg: libc::c_int = 0;
    while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
    }
    neg = isneg(&mut s);
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && (*s.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *s.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
    {
        s = s.offset(2 as libc::c_int as isize);
        while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            a = a
                .wrapping_mul(16 as libc::c_int as libc::c_ulonglong)
                .wrapping_add(luaO_hexavalue(*s as libc::c_int) as libc::c_ulonglong);
            empty = 0 as libc::c_int;
            s = s.offset(1);
        }
    } else {
        while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            let mut d = *s as libc::c_int - '0' as i32;
            if a >= MAXBY10 as lua_Unsigned
                && (a > MAXBY10 as lua_Unsigned || d > MAXLASTD as libc::c_int + neg)
            {
                return NULL as *const libc::c_char;
            }
            a = a
                .wrapping_mul(10 as libc::c_int as libc::c_ulonglong)
                .wrapping_add(d as libc::c_ulonglong);
            empty = 0 as libc::c_int;
            s = s.offset(1);
        }
    }
    while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
    }
    if empty != 0 || *s as libc::c_int != '\0' as i32 {
        return NULL as *const libc::c_char
    } else {
        *result = l_castU2S!((neg) ? 0u - a : a);
        return s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_str2num(
    mut s: *const libc::c_char,
    mut o: *mut TValue,
) -> size_t {
    let mut i: lua_Integer = 0;
    let mut n: lua_Number = 0.;
    let mut e = 0 as *const libc::c_char;
    e = l_str2int(s, &mut i);
    if !e.is_null() {
        let mut io = setivalue!(o, i);
        (*io).value_.i = setivalue!(o, i);
        (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
    } else {
        e = l_str2d(s, &mut n);
        if !e.is_null() {
            let mut io_0 = setfltvalue!(o, n);
            (*io_0).value_.n = setfltvalue!(o, n);
            (*io_0).tt_ = 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int;
        } else {
            return 0 as libc::c_int as size_t
        }
    }
    return (e.offset_from(s) as libc::c_long + 1 as libc::c_int as libc::c_long)
        as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_utf8esc(
    mut buff: *mut libc::c_char,
    mut x: libc::c_ulong,
) -> libc::c_int {
    let mut n = 1 as libc::c_int;
    if x < 0x80 as libc::c_int as libc::c_ulong {
        *buff.offset((UTF8BUFFSZ - 1 as libc::c_int) as isize) = cast!(char, x);
    } else {
        let mut mfb = 0x3f as libc::c_int as libc::c_uint;
        loop {
            let fresh195 = n;
            n = n + 1;
            *buff
                .offset(
                    (UTF8BUFFSZ - fresh195) as isize,
                ) = (0x80 as libc::c_int as libc::c_ulong
                | x & 0x3f as libc::c_int as libc::c_ulong) as libc::c_char;
            x >>= 6 as libc::c_int;
            mfb >>= 1 as libc::c_int;
            if !(x > mfb as libc::c_ulong) {
                break;
            }
        }
        *buff.offset((UTF8BUFFSZ - n) as isize) = cast!(char, (~ mfb << 1) | x);
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_tostring(mut L: *mut lua_State, mut obj: StkId) {
    let mut buff: [libc::c_char; 50] = [0; 50];
    let mut len: size_t = 0;
    if ttisinteger!(obj) != 0 {
        len = snprintf(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
            b"%lld\0" as *const u8 as *const libc::c_char,
            (*obj).value_.i,
        ) as size_t;
    } else {
        len = snprintf(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
            b"%.14g\0" as *const u8 as *const libc::c_char,
            (*obj).value_.n,
        ) as size_t;
        if buff[strspn(
            buff.as_mut_ptr(),
            b"-0123456789\0" as *const u8 as *const libc::c_char,
        ) as usize] as libc::c_int == '\0' as i32
        {
            let fresh196 = len;
            len = len.wrapping_add(1);
            buff[fresh196 as usize] = lua_getlocaledecpoint!();
            let fresh197 = len;
            len = len.wrapping_add(1);
            buff[fresh197 as usize] = '0' as i32 as libc::c_char;
        }
    }
    let mut io = obj;
    let mut x_ = luaS_newlstr(L, buff.as_mut_ptr(), len);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
}
unsafe extern "C" fn pushstr(
    mut L: *mut lua_State,
    mut str: *const libc::c_char,
    mut l: size_t,
) {
    let mut io = (*L).top;
    let mut x_ = luaS_newlstr(L, str, l);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    luaD_inctop(L);
}
#[no_mangle]
pub unsafe extern "C" fn luaO_pushvfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut argp: ::core::ffi::VaList,
) -> *const libc::c_char {
    let mut current_block: u64;
    let mut n = 0 as libc::c_int;
    loop {
        let mut e: *const libc::c_char = strchr(fmt, '%' as i32);
        if e.is_null() {
            break;
        }
        pushstr(L, fmt, e.offset_from(fmt) as libc::c_long as size_t);
        match *e.offset(1 as libc::c_int as isize) as libc::c_int {
            115 => {
                let mut s: *const libc::c_char = va_arg!(argp, char *);
                if s.is_null() {
                    s = b"(null)\0" as *const u8 as *const libc::c_char;
                }
                pushstr(L, s, strlen(s));
                current_block = 14136749492126903395;
            }
            99 => {
                let mut buff = argp.arg::<libc::c_int>() as libc::c_char;
                if *(*__ctype_b_loc())
                    .offset(buff as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    pushstr(L, &mut buff, 1 as libc::c_int as size_t);
                } else {
                    luaO_pushfstring(
                        L,
                        b"<\\%d>\0" as *const u8 as *const libc::c_char,
                        cast_uchar!(buff),
                    );
                }
                current_block = 14136749492126903395;
            }
            100 => {
                let mut io = (*L).top;
                (*io).value_.i = argp.arg::<libc::c_int>() as lua_Integer;
                (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
                current_block = 7845224851545032865;
            }
            73 => {
                let mut io_0 = (*L).top;
                (*io_0).value_.i = argp.arg::<l_uacInt>();
                (*io_0).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
                current_block = 7845224851545032865;
            }
            102 => {
                let mut io_1 = (*L).top;
                (*io_1).value_.n = argp.arg::<l_uacNumber>();
                (*io_1).tt_ = 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int;
                current_block = 7845224851545032865;
            }
            112 => {
                let mut buff_0: [libc::c_char; 40] = [0; 40];
                let mut p = va_arg!(argp, void *);
                let mut l = lua_pointer2str!(buff, sizeof(buff), p);
                pushstr(L, buff_0.as_mut_ptr(), l as size_t);
                current_block = 14136749492126903395;
            }
            85 => {
                let mut buff_1: [libc::c_char; 8] = [0; 8];
                let mut l_0 = luaO_utf8esc(
                    buff_1.as_mut_ptr(),
                    argp.arg::<libc::c_long>() as libc::c_ulong,
                );
                pushstr(
                    L,
                    buff_1
                        .as_mut_ptr()
                        .offset(UTF8BUFFSZ as isize)
                        .offset(-(l_0 as isize)),
                    l_0 as size_t,
                );
                current_block = 14136749492126903395;
            }
            37 => {
                pushstr(
                    L,
                    b"%\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                );
                current_block = 14136749492126903395;
            }
            _ => {
                luaG_runerror(
                    L,
                    b"invalid option '%%%c' to 'lua_pushfstring'\0" as *const u8
                        as *const libc::c_char,
                    *e.offset(1 as libc::c_int as isize) as libc::c_int,
                );
            }
        }
        match current_block {
            7845224851545032865 => {
                luaD_inctop(L);
                luaO_tostring(L, ((*L).top).offset(-(1 as libc::c_int as isize)));
            }
            _ => {}
        }
        n += 2 as libc::c_int;
        fmt = e.offset(2 as libc::c_int as isize);
    }
    if ((*L).stack_last).offset_from((*L).top) as libc::c_long <= luaD_checkstack!(L, 1)
    {
        luaD_checkstack!(L, 1)(L, 1 as libc::c_int);
    }
    pushstr(L, fmt, strlen(fmt));
    if n > 0 as libc::c_int {
        luaV_concat(L, n + 1 as libc::c_int);
    }
    return svalue!(L -> top - 1);
}
#[no_mangle]
pub unsafe extern "C" fn luaO_pushfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *const libc::c_char {
    let mut msg = 0 as *const libc::c_char;
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    msg = luaO_pushvfstring(L, fmt, argp.as_va_list());
    return msg;
}
pub const POS: [libc::c_char; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"\"]\0")
};
#[no_mangle]
pub unsafe extern "C" fn luaO_chunkid(
    mut out: *mut libc::c_char,
    mut source: *const libc::c_char,
    mut bufflen: size_t,
) {
    let mut l = strlen(source);
    if *source as libc::c_int == '=' as i32 {
        if l <= bufflen {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1 as libc::c_int as isize) as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        } else {
            out = out.offset(addstr!(out, source + 1, bufflen - 1) as isize);
            *out = '\0' as i32 as libc::c_char;
        }
    } else if *source as libc::c_int == '@' as i32 {
        if l <= bufflen {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1 as libc::c_int as isize) as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        } else {
            memcpy(
                out as *mut libc::c_void,
                b"...\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            );
            out = out
                .offset(
                    (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
            bufflen = (bufflen as libc::c_ulong).wrapping_sub(LL!(RETS)) as size_t
                as size_t;
            memcpy(
                out as *mut libc::c_void,
                source
                    .offset(1 as libc::c_int as isize)
                    .offset(l as isize)
                    .offset(-(bufflen as isize)) as *const libc::c_void,
                bufflen
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            );
        }
    } else {
        let mut nl: *const libc::c_char = strchr(source, '\n' as i32);
        memcpy(
            out as *mut libc::c_void,
            b"[string \"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        out = out
            .offset(
                (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        bufflen = (bufflen as libc::c_ulong)
            .wrapping_sub(
                LL!(PRE RETS POS).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        if l < bufflen && nl.is_null() {
            out = out.offset(addstr!(out, source, l) as isize);
        } else {
            if !nl.is_null() {
                l = nl.offset_from(source) as libc::c_long as size_t;
            }
            if l > bufflen {
                l = bufflen;
            }
            out = out.offset(addstr!(out, source, l) as isize);
            memcpy(
                out as *mut libc::c_void,
                b"...\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            );
            out = out
                .offset(
                    (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
        }
        memcpy(
            out as *mut libc::c_void,
            POS.as_ptr(),
            LL!(POS)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    };
}
static mut udatatypename: [libc::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"userdata\0")
};
#[no_mangle]
pub static mut luaT_typenames_: [*const libc::c_char; 11] = unsafe {
    [
        b"no value\0" as *const u8 as *const libc::c_char,
        b"nil\0" as *const u8 as *const libc::c_char,
        b"boolean\0" as *const u8 as *const libc::c_char,
        udatatypename.as_ptr(),
        b"number\0" as *const u8 as *const libc::c_char,
        b"string\0" as *const u8 as *const libc::c_char,
        b"table\0" as *const u8 as *const libc::c_char,
        b"function\0" as *const u8 as *const libc::c_char,
        udatatypename.as_ptr(),
        b"thread\0" as *const u8 as *const libc::c_char,
        b"proto\0" as *const u8 as *const libc::c_char,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaT_init(mut L: *mut lua_State) {
    static mut luaT_eventname: [*const libc::c_char; 24] = [
        b"__index\0" as *const u8 as *const libc::c_char,
        b"__newindex\0" as *const u8 as *const libc::c_char,
        b"__gc\0" as *const u8 as *const libc::c_char,
        b"__mode\0" as *const u8 as *const libc::c_char,
        b"__len\0" as *const u8 as *const libc::c_char,
        b"__eq\0" as *const u8 as *const libc::c_char,
        b"__add\0" as *const u8 as *const libc::c_char,
        b"__sub\0" as *const u8 as *const libc::c_char,
        b"__mul\0" as *const u8 as *const libc::c_char,
        b"__mod\0" as *const u8 as *const libc::c_char,
        b"__pow\0" as *const u8 as *const libc::c_char,
        b"__div\0" as *const u8 as *const libc::c_char,
        b"__idiv\0" as *const u8 as *const libc::c_char,
        b"__band\0" as *const u8 as *const libc::c_char,
        b"__bor\0" as *const u8 as *const libc::c_char,
        b"__bxor\0" as *const u8 as *const libc::c_char,
        b"__shl\0" as *const u8 as *const libc::c_char,
        b"__shr\0" as *const u8 as *const libc::c_char,
        b"__unm\0" as *const u8 as *const libc::c_char,
        b"__bnot\0" as *const u8 as *const libc::c_char,
        b"__lt\0" as *const u8 as *const libc::c_char,
        b"__le\0" as *const u8 as *const libc::c_char,
        b"__concat\0" as *const u8 as *const libc::c_char,
        b"__call\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < TM_N as libc::c_int {
        let ref mut fresh198 = (*G!(L)).tmname[i as usize];
        *fresh198 = luaS_new(L, luaT_eventname[i as usize]);
        luaC_fix(L, obj2gco!(G(L) -> tmname[i]));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaT_gettm(
    mut events: *mut Table,
    mut event: TMS,
    mut ename: *mut TString,
) -> *const TValue {
    let mut tm = luaH_getshortstr(events, ename);
    if ttisnil!(tm) != 0 {
        (*events)
            .flags = ((*events).flags as libc::c_int | cast_byte!(1u << event))
            as lu_byte;
        return NULL as *const TValue;
    } else {
        return tm
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_gettmbyobj(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut event: TMS,
) -> *const TValue {
    let mut mt = 0 as *mut Table;
    match ttnov!(o) {
        LUA_TTABLE => {
            mt = (*((*o).value_.gc as *mut GCUnion)).h.metatable;
        }
        LUA_TUSERDATA => {
            mt = (*((*o).value_.gc as *mut GCUnion)).u.metatable;
        }
        _ => {
            mt = (*G!(L)).mt[ttnov!(o) as usize];
        }
    }
    return if !mt.is_null() {
        luaH_getshortstr(mt, (*G!(L)).tmname[event as usize])
    } else {
        &luaO_nilobject_
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_objtypename(
    mut L: *mut lua_State,
    mut o: *const TValue,
) -> *const libc::c_char {
    let mut mt = 0 as *mut Table;
    if ttistable!(o) != 0
        && {
            mt = (*((*o).value_.gc as *mut GCUnion)).h.metatable;
            !mt.is_null()
        }
        || ttisfulluserdata!(o) != 0
            && {
                mt = (*((*o).value_.gc as *mut GCUnion)).u.metatable;
                !mt.is_null()
            }
    {
        let mut name = luaH_getshortstr(
            mt,
            luaS_new(L, b"__name\0" as *const u8 as *const libc::c_char),
        );
        if ttisstring!(name) != 0 {
            return (&mut (*((*name).value_.gc as *mut GCUnion)).ts as *mut TString
                as *mut libc::c_char)
                .offset(::core::mem::size_of::<UTString>() as libc::c_ulong as isize);
        }
    }
    return luaT_typenames_[(((*o).tt_ & 0xf as libc::c_int) + 1 as libc::c_int)
        as usize];
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callTM(
    mut L: *mut lua_State,
    mut f: *const TValue,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut p3: *mut TValue,
    mut hasres: libc::c_int,
) {
    let mut result = savestack!(L, p3);
    let mut func = (*L).top;
    let mut io1 = setobj2s!(L, func, f);
    let ref mut fresh199 = setobj2s!(L, func, f);
    *fresh199 = setobj2s!(L, func, f);
    let mut io1_0 = setobj2s!(L, func + 1, p1);
    let ref mut fresh200 = setobj2s!(L, func + 1, p1);
    *fresh200 = setobj2s!(L, func + 1, p1);
    let mut io1_1 = setobj2s!(L, func + 2, p2);
    let ref mut fresh201 = setobj2s!(L, func + 2, p2);
    *fresh201 = setobj2s!(L, func + 2, p2);
    (*L).top = ((*L).top).offset(3 as libc::c_int as isize);
    if hasres == 0 {
        let mut io1_2 = setobj2s!(L, L -> top ++, p3);
        let ref mut fresh202 = setobj2s!(L, L -> top ++, p3);
        *fresh202 = setobj2s!(L, L -> top ++, p3);
    }
    if isLua!(L -> ci) != 0 {
        luaD_call(L, func, hasres);
    } else {
        luaD_callnoyield(L, func, hasres);
    }
    if hasres != 0 {
        p3 = restorestack!(L, result);
        let mut io1_3 = setobjs2s!(L, p3, -- L -> top);
        let ref mut fresh203 = setobjs2s!(L, p3, -- L -> top);
        *fresh203 = setobjs2s!(L, p3, -- L -> top);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callbinTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) -> libc::c_int {
    let mut tm = luaT_gettmbyobj(L, p1, event);
    if ttisnil!(tm) != 0 {
        tm = luaT_gettmbyobj(L, p2, event);
    }
    if ttisnil!(tm) != 0 {
        return 0 as libc::c_int;
    }
    luaT_callTM(L, tm, p1, p2, res, 1 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaT_trybinTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) {
    if luaT_callbinTM(L, p1, p2, res, event) == 0 {
        match event as libc::c_uint {
            22 => {
                luaG_concaterror(L, p1, p2);
            }
            13 | 14 | 15 | 16 | 17 | 19 => {
                let mut dummy: lua_Number = 0.;
                if tonumber!(p1, & dummy) != 0 && tonumber!(p2, & dummy) != 0 {
                    luaG_tointerror(L, p1, p2);
                } else {
                    luaG_opinterror(
                        L,
                        p1,
                        p2,
                        b"perform bitwise operation on\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            _ => {
                luaG_opinterror(
                    L,
                    p1,
                    p2,
                    b"perform arithmetic on\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callorderTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut event: TMS,
) -> libc::c_int {
    if luaT_callbinTM(L, p1, p2, (*L).top, event) == 0 {
        return -(1 as libc::c_int)
    } else {
        return (l_isfalse!(L -> top) == 0) as libc::c_int
    };
}
pub const LUAI_HASHLIMIT: libc::c_int = 5 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn luaS_eqlngstr(
    mut a: *mut TString,
    mut b: *mut TString,
) -> libc::c_int {
    let mut len = (*a).u.lnglen;
    return (a == b
        || len == (*b).u.lnglen
            && memcmp(getstr!(a), getstr!(b), len) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_hash(
    mut str: *const libc::c_char,
    mut l: size_t,
    mut seed: libc::c_uint,
) -> libc::c_uint {
    let mut h = seed ^ cast!(unsigned int, l);
    let mut step = (l >> LUAI_HASHLIMIT).wrapping_add(1 as libc::c_int as libc::c_ulong);
    while l >= step {
        h
            ^= (h << 5 as libc::c_int)
                .wrapping_add(h >> 2 as libc::c_int)
                .wrapping_add(cast_byte!(str[l - 1]));
        l = (l as libc::c_ulong).wrapping_sub(step) as size_t as size_t;
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_hashlongstr(mut ts: *mut TString) -> libc::c_uint {
    if (*ts).extra as libc::c_int == 0 as libc::c_int {
        (*ts).hash = luaS_hash(getstr!(ts), (*ts).u.lnglen, (*ts).hash);
        (*ts).extra = 1 as libc::c_int as lu_byte;
    }
    return (*ts).hash;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_resize(mut L: *mut lua_State, mut newsize: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut tb: *mut stringtable = &mut (*G!(L)).strt;
    if newsize > (*tb).size {
        if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            >= ::core::mem::size_of::<size_t>() as libc::c_ulong
            && (newsize as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
                > (!(0 as libc::c_int as size_t))
                    .wrapping_div(
                        ::core::mem::size_of::<*mut TString>() as libc::c_ulong,
                    )
        {
            luaM_toobig(L);
        } else {};
        let ref mut fresh204 = luaM_reallocvector!(
            L, tb -> hash, tb -> size, newsize, TString *
        );
        *fresh204 = luaM_realloc_(
            L,
            (*tb).hash as *mut libc::c_void,
            ((*tb).size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut TString>() as libc::c_ulong),
            (newsize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut TString>() as libc::c_ulong),
        ) as *mut *mut TString;
        i = (*tb).size;
        while i < newsize {
            let ref mut fresh205 = *((*tb).hash).offset(i as isize);
            *fresh205 = NULL as *mut TString;
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < (*tb).size {
        let mut p = *((*tb).hash).offset(i as isize);
        let ref mut fresh206 = *((*tb).hash).offset(i as isize);
        *fresh206 = NULL as *mut TString;
        while !p.is_null() {
            let mut hnext = (*p).u.hnext;
            let mut h = lmod!(p -> hash, newsize);
            (*p).u.hnext = *((*tb).hash).offset(h as isize);
            let ref mut fresh207 = *((*tb).hash).offset(h as isize);
            *fresh207 = p;
            p = hnext;
        }
        i += 1;
    }
    if newsize < (*tb).size {
        if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            >= ::core::mem::size_of::<size_t>() as libc::c_ulong
            && (newsize as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
                > (!(0 as libc::c_int as size_t))
                    .wrapping_div(
                        ::core::mem::size_of::<*mut TString>() as libc::c_ulong,
                    )
        {
            luaM_toobig(L);
        } else {};
        let ref mut fresh208 = luaM_reallocvector!(
            L, tb -> hash, tb -> size, newsize, TString *
        );
        *fresh208 = luaM_realloc_(
            L,
            (*tb).hash as *mut libc::c_void,
            ((*tb).size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut TString>() as libc::c_ulong),
            (newsize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut TString>() as libc::c_ulong),
        ) as *mut *mut TString;
    }
    (*tb).size = newsize;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_clearcache(mut g: *mut global_State) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < STRCACHE_N {
        j = 0 as libc::c_int;
        while j < STRCACHE_M {
            if iswhite!(g -> strcache[i] [j]) != 0 {
                (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
            }
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaS_init(mut L: *mut lua_State) {
    let mut g = G!(L);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    luaS_resize(L, MINSTRTABSIZE);
    (*g).memerrmsg = luaS_newliteral!(L, MEMERRMSG);
    luaC_fix(L, obj2gco!(g -> memerrmsg));
    i = 0 as libc::c_int;
    while i < STRCACHE_N {
        j = 0 as libc::c_int;
        while j < STRCACHE_M {
            (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn createstrobj(
    mut L: *mut lua_State,
    mut l: size_t,
    mut tag: libc::c_int,
    mut h: libc::c_uint,
) -> *mut TString {
    let mut ts = 0 as *mut TString;
    let mut o = 0 as *mut GCObject;
    let mut totalsize: size_t = 0;
    totalsize = sizelstring!(l);
    o = luaC_newobj(L, tag, totalsize);
    ts = gco2ts!(o);
    (*ts).hash = h;
    (*ts).extra = 0 as libc::c_int as lu_byte;
    *getstr!(ts).offset(l as isize) = '\0' as i32 as libc::c_char;
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_createlngstrobj(
    mut L: *mut lua_State,
    mut l: size_t,
) -> *mut TString {
    let mut ts = createstrobj(L, l, LUA_TLNGSTR, (*G!(L)).seed);
    (*ts).u.lnglen = l;
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_remove(mut L: *mut lua_State, mut ts: *mut TString) {
    let mut tb: *mut stringtable = &mut (*G!(L)).strt;
    let mut p: *mut *mut TString = &mut *((*tb).hash)
        .offset(lmod!(ts -> hash, tb -> size) as isize) as *mut *mut TString;
    while *p != ts {
        p = &mut (**p).u.hnext;
    }
    *p = (**p).u.hnext;
    (*tb).nuse -= 1;
}
unsafe extern "C" fn internshrstr(
    mut L: *mut lua_State,
    mut str: *const libc::c_char,
    mut l: size_t,
) -> *mut TString {
    let mut ts = 0 as *mut TString;
    let mut g = G!(L);
    let mut h = luaS_hash(str, l, (*g).seed);
    let mut list: *mut *mut TString = &mut *((*g).strt.hash)
        .offset(lmod!(h, g -> strt.size) as isize) as *mut *mut TString;
    ts = *list;
    while !ts.is_null() {
        if l == (*ts).shrlen as libc::c_ulong
            && memcmp(
                str as *const libc::c_void,
                getstr!(ts),
                l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            if isdead!(g, ts) == 0 {
                let ref mut fresh209 = changewhite!(ts);
                *fresh209 = (*fresh209 as libc::c_int ^ WHITEBITS) as lu_byte;
            }
            return ts;
        }
        ts = (*ts).u.hnext;
    }
    if (*g).strt.nuse >= (*g).strt.size && (*g).strt.size <= MAX_INT / 2 as libc::c_int {
        luaS_resize(L, (*g).strt.size * 2 as libc::c_int);
        list = &mut *((*g).strt.hash).offset(lmod!(h, g -> strt.size) as isize)
            as *mut *mut TString;
    }
    ts = createstrobj(L, l, LUA_TSHRSTR, h);
    memcpy(
        getstr!(ts),
        str as *const libc::c_void,
        l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    (*ts).shrlen = cast_byte!(l);
    (*ts).u.hnext = *list;
    *list = ts;
    (*g).strt.nuse += 1;
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newlstr(
    mut L: *mut lua_State,
    mut str: *const libc::c_char,
    mut l: size_t,
) -> *mut TString {
    if l <= LUAI_MAXSHORTLEN as libc::c_ulong {
        return internshrstr(L, str, l)
    } else {
        let mut ts = 0 as *mut TString;
        if l
            >= (if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
            {
                MAX_SIZET
            } else {
                9223372036854775807 as libc::c_longlong as size_t
            })
                .wrapping_sub(::core::mem::size_of::<TString>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
        {
            luaM_toobig(L);
        }
        ts = luaS_createlngstrobj(L, l);
        memcpy(
            getstr!(ts),
            str as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        return ts;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_new(
    mut L: *mut lua_State,
    mut str: *const libc::c_char,
) -> *mut TString {
    let mut i = point2uint!(str).wrapping_rem(STRCACHE_N as libc::c_uint);
    let mut j: libc::c_int = 0;
    let mut p = ((*G!(L)).strcache[i as usize]).as_mut_ptr();
    j = 0 as libc::c_int;
    while j < STRCACHE_M {
        if strcmp(str, getstr!(p[j])) == 0 as libc::c_int {
            return *p.offset(j as isize);
        }
        j += 1;
    }
    j = STRCACHE_M - 1 as libc::c_int;
    while j > 0 as libc::c_int {
        let ref mut fresh210 = *p.offset(j as isize);
        *fresh210 = *p.offset((j - 1 as libc::c_int) as isize);
        j -= 1;
    }
    let ref mut fresh211 = *p.offset(0 as libc::c_int as isize);
    *fresh211 = luaS_newlstr(L, str, strlen(str));
    return *p.offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newudata(
    mut L: *mut lua_State,
    mut s: size_t,
) -> *mut Udata {
    let mut u = 0 as *mut Udata;
    let mut o = 0 as *mut GCObject;
    if s
        > (if (::core::mem::size_of::<size_t>() as libc::c_ulong)
            < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
        {
            MAX_SIZET
        } else {
            9223372036854775807 as libc::c_longlong as size_t
        })
            .wrapping_sub(::core::mem::size_of::<Udata>() as libc::c_ulong)
    {
        luaM_toobig(L);
    }
    o = luaC_newobj(L, LUA_TUSERDATA, sizeludata!(s));
    u = gco2u!(o);
    (*u).len = s;
    (*u).metatable = NULL as *mut Table;
    let mut io: *const TValue = setuservalue!(L, u, luaO_nilobject);
    let mut iu = setuservalue!(L, u, luaO_nilobject);
    let ref mut fresh212 = setuservalue!(L, u, luaO_nilobject);
    *fresh212 = setuservalue!(L, u, luaO_nilobject);
    let ref mut fresh213 = setuservalue!(L, u, luaO_nilobject);
    *fresh213 = (*io).tt_ as lu_byte;
    return u;
}
pub const MAXABITS: libc::c_ulong = (::core::mem::size_of::<libc::c_int>()
    as libc::c_ulong)
    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
pub const MAXASIZE: libc::c_uint = (1 as libc::c_uint) << MAXABITS as libc::c_int;
pub const MAXHBITS: libc::c_int = MAXABITS as libc::c_int - 1 as libc::c_int;
static mut dummynode_: Node = {
    let mut init = Node {
        i_val: {
            let mut init = lua_TValue {
                value_: Value { gc: NULL as *mut GCObject },
                tt_: LUA_TNIL,
            };
            init
        },
        i_key: TKey {
            nk: {
                let mut init = C2RustUnnamed_6 {
                    value_: Value { gc: NULL as *mut GCObject },
                    tt_: LUA_TNIL,
                    next: 0 as libc::c_int,
                };
                init
            },
        },
    };
    init
};
unsafe extern "C" fn l_hashfloat(mut n: lua_Number) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ni: lua_Integer = 0;
    n = l_mathop!(frexp)(n, &mut i) * -cast_num!(INT_MIN);
    if lua_numbertointeger!(n, & ni) == 0 {
        return 0 as libc::c_int
    } else {
        let mut u = cast!(unsigned int, i).wrapping_add(cast!(unsigned int, ni));
        return cast_int!(u <= cast(unsigned int, INT_MAX) ? u : ~ u);
    };
}
unsafe extern "C" fn mainposition(
    mut t: *const Table,
    mut key: *const TValue,
) -> *mut Node {
    match ttype!(key) {
        LUA_TNUMINT => {
            return &mut *((*t).node)
                .offset(
                    ((*key).value_.i
                        & (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int) as libc::c_longlong) as libc::c_int
                        as isize,
                ) as *mut Node;
        }
        LUA_TNUMFLT => {
            return &mut *((*t).node)
                .offset(
                    ((l_hashfloat
                        as unsafe extern "C" fn(
                            lua_Number,
                        ) -> libc::c_int)((*key).value_.n)
                        % (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int | 1 as libc::c_int)) as isize,
                ) as *mut Node;
        }
        LUA_TSHRSTR => {
            return &mut *((*t).node)
                .offset(
                    ((*((*key).value_.gc as *mut GCUnion)).ts.hash
                        & (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int) as libc::c_uint) as libc::c_int as isize,
                ) as *mut Node;
        }
        LUA_TLNGSTR => {
            return &mut *((*t).node)
                .offset(
                    ((luaS_hashlongstr
                        as unsafe extern "C" fn(
                            *mut TString,
                        ) -> libc::c_uint)(&mut (*((*key).value_.gc as *mut GCUnion)).ts)
                        & (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int) as libc::c_uint) as libc::c_int as isize,
                ) as *mut Node;
        }
        LUA_TBOOLEAN => {
            return &mut *((*t).node)
                .offset(
                    ((*key).value_.b
                        & ((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int) as isize,
                ) as *mut Node;
        }
        LUA_TLIGHTUSERDATA => {
            return &mut *((*t).node)
                .offset(
                    (((*key).value_.p as size_t
                        & (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                        as libc::c_uint)
                        .wrapping_rem(
                            (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                                - 1 as libc::c_int | 1 as libc::c_int) as libc::c_uint,
                        ) as isize,
                ) as *mut Node;
        }
        LUA_TLCF => {
            return &mut *((*t).node)
                .offset(
                    ((::core::mem::transmute::<lua_CFunction, size_t>((*key).value_.f)
                        & (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                        as libc::c_uint)
                        .wrapping_rem(
                            (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                                - 1 as libc::c_int | 1 as libc::c_int) as libc::c_uint,
                        ) as isize,
                ) as *mut Node;
        }
        _ => {
            return &mut *((*t).node)
                .offset(
                    (((*key).value_.gc as size_t
                        & (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                        as libc::c_uint)
                        .wrapping_rem(
                            (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                                - 1 as libc::c_int | 1 as libc::c_int) as libc::c_uint,
                        ) as isize,
                ) as *mut Node;
        }
    };
}
unsafe extern "C" fn arrayindex(mut key: *const TValue) -> libc::c_uint {
    if ttisinteger!(key) != 0 {
        let mut k = ivalue!(key);
        if (0 as libc::c_int as libc::c_longlong) < k
            && k as lua_Unsigned <= MAXASIZE as libc::c_ulonglong
        {
            return cast!(unsigned int, k);
        }
    }
    return 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn findindex(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: StkId,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    if ttisnil!(key) != 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    i = arrayindex(key as *const TValue);
    if i != 0 as libc::c_int as libc::c_uint && i <= (*t).sizearray {
        return i
    } else {
        let mut nx: libc::c_int = 0;
        let mut n = mainposition(t, key as *const TValue);
        loop {
            if luaV_rawequalobj!(gkey(n), key) != 0
                || (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_
                    == 9 as libc::c_int + 1 as libc::c_int && iscollectable!(key) != 0
                    && (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).value_.gc
                        as *mut libc::c_void == gcvalue!(key)
            {
                i = n
                    .offset_from(
                        &mut *((*t).node).offset(0 as libc::c_int as isize) as *mut Node,
                    ) as libc::c_long as libc::c_int as libc::c_uint;
                return i
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_add((*t).sizearray);
            }
            nx = gnext!(n);
            if nx == 0 as libc::c_int {
                luaG_runerror(
                    L,
                    b"invalid key to 'next'\0" as *const u8 as *const libc::c_char,
                );
            } else {
                n = n.offset(nx as isize);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_next(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: StkId,
) -> libc::c_int {
    let mut i = findindex(L, t, key);
    while i < (*t).sizearray {
        if ttisnil!(& t -> array[i]) == 0 {
            let mut io = setivalue!(key, i + 1);
            (*io).value_.i = setivalue!(key, i + 1);
            (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
            let mut io1 = setobj2s!(L, key + 1, & t -> array[i]);
            let ref mut fresh214 = setobj2s!(L, key + 1, & t -> array[i]);
            *fresh214 = setobj2s!(L, key + 1, & t -> array[i]);
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    i = i.wrapping_sub((*t).sizearray);
    while cast_int!(i) < sizenode!(t) {
        if !((*((*t).node).offset(i as isize)).i_val.tt_ == 0 as libc::c_int) {
            let mut io1_0 = key;
            *io1_0 = *(&mut (*((*t).node).offset(i as isize)).i_key.tvk as *mut TValue
                as *const TValue);
            let mut io1_1 = key.offset(1 as libc::c_int as isize);
            *io1_1 = (*((*t).node).offset(i as isize)).i_val;
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn computesizes(
    mut nums: *mut libc::c_uint,
    mut pna: *mut libc::c_uint,
) -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut twotoi: libc::c_uint = 0;
    let mut a = 0 as libc::c_int as libc::c_uint;
    let mut na = 0 as libc::c_int as libc::c_uint;
    let mut optimal = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    twotoi = 1 as libc::c_int as libc::c_uint;
    while twotoi > 0 as libc::c_int as libc::c_uint
        && *pna > twotoi.wrapping_div(2 as libc::c_int as libc::c_uint)
    {
        if *nums.offset(i as isize) > 0 as libc::c_int as libc::c_uint {
            a = a.wrapping_add(*nums.offset(i as isize));
            if a > twotoi.wrapping_div(2 as libc::c_int as libc::c_uint) {
                optimal = twotoi;
                na = a;
            }
        }
        i += 1;
        twotoi = twotoi.wrapping_mul(2 as libc::c_int as libc::c_uint);
    }
    *pna = na;
    return optimal;
}
unsafe extern "C" fn countint(
    mut key: *const TValue,
    mut nums: *mut libc::c_uint,
) -> libc::c_int {
    let mut k = arrayindex(key);
    if k != 0 as libc::c_int as libc::c_uint {
        let ref mut fresh215 = *nums.offset(luaO_ceillog2(k) as isize);
        *fresh215 = (*fresh215).wrapping_add(1);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn numusearray(
    mut t: *const Table,
    mut nums: *mut libc::c_uint,
) -> libc::c_uint {
    let mut lg: libc::c_int = 0;
    let mut ttlg: libc::c_uint = 0;
    let mut ause = 0 as libc::c_int as libc::c_uint;
    let mut i = 1 as libc::c_int as libc::c_uint;
    lg = 0 as libc::c_int;
    ttlg = 1 as libc::c_int as libc::c_uint;
    while lg <= MAXABITS as libc::c_int {
        let mut lc = 0 as libc::c_int as libc::c_uint;
        let mut lim = ttlg;
        if lim > (*t).sizearray {
            lim = (*t).sizearray;
            if i > lim {
                break;
            }
        }
        while i <= lim {
            if ttisnil!(& t -> array[i - 1]) == 0 {
                lc = lc.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        let ref mut fresh216 = *nums.offset(lg as isize);
        *fresh216 = (*fresh216).wrapping_add(lc);
        ause = ause.wrapping_add(lc);
        lg += 1;
        ttlg = ttlg.wrapping_mul(2 as libc::c_int as libc::c_uint);
    }
    return ause;
}
unsafe extern "C" fn numusehash(
    mut t: *const Table,
    mut nums: *mut libc::c_uint,
    mut pna: *mut libc::c_uint,
) -> libc::c_int {
    let mut totaluse = 0 as libc::c_int;
    let mut ause = 0 as libc::c_int;
    let mut i = sizenode!(t);
    loop {
        let fresh217 = i;
        i = i - 1;
        if !(fresh217 != 0) {
            break;
        }
        let mut n: *mut Node = &mut *((*t).node).offset(i as isize) as *mut Node;
        if !((*n).i_val.tt_ == 0 as libc::c_int) {
            ause += countint(gkey!(n), nums);
            totaluse += 1;
        }
    }
    *pna = (*pna).wrapping_add(ause as libc::c_uint);
    return totaluse;
}
unsafe extern "C" fn setarrayvector(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut size: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    if ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
        && (size as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
            > (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {};
    let ref mut fresh218 = luaM_reallocvector!(
        L, t -> array, t -> sizearray, size, TValue
    );
    *fresh218 = luaM_realloc_(
        L,
        (*t).array as *mut libc::c_void,
        ((*t).sizearray as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
        (size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    i = (*t).sizearray;
    while i < size {
        let ref mut fresh219 = setnilvalue!(& t -> array[i]);
        *fresh219 = setnilvalue!(& t -> array[i]);
        i = i.wrapping_add(1);
    }
    (*t).sizearray = size;
}
unsafe extern "C" fn setnodevector(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut size: libc::c_uint,
) {
    if size == 0 as libc::c_int as libc::c_uint {
        (*t).node = cast!(Node *, dummynode);
        (*t).lsizenode = 0 as libc::c_int as lu_byte;
        (*t).lastfree = NULL as *mut Node;
    } else {
        let mut i: libc::c_int = 0;
        let mut lsize = luaO_ceillog2(size);
        if lsize > MAXHBITS {
            luaG_runerror(L, b"table overflow\0" as *const u8 as *const libc::c_char);
        }
        size = twoto!(lsize);
        (*t).node = luaM_newvector!(L, size, Node);
        i = 0 as libc::c_int;
        while i < size as libc::c_int {
            let mut n: *mut Node = gnode!(t, i);
            let ref mut fresh220 = gnext!(n);
            *fresh220 = 0 as libc::c_int;
            (*n).i_key.nk.tt_ = 0 as libc::c_int;
            (*n).i_val.tt_ = 0 as libc::c_int;
            i += 1;
        }
        (*t).lsizenode = cast_byte!(lsize);
        (*t).lastfree = gnode!(t, size);
    };
}
unsafe extern "C" fn auxsetnode(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut asn = cast!(AuxsetnodeT *, ud);
    setnodevector(L, (*asn).t, (*asn).nhsize);
}
#[no_mangle]
pub unsafe extern "C" fn luaH_resize(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut nasize: libc::c_uint,
    mut nhsize: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_int = 0;
    let mut asn = AuxsetnodeT {
        t: 0 as *mut Table,
        nhsize: 0,
    };
    let mut oldasize = (*t).sizearray;
    let mut oldhsize = allocsizenode!(t);
    let mut nold = (*t).node;
    if nasize > oldasize {
        setarrayvector(L, t, nasize);
    }
    asn.t = t;
    asn.nhsize = nhsize;
    if luaD_rawrunprotected(
        L,
        Some(
            auxsetnode as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
        ),
        &mut asn as *mut AuxsetnodeT as *mut libc::c_void,
    ) != LUA_OK
    {
        setarrayvector(L, t, oldasize);
        luaD_throw(L, LUA_ERRMEM);
    }
    if nasize < oldasize {
        (*t).sizearray = nasize;
        i = nasize;
        while i < oldasize {
            if ttisnil!(& t -> array[i]) == 0 {
                luaH_setint(
                    L,
                    t,
                    i.wrapping_add(1 as libc::c_int as libc::c_uint) as lua_Integer,
                    &mut *((*t).array).offset(i as isize),
                );
            }
            i = i.wrapping_add(1);
        }
        if ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
            >= ::core::mem::size_of::<size_t>() as libc::c_ulong
            && (nasize as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
                > (!(0 as libc::c_int as size_t))
                    .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
        {
            luaM_toobig(L);
        } else {};
        let ref mut fresh221 = luaM_reallocvector!(
            L, t -> array, oldasize, nasize, TValue
        );
        *fresh221 = luaM_realloc_(
            L,
            (*t).array as *mut libc::c_void,
            (oldasize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
            (nasize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
        ) as *mut TValue;
    }
    j = oldhsize - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        let mut old = nold.offset(j as isize);
        if !((*old).i_val.tt_ == 0 as libc::c_int) {
            let mut io1 = luaH_set(
                L,
                t,
                &mut (*old).i_key.tvk as *mut TValue as *const TValue,
            );
            *io1 = (*old).i_val;
        }
        j -= 1;
    }
    if oldhsize > 0 as libc::c_int {
        luaM_realloc_(
            L,
            nold as *mut libc::c_void,
            (oldhsize as size_t)
                .wrapping_mul(::core::mem::size_of::<Node>() as libc::c_ulong),
            0 as libc::c_int as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaH_resizearray(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut nasize: libc::c_uint,
) {
    let mut nsize = allocsizenode!(t);
    luaH_resize(L, t, nasize, nsize as libc::c_uint);
}
unsafe extern "C" fn rehash(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut ek: *const TValue,
) {
    let mut asize: libc::c_uint = 0;
    let mut na: libc::c_uint = 0;
    let mut nums: [libc::c_uint; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut totaluse: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= MAXABITS as libc::c_int {
        nums[i as usize] = 0 as libc::c_int as libc::c_uint;
        i += 1;
    }
    na = numusearray(t, nums.as_mut_ptr());
    totaluse = na as libc::c_int;
    totaluse += numusehash(t, nums.as_mut_ptr(), &mut na);
    na = na.wrapping_add(countint(ek, nums.as_mut_ptr()) as libc::c_uint);
    totaluse += 1;
    asize = computesizes(nums.as_mut_ptr(), &mut na);
    luaH_resize(L, t, asize, (totaluse as libc::c_uint).wrapping_sub(na));
}
#[no_mangle]
pub unsafe extern "C" fn luaH_new(mut L: *mut lua_State) -> *mut Table {
    let mut o = luaC_newobj(
        L,
        LUA_TTABLE,
        ::core::mem::size_of::<Table>() as libc::c_ulong,
    );
    let mut t: *mut Table = gco2t!(o);
    (*t).metatable = NULL as *mut Table;
    (*t).flags = cast_byte!(~ 0);
    (*t).array = NULL as *mut TValue;
    (*t).sizearray = 0 as libc::c_int as libc::c_uint;
    setnodevector(L, t, 0 as libc::c_int as libc::c_uint);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_free(mut L: *mut lua_State, mut t: *mut Table) {
    if !isdummy!(t).is_null() {
        luaM_realloc_(
            L,
            (*t).node as *mut libc::c_void,
            (((1 as libc::c_int) << (*t).lsizenode as libc::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<Node>() as libc::c_ulong),
            0 as libc::c_int as size_t,
        );
    }
    luaM_freearray!(
        L, t -> array, t -> sizearray
    )(
        L,
        luaM_freearray!(L, t -> array, t -> sizearray),
        luaM_freearray!(L, t -> array, t -> sizearray),
        luaM_freearray!(L, t -> array, t -> sizearray),
    );
    luaM_free!(L, t)(L, luaM_free!(L, t), luaM_free!(L, t), luaM_free!(L, t));
}
unsafe extern "C" fn getfreepos(mut t: *mut Table) -> *mut Node {
    if !isdummy!(t).is_null() {
        while (*t).lastfree > (*t).node {
            (*t).lastfree = ((*t).lastfree).offset(-1);
            if (*(&mut (*(*t).lastfree).i_key.tvk as *mut TValue as *const TValue)).tt_
                == 0 as libc::c_int
            {
                return (*t).lastfree;
            }
        }
    }
    return NULL as *mut Node;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_newkey(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
) -> *mut TValue {
    let mut mp = 0 as *mut Node;
    let mut aux = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    if ttisnil!(key) != 0 {
        luaG_runerror(L, b"table index is nil\0" as *const u8 as *const libc::c_char);
    } else {
        if ttisfloat!(key) != 0 {
            let mut k: lua_Integer = 0;
            if luaV_tointeger(key, &mut k, 0 as libc::c_int) != 0 {
                let mut io: *mut TValue = setivalue!(& aux, k);
                (*io).value_.i = setivalue!(& aux, k);
                (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
                key = &mut aux;
            } else if !((*key).value_.n == (*key).value_.n) {
                luaG_runerror(
                    L,
                    b"table index is NaN\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    mp = mainposition(t, key);
    if !((*mp).i_val.tt_ == 0 as libc::c_int) || isdummy!(t).is_null() {
        let mut othern = 0 as *mut Node;
        let mut f = getfreepos(t);
        if f.is_null() {
            rehash(L, t, key);
            return luaH_set(L, t, key);
        }
        othern = mainposition(t, gkey!(mp));
        if othern != mp {
            while othern.offset(gnext!(othern) as isize) != mp {
                othern = othern.offset(gnext!(othern) as isize);
            }
            let ref mut fresh222 = gnext!(othern);
            *fresh222 = cast_int!(f - othern);
            *f = *mp;
            if gnext!(mp) != 0 as libc::c_int {
                let ref mut fresh223 = gnext!(f);
                *fresh223 += cast_int!(mp - f);
                let ref mut fresh224 = gnext!(mp);
                *fresh224 = 0 as libc::c_int;
            }
            (*mp).i_val.tt_ = 0 as libc::c_int;
        } else {
            if gnext!(mp) != 0 as libc::c_int {
                let ref mut fresh225 = gnext!(f);
                *fresh225 = cast_int!((mp + gnext(mp)) - f);
            }
            let ref mut fresh226 = gnext!(mp);
            *fresh226 = cast_int!(f - mp);
            mp = f;
        }
    }
    let mut k_: *mut TKey = setnodekey!(L, & mp -> i_key, key);
    let mut io_ = setnodekey!(L, & mp -> i_key, key);
    let ref mut fresh227 = setnodekey!(L, & mp -> i_key, key);
    *fresh227 = setnodekey!(L, & mp -> i_key, key);
    let ref mut fresh228 = setnodekey!(L, & mp -> i_key, key);
    *fresh228 = setnodekey!(L, & mp -> i_key, key);
    if luaC_barrierback!(L, t, key) != 0 {} else {};
    return gval!(mp);
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getint(
    mut t: *mut Table,
    mut key: lua_Integer,
) -> *const TValue {
    if l_castS2U!(key).wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
        < (*t).sizearray as libc::c_ulonglong
    {
        return &mut *((*t).array)
            .offset((key - 1 as libc::c_int as libc::c_longlong) as isize) as *mut TValue
    } else {
        let mut n: *mut Node = hashint!(t, key);
        loop {
            if (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_
                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                && (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).value_.i
                    == key
            {
                return gval!(n)
            } else {
                let mut nx = gnext!(n);
                if nx == 0 as libc::c_int {
                    break;
                }
                n = n.offset(nx as isize);
            }
        }
        return &luaO_nilobject_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getshortstr(
    mut t: *mut Table,
    mut key: *mut TString,
) -> *const TValue {
    let mut n: *mut Node = hashstr!(t, key);
    loop {
        let mut k = gkey!(n);
        if ttisshrstring!(k) != 0 && eqshrstr!(tsvalue(k), key) != 0 {
            return gval!(n)
        } else {
            let mut nx = gnext!(n);
            if nx == 0 as libc::c_int {
                return &luaO_nilobject_;
            }
            n = n.offset(nx as isize);
        }
    };
}
unsafe extern "C" fn getgeneric(
    mut t: *mut Table,
    mut key: *const TValue,
) -> *const TValue {
    let mut n = mainposition(t, key);
    loop {
        if luaV_rawequalobj!(gkey(n), key) != 0 {
            return gval!(n)
        } else {
            let mut nx = gnext!(n);
            if nx == 0 as libc::c_int {
                return &luaO_nilobject_;
            }
            n = n.offset(nx as isize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getstr(
    mut t: *mut Table,
    mut key: *mut TString,
) -> *const TValue {
    if (*key).tt as libc::c_int == LUA_TSHRSTR {
        return luaH_getshortstr(t, key)
    } else {
        let mut ko = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut io: *mut TValue = setsvalue!(cast(lua_State *, NULL), & ko, key);
        let mut x_ = setsvalue!(cast(lua_State *, NULL), & ko, key);
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        return getgeneric(t, &mut ko);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_get(
    mut t: *mut Table,
    mut key: *const TValue,
) -> *const TValue {
    match ttype!(key) {
        LUA_TSHRSTR => return luaH_getshortstr(t, tsvalue!(key)),
        LUA_TNUMINT => return luaH_getint(t, ivalue!(key)),
        LUA_TNIL => return &luaO_nilobject_,
        LUA_TNUMFLT => {
            let mut k: lua_Integer = 0;
            if luaV_tointeger(key, &mut k, 0 as libc::c_int) != 0 {
                return luaH_getint(t, k);
            }
        }
        _ => {}
    }
    return getgeneric(t, key);
}
#[no_mangle]
pub unsafe extern "C" fn luaH_set(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
) -> *mut TValue {
    let mut p = luaH_get(t, key);
    if p != &luaO_nilobject_ as *const TValue {
        return cast!(TValue *, p)
    } else {
        return luaH_newkey(L, t, key)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_setint(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: lua_Integer,
    mut value: *mut TValue,
) {
    let mut p = luaH_getint(t, key);
    let mut cell = 0 as *mut TValue;
    if p != &luaO_nilobject_ as *const TValue {
        cell = cast!(TValue *, p);
    } else {
        let mut k = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut io: *mut TValue = setivalue!(& k, key);
        (*io).value_.i = setivalue!(& k, key);
        (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
        cell = luaH_newkey(L, t, &mut k);
    };
}
unsafe extern "C" fn unbound_search(
    mut t: *mut Table,
    mut j: lua_Unsigned,
) -> lua_Unsigned {
    let mut i = j;
    j = j.wrapping_add(1);
    while !((*luaH_getint(t, j as lua_Integer)).tt_ == 0 as libc::c_int) {
        i = j;
        if j
            > l_castS2U!(LUA_MAXINTEGER)
                .wrapping_div(2 as libc::c_int as libc::c_ulonglong)
        {
            i = 1 as libc::c_int as lua_Unsigned;
            while !((*luaH_getint(t, i as lua_Integer)).tt_ == 0 as libc::c_int) {
                i = i.wrapping_add(1);
            }
            return i.wrapping_sub(1 as libc::c_int as libc::c_ulonglong);
        }
        j = (j as libc::c_ulonglong).wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
            as lua_Unsigned as lua_Unsigned;
    }
    while j.wrapping_sub(i) > 1 as libc::c_int as libc::c_ulonglong {
        let mut m = i
            .wrapping_add(j)
            .wrapping_div(2 as libc::c_int as libc::c_ulonglong);
        if (*luaH_getint(t, m as lua_Integer)).tt_ == 0 as libc::c_int {
            j = m;
        } else {
            i = m;
        }
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getn(mut t: *mut Table) -> lua_Unsigned {
    let mut j = (*t).sizearray;
    if j > 0 as libc::c_int as libc::c_uint && ttisnil!(& t -> array[j - 1]) != 0 {
        let mut i = 0 as libc::c_int as libc::c_uint;
        while j.wrapping_sub(i) > 1 as libc::c_int as libc::c_uint {
            let mut m = i.wrapping_add(j).wrapping_div(2 as libc::c_int as libc::c_uint);
            if ttisnil!(& t -> array[m - 1]) != 0 {
                j = m;
            } else {
                i = m;
            }
        }
        return i as lua_Unsigned;
    } else if isdummy!(t).is_null() {
        return j as lua_Unsigned
    } else {
        return unbound_search(t, j as lua_Unsigned)
    };
}
unsafe extern "C" fn seterrorobj(
    mut L: *mut lua_State,
    mut errcode: libc::c_int,
    mut oldtop: StkId,
) {
    match errcode {
        LUA_ERRMEM => {
            let mut io = setsvalue2s!(L, oldtop, G(L) -> memerrmsg);
            let mut x_ = setsvalue2s!(L, oldtop, G(L) -> memerrmsg);
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        }
        LUA_ERRERR => {
            let mut io_0 = oldtop;
            let mut x__0 = luaS_newlstr(
                L,
                b"error in error handling\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
            (*io_0)
                .tt_ = (*x__0).tt as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int;
        }
        _ => {
            let mut io1 = setobjs2s!(L, oldtop, L -> top - 1);
            let ref mut fresh229 = setobjs2s!(L, oldtop, L -> top - 1);
            *fresh229 = setobjs2s!(L, oldtop, L -> top - 1);
        }
    }
    (*L).top = oldtop.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_throw(
    mut L: *mut lua_State,
    mut errcode: libc::c_int,
) -> ! {
    if !((*L).errorJmp).is_null() {
        ::core::ptr::write_volatile(
            &mut (*(*L).errorJmp).status as *mut libc::c_int,
            errcode,
        );
        LUAI_THROW!(
            L, L -> errorJmp
        )(LUAI_THROW!(L, L -> errorJmp), LUAI_THROW!(L, L -> errorJmp));
    } else {
        let mut g = G!(L);
        (*L).status = cast_byte!(errcode);
        if !((*(*g).mainthread).errorJmp).is_null() {
            let mut io1 = setobjs2s!(L, g -> mainthread -> top ++, L -> top - 1);
            let ref mut fresh230 = setobjs2s!(
                L, g -> mainthread -> top ++, L -> top - 1
            );
            *fresh230 = setobjs2s!(L, g -> mainthread -> top ++, L -> top - 1);
            luaD_throw((*g).mainthread, errcode);
        } else {
            if ((*g).panic).is_some() {
                seterrorobj(L, errcode, (*L).top);
                if (*(*L).ci).top < (*L).top {
                    (*(*L).ci).top = (*L).top;
                }
                ((*g).panic).expect("non-null function pointer")(L);
            }
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_rawrunprotected(
    mut L: *mut lua_State,
    mut f: Pfunc,
    mut ud: *mut libc::c_void,
) -> libc::c_int {
    let mut oldnCcalls = (*L).nCcalls;
    let mut lj = lua_longjmp {
        previous: 0 as *mut lua_longjmp,
        b: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        status: 0,
    };
    ::core::ptr::write_volatile(&mut lj.status as *mut libc::c_int, LUA_OK);
    lj.previous = (*L).errorJmp;
    (*L).errorJmp = &mut lj;
    if _setjmp((lj.b).as_mut_ptr()) == LUAI_TRY!(L, & lj, (* f) (L, ud);) {
        (Some(f.expect("non-null function pointer")))
            .expect("non-null function pointer")(L, ud);
    }
    (*L).errorJmp = lj.previous;
    (*L).nCcalls = oldnCcalls;
    return lj.status;
}
unsafe extern "C" fn correctstack(mut L: *mut lua_State, mut oldstack: *mut TValue) {
    let mut ci = 0 as *mut CallInfo;
    let mut up = 0 as *mut UpVal;
    (*L)
        .top = ((*L).stack)
        .offset(((*L).top).offset_from(oldstack) as libc::c_long as isize);
    up = (*L).openupval;
    while !up.is_null() {
        (*up)
            .v = ((*L).stack)
            .offset(((*up).v).offset_from(oldstack) as libc::c_long as isize);
        up = (*up).u.open.next;
    }
    ci = (*L).ci;
    while !ci.is_null() {
        (*ci)
            .top = ((*L).stack)
            .offset(((*ci).top).offset_from(oldstack) as libc::c_long as isize);
        (*ci)
            .func = ((*L).stack)
            .offset(((*ci).func).offset_from(oldstack) as libc::c_long as isize);
        if isLua!(ci) != 0 {
            (*ci)
                .u
                .l
                .base = ((*L).stack)
                .offset(((*ci).u.l.base).offset_from(oldstack) as libc::c_long as isize);
        }
        ci = (*ci).previous;
    }
}
pub const ERRORSTACKSIZE: libc::c_int = LUAI_MAXSTACK + 200 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn luaD_reallocstack(
    mut L: *mut lua_State,
    mut newsize: libc::c_int,
) {
    let mut oldstack = (*L).stack;
    let mut lim = (*L).stacksize;
    if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::core::mem::size_of::<size_t>() as libc::c_ulong
        && (newsize as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
            > (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {};
    let ref mut fresh231 = luaM_reallocvector!(
        L, L -> stack, L -> stacksize, newsize, TValue
    );
    *fresh231 = luaM_realloc_(
        L,
        (*L).stack as *mut libc::c_void,
        ((*L).stacksize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
        (newsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    while lim < newsize {
        let ref mut fresh232 = setnilvalue!(L -> stack + lim);
        *fresh232 = setnilvalue!(L -> stack + lim);
        lim += 1;
    }
    (*L).stacksize = newsize;
    (*L)
        .stack_last = ((*L).stack)
        .offset(newsize as isize)
        .offset(-(EXTRA_STACK as isize));
    correctstack(L, oldstack);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_growstack(mut L: *mut lua_State, mut n: libc::c_int) {
    let mut size = (*L).stacksize;
    if size > LUAI_MAXSTACK {
        luaD_throw(L, LUA_ERRERR);
    } else {
        let mut needed = cast_int!(L -> top - L -> stack) + n + EXTRA_STACK;
        let mut newsize = 2 as libc::c_int * size;
        if newsize > LUAI_MAXSTACK {
            newsize = LUAI_MAXSTACK;
        }
        if newsize < needed {
            newsize = needed;
        }
        if newsize > LUAI_MAXSTACK {
            luaD_reallocstack(L, ERRORSTACKSIZE);
            luaG_runerror(L, b"stack overflow\0" as *const u8 as *const libc::c_char);
        } else {
            luaD_reallocstack(L, newsize);
        }
    };
}
unsafe extern "C" fn stackinuse(mut L: *mut lua_State) -> libc::c_int {
    let mut ci = 0 as *mut CallInfo;
    let mut lim = (*L).top;
    ci = (*L).ci;
    while !ci.is_null() {
        if lim < (*ci).top {
            lim = (*ci).top;
        }
        ci = (*ci).previous;
    }
    return cast_int!(lim - L -> stack) + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_shrinkstack(mut L: *mut lua_State) {
    let mut inuse = stackinuse(L);
    let mut goodsize = inuse + inuse / 8 as libc::c_int + 2 as libc::c_int * EXTRA_STACK;
    if goodsize > LUAI_MAXSTACK {
        goodsize = LUAI_MAXSTACK;
    }
    if (*L).stacksize > LUAI_MAXSTACK {
        luaE_freeCI(L);
    } else {
        luaE_shrinkCI(L);
    }
    if inuse <= LUAI_MAXSTACK - EXTRA_STACK && goodsize < (*L).stacksize {
        luaD_reallocstack(L, goodsize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaD_inctop(mut L: *mut lua_State) {
    if ((*L).stack_last).offset_from((*L).top) as libc::c_long <= luaD_checkstack!(L, 1)
    {
        luaD_checkstack!(L, 1)(L, 1 as libc::c_int);
    }
    (*L).top = ((*L).top).offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_hook(
    mut L: *mut lua_State,
    mut event: libc::c_int,
    mut line: libc::c_int,
) {
    let mut hook: lua_Hook = (*L).hook;
    if hook.is_some() && (*L).allowhook as libc::c_int != 0 {
        let mut ci = (*L).ci;
        let mut top = savestack!(L, L -> top);
        let mut ci_top = savestack!(L, ci -> top);
        let mut ar = lua_Debug {
            event: 0,
            name: 0 as *const libc::c_char,
            namewhat: 0 as *const libc::c_char,
            what: 0 as *const libc::c_char,
            source: 0 as *const libc::c_char,
            currentline: 0,
            linedefined: 0,
            lastlinedefined: 0,
            nups: 0,
            nparams: 0,
            isvararg: 0,
            istailcall: 0,
            short_src: [0; 60],
            i_ci: 0 as *mut CallInfo,
        };
        ar.event = event;
        ar.currentline = line;
        ar.i_ci = ci;
        if ((*L).stack_last).offset_from((*L).top) as libc::c_long
            <= luaD_checkstack!(L, LUA_MINSTACK)
        {
            luaD_checkstack!(L, LUA_MINSTACK)(L, 20 as libc::c_int);
        }
        (*ci).top = ((*L).top).offset(LUA_MINSTACK as isize);
        (*L).allowhook = 0 as libc::c_int as lu_byte;
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int | CIST_HOOKED)
            as libc::c_ushort;
        (Some(hook.expect("non-null function pointer")))
            .expect("non-null function pointer")(L, &mut ar);
        (*L).allowhook = 1 as libc::c_int as lu_byte;
        (*ci).top = restorestack!(L, ci_top);
        (*L).top = restorestack!(L, top);
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int & !CIST_HOOKED)
            as libc::c_ushort;
    }
}
unsafe extern "C" fn callhook(mut L: *mut lua_State, mut ci: *mut CallInfo) {
    let mut hook = LUA_HOOKCALL;
    (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
    if isLua!(ci -> previous) != 0
        && (*((*(*ci).previous).u.l.savedpc).offset(-(1 as libc::c_int as isize))
            >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 6 as libc::c_int)
                << 0 as libc::c_int) as OpCode as libc::c_uint
            == OP_TAILCALL as libc::c_int as libc::c_uint
    {
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int | CIST_TAIL)
            as libc::c_ushort;
        hook = LUA_HOOKTAILCALL;
    }
    luaD_hook(L, hook, -(1 as libc::c_int));
    (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(-1);
}
unsafe extern "C" fn adjust_varargs(
    mut L: *mut lua_State,
    mut p: *mut Proto,
    mut actual: libc::c_int,
) -> StkId {
    let mut i: libc::c_int = 0;
    let mut nfixargs = (*p).numparams as libc::c_int;
    let mut base = 0 as *mut TValue;
    let mut fixed = 0 as *mut TValue;
    fixed = ((*L).top).offset(-(actual as isize));
    base = (*L).top;
    i = 0 as libc::c_int;
    while i < nfixargs && i < actual {
        let mut io1 = setobjs2s!(L, L -> top ++, fixed + i);
        let ref mut fresh233 = setobjs2s!(L, L -> top ++, fixed + i);
        *fresh233 = setobjs2s!(L, L -> top ++, fixed + i);
        let ref mut fresh234 = setnilvalue!(fixed + i);
        *fresh234 = setnilvalue!(fixed + i);
        i += 1;
    }
    while i < nfixargs {
        let ref mut fresh235 = setnilvalue!(L -> top ++);
        *fresh235 = setnilvalue!(L -> top ++);
        i += 1;
    }
    return base;
}
unsafe extern "C" fn tryfuncTM(mut L: *mut lua_State, mut func: StkId) {
    let mut tm = luaT_gettmbyobj(L, func as *const TValue, TM_CALL);
    let mut p = 0 as *mut TValue;
    if ttisfunction!(tm) == 0 {
        luaG_typeerror(
            L,
            func as *const TValue,
            b"call\0" as *const u8 as *const libc::c_char,
        );
    }
    p = (*L).top;
    while p > func {
        let mut io1 = setobjs2s!(L, p, p - 1);
        let ref mut fresh236 = setobjs2s!(L, p, p - 1);
        *fresh236 = setobjs2s!(L, p, p - 1);
        p = p.offset(-1);
    }
    (*L).top = ((*L).top).offset(1);
    let mut io1_0 = setobj2s!(L, func, tm);
    let ref mut fresh237 = setobj2s!(L, func, tm);
    *fresh237 = setobj2s!(L, func, tm);
}
unsafe extern "C" fn moveresults(
    mut L: *mut lua_State,
    mut firstResult: *const TValue,
    mut res: StkId,
    mut nres: libc::c_int,
    mut wanted: libc::c_int,
) -> libc::c_int {
    match wanted {
        0 => {}
        1 => {
            if nres == 0 as libc::c_int {
                firstResult = &luaO_nilobject_;
            }
            let mut io1 = setobjs2s!(L, res, firstResult);
            let ref mut fresh238 = setobjs2s!(L, res, firstResult);
            *fresh238 = setobjs2s!(L, res, firstResult);
        }
        LUA_MULTRET => {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < nres {
                let mut io1_0 = setobjs2s!(L, res + i, firstResult + i);
                let ref mut fresh239 = setobjs2s!(L, res + i, firstResult + i);
                *fresh239 = setobjs2s!(L, res + i, firstResult + i);
                i += 1;
            }
            (*L).top = res.offset(nres as isize);
            return 0 as libc::c_int;
        }
        _ => {
            let mut i_0: libc::c_int = 0;
            if wanted <= nres {
                i_0 = 0 as libc::c_int;
                while i_0 < wanted {
                    let mut io1_1 = setobjs2s!(L, res + i, firstResult + i);
                    let ref mut fresh240 = setobjs2s!(L, res + i, firstResult + i);
                    *fresh240 = setobjs2s!(L, res + i, firstResult + i);
                    i_0 += 1;
                }
            } else {
                i_0 = 0 as libc::c_int;
                while i_0 < nres {
                    let mut io1_2 = setobjs2s!(L, res + i, firstResult + i);
                    let ref mut fresh241 = setobjs2s!(L, res + i, firstResult + i);
                    *fresh241 = setobjs2s!(L, res + i, firstResult + i);
                    i_0 += 1;
                }
                while i_0 < wanted {
                    let ref mut fresh242 = setnilvalue!(res + i);
                    *fresh242 = setnilvalue!(res + i);
                    i_0 += 1;
                }
            }
        }
    }
    (*L).top = res.offset(wanted as isize);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_poscall(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut firstResult: StkId,
    mut nres: libc::c_int,
) -> libc::c_int {
    let mut res = 0 as *mut TValue;
    let mut wanted = (*ci).nresults as libc::c_int;
    if (*L).hookmask & (LUA_MASKRET | LUA_MASKLINE) != 0 {
        if (*L).hookmask & LUA_MASKRET != 0 {
            let mut fr = savestack!(L, firstResult);
            luaD_hook(L, LUA_HOOKRET, -(1 as libc::c_int));
            firstResult = restorestack!(L, fr);
        }
        (*L).oldpc = (*(*ci).previous).u.l.savedpc;
    }
    res = (*ci).func;
    (*L).ci = (*ci).previous;
    return moveresults(L, firstResult as *const TValue, res, nres, wanted);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_precall(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nresults: libc::c_int,
) -> libc::c_int {
    let mut f: lua_CFunction = None;
    let mut ci = 0 as *mut CallInfo;
    match ttype!(func) {
        LUA_TCCL => {
            f = (*((*func).value_.gc as *mut GCUnion)).cl.c.f;
        }
        LUA_TLCF => {
            f = fvalue!(func);
        }
        LUA_TLCL => {
            let mut base = 0 as *mut TValue;
            let mut p = (*((*func).value_.gc as *mut GCUnion)).cl.l.p;
            let mut n_0 = cast_int!(L -> top - func) - 1 as libc::c_int;
            let mut fsize = (*p).maxstacksize as libc::c_int;
            if ((*L).stack_last).offset_from((*L).top) as libc::c_long
                <= checkstackp!(L, fsize, func)
            {
                let mut t___0 = (func as *mut libc::c_char)
                    .offset_from((*L).stack as *mut libc::c_char) as libc::c_long;
                if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                    luaC_step(L);
                }
                checkstackp!(L, fsize, func)(L, fsize);
                func = ((*L).stack as *mut libc::c_char).offset(t___0 as isize)
                    as *mut TValue;
            }
            if (*p).is_vararg != 0 {
                base = adjust_varargs(L, p, n_0);
            } else {
                while n_0 < (*p).numparams as libc::c_int {
                    let ref mut fresh243 = setnilvalue!(L -> top ++);
                    *fresh243 = setnilvalue!(L -> top ++);
                    n_0 += 1;
                }
                base = func.offset(1 as libc::c_int as isize);
            }
            ci = next_ci!(L);
            (*ci).nresults = nresults as libc::c_short;
            (*ci).func = func;
            (*ci).u.l.base = base;
            (*ci).top = base.offset(fsize as isize);
            (*L).top = (*ci).top;
            (*ci).u.l.savedpc = (*p).code;
            (*ci).callstatus = CIST_LUA as libc::c_ushort;
            if (*L).hookmask & LUA_MASKCALL != 0 {
                callhook(L, ci);
            }
            return 0 as libc::c_int;
        }
        _ => {
            if ((*L).stack_last).offset_from((*L).top) as libc::c_long
                <= checkstackp!(L, 1, func)
            {
                let mut t___1 = (func as *mut libc::c_char)
                    .offset_from((*L).stack as *mut libc::c_char) as libc::c_long;
                if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                    luaC_step(L);
                }
                checkstackp!(L, 1, func)(L, 1 as libc::c_int);
                func = ((*L).stack as *mut libc::c_char).offset(t___1 as isize)
                    as *mut TValue;
            }
            tryfuncTM(L, func);
            return luaD_precall(L, func, nresults);
        }
    }
    let mut n: libc::c_int = 0;
    if ((*L).stack_last).offset_from((*L).top) as libc::c_long
        <= checkstackp!(L, LUA_MINSTACK, func)
    {
        let mut t__ = (func as *mut libc::c_char)
            .offset_from((*L).stack as *mut libc::c_char) as libc::c_long;
        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
            luaC_step(L);
        }
        checkstackp!(L, LUA_MINSTACK, func)(L, 20 as libc::c_int);
        func = ((*L).stack as *mut libc::c_char).offset(t__ as isize) as *mut TValue;
    }
    ci = next_ci!(L);
    (*ci).nresults = nresults as libc::c_short;
    (*ci).func = func;
    (*ci).top = ((*L).top).offset(LUA_MINSTACK as isize);
    (*ci).callstatus = 0 as libc::c_int as libc::c_ushort;
    if (*L).hookmask & LUA_MASKCALL != 0 {
        luaD_hook(L, LUA_HOOKCALL, -(1 as libc::c_int));
    }
    n = (Some(f.expect("non-null function pointer")))
        .expect("non-null function pointer")(L);
    luaD_poscall(L, ci, ((*L).top).offset(-(n as isize)), n);
    return 1 as libc::c_int;
}
unsafe extern "C" fn stackerror(mut L: *mut lua_State) {
    if (*L).nCcalls as libc::c_int == LUAI_MAXCCALLS {
        luaG_runerror(L, b"C stack overflow\0" as *const u8 as *const libc::c_char);
    } else {
        if (*L).nCcalls as libc::c_int
            >= LUAI_MAXCCALLS + (LUAI_MAXCCALLS >> 3 as libc::c_int)
        {
            luaD_throw(L, LUA_ERRERR);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_call(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nResults: libc::c_int,
) {
    (*L).nCcalls = ((*L).nCcalls).wrapping_add(1);
    if (*L).nCcalls as libc::c_int >= LUAI_MAXCCALLS {
        stackerror(L);
    }
    if luaD_precall(L, func, nResults) == 0 {
        luaV_execute(L);
    }
    (*L).nCcalls = ((*L).nCcalls).wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_callnoyield(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nResults: libc::c_int,
) {
    (*L).nny = ((*L).nny).wrapping_add(1);
    luaD_call(L, func, nResults);
    (*L).nny = ((*L).nny).wrapping_sub(1);
}
unsafe extern "C" fn finishCcall(mut L: *mut lua_State, mut status: libc::c_int) {
    let mut ci = (*L).ci;
    let mut n: libc::c_int = 0;
    if (*ci).callstatus as libc::c_int & CIST_YPCALL != 0 {
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int & !CIST_YPCALL)
            as libc::c_ushort;
        (*L).errfunc = (*ci).u.c.old_errfunc;
    }
    if adjustresults!(L, ci -> nresults) != 0 {
        (*(*L).ci).top = (*L).top;
    }
    n = (Some(((*ci).u.c.k).expect("non-null function pointer")))
        .expect("non-null function pointer")(L, status, (*ci).u.c.ctx);
    luaD_poscall(L, ci, ((*L).top).offset(-(n as isize)), n);
}
unsafe extern "C" fn unroll(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    if !ud.is_null() {
        finishCcall(L, *(ud as *mut libc::c_int));
    }
    while (*L).ci != &mut (*L).base_ci as *mut CallInfo {
        if isLua!(L -> ci) == 0 {
            finishCcall(L, LUA_YIELD);
        } else {
            luaV_finishOp(L);
            luaV_execute(L);
        }
    }
}
unsafe extern "C" fn findpcall(mut L: *mut lua_State) -> *mut CallInfo {
    let mut ci = 0 as *mut CallInfo;
    ci = (*L).ci;
    while !ci.is_null() {
        if (*ci).callstatus as libc::c_int & CIST_YPCALL != 0 {
            return ci;
        }
        ci = (*ci).previous;
    }
    return NULL as *mut CallInfo;
}
unsafe extern "C" fn recover(
    mut L: *mut lua_State,
    mut status: libc::c_int,
) -> libc::c_int {
    let mut oldtop = 0 as *mut TValue;
    let mut ci = findpcall(L);
    if ci.is_null() {
        return 0 as libc::c_int;
    }
    oldtop = restorestack!(L, ci -> extra);
    luaF_close(L, oldtop);
    seterrorobj(L, status, oldtop);
    (*L).ci = ci;
    (*L).allowhook = getoah!(ci -> callstatus);
    (*L).nny = 0 as libc::c_int as libc::c_ushort;
    luaD_shrinkstack(L);
    (*L).errfunc = (*ci).u.c.old_errfunc;
    return 1 as libc::c_int;
}
unsafe extern "C" fn resume_error(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut narg: libc::c_int,
) -> libc::c_int {
    (*L).top = ((*L).top).offset(-(narg as isize));
    let mut io = (*L).top;
    let mut x_ = luaS_new(L, msg);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./ldo.c\0" as *const u8 as *const libc::c_char,
            606 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"int resume_error(lua_State *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    return LUA_ERRRUN;
}
unsafe extern "C" fn resume(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut n = *(cast!(int *, ud) as *mut libc::c_int);
    let mut firstArg = ((*L).top).offset(-(n as isize));
    let mut ci = (*L).ci;
    if (*L).status as libc::c_int == LUA_OK {
        if luaD_precall(L, firstArg.offset(-(1 as libc::c_int as isize)), LUA_MULTRET)
            == 0
        {
            luaV_execute(L);
        }
    } else {
        (*L).status = LUA_OK as lu_byte;
        (*ci).func = restorestack!(L, ci -> extra);
        if isLua!(ci) != 0 {
            luaV_execute(L);
        } else {
            if ((*ci).u.c.k).is_some() {
                n = (Some(((*ci).u.c.k).expect("non-null function pointer")))
                    .expect("non-null function pointer")(L, LUA_YIELD, (*ci).u.c.ctx);
                firstArg = ((*L).top).offset(-(n as isize));
            }
            luaD_poscall(L, ci, firstArg, n);
        }
        unroll(L, NULL as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_resume(
    mut L: *mut lua_State,
    mut from: *mut lua_State,
    mut nargs: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut oldnny = (*L).nny;
    if (*L).status as libc::c_int == LUA_OK {
        if (*L).ci != &mut (*L).base_ci as *mut CallInfo {
            return resume_error(
                L,
                b"cannot resume non-suspended coroutine\0" as *const u8
                    as *const libc::c_char,
                nargs,
            );
        }
    } else if (*L).status as libc::c_int != LUA_YIELD {
        return resume_error(
            L,
            b"cannot resume dead coroutine\0" as *const u8 as *const libc::c_char,
            nargs,
        )
    }
    (*L)
        .nCcalls = (if !from.is_null() {
        (*from).nCcalls as libc::c_int + 1 as libc::c_int
    } else {
        1 as libc::c_int
    }) as libc::c_ushort;
    if (*L).nCcalls as libc::c_int >= LUAI_MAXCCALLS {
        return resume_error(
            L,
            b"C stack overflow\0" as *const u8 as *const libc::c_char,
            nargs,
        );
    }
    (*L).nny = 0 as libc::c_int as libc::c_ushort;
    status = luaD_rawrunprotected(
        L,
        Some(resume as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
        &mut nargs as *mut libc::c_int as *mut libc::c_void,
    );
    if status == -(1 as libc::c_int) {
        status = LUA_ERRRUN;
    } else {
        while errorstatus!(status) != 0 && recover(L, status) != 0 {
            status = luaD_rawrunprotected(
                L,
                Some(
                    unroll
                        as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
                ),
                &mut status as *mut libc::c_int as *mut libc::c_void,
            );
        }
        if errorstatus!(status) != 0 {
            (*L).status = cast_byte!(status);
            seterrorobj(L, status, (*L).top);
            (*(*L).ci).top = (*L).top;
        }
    }
    (*L).nny = oldnny;
    (*L).nCcalls = ((*L).nCcalls).wrapping_sub(1);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isyieldable(mut L: *mut lua_State) -> libc::c_int {
    return ((*L).nny as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_yieldk(
    mut L: *mut lua_State,
    mut nresults: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> libc::c_int {
    let mut ci = (*L).ci;
    if (*L).nny as libc::c_int > 0 as libc::c_int {
        if L != (*G!(L)).mainthread {
            luaG_runerror(
                L,
                b"attempt to yield across a C-call boundary\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            luaG_runerror(
                L,
                b"attempt to yield from outside a coroutine\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    (*L).status = LUA_YIELD as lu_byte;
    (*ci).extra = savestack!(L, ci -> func);
    if isLua!(ci) != 0 {} else {
        (*ci).u.c.k = k;
        if ((*ci).u.c.k).is_some() {
            (*ci).u.c.ctx = ctx;
        }
        (*ci)
            .func = ((*L).top)
            .offset(-(nresults as isize))
            .offset(-(1 as libc::c_int as isize));
        luaD_throw(L, LUA_YIELD);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_pcall(
    mut L: *mut lua_State,
    mut func: Pfunc,
    mut u: *mut libc::c_void,
    mut old_top: ptrdiff_t,
    mut ef: ptrdiff_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut old_ci = (*L).ci;
    let mut old_allowhooks = (*L).allowhook;
    let mut old_nny = (*L).nny;
    let mut old_errfunc = (*L).errfunc;
    (*L).errfunc = ef;
    status = luaD_rawrunprotected(L, func, u);
    if status != LUA_OK {
        let mut oldtop = restorestack!(L, old_top);
        luaF_close(L, oldtop);
        seterrorobj(L, status, oldtop);
        (*L).ci = old_ci;
        (*L).allowhook = old_allowhooks;
        (*L).nny = old_nny;
        luaD_shrinkstack(L);
    }
    (*L).errfunc = old_errfunc;
    return status;
}
unsafe extern "C" fn checkmode(
    mut L: *mut lua_State,
    mut mode: *const libc::c_char,
    mut x: *const libc::c_char,
) {
    if !mode.is_null()
        && (strchr(mode, *x.offset(0 as libc::c_int as isize) as libc::c_int)).is_null()
    {
        luaO_pushfstring(
            L,
            b"attempt to load a %s chunk (mode is '%s')\0" as *const u8
                as *const libc::c_char,
            x,
            mode,
        );
        luaD_throw(L, LUA_ERRSYNTAX);
    }
}
unsafe extern "C" fn f_parser(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut cl = 0 as *mut LClosure;
    let mut p = cast!(struct SParser *, ud);
    let mut c = zgetc!(p -> z);
    if c
        == (*::core::mem::transmute::<
            &[u8; 5],
            &[libc::c_char; 5],
        >(b"\x1BLua\0"))[0 as libc::c_int as usize] as libc::c_int
    {
        checkmode(L, (*p).mode, b"binary\0" as *const u8 as *const libc::c_char);
        cl = luaU_undump(L, (*p).z, (*p).name);
    } else {
        checkmode(L, (*p).mode, b"text\0" as *const u8 as *const libc::c_char);
        cl = luaY_parser(L, (*p).z, &mut (*p).buff, &mut (*p).dyd, (*p).name, c);
    }
    luaF_initupvals(L, cl);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_protectedparser(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut p = SParser {
        z: 0 as *mut ZIO,
        buff: Mbuffer {
            buffer: 0 as *mut libc::c_char,
            n: 0,
            buffsize: 0,
        },
        dyd: Dyndata {
            actvar: C2RustUnnamed_7 {
                arr: 0 as *mut Vardesc,
                n: 0,
                size: 0,
            },
            gt: Labellist {
                arr: 0 as *mut Labeldesc,
                n: 0,
                size: 0,
            },
            label: Labellist {
                arr: 0 as *mut Labeldesc,
                n: 0,
                size: 0,
            },
        },
        mode: 0 as *const libc::c_char,
        name: 0 as *const libc::c_char,
    };
    let mut status: libc::c_int = 0;
    (*L).nny = ((*L).nny).wrapping_add(1);
    p.z = z;
    p.name = name;
    p.mode = mode;
    p.dyd.actvar.arr = NULL as *mut Vardesc;
    p.dyd.actvar.size = 0 as libc::c_int;
    p.dyd.gt.arr = NULL as *mut Labeldesc;
    p.dyd.gt.size = 0 as libc::c_int;
    p.dyd.label.arr = NULL as *mut Labeldesc;
    p.dyd.label.size = 0 as libc::c_int;
    let ref mut fresh244 = luaZ_initbuffer!(L, & p.buff);
    *fresh244 = NULL as *mut libc::c_char;
    status = luaD_pcall(
        L,
        Some(f_parser as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
        &mut p as *mut SParser as *mut libc::c_void,
        savestack!(L, L -> top),
        (*L).errfunc,
    );
    let ref mut fresh245 = luaZ_freebuffer!(L, & p.buff);
    *fresh245 = luaM_realloc_(
        L,
        p.buff.buffer as *mut libc::c_void,
        (p.buff.buffsize)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let ref mut fresh246 = luaZ_freebuffer!(L, & p.buff);
    *fresh246 = 0 as libc::c_int as size_t;
    luaM_freearray!(
        L, p.dyd.actvar.arr, p.dyd.actvar.size
    )(
        L,
        luaM_freearray!(L, p.dyd.actvar.arr, p.dyd.actvar.size),
        luaM_freearray!(L, p.dyd.actvar.arr, p.dyd.actvar.size),
        luaM_freearray!(L, p.dyd.actvar.arr, p.dyd.actvar.size),
    );
    luaM_freearray!(
        L, p.dyd.gt.arr, p.dyd.gt.size
    )(
        L,
        luaM_freearray!(L, p.dyd.gt.arr, p.dyd.gt.size),
        luaM_freearray!(L, p.dyd.gt.arr, p.dyd.gt.size),
        luaM_freearray!(L, p.dyd.gt.arr, p.dyd.gt.size),
    );
    luaM_freearray!(
        L, p.dyd.label.arr, p.dyd.label.size
    )(
        L,
        luaM_freearray!(L, p.dyd.label.arr, p.dyd.label.size),
        luaM_freearray!(L, p.dyd.label.arr, p.dyd.label.size),
        luaM_freearray!(L, p.dyd.label.arr, p.dyd.label.size),
    );
    (*L).nny = ((*L).nny).wrapping_sub(1);
    return status;
}
pub const MAXTAGLOOP: libc::c_int = 2000 as libc::c_int;
pub const NBM: libc::c_int = 53 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn luaV_tonumber_(
    mut obj: *const TValue,
    mut n: *mut lua_Number,
) -> libc::c_int {
    let mut v = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    if ttisinteger!(obj) != 0 {
        *n = (*obj).value_.i as lua_Number;
        return 1 as libc::c_int;
    } else if cvt2num!(obj) != 0
        && luaO_str2num(svalue!(obj), &mut v)
            == vslen!(obj).wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        *n = nvalue!(& v);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_tointeger(
    mut obj: *const TValue,
    mut p: *mut lua_Integer,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut v = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    's_66: {
        loop {
            if ttisfloat!(obj) != 0 {
                let mut n = fltvalue!(obj);
                let mut f = l_floor!(n);
                if n != f {
                    if mode == 0 as libc::c_int {
                        return 0 as libc::c_int
                    } else {
                        if mode > 1 as libc::c_int {
                            f += 1 as libc::c_int as libc::c_double;
                        }
                    }
                }
                return lua_numbertointeger!(f, p);
            } else if ttisinteger!(obj) != 0 {
                *p = ivalue!(obj);
                return 1 as libc::c_int;
            } else if cvt2num!(obj) != 0
                && luaO_str2num(svalue!(obj), &mut v)
                    == vslen!(obj).wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                obj = &mut v;
            } else {
                break 's_66;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn forlimit(
    mut obj: *const TValue,
    mut p: *mut lua_Integer,
    mut step: lua_Integer,
    mut stopnow: *mut libc::c_int,
) -> libc::c_int {
    *stopnow = 0 as libc::c_int;
    if luaV_tointeger(
        obj,
        p,
        if step < 0 as libc::c_int as libc::c_longlong {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        },
    ) == 0
    {
        let mut n: lua_Number = 0.;
        if tonumber!(obj, & n) == 0 {
            return 0 as libc::c_int;
        }
        if luai_numlt!(0, n) != 0 {
            *p = LUA_MAXINTEGER;
            if step < 0 as libc::c_int as libc::c_longlong {
                *stopnow = 1 as libc::c_int;
            }
        } else {
            *p = LUA_MININTEGER;
            if step >= 0 as libc::c_int as libc::c_longlong {
                *stopnow = 1 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishget(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut key: *mut TValue,
    mut val: StkId,
    mut slot: *const TValue,
) {
    let mut loop_0: libc::c_int = 0;
    let mut tm = 0 as *const TValue;
    loop_0 = 0 as libc::c_int;
    while loop_0 < MAXTAGLOOP {
        if slot.is_null() {
            tm = luaT_gettmbyobj(L, t, TM_INDEX);
            if ttisnil!(tm) != 0 {
                luaG_typeerror(L, t, b"index\0" as *const u8 as *const libc::c_char);
            }
        } else {
            tm = fasttm!(L, hvalue(t) -> metatable, TM_INDEX);
            if tm.is_null() {
                let ref mut fresh247 = setnilvalue!(val);
                *fresh247 = setnilvalue!(val);
                return;
            }
        }
        if ttisfunction!(tm) != 0 {
            luaT_callTM(L, tm, t, key, val, 1 as libc::c_int);
            return;
        }
        t = tm;
        if luaV_fastget!(L, t, key, slot, luaH_get) != 0 {
            let mut io1 = setobj2s!(L, val, slot);
            let ref mut fresh248 = setobj2s!(L, val, slot);
            *fresh248 = setobj2s!(L, val, slot);
            return;
        }
        loop_0 += 1;
    }
    luaG_runerror(
        L,
        b"'__index' chain too long; possible loop\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishset(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut key: *mut TValue,
    mut val: StkId,
    mut slot: *const TValue,
) {
    let mut loop_0: libc::c_int = 0;
    loop_0 = 0 as libc::c_int;
    while loop_0 < MAXTAGLOOP {
        let mut tm = 0 as *const TValue;
        if !slot.is_null() {
            let mut h: *mut Table = hvalue!(t);
            tm = fasttm!(L, h -> metatable, TM_NEWINDEX);
            if tm.is_null() {
                if slot == &luaO_nilobject_ as *const TValue {
                    slot = luaH_newkey(L, h, key);
                }
                let ref mut fresh249 = invalidateTMcache!(h);
                *fresh249 = invalidateTMcache!(h);
                if luaC_barrierback!(L, h, val) != 0 {} else {};
                return;
            }
        } else {
            tm = luaT_gettmbyobj(L, t, TM_NEWINDEX);
            if (*tm).tt_ == 0 as libc::c_int {
                luaG_typeerror(L, t, b"index\0" as *const u8 as *const libc::c_char);
            }
        }
        if ttisfunction!(tm) != 0 {
            luaT_callTM(L, tm, t, key, val, 0 as libc::c_int);
            return;
        }
        t = tm;
        if luaV_fastset!(L, t, key, slot, luaH_get, val) != 0 {
            return;
        }
        loop_0 += 1;
    }
    luaG_runerror(
        L,
        b"'__newindex' chain too long; possible loop\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn l_strcmp(
    mut ls: *const TString,
    mut rs: *const TString,
) -> libc::c_int {
    let mut l: *const libc::c_char = getstr!(ls);
    let mut ll = tsslen!(ls);
    let mut r: *const libc::c_char = getstr!(rs);
    let mut lr = tsslen!(rs);
    loop {
        let mut temp = strcoll(l, r);
        if temp != 0 as libc::c_int {
            return temp
        } else {
            let mut len = strlen(l);
            if len == lr {
                return if len == ll { 0 as libc::c_int } else { 1 as libc::c_int }
            } else {
                if len == ll {
                    return -(1 as libc::c_int);
                }
            }
            len = len.wrapping_add(1);
            l = l.offset(len as isize);
            ll = (ll as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
            r = r.offset(len as isize);
            lr = (lr as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
        }
    };
}
unsafe extern "C" fn LTintfloat(mut i: lua_Integer, mut f: lua_Number) -> libc::c_int {
    if l_intfitsf!(i) == 0 {
        if f >= -cast_num!(LUA_MININTEGER) {
            return 1 as libc::c_int
        } else if f > cast_num!(LUA_MININTEGER) {
            return (i < cast!(lua_Integer, f)) as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    }
    return luai_numlt!(cast_num(i), f);
}
unsafe extern "C" fn LEintfloat(mut i: lua_Integer, mut f: lua_Number) -> libc::c_int {
    if l_intfitsf!(i) == 0 {
        if f >= -cast_num!(LUA_MININTEGER) {
            return 1 as libc::c_int
        } else if f >= cast_num!(LUA_MININTEGER) {
            return (i <= cast!(lua_Integer, f)) as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    }
    return luai_numle!(cast_num(i), f);
}
unsafe extern "C" fn LTnum(mut l: *const TValue, mut r: *const TValue) -> libc::c_int {
    if ttisinteger!(l) != 0 {
        let mut li = ivalue!(l);
        if ttisinteger!(r) != 0 {
            return (li < ivalue!(r)) as libc::c_int
        } else {
            return LTintfloat(li, fltvalue!(r))
        }
    } else {
        let mut lf = fltvalue!(l);
        if ttisfloat!(r) != 0 {
            return (lf < (*r).value_.n) as libc::c_int
        } else if !(lf == lf) {
            return 0 as libc::c_int
        } else {
            return (LEintfloat(ivalue!(r), lf) == 0) as libc::c_int
        }
    };
}
unsafe extern "C" fn LEnum(mut l: *const TValue, mut r: *const TValue) -> libc::c_int {
    if ttisinteger!(l) != 0 {
        let mut li = ivalue!(l);
        if ttisinteger!(r) != 0 {
            return (li <= ivalue!(r)) as libc::c_int
        } else {
            return LEintfloat(li, fltvalue!(r))
        }
    } else {
        let mut lf = fltvalue!(l);
        if ttisfloat!(r) != 0 {
            return (lf <= (*r).value_.n) as libc::c_int
        } else if !(lf == lf) {
            return 0 as libc::c_int
        } else {
            return (LTintfloat(ivalue!(r), lf) == 0) as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_lessthan(
    mut L: *mut lua_State,
    mut l: *const TValue,
    mut r: *const TValue,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    if ttisnumber!(l) != 0 && ttisnumber!(r) != 0 {
        return LTnum(l, r)
    } else {
        if ttisstring!(l) != 0 && ttisstring!(r) != 0 {
            return (l_strcmp(tsvalue!(l), tsvalue!(r)) < 0 as libc::c_int) as libc::c_int
        } else {
            res = luaT_callorderTM(L, l, r, TM_LT);
            if res < 0 as libc::c_int {
                luaG_ordererror(L, l, r);
            }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_lessequal(
    mut L: *mut lua_State,
    mut l: *const TValue,
    mut r: *const TValue,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    if ttisnumber!(l) != 0 && ttisnumber!(r) != 0 {
        return LEnum(l, r)
    } else if ttisstring!(l) != 0 && ttisstring!(r) != 0 {
        return (l_strcmp(tsvalue!(l), tsvalue!(r)) <= 0 as libc::c_int) as libc::c_int
    } else {
        res = luaT_callorderTM(L, l, r, TM_LE);
        if res >= 0 as libc::c_int {
            return res
        } else {
            (*(*L).ci)
                .callstatus = ((*(*L).ci).callstatus as libc::c_int | CIST_LEQ)
                as libc::c_ushort;
            res = luaT_callorderTM(L, r, l, TM_LT);
            (*(*L).ci)
                .callstatus = ((*(*L).ci).callstatus as libc::c_int ^ CIST_LEQ)
                as libc::c_ushort;
            if res < 0 as libc::c_int {
                luaG_ordererror(L, l, r);
            }
            return (res == 0) as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_equalobj(
    mut L: *mut lua_State,
    mut t1: *const TValue,
    mut t2: *const TValue,
) -> libc::c_int {
    let mut tm = 0 as *const TValue;
    if ttype!(t1) != ttype!(t2) {
        if ttnov!(t1) != ttnov!(t2) || ttnov!(t1) != LUA_TNUMBER {
            return 0 as libc::c_int
        } else {
            let mut i1: lua_Integer = 0;
            let mut i2: lua_Integer = 0;
            return (tointeger!(t1, & i1) != 0 && tointeger!(t2, & i2) != 0 && i1 == i2)
                as libc::c_int;
        }
    }
    match ttype!(t1) {
        LUA_TNIL => return 1 as libc::c_int,
        LUA_TNUMINT => return (ivalue!(t1) == ivalue!(t2)) as libc::c_int,
        LUA_TNUMFLT => return ((*t1).value_.n == (*t2).value_.n) as libc::c_int,
        LUA_TBOOLEAN => return (bvalue!(t1) == bvalue!(t2)) as libc::c_int,
        LUA_TLIGHTUSERDATA => return (pvalue!(t1) == pvalue!(t2)) as libc::c_int,
        LUA_TLCF => return (fvalue!(t1) == fvalue!(t2)) as libc::c_int,
        LUA_TSHRSTR => {
            return (&mut (*((*t1).value_.gc as *mut GCUnion)).ts as *mut TString
                == &mut (*((*t2).value_.gc as *mut GCUnion)).ts as *mut TString)
                as libc::c_int;
        }
        LUA_TLNGSTR => return luaS_eqlngstr(tsvalue!(t1), tsvalue!(t2)),
        LUA_TUSERDATA => {
            if uvalue!(t1) == uvalue!(t2) {
                return 1 as libc::c_int
            } else {
                if L.is_null() {
                    return 0 as libc::c_int;
                }
            }
            tm = fasttm!(L, uvalue(t1) -> metatable, TM_EQ);
            if tm.is_null() {
                tm = fasttm!(L, uvalue(t2) -> metatable, TM_EQ);
            }
        }
        LUA_TTABLE => {
            if hvalue!(t1) == hvalue!(t2) {
                return 1 as libc::c_int
            } else {
                if L.is_null() {
                    return 0 as libc::c_int;
                }
            }
            tm = fasttm!(L, hvalue(t1) -> metatable, TM_EQ);
            if tm.is_null() {
                tm = fasttm!(L, hvalue(t2) -> metatable, TM_EQ);
            }
        }
        _ => return (gcvalue!(t1) == gcvalue!(t2)) as libc::c_int,
    }
    if tm.is_null() {
        return 0 as libc::c_int;
    }
    luaT_callTM(L, tm, t1, t2, (*L).top, 1 as libc::c_int);
    return (l_isfalse!(L -> top) == 0) as libc::c_int;
}
unsafe extern "C" fn copy2buff(
    mut top: StkId,
    mut n: libc::c_int,
    mut buff: *mut libc::c_char,
) {
    let mut tl = 0 as libc::c_int as size_t;
    loop {
        let mut l = vslen!(top - n);
        memcpy(
            buff.offset(tl as isize) as *mut libc::c_void,
            svalue!(top - n),
            l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        tl = (tl as libc::c_ulong).wrapping_add(l) as size_t as size_t;
        n -= 1;
        if !(n > 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_concat(mut L: *mut lua_State, mut total: libc::c_int) {
    loop {
        let mut top = (*L).top;
        let mut n = 2 as libc::c_int;
        if !(ttisstring!(top - 2) != 0 || cvt2str!(top - 2) != 0)
            || tostring!(L, top - 1) == 0
        {
            luaT_trybinTM(
                L,
                top.offset(-(2 as libc::c_int as isize)) as *const TValue,
                top.offset(-(1 as libc::c_int as isize)) as *const TValue,
                top.offset(-(2 as libc::c_int as isize)),
                TM_CONCAT,
            );
        } else if isemptystr!(top - 1) != 0 {
            ((*top.offset(-(2 as libc::c_int as isize))).tt_ & 0xf as libc::c_int
                == 4 as libc::c_int
                || (*top.offset(-(2 as libc::c_int as isize))).tt_ & 0xf as libc::c_int
                    == 3 as libc::c_int
                    && {
                        luaO_tostring(L, top.offset(-(2 as libc::c_int as isize)));
                        1 as libc::c_int != 0
                    }) as libc::c_int;
        } else if isemptystr!(top - 2) != 0 {
            let mut io1 = setobjs2s!(L, top - 2, top - 1);
            let ref mut fresh250 = setobjs2s!(L, top - 2, top - 1);
            *fresh250 = setobjs2s!(L, top - 2, top - 1);
        } else {
            let mut tl = vslen!(top - 1);
            let mut ts = 0 as *mut TString;
            n = 1 as libc::c_int;
            while n < total && tostring!(L, top - n - 1) != 0 {
                let mut l = vslen!(top - n - 1);
                if l
                    >= (if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                        < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
                    {
                        MAX_SIZET
                    } else {
                        9223372036854775807 as libc::c_longlong as size_t
                    })
                        .wrapping_div(
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(tl)
                {
                    luaG_runerror(
                        L,
                        b"string length overflow\0" as *const u8 as *const libc::c_char,
                    );
                }
                tl = (tl as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                n += 1;
            }
            if tl <= LUAI_MAXSHORTLEN as libc::c_ulong {
                let mut buff: [libc::c_char; 40] = [0; 40];
                copy2buff(top, n, buff.as_mut_ptr());
                ts = luaS_newlstr(L, buff.as_mut_ptr(), tl);
            } else {
                ts = luaS_createlngstrobj(L, tl);
                copy2buff(top, n, getstr!(ts));
            }
            let mut io = setsvalue2s!(L, top - n, ts);
            let mut x_ = setsvalue2s!(L, top - n, ts);
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        }
        total -= n - 1 as libc::c_int;
        (*L).top = ((*L).top).offset(-((n - 1 as libc::c_int) as isize));
        if !(total > 1 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_objlen(
    mut L: *mut lua_State,
    mut ra: StkId,
    mut rb: *const TValue,
) {
    let mut tm = 0 as *const TValue;
    match ttype!(rb) {
        LUA_TTABLE => {
            let mut h: *mut Table = hvalue!(rb);
            tm = fasttm!(L, h -> metatable, TM_LEN);
            if tm.is_null() {
                let mut io = ra;
                (*io).value_.i = luaH_getn(h) as lua_Integer;
                (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
                return;
            }
        }
        LUA_TSHRSTR => {
            let mut io_0 = setivalue!(ra, tsvalue(rb) -> shrlen);
            (*io_0).value_.i = setivalue!(ra, tsvalue(rb) -> shrlen);
            (*io_0).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
            return;
        }
        LUA_TLNGSTR => {
            let mut io_1 = setivalue!(ra, tsvalue(rb) -> u.lnglen);
            (*io_1).value_.i = setivalue!(ra, tsvalue(rb) -> u.lnglen);
            (*io_1).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
            return;
        }
        _ => {
            tm = luaT_gettmbyobj(L, rb, TM_LEN);
            if ttisnil!(tm) != 0 {
                luaG_typeerror(
                    L,
                    rb,
                    b"get length of\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    luaT_callTM(L, tm, rb, rb, ra, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn luaV_div(
    mut L: *mut lua_State,
    mut m: lua_Integer,
    mut n: lua_Integer,
) -> lua_Integer {
    if l_castS2U!(n).wrapping_add(1 as libc::c_uint as libc::c_ulonglong)
        <= 1 as libc::c_uint as libc::c_ulonglong
    {
        if n == 0 as libc::c_int as libc::c_longlong {
            luaG_runerror(
                L,
                b"attempt to divide by zero\0" as *const u8 as *const libc::c_char,
            );
        }
        return intop!(-, 0, m);
    } else {
        let mut q = m / n;
        if m ^ n < 0 as libc::c_int as libc::c_longlong
            && m % n != 0 as libc::c_int as libc::c_longlong
        {
            q -= 1 as libc::c_int as libc::c_longlong;
        }
        return q;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_mod(
    mut L: *mut lua_State,
    mut m: lua_Integer,
    mut n: lua_Integer,
) -> lua_Integer {
    if l_castS2U!(n).wrapping_add(1 as libc::c_uint as libc::c_ulonglong)
        <= 1 as libc::c_uint as libc::c_ulonglong
    {
        if n == 0 as libc::c_int as libc::c_longlong {
            luaG_runerror(
                L,
                b"attempt to perform 'n%%0'\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int as lua_Integer;
    } else {
        let mut r = m % n;
        if r != 0 as libc::c_int as libc::c_longlong
            && m ^ n < 0 as libc::c_int as libc::c_longlong
        {
            r += n;
        }
        return r;
    };
}
pub const NBITS: libc::c_ulong = (::core::mem::size_of::<lua_Integer>() as libc::c_ulong)
    .wrapping_mul(8 as libc::c_int as libc::c_ulong);
#[no_mangle]
pub unsafe extern "C" fn luaV_shiftl(
    mut x: lua_Integer,
    mut y: lua_Integer,
) -> lua_Integer {
    if y < 0 as libc::c_int as libc::c_longlong {
        if y <= -(NBITS as libc::c_int) as libc::c_longlong {
            return 0 as libc::c_int as lua_Integer
        } else {
            return intop!(>>, x, - y)
        }
    } else if y >= NBITS as libc::c_longlong {
        return 0 as libc::c_int as lua_Integer
    } else {
        return intop!(<<, x, y)
    };
}
unsafe extern "C" fn getcached(
    mut p: *mut Proto,
    mut encup: *mut *mut UpVal,
    mut base: StkId,
) -> *mut LClosure {
    let mut c = (*p).cache;
    if !c.is_null() {
        let mut nup = (*p).sizeupvalues;
        let mut uv = (*p).upvalues;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < nup {
            let mut v = if (*uv.offset(i as isize)).instack as libc::c_int != 0 {
                base.offset((*uv.offset(i as isize)).idx as libc::c_int as isize)
            } else {
                (**encup.offset((*uv.offset(i as isize)).idx as isize)).v
            };
            if (**((*c).upvals).as_mut_ptr().offset(i as isize)).v != v {
                return NULL as *mut LClosure;
            }
            i += 1;
        }
    }
    return c;
}
unsafe extern "C" fn pushclosure(
    mut L: *mut lua_State,
    mut p: *mut Proto,
    mut encup: *mut *mut UpVal,
    mut base: StkId,
    mut ra: StkId,
) {
    let mut nup = (*p).sizeupvalues;
    let mut uv = (*p).upvalues;
    let mut i: libc::c_int = 0;
    let mut ncl = luaF_newLclosure(L, nup);
    (*ncl).p = p;
    let mut io = setclLvalue!(L, ra, ncl);
    let mut x_ = setclLvalue!(L, ra, ncl);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nup {
        if (*uv.offset(i as isize)).instack != 0 {
            let ref mut fresh251 = *((*ncl).upvals).as_mut_ptr().offset(i as isize);
            *fresh251 = luaF_findupval(
                L,
                base.offset((*uv.offset(i as isize)).idx as libc::c_int as isize),
            );
        } else {
            let ref mut fresh252 = *((*ncl).upvals).as_mut_ptr().offset(i as isize);
            *fresh252 = *encup.offset((*uv.offset(i as isize)).idx as isize);
        }
        let ref mut fresh253 = (**((*ncl).upvals).as_mut_ptr().offset(i as isize))
            .refcount;
        *fresh253 = (*fresh253).wrapping_add(1);
        i += 1;
    }
    if isblack!(p) == 0 {
        (*p).cache = ncl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishOp(mut L: *mut lua_State) {
    let mut ci = (*L).ci;
    let mut base = (*ci).u.l.base;
    let mut inst = *((*ci).u.l.savedpc).offset(-(1 as libc::c_int as isize));
    let mut op = GET_OPCODE!(inst);
    match op as libc::c_uint {
        13 | 14 | 15 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 16 | 17 | 25 | 26 | 28 | 6 | 7
        | 12 => {
            let mut io1 = setobjs2s!(L, base + GETARG_A(inst), -- L -> top);
            let ref mut fresh254 = setobjs2s!(L, base + GETARG_A(inst), -- L -> top);
            *fresh254 = setobjs2s!(L, base + GETARG_A(inst), -- L -> top);
        }
        33 | 32 | 31 => {
            let mut res = (l_isfalse!(L -> top - 1) == 0) as libc::c_int;
            (*L).top = ((*L).top).offset(-1);
            if (*ci).callstatus as libc::c_int & CIST_LEQ != 0 {
                (*ci)
                    .callstatus = ((*ci).callstatus as libc::c_int ^ CIST_LEQ)
                    as libc::c_ushort;
                res = (res == 0) as libc::c_int;
            }
            if res != GETARG_A!(inst) {
                (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
            }
        }
        29 => {
            let mut top = ((*L).top).offset(-(1 as libc::c_int as isize));
            let mut b = GETARG_B!(inst);
            let mut total = top
                .offset(-(1 as libc::c_int as isize))
                .offset_from(base.offset(b as isize)) as libc::c_long as libc::c_int;
            let mut io1_0 = setobj2s!(L, top - 2, top);
            let ref mut fresh255 = setobj2s!(L, top - 2, top);
            *fresh255 = setobj2s!(L, top - 2, top);
            if total > 1 as libc::c_int {
                (*L).top = top.offset(-(1 as libc::c_int as isize));
                luaV_concat(L, total);
            }
            let mut io1_1 = setobj2s!(L, ci -> u.l.base + GETARG_A(inst), L -> top - 1);
            let ref mut fresh256 = setobj2s!(
                L, ci -> u.l.base + GETARG_A(inst), L -> top - 1
            );
            *fresh256 = setobj2s!(L, ci -> u.l.base + GETARG_A(inst), L -> top - 1);
            (*L).top = (*ci).top;
        }
        41 => {
            (*L).top = (*ci).top;
        }
        36 => {
            if GETARG_C!(inst) - 1 as libc::c_int >= 0 as libc::c_int {
                (*L).top = (*ci).top;
            }
        }
        37 | 8 | 10 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_execute(mut L: *mut lua_State) {
    let mut i: Instruction = 0;
    let mut ra: StkId = 0 as *mut TValue;
    let mut b_5: libc::c_int = 0;
    let mut ci = (*L).ci;
    let mut cl = 0 as *mut LClosure;
    let mut k = 0 as *mut TValue;
    let mut base = 0 as *mut TValue;
    (*ci).callstatus = ((*ci).callstatus as libc::c_int | CIST_FRESH) as libc::c_ushort;
    's_16: loop {
        cl = clLvalue!(ci -> func);
        k = (*(*cl).p).k;
        base = (*ci).u.l.base;
        loop {
            i = 0;
            ra = 0 as *mut TValue;
            let ref mut fresh257 = vmfetch!();
            *fresh257 = vmfetch!();
            if vmfetch!() != 0 {
                luaG_traceexec(L);
                base = (*ci).u.l.base;
            }
            let ref mut fresh258 = vmfetch!();
            *fresh258 = base
                .offset(
                    (i >> 0 as libc::c_int + 6 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                            << 0 as libc::c_int) as libc::c_int as isize,
                );
            match (i >> 0 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 6 as libc::c_int)
                    << 0 as libc::c_int) as OpCode as libc::c_uint
            {
                0 => {
                    let mut io1 = ra;
                    *io1 = *base
                        .offset(
                            (i
                                >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    + 9 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int as isize,
                        );
                    continue;
                }
                1 => {
                    let mut rb = k.offset(GETARG_Bx!(i) as isize);
                    let mut io1_0 = setobj2s!(L, ra, rb);
                    let ref mut fresh259 = setobj2s!(L, ra, rb);
                    *fresh259 = setobj2s!(L, ra, rb);
                    continue;
                }
                2 => {
                    let mut rb_0 = 0 as *mut TValue;
                    rb_0 = k.offset(GETARG_Ax!(* ci -> u.l.savedpc ++) as isize);
                    let mut io1_1 = setobj2s!(L, ra, rb);
                    let ref mut fresh260 = setobj2s!(L, ra, rb);
                    *fresh260 = setobj2s!(L, ra, rb);
                    continue;
                }
                3 => {
                    let mut io = ra;
                    (*io)
                        .value_
                        .b = (i
                        >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                            + 9 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                            << 0 as libc::c_int) as libc::c_int;
                    (*io).tt_ = 1 as libc::c_int;
                    if GETARG_C!(i) != 0 {
                        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
                    }
                    continue;
                }
                4 => {
                    let mut b = GETARG_B!(i);
                    loop {
                        let ref mut fresh261 = setnilvalue!(ra ++);
                        *fresh261 = setnilvalue!(ra ++);
                        let fresh262 = b;
                        b = b - 1;
                        if !(fresh262 != 0) {
                            break;
                        }
                    }
                    continue;
                }
                5 => {
                    let mut b_0 = GETARG_B!(i);
                    let mut io1_2 = setobj2s!(L, ra, cl -> upvals[b] -> v);
                    let ref mut fresh263 = setobj2s!(L, ra, cl -> upvals[b] -> v);
                    *fresh263 = setobj2s!(L, ra, cl -> upvals[b] -> v);
                    continue;
                }
                6 => {
                    let mut upval = (**((*cl).upvals)
                        .as_mut_ptr()
                        .offset(GETARG_B!(i) as isize))
                        .v;
                    let mut rc = RKC!(i);
                    let mut slot = 0 as *const TValue;
                    if if !((*upval).tt_
                        == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
                    {
                        slot = NULL as *const TValue;
                        0 as libc::c_int
                    } else {
                        slot = luaH_get(
                            &mut (*((*upval).value_.gc as *mut GCUnion)).h,
                            rc,
                        );
                        !((*slot).tt_ == 0 as libc::c_int) as libc::c_int
                    } != 0
                    {
                        let mut io1_3 = ra;
                        *io1_3 = *slot;
                    } else {
                        luaV_finishget(L, upval, rc, ra, slot);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                7 => {
                    let mut rb_1 = RB!(i);
                    let mut rc_0 = RKC!(i);
                    let mut slot_0 = 0 as *const TValue;
                    if if !((*rb_1).tt_
                        == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
                    {
                        slot_0 = NULL as *const TValue;
                        0 as libc::c_int
                    } else {
                        slot_0 = luaH_get(
                            &mut (*((*rb_1).value_.gc as *mut GCUnion)).h,
                            rc_0,
                        );
                        !((*slot_0).tt_ == 0 as libc::c_int) as libc::c_int
                    } != 0
                    {
                        let mut io1_4 = ra;
                        *io1_4 = *slot_0;
                    } else {
                        luaV_finishget(L, rb_1 as *const TValue, rc_0, ra, slot_0);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                8 => {
                    let mut upval_0 = (**((*cl).upvals)
                        .as_mut_ptr()
                        .offset(GETARG_A!(i) as isize))
                        .v;
                    let mut rb_2 = RKB!(i);
                    let mut rc_1 = RKC!(i);
                    let mut slot_1 = 0 as *const TValue;
                    if if !((*upval_0).tt_
                        == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
                    {
                        slot_1 = NULL as *const TValue;
                        0 as libc::c_int
                    } else {
                        slot_1 = luaH_get(
                            &mut (*((*upval_0).value_.gc as *mut GCUnion)).h,
                            rb_2,
                        );
                        (if (*slot_1).tt_ == 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            if (*rc_1).tt_ & BIT_ISCOLLECTABLE != 0
                                && (*((*upval_0).value_.gc as *mut GCUnion)).h.marked
                                    as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0
                                && (*(*rc_1).value_.gc).marked as libc::c_int
                                    & ((1 as libc::c_int) << 0 as libc::c_int
                                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
                            {
                                luaC_barrierback_(
                                    L,
                                    &mut (*((*upval_0).value_.gc as *mut GCUnion)).h,
                                );
                            } else {};
                            *(slot_1 as *mut TValue) = *rc_1;
                            1 as libc::c_int
                        })
                    } == 0
                    {
                        luaV_finishset(L, upval_0, rb_2, rc_1, slot_1);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                9 => {
                    let mut uv = *((*cl).upvals)
                        .as_mut_ptr()
                        .offset(GETARG_B!(i) as isize);
                    let mut io1_5 = setobj!(L, uv -> v, ra);
                    let ref mut fresh264 = setobj!(L, uv -> v, ra);
                    *fresh264 = setobj!(L, uv -> v, ra);
                    if luaC_upvalbarrier!(L, uv) != 0 {} else {};
                    continue;
                }
                10 => {
                    let mut rb_3 = RKB!(i);
                    let mut rc_2 = RKC!(i);
                    let mut slot_2 = 0 as *const TValue;
                    if if !((*ra).tt_
                        == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
                    {
                        slot_2 = NULL as *const TValue;
                        0 as libc::c_int
                    } else {
                        slot_2 = luaH_get(
                            &mut (*((*ra).value_.gc as *mut GCUnion)).h,
                            rb_3,
                        );
                        (if (*slot_2).tt_ == 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            if (*rc_2).tt_ & BIT_ISCOLLECTABLE != 0
                                && (*((*ra).value_.gc as *mut GCUnion)).h.marked
                                    as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0
                                && (*(*rc_2).value_.gc).marked as libc::c_int
                                    & ((1 as libc::c_int) << 0 as libc::c_int
                                        | (1 as libc::c_int) << 1 as libc::c_int) != 0
                            {
                                luaC_barrierback_(
                                    L,
                                    &mut (*((*ra).value_.gc as *mut GCUnion)).h,
                                );
                            } else {};
                            *(slot_2 as *mut TValue) = *rc_2;
                            1 as libc::c_int
                        })
                    } == 0
                    {
                        luaV_finishset(L, ra as *const TValue, rb_3, rc_2, slot_2);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                11 => {
                    let mut b_1 = GETARG_B!(i);
                    let mut c = GETARG_C!(i);
                    let mut t = luaH_new(L);
                    let mut io_0 = sethvalue!(L, ra, t);
                    let mut x_ = sethvalue!(L, ra, t);
                    (*io_0).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
                    (*io_0)
                        .tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
                    if b_1 != 0 as libc::c_int || c != 0 as libc::c_int {
                        luaH_resize(
                            L,
                            t,
                            luaO_fb2int(b_1) as libc::c_uint,
                            luaO_fb2int(c) as libc::c_uint,
                        );
                    }
                    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                        (*L).top = ra.offset(1 as libc::c_int as isize);
                        luaC_step(L);
                        (*L).top = (*ci).top;
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                12 => {
                    let mut aux = 0 as *const TValue;
                    let mut rb_4 = RB!(i);
                    let mut rc_3 = RKC!(i);
                    let mut key: *mut TString = tsvalue!(rc);
                    let mut io1_6 = setobjs2s!(L, ra + 1, rb);
                    let ref mut fresh265 = setobjs2s!(L, ra + 1, rb);
                    *fresh265 = setobjs2s!(L, ra + 1, rb);
                    if luaV_fastget!(L, rb, key, aux, luaH_getstr) != 0 {
                        let mut io1_7 = setobj2s!(L, ra, aux);
                        let ref mut fresh266 = setobj2s!(L, ra, aux);
                        *fresh266 = setobj2s!(L, ra, aux);
                    } else {
                        luaV_finishget(L, rb_4 as *const TValue, rc_3, ra, aux);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                13 => {
                    let mut rb_5 = RKB!(i);
                    let mut rc_4 = RKC!(i);
                    let mut nb: lua_Number = 0.;
                    let mut nc: lua_Number = 0.;
                    if ttisinteger!(rb) != 0 && ttisinteger!(rc) != 0 {
                        let mut ib = ivalue!(rb);
                        let mut ic = ivalue!(rc);
                        let mut io_1 = ra;
                        (*io_1)
                            .value_
                            .i = (ib as lua_Unsigned).wrapping_add(ic as lua_Unsigned)
                            as lua_Integer;
                        (*io_1)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else if tonumber!(rb, & nb) != 0 && tonumber!(rc, & nc) != 0 {
                        let mut io_2 = ra;
                        (*io_2).value_.n = nb + nc;
                        (*io_2)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_5, rc_4, ra, TM_ADD);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                14 => {
                    let mut rb_6 = RKB!(i);
                    let mut rc_5 = RKC!(i);
                    let mut nb_0: lua_Number = 0.;
                    let mut nc_0: lua_Number = 0.;
                    if ttisinteger!(rb) != 0 && ttisinteger!(rc) != 0 {
                        let mut ib_0 = ivalue!(rb);
                        let mut ic_0 = ivalue!(rc);
                        let mut io_3 = ra;
                        (*io_3)
                            .value_
                            .i = (ib_0 as lua_Unsigned)
                            .wrapping_sub(ic_0 as lua_Unsigned) as lua_Integer;
                        (*io_3)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else if tonumber!(rb, & nb) != 0 && tonumber!(rc, & nc) != 0 {
                        let mut io_4 = ra;
                        (*io_4).value_.n = nb_0 - nc_0;
                        (*io_4)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_6, rc_5, ra, TM_SUB);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                15 => {
                    let mut rb_7 = RKB!(i);
                    let mut rc_6 = RKC!(i);
                    let mut nb_1: lua_Number = 0.;
                    let mut nc_1: lua_Number = 0.;
                    if ttisinteger!(rb) != 0 && ttisinteger!(rc) != 0 {
                        let mut ib_1 = ivalue!(rb);
                        let mut ic_1 = ivalue!(rc);
                        let mut io_5 = ra;
                        (*io_5)
                            .value_
                            .i = (ib_1 as lua_Unsigned)
                            .wrapping_mul(ic_1 as lua_Unsigned) as lua_Integer;
                        (*io_5)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else if tonumber!(rb, & nb) != 0 && tonumber!(rc, & nc) != 0 {
                        let mut io_6 = ra;
                        (*io_6).value_.n = nb_1 * nc_1;
                        (*io_6)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_7, rc_6, ra, TM_MUL);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                18 => {
                    let mut rb_8 = RKB!(i);
                    let mut rc_7 = RKC!(i);
                    let mut nb_2: lua_Number = 0.;
                    let mut nc_2: lua_Number = 0.;
                    if tonumber!(rb, & nb) != 0 && tonumber!(rc, & nc) != 0 {
                        let mut io_7 = ra;
                        (*io_7).value_.n = nb_2 / nc_2;
                        (*io_7)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_8, rc_7, ra, TM_DIV);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                20 => {
                    let mut rb_9 = RKB!(i);
                    let mut rc_8 = RKC!(i);
                    let mut ib_2: lua_Integer = 0;
                    let mut ic_2: lua_Integer = 0;
                    if tointeger!(rb, & ib) != 0 && tointeger!(rc, & ic) != 0 {
                        let mut io_8 = ra;
                        (*io_8)
                            .value_
                            .i = (ib_2 as lua_Unsigned & ic_2 as lua_Unsigned)
                            as lua_Integer;
                        (*io_8)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_9, rc_8, ra, TM_BAND);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                21 => {
                    let mut rb_10 = RKB!(i);
                    let mut rc_9 = RKC!(i);
                    let mut ib_3: lua_Integer = 0;
                    let mut ic_3: lua_Integer = 0;
                    if tointeger!(rb, & ib) != 0 && tointeger!(rc, & ic) != 0 {
                        let mut io_9 = ra;
                        (*io_9)
                            .value_
                            .i = (ib_3 as lua_Unsigned | ic_3 as lua_Unsigned)
                            as lua_Integer;
                        (*io_9)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_10, rc_9, ra, TM_BOR);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                22 => {
                    let mut rb_11 = RKB!(i);
                    let mut rc_10 = RKC!(i);
                    let mut ib_4: lua_Integer = 0;
                    let mut ic_4: lua_Integer = 0;
                    if tointeger!(rb, & ib) != 0 && tointeger!(rc, & ic) != 0 {
                        let mut io_10 = ra;
                        (*io_10)
                            .value_
                            .i = (ib_4 as lua_Unsigned ^ ic_4 as lua_Unsigned)
                            as lua_Integer;
                        (*io_10)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_11, rc_10, ra, TM_BXOR);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                23 => {
                    let mut rb_12 = RKB!(i);
                    let mut rc_11 = RKC!(i);
                    let mut ib_5: lua_Integer = 0;
                    let mut ic_5: lua_Integer = 0;
                    if tointeger!(rb, & ib) != 0 && tointeger!(rc, & ic) != 0 {
                        let mut io_11 = ra;
                        (*io_11).value_.i = luaV_shiftl(ib_5, ic_5);
                        (*io_11)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_12, rc_11, ra, TM_SHL);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                24 => {
                    let mut rb_13 = RKB!(i);
                    let mut rc_12 = RKC!(i);
                    let mut ib_6: lua_Integer = 0;
                    let mut ic_6: lua_Integer = 0;
                    if tointeger!(rb, & ib) != 0 && tointeger!(rc, & ic) != 0 {
                        let mut io_12 = ra;
                        (*io_12).value_.i = luaV_shiftl(ib_6, -ic_6);
                        (*io_12)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_13, rc_12, ra, TM_SHR);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                16 => {
                    let mut rb_14 = RKB!(i);
                    let mut rc_13 = RKC!(i);
                    let mut nb_3: lua_Number = 0.;
                    let mut nc_3: lua_Number = 0.;
                    if ttisinteger!(rb) != 0 && ttisinteger!(rc) != 0 {
                        let mut ib_7 = ivalue!(rb);
                        let mut ic_7 = ivalue!(rc);
                        let mut io_13 = ra;
                        (*io_13).value_.i = luaV_mod(L, ib_7, ic_7);
                        (*io_13)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else if tonumber!(rb, & nb) != 0 && tonumber!(rc, & nc) != 0 {
                        let mut m: lua_Number = 0.;
                        let ref mut fresh267 = luai_nummod!(L, nb, nc, m);
                        *fresh267 = fmod(nb_3, nc_3);
                        if luai_nummod!(L, nb, nc, m) != 0 {
                            let ref mut fresh268 = luai_nummod!(L, nb, nc, m);
                            *fresh268 += luai_nummod!(L, nb, nc, m);
                        }
                        let mut io_14 = setfltvalue!(ra, m);
                        (*io_14).value_.n = setfltvalue!(ra, m);
                        (*io_14)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_14, rc_13, ra, TM_MOD);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                19 => {
                    let mut rb_15 = RKB!(i);
                    let mut rc_14 = RKC!(i);
                    let mut nb_4: lua_Number = 0.;
                    let mut nc_4: lua_Number = 0.;
                    if ttisinteger!(rb) != 0 && ttisinteger!(rc) != 0 {
                        let mut ib_8 = ivalue!(rb);
                        let mut ic_8 = ivalue!(rc);
                        let mut io_15 = ra;
                        (*io_15).value_.i = luaV_div(L, ib_8, ic_8);
                        (*io_15)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else if tonumber!(rb, & nb) != 0 && tonumber!(rc, & nc) != 0 {
                        let mut io_16 = ra;
                        (*io_16).value_.n = floor(nb_4 / nc_4);
                        (*io_16)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_15, rc_14, ra, TM_IDIV);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                17 => {
                    let mut rb_16 = RKB!(i);
                    let mut rc_15 = RKC!(i);
                    let mut nb_5: lua_Number = 0.;
                    let mut nc_5: lua_Number = 0.;
                    if tonumber!(rb, & nb) != 0 && tonumber!(rc, & nc) != 0 {
                        let mut io_17 = ra;
                        (*io_17).value_.n = pow(nb_5, nc_5);
                        (*io_17)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_16, rc_15, ra, TM_POW);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                25 => {
                    let mut rb_17 = RB!(i);
                    let mut nb_6: lua_Number = 0.;
                    if ttisinteger!(rb) != 0 {
                        let mut ib_9 = ivalue!(rb);
                        let mut io_18 = ra;
                        (*io_18)
                            .value_
                            .i = (0 as libc::c_int as lua_Unsigned)
                            .wrapping_sub(ib_9 as lua_Unsigned) as lua_Integer;
                        (*io_18)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else if tonumber!(rb, & nb) != 0 {
                        let mut io_19 = ra;
                        (*io_19).value_.n = -nb_6;
                        (*io_19)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_17, rb_17, ra, TM_UNM);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                26 => {
                    let mut rb_18 = RB!(i);
                    let mut ib_10: lua_Integer = 0;
                    if tointeger!(rb, & ib) != 0 {
                        let mut io_20 = ra;
                        (*io_20)
                            .value_
                            .i = (!(0 as libc::c_int as lua_Unsigned)
                            ^ ib_10 as lua_Unsigned) as lua_Integer;
                        (*io_20)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        luaT_trybinTM(L, rb_18, rb_18, ra, TM_BNOT);
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                27 => {
                    let mut rb_19 = RB!(i);
                    let mut res = l_isfalse!(rb);
                    let mut io_21 = setbvalue!(ra, res);
                    (*io_21).value_.b = setbvalue!(ra, res);
                    (*io_21).tt_ = 1 as libc::c_int;
                    continue;
                }
                28 => {
                    luaV_objlen(
                        L,
                        ra,
                        base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                        + 9 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ) as *const TValue,
                    );
                    base = (*ci).u.l.base;
                    continue;
                }
                29 => {
                    let mut b_2 = GETARG_B!(i);
                    let mut c_0 = GETARG_C!(i);
                    let mut rb_20 = 0 as *mut TValue;
                    (*L)
                        .top = base
                        .offset(c_0 as isize)
                        .offset(1 as libc::c_int as isize);
                    luaV_concat(L, c_0 - b_2 + 1 as libc::c_int);
                    base = (*ci).u.l.base;
                    ra = RA!(i);
                    rb_20 = base.offset(b_2 as isize);
                    let mut io1_8 = setobjs2s!(L, ra, rb);
                    let ref mut fresh269 = setobjs2s!(L, ra, rb);
                    *fresh269 = setobjs2s!(L, ra, rb);
                    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                        (*L)
                            .top = if ra >= rb_20 {
                            ra.offset(1 as libc::c_int as isize)
                        } else {
                            rb_20
                        };
                        luaC_step(L);
                        (*L).top = (*ci).top;
                        base = (*ci).u.l.base;
                    }
                    (*L).top = (*ci).top;
                    continue;
                }
                30 => {
                    let mut a = (i >> 0 as libc::c_int + 6 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                            << 0 as libc::c_int) as libc::c_int;
                    if dojump!(ci, i, 0) != 0 {
                        dojump!(
                            ci, i, 0
                        )(
                            dojump!(ci, i, 0),
                            ((*ci).u.l.base)
                                .offset(dojump!(ci, i, 0) as isize)
                                .offset(-(dojump!(ci, i, 0) as isize)),
                        );
                    }
                    (*ci)
                        .u
                        .l
                        .savedpc = ((*ci).u.l.savedpc)
                        .offset(
                            ((i >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction)
                                    << 9 as libc::c_int + 9 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_int - MAXARG_sBx + 0 as libc::c_int) as isize,
                        );
                    continue;
                }
                31 => {
                    let mut rb_21 = RKB!(i);
                    let mut rc_16 = RKC!(i);
                    if luaV_equalobj(L, rb_21, rc_16)
                        != (i >> 0 as libc::c_int + 6 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                    {
                        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_0 = (i >> 0 as libc::c_int + 6 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        if a_0 != 0 as libc::c_int {
                            luaF_close(
                                L,
                                ((*ci).u.l.base)
                                    .offset(a_0 as isize)
                                    .offset(-(1 as libc::c_int as isize)),
                            );
                        }
                        (*ci)
                            .u
                            .l
                            .savedpc = ((*ci).u.l.savedpc)
                            .offset(
                                ((i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 9 as libc::c_int + 9 as libc::c_int) << 0 as libc::c_int)
                                    as libc::c_int
                                    - (((1 as libc::c_int)
                                        << 9 as libc::c_int + 9 as libc::c_int) - 1 as libc::c_int
                                        >> 1 as libc::c_int) + 1 as libc::c_int) as isize,
                            );
                    }
                    let ref mut fresh270 = Protect!(
                        if (luaV_equalobj(L, rb, rc) != GETARG_A(i)) ci -> u.l.savedpc
                        ++; else donextjump(ci);
                    );
                    *fresh270 = Protect!(
                        if (luaV_equalobj(L, rb, rc) != GETARG_A(i)) ci -> u.l.savedpc
                        ++; else donextjump(ci);
                    );
                    continue;
                }
                32 => {
                    if luaV_lessthan(
                        L,
                        (if (i
                            >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                + 9 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            & (1 as libc::c_int) << 9 as libc::c_int - 1 as libc::c_int
                            != 0
                        {
                            k.offset(
                                ((i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                        + 9 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int
                                    & !((1 as libc::c_int)
                                        << 9 as libc::c_int - 1 as libc::c_int)) as isize,
                            )
                        } else {
                            base.offset(
                                (i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                        + 9 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            )
                        }),
                        (if (i >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            & (1 as libc::c_int) << 9 as libc::c_int - 1 as libc::c_int
                            != 0
                        {
                            k.offset(
                                ((i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int
                                    & !((1 as libc::c_int)
                                        << 9 as libc::c_int - 1 as libc::c_int)) as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            )
                        }),
                    )
                        != (i >> 0 as libc::c_int + 6 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                    {
                        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_1 = (i >> 0 as libc::c_int + 6 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        if a_1 != 0 as libc::c_int {
                            luaF_close(
                                L,
                                ((*ci).u.l.base)
                                    .offset(a_1 as isize)
                                    .offset(-(1 as libc::c_int as isize)),
                            );
                        }
                        (*ci)
                            .u
                            .l
                            .savedpc = ((*ci).u.l.savedpc)
                            .offset(
                                ((i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 9 as libc::c_int + 9 as libc::c_int) << 0 as libc::c_int)
                                    as libc::c_int
                                    - (((1 as libc::c_int)
                                        << 9 as libc::c_int + 9 as libc::c_int) - 1 as libc::c_int
                                        >> 1 as libc::c_int) + 1 as libc::c_int) as isize,
                            );
                    }
                    let ref mut fresh271 = Protect!(
                        if (luaV_lessthan(L, RKB(i), RKC(i)) != GETARG_A(i)) ci -> u.l
                        .savedpc ++; else donextjump(ci);
                    );
                    *fresh271 = Protect!(
                        if (luaV_lessthan(L, RKB(i), RKC(i)) != GETARG_A(i)) ci -> u.l
                        .savedpc ++; else donextjump(ci);
                    );
                    continue;
                }
                33 => {
                    if luaV_lessequal(
                        L,
                        (if (i
                            >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                + 9 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            & (1 as libc::c_int) << 9 as libc::c_int - 1 as libc::c_int
                            != 0
                        {
                            k.offset(
                                ((i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                        + 9 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int
                                    & !((1 as libc::c_int)
                                        << 9 as libc::c_int - 1 as libc::c_int)) as isize,
                            )
                        } else {
                            base.offset(
                                (i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                        + 9 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            )
                        }),
                        (if (i >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            & (1 as libc::c_int) << 9 as libc::c_int - 1 as libc::c_int
                            != 0
                        {
                            k.offset(
                                ((i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int
                                    & !((1 as libc::c_int)
                                        << 9 as libc::c_int - 1 as libc::c_int)) as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            )
                        }),
                    )
                        != (i >> 0 as libc::c_int + 6 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                    {
                        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_2 = (i >> 0 as libc::c_int + 6 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        if a_2 != 0 as libc::c_int {
                            luaF_close(
                                L,
                                ((*ci).u.l.base)
                                    .offset(a_2 as isize)
                                    .offset(-(1 as libc::c_int as isize)),
                            );
                        }
                        (*ci)
                            .u
                            .l
                            .savedpc = ((*ci).u.l.savedpc)
                            .offset(
                                ((i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 9 as libc::c_int + 9 as libc::c_int) << 0 as libc::c_int)
                                    as libc::c_int
                                    - (((1 as libc::c_int)
                                        << 9 as libc::c_int + 9 as libc::c_int) - 1 as libc::c_int
                                        >> 1 as libc::c_int) + 1 as libc::c_int) as isize,
                            );
                    }
                    let ref mut fresh272 = Protect!(
                        if (luaV_lessequal(L, RKB(i), RKC(i)) != GETARG_A(i)) ci -> u.l
                        .savedpc ++; else donextjump(ci);
                    );
                    *fresh272 = Protect!(
                        if (luaV_lessequal(L, RKB(i), RKC(i)) != GETARG_A(i)) ci -> u.l
                        .savedpc ++; else donextjump(ci);
                    );
                    continue;
                }
                34 => {
                    if if GETARG_C!(i) != 0 {
                        l_isfalse!(ra)
                    } else {
                        (l_isfalse!(ra) == 0) as libc::c_int
                    } != 0
                    {
                        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
                    } else {
                        let ref mut fresh273 = donextjump!(ci);
                        *fresh273 = donextjump!(ci);
                        let mut a_3 = (i >> 0 as libc::c_int + 6 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        if a_3 != 0 as libc::c_int {
                            luaF_close(
                                L,
                                ((*ci).u.l.base)
                                    .offset(a_3 as isize)
                                    .offset(-(1 as libc::c_int as isize)),
                            );
                        }
                        (*ci)
                            .u
                            .l
                            .savedpc = ((*ci).u.l.savedpc)
                            .offset(
                                ((i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 9 as libc::c_int + 9 as libc::c_int) << 0 as libc::c_int)
                                    as libc::c_int - MAXARG_sBx + 1 as libc::c_int) as isize,
                            );
                    }
                    continue;
                }
                35 => {
                    let mut rb_22 = RB!(i);
                    if if GETARG_C!(i) != 0 {
                        l_isfalse!(rb)
                    } else {
                        (l_isfalse!(rb) == 0) as libc::c_int
                    } != 0
                    {
                        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
                    } else {
                        let mut io1_9 = setobjs2s!(L, ra, rb);
                        let ref mut fresh274 = setobjs2s!(L, ra, rb);
                        *fresh274 = setobjs2s!(L, ra, rb);
                        let ref mut fresh275 = donextjump!(ci);
                        *fresh275 = donextjump!(ci);
                        let mut a_4 = (i >> 0 as libc::c_int + 6 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        if a_4 != 0 as libc::c_int {
                            luaF_close(
                                L,
                                ((*ci).u.l.base)
                                    .offset(a_4 as isize)
                                    .offset(-(1 as libc::c_int as isize)),
                            );
                        }
                        (*ci)
                            .u
                            .l
                            .savedpc = ((*ci).u.l.savedpc)
                            .offset(
                                ((i
                                    >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 9 as libc::c_int + 9 as libc::c_int) << 0 as libc::c_int)
                                    as libc::c_int - MAXARG_sBx + 1 as libc::c_int) as isize,
                            );
                    }
                    continue;
                }
                36 => {
                    let mut b_3 = GETARG_B!(i);
                    let mut nresults = GETARG_C!(i) - 1 as libc::c_int;
                    if b_3 != 0 as libc::c_int {
                        (*L).top = ra.offset(b_3 as isize);
                    }
                    if luaD_precall(L, ra, nresults) != 0 {
                        if nresults >= 0 as libc::c_int {
                            (*L).top = (*ci).top;
                        }
                        let ref mut fresh276 = Protect!((void) 0);
                        *fresh276 = Protect!((void) 0);
                        continue;
                    } else {
                        ci = (*L).ci;
                        break;
                    }
                }
                37 => {
                    let mut b_4 = GETARG_B!(i);
                    if b_4 != 0 as libc::c_int {
                        (*L).top = ra.offset(b_4 as isize);
                    }
                    if luaD_precall(L, ra, LUA_MULTRET) != 0 {
                        let ref mut fresh277 = Protect!((void) 0);
                        *fresh277 = Protect!((void) 0);
                        continue;
                    } else {
                        let mut nci = (*L).ci;
                        let mut oci = (*nci).previous;
                        let mut nfunc = (*nci).func;
                        let mut ofunc = (*oci).func;
                        let mut lim = ((*nci).u.l.base)
                            .offset(
                                (*getproto!(nfunc)).numparams as libc::c_int as isize,
                            );
                        let mut aux_0: libc::c_int = 0;
                        if (*(*cl).p).sizep > 0 as libc::c_int {
                            luaF_close(L, (*oci).u.l.base);
                        }
                        aux_0 = 0 as libc::c_int;
                        while nfunc.offset(aux_0 as isize) < lim {
                            let mut io1_10 = setobjs2s!(L, ofunc + aux, nfunc + aux);
                            let ref mut fresh278 = setobjs2s!(
                                L, ofunc + aux, nfunc + aux
                            );
                            *fresh278 = setobjs2s!(L, ofunc + aux, nfunc + aux);
                            aux_0 += 1;
                        }
                        (*oci)
                            .u
                            .l
                            .base = ofunc
                            .offset(
                                ((*nci).u.l.base).offset_from(nfunc) as libc::c_long
                                    as isize,
                            );
                        (*L)
                            .top = ofunc
                            .offset(
                                ((*L).top).offset_from(nfunc) as libc::c_long as isize,
                            );
                        (*oci).top = (*L).top;
                        (*oci).u.l.savedpc = (*nci).u.l.savedpc;
                        (*oci)
                            .callstatus = ((*oci).callstatus as libc::c_int | CIST_TAIL)
                            as libc::c_ushort;
                        (*L).ci = oci;
                        ci = (*L).ci;
                        break;
                    }
                }
                38 => {
                    b_5 = GETARG_B!(i);
                    if (*(*cl).p).sizep > 0 as libc::c_int {
                        luaF_close(L, base);
                    }
                    b_5 = luaD_poscall(
                        L,
                        ci,
                        ra,
                        if b_5 != 0 as libc::c_int {
                            b_5 - 1 as libc::c_int
                        } else {
                            cast_int!(L -> top - ra)
                        },
                    );
                    if (*ci).callstatus as libc::c_int & CIST_FRESH != 0 {
                        break 's_16;
                    }
                    ci = (*L).ci;
                    if b_5 != 0 {
                        (*L).top = (*ci).top;
                    }
                    break;
                }
                39 => {
                    if ttisinteger!(ra) != 0 {
                        let mut step = ivalue!(ra + 2);
                        let mut idx = intop!(+, ivalue(ra), step);
                        let mut limit = ivalue!(ra + 1);
                        if if (0 as libc::c_int as libc::c_longlong) < step {
                            (idx <= limit) as libc::c_int
                        } else {
                            (limit <= idx) as libc::c_int
                        } != 0
                        {
                            (*ci)
                                .u
                                .l
                                .savedpc = ((*ci).u.l.savedpc)
                                .offset(GETARG_sBx!(i) as isize);
                            let mut io_22 = chgivalue!(ra, idx);
                            (*io_22).value_.i = chgivalue!(ra, idx);
                            let mut io_23 = setivalue!(ra + 3, idx);
                            (*io_23).value_.i = setivalue!(ra + 3, idx);
                            (*io_23)
                                .tt_ = 3 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int;
                        }
                    } else {
                        let mut step_0 = fltvalue!(ra + 2);
                        let mut idx_0 = luai_numadd!(L, fltvalue(ra), step);
                        let mut limit_0 = fltvalue!(ra + 1);
                        if if luai_numlt!(0, step) != 0 {
                            luai_numle!(idx, limit)
                        } else {
                            luai_numle!(limit, idx)
                        } != 0
                        {
                            (*ci)
                                .u
                                .l
                                .savedpc = ((*ci).u.l.savedpc)
                                .offset(GETARG_sBx!(i) as isize);
                            let mut io_24 = chgfltvalue!(ra, idx);
                            (*io_24).value_.n = chgfltvalue!(ra, idx);
                            let mut io_25 = setfltvalue!(ra + 3, idx);
                            (*io_25).value_.n = setfltvalue!(ra + 3, idx);
                            (*io_25)
                                .tt_ = 3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int;
                        }
                    }
                    continue;
                }
                40 => {
                    let mut init = ra;
                    let mut plimit = ra.offset(1 as libc::c_int as isize);
                    let mut pstep = ra.offset(2 as libc::c_int as isize);
                    let mut ilimit: lua_Integer = 0;
                    let mut stopnow: libc::c_int = 0;
                    if ttisinteger!(init) != 0 && ttisinteger!(pstep) != 0
                        && forlimit(plimit, &mut ilimit, ivalue!(pstep), &mut stopnow)
                            != 0
                    {
                        let mut initv = if stopnow != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            ivalue!(init)
                        };
                        let mut io_26 = setivalue!(plimit, ilimit);
                        (*io_26).value_.i = setivalue!(plimit, ilimit);
                        (*io_26)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                        let mut io_27 = init;
                        (*io_27)
                            .value_
                            .i = (initv as lua_Unsigned)
                            .wrapping_sub((*pstep).value_.i as lua_Unsigned)
                            as lua_Integer;
                        (*io_27)
                            .tt_ = 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int;
                    } else {
                        let mut ninit: lua_Number = 0.;
                        let mut nlimit: lua_Number = 0.;
                        let mut nstep: lua_Number = 0.;
                        if tonumber!(plimit, & nlimit) == 0 {
                            luaG_runerror(
                                L,
                                b"'for' limit must be a number\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        let mut io_28 = setfltvalue!(plimit, nlimit);
                        (*io_28).value_.n = setfltvalue!(plimit, nlimit);
                        (*io_28)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                        if tonumber!(pstep, & nstep) == 0 {
                            luaG_runerror(
                                L,
                                b"'for' step must be a number\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        let mut io_29 = setfltvalue!(pstep, nstep);
                        (*io_29).value_.n = setfltvalue!(pstep, nstep);
                        (*io_29)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                        if tonumber!(init, & ninit) == 0 {
                            luaG_runerror(
                                L,
                                b"'for' initial value must be a number\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        let mut io_30 = init;
                        (*io_30).value_.n = ninit - nstep;
                        (*io_30)
                            .tt_ = 3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int;
                    }
                    (*ci)
                        .u
                        .l
                        .savedpc = ((*ci).u.l.savedpc).offset(GETARG_sBx!(i) as isize);
                    continue;
                }
                41 => {
                    let mut cb = ra.offset(3 as libc::c_int as isize);
                    let mut io1_11 = setobjs2s!(L, cb + 2, ra + 2);
                    let ref mut fresh279 = setobjs2s!(L, cb + 2, ra + 2);
                    *fresh279 = setobjs2s!(L, cb + 2, ra + 2);
                    let mut io1_12 = setobjs2s!(L, cb + 1, ra + 1);
                    let ref mut fresh280 = setobjs2s!(L, cb + 1, ra + 1);
                    *fresh280 = setobjs2s!(L, cb + 1, ra + 1);
                    let mut io1_13 = setobjs2s!(L, cb, ra);
                    let ref mut fresh281 = setobjs2s!(L, cb, ra);
                    *fresh281 = setobjs2s!(L, cb, ra);
                    (*L).top = cb.offset(3 as libc::c_int as isize);
                    luaD_call(
                        L,
                        cb,
                        (i >> 0 as libc::c_int + 6 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 9 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int,
                    );
                    base = (*ci).u.l.base;
                    (*L).top = (*ci).top;
                    let fresh282 = (*ci).u.l.savedpc;
                    (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
                    i = *fresh282;
                    ra = RA!(i);
                }
                42 => {}
                43 => {
                    let mut n = GETARG_B!(i);
                    let mut c_1 = GETARG_C!(i);
                    let mut last: libc::c_uint = 0;
                    let mut h = 0 as *mut Table;
                    if n == 0 as libc::c_int {
                        n = cast_int!(L -> top - ra) - 1 as libc::c_int;
                    }
                    if c_1 == 0 as libc::c_int {
                        c_1 = GETARG_Ax!(* ci -> u.l.savedpc ++);
                    }
                    h = hvalue!(ra);
                    last = ((c_1 - 1 as libc::c_int) * LFIELDS_PER_FLUSH + n)
                        as libc::c_uint;
                    if last > (*h).sizearray {
                        luaH_resizearray(L, h, last);
                    }
                    while n > 0 as libc::c_int {
                        let mut val = ra.offset(n as isize);
                        let fresh284 = last;
                        last = last.wrapping_sub(1);
                        luaH_setint(L, h, fresh284 as lua_Integer, val);
                        if luaC_barrierback!(L, h, val) != 0 {} else {};
                        n -= 1;
                    }
                    (*L).top = (*ci).top;
                    continue;
                }
                44 => {
                    let mut p = *((*(*cl).p).p).offset(GETARG_Bx!(i) as isize);
                    let mut ncl = getcached(p, ((*cl).upvals).as_mut_ptr(), base);
                    if ncl.is_null() {
                        pushclosure(L, p, ((*cl).upvals).as_mut_ptr(), base, ra);
                    } else {
                        let mut io_31 = setclLvalue!(L, ra, ncl);
                        let mut x__0 = setclLvalue!(L, ra, ncl);
                        (*io_31).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
                        (*io_31)
                            .tt_ = 6 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int
                            | (1 as libc::c_int) << 6 as libc::c_int;
                    }
                    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                        (*L).top = ra.offset(1 as libc::c_int as isize);
                        luaC_step(L);
                        (*L).top = (*ci).top;
                        base = (*ci).u.l.base;
                    }
                    continue;
                }
                45 => {
                    let mut b_6 = GETARG_B!(i) - 1 as libc::c_int;
                    let mut j: libc::c_int = 0;
                    let mut n_0 = cast_int!(base - ci -> func)
                        - (*(*cl).p).numparams as libc::c_int - 1 as libc::c_int;
                    if n_0 < 0 as libc::c_int {
                        n_0 = 0 as libc::c_int;
                    }
                    if b_6 < 0 as libc::c_int {
                        b_6 = n_0;
                        if ((*L).stack_last).offset_from((*L).top) as libc::c_long
                            <= n_0 as libc::c_long
                        {
                            luaD_growstack(L, n_0);
                        }
                        base = (*ci).u.l.base;
                        ra = RA!(i);
                        (*L).top = ra.offset(n_0 as isize);
                    }
                    j = 0 as libc::c_int;
                    while j < b_6 && j < n_0 {
                        let mut io1_15 = setobjs2s!(L, ra + j, base - n + j);
                        let ref mut fresh285 = setobjs2s!(L, ra + j, base - n + j);
                        *fresh285 = setobjs2s!(L, ra + j, base - n + j);
                        j += 1;
                    }
                    while j < b_6 {
                        let ref mut fresh286 = setnilvalue!(ra + j);
                        *fresh286 = setnilvalue!(ra + j);
                        j += 1;
                    }
                    continue;
                }
                46 | _ => {
                    continue;
                }
            }
            if ttisnil!(ra + 1) == 0 {
                let mut io1_14 = setobjs2s!(L, ra, ra + 1);
                let ref mut fresh283 = setobjs2s!(L, ra, ra + 1);
                *fresh283 = setobjs2s!(L, ra, ra + 1);
                (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(GETARG_sBx!(i) as isize);
            }
        }
    };
}
#[no_mangle]
pub static mut lua_ident: [libc::c_char; 129] = unsafe {
    *::core::mem::transmute::<
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
    let mut ci = (*L).ci;
    if idx > 0 as libc::c_int {
        let mut o = ((*ci).func).offset(idx as isize);
        if o >= (*L).top {
            return &luaO_nilobject_ as *const TValue as *mut TValue
        } else {
            return o
        }
    } else if ispseudo!(idx) == 0 {
        return ((*L).top).offset(idx as isize)
    } else if idx == LUA_REGISTRYINDEX {
        return &mut (*G!(L)).l_registry
    } else {
        idx = LUA_REGISTRYINDEX - idx;
        if ttislcf!(ci -> func) != 0 {
            return &luaO_nilobject_ as *const TValue as *mut TValue
        } else {
            let mut func: *mut CClosure = clCvalue!(ci -> func);
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
    let mut size = *(ud as *mut libc::c_int);
    luaD_growstack(L, size);
}
#[no_mangle]
pub unsafe extern "C" fn lua_checkstack(
    mut L: *mut lua_State,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut ci = (*L).ci;
    if ((*L).stack_last).offset_from((*L).top) as libc::c_long > n as libc::c_long {
        res = 1 as libc::c_int;
    } else {
        let mut inuse = cast_int!(L -> top - L -> stack) + EXTRA_STACK;
        if inuse > LUAI_MAXSTACK - n {
            res = 0 as libc::c_int;
        } else {
            res = (luaD_rawrunprotected(
                L,
                Some(
                    growstack
                        as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
                ),
                &mut n as *mut libc::c_int as *mut libc::c_void,
            ) == LUA_OK) as libc::c_int;
        }
    }
    if res != 0 && (*ci).top < ((*L).top).offset(n as isize) {
        (*ci).top = ((*L).top).offset(n as isize);
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
    (*from).top = ((*from).top).offset(-(n as isize));
    i = 0 as libc::c_int;
    while i < n {
        let mut io1 = setobj2s!(to, to -> top, from -> top + i);
        let ref mut fresh287 = setobj2s!(to, to -> top, from -> top + i);
        *fresh287 = setobj2s!(to, to -> top, from -> top + i);
        (*to).top = ((*to).top).offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_atpanic(
    mut L: *mut lua_State,
    mut panicf: lua_CFunction,
) -> lua_CFunction {
    let mut old: lua_CFunction = None;
    old = (*G!(L)).panic;
    let ref mut fresh288 = (*G!(L)).panic;
    *fresh288 = panicf;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn lua_version(mut L: *mut lua_State) -> *const lua_Number {
    static mut version: lua_Number = LUA_VERSION_NUM as lua_Number;
    if L.is_null() { return &version } else { return (*G!(L)).version };
}
#[no_mangle]
pub unsafe extern "C" fn lua_absindex(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    return if idx > 0 as libc::c_int || ispseudo!(idx) != 0 {
        idx
    } else {
        cast_int!(L -> top - L -> ci -> func) + idx
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettop(mut L: *mut lua_State) -> libc::c_int {
    return ((*L).top).offset_from(((*(*L).ci).func).offset(1 as libc::c_int as isize))
        as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_settop(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut func = (*(*L).ci).func;
    if idx >= 0 as libc::c_int {
        while (*L).top < func.offset(1 as libc::c_int as isize).offset(idx as isize) {
            let ref mut fresh289 = setnilvalue!(L -> top ++);
            *fresh289 = setnilvalue!(L -> top ++);
        }
        (*L).top = func.offset(1 as libc::c_int as isize).offset(idx as isize);
    } else {
        (*L).top = ((*L).top).offset((idx + 1 as libc::c_int) as isize);
    };
}
unsafe extern "C" fn reverse(mut L: *mut lua_State, mut from: StkId, mut to: StkId) {
    while from < to {
        let mut temp = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut io1: *mut TValue = setobj!(L, & temp, from);
        let ref mut fresh290 = setobj!(L, & temp, from);
        *fresh290 = setobj!(L, & temp, from);
        let mut io1_0 = setobjs2s!(L, from, to);
        let ref mut fresh291 = setobjs2s!(L, from, to);
        *fresh291 = setobjs2s!(L, from, to);
        let mut io1_1 = setobj2s!(L, to, & temp);
        let ref mut fresh292 = setobj2s!(L, to, & temp);
        *fresh292 = setobj2s!(L, to, & temp);
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
    let mut p = 0 as *mut TValue;
    let mut t = 0 as *mut TValue;
    let mut m = 0 as *mut TValue;
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
    let mut fr = 0 as *mut TValue;
    let mut to = 0 as *mut TValue;
    fr = index2addr(L, fromidx);
    to = index2addr(L, toidx);
    let mut io1 = setobj!(L, to, fr);
    let ref mut fresh293 = setobj!(L, to, fr);
    *fresh293 = setobj!(L, to, fr);
    if isupvalue!(toidx) != 0 {
        if luaC_barrier!(L, clCvalue(L -> ci -> func), fr) != 0 {} else {};
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvalue(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut io1 = (*L).top;
    *io1 = *index2addr(L, idx);
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void lua_pushvalue(lua_State *, int)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_type(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o = index2addr(L, idx);
    return if isvalid!(o) != 0 { ttnov!(o) } else { LUA_TNONE };
}
#[no_mangle]
pub unsafe extern "C" fn lua_typename(
    mut L: *mut lua_State,
    mut t: libc::c_int,
) -> *const libc::c_char {
    return ttypename!(t);
}
#[no_mangle]
pub unsafe extern "C" fn lua_iscfunction(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o = index2addr(L, idx);
    return (ttislcf!(o) != 0 || (*o).tt_ == ttisCclosure!(o)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isinteger(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o = index2addr(L, idx);
    return ttisinteger!(o);
}
#[no_mangle]
pub unsafe extern "C" fn lua_isnumber(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2addr(L, idx);
    return tonumber!(o, & n);
}
#[no_mangle]
pub unsafe extern "C" fn lua_isstring(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return (ttisstring!(o) != 0 || cvt2str!(o) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isuserdata(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return (ttisfulluserdata!(o) != 0 || ttislightuserdata!(o) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawequal(
    mut L: *mut lua_State,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
) -> libc::c_int {
    let mut o1 = index2addr(L, index1);
    let mut o2 = index2addr(L, index2);
    return if isvalid!(o1) != 0 && isvalid!(o2) != 0 {
        luaV_rawequalobj!(o1, o2)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_arith(mut L: *mut lua_State, mut op: libc::c_int) {
    if !(op != LUA_OPUNM && op != LUA_OPBNOT) {
        let mut io1 = setobjs2s!(L, L -> top, L -> top - 1);
        let ref mut fresh294 = setobjs2s!(L, L -> top, L -> top - 1);
        *fresh294 = setobjs2s!(L, L -> top, L -> top - 1);
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                309 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void lua_arith(lua_State *, int)\0"))
                    .as_ptr(),
            );
        }
    }
    luaO_arith(
        L,
        op,
        ((*L).top).offset(-(2 as libc::c_int as isize)) as *const TValue,
        ((*L).top).offset(-(1 as libc::c_int as isize)) as *const TValue,
        ((*L).top).offset(-(2 as libc::c_int as isize)),
    );
    (*L).top = ((*L).top).offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_compare(
    mut L: *mut lua_State,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut o1 = 0 as *mut TValue;
    let mut o2 = 0 as *mut TValue;
    let mut i = 0 as libc::c_int;
    o1 = index2addr(L, index1);
    o2 = index2addr(L, index2);
    if isvalid!(o1) != 0 && isvalid!(o2) != 0 {
        match op {
            LUA_OPEQ => {
                i = luaV_equalobj(L, o1 as *const TValue, o2 as *const TValue);
            }
            LUA_OPLT => {
                i = luaV_lessthan(L, o1 as *const TValue, o2 as *const TValue);
            }
            LUA_OPLE => {
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
    let mut sz = luaO_str2num(s, (*L).top);
    if sz != 0 as libc::c_int as libc::c_ulong {
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                340 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"size_t lua_stringtonumber(lua_State *, const char *)\0"))
                    .as_ptr(),
            );
        }
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
    let mut isnum = tonumber!(o, & n);
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
    let mut isnum = tointeger!(o, & res);
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
    return (l_isfalse!(o) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tolstring(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut len: *mut size_t,
) -> *const libc::c_char {
    let mut o = index2addr(L, idx);
    if ttisstring!(o) == 0 {
        if cvt2str!(o) == 0 {
            if !len.is_null() {
                *len = 0 as libc::c_int as size_t;
            }
            return NULL as *const libc::c_char;
        }
        luaO_tostring(L, o);
        if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
            luaC_checkGC!(L)(L);
        }
        o = index2addr(L, idx);
    }
    if !len.is_null() {
        *len = vslen!(o);
    }
    return svalue!(o);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawlen(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> size_t {
    let mut o = index2addr(L, idx);
    match ttype!(o) {
        LUA_TSHRSTR => return (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as size_t,
        LUA_TLNGSTR => return (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen,
        LUA_TUSERDATA => return (*((*o).value_.gc as *mut GCUnion)).u.len,
        LUA_TTABLE => return luaH_getn(hvalue!(o)) as size_t,
        _ => return 0 as libc::c_int as size_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tocfunction(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> lua_CFunction {
    let mut o = index2addr(L, idx);
    if ttislcf!(o) != 0 {
        return fvalue!(o)
    } else if ttisCclosure!(o) != 0 {
        return (*((*o).value_.gc as *mut GCUnion)).cl.c.f
    } else {
        return ::core::mem::transmute::<
            libc::intptr_t,
            lua_CFunction,
        >(NULL as libc::intptr_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_touserdata(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *mut libc::c_void {
    let mut o = index2addr(L, idx);
    match ttnov!(o) {
        LUA_TUSERDATA => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata
                as *mut libc::c_char)
                .offset(::core::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *mut libc::c_void;
        }
        LUA_TLIGHTUSERDATA => return pvalue!(o),
        _ => return NULL as *mut libc::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tothread(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *mut lua_State {
    let mut o = index2addr(L, idx);
    return if ttisthread!(o) == 0 { NULL as *mut lua_State } else { thvalue!(o) };
}
#[no_mangle]
pub unsafe extern "C" fn lua_topointer(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *const libc::c_void {
    let mut o = index2addr(L, idx);
    match ttype!(o) {
        LUA_TTABLE => return hvalue!(o),
        LUA_TLCL => return clLvalue!(o),
        LUA_TCCL => return clCvalue!(o),
        LUA_TLCF => {
            return ::core::mem::transmute::<lua_CFunction, size_t>((*o).value_.f)
                as *mut libc::c_void;
        }
        LUA_TTHREAD => return thvalue!(o),
        LUA_TUSERDATA => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata
                as *mut libc::c_char)
                .offset(::core::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *const libc::c_void;
        }
        LUA_TLIGHTUSERDATA => return pvalue!(o),
        _ => return NULL as *const libc::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushnil(mut L: *mut lua_State) {
    let ref mut fresh295 = setnilvalue!(L -> top);
    *fresh295 = setnilvalue!(L -> top);
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            453 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void lua_pushnil(lua_State *)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushnumber(mut L: *mut lua_State, mut n: lua_Number) {
    let mut io = setfltvalue!(L -> top, n);
    (*io).value_.n = setfltvalue!(L -> top, n);
    (*io).tt_ = 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void lua_pushnumber(lua_State *, lua_Number)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushinteger(mut L: *mut lua_State, mut n: lua_Integer) {
    let mut io = setivalue!(L -> top, n);
    (*io).value_.i = setivalue!(L -> top, n);
    (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            469 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void lua_pushinteger(lua_State *, lua_Integer)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    let mut ts = 0 as *mut TString;
    ts = if len == 0 as libc::c_int as libc::c_ulong {
        luaS_new(L, b"\0" as *const u8 as *const libc::c_char)
    } else {
        luaS_newlstr(L, s, len)
    };
    let mut io = setsvalue2s!(L, L -> top, ts);
    let mut x_ = setsvalue2s!(L, L -> top, ts);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            484 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"const char *lua_pushlstring(lua_State *, const char *, size_t)\0"))
                .as_ptr(),
        );
    }
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
    }
    return getstr!(ts);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
) -> *const libc::c_char {
    if s.is_null() {
        let ref mut fresh296 = setnilvalue!(L -> top);
        *fresh296 = setnilvalue!(L -> top);
    } else {
        let mut ts = 0 as *mut TString;
        ts = luaS_new(L, s);
        let mut io = setsvalue2s!(L, L -> top, ts);
        let mut x_ = setsvalue2s!(L, L -> top, ts);
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        s = getstr!(ts);
    }
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            501 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"const char *lua_pushstring(lua_State *, const char *)\0"))
                .as_ptr(),
        );
    }
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut argp: ::core::ffi::VaList,
) -> *const libc::c_char {
    let mut ret = 0 as *const libc::c_char;
    ret = luaO_pushvfstring(L, fmt, argp.as_va_list());
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *const libc::c_char {
    let mut ret = 0 as *const libc::c_char;
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    ret = luaO_pushvfstring(L, fmt, argp.as_va_list());
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
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
        let mut io = setfvalue!(L -> top, fn);
        (*io).value_.f = setfvalue!(L -> top, fn);
        (*io).tt_ = 6 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                536 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\0"))
                    .as_ptr(),
            );
        }
    } else {
        let mut cl = 0 as *mut CClosure;
        cl = luaF_newCclosure(L, n);
        (*cl).f = fn_0;
        (*L).top = ((*L).top).offset(-(n as isize));
        loop {
            let fresh297 = n;
            n = n - 1;
            if !(fresh297 != 0) {
                break;
            }
            let mut io1: *mut TValue = setobj2n!(L, & cl -> upvalue[n], L -> top + n);
            let ref mut fresh298 = setobj2n!(L, & cl -> upvalue[n], L -> top + n);
            *fresh298 = setobj2n!(L, & cl -> upvalue[n], L -> top + n);
        }
        let mut io_0 = setclCvalue!(L, L -> top, cl);
        let mut x_ = setclCvalue!(L, L -> top, cl);
        (*io_0).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io_0)
            .tt_ = 6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int;
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                550 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\0"))
                    .as_ptr(),
            );
        }
        if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
            luaC_checkGC!(L)(L);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushboolean(mut L: *mut lua_State, mut b: libc::c_int) {
    let mut io = (*L).top;
    (*io).value_.b = (b != 0 as libc::c_int) as libc::c_int;
    (*io).tt_ = 1 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            560 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void lua_pushboolean(lua_State *, int)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlightuserdata(
    mut L: *mut lua_State,
    mut p: *mut libc::c_void,
) {
    let mut io = setpvalue!(L -> top, p);
    (*io).value_.p = setpvalue!(L -> top, p);
    (*io).tt_ = 2 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            568 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void lua_pushlightuserdata(lua_State *, void *)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushthread(mut L: *mut lua_State) -> libc::c_int {
    let mut io = setthvalue!(L, L -> top, L);
    let mut x_ = setthvalue!(L, L -> top, L);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 8 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            576 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"int lua_pushthread(lua_State *)\0"))
                .as_ptr(),
        );
    }
    return ((*G!(L)).mainthread == L) as libc::c_int;
}
unsafe extern "C" fn auxgetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) -> libc::c_int {
    let mut slot = 0 as *const TValue;
    let mut str = luaS_new(L, k);
    if luaV_fastget!(L, t, str, slot, luaH_getstr) != 0 {
        let mut io1 = setobj2s!(L, L -> top, slot);
        let ref mut fresh299 = setobj2s!(L, L -> top, slot);
        *fresh299 = setobj2s!(L, L -> top, slot);
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                593 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int auxgetstr(lua_State *, const TValue *, const char *)\0"))
                    .as_ptr(),
            );
        }
    } else {
        let mut io = setsvalue2s!(L, L -> top, str);
        let mut x_ = setsvalue2s!(L, L -> top, str);
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                597 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int auxgetstr(lua_State *, const TValue *, const char *)\0"))
                    .as_ptr(),
            );
        }
        luaV_finishget(
            L,
            t,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            slot,
        );
    }
    return ttnov!(L -> top - 1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_getglobal(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut reg: *mut Table = hvalue!(& G(L) -> l_registry);
    return auxgetstr(L, luaH_getint(reg, LUA_RIDX_GLOBALS as lua_Integer), name);
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettable(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut t = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut slot = 0 as *const TValue;
    if if !((*t).tt_ == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) {
        slot = NULL as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            ((*L).top).offset(-(1 as libc::c_int as isize)) as *const TValue,
        );
        !((*slot).tt_ == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        let mut io1 = ((*L).top).offset(-(1 as libc::c_int as isize));
        *io1 = *slot;
    } else {
        luaV_gettable!(
            L, t, L -> top - 1, L -> top - 1
        )(
            L,
            t as *const TValue,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            luaV_gettable!(L, t, L -> top - 1, L -> top - 1),
        );
    }
    return ttnov!(L -> top - 1);
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
    let mut t = 0 as *mut TValue;
    let mut slot = 0 as *const TValue;
    t = index2addr(L, idx);
    if luaV_fastget!(L, t, n, slot, luaH_getint) != 0 {
        let mut io1 = setobj2s!(L, L -> top, slot);
        let ref mut fresh300 = setobj2s!(L, L -> top, slot);
        *fresh300 = setobj2s!(L, L -> top, slot);
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                635 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"int lua_geti(lua_State *, int, lua_Integer)\0"))
                    .as_ptr(),
            );
        }
    } else {
        let mut io = setivalue!(L -> top, n);
        (*io).value_.i = setivalue!(L -> top, n);
        (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                639 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"int lua_geti(lua_State *, int, lua_Integer)\0"))
                    .as_ptr(),
            );
        }
        luaV_finishget(
            L,
            t as *const TValue,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            slot,
        );
    }
    return ttnov!(L -> top - 1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawget(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut t = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut io1 = ((*L).top).offset(-(1 as libc::c_int as isize));
    *io1 = *luaH_get(
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        ((*L).top).offset(-(1 as libc::c_int as isize)) as *const TValue,
    );
    return ttnov!(L -> top - 1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgeti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut t = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut io1 = (*L).top;
    *io1 = *luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            664 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int lua_rawgeti(lua_State *, int, lua_Integer)\0"))
                .as_ptr(),
        );
    }
    return ttnov!(L -> top - 1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgetp(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut p: *const libc::c_void,
) -> libc::c_int {
    let mut t = 0 as *mut TValue;
    let mut k = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    t = index2addr(L, idx);
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut libc::c_void;
    (*io).tt_ = 2 as libc::c_int;
    let mut io1 = (*L).top;
    *io1 = *luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, &mut k);
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            678 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int lua_rawgetp(lua_State *, int, const void *)\0"))
                .as_ptr(),
        );
    }
    return ttnov!(L -> top - 1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_createtable(
    mut L: *mut lua_State,
    mut narray: libc::c_int,
    mut nrec: libc::c_int,
) {
    let mut t = 0 as *mut Table;
    t = luaH_new(L);
    let mut io = sethvalue!(L, L -> top, t);
    let mut x_ = sethvalue!(L, L -> top, t);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            689 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void lua_createtable(lua_State *, int, int)\0"))
                .as_ptr(),
        );
    }
    if narray > 0 as libc::c_int || nrec > 0 as libc::c_int {
        luaH_resize(L, t, narray as libc::c_uint, nrec as libc::c_uint);
    }
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_getmetatable(
    mut L: *mut lua_State,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj = 0 as *const TValue;
    let mut mt = 0 as *mut Table;
    let mut res = 0 as libc::c_int;
    obj = index2addr(L, objindex);
    match ttnov!(obj) {
        LUA_TTABLE => {
            mt = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
        }
        LUA_TUSERDATA => {
            mt = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
        }
        _ => {
            mt = (*G!(L)).mt[ttnov!(obj) as usize];
        }
    }
    if !mt.is_null() {
        let mut io = sethvalue!(L, L -> top, mt);
        let mut x_ = sethvalue!(L, L -> top, mt);
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                716 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"int lua_getmetatable(lua_State *, int)\0"))
                    .as_ptr(),
            );
        }
        res = 1 as libc::c_int;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getuservalue(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io = getuservalue!(L, uvalue(o), L -> top);
    let mut iu: *const Udata = getuservalue!(L, uvalue(o), L -> top);
    let ref mut fresh301 = getuservalue!(L, uvalue(o), L -> top);
    *fresh301 = getuservalue!(L, uvalue(o), L -> top);
    (*io).tt_ = (*iu).ttuv_ as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            730 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int lua_getuservalue(lua_State *, int)\0"))
                .as_ptr(),
        );
    }
    return ttnov!(L -> top - 1);
}
unsafe extern "C" fn auxsetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) {
    let mut slot = 0 as *const TValue;
    let mut str = luaS_new(L, k);
    if luaV_fastset!(L, t, str, slot, luaH_getstr, L -> top - 1) != 0 {
        (*L).top = ((*L).top).offset(-1);
    } else {
        let mut io = setsvalue2s!(L, L -> top, str);
        let mut x_ = setsvalue2s!(L, L -> top, str);
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                751 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"void auxsetstr(lua_State *, const TValue *, const char *)\0"))
                    .as_ptr(),
            );
        }
        luaV_finishset(
            L,
            t,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(2 as libc::c_int as isize)),
            slot,
        );
        (*L).top = ((*L).top).offset(-(2 as libc::c_int as isize));
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setglobal(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) {
    let mut reg: *mut Table = hvalue!(& G(L) -> l_registry);
    auxsetstr(L, luaH_getint(reg, LUA_RIDX_GLOBALS as lua_Integer), name);
}
#[no_mangle]
pub unsafe extern "C" fn lua_settable(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut t = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut slot = 0 as *const TValue;
    if if !((*t).tt_ == 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) {
        slot = NULL as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            ((*L).top).offset(-(2 as libc::c_int as isize)) as *const TValue,
        );
        (if (*slot).tt_ == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            if (*((*L).top).offset(-(1 as libc::c_int as isize))).tt_ & BIT_ISCOLLECTABLE
                != 0
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
        luaV_settable!(
            L, t, L -> top - 2, L -> top - 1
        )(
            L,
            t as *const TValue,
            ((*L).top).offset(-(2 as libc::c_int as isize)),
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            luaV_settable!(L, t, L -> top - 2, L -> top - 1),
        );
    }
    (*L).top = ((*L).top).offset(-(2 as libc::c_int as isize));
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
    let mut t = 0 as *mut TValue;
    let mut slot = 0 as *const TValue;
    t = index2addr(L, idx);
    if luaV_fastset!(L, t, n, slot, luaH_getint, L -> top - 1) != 0 {
        (*L).top = ((*L).top).offset(-1);
    } else {
        let mut io = setivalue!(L -> top, n);
        (*io).value_.i = setivalue!(L -> top, n);
        (*io).tt_ = 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int;
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                793 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void lua_seti(lua_State *, int, lua_Integer)\0"))
                    .as_ptr(),
            );
        }
        luaV_finishset(
            L,
            t as *const TValue,
            ((*L).top).offset(-(1 as libc::c_int as isize)),
            ((*L).top).offset(-(2 as libc::c_int as isize)),
            slot,
        );
        (*L).top = ((*L).top).offset(-(2 as libc::c_int as isize));
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawset(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut o = 0 as *mut TValue;
    let mut slot = 0 as *mut TValue;
    o = index2addr(L, idx);
    slot = luaH_set(
        L,
        hvalue!(o),
        ((*L).top).offset(-(2 as libc::c_int as isize)) as *const TValue,
    );
    (*((*o).value_.gc as *mut GCUnion)).h.flags = 0 as libc::c_int as lu_byte;
    if luaC_barrierback!(L, hvalue(o), L -> top - 1) != 0 {} else {};
    (*L).top = ((*L).top).offset(-(2 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawseti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) {
    let mut o = 0 as *mut TValue;
    o = index2addr(L, idx);
    luaH_setint(L, hvalue!(o), n, ((*L).top).offset(-(1 as libc::c_int as isize)));
    if luaC_barrierback!(L, hvalue(o), L -> top - 1) != 0 {} else {};
    (*L).top = ((*L).top).offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawsetp(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut p: *const libc::c_void,
) {
    let mut o = 0 as *mut TValue;
    let mut k = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut slot = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut libc::c_void;
    (*io).tt_ = 2 as libc::c_int;
    slot = luaH_set(L, hvalue!(o), &mut k);
    if luaC_barrierback!(L, hvalue(o), L -> top - 1) != 0 {} else {};
    (*L).top = ((*L).top).offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_setmetatable(
    mut L: *mut lua_State,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj = 0 as *mut TValue;
    let mut mt = 0 as *mut Table;
    obj = index2addr(L, objindex);
    if ttisnil!(L -> top - 1) != 0 {
        mt = NULL as *mut Table;
    } else {
        mt = hvalue!(L -> top - 1);
    }
    match ttnov!(obj) {
        LUA_TTABLE => {
            let ref mut fresh302 = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
            *fresh302 = mt;
            if !mt.is_null() {
                if luaC_objbarrier!(L, gcvalue(obj), mt) != 0 {} else {};
                luaC_checkfinalizer(L, gcvalue!(obj), mt);
            }
        }
        LUA_TUSERDATA => {
            let ref mut fresh303 = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
            *fresh303 = mt;
            if !mt.is_null() {
                if luaC_objbarrier!(L, uvalue(obj), mt) != 0 {} else {};
                luaC_checkfinalizer(L, gcvalue!(obj), mt);
            }
        }
        _ => {
            let ref mut fresh304 = (*G!(L)).mt[ttnov!(obj) as usize];
            *fresh304 = mt;
        }
    }
    (*L).top = ((*L).top).offset(-1);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setuservalue(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut o = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io = setuservalue!(L, uvalue(o), L -> top - 1);
    let mut iu: *mut Udata = setuservalue!(L, uvalue(o), L -> top - 1);
    let ref mut fresh305 = setuservalue!(L, uvalue(o), L -> top - 1);
    *fresh305 = setuservalue!(L, uvalue(o), L -> top - 1);
    let ref mut fresh306 = setuservalue!(L, uvalue(o), L -> top - 1);
    *fresh306 = (*io).tt_ as lu_byte;
    if luaC_barrier!(L, gcvalue(o), L -> top - 1) != 0 {} else {};
    (*L).top = ((*L).top).offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_callk(
    mut L: *mut lua_State,
    mut nargs: libc::c_int,
    mut nresults: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) {
    let mut func = 0 as *mut TValue;
    func = ((*L).top).offset(-((nargs + 1 as libc::c_int) as isize));
    if k.is_some() && (*L).nny as libc::c_int == 0 as libc::c_int {
        (*(*L).ci).u.c.k = k;
        (*(*L).ci).u.c.ctx = ctx;
        luaD_call(L, func, nresults);
    } else {
        luaD_callnoyield(L, func, nresults);
    }
    if adjustresults!(L, nresults) != 0 {
        (*(*L).ci).top = (*L).top;
    }
}
unsafe extern "C" fn f_call(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut c = cast!(struct CallS *, ud);
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
    let mut c = CallS {
        func: 0 as *mut TValue,
        nresults: 0,
    };
    let mut status: libc::c_int = 0;
    let mut func: ptrdiff_t = 0;
    if errfunc == 0 as libc::c_int {
        func = 0 as libc::c_int as ptrdiff_t;
    } else {
        let mut o = index2addr(L, errfunc);
        func = savestack!(L, o);
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
            savestack!(L, c.func),
            func,
        );
    } else {
        let mut ci = (*L).ci;
        (*ci).u.c.k = k;
        (*ci).u.c.ctx = ctx;
        (*ci).extra = savestack!(L, c.func);
        (*ci).u.c.old_errfunc = (*L).errfunc;
        (*L).errfunc = func;
        let ref mut fresh307 = setoah!(ci -> callstatus, L -> allowhook);
        *fresh307 = setoah!(ci -> callstatus, L -> allowhook);
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int | CIST_YPCALL)
            as libc::c_ushort;
        luaD_call(L, c.func, nresults);
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int & !CIST_YPCALL)
            as libc::c_ushort;
        (*L).errfunc = (*ci).u.c.old_errfunc;
        status = LUA_OK;
    }
    if adjustresults!(L, nresults) != 0 {
        (*(*L).ci).top = (*L).top;
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
    let mut z = ZIO {
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
    if status == LUA_OK {
        let mut f: *mut LClosure = clLvalue!(L -> top - 1);
        if (*f).nupvalues as libc::c_int >= 1 as libc::c_int {
            let mut reg: *mut Table = hvalue!(& G(L) -> l_registry);
            let mut gt = luaH_getint(reg, LUA_RIDX_GLOBALS as lua_Integer);
            let mut io1 = setobj!(L, f -> upvals[0] -> v, gt);
            let ref mut fresh308 = setobj!(L, f -> upvals[0] -> v, gt);
            *fresh308 = setobj!(L, f -> upvals[0] -> v, gt);
            if luaC_upvalbarrier!(L, f -> upvals[0]) != 0 {} else {};
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_dump(
    mut L: *mut lua_State,
    mut writer_0: lua_Writer,
    mut data: *mut libc::c_void,
    mut strip: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut o = 0 as *mut TValue;
    o = ((*L).top).offset(-(1 as libc::c_int as isize));
    if isLfunction!(o) != 0 {
        status = luaU_dump(L, getproto!(o), writer_0, data, strip);
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
    let mut res = 0 as libc::c_int;
    let mut g = 0 as *mut global_State;
    g = G!(L);
    match what {
        LUA_GCSTOP => {
            (*g).gcrunning = 0 as libc::c_int as lu_byte;
        }
        LUA_GCRESTART => {
            luaE_setdebt(g, 0 as libc::c_int as l_mem);
            (*g).gcrunning = 1 as libc::c_int as lu_byte;
        }
        LUA_GCCOLLECT => {
            luaC_fullgc(L, 0 as libc::c_int);
        }
        LUA_GCCOUNT => {
            res = cast_int!(gettotalbytes(g) >> 10);
        }
        LUA_GCCOUNTB => {
            res = cast_int!(gettotalbytes(g) & 0x3ff);
        }
        LUA_GCSTEP => {
            let mut debt = 1 as libc::c_int as l_mem;
            let mut oldrunning = (*g).gcrunning;
            (*g).gcrunning = 1 as libc::c_int as lu_byte;
            if data == 0 as libc::c_int {
                luaE_setdebt(g, -(GCSTEPSIZE as libc::c_int) as l_mem);
                luaC_step(L);
            } else {
                debt = cast!(l_mem, data) * 1024 as libc::c_int as libc::c_long
                    + (*g).GCdebt;
                luaE_setdebt(g, debt);
                if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
                    luaC_checkGC!(L)(L);
                }
            }
            (*g).gcrunning = oldrunning;
            if debt > 0 as libc::c_int as libc::c_long
                && (*g).gcstate as libc::c_int == GCSpause
            {
                res = 1 as libc::c_int;
            }
        }
        LUA_GCSETPAUSE => {
            res = (*g).gcpause;
            (*g).gcpause = data;
        }
        LUA_GCSETSTEPMUL => {
            res = (*g).gcstepmul;
            if data < 40 as libc::c_int {
                data = 40 as libc::c_int;
            }
            (*g).gcstepmul = data;
        }
        LUA_GCISRUNNING => {
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
    let mut t = 0 as *mut TValue;
    let mut more: libc::c_int = 0;
    t = index2addr(L, idx);
    more = luaH_next(L, hvalue!(t), ((*L).top).offset(-(1 as libc::c_int as isize)));
    if more != 0 {
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                1131 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"int lua_next(lua_State *, int)\0"))
                    .as_ptr(),
            );
        }
    } else {
        (*L).top = ((*L).top).offset(-(1 as libc::c_int as isize));
    }
    return more;
}
#[no_mangle]
pub unsafe extern "C" fn lua_concat(mut L: *mut lua_State, mut n: libc::c_int) {
    if n >= 2 as libc::c_int {
        luaV_concat(L, n);
    } else if n == 0 as libc::c_int {
        let mut io = (*L).top;
        let mut x_ = luaS_newlstr(
            L,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                1148 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void lua_concat(lua_State *, int)\0"))
                    .as_ptr(),
            );
        }
    }
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_len(mut L: *mut lua_State, mut idx: libc::c_int) {
    let mut t = 0 as *mut TValue;
    t = index2addr(L, idx);
    luaV_objlen(L, (*L).top, t as *const TValue);
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            1161 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void lua_len(lua_State *, int)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_getallocf(
    mut L: *mut lua_State,
    mut ud: *mut *mut libc::c_void,
) -> lua_Alloc {
    let mut f: lua_Alloc = None;
    if !ud.is_null() {
        *ud = (*G!(L)).ud;
    }
    f = (*G!(L)).frealloc;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setallocf(
    mut L: *mut lua_State,
    mut f: lua_Alloc,
    mut ud: *mut libc::c_void,
) {
    let ref mut fresh309 = (*G!(L)).ud;
    *fresh309 = ud;
    let ref mut fresh310 = (*G!(L)).frealloc;
    *fresh310 = f;
}
#[no_mangle]
pub unsafe extern "C" fn lua_newuserdata(
    mut L: *mut lua_State,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut u = 0 as *mut Udata;
    u = luaS_newudata(L, size);
    let mut io = setuvalue!(L, L -> top, u);
    let mut x_ = setuvalue!(L, L -> top, u);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 7 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int;
    (*L).top = ((*L).top).offset(1);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                as *const libc::c_char,
            b"./lapi.c\0" as *const u8 as *const libc::c_char,
            1189 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"void *lua_newuserdata(lua_State *, size_t)\0"))
                .as_ptr(),
        );
    }
    if (*(*L).l_G).GCdebt > luaC_checkGC!(L) {
        luaC_checkGC!(L)(L);
    }
    return getudatamem!(u);
}
unsafe extern "C" fn aux_upvalue(
    mut fi: StkId,
    mut n: libc::c_int,
    mut val: *mut *mut TValue,
    mut owner: *mut *mut CClosure,
    mut uv: *mut *mut UpVal,
) -> *const libc::c_char {
    match ttype!(fi) {
        LUA_TCCL => {
            let mut f: *mut CClosure = clCvalue!(fi);
            if !(1 as libc::c_int <= n && n <= (*f).nupvalues as libc::c_int) {
                return NULL as *const libc::c_char;
            }
            *val = &mut *((*f).upvalue)
                .as_mut_ptr()
                .offset((n - 1 as libc::c_int) as isize) as *mut TValue;
            if !owner.is_null() {
                *owner = f;
            }
            return b"\0" as *const u8 as *const libc::c_char;
        }
        LUA_TLCL => {
            let mut f_0: *mut LClosure = clLvalue!(fi);
            let mut name = 0 as *mut TString;
            let mut p = (*f_0).p;
            if !(1 as libc::c_int <= n && n <= (*p).sizeupvalues) {
                return NULL as *const libc::c_char;
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
                getstr!(name)
            };
        }
        _ => return NULL as *const libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_getupvalue(
    mut L: *mut lua_State,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name = 0 as *const libc::c_char;
    let mut val = NULL as *mut TValue;
    name = aux_upvalue(
        index2addr(L, funcindex),
        n,
        &mut val,
        NULL as *mut *mut CClosure,
        NULL as *mut *mut UpVal,
    );
    if !name.is_null() {
        let mut io1 = setobj2s!(L, L -> top, val);
        let ref mut fresh311 = setobj2s!(L, L -> top, val);
        *fresh311 = setobj2s!(L, L -> top, val);
        (*L).top = ((*L).top).offset(1);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"./lapi.c\0" as *const u8 as *const libc::c_char,
                1229 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"const char *lua_getupvalue(lua_State *, int, int)\0"))
                    .as_ptr(),
            );
        }
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setupvalue(
    mut L: *mut lua_State,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name = 0 as *const libc::c_char;
    let mut val = NULL as *mut TValue;
    let mut owner = NULL as *mut CClosure;
    let mut uv = NULL as *mut UpVal;
    let mut fi = 0 as *mut TValue;
    fi = index2addr(L, funcindex);
    name = aux_upvalue(fi, n, &mut val, &mut owner, &mut uv);
    if !name.is_null() {
        (*L).top = ((*L).top).offset(-1);
        let mut io1 = setobj!(L, val, L -> top);
        let ref mut fresh312 = setobj!(L, val, L -> top);
        *fresh312 = setobj!(L, val, L -> top);
        if !owner.is_null() {
            if luaC_barrier!(L, owner, L -> top) != 0 {} else {};
        } else if !uv.is_null() {
            if luaC_upvalbarrier!(L, uv) != 0 {} else {};
        }
    }
    return name;
}
unsafe extern "C" fn getupvalref(
    mut L: *mut lua_State,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
) -> *mut *mut UpVal {
    let mut f = 0 as *mut LClosure;
    let mut fi = index2addr(L, fidx);
    f = clLvalue!(fi);
    return &mut *((*f).upvals).as_mut_ptr().offset((n - 1 as libc::c_int) as isize)
        as *mut *mut UpVal;
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvalueid(
    mut L: *mut lua_State,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
) -> *mut libc::c_void {
    let mut fi = index2addr(L, fidx);
    match ttype!(fi) {
        LUA_TLCL => return *getupvalref(L, fidx, n) as *mut libc::c_void,
        LUA_TCCL => {
            let mut f: *mut CClosure = clCvalue!(fi);
            return &mut *((*f).upvalue)
                .as_mut_ptr()
                .offset((n - 1 as libc::c_int) as isize) as *mut TValue
                as *mut libc::c_void;
        }
        _ => return NULL as *mut libc::c_void,
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
    let mut up1 = getupvalref(L, fidx1, n1);
    let mut up2 = getupvalref(L, fidx2, n2);
    if *up1 == *up2 {
        return;
    }
    luaC_upvdeccount(L, *up1);
    *up1 = *up2;
    (**up1).refcount = ((**up1).refcount).wrapping_add(1);
    if upisopen!(* up1) != 0 {
        (**up1).u.open.touched = 1 as libc::c_int;
    }
    if luaC_upvalbarrier!(L, * up1) != 0 {} else {};
}
pub const LUA_ERRFILE: libc::c_int = LUA_ERRERR + 1 as libc::c_int;
pub const LUA_LOADED_TABLE: [libc::c_char; 8] = unsafe {
    *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"_LOADED\0")
};
pub const LUA_PRELOAD_TABLE: [libc::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"_PRELOAD\0")
};
pub const LUAL_NUMSIZES: libc::c_ulong = (::core::mem::size_of::<lua_Integer>()
    as libc::c_ulong)
    .wrapping_mul(16 as libc::c_int as libc::c_ulong)
    .wrapping_add(::core::mem::size_of::<lua_Number>() as libc::c_ulong);
pub const LUA_REFNIL: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub unsafe extern "C" fn luaL_ref(
    mut L: *mut lua_State,
    mut t: libc::c_int,
) -> libc::c_int {
    let mut ref_0: libc::c_int = 0;
    if lua_isnil!(L, - 1) != 0 {
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        return LUA_REFNIL;
    }
    t = lua_absindex(L, t);
    lua_rawgeti(L, t, freelist as lua_Integer);
    ref_0 = lua_tointeger!(L, - 1) as libc::c_int;
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    if ref_0 != 0 as libc::c_int {
        lua_rawgeti(L, t, ref_0 as lua_Integer);
        lua_rawseti(L, t, freelist as lua_Integer);
    } else {
        ref_0 = lua_rawlen(L, t) as libc::c_int + 1 as libc::c_int;
    }
    lua_rawseti(L, t, ref_0 as lua_Integer);
    return ref_0;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_where(mut L: *mut lua_State, mut level: libc::c_int) {
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    if lua_getstack(L, level, &mut ar) != 0 {
        lua_getinfo(L, b"Sl\0" as *const u8 as *const libc::c_char, &mut ar);
        if ar.currentline > 0 as libc::c_int {
            lua_pushfstring(
                L,
                b"%s:%d: \0" as *const u8 as *const libc::c_char,
                (ar.short_src).as_mut_ptr(),
                ar.currentline,
            );
            return;
        }
    }
    lua_pushfstring(L, b"\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_pushresult(mut B: *mut luaL_Buffer) {
    let mut L = (*B).L;
    lua_pushlstring(L, (*B).b, (*B).n);
    if buffonstack!(B) != 0 {
        resizebox(L, -(2 as libc::c_int), 0 as libc::c_int as size_t);
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    }
}
unsafe extern "C" fn resizebox(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut newsize: size_t,
) -> *mut libc::c_void {
    let mut ud = 0 as *mut libc::c_void;
    let mut allocf = lua_getallocf(L, &mut ud);
    let mut box_0 = lua_touserdata(L, idx) as *mut UBox;
    let mut temp = allocf
        .expect(
            "non-null function pointer",
        )(ud, (*box_0).box_0, (*box_0).bsize, newsize);
    if temp.is_null() && newsize > 0 as libc::c_int as libc::c_ulong {
        resizebox(L, idx, 0 as libc::c_int as size_t);
        luaL_error(
            L,
            b"not enough memory for buffer allocation\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*box_0).box_0 = temp;
    (*box_0).bsize = newsize;
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addstring(
    mut B: *mut luaL_Buffer,
    mut s: *const libc::c_char,
) {
    luaL_addlstring(B, s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addlstring(
    mut B: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut l: size_t,
) {
    if l > 0 as libc::c_int as libc::c_ulong {
        let mut b = luaL_prepbuffsize(B, l);
        memcpy(
            b as *mut libc::c_void,
            s as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        let ref mut fresh313 = luaL_addsize!(B, l);
        *fresh313 = (*fresh313 as libc::c_ulong).wrapping_add(luaL_addsize!(B, l))
            as size_t as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaL_prepbuffsize(
    mut B: *mut luaL_Buffer,
    mut sz: size_t,
) -> *mut libc::c_char {
    let mut L = (*B).L;
    if ((*B).size).wrapping_sub((*B).n) < sz {
        let mut newbuff = 0 as *mut libc::c_char;
        let mut newsize = ((*B).size).wrapping_mul(2 as libc::c_int as libc::c_ulong);
        if newsize.wrapping_sub((*B).n) < sz {
            newsize = ((*B).n).wrapping_add(sz);
        }
        if newsize < (*B).n || newsize.wrapping_sub((*B).n) < sz {
            luaL_error(L, b"buffer too large\0" as *const u8 as *const libc::c_char);
        }
        if buffonstack!(B) != 0 {
            newbuff = resizebox(L, -(1 as libc::c_int), newsize) as *mut libc::c_char;
        } else {
            newbuff = newbox(L, newsize) as *mut libc::c_char;
            memcpy(
                newbuff as *mut libc::c_void,
                (*B).b as *const libc::c_void,
                ((*B).n)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            );
        }
        (*B).b = newbuff;
        (*B).size = newsize;
    }
    return &mut *((*B).b).offset((*B).n as isize) as *mut libc::c_char;
}
unsafe extern "C" fn newbox(
    mut L: *mut lua_State,
    mut newsize: size_t,
) -> *mut libc::c_void {
    let mut box_0 = lua_newuserdata(L, ::core::mem::size_of::<UBox>() as libc::c_ulong)
        as *mut UBox;
    (*box_0).box_0 = NULL as *mut libc::c_void;
    (*box_0).bsize = 0 as libc::c_int as size_t;
    if luaL_newmetatable(L, b"LUABOX\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushcfunction!(
            L, boxgc
        )(L, lua_pushcfunction!(L, boxgc), lua_pushcfunction!(L, boxgc));
        lua_setfield(
            L,
            -(2 as libc::c_int),
            b"__gc\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_setmetatable(L, -(2 as libc::c_int));
    return resizebox(L, -(1 as libc::c_int), newsize);
}
unsafe extern "C" fn boxgc(mut L: *mut lua_State) -> libc::c_int {
    resizebox(L, 1 as libc::c_int, 0 as libc::c_int as size_t);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_buffinit(mut L: *mut lua_State, mut B: *mut luaL_Buffer) {
    (*B).L = L;
    (*B).b = ((*B).initb).as_mut_ptr();
    (*B).n = 0 as libc::c_int as size_t;
    (*B).size = LUAL_BUFFERSIZE;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_error(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    luaL_where(L, 1 as libc::c_int);
    lua_pushvfstring(L, fmt, argp.as_va_list());
    lua_concat(L, 2 as libc::c_int);
    return lua_error(L);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_traceback(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut msg: *const libc::c_char,
    mut level: libc::c_int,
) {
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut top = lua_gettop(L);
    let mut last = lastlevel(L1);
    let mut n1 = if last - level > LEVELS1 + LEVELS2 {
        LEVELS1
    } else {
        -(1 as libc::c_int)
    };
    if !msg.is_null() {
        lua_pushfstring(L, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    }
    luaL_checkstack(L, 10 as libc::c_int, NULL as *const libc::c_char);
    lua_pushliteral!(
        L, "stack traceback:"
    )(L, b"stack traceback:\0" as *const u8 as *const libc::c_char);
    loop {
        let fresh314 = level;
        level = level + 1;
        if !(lua_getstack(L1, fresh314, &mut ar) != 0) {
            break;
        }
        let fresh315 = n1;
        n1 = n1 - 1;
        if fresh315 == 0 as libc::c_int {
            lua_pushliteral!(
                L, "\n\t..."
            )(L, b"\n\t...\0" as *const u8 as *const libc::c_char);
            level = last - LEVELS2 + 1 as libc::c_int;
        } else {
            lua_getinfo(L1, b"Slnt\0" as *const u8 as *const libc::c_char, &mut ar);
            lua_pushfstring(
                L,
                b"\n\t%s:\0" as *const u8 as *const libc::c_char,
                (ar.short_src).as_mut_ptr(),
            );
            if ar.currentline > 0 as libc::c_int {
                lua_pushfstring(
                    L,
                    b"%d:\0" as *const u8 as *const libc::c_char,
                    ar.currentline,
                );
            }
            lua_pushliteral!(
                L, " in "
            )(L, b" in \0" as *const u8 as *const libc::c_char);
            pushfuncname(L, &mut ar);
            if ar.istailcall != 0 {
                lua_pushliteral!(
                    L, "\n\t(...tail calls...)"
                )(L, b"\n\t(...tail calls...)\0" as *const u8 as *const libc::c_char);
            }
            lua_concat(L, lua_gettop(L) - top);
        }
    }
    lua_concat(L, lua_gettop(L) - top);
}
unsafe extern "C" fn pushfuncname(mut L: *mut lua_State, mut ar: *mut lua_Debug) {
    if pushglobalfuncname(L, ar) != 0 {
        lua_pushfstring(
            L,
            b"function '%s'\0" as *const u8 as *const libc::c_char,
            lua_tostring!(L, - 1),
        );
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    } else if *(*ar).namewhat as libc::c_int != '\0' as i32 {
        lua_pushfstring(
            L,
            b"%s '%s'\0" as *const u8 as *const libc::c_char,
            (*ar).namewhat,
            (*ar).name,
        );
    } else if *(*ar).what as libc::c_int == 'm' as i32 {
        lua_pushliteral!(
            L, "main chunk"
        )(L, b"main chunk\0" as *const u8 as *const libc::c_char);
    } else if *(*ar).what as libc::c_int != 'C' as i32 {
        lua_pushfstring(
            L,
            b"function <%s:%d>\0" as *const u8 as *const libc::c_char,
            ((*ar).short_src).as_mut_ptr(),
            (*ar).linedefined,
        );
    } else {
        lua_pushliteral!(L, "?")(L, b"?\0" as *const u8 as *const libc::c_char);
    };
}
pub const LEVELS2: libc::c_int = 11 as libc::c_int;
unsafe extern "C" fn lastlevel(mut L: *mut lua_State) -> libc::c_int {
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut li = 1 as libc::c_int;
    let mut le = 1 as libc::c_int;
    while lua_getstack(L, le, &mut ar) != 0 {
        li = le;
        le *= 2 as libc::c_int;
    }
    while li < le {
        let mut m = (li + le) / 2 as libc::c_int;
        if lua_getstack(L, m, &mut ar) != 0 {
            li = m + 1 as libc::c_int;
        } else {
            le = m;
        }
    }
    return le - 1 as libc::c_int;
}
pub const LEVELS1: libc::c_int = 10 as libc::c_int;
pub const LUA_FILEHANDLE: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"FILE*\0")
};
#[no_mangle]
pub unsafe extern "C" fn luaL_addvalue(mut B: *mut luaL_Buffer) {
    let mut L = (*B).L;
    let mut l: size_t = 0;
    let mut s = lua_tolstring(L, -(1 as libc::c_int), &mut l);
    if buffonstack!(B) != 0 {
        lua_insert!(L, - 2)(L, lua_insert!(L, - 2), lua_insert!(L, - 2));
    }
    luaL_addlstring(B, s, l);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_pushresultsize(mut B: *mut luaL_Buffer, mut sz: size_t) {
    let ref mut fresh316 = luaL_addsize!(B, sz);
    *fresh316 = (*fresh316 as libc::c_ulong).wrapping_add(luaL_addsize!(B, sz)) as size_t
        as size_t;
    luaL_pushresult(B);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_buffinitsize(
    mut L: *mut lua_State,
    mut B: *mut luaL_Buffer,
    mut sz: size_t,
) -> *mut libc::c_char {
    luaL_buffinit(L, B);
    return luaL_prepbuffsize(B, sz);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_setmetatable(
    mut L: *mut lua_State,
    mut tname: *const libc::c_char,
) {
    luaL_getmetatable!(L, tname)(L, LUA_REGISTRYINDEX, luaL_getmetatable!(L, tname));
    lua_setmetatable(L, -(2 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn luaL_testudata(
    mut L: *mut lua_State,
    mut ud: libc::c_int,
    mut tname: *const libc::c_char,
) -> *mut libc::c_void {
    let mut p = lua_touserdata(L, ud);
    if !p.is_null() {
        if lua_getmetatable(L, ud) != 0 {
            luaL_getmetatable!(
                L, tname
            )(L, LUA_REGISTRYINDEX, luaL_getmetatable!(L, tname));
            if lua_rawequal(L, -(1 as libc::c_int), -(2 as libc::c_int)) == 0 {
                p = NULL as *mut libc::c_void;
            }
            lua_pop!(L, 2)(L, lua_pop!(L, 2));
            return p;
        }
    }
    return NULL as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkudata(
    mut L: *mut lua_State,
    mut ud: libc::c_int,
    mut tname: *const libc::c_char,
) -> *mut libc::c_void {
    let mut p = luaL_testudata(L, ud, tname);
    if p.is_null() {
        typeerror(L, ud, tname);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkoption(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut def: *const libc::c_char,
    mut lst: *const *const libc::c_char,
) -> libc::c_int {
    let mut name = if !def.is_null() {
        luaL_optstring!(L, arg, def)
    } else {
        luaL_checkstring!(L, arg)
    };
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*lst.offset(i as isize)).is_null() {
        if strcmp(*lst.offset(i as isize), name) == 0 as libc::c_int {
            return i;
        }
        i += 1;
    }
    return luaL_argerror(
        L,
        arg,
        lua_pushfstring(
            L,
            b"invalid option '%s'\0" as *const u8 as *const libc::c_char,
            name,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaL_fileresult(
    mut L: *mut lua_State,
    mut stat: libc::c_int,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut en = errno;
    if stat != 0 {
        lua_pushboolean(L, 1 as libc::c_int);
        return 1 as libc::c_int;
    } else {
        lua_pushnil(L);
        if !fname.is_null() {
            lua_pushfstring(
                L,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                fname,
                strerror(en),
            );
        } else {
            lua_pushstring(L, strerror(en));
        }
        lua_pushinteger(L, en as lua_Integer);
        return 3 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_execresult(
    mut L: *mut lua_State,
    mut stat: libc::c_int,
) -> libc::c_int {
    let mut what = b"exit\0" as *const u8 as *const libc::c_char;
    if stat == -(1 as libc::c_int) {
        return luaL_fileresult(L, 0 as libc::c_int, NULL as *const libc::c_char)
    } else {
        if *what as libc::c_int == 'e' as i32 && stat == 0 as libc::c_int {
            lua_pushboolean(L, 1 as libc::c_int);
        } else {
            lua_pushnil(L);
        }
        lua_pushstring(L, what);
        lua_pushinteger(L, stat as lua_Integer);
        return 3 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_argerror(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut extramsg: *const libc::c_char,
) -> libc::c_int {
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    if lua_getstack(L, 0 as libc::c_int, &mut ar) == 0 {
        return luaL_error(
            L,
            b"bad argument #%d (%s)\0" as *const u8 as *const libc::c_char,
            arg,
            extramsg,
        );
    }
    lua_getinfo(L, b"n\0" as *const u8 as *const libc::c_char, &mut ar);
    if strcmp(ar.namewhat, b"method\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        arg -= 1;
        if arg == 0 as libc::c_int {
            return luaL_error(
                L,
                b"calling '%s' on bad self (%s)\0" as *const u8 as *const libc::c_char,
                ar.name,
                extramsg,
            );
        }
    }
    if (ar.name).is_null() {
        ar
            .name = if pushglobalfuncname(L, &mut ar) != 0 {
            lua_tostring!(L, - 1)
        } else {
            b"?\0" as *const u8 as *const libc::c_char
        };
    }
    return luaL_error(
        L,
        b"bad argument #%d to '%s' (%s)\0" as *const u8 as *const libc::c_char,
        arg,
        ar.name,
        extramsg,
    );
}
pub const freelist: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn luaL_checkany(mut L: *mut lua_State, mut arg: libc::c_int) {
    if lua_type(L, arg) == LUA_TNONE {
        luaL_argerror(L, arg, b"value expected\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaL_newmetatable(
    mut L: *mut lua_State,
    mut tname: *const libc::c_char,
) -> libc::c_int {
    if luaL_getmetatable!(L, tname) != LUA_TNIL {
        return 0 as libc::c_int;
    }
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    lua_createtable(L, 0 as libc::c_int, 2 as libc::c_int);
    lua_pushstring(L, tname);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__name\0" as *const u8 as *const libc::c_char,
    );
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_setfield(L, LUA_REGISTRYINDEX, tname);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checklstring(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut len: *mut size_t,
) -> *const libc::c_char {
    let mut s = lua_tolstring(L, arg, len);
    if s.is_null() {
        tag_error(L, arg, LUA_TSTRING);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checktype(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut t: libc::c_int,
) {
    if lua_type(L, arg) != t {
        tag_error(L, arg, t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkstack(
    mut L: *mut lua_State,
    mut space: libc::c_int,
    mut msg: *const libc::c_char,
) {
    if lua_checkstack(L, space) == 0 {
        if !msg.is_null() {
            luaL_error(
                L,
                b"stack overflow (%s)\0" as *const u8 as *const libc::c_char,
                msg,
            );
        } else {
            luaL_error(L, b"stack overflow\0" as *const u8 as *const libc::c_char);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaL_optinteger(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut def: lua_Integer,
) -> lua_Integer {
    return luaL_opt!(L, luaL_checkinteger, arg, def);
}
unsafe extern "C" fn interror(mut L: *mut lua_State, mut arg: libc::c_int) {
    if lua_isnumber(L, arg) != 0 {
        luaL_argerror(
            L,
            arg,
            b"number has no integer representation\0" as *const u8 as *const libc::c_char,
        );
    } else {
        tag_error(L, arg, LUA_TNUMBER);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkinteger(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
) -> lua_Integer {
    let mut isnum: libc::c_int = 0;
    let mut d = lua_tointegerx(L, arg, &mut isnum);
    if isnum == 0 {
        interror(L, arg);
    }
    return d;
}
unsafe extern "C" fn findfield(
    mut L: *mut lua_State,
    mut objidx: libc::c_int,
    mut level: libc::c_int,
) -> libc::c_int {
    if level == 0 as libc::c_int || lua_istable!(L, - 1) == 0 {
        return 0 as libc::c_int;
    }
    lua_pushnil(L);
    while lua_next(L, -(2 as libc::c_int)) != 0 {
        if lua_type(L, -(2 as libc::c_int)) == LUA_TSTRING {
            if lua_rawequal(L, objidx, -(1 as libc::c_int)) != 0 {
                lua_pop!(L, 1)(L, lua_pop!(L, 1));
                return 1 as libc::c_int;
            } else {
                if findfield(L, objidx, level - 1 as libc::c_int) != 0 {
                    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
                    lua_pushliteral!(
                        L, "."
                    )(L, b".\0" as *const u8 as *const libc::c_char);
                    lua_insert!(L, - 2)(L, lua_insert!(L, - 2), lua_insert!(L, - 2));
                    lua_concat(L, 3 as libc::c_int);
                    return 1 as libc::c_int;
                }
            }
        }
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_optnumber(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut def: lua_Number,
) -> lua_Number {
    return luaL_opt!(L, luaL_checknumber, arg, def);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checknumber(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
) -> lua_Number {
    let mut isnum: libc::c_int = 0;
    let mut d = lua_tonumberx(L, arg, &mut isnum);
    if isnum == 0 {
        tag_error(L, arg, LUA_TNUMBER);
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_optlstring(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut def: *const libc::c_char,
    mut len: *mut size_t,
) -> *const libc::c_char {
    if lua_isnoneornil!(L, arg) != 0 {
        if !len.is_null() {
            *len = if !def.is_null() {
                strlen(def)
            } else {
                0 as libc::c_int as libc::c_ulong
            };
        }
        return def;
    } else {
        return luaL_checklstring(L, arg, len)
    };
}
unsafe extern "C" fn typeerror(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut tname: *const libc::c_char,
) -> libc::c_int {
    let mut msg = 0 as *const libc::c_char;
    let mut typearg = 0 as *const libc::c_char;
    if luaL_getmetafield(L, arg, b"__name\0" as *const u8 as *const libc::c_char)
        == LUA_TSTRING
    {
        typearg = lua_tostring!(L, - 1);
    } else if lua_type(L, arg) == LUA_TLIGHTUSERDATA {
        typearg = b"light userdata\0" as *const u8 as *const libc::c_char;
    } else {
        typearg = luaL_typename!(L, arg);
    }
    msg = lua_pushfstring(
        L,
        b"%s expected, got %s\0" as *const u8 as *const libc::c_char,
        tname,
        typearg,
    );
    return luaL_argerror(L, arg, msg);
}
unsafe extern "C" fn tag_error(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut tag: libc::c_int,
) {
    typeerror(L, arg, lua_typename(L, tag));
}
unsafe extern "C" fn pushglobalfuncname(
    mut L: *mut lua_State,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut top = lua_gettop(L);
    lua_getinfo(L, b"f\0" as *const u8 as *const libc::c_char, ar);
    lua_getfield(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE.as_ptr());
    if findfield(L, top + 1 as libc::c_int, 2 as libc::c_int) != 0 {
        let mut name = lua_tostring!(L, - 1);
        if strncmp(
            name,
            b"_G.\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            lua_pushstring(L, name.offset(3 as libc::c_int as isize));
            lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        }
        lua_copy(L, -(1 as libc::c_int), top + 1 as libc::c_int);
        lua_pop!(L, 2)(L, lua_pop!(L, 2));
        return 1 as libc::c_int;
    } else {
        lua_settop(L, top);
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_getmetafield(
    mut L: *mut lua_State,
    mut obj: libc::c_int,
    mut event: *const libc::c_char,
) -> libc::c_int {
    if lua_getmetatable(L, obj) == 0 {
        return LUA_TNIL
    } else {
        let mut tt: libc::c_int = 0;
        lua_pushstring(L, event);
        tt = lua_rawget(L, -(2 as libc::c_int));
        if tt == LUA_TNIL {
            lua_pop!(L, 2)(L, lua_pop!(L, 2));
        } else {
            lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        }
        return tt;
    };
}
unsafe extern "C" fn errfile(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut fnameindex: libc::c_int,
) -> libc::c_int {
    let mut serr: *const libc::c_char = strerror(errno);
    let mut filename = lua_tostring!(L, fnameindex).offset(1 as libc::c_int as isize);
    lua_pushfstring(
        L,
        b"cannot %s %s: %s\0" as *const u8 as *const libc::c_char,
        what,
        filename,
        serr,
    );
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return LUA_ERRFILE;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_loadfilex(
    mut L: *mut lua_State,
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut lf = LoadF {
        n: 0,
        f: 0 as *mut FILE,
        buff: [0; 8192],
    };
    let mut status: libc::c_int = 0;
    let mut readstatus: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut fnameindex = lua_gettop(L) + 1 as libc::c_int;
    if filename.is_null() {
        lua_pushliteral!(
            L, "=stdin"
        )(L, b"=stdin\0" as *const u8 as *const libc::c_char);
        lf.f = stdin;
    } else {
        lua_pushfstring(L, b"@%s\0" as *const u8 as *const libc::c_char, filename);
        lf.f = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        if (lf.f).is_null() {
            return errfile(L, b"open\0" as *const u8 as *const libc::c_char, fnameindex);
        }
    }
    if skipcomment(&mut lf, &mut c) != 0 {
        let fresh317 = lf.n;
        lf.n = lf.n + 1;
        lf.buff[fresh317 as usize] = '\n' as i32 as libc::c_char;
    }
    if c
        == (*::core::mem::transmute::<
            &[u8; 5],
            &[libc::c_char; 5],
        >(b"\x1BLua\0"))[0 as libc::c_int as usize] as libc::c_int && !filename.is_null()
    {
        lf.f = freopen(filename, b"rb\0" as *const u8 as *const libc::c_char, lf.f);
        if (lf.f).is_null() {
            return errfile(
                L,
                b"reopen\0" as *const u8 as *const libc::c_char,
                fnameindex,
            );
        }
        skipcomment(&mut lf, &mut c);
    }
    if c != EOF {
        let fresh318 = lf.n;
        lf.n = lf.n + 1;
        lf.buff[fresh318 as usize] = c as libc::c_char;
    }
    status = lua_load(
        L,
        Some(
            getF
                as unsafe extern "C" fn(
                    *mut lua_State,
                    *mut libc::c_void,
                    *mut size_t,
                ) -> *const libc::c_char,
        ),
        &mut lf as *mut LoadF as *mut libc::c_void,
        lua_tostring!(L, - 1),
        mode,
    );
    readstatus = ferror(lf.f);
    if !filename.is_null() {
        fclose(lf.f);
    }
    if readstatus != 0 {
        lua_settop(L, fnameindex);
        return errfile(L, b"read\0" as *const u8 as *const libc::c_char, fnameindex);
    }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return status;
}
unsafe extern "C" fn getF(
    mut L: *mut lua_State,
    mut ud: *mut libc::c_void,
    mut size: *mut size_t,
) -> *const libc::c_char {
    let mut lf = ud as *mut LoadF;
    if (*lf).n > 0 as libc::c_int {
        *size = (*lf).n as size_t;
        (*lf).n = 0 as libc::c_int;
    } else {
        if feof((*lf).f) != 0 {
            return NULL as *const libc::c_char;
        }
        *size = fread(
            ((*lf).buff).as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            (*lf).f,
        );
    }
    return ((*lf).buff).as_mut_ptr();
}
unsafe extern "C" fn skipcomment(
    mut lf: *mut LoadF,
    mut cp: *mut libc::c_int,
) -> libc::c_int {
    *cp = skipBOM(lf);
    let mut c = *cp;
    if c == '#' as i32 {
        loop {
            c = getc((*lf).f);
            if !(c != EOF && c != '\n' as i32) {
                break;
            }
        }
        *cp = getc((*lf).f);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn skipBOM(mut lf: *mut LoadF) -> libc::c_int {
    let mut p = b"\xEF\xBB\xBF\0" as *const u8 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    (*lf).n = 0 as libc::c_int;
    loop {
        c = getc((*lf).f);
        if c == EOF
            || {
                let fresh319 = p;
                p = p.offset(1);
                c != *(fresh319 as *const libc::c_uchar) as libc::c_int
            }
        {
            return c;
        }
        let fresh320 = (*lf).n;
        (*lf).n = (*lf).n + 1;
        (*lf).buff[fresh320 as usize] = c as libc::c_char;
        if !(*p as libc::c_int != '\0' as i32) {
            break;
        }
    }
    (*lf).n = 0 as libc::c_int;
    return getc((*lf).f);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_loadbufferx(
    mut L: *mut lua_State,
    mut buff: *const libc::c_char,
    mut size: size_t,
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut ls = LoadS {
        s: 0 as *const libc::c_char,
        size: 0,
    };
    ls.s = buff;
    ls.size = size;
    return lua_load(
        L,
        Some(
            getS
                as unsafe extern "C" fn(
                    *mut lua_State,
                    *mut libc::c_void,
                    *mut size_t,
                ) -> *const libc::c_char,
        ),
        &mut ls as *mut LoadS as *mut libc::c_void,
        name,
        mode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaL_loadstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
) -> libc::c_int {
    return luaL_loadbuffer!(L, s, strlen(s), s);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_unref(
    mut L: *mut lua_State,
    mut t: libc::c_int,
    mut ref_0: libc::c_int,
) {
    if ref_0 >= 0 as libc::c_int {
        t = lua_absindex(L, t);
        lua_rawgeti(L, t, freelist as lua_Integer);
        lua_rawseti(L, t, ref_0 as lua_Integer);
        lua_pushinteger(L, ref_0 as lua_Integer);
        lua_rawseti(L, t, freelist as lua_Integer);
    }
}
unsafe extern "C" fn getS(
    mut L: *mut lua_State,
    mut ud: *mut libc::c_void,
    mut size: *mut size_t,
) -> *const libc::c_char {
    let mut ls = ud as *mut LoadS;
    if (*ls).size == 0 as libc::c_int as libc::c_ulong {
        return NULL as *const libc::c_char;
    }
    *size = (*ls).size;
    (*ls).size = 0 as libc::c_int as size_t;
    return (*ls).s;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_callmeta(
    mut L: *mut lua_State,
    mut obj: libc::c_int,
    mut event: *const libc::c_char,
) -> libc::c_int {
    obj = lua_absindex(L, obj);
    if luaL_getmetafield(L, obj, event) == LUA_TNIL {
        return 0 as libc::c_int;
    }
    lua_pushvalue(L, obj);
    lua_call!(
        L, 1, 1
    )(
        L,
        lua_call!(L, 1, 1),
        lua_call!(L, 1, 1),
        lua_call!(L, 1, 1),
        ::core::mem::transmute::<libc::intptr_t, lua_KFunction>(NULL as libc::intptr_t),
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_len(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> lua_Integer {
    let mut l: lua_Integer = 0;
    let mut isnum: libc::c_int = 0;
    lua_len(L, idx);
    l = lua_tointegerx(L, -(1 as libc::c_int), &mut isnum);
    if isnum == 0 {
        luaL_error(
            L,
            b"object length is not an integer\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_tolstring(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut len: *mut size_t,
) -> *const libc::c_char {
    if luaL_callmeta(L, idx, b"__tostring\0" as *const u8 as *const libc::c_char) != 0 {
        if lua_isstring(L, -(1 as libc::c_int)) == 0 {
            luaL_error(
                L,
                b"'__tostring' must return a string\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        match lua_type(L, idx) {
            LUA_TNUMBER => {
                if lua_isinteger(L, idx) != 0 {
                    lua_pushfstring(
                        L,
                        b"%I\0" as *const u8 as *const libc::c_char,
                        lua_tointeger!(L, idx),
                    );
                } else {
                    lua_pushfstring(
                        L,
                        b"%f\0" as *const u8 as *const libc::c_char,
                        lua_tonumber!(L, idx),
                    );
                }
            }
            LUA_TSTRING => {
                lua_pushvalue(L, idx);
            }
            LUA_TBOOLEAN => {
                lua_pushstring(
                    L,
                    if lua_toboolean(L, idx) != 0 {
                        b"true\0" as *const u8 as *const libc::c_char
                    } else {
                        b"false\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            LUA_TNIL => {
                lua_pushliteral!(
                    L, "nil"
                )(L, b"nil\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                let mut tt = luaL_getmetafield(
                    L,
                    idx,
                    b"__name\0" as *const u8 as *const libc::c_char,
                );
                let mut kind = if tt == LUA_TSTRING {
                    lua_tostring!(L, - 1)
                } else {
                    luaL_typename!(L, idx)
                };
                lua_pushfstring(
                    L,
                    b"%s: %p\0" as *const u8 as *const libc::c_char,
                    kind,
                    lua_topointer(L, idx),
                );
                if tt != LUA_TNIL {
                    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
                }
            }
        }
    }
    return lua_tolstring(L, -(1 as libc::c_int), len);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_setfuncs(
    mut L: *mut lua_State,
    mut l: *const luaL_Reg,
    mut nup: libc::c_int,
) {
    luaL_checkstack(L, nup, b"too many upvalues\0" as *const u8 as *const libc::c_char);
    while !((*l).name).is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < nup {
            lua_pushvalue(L, -nup);
            i += 1;
        }
        lua_pushcclosure(L, (*l).func, nup);
        lua_setfield(L, -(nup + 2 as libc::c_int), (*l).name);
        l = l.offset(1);
    }
    lua_pop!(L, nup)(L, lua_pop!(L, nup));
}
#[no_mangle]
pub unsafe extern "C" fn luaL_getsubtable(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    if lua_getfield(L, idx, fname) == LUA_TTABLE {
        return 1 as libc::c_int
    } else {
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        idx = lua_absindex(L, idx);
        lua_newtable!(L)(L, lua_newtable!(L), lua_newtable!(L));
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_setfield(L, idx, fname);
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_requiref(
    mut L: *mut lua_State,
    mut modname: *const libc::c_char,
    mut openf: lua_CFunction,
    mut glb: libc::c_int,
) {
    luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE.as_ptr());
    lua_getfield(L, -(1 as libc::c_int), modname);
    if lua_toboolean(L, -(1 as libc::c_int)) == 0 {
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        lua_pushcfunction!(
            L, openf
        )(L, lua_pushcfunction!(L, openf), lua_pushcfunction!(L, openf));
        lua_pushstring(L, modname);
        lua_call!(
            L, 1, 1
        )(
            L,
            lua_call!(L, 1, 1),
            lua_call!(L, 1, 1),
            lua_call!(L, 1, 1),
            ::core::mem::transmute::<
                libc::intptr_t,
                lua_KFunction,
            >(NULL as libc::intptr_t),
        );
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_setfield(L, -(3 as libc::c_int), modname);
    }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    if glb != 0 {
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_setglobal(L, modname);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaL_gsub(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut r: *const libc::c_char,
) -> *const libc::c_char {
    let mut wild = 0 as *const libc::c_char;
    let mut l = strlen(p);
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    loop {
        wild = strstr(s, p);
        if wild.is_null() {
            break;
        }
        luaL_addlstring(&mut b, s, wild.offset_from(s) as libc::c_long as size_t);
        luaL_addstring(&mut b, r);
        s = wild.offset(l as isize);
    }
    luaL_addstring(&mut b, s);
    luaL_pushresult(&mut b);
    return lua_tostring!(L, - 1);
}
unsafe extern "C" fn l_alloc(
    mut ud: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut libc::c_void {
    if nsize == 0 as libc::c_int as libc::c_ulong {
        free(ptr);
        return NULL as *mut libc::c_void;
    } else {
        let mut newptr = realloc(ptr, nsize);
        if newptr.is_null() && !ptr.is_null() && nsize <= osize {
            return ptr
        } else {
            return newptr
        }
    };
}
unsafe extern "C" fn panic(mut L: *mut lua_State) -> libc::c_int {
    fprintf(
        stderr,
        b"PANIC: unprotected error in call to Lua API (%s)\n\0" as *const u8
            as *const libc::c_char,
        lua_tolstring(L, -(1 as libc::c_int), 0 as *mut size_t),
    );
    fflush(stderr);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_newstate() -> *mut lua_State {
    let mut L = lua_newstate(
        Some(
            l_alloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
        ),
        NULL as *mut libc::c_void,
    );
    if !L.is_null() {
        lua_atpanic(
            L,
            Some(panic as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        );
    }
    return L;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkversion_(
    mut L: *mut lua_State,
    mut ver: lua_Number,
    mut sz: size_t,
) {
    let mut v = lua_version(L);
    if sz != LUAL_NUMSIZES {
        luaL_error(
            L,
            b"core and library have incompatible numeric types\0" as *const u8
                as *const libc::c_char,
        );
    }
    if v != lua_version(NULL as *mut lua_State) {
        luaL_error(
            L,
            b"multiple Lua VMs detected\0" as *const u8 as *const libc::c_char,
        );
    } else if *v != ver {
        luaL_error(
            L,
            b"version mismatch: app. needs %f, Lua core provides %f\0" as *const u8
                as *const libc::c_char,
            ver,
            *v,
        );
    }
}
pub const LUA_COLIBNAME: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"coroutine\0")
};
pub const LUA_TABLIBNAME: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"table\0")
};
pub const LUA_IOLIBNAME: [libc::c_char; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"io\0")
};
pub const LUA_OSLIBNAME: [libc::c_char; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"os\0")
};
pub const LUA_STRLIBNAME: [libc::c_char; 7] = unsafe {
    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"string\0")
};
pub const LUA_UTF8LIBNAME: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"utf8\0")
};
unsafe extern "C" fn luaB_pcall(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    luaL_checkany(L, 1 as libc::c_int);
    lua_pushboolean(L, 1 as libc::c_int);
    lua_insert!(L, 1)(L, lua_insert!(L, 1), lua_insert!(L, 1));
    status = lua_pcallk(
        L,
        lua_gettop(L) - 2 as libc::c_int,
        LUA_MULTRET,
        0 as libc::c_int,
        0 as libc::c_int as lua_KContext,
        Some(
            finishpcall
                as unsafe extern "C" fn(
                    *mut lua_State,
                    libc::c_int,
                    lua_KContext,
                ) -> libc::c_int,
        ),
    );
    return finishpcall(L, status, 0 as libc::c_int as lua_KContext);
}
unsafe extern "C" fn pairsmeta(
    mut L: *mut lua_State,
    mut method: *const libc::c_char,
    mut iszero: libc::c_int,
    mut iter: lua_CFunction,
) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if luaL_getmetafield(L, 1 as libc::c_int, method) == LUA_TNIL {
        lua_pushcfunction!(
            L, iter
        )(L, lua_pushcfunction!(L, iter), lua_pushcfunction!(L, iter));
        lua_pushvalue(L, 1 as libc::c_int);
        if iszero != 0 {
            lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
        } else {
            lua_pushnil(L);
        }
    } else {
        lua_pushvalue(L, 1 as libc::c_int);
        lua_call!(
            L, 1, 3
        )(
            L,
            lua_call!(L, 1, 3),
            lua_call!(L, 1, 3),
            lua_call!(L, 1, 3),
            ::core::mem::transmute::<
                libc::intptr_t,
                lua_KFunction,
            >(NULL as libc::intptr_t),
        );
    }
    return 3 as libc::c_int;
}
unsafe extern "C" fn luaB_load(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut l: size_t = 0;
    let mut s = lua_tolstring(L, 1 as libc::c_int, &mut l);
    let mut mode = luaL_optstring!(L, 3, "bt");
    let mut env = if lua_isnone!(L, 4) == 0 {
        4 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if !s.is_null() {
        let mut chunkname = luaL_optstring!(L, 2, s);
        status = luaL_loadbufferx(L, s, l, chunkname, mode);
    } else {
        let mut chunkname_0 = luaL_optstring!(L, 2, "=(load)");
        luaL_checktype(L, 1 as libc::c_int, LUA_TFUNCTION);
        lua_settop(L, RESERVEDSLOT);
        status = lua_load(
            L,
            Some(
                generic_reader
                    as unsafe extern "C" fn(
                        *mut lua_State,
                        *mut libc::c_void,
                        *mut size_t,
                    ) -> *const libc::c_char,
            ),
            NULL as *mut libc::c_void,
            chunkname_0,
            mode,
        );
    }
    return load_aux(L, status, env);
}
unsafe extern "C" fn load_aux(
    mut L: *mut lua_State,
    mut status: libc::c_int,
    mut envidx: libc::c_int,
) -> libc::c_int {
    if status == LUA_OK {
        if envidx != 0 as libc::c_int {
            lua_pushvalue(L, envidx);
            if (lua_setupvalue(L, -(2 as libc::c_int), 1 as libc::c_int)).is_null() {
                lua_pop!(L, 1)(L, lua_pop!(L, 1));
            }
        }
        return 1 as libc::c_int;
    } else {
        lua_pushnil(L);
        lua_insert!(L, - 2)(L, lua_insert!(L, - 2), lua_insert!(L, - 2));
        return 2 as libc::c_int;
    };
}
pub const LUA_LOADLIBNAME: [libc::c_char; 8] = unsafe {
    *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"package\0")
};
pub const LUA_MATHLIBNAME: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"math\0")
};
pub const LUA_DBLIBNAME: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"debug\0")
};
unsafe extern "C" fn generic_reader(
    mut L: *mut lua_State,
    mut ud: *mut libc::c_void,
    mut size: *mut size_t,
) -> *const libc::c_char {
    luaL_checkstack(
        L,
        2 as libc::c_int,
        b"too many nested functions\0" as *const u8 as *const libc::c_char,
    );
    lua_pushvalue(L, 1 as libc::c_int);
    lua_call!(
        L, 0, 1
    )(
        L,
        lua_call!(L, 0, 1),
        lua_call!(L, 0, 1),
        lua_call!(L, 0, 1),
        ::core::mem::transmute::<libc::intptr_t, lua_KFunction>(NULL as libc::intptr_t),
    );
    if lua_isnil!(L, - 1) != 0 {
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        *size = 0 as libc::c_int as size_t;
        return NULL as *const libc::c_char;
    } else {
        if lua_isstring(L, -(1 as libc::c_int)) == 0 {
            luaL_error(
                L,
                b"reader function must return a string\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return lua_tolstring(L, RESERVEDSLOT, size);
}
pub const RESERVEDSLOT: libc::c_int = 5 as libc::c_int;
unsafe extern "C" fn luaB_loadfile(mut L: *mut lua_State) -> libc::c_int {
    let mut fname = luaL_optstring!(L, 1, NULL);
    let mut mode = luaL_optstring!(L, 2, NULL);
    let mut env = if lua_isnone!(L, 3) == 0 {
        3 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut status = luaL_loadfilex(L, fname, mode);
    return load_aux(L, status, env);
}
unsafe extern "C" fn luaB_ipairs(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    lua_pushcfunction!(
        L, ipairsaux
    )(L, lua_pushcfunction!(L, ipairsaux), lua_pushcfunction!(L, ipairsaux));
    lua_pushvalue(L, 1 as libc::c_int);
    lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
    return 3 as libc::c_int;
}
unsafe extern "C" fn finishpcall(
    mut L: *mut lua_State,
    mut status: libc::c_int,
    mut extra: lua_KContext,
) -> libc::c_int {
    if status != LUA_OK && status != LUA_YIELD {
        lua_pushboolean(L, 0 as libc::c_int);
        lua_pushvalue(L, -(2 as libc::c_int));
        return 2 as libc::c_int;
    } else {
        return lua_gettop(L) - extra as libc::c_int
    };
}
unsafe extern "C" fn ipairsaux(mut L: *mut lua_State) -> libc::c_int {
    let mut i = luaL_checkinteger(L, 2 as libc::c_int)
        + 1 as libc::c_int as libc::c_longlong;
    lua_pushinteger(L, i);
    return if lua_geti(L, 1 as libc::c_int, i) == LUA_TNIL {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
}
unsafe extern "C" fn luaB_type(mut L: *mut lua_State) -> libc::c_int {
    let mut t = lua_type(L, 1 as libc::c_int);
    lua_pushstring(L, lua_typename(L, t));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_getmetatable(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if lua_getmetatable(L, 1 as libc::c_int) == 0 {
        lua_pushnil(L);
        return 1 as libc::c_int;
    }
    luaL_getmetafield(
        L,
        1 as libc::c_int,
        b"__metatable\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_error(mut L: *mut lua_State) -> libc::c_int {
    let mut level = luaL_optinteger(L, 2 as libc::c_int, 1 as libc::c_int as lua_Integer)
        as libc::c_int;
    lua_settop(L, 1 as libc::c_int);
    if lua_type(L, 1 as libc::c_int) == LUA_TSTRING && level > 0 as libc::c_int {
        luaL_where(L, level);
        lua_pushvalue(L, 1 as libc::c_int);
        lua_concat(L, 2 as libc::c_int);
    }
    return lua_error(L);
}
unsafe extern "C" fn luaB_dofile(mut L: *mut lua_State) -> libc::c_int {
    let mut fname = luaL_optstring!(L, 1, NULL);
    lua_settop(L, 1 as libc::c_int);
    if luaL_loadfile!(L, fname) != LUA_OK {
        return lua_error(L);
    }
    lua_callk(
        L,
        0 as libc::c_int,
        LUA_MULTRET,
        0 as libc::c_int as lua_KContext,
        Some(
            dofilecont
                as unsafe extern "C" fn(
                    *mut lua_State,
                    libc::c_int,
                    lua_KContext,
                ) -> libc::c_int,
        ),
    );
    return dofilecont(L, 0 as libc::c_int, 0 as libc::c_int as lua_KContext);
}
unsafe extern "C" fn luaB_pairs(mut L: *mut lua_State) -> libc::c_int {
    return pairsmeta(
        L,
        b"__pairs\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(luaB_next as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
}
unsafe extern "C" fn luaB_next(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, LUA_TTABLE);
    lua_settop(L, 2 as libc::c_int);
    if lua_next(L, 1 as libc::c_int) != 0 {
        return 2 as libc::c_int
    } else {
        lua_pushnil(L);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn luaB_print(mut L: *mut lua_State) -> libc::c_int {
    let mut n = lua_gettop(L);
    let mut i: libc::c_int = 0;
    lua_getglobal(L, b"tostring\0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i <= n {
        let mut s = 0 as *const libc::c_char;
        let mut l: size_t = 0;
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_pushvalue(L, i);
        lua_call!(
            L, 1, 1
        )(
            L,
            lua_call!(L, 1, 1),
            lua_call!(L, 1, 1),
            lua_call!(L, 1, 1),
            ::core::mem::transmute::<
                libc::intptr_t,
                lua_KFunction,
            >(NULL as libc::intptr_t),
        );
        s = lua_tolstring(L, -(1 as libc::c_int), &mut l);
        if s.is_null() {
            return luaL_error(
                L,
                b"'tostring' must return a string to 'print'\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if i > 1 as libc::c_int {
            lua_writestring!(
                "\t", 1
            )(
                lua_writestring!("\t", 1),
                lua_writestring!("\t", 1),
                lua_writestring!("\t", 1),
                stdout,
            );
        }
        lua_writestring!(
            s, l
        )(
            lua_writestring!(s, l),
            lua_writestring!(s, l),
            lua_writestring!(s, l),
            stdout,
        );
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        i += 1;
    }
    fwrite(
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stdout,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaB_rawequal(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    lua_pushboolean(L, lua_rawequal(L, 1 as libc::c_int, 2 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawlen(mut L: *mut lua_State) -> libc::c_int {
    let mut t = lua_type(L, 1 as libc::c_int);
    lua_pushinteger(L, lua_rawlen(L, 1 as libc::c_int) as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawget(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, LUA_TTABLE);
    luaL_checkany(L, 2 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    lua_rawget(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn dofilecont(
    mut L: *mut lua_State,
    mut d1: libc::c_int,
    mut d2: lua_KContext,
) -> libc::c_int {
    return lua_gettop(L) - 1 as libc::c_int;
}
unsafe extern "C" fn luaB_tonumber(mut L: *mut lua_State) -> libc::c_int {
    if lua_isnoneornil!(L, 2) != 0 {
        luaL_checkany(L, 1 as libc::c_int);
        if lua_type(L, 1 as libc::c_int) == LUA_TNUMBER {
            lua_settop(L, 1 as libc::c_int);
            return 1 as libc::c_int;
        } else {
            let mut l: size_t = 0;
            let mut s = lua_tolstring(L, 1 as libc::c_int, &mut l);
            if !s.is_null()
                && lua_stringtonumber(L, s)
                    == l.wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                return 1 as libc::c_int;
            }
        }
    } else {
        let mut l_0: size_t = 0;
        let mut s_0 = 0 as *const libc::c_char;
        let mut n = 0 as libc::c_int as lua_Integer;
        let mut base = luaL_checkinteger(L, 2 as libc::c_int);
        luaL_checktype(L, 1 as libc::c_int, LUA_TSTRING);
        s_0 = lua_tolstring(L, 1 as libc::c_int, &mut l_0);
        if b_str2int(s_0, base as libc::c_int, &mut n) == s_0.offset(l_0 as isize) {
            lua_pushinteger(L, n);
            return 1 as libc::c_int;
        }
    }
    lua_pushnil(L);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawset(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, LUA_TTABLE);
    luaL_checkany(L, 2 as libc::c_int);
    luaL_checkany(L, 3 as libc::c_int);
    lua_settop(L, 3 as libc::c_int);
    lua_rawset(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_select(mut L: *mut lua_State) -> libc::c_int {
    let mut n = lua_gettop(L);
    if lua_type(L, 1 as libc::c_int) == LUA_TSTRING
        && *lua_tostring!(L, 1) as libc::c_int == '#' as i32
    {
        lua_pushinteger(L, (n - 1 as libc::c_int) as lua_Integer);
        return 1 as libc::c_int;
    } else {
        let mut i = luaL_checkinteger(L, 1 as libc::c_int);
        if i < 0 as libc::c_int as libc::c_longlong {
            i = n as libc::c_longlong + i;
        } else if i > n as libc::c_longlong {
            i = n as lua_Integer;
        }
        return n - i as libc::c_int;
    };
}
unsafe extern "C" fn luaB_setmetatable(mut L: *mut lua_State) -> libc::c_int {
    let mut t = lua_type(L, 2 as libc::c_int);
    luaL_checktype(L, 1 as libc::c_int, LUA_TTABLE);
    if luaL_getmetafield(
        L,
        1 as libc::c_int,
        b"__metatable\0" as *const u8 as *const libc::c_char,
    ) != LUA_TNIL
    {
        return luaL_error(
            L,
            b"cannot change a protected metatable\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_settop(L, 2 as libc::c_int);
    lua_setmetatable(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn b_str2int(
    mut s: *const libc::c_char,
    mut base: libc::c_int,
    mut pn: *mut lua_Integer,
) -> *const libc::c_char {
    let mut n = 0 as libc::c_int as lua_Unsigned;
    let mut neg = 0 as libc::c_int;
    s = s.offset(strspn(s, SPACECHARS.as_ptr()) as isize);
    if *s as libc::c_int == '-' as i32 {
        s = s.offset(1);
        neg = 1 as libc::c_int;
    } else if *s as libc::c_int == '+' as i32 {
        s = s.offset(1);
    }
    if isalnum!((unsigned char) * s) == 0 {
        return NULL as *const libc::c_char;
    }
    loop {
        let mut digit_0 = if isdigit!((unsigned char) * s)
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            *s as libc::c_int - '0' as i32
        } else {
            toupper!((unsigned char) * s) - 'A' as i32 + 10 as libc::c_int
        };
        if digit_0 >= base {
            return NULL as *const libc::c_char;
        }
        n = n
            .wrapping_mul(base as libc::c_ulonglong)
            .wrapping_add(digit_0 as libc::c_ulonglong);
        s = s.offset(1);
        if !(isalnum!((unsigned char) * s) != 0) {
            break;
        }
    }
    s = s.offset(strspn(s, SPACECHARS.as_ptr()) as isize);
    *pn = (if neg != 0 {
        (0 as libc::c_uint as libc::c_ulonglong).wrapping_sub(n)
    } else {
        n
    }) as lua_Integer;
    return s;
}
pub const SPACECHARS: [libc::c_char; 7] = unsafe {
    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b" \x0C\n\r\t\x0B\0")
};
unsafe extern "C" fn luaB_collectgarbage(mut L: *mut lua_State) -> libc::c_int {
    static mut opts: [*const libc::c_char; 9] = [
        b"stop\0" as *const u8 as *const libc::c_char,
        b"restart\0" as *const u8 as *const libc::c_char,
        b"collect\0" as *const u8 as *const libc::c_char,
        b"count\0" as *const u8 as *const libc::c_char,
        b"step\0" as *const u8 as *const libc::c_char,
        b"setpause\0" as *const u8 as *const libc::c_char,
        b"setstepmul\0" as *const u8 as *const libc::c_char,
        b"isrunning\0" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut optsnum: [libc::c_int; 8] = [
        LUA_GCSTOP,
        LUA_GCRESTART,
        LUA_GCCOLLECT,
        LUA_GCCOUNT,
        LUA_GCSTEP,
        LUA_GCSETPAUSE,
        LUA_GCSETSTEPMUL,
        LUA_GCISRUNNING,
    ];
    let mut o = optsnum[luaL_checkoption(
        L,
        1 as libc::c_int,
        b"collect\0" as *const u8 as *const libc::c_char,
        opts.as_ptr(),
    ) as usize];
    let mut ex = luaL_optinteger(L, 2 as libc::c_int, 0 as libc::c_int as lua_Integer)
        as libc::c_int;
    let mut res = lua_gc(L, o, ex);
    match o {
        LUA_GCCOUNT => {
            let mut b = lua_gc(L, LUA_GCCOUNTB, 0 as libc::c_int);
            lua_pushnumber(
                L,
                res as lua_Number
                    + b as lua_Number / 1024 as libc::c_int as libc::c_double,
            );
            return 1 as libc::c_int;
        }
        LUA_GCSTEP | LUA_GCISRUNNING => {
            lua_pushboolean(L, res);
            return 1 as libc::c_int;
        }
        _ => {
            lua_pushinteger(L, res as lua_Integer);
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn luaB_assert(mut L: *mut lua_State) -> libc::c_int {
    if lua_toboolean(L, 1 as libc::c_int) != 0 {
        return lua_gettop(L)
    } else {
        luaL_checkany(L, 1 as libc::c_int);
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        lua_pushliteral!(
            L, "assertion failed!"
        )(L, b"assertion failed!\0" as *const u8 as *const libc::c_char);
        lua_settop(L, 1 as libc::c_int);
        return luaB_error(L);
    };
}
unsafe extern "C" fn luaB_xpcall(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut n = lua_gettop(L);
    luaL_checktype(L, 2 as libc::c_int, LUA_TFUNCTION);
    lua_pushboolean(L, 1 as libc::c_int);
    lua_pushvalue(L, 1 as libc::c_int);
    lua_rotate(L, 3 as libc::c_int, 2 as libc::c_int);
    status = lua_pcallk(
        L,
        n - 2 as libc::c_int,
        LUA_MULTRET,
        2 as libc::c_int,
        2 as libc::c_int as lua_KContext,
        Some(
            finishpcall
                as unsafe extern "C" fn(
                    *mut lua_State,
                    libc::c_int,
                    lua_KContext,
                ) -> libc::c_int,
        ),
    );
    return finishpcall(L, status, 2 as libc::c_int as lua_KContext);
}
unsafe extern "C" fn luaB_tostring(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    luaL_tolstring(L, 1 as libc::c_int, NULL as *mut size_t);
    return 1 as libc::c_int;
}
static mut base_funcs: [luaL_Reg; 25] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"assert\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_assert as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"collectgarbage\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_collectgarbage
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"dofile\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_dofile as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"error\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_error as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_getmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ipairs\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_ipairs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"loadfile\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_loadfile as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"load\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_load as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"next\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_next as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pairs\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_pairs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pcall\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_pcall as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"print\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_print as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawequal\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawequal as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawlen\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawlen as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawget\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawget as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawset\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawset as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"select\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_select as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_setmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tonumber\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_tonumber as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tostring\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_tostring as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_type as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"xpcall\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_xpcall as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"_G\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"_VERSION\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaopen_base(mut L: *mut lua_State) -> libc::c_int {
    luaL_setfuncs(L, base_funcs.as_ptr(), 0 as libc::c_int);
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_setfield(L, -(2 as libc::c_int), b"_G\0" as *const u8 as *const libc::c_char);
    lua_pushliteral!(
        L, LUA_VERSION
    )(L, b"Lua 5.3\0" as *const u8 as *const libc::c_char);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"_VERSION\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn getco(mut L: *mut lua_State) -> *mut lua_State {
    let mut co = lua_tothread(L, 1 as libc::c_int);
    return co;
}
unsafe extern "C" fn auxresume(
    mut L: *mut lua_State,
    mut co: *mut lua_State,
    mut narg: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if lua_checkstack(co, narg) == 0 {
        lua_pushliteral!(
            L, "too many arguments to resume"
        )(L, b"too many arguments to resume\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if lua_status(co) == LUA_OK && lua_gettop(co) == 0 as libc::c_int {
        lua_pushliteral!(
            L, "cannot resume dead coroutine"
        )(L, b"cannot resume dead coroutine\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    lua_xmove(L, co, narg);
    status = lua_resume(co, L, narg);
    if status == LUA_OK || status == LUA_YIELD {
        let mut nres = lua_gettop(co);
        if lua_checkstack(L, nres + 1 as libc::c_int) == 0 {
            lua_pop!(co, nres)(co, lua_pop!(co, nres));
            lua_pushliteral!(
                L, "too many results to resume"
            )(L, b"too many results to resume\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        lua_xmove(co, L, nres);
        return nres;
    } else {
        lua_xmove(co, L, 1 as libc::c_int);
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn luaB_coresume(mut L: *mut lua_State) -> libc::c_int {
    let mut co = getco(L);
    let mut r: libc::c_int = 0;
    r = auxresume(L, co, lua_gettop(L) - 1 as libc::c_int);
    if r < 0 as libc::c_int {
        lua_pushboolean(L, 0 as libc::c_int);
        lua_insert!(L, - 2)(L, lua_insert!(L, - 2), lua_insert!(L, - 2));
        return 2 as libc::c_int;
    } else {
        lua_pushboolean(L, 1 as libc::c_int);
        lua_rotate(L, -(r + 1 as libc::c_int), 1 as libc::c_int);
        return r + 1 as libc::c_int;
    };
}
unsafe extern "C" fn luaB_auxwrap(mut L: *mut lua_State) -> libc::c_int {
    let mut co = lua_tothread(L, lua_upvalueindex!(1));
    let mut r = auxresume(L, co, lua_gettop(L));
    if r < 0 as libc::c_int {
        if lua_type(L, -(1 as libc::c_int)) == LUA_TSTRING {
            luaL_where(L, 1 as libc::c_int);
            lua_insert!(L, - 2)(L, lua_insert!(L, - 2), lua_insert!(L, - 2));
            lua_concat(L, 2 as libc::c_int);
        }
        return lua_error(L);
    }
    return r;
}
unsafe extern "C" fn luaB_cocreate(mut L: *mut lua_State) -> libc::c_int {
    let mut NL = 0 as *mut lua_State;
    luaL_checktype(L, 1 as libc::c_int, LUA_TFUNCTION);
    NL = lua_newthread(L);
    lua_pushvalue(L, 1 as libc::c_int);
    lua_xmove(L, NL, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_cowrap(mut L: *mut lua_State) -> libc::c_int {
    luaB_cocreate(L);
    lua_pushcclosure(
        L,
        Some(luaB_auxwrap as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        1 as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_yield(mut L: *mut lua_State) -> libc::c_int {
    return lua_yieldk(
        L,
        lua_gettop(L),
        0 as libc::c_int as lua_KContext,
        ::core::mem::transmute::<libc::intptr_t, lua_KFunction>(NULL as libc::intptr_t),
    );
}
unsafe extern "C" fn luaB_costatus(mut L: *mut lua_State) -> libc::c_int {
    let mut co = getco(L);
    if L == co {
        lua_pushliteral!(
            L, "running"
        )(L, b"running\0" as *const u8 as *const libc::c_char);
    } else {
        match lua_status(co) {
            LUA_YIELD => {
                lua_pushliteral!(
                    L, "suspended"
                )(L, b"suspended\0" as *const u8 as *const libc::c_char);
            }
            LUA_OK => {
                let mut ar = lua_Debug {
                    event: 0,
                    name: 0 as *const libc::c_char,
                    namewhat: 0 as *const libc::c_char,
                    what: 0 as *const libc::c_char,
                    source: 0 as *const libc::c_char,
                    currentline: 0,
                    linedefined: 0,
                    lastlinedefined: 0,
                    nups: 0,
                    nparams: 0,
                    isvararg: 0,
                    istailcall: 0,
                    short_src: [0; 60],
                    i_ci: 0 as *mut CallInfo,
                };
                if lua_getstack(co, 0 as libc::c_int, &mut ar) > 0 as libc::c_int {
                    lua_pushliteral!(
                        L, "normal"
                    )(L, b"normal\0" as *const u8 as *const libc::c_char);
                } else if lua_gettop(co) == 0 as libc::c_int {
                    lua_pushliteral!(
                        L, "dead"
                    )(L, b"dead\0" as *const u8 as *const libc::c_char);
                } else {
                    lua_pushliteral!(
                        L, "suspended"
                    )(L, b"suspended\0" as *const u8 as *const libc::c_char);
                }
            }
            _ => {
                lua_pushliteral!(
                    L, "dead"
                )(L, b"dead\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_yieldable(mut L: *mut lua_State) -> libc::c_int {
    lua_pushboolean(L, lua_isyieldable(L));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_corunning(mut L: *mut lua_State) -> libc::c_int {
    let mut ismain = lua_pushthread(L);
    lua_pushboolean(L, ismain);
    return 2 as libc::c_int;
}
static mut co_funcs: [luaL_Reg; 8] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"create\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_cocreate as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"resume\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_coresume as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"running\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_corunning as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"status\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_costatus as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"wrap\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_cowrap as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"yield\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_yield as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"isyieldable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_yieldable as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaopen_coroutine(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    return 1 as libc::c_int;
}
static mut HOOKKEY: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn checkstack(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut n: libc::c_int,
) {
    if L != L1 && lua_checkstack(L1, n) == 0 {
        luaL_error(L, b"stack overflow\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn db_getregistry(mut L: *mut lua_State) -> libc::c_int {
    lua_pushvalue(L, LUA_REGISTRYINDEX);
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_getmetatable(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if lua_getmetatable(L, 1 as libc::c_int) == 0 {
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_setmetatable(mut L: *mut lua_State) -> libc::c_int {
    let mut t = lua_type(L, 2 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    lua_setmetatable(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_getuservalue(mut L: *mut lua_State) -> libc::c_int {
    if lua_type(L, 1 as libc::c_int) != LUA_TUSERDATA {
        lua_pushnil(L);
    } else {
        lua_getuservalue(L, 1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_setuservalue(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, LUA_TUSERDATA);
    luaL_checkany(L, 2 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    lua_setuservalue(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn getthread(
    mut L: *mut lua_State,
    mut arg: *mut libc::c_int,
) -> *mut lua_State {
    if lua_isthread!(L, 1) != 0 {
        *arg = 1 as libc::c_int;
        return lua_tothread(L, 1 as libc::c_int);
    } else {
        *arg = 0 as libc::c_int;
        return L;
    };
}
unsafe extern "C" fn settabss(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: *const libc::c_char,
) {
    lua_pushstring(L, v);
    lua_setfield(L, -(2 as libc::c_int), k);
}
unsafe extern "C" fn settabsi(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: libc::c_int,
) {
    lua_pushinteger(L, v as lua_Integer);
    lua_setfield(L, -(2 as libc::c_int), k);
}
unsafe extern "C" fn settabsb(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: libc::c_int,
) {
    lua_pushboolean(L, v);
    lua_setfield(L, -(2 as libc::c_int), k);
}
unsafe extern "C" fn treatstackoption(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut fname: *const libc::c_char,
) {
    if L == L1 {
        lua_rotate(L, -(2 as libc::c_int), 1 as libc::c_int);
    } else {
        lua_xmove(L1, L, 1 as libc::c_int);
    }
    lua_setfield(L, -(2 as libc::c_int), fname);
}
unsafe extern "C" fn db_getinfo(mut L: *mut lua_State) -> libc::c_int {
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut arg: libc::c_int = 0;
    let mut L1 = getthread(L, &mut arg);
    let mut options = luaL_optstring!(L, arg + 2, "flnStu");
    checkstack(L, L1, 3 as libc::c_int);
    if lua_isfunction!(L, arg + 1) != 0 {
        options = lua_pushfstring(
            L,
            b">%s\0" as *const u8 as *const libc::c_char,
            options,
        );
        lua_pushvalue(L, arg + 1 as libc::c_int);
        lua_xmove(L, L1, 1 as libc::c_int);
    } else if lua_getstack(
        L1,
        luaL_checkinteger(L, arg + 1 as libc::c_int) as libc::c_int,
        &mut ar,
    ) == 0
    {
        lua_pushnil(L);
        return 1 as libc::c_int;
    }
    if lua_getinfo(L1, options, &mut ar) == 0 {
        return luaL_argerror(
            L,
            arg + 2 as libc::c_int,
            b"invalid option\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_newtable!(L)(L, lua_newtable!(L), lua_newtable!(L));
    if !(strchr(options, 'S' as i32)).is_null() {
        settabss(L, b"source\0" as *const u8 as *const libc::c_char, ar.source);
        settabss(
            L,
            b"short_src\0" as *const u8 as *const libc::c_char,
            (ar.short_src).as_mut_ptr(),
        );
        settabsi(
            L,
            b"linedefined\0" as *const u8 as *const libc::c_char,
            ar.linedefined,
        );
        settabsi(
            L,
            b"lastlinedefined\0" as *const u8 as *const libc::c_char,
            ar.lastlinedefined,
        );
        settabss(L, b"what\0" as *const u8 as *const libc::c_char, ar.what);
    }
    if !(strchr(options, 'l' as i32)).is_null() {
        settabsi(
            L,
            b"currentline\0" as *const u8 as *const libc::c_char,
            ar.currentline,
        );
    }
    if !(strchr(options, 'u' as i32)).is_null() {
        settabsi(
            L,
            b"nups\0" as *const u8 as *const libc::c_char,
            ar.nups as libc::c_int,
        );
        settabsi(
            L,
            b"nparams\0" as *const u8 as *const libc::c_char,
            ar.nparams as libc::c_int,
        );
        settabsb(
            L,
            b"isvararg\0" as *const u8 as *const libc::c_char,
            ar.isvararg as libc::c_int,
        );
    }
    if !(strchr(options, 'n' as i32)).is_null() {
        settabss(L, b"name\0" as *const u8 as *const libc::c_char, ar.name);
        settabss(L, b"namewhat\0" as *const u8 as *const libc::c_char, ar.namewhat);
    }
    if !(strchr(options, 't' as i32)).is_null() {
        settabsb(
            L,
            b"istailcall\0" as *const u8 as *const libc::c_char,
            ar.istailcall as libc::c_int,
        );
    }
    if !(strchr(options, 'L' as i32)).is_null() {
        treatstackoption(L, L1, b"activelines\0" as *const u8 as *const libc::c_char);
    }
    if !(strchr(options, 'f' as i32)).is_null() {
        treatstackoption(L, L1, b"func\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_getlocal(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1 = getthread(L, &mut arg);
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut name = 0 as *const libc::c_char;
    let mut nvar = luaL_checkinteger(L, arg + 2 as libc::c_int) as libc::c_int;
    if lua_isfunction!(L, arg + 1) != 0 {
        lua_pushvalue(L, arg + 1 as libc::c_int);
        lua_pushstring(L, lua_getlocal(L, NULL as *const lua_Debug, nvar));
        return 1 as libc::c_int;
    } else {
        let mut level = luaL_checkinteger(L, arg + 1 as libc::c_int) as libc::c_int;
        if lua_getstack(L1, level, &mut ar) == 0 {
            return luaL_argerror(
                L,
                arg + 1 as libc::c_int,
                b"level out of range\0" as *const u8 as *const libc::c_char,
            );
        }
        checkstack(L, L1, 1 as libc::c_int);
        name = lua_getlocal(L1, &mut ar, nvar);
        if !name.is_null() {
            lua_xmove(L1, L, 1 as libc::c_int);
            lua_pushstring(L, name);
            lua_rotate(L, -(2 as libc::c_int), 1 as libc::c_int);
            return 2 as libc::c_int;
        } else {
            lua_pushnil(L);
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn db_setlocal(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut name = 0 as *const libc::c_char;
    let mut L1 = getthread(L, &mut arg);
    let mut ar = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut level = luaL_checkinteger(L, arg + 1 as libc::c_int) as libc::c_int;
    let mut nvar = luaL_checkinteger(L, arg + 2 as libc::c_int) as libc::c_int;
    if lua_getstack(L1, level, &mut ar) == 0 {
        return luaL_argerror(
            L,
            arg + 1 as libc::c_int,
            b"level out of range\0" as *const u8 as *const libc::c_char,
        );
    }
    luaL_checkany(L, arg + 3 as libc::c_int);
    lua_settop(L, arg + 3 as libc::c_int);
    checkstack(L, L1, 1 as libc::c_int);
    lua_xmove(L, L1, 1 as libc::c_int);
    name = lua_setlocal(L1, &mut ar, nvar);
    if name.is_null() {
        lua_pop!(L1, 1)(L1, lua_pop!(L1, 1));
    }
    lua_pushstring(L, name);
    return 1 as libc::c_int;
}
unsafe extern "C" fn auxupvalue(
    mut L: *mut lua_State,
    mut get: libc::c_int,
) -> libc::c_int {
    let mut name = 0 as *const libc::c_char;
    let mut n = luaL_checkinteger(L, 2 as libc::c_int) as libc::c_int;
    luaL_checktype(L, 1 as libc::c_int, LUA_TFUNCTION);
    name = if get != 0 {
        lua_getupvalue(L, 1 as libc::c_int, n)
    } else {
        lua_setupvalue(L, 1 as libc::c_int, n)
    };
    if name.is_null() {
        return 0 as libc::c_int;
    }
    lua_pushstring(L, name);
    lua_rotate(L, -(get + 1 as libc::c_int), 1 as libc::c_int);
    return get + 1 as libc::c_int;
}
unsafe extern "C" fn db_getupvalue(mut L: *mut lua_State) -> libc::c_int {
    return auxupvalue(L, 1 as libc::c_int);
}
unsafe extern "C" fn db_setupvalue(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 3 as libc::c_int);
    return auxupvalue(L, 0 as libc::c_int);
}
unsafe extern "C" fn checkupval(
    mut L: *mut lua_State,
    mut argf: libc::c_int,
    mut argnup: libc::c_int,
) -> libc::c_int {
    let mut nup = luaL_checkinteger(L, argnup) as libc::c_int;
    luaL_checktype(L, argf, LUA_TFUNCTION);
    return nup;
}
unsafe extern "C" fn db_upvalueid(mut L: *mut lua_State) -> libc::c_int {
    let mut n = checkupval(L, 1 as libc::c_int, 2 as libc::c_int);
    lua_pushlightuserdata(L, lua_upvalueid(L, 1 as libc::c_int, n));
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_upvaluejoin(mut L: *mut lua_State) -> libc::c_int {
    let mut n1 = checkupval(L, 1 as libc::c_int, 2 as libc::c_int);
    let mut n2 = checkupval(L, 3 as libc::c_int, 4 as libc::c_int);
    lua_upvaluejoin(L, 1 as libc::c_int, n1, 3 as libc::c_int, n2);
    return 0 as libc::c_int;
}
unsafe extern "C" fn hookf(mut L: *mut lua_State, mut ar: *mut lua_Debug) {
    static mut hooknames: [*const libc::c_char; 5] = [
        b"call\0" as *const u8 as *const libc::c_char,
        b"return\0" as *const u8 as *const libc::c_char,
        b"line\0" as *const u8 as *const libc::c_char,
        b"count\0" as *const u8 as *const libc::c_char,
        b"tail call\0" as *const u8 as *const libc::c_char,
    ];
    lua_rawgetp(
        L,
        LUA_REGISTRYINDEX,
        &HOOKKEY as *const libc::c_int as *const libc::c_void,
    );
    lua_pushthread(L);
    if lua_rawget(L, -(2 as libc::c_int)) == LUA_TFUNCTION {
        lua_pushstring(L, hooknames[(*ar).event as usize]);
        if (*ar).currentline >= 0 as libc::c_int {
            lua_pushinteger(L, (*ar).currentline as lua_Integer);
        } else {
            lua_pushnil(L);
        }
        lua_call!(
            L, 2, 0
        )(
            L,
            lua_call!(L, 2, 0),
            lua_call!(L, 2, 0),
            lua_call!(L, 2, 0),
            ::core::mem::transmute::<
                libc::intptr_t,
                lua_KFunction,
            >(NULL as libc::intptr_t),
        );
    }
}
unsafe extern "C" fn makemask(
    mut smask: *const libc::c_char,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut mask = 0 as libc::c_int;
    if !(strchr(smask, 'c' as i32)).is_null() {
        mask |= LUA_MASKCALL;
    }
    if !(strchr(smask, 'r' as i32)).is_null() {
        mask |= LUA_MASKRET;
    }
    if !(strchr(smask, 'l' as i32)).is_null() {
        mask |= LUA_MASKLINE;
    }
    if count > 0 as libc::c_int {
        mask |= LUA_MASKCOUNT;
    }
    return mask;
}
unsafe extern "C" fn unmakemask(
    mut mask: libc::c_int,
    mut smask: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i = 0 as libc::c_int;
    if mask & LUA_MASKCALL != 0 {
        let fresh321 = i;
        i = i + 1;
        *smask.offset(fresh321 as isize) = 'c' as i32 as libc::c_char;
    }
    if mask & LUA_MASKRET != 0 {
        let fresh322 = i;
        i = i + 1;
        *smask.offset(fresh322 as isize) = 'r' as i32 as libc::c_char;
    }
    if mask & LUA_MASKLINE != 0 {
        let fresh323 = i;
        i = i + 1;
        *smask.offset(fresh323 as isize) = 'l' as i32 as libc::c_char;
    }
    *smask.offset(i as isize) = '\0' as i32 as libc::c_char;
    return smask;
}
unsafe extern "C" fn db_sethook(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut func: lua_Hook = None;
    let mut L1 = getthread(L, &mut arg);
    if lua_isnoneornil!(L, arg + 1) != 0 {
        lua_settop(L, arg + 1 as libc::c_int);
        func = ::core::mem::transmute::<
            libc::intptr_t,
            lua_Hook,
        >(NULL as libc::intptr_t);
        mask = 0 as libc::c_int;
        count = 0 as libc::c_int;
    } else {
        let mut smask = luaL_checkstring!(L, arg + 2);
        luaL_checktype(L, arg + 1 as libc::c_int, LUA_TFUNCTION);
        count = luaL_optinteger(
            L,
            arg + 3 as libc::c_int,
            0 as libc::c_int as lua_Integer,
        ) as libc::c_int;
        func = Some(hookf as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ());
        mask = makemask(smask, count);
    }
    if lua_rawgetp(
        L,
        LUA_REGISTRYINDEX,
        &HOOKKEY as *const libc::c_int as *const libc::c_void,
    ) == LUA_TNIL
    {
        lua_createtable(L, 0 as libc::c_int, 2 as libc::c_int);
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_rawsetp(
            L,
            LUA_REGISTRYINDEX,
            &HOOKKEY as *const libc::c_int as *const libc::c_void,
        );
        lua_pushstring(L, b"k\0" as *const u8 as *const libc::c_char);
        lua_setfield(
            L,
            -(2 as libc::c_int),
            b"__mode\0" as *const u8 as *const libc::c_char,
        );
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_setmetatable(L, -(2 as libc::c_int));
    }
    checkstack(L, L1, 1 as libc::c_int);
    lua_pushthread(L1);
    lua_xmove(L1, L, 1 as libc::c_int);
    lua_pushvalue(L, arg + 1 as libc::c_int);
    lua_rawset(L, -(3 as libc::c_int));
    lua_sethook(L1, func, mask, count);
    return 0 as libc::c_int;
}
unsafe extern "C" fn db_gethook(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1 = getthread(L, &mut arg);
    let mut buff: [libc::c_char; 5] = [0; 5];
    let mut mask = lua_gethookmask(L1);
    let mut hook = lua_gethook(L1);
    if hook.is_none() {
        lua_pushnil(L);
    } else if hook
        != Some(hookf as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ())
    {
        lua_pushliteral!(
            L, "external hook"
        )(L, b"external hook\0" as *const u8 as *const libc::c_char);
    } else {
        lua_rawgetp(
            L,
            LUA_REGISTRYINDEX,
            &HOOKKEY as *const libc::c_int as *const libc::c_void,
        );
        checkstack(L, L1, 1 as libc::c_int);
        lua_pushthread(L1);
        lua_xmove(L1, L, 1 as libc::c_int);
        lua_rawget(L, -(2 as libc::c_int));
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    lua_pushstring(L, unmakemask(mask, buff.as_mut_ptr()));
    lua_pushinteger(L, lua_gethookcount(L1) as lua_Integer);
    return 3 as libc::c_int;
}
unsafe extern "C" fn db_debug(mut L: *mut lua_State) -> libc::c_int {
    loop {
        let mut buffer: [libc::c_char; 250] = [0; 250];
        if (fgets(
            buffer.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 250]>() as libc::c_ulong
                as libc::c_int,
            stdin,
        ))
            .is_null()
            || strcmp(
                buffer.as_mut_ptr(),
                b"cont\n\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if luaL_loadbuffer!(L, buffer, strlen(buffer), "=(debug command)") != 0
            || lua_pcall!(L, 0, 0, 0) != 0
        {
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                lua_tolstring(L, -(1 as libc::c_int), 0 as *mut size_t),
            );
            fflush(stderr);
        }
        lua_settop(L, 0 as libc::c_int);
    };
}
unsafe extern "C" fn db_traceback(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1 = getthread(L, &mut arg);
    let mut msg = lua_tostring!(L, arg + 1);
    if msg.is_null() && lua_isnoneornil!(L, arg + 1) == 0 {
        lua_pushvalue(L, arg + 1 as libc::c_int);
    } else {
        let mut level = luaL_optinteger(
            L,
            arg + 2 as libc::c_int,
            (if L == L1 { 1 as libc::c_int } else { 0 as libc::c_int }) as lua_Integer,
        ) as libc::c_int;
        luaL_traceback(L, L1, msg, level);
    }
    return 1 as libc::c_int;
}
static mut dblib: [luaL_Reg; 17] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"debug\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_debug as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getuservalue\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getuservalue
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gethook\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_gethook as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getinfo\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getinfo as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getlocal\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getlocal as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getregistry\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getregistry as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getupvalue\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getupvalue as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"upvaluejoin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_upvaluejoin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"upvalueid\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_upvalueid as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setuservalue\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_setuservalue
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sethook\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_sethook as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setlocal\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_setlocal as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_setmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setupvalue\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_setupvalue as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"traceback\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_traceback as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaopen_debug(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 17]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    return 1 as libc::c_int;
}
pub const L_MODEEXT: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"b\0")
};
unsafe extern "C" fn l_checkmode(mut mode: *const libc::c_char) -> libc::c_int {
    return (*mode as libc::c_int != '\0' as i32
        && {
            let fresh324 = mode;
            mode = mode.offset(1);
            !(strchr(
                b"rwa\0" as *const u8 as *const libc::c_char,
                *fresh324 as libc::c_int,
            ))
                .is_null()
        }
        && (*mode as libc::c_int != '+' as i32
            || {
                mode = mode.offset(1);
                1 as libc::c_int != 0
            }) && strspn(mode, L_MODEEXT.as_ptr()) == strlen(mode)) as libc::c_int;
}
pub const IOPREF_LEN: libc::c_ulong = (::core::mem::size_of::<[libc::c_char; 5]>()
    as libc::c_ulong)
    .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
pub const IO_INPUT: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"_IO_input\0")
};
pub const IO_OUTPUT: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"_IO_output\0")
};
unsafe extern "C" fn io_type(mut L: *mut lua_State) -> libc::c_int {
    let mut p = 0 as *mut LStream;
    luaL_checkany(L, 1 as libc::c_int);
    p = luaL_testudata(L, 1 as libc::c_int, LUA_FILEHANDLE.as_ptr()) as *mut LStream;
    if p.is_null() {
        lua_pushnil(L);
    } else if isclosed!(p).is_none() {
        lua_pushliteral!(
            L, "closed file"
        )(L, b"closed file\0" as *const u8 as *const libc::c_char);
    } else {
        lua_pushliteral!(L, "file")(L, b"file\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn f_tostring(mut L: *mut lua_State) -> libc::c_int {
    let mut p = tolstream!(L);
    if isclosed!(p).is_none() {
        lua_pushliteral!(
            L, "file (closed)"
        )(L, b"file (closed)\0" as *const u8 as *const libc::c_char);
    } else {
        lua_pushfstring(L, b"file (%p)\0" as *const u8 as *const libc::c_char, (*p).f);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn tofile(mut L: *mut lua_State) -> *mut FILE {
    let mut p = tolstream!(L);
    if isclosed!(p).is_none() {
        luaL_error(
            L,
            b"attempt to use a closed file\0" as *const u8 as *const libc::c_char,
        );
    }
    return (*p).f;
}
unsafe extern "C" fn newprefile(mut L: *mut lua_State) -> *mut LStream {
    let mut p = lua_newuserdata(L, ::core::mem::size_of::<LStream>() as libc::c_ulong)
        as *mut LStream;
    (*p)
        .closef = ::core::mem::transmute::<
        libc::intptr_t,
        lua_CFunction,
    >(NULL as libc::intptr_t);
    luaL_setmetatable(L, LUA_FILEHANDLE.as_ptr());
    return p;
}
unsafe extern "C" fn aux_close(mut L: *mut lua_State) -> libc::c_int {
    let mut p = tolstream!(L);
    let mut cf: lua_CFunction = (*p).closef;
    (*p)
        .closef = ::core::mem::transmute::<
        libc::intptr_t,
        lua_CFunction,
    >(NULL as libc::intptr_t);
    return (Some(cf.expect("non-null function pointer")))
        .expect("non-null function pointer")(L);
}
unsafe extern "C" fn f_close(mut L: *mut lua_State) -> libc::c_int {
    tofile(L);
    return aux_close(L);
}
unsafe extern "C" fn io_close(mut L: *mut lua_State) -> libc::c_int {
    if lua_isnone!(L, 1) != 0 {
        lua_getfield(L, LUA_REGISTRYINDEX, IO_OUTPUT.as_ptr());
    }
    return f_close(L);
}
unsafe extern "C" fn f_gc(mut L: *mut lua_State) -> libc::c_int {
    let mut p = tolstream!(L);
    if isclosed!(p).is_some() && !((*p).f).is_null() {
        aux_close(L);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn io_fclose(mut L: *mut lua_State) -> libc::c_int {
    let mut p = tolstream!(L);
    let mut res = fclose((*p).f);
    return luaL_fileresult(
        L,
        (res == 0 as libc::c_int) as libc::c_int,
        NULL as *const libc::c_char,
    );
}
unsafe extern "C" fn newfile(mut L: *mut lua_State) -> *mut LStream {
    let mut p = newprefile(L);
    (*p).f = NULL as *mut FILE;
    (*p).closef = Some(io_fclose as unsafe extern "C" fn(*mut lua_State) -> libc::c_int);
    return p;
}
unsafe extern "C" fn opencheck(
    mut L: *mut lua_State,
    mut fname: *const libc::c_char,
    mut mode: *const libc::c_char,
) {
    let mut p = newfile(L);
    (*p).f = fopen(fname, mode);
    if ((*p).f).is_null() {
        luaL_error(
            L,
            b"cannot open file '%s' (%s)\0" as *const u8 as *const libc::c_char,
            fname,
            strerror(errno),
        );
    }
}
unsafe extern "C" fn io_open(mut L: *mut lua_State) -> libc::c_int {
    let mut filename = luaL_checkstring!(L, 1);
    let mut mode = luaL_optstring!(L, 2, "r");
    let mut p = newfile(L);
    let mut md = mode;
    (*p).f = fopen(filename, mode);
    return if ((*p).f).is_null() {
        luaL_fileresult(L, 0 as libc::c_int, filename)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn io_pclose(mut L: *mut lua_State) -> libc::c_int {
    let mut p = tolstream!(L);
    return luaL_execresult(L, l_pclose!(L, p -> f));
}
unsafe extern "C" fn io_popen(mut L: *mut lua_State) -> libc::c_int {
    let mut filename = luaL_checkstring!(L, 1);
    let mut mode = luaL_optstring!(L, 2, "r");
    let mut p = newprefile(L);
    (*p).f = l_popen!(L, filename, mode);
    (*p).closef = Some(io_pclose as unsafe extern "C" fn(*mut lua_State) -> libc::c_int);
    return if ((*p).f).is_null() {
        luaL_fileresult(L, 0 as libc::c_int, filename)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn io_tmpfile(mut L: *mut lua_State) -> libc::c_int {
    let mut p = newfile(L);
    (*p).f = tmpfile();
    return if ((*p).f).is_null() {
        luaL_fileresult(L, 0 as libc::c_int, NULL as *const libc::c_char)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn getiofile(
    mut L: *mut lua_State,
    mut findex: *const libc::c_char,
) -> *mut FILE {
    let mut p = 0 as *mut LStream;
    lua_getfield(L, LUA_REGISTRYINDEX, findex);
    p = lua_touserdata(L, -(1 as libc::c_int)) as *mut LStream;
    if isclosed!(p).is_none() {
        luaL_error(
            L,
            b"standard %s file is closed\0" as *const u8 as *const libc::c_char,
            findex.offset(IOPREF_LEN as isize),
        );
    }
    return (*p).f;
}
unsafe extern "C" fn g_iofile(
    mut L: *mut lua_State,
    mut f: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    if lua_isnoneornil!(L, 1) == 0 {
        let mut filename = lua_tostring!(L, 1);
        if !filename.is_null() {
            opencheck(L, filename, mode);
        } else {
            tofile(L);
            lua_pushvalue(L, 1 as libc::c_int);
        }
        lua_setfield(L, LUA_REGISTRYINDEX, f);
    }
    lua_getfield(L, LUA_REGISTRYINDEX, f);
    return 1 as libc::c_int;
}
unsafe extern "C" fn io_input(mut L: *mut lua_State) -> libc::c_int {
    return g_iofile(L, IO_INPUT.as_ptr(), b"r\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn io_output(mut L: *mut lua_State) -> libc::c_int {
    return g_iofile(L, IO_OUTPUT.as_ptr(), b"w\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn aux_lines(mut L: *mut lua_State, mut toclose: libc::c_int) {
    let mut n = lua_gettop(L) - 1 as libc::c_int;
    lua_pushinteger(L, n as lua_Integer);
    lua_pushboolean(L, toclose);
    lua_rotate(L, 2 as libc::c_int, 2 as libc::c_int);
    lua_pushcclosure(
        L,
        Some(io_readline as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        3 as libc::c_int + n,
    );
}
unsafe extern "C" fn f_lines(mut L: *mut lua_State) -> libc::c_int {
    tofile(L);
    aux_lines(L, 0 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn io_lines(mut L: *mut lua_State) -> libc::c_int {
    let mut toclose: libc::c_int = 0;
    if lua_isnone!(L, 1) != 0 {
        lua_pushnil(L);
    }
    if lua_isnil!(L, 1) != 0 {
        lua_getfield(L, LUA_REGISTRYINDEX, IO_INPUT.as_ptr());
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        tofile(L);
        toclose = 0 as libc::c_int;
    } else {
        let mut filename = luaL_checkstring!(L, 1);
        opencheck(L, filename, b"r\0" as *const u8 as *const libc::c_char);
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        toclose = 1 as libc::c_int;
    }
    aux_lines(L, toclose);
    return 1 as libc::c_int;
}
unsafe extern "C" fn nextc(mut rn: *mut RN) -> libc::c_int {
    if (*rn).n >= L_MAXLENNUM {
        (*rn).buff[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        return 0 as libc::c_int;
    } else {
        let fresh325 = (*rn).n;
        (*rn).n = (*rn).n + 1;
        (*rn).buff[fresh325 as usize] = (*rn).c as libc::c_char;
        (*rn).c = l_getc!(rn -> f);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn test2(
    mut rn: *mut RN,
    mut set: *const libc::c_char,
) -> libc::c_int {
    if (*rn).c == *set.offset(0 as libc::c_int as isize) as libc::c_int
        || (*rn).c == *set.offset(1 as libc::c_int as isize) as libc::c_int
    {
        return nextc(rn)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn readdigits(mut rn: *mut RN, mut hex: libc::c_int) -> libc::c_int {
    let mut count = 0 as libc::c_int;
    while (if hex != 0 { isxdigit!(rn -> c) } else { isdigit!(rn -> c) }) != 0
        && nextc(rn) != 0
    {
        count += 1;
    }
    return count;
}
unsafe extern "C" fn read_number(
    mut L: *mut lua_State,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut rn = RN {
        f: 0 as *mut FILE,
        c: 0,
        n: 0,
        buff: [0; 201],
    };
    let mut count = 0 as libc::c_int;
    let mut hex = 0 as libc::c_int;
    let mut decp: [libc::c_char; 2] = [0; 2];
    rn.f = f;
    rn.n = 0 as libc::c_int;
    decp[0 as libc::c_int as usize] = lua_getlocaledecpoint!();
    decp[1 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    loop {
        rn.c = l_getc!(rn.f);
        if !(isspace!(rn.c) != 0) {
            break;
        }
    }
    test2(&mut rn, b"-+\0" as *const u8 as *const libc::c_char);
    if test2(&mut rn, b"00\0" as *const u8 as *const libc::c_char) != 0 {
        if test2(&mut rn, b"xX\0" as *const u8 as *const libc::c_char) != 0 {
            hex = 1 as libc::c_int;
        } else {
            count = 1 as libc::c_int;
        }
    }
    count += readdigits(&mut rn, hex);
    if test2(&mut rn, decp.as_mut_ptr()) != 0 {
        count += readdigits(&mut rn, hex);
    }
    if count > 0 as libc::c_int
        && test2(
            &mut rn,
            (if hex != 0 {
                b"pP\0" as *const u8 as *const libc::c_char
            } else {
                b"eE\0" as *const u8 as *const libc::c_char
            }),
        ) != 0
    {
        test2(&mut rn, b"-+\0" as *const u8 as *const libc::c_char);
        readdigits(&mut rn, 0 as libc::c_int);
    }
    ungetc(rn.c, rn.f);
    rn.buff[rn.n as usize] = '\0' as i32 as libc::c_char;
    if lua_stringtonumber(L, (rn.buff).as_mut_ptr()) != 0 {
        return 1 as libc::c_int
    } else {
        lua_pushnil(L);
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn test_eof(mut L: *mut lua_State, mut f: *mut FILE) -> libc::c_int {
    let mut c = getc(f);
    ungetc(c, f);
    lua_pushliteral!(L, "")(L, b"\0" as *const u8 as *const libc::c_char);
    return (c != EOF) as libc::c_int;
}
unsafe extern "C" fn read_line(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut chop: libc::c_int,
) -> libc::c_int {
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut c = '\0' as i32;
    luaL_buffinit(L, &mut b);
    while c != EOF && c != '\n' as i32 {
        let mut buff = luaL_prepbuffer!(& b);
        let mut i = 0 as libc::c_int;
        while i < LUAL_BUFFERSIZE as libc::c_int
            && {
                c = l_getc!(f);
                c != EOF
            } && c != '\n' as i32
        {
            let fresh326 = i;
            i = i + 1;
            *buff.offset(fresh326 as isize) = c as libc::c_char;
        }
        let ref mut fresh327 = luaL_addsize!(& b, i);
        *fresh327 = (*fresh327 as libc::c_ulong).wrapping_add(luaL_addsize!(& b, i))
            as size_t as size_t;
    }
    chop == 0 && c == '\n' as i32;
    luaL_pushresult(&mut b);
    return (c == '\n' as i32
        || lua_rawlen(L, -(1 as libc::c_int)) > 0 as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
unsafe extern "C" fn read_all(mut L: *mut lua_State, mut f: *mut FILE) {
    let mut nr: size_t = 0;
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    loop {
        let mut p = luaL_prepbuffer!(& b);
        nr = fread(
            p as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            LUAL_BUFFERSIZE,
            f,
        );
        let ref mut fresh328 = luaL_addsize!(& b, nr);
        *fresh328 = (*fresh328 as libc::c_ulong).wrapping_add(luaL_addsize!(& b, nr))
            as size_t as size_t;
        if !(nr == LUAL_BUFFERSIZE) {
            break;
        }
    }
    luaL_pushresult(&mut b);
}
unsafe extern "C" fn read_chars(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut n: size_t,
) -> libc::c_int {
    let mut nr: size_t = 0;
    let mut p = 0 as *mut libc::c_char;
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    p = luaL_prepbuffsize(&mut b, n);
    nr = fread(
        p as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        n,
        f,
    );
    let ref mut fresh329 = luaL_addsize!(& b, nr);
    *fresh329 = (*fresh329 as libc::c_ulong).wrapping_add(luaL_addsize!(& b, nr))
        as size_t as size_t;
    luaL_pushresult(&mut b);
    return (nr > 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn g_read(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut first: libc::c_int,
) -> libc::c_int {
    let mut nargs = lua_gettop(L) - 1 as libc::c_int;
    let mut success: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    clearerr(f);
    if nargs == 0 as libc::c_int {
        success = read_line(L, f, 1 as libc::c_int);
        n = first + 1 as libc::c_int;
    } else {
        luaL_checkstack(
            L,
            nargs + LUA_MINSTACK,
            b"too many arguments\0" as *const u8 as *const libc::c_char,
        );
        success = 1 as libc::c_int;
        n = first;
        loop {
            let fresh330 = nargs;
            nargs = nargs - 1;
            if !(fresh330 != 0 && success != 0) {
                break;
            }
            if lua_type(L, n) == LUA_TNUMBER {
                let mut l = luaL_checkinteger(L, n) as size_t;
                success = if l == 0 as libc::c_int as libc::c_ulong {
                    test_eof(L, f)
                } else {
                    read_chars(L, f, l)
                };
            } else {
                let mut p = luaL_checkstring!(L, n);
                if *p as libc::c_int == '*' as i32 {
                    p = p.offset(1);
                }
                match *p as libc::c_int {
                    110 => {
                        success = read_number(L, f);
                    }
                    108 => {
                        success = read_line(L, f, 1 as libc::c_int);
                    }
                    76 => {
                        success = read_line(L, f, 0 as libc::c_int);
                    }
                    97 => {
                        read_all(L, f);
                        success = 1 as libc::c_int;
                    }
                    _ => {
                        return luaL_argerror(
                            L,
                            n,
                            b"invalid format\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            n += 1;
        }
    }
    if ferror(f) != 0 {
        return luaL_fileresult(L, 0 as libc::c_int, NULL as *const libc::c_char);
    }
    if success == 0 {
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        lua_pushnil(L);
    }
    return n - first;
}
unsafe extern "C" fn io_read(mut L: *mut lua_State) -> libc::c_int {
    return g_read(L, getiofile(L, IO_INPUT.as_ptr()), 1 as libc::c_int);
}
unsafe extern "C" fn f_read(mut L: *mut lua_State) -> libc::c_int {
    return g_read(L, tofile(L), 2 as libc::c_int);
}
unsafe extern "C" fn io_readline(mut L: *mut lua_State) -> libc::c_int {
    let mut p = lua_touserdata(L, lua_upvalueindex!(1)) as *mut LStream;
    let mut i: libc::c_int = 0;
    let mut n = lua_tointegerx(
        L,
        -(1000000 as libc::c_int) - 1000 as libc::c_int - 2 as libc::c_int,
        NULL as *mut libc::c_int,
    ) as libc::c_int;
    if isclosed!(p).is_none() {
        return luaL_error(
            L,
            b"file is already closed\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_settop(L, 1 as libc::c_int);
    luaL_checkstack(L, n, b"too many arguments\0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i <= n {
        lua_pushvalue(L, lua_upvalueindex!(3 + i));
        i += 1;
    }
    n = g_read(L, (*p).f, 2 as libc::c_int);
    if lua_toboolean(L, -n) != 0 {
        return n
    } else {
        if n > 1 as libc::c_int {
            return luaL_error(
                L,
                b"%s\0" as *const u8 as *const libc::c_char,
                lua_tostring!(L, - n + 1),
            );
        }
        if lua_toboolean(L, lua_upvalueindex!(3)) != 0 {
            lua_settop(L, 0 as libc::c_int);
            lua_pushvalue(L, lua_upvalueindex!(1));
            aux_close(L);
        }
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn g_write(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut arg: libc::c_int,
) -> libc::c_int {
    let mut nargs = lua_gettop(L) - arg;
    let mut status = 1 as libc::c_int;
    loop {
        let fresh331 = nargs;
        nargs = nargs - 1;
        if !(fresh331 != 0) {
            break;
        }
        if lua_type(L, arg) == LUA_TNUMBER {
            let mut len = if lua_isinteger(L, arg) != 0 {
                fprintf(f, LUA_INTEGER_FMT.as_ptr(), lua_tointeger!(L, arg))
            } else {
                fprintf(f, LUA_NUMBER_FMT.as_ptr(), lua_tonumber!(L, arg))
            };
            status = (status != 0 && len > 0 as libc::c_int) as libc::c_int;
        } else {
            let mut l: size_t = 0;
            let mut s = luaL_checklstring(L, arg, &mut l);
            status = (status != 0
                && fwrite(
                    s as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    l,
                    f,
                ) == l) as libc::c_int;
        }
        arg += 1;
    }
    if status != 0 {
        return 1 as libc::c_int
    } else {
        return luaL_fileresult(L, status, NULL as *const libc::c_char)
    };
}
unsafe extern "C" fn io_write(mut L: *mut lua_State) -> libc::c_int {
    return g_write(L, getiofile(L, IO_OUTPUT.as_ptr()), 1 as libc::c_int);
}
unsafe extern "C" fn f_write(mut L: *mut lua_State) -> libc::c_int {
    let mut f = tofile(L);
    lua_pushvalue(L, 1 as libc::c_int);
    return g_write(L, f, 2 as libc::c_int);
}
unsafe extern "C" fn f_seek(mut L: *mut lua_State) -> libc::c_int {
    static mut mode: [libc::c_int; 3] = [SEEK_SET, SEEK_CUR, SEEK_END];
    static mut modenames: [*const libc::c_char; 4] = [
        b"set\0" as *const u8 as *const libc::c_char,
        b"cur\0" as *const u8 as *const libc::c_char,
        b"end\0" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    let mut f = tofile(L);
    let mut op = luaL_checkoption(
        L,
        2 as libc::c_int,
        b"cur\0" as *const u8 as *const libc::c_char,
        modenames.as_ptr(),
    );
    let mut p3 = luaL_optinteger(L, 3 as libc::c_int, 0 as libc::c_int as lua_Integer);
    let mut offset = p3 as libc::c_long;
    op = l_fseek!(f, offset, mode[op]);
    if op != 0 {
        return luaL_fileresult(L, 0 as libc::c_int, NULL as *const libc::c_char)
    } else {
        lua_pushinteger(L, l_ftell!(f) as lua_Integer);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn f_setvbuf(mut L: *mut lua_State) -> libc::c_int {
    static mut mode: [libc::c_int; 3] = [_IONBF, _IOFBF, _IOLBF];
    static mut modenames: [*const libc::c_char; 4] = [
        b"no\0" as *const u8 as *const libc::c_char,
        b"full\0" as *const u8 as *const libc::c_char,
        b"line\0" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    let mut f = tofile(L);
    let mut op = luaL_checkoption(
        L,
        2 as libc::c_int,
        NULL as *const libc::c_char,
        modenames.as_ptr(),
    );
    let mut sz = luaL_optinteger(L, 3 as libc::c_int, LUAL_BUFFERSIZE as lua_Integer);
    let mut res = setvbuf(f, NULL as *mut libc::c_char, mode[op as usize], sz as size_t);
    return luaL_fileresult(
        L,
        (res == 0 as libc::c_int) as libc::c_int,
        NULL as *const libc::c_char,
    );
}
unsafe extern "C" fn io_flush(mut L: *mut lua_State) -> libc::c_int {
    return luaL_fileresult(
        L,
        (fflush(getiofile(L, IO_OUTPUT.as_ptr())) == 0 as libc::c_int) as libc::c_int,
        NULL as *const libc::c_char,
    );
}
unsafe extern "C" fn f_flush(mut L: *mut lua_State) -> libc::c_int {
    return luaL_fileresult(
        L,
        (fflush(tofile(L)) == 0 as libc::c_int) as libc::c_int,
        NULL as *const libc::c_char,
    );
}
static mut iolib: [luaL_Reg; 12] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"close\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_close as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"flush\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_flush as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"input\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_input as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"lines\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_lines as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"open\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_open as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"output\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_output as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"popen\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_popen as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"read\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_read as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tmpfile\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_tmpfile as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_type as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"write\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_write as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
static mut flib: [luaL_Reg; 10] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"close\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_close as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"flush\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_flush as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"lines\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_lines as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"read\0" as *const u8 as *const libc::c_char,
                func: Some(f_read as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"seek\0" as *const u8 as *const libc::c_char,
                func: Some(f_seek as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setvbuf\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_setvbuf as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"write\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_write as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__gc\0" as *const u8 as *const libc::c_char,
                func: Some(f_gc as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__tostring\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_tostring as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
unsafe extern "C" fn createmeta(mut L: *mut lua_State) {
    luaL_newmetatable(L, LUA_FILEHANDLE.as_ptr());
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    luaL_setfuncs(L, flib.as_ptr(), 0 as libc::c_int);
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
}
unsafe extern "C" fn io_noclose(mut L: *mut lua_State) -> libc::c_int {
    let mut p = tolstream!(L);
    (*p)
        .closef = Some(
        io_noclose as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
    );
    lua_pushnil(L);
    lua_pushliteral!(
        L, "cannot close standard file"
    )(L, b"cannot close standard file\0" as *const u8 as *const libc::c_char);
    return 2 as libc::c_int;
}
unsafe extern "C" fn createstdfile(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut k: *const libc::c_char,
    mut fname: *const libc::c_char,
) {
    let mut p = newprefile(L);
    (*p).f = f;
    (*p)
        .closef = Some(
        io_noclose as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
    );
    if !k.is_null() {
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_setfield(L, LUA_REGISTRYINDEX, k);
    }
    lua_setfield(L, -(2 as libc::c_int), fname);
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_io(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 12]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    createmeta(L);
    createstdfile(
        L,
        stdin,
        IO_INPUT.as_ptr(),
        b"stdin\0" as *const u8 as *const libc::c_char,
    );
    createstdfile(
        L,
        stdout,
        IO_OUTPUT.as_ptr(),
        b"stdout\0" as *const u8 as *const libc::c_char,
    );
    createstdfile(
        L,
        stderr,
        NULL as *const libc::c_char,
        b"stderr\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
pub const PI: libc::c_double = 3.141592653589793238462643383279502884f64;
pub const L_RANDMAX: libc::c_int = RAND_MAX;
unsafe extern "C" fn math_abs(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        let mut n = lua_tointeger!(L, 1);
        if n < 0 as libc::c_int as libc::c_longlong {
            n = (0 as libc::c_uint as libc::c_ulonglong).wrapping_sub(n as lua_Unsigned)
                as lua_Integer;
        }
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, l_mathop!(fabs)(luaL_checknumber(L, 1 as libc::c_int)));
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_sin(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, l_mathop!(sin)(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_cos(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, l_mathop!(cos)(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_tan(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, l_mathop!(tan)(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_asin(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, l_mathop!(asin)(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_acos(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, l_mathop!(acos)(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_atan(mut L: *mut lua_State) -> libc::c_int {
    let mut y = luaL_checknumber(L, 1 as libc::c_int);
    let mut x = luaL_optnumber(L, 2 as libc::c_int, 1 as libc::c_int as lua_Number);
    lua_pushnumber(L, l_mathop!(atan2)(y, x));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_toint(mut L: *mut lua_State) -> libc::c_int {
    let mut valid: libc::c_int = 0;
    let mut n = lua_tointegerx(L, 1 as libc::c_int, &mut valid);
    if valid != 0 {
        lua_pushinteger(L, n);
    } else {
        luaL_checkany(L, 1 as libc::c_int);
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn pushnumint(mut L: *mut lua_State, mut d: lua_Number) {
    let mut n: lua_Integer = 0;
    if lua_numbertointeger!(d, & n) != 0 {
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, d);
    };
}
unsafe extern "C" fn math_floor(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
    } else {
        let mut d = l_mathop!(floor)(luaL_checknumber(L, 1 as libc::c_int));
        pushnumint(L, d);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_ceil(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
    } else {
        let mut d = l_mathop!(ceil)(luaL_checknumber(L, 1 as libc::c_int));
        pushnumint(L, d);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_fmod(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 && lua_isinteger(L, 2 as libc::c_int) != 0
    {
        let mut d = lua_tointeger!(L, 2);
        if (d as lua_Unsigned).wrapping_add(1 as libc::c_uint as libc::c_ulonglong)
            <= 1 as libc::c_uint as libc::c_ulonglong
        {
            lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
        } else {
            lua_pushinteger(L, lua_tointeger!(L, 1) % d);
        }
    } else {
        lua_pushnumber(
            L,
            l_mathop!(
                fmod
            )(
                luaL_checknumber(L, 1 as libc::c_int),
                luaL_checknumber(L, 2 as libc::c_int),
            ),
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_modf(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
        lua_pushnumber(L, 0 as libc::c_int as lua_Number);
    } else {
        let mut n = luaL_checknumber(L, 1 as libc::c_int);
        let mut ip = if n < 0 as libc::c_int as libc::c_double {
            l_mathop!(ceil)(n)
        } else {
            l_mathop!(floor)(n)
        };
        pushnumint(L, ip);
        lua_pushnumber(L, if n == ip { l_mathop!(0.0) } else { n - ip });
    }
    return 2 as libc::c_int;
}
unsafe extern "C" fn math_sqrt(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, l_mathop!(sqrt)(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_ult(mut L: *mut lua_State) -> libc::c_int {
    let mut a = luaL_checkinteger(L, 1 as libc::c_int);
    let mut b = luaL_checkinteger(L, 2 as libc::c_int);
    lua_pushboolean(L, ((a as lua_Unsigned) < b as lua_Unsigned) as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_log(mut L: *mut lua_State) -> libc::c_int {
    let mut x = luaL_checknumber(L, 1 as libc::c_int);
    let mut res: lua_Number = 0.;
    if lua_isnoneornil!(L, 2) != 0 {
        res = l_mathop!(log)(x);
    } else {
        let mut base = luaL_checknumber(L, 2 as libc::c_int);
        if base == l_mathop!(2.0) {
            res = l_mathop!(log2)(x);
        } else if base == l_mathop!(10.0) {
            res = l_mathop!(log10)(x);
        } else {
            res = l_mathop!(log)(x) / l_mathop!(log)(base);
        }
    }
    lua_pushnumber(L, res);
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_exp(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, l_mathop!(exp)(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_deg(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, luaL_checknumber(L, 1 as libc::c_int) * (l_mathop!(180.0) / PI));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_rad(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, luaL_checknumber(L, 1 as libc::c_int) * (PI / l_mathop!(180.0)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_min(mut L: *mut lua_State) -> libc::c_int {
    let mut n = lua_gettop(L);
    let mut imin = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 2 as libc::c_int;
    while i <= n {
        if lua_compare(L, i, imin, LUA_OPLT) != 0 {
            imin = i;
        }
        i += 1;
    }
    lua_pushvalue(L, imin);
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_max(mut L: *mut lua_State) -> libc::c_int {
    let mut n = lua_gettop(L);
    let mut imax = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 2 as libc::c_int;
    while i <= n {
        if lua_compare(L, imax, i, LUA_OPLT) != 0 {
            imax = i;
        }
        i += 1;
    }
    lua_pushvalue(L, imax);
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_random(mut L: *mut lua_State) -> libc::c_int {
    let mut low: lua_Integer = 0;
    let mut up: lua_Integer = 0;
    let mut r = l_rand!() as libc::c_double
        * (1.0f64 / (L_RANDMAX as libc::c_double + 1.0f64));
    match lua_gettop(L) {
        0 => {
            lua_pushnumber(L, r);
            return 1 as libc::c_int;
        }
        1 => {
            low = 1 as libc::c_int as lua_Integer;
            up = luaL_checkinteger(L, 1 as libc::c_int);
        }
        2 => {
            low = luaL_checkinteger(L, 1 as libc::c_int);
            up = luaL_checkinteger(L, 2 as libc::c_int);
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    r *= (up - low) as libc::c_double + 1.0f64;
    lua_pushinteger(L, r as lua_Integer + low);
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_randomseed(mut L: *mut lua_State) -> libc::c_int {
    srand(luaL_checknumber(L, 1 as libc::c_int) as lua_Integer as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_type(mut L: *mut lua_State) -> libc::c_int {
    if lua_type(L, 1 as libc::c_int) == LUA_TNUMBER {
        if lua_isinteger(L, 1 as libc::c_int) != 0 {
            lua_pushliteral!(
                L, "integer"
            )(L, b"integer\0" as *const u8 as *const libc::c_char);
        } else {
            lua_pushliteral!(
                L, "float"
            )(L, b"float\0" as *const u8 as *const libc::c_char);
        }
    } else {
        luaL_checkany(L, 1 as libc::c_int);
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
static mut mathlib: [luaL_Reg; 28] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"abs\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_abs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"acos\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_acos as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"asin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_asin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"atan\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_atan as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ceil\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_ceil as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"cos\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_cos as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"deg\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_deg as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"exp\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_exp as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tointeger\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_toint as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"floor\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_floor as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fmod\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_fmod as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ult\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_ult as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"log\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_log as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"max\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_max as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"min\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_min as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"modf\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_modf as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rad\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_rad as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"random\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_random as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"randomseed\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_randomseed
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_sin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sqrt\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_sqrt as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tan\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_tan as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_type as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pi\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"huge\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"maxinteger\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"mininteger\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaopen_math(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 28]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    lua_pushnumber(L, PI);
    lua_setfield(L, -(2 as libc::c_int), b"pi\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(L, HUGE_VAL);
    lua_setfield(L, -(2 as libc::c_int), b"huge\0" as *const u8 as *const libc::c_char);
    lua_pushinteger(L, LUA_MAXINTEGER);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"maxinteger\0" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(L, LUA_MININTEGER);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"mininteger\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
pub const LUA_IGMARK: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"-\0")
};
pub const LUA_CSUBSEP: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"/\0")
};
pub const LUA_LSUBSEP: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"/\0")
};
pub const LUA_OFSEP: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"_\0")
};
static mut CLIBS: libc::c_int = 0 as libc::c_int;
pub const LIB_FAIL: [libc::c_char; 7] = unsafe {
    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"absent\0")
};
unsafe extern "C" fn lsys_unloadlib(mut lib: *mut libc::c_void) {}
unsafe extern "C" fn lsys_load(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
    mut seeglb: libc::c_int,
) -> *mut libc::c_void {
    lua_pushliteral!(
        L, DLMSG
    )(
        L,
        b"dynamic libraries not enabled; check your Lua installation\0" as *const u8
            as *const libc::c_char,
    );
    return NULL as *mut libc::c_void;
}
unsafe extern "C" fn lsys_sym(
    mut L: *mut lua_State,
    mut lib: *mut libc::c_void,
    mut sym: *const libc::c_char,
) -> lua_CFunction {
    lua_pushliteral!(
        L, DLMSG
    )(
        L,
        b"dynamic libraries not enabled; check your Lua installation\0" as *const u8
            as *const libc::c_char,
    );
    return ::core::mem::transmute::<
        libc::intptr_t,
        lua_CFunction,
    >(NULL as libc::intptr_t);
}
pub const LUA_PATH_VAR: [libc::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"LUA_PATH\0")
};
pub const LUA_CPATH_VAR: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"LUA_CPATH\0")
};
pub const AUXMARK: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"\x01\0")
};
unsafe extern "C" fn noenv(mut L: *mut lua_State) -> libc::c_int {
    let mut b: libc::c_int = 0;
    lua_getfield(
        L,
        LUA_REGISTRYINDEX,
        b"LUA_NOENV\0" as *const u8 as *const libc::c_char,
    );
    b = lua_toboolean(L, -(1 as libc::c_int));
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    return b;
}
unsafe extern "C" fn setpath(
    mut L: *mut lua_State,
    mut fieldname: *const libc::c_char,
    mut envname: *const libc::c_char,
    mut dft: *const libc::c_char,
) {
    let mut nver = lua_pushfstring(
        L,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        envname,
        b"_5_3\0" as *const u8 as *const libc::c_char,
    );
    let mut path: *const libc::c_char = getenv(nver);
    if path.is_null() {
        path = getenv(envname);
    }
    if path.is_null() || noenv(L) != 0 {
        lua_pushstring(L, dft);
    } else {
        path = luaL_gsub(
            L,
            path,
            b";;\0" as *const u8 as *const libc::c_char,
            b";\x01;\0" as *const u8 as *const libc::c_char,
        );
        luaL_gsub(L, path, AUXMARK.as_ptr(), dft);
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    lua_setfield(L, -(3 as libc::c_int), fieldname);
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
}
unsafe extern "C" fn checkclib(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
) -> *mut libc::c_void {
    let mut plib = 0 as *mut libc::c_void;
    lua_rawgetp(
        L,
        LUA_REGISTRYINDEX,
        &CLIBS as *const libc::c_int as *const libc::c_void,
    );
    lua_getfield(L, -(1 as libc::c_int), path);
    plib = lua_touserdata(L, -(1 as libc::c_int));
    lua_pop!(L, 2)(L, lua_pop!(L, 2));
    return plib;
}
unsafe extern "C" fn addtoclib(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
    mut plib: *mut libc::c_void,
) {
    lua_rawgetp(
        L,
        LUA_REGISTRYINDEX,
        &CLIBS as *const libc::c_int as *const libc::c_void,
    );
    lua_pushlightuserdata(L, plib);
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_setfield(L, -(3 as libc::c_int), path);
    lua_rawseti(
        L,
        -(2 as libc::c_int),
        luaL_len(L, -(2 as libc::c_int)) + 1 as libc::c_int as libc::c_longlong,
    );
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
}
unsafe extern "C" fn gctm(mut L: *mut lua_State) -> libc::c_int {
    let mut n = luaL_len(L, 1 as libc::c_int);
    while n >= 1 as libc::c_int as libc::c_longlong {
        lua_rawgeti(L, 1 as libc::c_int, n);
        lsys_unloadlib(lua_touserdata(L, -(1 as libc::c_int)));
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        n -= 1;
    }
    return 0 as libc::c_int;
}
pub const ERRLIB: libc::c_int = 1 as libc::c_int;
pub const ERRFUNC: libc::c_int = 2 as libc::c_int;
unsafe extern "C" fn lookforfunc(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
    mut sym: *const libc::c_char,
) -> libc::c_int {
    let mut reg = checkclib(L, path);
    if reg.is_null() {
        reg = lsys_load(L, path, (*sym as libc::c_int == '*' as i32) as libc::c_int);
        if reg.is_null() {
            return ERRLIB;
        }
        addtoclib(L, path, reg);
    }
    if *sym as libc::c_int == '*' as i32 {
        lua_pushboolean(L, 1 as libc::c_int);
        return 0 as libc::c_int;
    } else {
        let mut f = lsys_sym(L, reg, sym);
        if f.is_none() {
            return ERRFUNC;
        }
        lua_pushcfunction!(L, f)(L, lua_pushcfunction!(L, f), lua_pushcfunction!(L, f));
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn ll_loadlib(mut L: *mut lua_State) -> libc::c_int {
    let mut path = luaL_checkstring!(L, 1);
    let mut init = luaL_checkstring!(L, 2);
    let mut stat = lookforfunc(L, path, init);
    if stat == 0 as libc::c_int {
        return 1 as libc::c_int
    } else {
        lua_pushnil(L);
        lua_insert!(L, - 2)(L, lua_insert!(L, - 2), lua_insert!(L, - 2));
        lua_pushstring(
            L,
            if stat == ERRLIB {
                LIB_FAIL.as_ptr()
            } else {
                b"init\0" as *const u8 as *const libc::c_char
            },
        );
        return 3 as libc::c_int;
    };
}
unsafe extern "C" fn readable(mut filename: *const libc::c_char) -> libc::c_int {
    let mut f = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return 0 as libc::c_int;
    }
    fclose(f);
    return 1 as libc::c_int;
}
unsafe extern "C" fn pushnexttemplate(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
) -> *const libc::c_char {
    let mut l = 0 as *const libc::c_char;
    while *path as libc::c_int == *LUA_PATH_SEP.as_ptr() as libc::c_int {
        path = path.offset(1);
    }
    if *path as libc::c_int == '\0' as i32 {
        return NULL as *const libc::c_char;
    }
    l = strchr(path, *LUA_PATH_SEP.as_ptr() as libc::c_int);
    if l.is_null() {
        l = path.offset(strlen(path) as isize);
    }
    lua_pushlstring(L, path, l.offset_from(path) as libc::c_long as size_t);
    return l;
}
unsafe extern "C" fn searchpath(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
    mut path: *const libc::c_char,
    mut sep: *const libc::c_char,
    mut dirsep: *const libc::c_char,
) -> *const libc::c_char {
    let mut msg = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut msg);
    if *sep as libc::c_int != '\0' as i32 {
        name = luaL_gsub(L, name, sep, dirsep);
    }
    loop {
        path = pushnexttemplate(L, path);
        if path.is_null() {
            break;
        }
        let mut filename = luaL_gsub(
            L,
            lua_tostring!(L, - 1),
            LUA_PATH_MARK.as_ptr(),
            name,
        );
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        if readable(filename) != 0 {
            return filename;
        }
        lua_pushfstring(
            L,
            b"\n\tno file '%s'\0" as *const u8 as *const libc::c_char,
            filename,
        );
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        luaL_addvalue(&mut msg);
    }
    luaL_pushresult(&mut msg);
    return NULL as *const libc::c_char;
}
unsafe extern "C" fn ll_searchpath(mut L: *mut lua_State) -> libc::c_int {
    let mut f = searchpath(
        L,
        luaL_checkstring!(L, 1),
        luaL_checkstring!(L, 2),
        luaL_optstring!(L, 3, "."),
        luaL_optstring!(L, 4, LUA_DIRSEP),
    );
    if !f.is_null() {
        return 1 as libc::c_int
    } else {
        lua_pushnil(L);
        lua_insert!(L, - 2)(L, lua_insert!(L, - 2), lua_insert!(L, - 2));
        return 2 as libc::c_int;
    };
}
unsafe extern "C" fn findfile(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
    mut pname: *const libc::c_char,
    mut dirsep: *const libc::c_char,
) -> *const libc::c_char {
    let mut path = 0 as *const libc::c_char;
    lua_getfield(L, lua_upvalueindex!(1), pname);
    path = lua_tostring!(L, - 1);
    if path.is_null() {
        luaL_error(
            L,
            b"'package.%s' must be a string\0" as *const u8 as *const libc::c_char,
            pname,
        );
    }
    return searchpath(L, name, path, b".\0" as *const u8 as *const libc::c_char, dirsep);
}
unsafe extern "C" fn checkload(
    mut L: *mut lua_State,
    mut stat: libc::c_int,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    if stat != 0 {
        lua_pushstring(L, filename);
        return 2 as libc::c_int;
    } else {
        return luaL_error(
            L,
            b"error loading module '%s' from file '%s':\n\t%s\0" as *const u8
                as *const libc::c_char,
            lua_tostring!(L, 1),
            filename,
            lua_tostring!(L, - 1),
        )
    };
}
unsafe extern "C" fn searcher_Lua(mut L: *mut lua_State) -> libc::c_int {
    let mut filename = 0 as *const libc::c_char;
    let mut name = luaL_checkstring!(L, 1);
    filename = findfile(
        L,
        name,
        b"path\0" as *const u8 as *const libc::c_char,
        LUA_LSUBSEP.as_ptr(),
    );
    if filename.is_null() {
        return 1 as libc::c_int;
    }
    return checkload(
        L,
        (luaL_loadfile!(L, filename) == LUA_OK) as libc::c_int,
        filename,
    );
}
unsafe extern "C" fn loadfunc(
    mut L: *mut lua_State,
    mut filename: *const libc::c_char,
    mut modname: *const libc::c_char,
) -> libc::c_int {
    let mut openfunc = 0 as *const libc::c_char;
    let mut mark = 0 as *const libc::c_char;
    modname = luaL_gsub(
        L,
        modname,
        b".\0" as *const u8 as *const libc::c_char,
        LUA_OFSEP.as_ptr(),
    );
    mark = strchr(modname, *LUA_IGMARK.as_ptr() as libc::c_int);
    if !mark.is_null() {
        let mut stat: libc::c_int = 0;
        openfunc = lua_pushlstring(
            L,
            modname,
            mark.offset_from(modname) as libc::c_long as size_t,
        );
        openfunc = lua_pushfstring(
            L,
            b"luaopen_%s\0" as *const u8 as *const libc::c_char,
            openfunc,
        );
        stat = lookforfunc(L, filename, openfunc);
        if stat != ERRFUNC {
            return stat;
        }
        modname = mark.offset(1 as libc::c_int as isize);
    }
    openfunc = lua_pushfstring(
        L,
        b"luaopen_%s\0" as *const u8 as *const libc::c_char,
        modname,
    );
    return lookforfunc(L, filename, openfunc);
}
unsafe extern "C" fn searcher_C(mut L: *mut lua_State) -> libc::c_int {
    let mut name = luaL_checkstring!(L, 1);
    let mut filename = findfile(
        L,
        name,
        b"cpath\0" as *const u8 as *const libc::c_char,
        LUA_CSUBSEP.as_ptr(),
    );
    if filename.is_null() {
        return 1 as libc::c_int;
    }
    return checkload(
        L,
        (loadfunc(L, filename, name) == 0 as libc::c_int) as libc::c_int,
        filename,
    );
}
unsafe extern "C" fn searcher_Croot(mut L: *mut lua_State) -> libc::c_int {
    let mut filename = 0 as *const libc::c_char;
    let mut name = luaL_checkstring!(L, 1);
    let mut p: *const libc::c_char = strchr(name, '.' as i32);
    let mut stat: libc::c_int = 0;
    if p.is_null() {
        return 0 as libc::c_int;
    }
    lua_pushlstring(L, name, p.offset_from(name) as libc::c_long as size_t);
    filename = findfile(
        L,
        lua_tostring!(L, - 1),
        b"cpath\0" as *const u8 as *const libc::c_char,
        LUA_CSUBSEP.as_ptr(),
    );
    if filename.is_null() {
        return 1 as libc::c_int;
    }
    stat = loadfunc(L, filename, name);
    if stat != 0 as libc::c_int {
        if stat != ERRFUNC {
            return checkload(L, 0 as libc::c_int, filename)
        } else {
            lua_pushfstring(
                L,
                b"\n\tno module '%s' in file '%s'\0" as *const u8 as *const libc::c_char,
                name,
                filename,
            );
            return 1 as libc::c_int;
        }
    }
    lua_pushstring(L, filename);
    return 2 as libc::c_int;
}
unsafe extern "C" fn searcher_preload(mut L: *mut lua_State) -> libc::c_int {
    let mut name = luaL_checkstring!(L, 1);
    lua_getfield(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE.as_ptr());
    if lua_getfield(L, -(1 as libc::c_int), name) == LUA_TNIL {
        lua_pushfstring(
            L,
            b"\n\tno field package.preload['%s']\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn findloader(mut L: *mut lua_State, mut name: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut msg = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut msg);
    if lua_getfield(
        L,
        lua_upvalueindex!(1),
        b"searchers\0" as *const u8 as *const libc::c_char,
    ) != LUA_TTABLE
    {
        luaL_error(
            L,
            b"'package.searchers' must be a table\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 1 as libc::c_int;
    loop {
        if lua_rawgeti(L, 3 as libc::c_int, i as lua_Integer) == LUA_TNIL {
            lua_pop!(L, 1)(L, lua_pop!(L, 1));
            luaL_pushresult(&mut msg);
            luaL_error(
                L,
                b"module '%s' not found:%s\0" as *const u8 as *const libc::c_char,
                name,
                lua_tostring!(L, - 1),
            );
        }
        lua_pushstring(L, name);
        lua_call!(
            L, 1, 2
        )(
            L,
            lua_call!(L, 1, 2),
            lua_call!(L, 1, 2),
            lua_call!(L, 1, 2),
            ::core::mem::transmute::<
                libc::intptr_t,
                lua_KFunction,
            >(NULL as libc::intptr_t),
        );
        if lua_isfunction!(L, - 2) != 0 {
            return
        } else {
            if lua_isstring(L, -(2 as libc::c_int)) != 0 {
                lua_pop!(L, 1)(L, lua_pop!(L, 1));
                luaL_addvalue(&mut msg);
            } else {
                lua_pop!(L, 2)(L, lua_pop!(L, 2));
            }
        }
        i += 1;
    };
}
unsafe extern "C" fn ll_require(mut L: *mut lua_State) -> libc::c_int {
    let mut name = luaL_checkstring!(L, 1);
    lua_settop(L, 1 as libc::c_int);
    lua_getfield(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE.as_ptr());
    lua_getfield(L, 2 as libc::c_int, name);
    if lua_toboolean(L, -(1 as libc::c_int)) != 0 {
        return 1 as libc::c_int;
    }
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    findloader(L, name);
    lua_pushstring(L, name);
    lua_insert!(L, - 2)(L, lua_insert!(L, - 2), lua_insert!(L, - 2));
    lua_call!(
        L, 2, 1
    )(
        L,
        lua_call!(L, 2, 1),
        lua_call!(L, 2, 1),
        lua_call!(L, 2, 1),
        ::core::mem::transmute::<libc::intptr_t, lua_KFunction>(NULL as libc::intptr_t),
    );
    if lua_isnil!(L, - 1) == 0 {
        lua_setfield(L, 2 as libc::c_int, name);
    }
    if lua_getfield(L, 2 as libc::c_int, name) == LUA_TNIL {
        lua_pushboolean(L, 1 as libc::c_int);
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_setfield(L, 2 as libc::c_int, name);
    }
    return 1 as libc::c_int;
}
static mut pk_funcs: [luaL_Reg; 8] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"loadlib\0" as *const u8 as *const libc::c_char,
                func: Some(
                    ll_loadlib as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"searchpath\0" as *const u8 as *const libc::c_char,
                func: Some(
                    ll_searchpath as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"preload\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"cpath\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"path\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"searchers\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"loaded\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
static mut ll_funcs: [luaL_Reg; 2] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"require\0" as *const u8 as *const libc::c_char,
                func: Some(
                    ll_require as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
unsafe extern "C" fn createsearcherstable(mut L: *mut lua_State) {
    static mut searchers: [lua_CFunction; 5] = unsafe {
        [
            Some(
                searcher_preload as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            Some(searcher_Lua as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            Some(searcher_C as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            Some(searcher_Croot as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            ::core::mem::transmute::<
                libc::intptr_t,
                lua_CFunction,
            >(NULL as libc::intptr_t),
        ]
    };
    let mut i: libc::c_int = 0;
    lua_createtable(
        L,
        (::core::mem::size_of::<[lua_CFunction; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<lua_CFunction>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        0 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while (searchers[i as usize]).is_some() {
        lua_pushvalue(L, -(2 as libc::c_int));
        lua_pushcclosure(L, searchers[i as usize], 1 as libc::c_int);
        lua_rawseti(L, -(2 as libc::c_int), (i + 1 as libc::c_int) as lua_Integer);
        i += 1;
    }
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"searchers\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn createclibstable(mut L: *mut lua_State) {
    lua_newtable!(L)(L, lua_newtable!(L), lua_newtable!(L));
    lua_createtable(L, 0 as libc::c_int, 1 as libc::c_int);
    lua_pushcfunction!(
        L, gctm
    )(L, lua_pushcfunction!(L, gctm), lua_pushcfunction!(L, gctm));
    lua_setfield(L, -(2 as libc::c_int), b"__gc\0" as *const u8 as *const libc::c_char);
    lua_setmetatable(L, -(2 as libc::c_int));
    lua_rawsetp(
        L,
        LUA_REGISTRYINDEX,
        &CLIBS as *const libc::c_int as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_package(mut L: *mut lua_State) -> libc::c_int {
    createclibstable(L);
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    createsearcherstable(L);
    setpath(
        L,
        b"path\0" as *const u8 as *const libc::c_char,
        LUA_PATH_VAR.as_ptr(),
        b"/usr/local/share/lua/5.3/?.lua;/usr/local/share/lua/5.3/?/init.lua;/usr/local/lib/lua/5.3/?.lua;/usr/local/lib/lua/5.3/?/init.lua;./?.lua;./?/init.lua\0"
            as *const u8 as *const libc::c_char,
    );
    setpath(
        L,
        b"cpath\0" as *const u8 as *const libc::c_char,
        LUA_CPATH_VAR.as_ptr(),
        b"/usr/local/lib/lua/5.3/?.so;/usr/local/lib/lua/5.3/loadall.so;./?.so\0"
            as *const u8 as *const libc::c_char,
    );
    lua_pushliteral!(
        L, LUA_DIRSEP "\n" LUA_PATH_SEP "\n" LUA_PATH_MARK "\n" LUA_EXEC_DIR "\n"
        LUA_IGMARK "\n"
    )(L, b"/\n;\n?\n!\n-\n\0" as *const u8 as *const libc::c_char);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"config\0" as *const u8 as *const libc::c_char,
    );
    luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE.as_ptr());
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"loaded\0" as *const u8 as *const libc::c_char,
    );
    luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE.as_ptr());
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"preload\0" as *const u8 as *const libc::c_char,
    );
    lua_pushvalue(L, -(2 as libc::c_int));
    luaL_setfuncs(L, ll_funcs.as_ptr(), 1 as libc::c_int);
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    return 1 as libc::c_int;
}
pub const L_STRFTIMEC99: [libc::c_char; 78] = unsafe {
    *::core::mem::transmute::<
        &[u8; 78],
        &[libc::c_char; 78],
    >(b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\0")
};
pub const LUA_STRFTIMEOPTIONS: [libc::c_char; 78] = unsafe {
    *::core::mem::transmute::<
        &[u8; 78],
        &[libc::c_char; 78],
    >(b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\0")
};
unsafe extern "C" fn l_checktime(mut L: *mut lua_State, mut arg: libc::c_int) -> time_t {
    let mut t = luaL_checkinteger(L, arg);
    return t as time_t;
}
unsafe extern "C" fn os_execute(mut L: *mut lua_State) -> libc::c_int {
    let mut cmd = luaL_optstring!(L, 1, NULL);
    let mut stat = system(cmd);
    if !cmd.is_null() {
        return luaL_execresult(L, stat)
    } else {
        lua_pushboolean(L, stat);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn os_remove(mut L: *mut lua_State) -> libc::c_int {
    let mut filename = luaL_checkstring!(L, 1);
    return luaL_fileresult(
        L,
        (remove(filename) == 0 as libc::c_int) as libc::c_int,
        filename,
    );
}
unsafe extern "C" fn os_rename(mut L: *mut lua_State) -> libc::c_int {
    let mut fromname = luaL_checkstring!(L, 1);
    let mut toname = luaL_checkstring!(L, 2);
    return luaL_fileresult(
        L,
        (rename(fromname, toname) == 0 as libc::c_int) as libc::c_int,
        NULL as *const libc::c_char,
    );
}
unsafe extern "C" fn os_tmpname(mut L: *mut lua_State) -> libc::c_int {
    let mut buff: [libc::c_char; 20] = [0; 20];
    let mut err: libc::c_int = 0;
    err = lua_tmpnam!(buff, err);
    if err != 0 {
        return luaL_error(
            L,
            b"unable to generate a unique filename\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_pushstring(L, buff.as_mut_ptr());
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_getenv(mut L: *mut lua_State) -> libc::c_int {
    lua_pushstring(L, getenv(luaL_checkstring!(L, 1)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_clock(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, clock() as lua_Number / CLOCKS_PER_SEC as __clock_t as lua_Number);
    return 1 as libc::c_int;
}
unsafe extern "C" fn setfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut value: libc::c_int,
) {
    lua_pushinteger(L, value as lua_Integer);
    lua_setfield(L, -(2 as libc::c_int), key);
}
unsafe extern "C" fn setboolfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut value: libc::c_int,
) {
    if value < 0 as libc::c_int {
        return;
    }
    lua_pushboolean(L, value);
    lua_setfield(L, -(2 as libc::c_int), key);
}
unsafe extern "C" fn setallfields(mut L: *mut lua_State, mut stm: *mut tm) {
    setfield(L, b"sec\0" as *const u8 as *const libc::c_char, (*stm).tm_sec);
    setfield(L, b"min\0" as *const u8 as *const libc::c_char, (*stm).tm_min);
    setfield(L, b"hour\0" as *const u8 as *const libc::c_char, (*stm).tm_hour);
    setfield(L, b"day\0" as *const u8 as *const libc::c_char, (*stm).tm_mday);
    setfield(
        L,
        b"month\0" as *const u8 as *const libc::c_char,
        (*stm).tm_mon + 1 as libc::c_int,
    );
    setfield(
        L,
        b"year\0" as *const u8 as *const libc::c_char,
        (*stm).tm_year + 1900 as libc::c_int,
    );
    setfield(
        L,
        b"wday\0" as *const u8 as *const libc::c_char,
        (*stm).tm_wday + 1 as libc::c_int,
    );
    setfield(
        L,
        b"yday\0" as *const u8 as *const libc::c_char,
        (*stm).tm_yday + 1 as libc::c_int,
    );
    setboolfield(L, b"isdst\0" as *const u8 as *const libc::c_char, (*stm).tm_isdst);
}
unsafe extern "C" fn getboolfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    res = if lua_getfield(L, -(1 as libc::c_int), key) == LUA_TNIL {
        -(1 as libc::c_int)
    } else {
        lua_toboolean(L, -(1 as libc::c_int))
    };
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    return res;
}
pub const L_MAXDATEFIELD: libc::c_int = INT_MAX / 2 as libc::c_int;
unsafe extern "C" fn getfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut d: libc::c_int,
    mut delta: libc::c_int,
) -> libc::c_int {
    let mut isnum: libc::c_int = 0;
    let mut t = lua_getfield(L, -(1 as libc::c_int), key);
    let mut res = lua_tointegerx(L, -(1 as libc::c_int), &mut isnum);
    if isnum == 0 {
        if t != LUA_TNIL {
            return luaL_error(
                L,
                b"field '%s' is not an integer\0" as *const u8 as *const libc::c_char,
                key,
            )
        } else {
            if d < 0 as libc::c_int {
                return luaL_error(
                    L,
                    b"field '%s' missing in date table\0" as *const u8
                        as *const libc::c_char,
                    key,
                );
            }
        }
        res = d as lua_Integer;
    } else {
        if !(-L_MAXDATEFIELD as libc::c_longlong <= res
            && res <= L_MAXDATEFIELD as libc::c_longlong)
        {
            return luaL_error(
                L,
                b"field '%s' is out-of-bound\0" as *const u8 as *const libc::c_char,
                key,
            );
        }
        res -= delta as libc::c_longlong;
    }
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    return res as libc::c_int;
}
unsafe extern "C" fn checkoption(
    mut L: *mut lua_State,
    mut conv: *const libc::c_char,
    mut convlen: ptrdiff_t,
    mut buff: *mut libc::c_char,
) -> *const libc::c_char {
    let mut option = LUA_STRFTIMEOPTIONS.as_ptr();
    let mut oplen = 1 as libc::c_int;
    while *option as libc::c_int != '\0' as i32 && oplen as libc::c_long <= convlen {
        if *option as libc::c_int == '|' as i32 {
            oplen += 1;
        } else if memcmp(
            conv as *const libc::c_void,
            option as *const libc::c_void,
            oplen as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            memcpy(
                buff as *mut libc::c_void,
                conv as *const libc::c_void,
                oplen as libc::c_ulong,
            );
            *buff.offset(oplen as isize) = '\0' as i32 as libc::c_char;
            return conv.offset(oplen as isize);
        }
        option = option.offset(oplen as isize);
    }
    luaL_argerror(
        L,
        1 as libc::c_int,
        lua_pushfstring(
            L,
            b"invalid conversion specifier '%%%s'\0" as *const u8 as *const libc::c_char,
            conv,
        ),
    );
    return conv;
}
pub const SIZETIMEFMT: libc::c_int = 250 as libc::c_int;
unsafe extern "C" fn os_date(mut L: *mut lua_State) -> libc::c_int {
    let mut slen: size_t = 0;
    let mut s = luaL_optlstring(
        L,
        1 as libc::c_int,
        b"%c\0" as *const u8 as *const libc::c_char,
        &mut slen,
    );
    let mut t = if lua_type(L, 2 as libc::c_int) <= 0 as libc::c_int {
        time(0 as *mut time_t)
    } else {
        l_checktime(L, 2 as libc::c_int)
    };
    let mut se = s.offset(slen as isize);
    let mut tmr = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    let mut stm = 0 as *mut tm;
    if *s as libc::c_int == '!' as i32 {
        stm = l_gmtime!(& t, & tmr);
        s = s.offset(1);
    } else {
        stm = l_localtime!(& t, & tmr);
    }
    if stm.is_null() {
        return luaL_error(
            L,
            b"time result cannot be represented in this installation\0" as *const u8
                as *const libc::c_char,
        );
    }
    if strcmp(s, b"*t\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        lua_createtable(L, 0 as libc::c_int, 9 as libc::c_int);
        setallfields(L, stm);
    } else {
        let mut cc: [libc::c_char; 4] = [0; 4];
        let mut b = luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            initb: [0; 8192],
        };
        cc[0 as libc::c_int as usize] = '%' as i32 as libc::c_char;
        luaL_buffinit(L, &mut b);
        while s < se {
            if !(*s as libc::c_int != '%' as i32) {
                let mut reslen: size_t = 0;
                let mut buff = luaL_prepbuffsize(&mut b, SIZETIMEFMT as size_t);
                s = s.offset(1);
                s = checkoption(
                    L,
                    s,
                    se.offset_from(s) as libc::c_long,
                    cc.as_mut_ptr().offset(1 as libc::c_int as isize),
                );
                reslen = strftime(buff, SIZETIMEFMT as size_t, cc.as_mut_ptr(), stm);
                let ref mut fresh332 = luaL_addsize!(& b, reslen);
                *fresh332 = (*fresh332 as libc::c_ulong)
                    .wrapping_add(luaL_addsize!(& b, reslen)) as size_t as size_t;
            }
        }
        luaL_pushresult(&mut b);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_time(mut L: *mut lua_State) -> libc::c_int {
    let mut t: time_t = 0;
    if lua_isnoneornil!(L, 1) != 0 {
        t = time(NULL as *mut time_t);
    } else {
        let mut ts = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            __tm_gmtoff: 0,
            __tm_zone: 0 as *const libc::c_char,
        };
        luaL_checktype(L, 1 as libc::c_int, LUA_TTABLE);
        lua_settop(L, 1 as libc::c_int);
        ts
            .tm_sec = getfield(
            L,
            b"sec\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        ts
            .tm_min = getfield(
            L,
            b"min\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        ts
            .tm_hour = getfield(
            L,
            b"hour\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int,
            0 as libc::c_int,
        );
        ts
            .tm_mday = getfield(
            L,
            b"day\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            0 as libc::c_int,
        );
        ts
            .tm_mon = getfield(
            L,
            b"month\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            1 as libc::c_int,
        );
        ts
            .tm_year = getfield(
            L,
            b"year\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            1900 as libc::c_int,
        );
        ts.tm_isdst = getboolfield(L, b"isdst\0" as *const u8 as *const libc::c_char);
        t = mktime(&mut ts);
        setallfields(L, &mut ts);
    }
    if t != t as lua_Integer as time_t || t == -(1 as libc::c_int) as time_t {
        return luaL_error(
            L,
            b"time result cannot be represented in this installation\0" as *const u8
                as *const libc::c_char,
        );
    }
    l_pushtime!(L, t)(L, l_pushtime!(L, t));
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_difftime(mut L: *mut lua_State) -> libc::c_int {
    let mut t1 = l_checktime(L, 1 as libc::c_int);
    let mut t2 = l_checktime(L, 2 as libc::c_int);
    lua_pushnumber(L, difftime(t1, t2));
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_setlocale(mut L: *mut lua_State) -> libc::c_int {
    static mut cat: [libc::c_int; 6] = [
        LC_ALL,
        LC_COLLATE,
        LC_CTYPE,
        LC_MONETARY,
        LC_NUMERIC,
        LC_TIME,
    ];
    static mut catnames: [*const libc::c_char; 7] = [
        b"all\0" as *const u8 as *const libc::c_char,
        b"collate\0" as *const u8 as *const libc::c_char,
        b"ctype\0" as *const u8 as *const libc::c_char,
        b"monetary\0" as *const u8 as *const libc::c_char,
        b"numeric\0" as *const u8 as *const libc::c_char,
        b"time\0" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    let mut l = luaL_optstring!(L, 1, NULL);
    let mut op = luaL_checkoption(
        L,
        2 as libc::c_int,
        b"all\0" as *const u8 as *const libc::c_char,
        catnames.as_ptr(),
    );
    lua_pushstring(L, setlocale(cat[op as usize], l));
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_exit(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if lua_isboolean!(L, 1) != 0 {
        status = if lua_toboolean(L, 1 as libc::c_int) != 0 {
            EXIT_SUCCESS
        } else {
            EXIT_FAILURE
        };
    } else {
        status = luaL_optinteger(L, 1 as libc::c_int, EXIT_SUCCESS as lua_Integer)
            as libc::c_int;
    }
    if lua_toboolean(L, 2 as libc::c_int) != 0 {
        lua_close(L);
    }
    if !L.is_null() {
        exit(status);
    }
    return 0 as libc::c_int;
}
static mut syslib: [luaL_Reg; 12] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"clock\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_clock as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"date\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_date as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"difftime\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_difftime as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"execute\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_execute as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"exit\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_exit as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getenv\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_getenv as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"remove\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_remove as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rename\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_rename as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setlocale\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_setlocale as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"time\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_time as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tmpname\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_tmpname as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaopen_os(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 12]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    return 1 as libc::c_int;
}
pub const LUA_MAXCAPTURES: libc::c_int = 32 as libc::c_int;
pub const MAX_SIZET_0: size_t = !(0 as libc::c_int as size_t);
unsafe extern "C" fn str_len(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    luaL_checklstring(L, 1 as libc::c_int, &mut l);
    lua_pushinteger(L, l as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn posrelat(mut pos: lua_Integer, mut len: size_t) -> lua_Integer {
    if pos >= 0 as libc::c_int as libc::c_longlong {
        return pos
    } else if (0 as libc::c_uint as libc::c_ulong).wrapping_sub(pos as size_t) > len {
        return 0 as libc::c_int as lua_Integer
    } else {
        return len as lua_Integer + pos + 1 as libc::c_int as libc::c_longlong
    };
}
unsafe extern "C" fn str_sub(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut start = posrelat(luaL_checkinteger(L, 2 as libc::c_int), l);
    let mut end = posrelat(
        luaL_optinteger(L, 3 as libc::c_int, -(1 as libc::c_int) as lua_Integer),
        l,
    );
    if start < 1 as libc::c_int as libc::c_longlong {
        start = 1 as libc::c_int as lua_Integer;
    }
    if end > l as lua_Integer {
        end = l as lua_Integer;
    }
    if start <= end {
        lua_pushlstring(
            L,
            s.offset(start as isize).offset(-(1 as libc::c_int as isize)),
            ((end - start) as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        lua_pushliteral!(L, "")(L, b"\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_reverse(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut p = luaL_buffinitsize(L, &mut b, l);
    i = 0 as libc::c_int as size_t;
    while i < l {
        *p
            .offset(
                i as isize,
            ) = *s
            .offset(
                l.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            );
        i = i.wrapping_add(1);
    }
    luaL_pushresultsize(&mut b, l);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_lower(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut p = luaL_buffinitsize(L, &mut b, l);
    i = 0 as libc::c_int as size_t;
    while i < l {
        *p
            .offset(
                i as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c = *s.offset(i as isize) as libc::c_uchar as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(
                        *s.offset(i as isize) as libc::c_uchar as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *s.offset(i as isize) as libc::c_uchar as libc::c_int as isize,
                    );
            }
            __res
        }) as libc::c_char;
        i = i.wrapping_add(1);
    }
    luaL_pushresultsize(&mut b, l);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_upper(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut p = luaL_buffinitsize(L, &mut b, l);
    i = 0 as libc::c_int as size_t;
    while i < l {
        *p
            .offset(
                i as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c = *s.offset(i as isize) as libc::c_uchar as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(
                        *s.offset(i as isize) as libc::c_uchar as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(
                        *s.offset(i as isize) as libc::c_uchar as libc::c_int as isize,
                    );
            }
            __res
        }) as libc::c_char;
        i = i.wrapping_add(1);
    }
    luaL_pushresultsize(&mut b, l);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_rep(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut lsep: size_t = 0;
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut n = luaL_checkinteger(L, 2 as libc::c_int);
    let mut sep = luaL_optlstring(
        L,
        3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        &mut lsep,
    );
    if n <= 0 as libc::c_int as libc::c_longlong {
        lua_pushliteral!(L, "")(L, b"\0" as *const u8 as *const libc::c_char);
    } else if l.wrapping_add(lsep) < l
        || l.wrapping_add(lsep) as libc::c_ulonglong
            > ((if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                MAX_SIZET_0
            } else {
                2147483647 as libc::c_int as size_t
            }) as libc::c_ulonglong)
                .wrapping_div(n as libc::c_ulonglong)
    {
        return luaL_error(
            L,
            b"resulting string too large\0" as *const u8 as *const libc::c_char,
        )
    } else {
        let mut totallen = (n as size_t)
            .wrapping_mul(l)
            .wrapping_add(
                ((n - 1 as libc::c_int as libc::c_longlong) as size_t).wrapping_mul(lsep),
            );
        let mut b = luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            initb: [0; 8192],
        };
        let mut p = luaL_buffinitsize(L, &mut b, totallen);
        loop {
            let fresh333 = n;
            n = n - 1;
            if !(fresh333 > 1 as libc::c_int as libc::c_longlong) {
                break;
            }
            memcpy(
                p as *mut libc::c_void,
                s as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            p = p.offset(l as isize);
            if lsep > 0 as libc::c_int as libc::c_ulong {
                memcpy(
                    p as *mut libc::c_void,
                    sep as *const libc::c_void,
                    lsep
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                p = p.offset(lsep as isize);
            }
        }
        memcpy(
            p as *mut libc::c_void,
            s as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        luaL_pushresultsize(&mut b, totallen);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_byte(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut posi = posrelat(
        luaL_optinteger(L, 2 as libc::c_int, 1 as libc::c_int as lua_Integer),
        l,
    );
    let mut pose = posrelat(luaL_optinteger(L, 3 as libc::c_int, posi), l);
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if posi < 1 as libc::c_int as libc::c_longlong {
        posi = 1 as libc::c_int as lua_Integer;
    }
    if pose > l as lua_Integer {
        pose = l as lua_Integer;
    }
    if posi > pose {
        return 0 as libc::c_int;
    }
    if pose - posi >= INT_MAX as libc::c_longlong {
        return luaL_error(
            L,
            b"string slice too long\0" as *const u8 as *const libc::c_char,
        );
    }
    n = (pose - posi) as libc::c_int + 1 as libc::c_int;
    luaL_checkstack(
        L,
        n,
        b"string slice too long\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < n {
        lua_pushinteger(L, uchar!(s[posi + i - 1]));
        i += 1;
    }
    return n;
}
unsafe extern "C" fn str_char(mut L: *mut lua_State) -> libc::c_int {
    let mut n = lua_gettop(L);
    let mut i: libc::c_int = 0;
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut p = luaL_buffinitsize(L, &mut b, n as size_t);
    i = 1 as libc::c_int;
    while i <= n {
        let mut c = luaL_checkinteger(L, i);
        *p.offset((i - 1 as libc::c_int) as isize) = uchar!(c);
        i += 1;
    }
    luaL_pushresultsize(&mut b, n as size_t);
    return 1 as libc::c_int;
}
unsafe extern "C" fn writer(
    mut L: *mut lua_State,
    mut b: *const libc::c_void,
    mut size: size_t,
    mut B: *mut libc::c_void,
) -> libc::c_int {
    luaL_addlstring(B as *mut luaL_Buffer, b as *const libc::c_char, size);
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_dump(mut L: *mut lua_State) -> libc::c_int {
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut strip = lua_toboolean(L, 2 as libc::c_int);
    luaL_checktype(L, 1 as libc::c_int, LUA_TFUNCTION);
    lua_settop(L, 1 as libc::c_int);
    luaL_buffinit(L, &mut b);
    if lua_dump(
        L,
        Some(
            writer
                as unsafe extern "C" fn(
                    *mut lua_State,
                    *const libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut b as *mut luaL_Buffer as *mut libc::c_void,
        strip,
    ) != 0 as libc::c_int
    {
        return luaL_error(
            L,
            b"unable to dump given function\0" as *const u8 as *const libc::c_char,
        );
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
pub const CAP_UNFINISHED: libc::c_int = -(1 as libc::c_int);
pub const CAP_POSITION: libc::c_int = -(2 as libc::c_int);
pub const MAXCCALLS: libc::c_int = 200 as libc::c_int;
pub const L_ESC: libc::c_int = 37;
pub const SPECIALS: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"^$*+?.([%-\0")
};
unsafe extern "C" fn check_capture(
    mut ms: *mut MatchState,
    mut l: libc::c_int,
) -> libc::c_int {
    l -= '1' as i32;
    if l < 0 as libc::c_int || l >= (*ms).level as libc::c_int
        || (*ms).capture[l as usize].len == CAP_UNFINISHED as libc::c_long
    {
        return luaL_error(
            (*ms).L,
            b"invalid capture index %%%d\0" as *const u8 as *const libc::c_char,
            l + 1 as libc::c_int,
        );
    }
    return l;
}
unsafe extern "C" fn capture_to_close(mut ms: *mut MatchState) -> libc::c_int {
    let mut level = (*ms).level as libc::c_int;
    level -= 1;
    while level >= 0 as libc::c_int {
        if (*ms).capture[level as usize].len == CAP_UNFINISHED as libc::c_long {
            return level;
        }
        level -= 1;
    }
    return luaL_error(
        (*ms).L,
        b"invalid pattern capture\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn classend(
    mut ms: *mut MatchState,
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    let fresh334 = p;
    p = p.offset(1);
    match *fresh334 as libc::c_int {
        L_ESC => {
            if p == (*ms).p_end {
                luaL_error(
                    (*ms).L,
                    b"malformed pattern (ends with '%%')\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return p.offset(1 as libc::c_int as isize);
        }
        91 => {
            if *p as libc::c_int == '^' as i32 {
                p = p.offset(1);
            }
            loop {
                if p == (*ms).p_end {
                    luaL_error(
                        (*ms).L,
                        b"malformed pattern (missing ']')\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                let fresh335 = p;
                p = p.offset(1);
                if *fresh335 as libc::c_int == L_ESC && p < (*ms).p_end {
                    p = p.offset(1);
                }
                if !(*p as libc::c_int != ']' as i32) {
                    break;
                }
            }
            return p.offset(1 as libc::c_int as isize);
        }
        _ => return p,
    };
}
unsafe extern "C" fn match_class(
    mut c: libc::c_int,
    mut cl: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    match tolower!(cl) {
        97 => {
            res = isalpha!(c);
        }
        99 => {
            res = iscntrl!(c);
        }
        100 => {
            res = isdigit!(c);
        }
        103 => {
            res = isgraph!(c);
        }
        108 => {
            res = islower!(c);
        }
        112 => {
            res = ispunct!(c);
        }
        115 => {
            res = isspace!(c);
        }
        117 => {
            res = isupper!(c);
        }
        119 => {
            res = isalnum!(c);
        }
        120 => {
            res = isxdigit!(c);
        }
        122 => {
            res = (c == 0 as libc::c_int) as libc::c_int;
        }
        _ => return (cl == c) as libc::c_int,
    }
    return if islower!(cl) != 0 { res } else { (res == 0) as libc::c_int };
}
unsafe extern "C" fn matchbracketclass(
    mut c: libc::c_int,
    mut p: *const libc::c_char,
    mut ec: *const libc::c_char,
) -> libc::c_int {
    let mut sig = 1 as libc::c_int;
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '^' as i32 {
        sig = 0 as libc::c_int;
        p = p.offset(1);
    }
    loop {
        p = p.offset(1);
        if !(p < ec) {
            break;
        }
        if *p as libc::c_int == L_ESC {
            p = p.offset(1);
            if match_class(c, uchar!(* p)) != 0 {
                return sig;
            }
        } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
            && p.offset(2 as libc::c_int as isize) < ec
        {
            p = p.offset(2 as libc::c_int as isize);
            if *p.offset(-(2 as libc::c_int as isize)) as libc::c_uchar as libc::c_int
                <= c && c <= uchar!(* p)
            {
                return sig;
            }
        } else if uchar!(* p) == c {
            return sig
        }
    }
    return (sig == 0) as libc::c_int;
}
unsafe extern "C" fn singlematch(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut ep: *const libc::c_char,
) -> libc::c_int {
    if s >= (*ms).src_end {
        return 0 as libc::c_int
    } else {
        let mut c = uchar!(* s);
        match *p as libc::c_int {
            46 => return 1 as libc::c_int,
            L_ESC => {
                return match_class(
                    c,
                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int,
                );
            }
            91 => return matchbracketclass(c, p, ep.offset(-(1 as libc::c_int as isize))),
            _ => return (uchar!(* p) == c) as libc::c_int,
        }
    };
}
unsafe extern "C" fn matchbalance(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    if p >= ((*ms).p_end).offset(-(1 as libc::c_int as isize)) {
        luaL_error(
            (*ms).L,
            b"malformed pattern (missing arguments to '%%b')\0" as *const u8
                as *const libc::c_char,
        );
    }
    if *s as libc::c_int != *p as libc::c_int {
        return NULL as *const libc::c_char
    } else {
        let mut b = *p as libc::c_int;
        let mut e = *p.offset(1 as libc::c_int as isize) as libc::c_int;
        let mut cont = 1 as libc::c_int;
        loop {
            s = s.offset(1);
            if !(s < (*ms).src_end) {
                break;
            }
            if *s as libc::c_int == e {
                cont -= 1;
                if cont == 0 as libc::c_int {
                    return s.offset(1 as libc::c_int as isize);
                }
            } else if *s as libc::c_int == b {
                cont += 1;
            }
        }
    }
    return NULL as *const libc::c_char;
}
unsafe extern "C" fn max_expand(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut ep: *const libc::c_char,
) -> *const libc::c_char {
    let mut i = 0 as libc::c_int as ptrdiff_t;
    while singlematch(ms, s.offset(i as isize), p, ep) != 0 {
        i += 1;
    }
    while i >= 0 as libc::c_int as libc::c_long {
        let mut res = match_0(
            ms,
            s.offset(i as isize),
            ep.offset(1 as libc::c_int as isize),
        );
        if !res.is_null() {
            return res;
        }
        i -= 1;
    }
    return NULL as *const libc::c_char;
}
unsafe extern "C" fn min_expand(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut ep: *const libc::c_char,
) -> *const libc::c_char {
    loop {
        let mut res = match_0(ms, s, ep.offset(1 as libc::c_int as isize));
        if !res.is_null() {
            return res
        } else {
            if singlematch(ms, s, p, ep) != 0 {
                s = s.offset(1);
            } else {
                return NULL as *const libc::c_char
            }
        }
    };
}
unsafe extern "C" fn start_capture(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut what: libc::c_int,
) -> *const libc::c_char {
    let mut res = 0 as *const libc::c_char;
    let mut level = (*ms).level as libc::c_int;
    if level >= LUA_MAXCAPTURES {
        luaL_error((*ms).L, b"too many captures\0" as *const u8 as *const libc::c_char);
    }
    (*ms).capture[level as usize].init = s;
    (*ms).capture[level as usize].len = what as ptrdiff_t;
    (*ms).level = (level + 1 as libc::c_int) as libc::c_uchar;
    res = match_0(ms, s, p);
    if res.is_null() {
        (*ms).level = ((*ms).level).wrapping_sub(1);
    }
    return res;
}
unsafe extern "C" fn end_capture(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    let mut l = capture_to_close(ms);
    let mut res = 0 as *const libc::c_char;
    (*ms)
        .capture[l as usize]
        .len = s.offset_from((*ms).capture[l as usize].init) as libc::c_long;
    res = match_0(ms, s, p);
    if res.is_null() {
        (*ms).capture[l as usize].len = CAP_UNFINISHED as ptrdiff_t;
    }
    return res;
}
unsafe extern "C" fn match_capture(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut l: libc::c_int,
) -> *const libc::c_char {
    let mut len: size_t = 0;
    l = check_capture(ms, l);
    len = (*ms).capture[l as usize].len as size_t;
    if ((*ms).src_end).offset_from(s) as libc::c_long as size_t >= len
        && memcmp(
            (*ms).capture[l as usize].init as *const libc::c_void,
            s as *const libc::c_void,
            len,
        ) == 0 as libc::c_int
    {
        return s.offset(len as isize)
    } else {
        return NULL as *const libc::c_char
    };
}
unsafe extern "C" fn match_0(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    let mut ep_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block: u64;
    let fresh336 = (*ms).matchdepth;
    (*ms).matchdepth = (*ms).matchdepth - 1;
    if fresh336 == 0 as libc::c_int {
        luaL_error(
            (*ms).L,
            b"pattern too complex\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        if !(p != (*ms).p_end) {
            current_block = 6476622998065200121;
            break;
        }
        match *p as libc::c_int {
            40 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == ')' as i32 {
                    s = start_capture(
                        ms,
                        s,
                        p.offset(2 as libc::c_int as isize),
                        CAP_POSITION,
                    );
                } else {
                    s = start_capture(
                        ms,
                        s,
                        p.offset(1 as libc::c_int as isize),
                        CAP_UNFINISHED,
                    );
                }
                current_block = 6476622998065200121;
                break;
            }
            41 => {
                s = end_capture(ms, s, p.offset(1 as libc::c_int as isize));
                current_block = 6476622998065200121;
                break;
            }
            36 => {
                if !(p.offset(1 as libc::c_int as isize) != (*ms).p_end) {
                    s = if s == (*ms).src_end { s } else { NULL as *const libc::c_char };
                    current_block = 6476622998065200121;
                    break;
                }
            }
            L_ESC => {
                match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                    98 => {
                        current_block = 17965632435239708295;
                        match current_block {
                            17965632435239708295 => {
                                s = matchbalance(
                                    ms,
                                    s,
                                    p.offset(2 as libc::c_int as isize),
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(4 as libc::c_int as isize);
                                continue;
                            }
                            8236137900636309791 => {
                                let mut ep = 0 as *const libc::c_char;
                                let mut previous: libc::c_char = 0;
                                p = p.offset(2 as libc::c_int as isize);
                                if *p as libc::c_int != '[' as i32 {
                                    luaL_error(
                                        (*ms).L,
                                        b"missing '[' after '%%f' in pattern\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                ep = classend(ms, p);
                                previous = (if s == (*ms).src_init {
                                    '\0' as i32
                                } else {
                                    *s.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                }) as libc::c_char;
                                if matchbracketclass(
                                    uchar!(previous),
                                    p,
                                    ep.offset(-(1 as libc::c_int as isize)),
                                ) == 0
                                    && matchbracketclass(
                                        uchar!(* s),
                                        p,
                                        ep.offset(-(1 as libc::c_int as isize)),
                                    ) != 0
                                {
                                    p = ep;
                                    continue;
                                } else {
                                    s = NULL as *const libc::c_char;
                                    current_block = 6476622998065200121;
                                    break;
                                }
                            }
                            _ => {
                                s = match_capture(
                                    ms,
                                    s,
                                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int,
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(2 as libc::c_int as isize);
                                continue;
                            }
                        }
                    }
                    102 => {
                        current_block = 8236137900636309791;
                        match current_block {
                            17965632435239708295 => {
                                s = matchbalance(
                                    ms,
                                    s,
                                    p.offset(2 as libc::c_int as isize),
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(4 as libc::c_int as isize);
                                continue;
                            }
                            8236137900636309791 => {
                                let mut ep = 0 as *const libc::c_char;
                                let mut previous: libc::c_char = 0;
                                p = p.offset(2 as libc::c_int as isize);
                                if *p as libc::c_int != '[' as i32 {
                                    luaL_error(
                                        (*ms).L,
                                        b"missing '[' after '%%f' in pattern\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                ep = classend(ms, p);
                                previous = (if s == (*ms).src_init {
                                    '\0' as i32
                                } else {
                                    *s.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                }) as libc::c_char;
                                if matchbracketclass(
                                    uchar!(previous),
                                    p,
                                    ep.offset(-(1 as libc::c_int as isize)),
                                ) == 0
                                    && matchbracketclass(
                                        uchar!(* s),
                                        p,
                                        ep.offset(-(1 as libc::c_int as isize)),
                                    ) != 0
                                {
                                    p = ep;
                                    continue;
                                } else {
                                    s = NULL as *const libc::c_char;
                                    current_block = 6476622998065200121;
                                    break;
                                }
                            }
                            _ => {
                                s = match_capture(
                                    ms,
                                    s,
                                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int,
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(2 as libc::c_int as isize);
                                continue;
                            }
                        }
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 14576567515993809846;
                        match current_block {
                            17965632435239708295 => {
                                s = matchbalance(
                                    ms,
                                    s,
                                    p.offset(2 as libc::c_int as isize),
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(4 as libc::c_int as isize);
                                continue;
                            }
                            8236137900636309791 => {
                                let mut ep = 0 as *const libc::c_char;
                                let mut previous: libc::c_char = 0;
                                p = p.offset(2 as libc::c_int as isize);
                                if *p as libc::c_int != '[' as i32 {
                                    luaL_error(
                                        (*ms).L,
                                        b"missing '[' after '%%f' in pattern\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                ep = classend(ms, p);
                                previous = (if s == (*ms).src_init {
                                    '\0' as i32
                                } else {
                                    *s.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                }) as libc::c_char;
                                if matchbracketclass(
                                    uchar!(previous),
                                    p,
                                    ep.offset(-(1 as libc::c_int as isize)),
                                ) == 0
                                    && matchbracketclass(
                                        uchar!(* s),
                                        p,
                                        ep.offset(-(1 as libc::c_int as isize)),
                                    ) != 0
                                {
                                    p = ep;
                                    continue;
                                } else {
                                    s = NULL as *const libc::c_char;
                                    current_block = 6476622998065200121;
                                    break;
                                }
                            }
                            _ => {
                                s = match_capture(
                                    ms,
                                    s,
                                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int,
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(2 as libc::c_int as isize);
                                continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        ep_0 = classend(ms, p);
        if singlematch(ms, s, p, ep_0) == 0 {
            if *ep_0 as libc::c_int == '*' as i32 || *ep_0 as libc::c_int == '?' as i32
                || *ep_0 as libc::c_int == '-' as i32
            {
                p = ep_0.offset(1 as libc::c_int as isize);
            } else {
                s = NULL as *const libc::c_char;
                current_block = 6476622998065200121;
                break;
            }
        } else {
            match *ep_0 as libc::c_int {
                63 => {
                    let mut res = 0 as *const libc::c_char;
                    res = match_0(
                        ms,
                        s.offset(1 as libc::c_int as isize),
                        ep_0.offset(1 as libc::c_int as isize),
                    );
                    if !res.is_null() {
                        s = res;
                        current_block = 6476622998065200121;
                        break;
                    } else {
                        p = ep_0.offset(1 as libc::c_int as isize);
                    }
                }
                43 => {
                    s = s.offset(1);
                    current_block = 13243938374068099765;
                    break;
                }
                42 => {
                    current_block = 13243938374068099765;
                    break;
                }
                45 => {
                    s = min_expand(ms, s, p, ep_0);
                    current_block = 6476622998065200121;
                    break;
                }
                _ => {
                    s = s.offset(1);
                    p = ep_0;
                }
            }
        }
    }
    match current_block {
        13243938374068099765 => {
            s = max_expand(ms, s, p, ep_0);
        }
        _ => {}
    }
    (*ms).matchdepth += 1;
    return s;
}
unsafe extern "C" fn lmemfind(
    mut s1: *const libc::c_char,
    mut l1: size_t,
    mut s2: *const libc::c_char,
    mut l2: size_t,
) -> *const libc::c_char {
    if l2 == 0 as libc::c_int as libc::c_ulong {
        return s1
    } else if l2 > l1 {
        return NULL as *const libc::c_char
    } else {
        let mut init = 0 as *const libc::c_char;
        l2 = l2.wrapping_sub(1);
        l1 = l1.wrapping_sub(l2);
        while l1 > 0 as libc::c_int as libc::c_ulong
            && {
                init = memchr(s1 as *const libc::c_void, *s2 as libc::c_int, l1)
                    as *const libc::c_char;
                !init.is_null()
            }
        {
            init = init.offset(1);
            if memcmp(
                init as *const libc::c_void,
                s2.offset(1 as libc::c_int as isize) as *const libc::c_void,
                l2,
            ) == 0 as libc::c_int
            {
                return init.offset(-(1 as libc::c_int as isize))
            } else {
                l1 = (l1 as libc::c_ulong)
                    .wrapping_sub(init.offset_from(s1) as libc::c_long as libc::c_ulong)
                    as size_t as size_t;
                s1 = init;
            }
        }
        return NULL as *const libc::c_char;
    };
}
unsafe extern "C" fn push_onecapture(
    mut ms: *mut MatchState,
    mut i: libc::c_int,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
) {
    if i >= (*ms).level as libc::c_int {
        if i == 0 as libc::c_int {
            lua_pushlstring((*ms).L, s, e.offset_from(s) as libc::c_long as size_t);
        } else {
            luaL_error(
                (*ms).L,
                b"invalid capture index %%%d\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
            );
        }
    } else {
        let mut l = (*ms).capture[i as usize].len;
        if l == CAP_UNFINISHED as libc::c_long {
            luaL_error(
                (*ms).L,
                b"unfinished capture\0" as *const u8 as *const libc::c_char,
            );
        }
        if l == CAP_POSITION as libc::c_long {
            lua_pushinteger(
                (*ms).L,
                (((*ms).capture[i as usize].init).offset_from((*ms).src_init)
                    as libc::c_long + 1 as libc::c_int as libc::c_long) as lua_Integer,
            );
        } else {
            lua_pushlstring((*ms).L, (*ms).capture[i as usize].init, l as size_t);
        }
    };
}
unsafe extern "C" fn push_captures(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nlevels = if (*ms).level as libc::c_int == 0 as libc::c_int && !s.is_null() {
        1 as libc::c_int
    } else {
        (*ms).level as libc::c_int
    };
    luaL_checkstack(
        (*ms).L,
        nlevels,
        b"too many captures\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nlevels {
        push_onecapture(ms, i, s, e);
        i += 1;
    }
    return nlevels;
}
unsafe extern "C" fn nospecials(
    mut p: *const libc::c_char,
    mut l: size_t,
) -> libc::c_int {
    let mut upto = 0 as libc::c_int as size_t;
    loop {
        if !(strpbrk(p.offset(upto as isize), SPECIALS.as_ptr())).is_null() {
            return 0 as libc::c_int;
        }
        upto = (upto as libc::c_ulong)
            .wrapping_add(
                (strlen(p.offset(upto as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        if !(upto <= l) {
            break;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn prepstate(
    mut ms: *mut MatchState,
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut ls: size_t,
    mut p: *const libc::c_char,
    mut lp: size_t,
) {
    (*ms).L = L;
    (*ms).matchdepth = MAXCCALLS;
    (*ms).src_init = s;
    (*ms).src_end = s.offset(ls as isize);
    (*ms).p_end = p.offset(lp as isize);
}
unsafe extern "C" fn reprepstate(mut ms: *mut MatchState) {
    (*ms).level = 0 as libc::c_int as libc::c_uchar;
}
unsafe extern "C" fn str_find_aux(
    mut L: *mut lua_State,
    mut find: libc::c_int,
) -> libc::c_int {
    let mut ls: size_t = 0;
    let mut lp: size_t = 0;
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut ls);
    let mut p = luaL_checklstring(L, 2 as libc::c_int, &mut lp);
    let mut init = posrelat(
        luaL_optinteger(L, 3 as libc::c_int, 1 as libc::c_int as lua_Integer),
        ls,
    );
    if init < 1 as libc::c_int as libc::c_longlong {
        init = 1 as libc::c_int as lua_Integer;
    } else if init > ls as lua_Integer + 1 as libc::c_int as libc::c_longlong {
        lua_pushnil(L);
        return 1 as libc::c_int;
    }
    if find != 0 && (lua_toboolean(L, 4 as libc::c_int) != 0 || nospecials(p, lp) != 0) {
        let mut s2 = lmemfind(
            s.offset(init as isize).offset(-(1 as libc::c_int as isize)),
            ls
                .wrapping_sub(init as size_t)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
            p,
            lp,
        );
        if !s2.is_null() {
            lua_pushinteger(
                L,
                (s2.offset_from(s) as libc::c_long + 1 as libc::c_int as libc::c_long)
                    as lua_Integer,
            );
            lua_pushinteger(
                L,
                (s2.offset_from(s) as libc::c_long as libc::c_ulong).wrapping_add(lp)
                    as lua_Integer,
            );
            return 2 as libc::c_int;
        }
    } else {
        let mut ms = MatchState {
            src_init: 0 as *const libc::c_char,
            src_end: 0 as *const libc::c_char,
            p_end: 0 as *const libc::c_char,
            L: 0 as *mut lua_State,
            matchdepth: 0,
            level: 0,
            capture: [C2RustUnnamed_12 {
                init: 0 as *const libc::c_char,
                len: 0,
            }; 32],
        };
        let mut s1 = s.offset(init as isize).offset(-(1 as libc::c_int as isize));
        let mut anchor = (*p as libc::c_int == '^' as i32) as libc::c_int;
        if anchor != 0 {
            p = p.offset(1);
            lp = lp.wrapping_sub(1);
        }
        prepstate(&mut ms, L, s, ls, p, lp);
        loop {
            let mut res = 0 as *const libc::c_char;
            reprepstate(&mut ms);
            res = match_0(&mut ms, s1, p);
            if !res.is_null() {
                if find != 0 {
                    lua_pushinteger(
                        L,
                        (s1.offset_from(s) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as lua_Integer,
                    );
                    lua_pushinteger(
                        L,
                        res.offset_from(s) as libc::c_long as lua_Integer,
                    );
                    return push_captures(
                        &mut ms,
                        NULL as *const libc::c_char,
                        0 as *const libc::c_char,
                    ) + 2 as libc::c_int;
                } else {
                    return push_captures(&mut ms, s1, res)
                }
            }
            let fresh337 = s1;
            s1 = s1.offset(1);
            if !(fresh337 < ms.src_end && anchor == 0) {
                break;
            }
        }
    }
    lua_pushnil(L);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_find(mut L: *mut lua_State) -> libc::c_int {
    return str_find_aux(L, 1 as libc::c_int);
}
unsafe extern "C" fn str_match(mut L: *mut lua_State) -> libc::c_int {
    return str_find_aux(L, 0 as libc::c_int);
}
unsafe extern "C" fn gmatch_aux(mut L: *mut lua_State) -> libc::c_int {
    let mut gm = lua_touserdata(L, lua_upvalueindex!(3)) as *mut GMatchState;
    let mut src = 0 as *const libc::c_char;
    (*gm).ms.L = L;
    src = (*gm).src;
    while src <= (*gm).ms.src_end {
        let mut e = 0 as *const libc::c_char;
        reprepstate(&mut (*gm).ms);
        e = match_0(&mut (*gm).ms, src, (*gm).p);
        if !e.is_null() && e != (*gm).lastmatch {
            (*gm).lastmatch = e;
            (*gm).src = (*gm).lastmatch;
            return push_captures(&mut (*gm).ms, src, e);
        }
        src = src.offset(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gmatch(mut L: *mut lua_State) -> libc::c_int {
    let mut ls: size_t = 0;
    let mut lp: size_t = 0;
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut ls);
    let mut p = luaL_checklstring(L, 2 as libc::c_int, &mut lp);
    let mut gm = 0 as *mut GMatchState;
    lua_settop(L, 2 as libc::c_int);
    gm = lua_newuserdata(L, ::core::mem::size_of::<GMatchState>() as libc::c_ulong)
        as *mut GMatchState;
    prepstate(&mut (*gm).ms, L, s, ls, p, lp);
    (*gm).src = s;
    (*gm).p = p;
    (*gm).lastmatch = NULL as *const libc::c_char;
    lua_pushcclosure(
        L,
        Some(gmatch_aux as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        3 as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn add_s(
    mut ms: *mut MatchState,
    mut b: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
) {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut L = (*ms).L;
    let mut news = lua_tolstring(L, 3 as libc::c_int, &mut l);
    i = 0 as libc::c_int as size_t;
    while i < l {
        if !(*news.offset(i as isize) as libc::c_int != L_ESC) {
            i = i.wrapping_add(1);
            if *(*__ctype_b_loc())
                .offset(
                    *news.offset(i as isize) as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                if *news.offset(i as isize) as libc::c_int != L_ESC {
                    luaL_error(
                        L,
                        b"invalid use of '%c' in replacement string\0" as *const u8
                            as *const libc::c_char,
                        L_ESC,
                    );
                }
            } else if *news.offset(i as isize) as libc::c_int == '0' as i32 {
                luaL_addlstring(b, s, e.offset_from(s) as libc::c_long as size_t);
            } else {
                push_onecapture(
                    ms,
                    *news.offset(i as isize) as libc::c_int - '1' as i32,
                    s,
                    e,
                );
                luaL_tolstring(L, -(1 as libc::c_int), NULL as *mut size_t);
                lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
                luaL_addvalue(b);
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn add_value(
    mut ms: *mut MatchState,
    mut b: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
    mut tr: libc::c_int,
) {
    let mut L = (*ms).L;
    match tr {
        LUA_TFUNCTION => {
            let mut n: libc::c_int = 0;
            lua_pushvalue(L, 3 as libc::c_int);
            n = push_captures(ms, s, e);
            lua_call!(
                L, n, 1
            )(
                L,
                lua_call!(L, n, 1),
                lua_call!(L, n, 1),
                lua_call!(L, n, 1),
                ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_KFunction,
                >(NULL as libc::intptr_t),
            );
        }
        LUA_TTABLE => {
            push_onecapture(ms, 0 as libc::c_int, s, e);
            lua_gettable(L, 3 as libc::c_int);
        }
        _ => {
            add_s(ms, b, s, e);
            return;
        }
    }
    if lua_toboolean(L, -(1 as libc::c_int)) == 0 {
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        lua_pushlstring(L, s, e.offset_from(s) as libc::c_long as size_t);
    } else if lua_isstring(L, -(1 as libc::c_int)) == 0 {
        luaL_error(
            L,
            b"invalid replacement value (a %s)\0" as *const u8 as *const libc::c_char,
            luaL_typename!(L, - 1),
        );
    }
    luaL_addvalue(b);
}
unsafe extern "C" fn str_gsub(mut L: *mut lua_State) -> libc::c_int {
    let mut srcl: size_t = 0;
    let mut lp: size_t = 0;
    let mut src = luaL_checklstring(L, 1 as libc::c_int, &mut srcl);
    let mut p = luaL_checklstring(L, 2 as libc::c_int, &mut lp);
    let mut lastmatch = NULL as *const libc::c_char;
    let mut tr = lua_type(L, 3 as libc::c_int);
    let mut max_s = luaL_optinteger(
        L,
        4 as libc::c_int,
        srcl.wrapping_add(1 as libc::c_int as libc::c_ulong) as lua_Integer,
    );
    let mut anchor = (*p as libc::c_int == '^' as i32) as libc::c_int;
    let mut n = 0 as libc::c_int as lua_Integer;
    let mut ms = MatchState {
        src_init: 0 as *const libc::c_char,
        src_end: 0 as *const libc::c_char,
        p_end: 0 as *const libc::c_char,
        L: 0 as *mut lua_State,
        matchdepth: 0,
        level: 0,
        capture: [C2RustUnnamed_12 {
            init: 0 as *const libc::c_char,
            len: 0,
        }; 32],
    };
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    if anchor != 0 {
        p = p.offset(1);
        lp = lp.wrapping_sub(1);
    }
    prepstate(&mut ms, L, src, srcl, p, lp);
    while n < max_s {
        let mut e = 0 as *const libc::c_char;
        reprepstate(&mut ms);
        e = match_0(&mut ms, src, p);
        if !e.is_null() && e != lastmatch {
            n += 1;
            add_value(&mut ms, &mut b, src, e, tr);
            lastmatch = e;
            src = lastmatch;
        } else if !(src < ms.src_end) {
            break;
        }
        if anchor != 0 {
            break;
        }
    }
    luaL_addlstring(
        &mut b,
        src,
        (ms.src_end).offset_from(src) as libc::c_long as size_t,
    );
    luaL_pushresult(&mut b);
    lua_pushinteger(L, n);
    return 2 as libc::c_int;
}
pub const MAX_ITEM: libc::c_int = 120 as libc::c_int + 308 as libc::c_int;
pub const FLAGS: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"-+ #0\0")
};
unsafe extern "C" fn addquoted(
    mut b: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut len: size_t,
) {
    loop {
        let fresh338 = len;
        len = len.wrapping_sub(1);
        if !(fresh338 != 0) {
            break;
        }
        if !(*s as libc::c_int == '"' as i32 || *s as libc::c_int == '\\' as i32
            || *s as libc::c_int == '\n' as i32)
        {
            if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                let mut buff: [libc::c_char; 10] = [0; 10];
                if *(*__ctype_b_loc())
                    .offset(
                        *s.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    snprintf(
                        buff.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                        b"\\%d\0" as *const u8 as *const libc::c_char,
                        *s as libc::c_uchar as libc::c_int,
                    );
                } else {
                    snprintf(
                        buff.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                        b"\\%03d\0" as *const u8 as *const libc::c_char,
                        *s as libc::c_uchar as libc::c_int,
                    );
                }
                luaL_addstring(b, buff.as_mut_ptr());
            }
        }
        s = s.offset(1);
    };
}
unsafe extern "C" fn checkdp(mut buff: *mut libc::c_char, mut nb: libc::c_int) {
    if (memchr(buff as *const libc::c_void, '.' as i32, nb as libc::c_ulong)).is_null() {
        let mut point = lua_getlocaledecpoint!();
        let mut ppoint = memchr(
            buff as *const libc::c_void,
            point as libc::c_int,
            nb as libc::c_ulong,
        ) as *mut libc::c_char;
        if !ppoint.is_null() {
            *ppoint = '.' as i32 as libc::c_char;
        }
    }
}
unsafe extern "C" fn addliteral(
    mut L: *mut lua_State,
    mut b: *mut luaL_Buffer,
    mut arg: libc::c_int,
) {
    match lua_type(L, arg) {
        LUA_TSTRING => {
            let mut len: size_t = 0;
            let mut s = lua_tolstring(L, arg, &mut len);
            addquoted(b, s, len);
        }
        LUA_TNUMBER => {
            let mut buff = luaL_prepbuffsize(b, MAX_ITEM as size_t);
            let mut nb: libc::c_int = 0;
            if lua_isinteger(L, arg) == 0 {
                let mut n = lua_tonumber!(L, arg);
                nb = lua_number2strx!(L, buff, MAX_ITEM, "%" LUA_NUMBER_FRMLEN "a", n);
                checkdp(buff, nb);
            } else {
                let mut n_0 = lua_tointeger!(L, arg);
                let mut format = if n_0 == LUA_MININTEGER {
                    b"0x%llx\0" as *const u8 as *const libc::c_char
                } else {
                    LUA_INTEGER_FMT.as_ptr()
                };
                nb = l_sprintf!(buff, MAX_ITEM, format, (LUAI_UACINT) n);
            }
            let ref mut fresh339 = luaL_addsize!(b, nb);
            *fresh339 = (*fresh339 as libc::c_ulong).wrapping_add(luaL_addsize!(b, nb))
                as size_t as size_t;
        }
        LUA_TNIL | LUA_TBOOLEAN => {
            luaL_tolstring(L, arg, NULL as *mut size_t);
            luaL_addvalue(b);
        }
        _ => {
            luaL_argerror(
                L,
                arg,
                b"value has no literal form\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn scanformat(
    mut L: *mut lua_State,
    mut strfrmt: *const libc::c_char,
    mut form: *mut libc::c_char,
) -> *const libc::c_char {
    let mut p = strfrmt;
    while *p as libc::c_int != '\0' as i32
        && !(strchr(FLAGS.as_ptr(), *p as libc::c_int)).is_null()
    {
        p = p.offset(1);
    }
    if p.offset_from(strfrmt) as libc::c_long as size_t
        >= (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
    {
        luaL_error(
            L,
            b"invalid format (repeated flags)\0" as *const u8 as *const libc::c_char,
        );
    }
    if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        p = p.offset(1);
    }
    if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        p = p.offset(1);
    }
    if *p as libc::c_int == '.' as i32 {
        p = p.offset(1);
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            p = p.offset(1);
        }
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            p = p.offset(1);
        }
    }
    if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        luaL_error(
            L,
            b"invalid format (width or precision too long)\0" as *const u8
                as *const libc::c_char,
        );
    }
    let fresh340 = form;
    form = form.offset(1);
    *fresh340 = '%' as i32 as libc::c_char;
    memcpy(
        form as *mut libc::c_void,
        strfrmt as *const libc::c_void,
        ((p.offset_from(strfrmt) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    form = form
        .offset(
            (p.offset_from(strfrmt) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as isize,
        );
    *form = '\0' as i32 as libc::c_char;
    return p;
}
unsafe extern "C" fn addlenmod(
    mut form: *mut libc::c_char,
    mut lenmod: *const libc::c_char,
) {
    let mut l = strlen(form);
    let mut lm = strlen(lenmod);
    let mut spec = *form
        .offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    strcpy(form.offset(l as isize).offset(-(1 as libc::c_int as isize)), lenmod);
    *form
        .offset(
            l.wrapping_add(lm).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = spec;
    *form.offset(l.wrapping_add(lm) as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn str_format(mut L: *mut lua_State) -> libc::c_int {
    let mut top = lua_gettop(L);
    let mut arg = 1 as libc::c_int;
    let mut sfl: size_t = 0;
    let mut strfrmt = luaL_checklstring(L, arg, &mut sfl);
    let mut strfrmt_end = strfrmt.offset(sfl as isize);
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    while strfrmt < strfrmt_end {
        if !(*strfrmt as libc::c_int != L_ESC) {
            strfrmt = strfrmt.offset(1);
            if !(*strfrmt as libc::c_int == L_ESC) {
                let mut form: [libc::c_char; 32] = [0; 32];
                let mut buff = luaL_prepbuffsize(&mut b, MAX_ITEM as size_t);
                let mut nb = 0 as libc::c_int;
                arg += 1;
                if arg > top {
                    luaL_argerror(
                        L,
                        arg,
                        b"no value\0" as *const u8 as *const libc::c_char,
                    );
                }
                strfrmt = scanformat(L, strfrmt, form.as_mut_ptr());
                let fresh341 = strfrmt;
                strfrmt = strfrmt.offset(1);
                match *fresh341 as libc::c_int {
                    99 => {
                        nb = snprintf(
                            buff,
                            (120 as libc::c_int + 308 as libc::c_int) as libc::c_ulong,
                            form.as_mut_ptr(),
                            luaL_checkinteger(L, arg) as libc::c_int,
                        );
                    }
                    100 | 105 | 111 | 117 | 120 | 88 => {
                        let mut n = luaL_checkinteger(L, arg);
                        addlenmod(form.as_mut_ptr(), LUA_INTEGER_FRMLEN.as_ptr());
                        nb = l_sprintf!(buff, MAX_ITEM, form, (LUAI_UACINT) n);
                    }
                    97 | 65 => {
                        addlenmod(form.as_mut_ptr(), LUA_NUMBER_FRMLEN.as_ptr());
                        nb = snprintf(
                            buff,
                            (120 as libc::c_int + 308 as libc::c_int) as libc::c_ulong,
                            form.as_mut_ptr(),
                            luaL_checknumber(L, arg),
                        );
                    }
                    101 | 69 | 102 | 103 | 71 => {
                        let mut n_0 = luaL_checknumber(L, arg);
                        addlenmod(form.as_mut_ptr(), LUA_NUMBER_FRMLEN.as_ptr());
                        nb = l_sprintf!(buff, MAX_ITEM, form, (LUAI_UACNUMBER) n);
                    }
                    113 => {
                        addliteral(L, &mut b, arg);
                    }
                    115 => {
                        let mut l: size_t = 0;
                        let mut s = luaL_tolstring(L, arg, &mut l);
                        if form[2 as libc::c_int as usize] as libc::c_int == '\0' as i32
                        {
                            luaL_addvalue(&mut b);
                        } else if (strchr(form.as_mut_ptr(), '.' as i32)).is_null()
                            && l >= 100 as libc::c_int as libc::c_ulong
                        {
                            luaL_addvalue(&mut b);
                        } else {
                            nb = l_sprintf!(buff, MAX_ITEM, form, s);
                            lua_pop!(L, 1)(L, lua_pop!(L, 1));
                        }
                    }
                    _ => {
                        return luaL_error(
                            L,
                            b"invalid option '%%%c' to 'format'\0" as *const u8
                                as *const libc::c_char,
                            *strfrmt.offset(-(1 as libc::c_int as isize)) as libc::c_int,
                        );
                    }
                }
                let ref mut fresh342 = luaL_addsize!(& b, nb);
                *fresh342 = (*fresh342 as libc::c_ulong)
                    .wrapping_add(luaL_addsize!(& b, nb)) as size_t as size_t;
            }
        }
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
pub const MAXINTSIZE: libc::c_int = 16 as libc::c_int;
pub const NB: libc::c_int = CHAR_BIT;
pub const MC: libc::c_int = ((1 as libc::c_int) << NB) - 1 as libc::c_int;
pub const SZINT: libc::c_ulong = ::core::mem::size_of::<lua_Integer>() as libc::c_ulong;
static mut nativeendian: C2RustUnnamed_11 = C2RustUnnamed_11 {
    dummy: 1 as libc::c_int,
};
pub const MAXALIGN: libc::c_ulong = 8 as libc::c_ulong;
unsafe extern "C" fn digit(mut c: libc::c_int) -> libc::c_int {
    return ('0' as i32 <= c && c <= '9' as i32) as libc::c_int;
}
unsafe extern "C" fn getnum(
    mut fmt: *mut *const libc::c_char,
    mut df: libc::c_int,
) -> libc::c_int {
    if digit(**fmt as libc::c_int) == 0 {
        return df
    } else {
        let mut a = 0 as libc::c_int;
        loop {
            let fresh343 = *fmt;
            *fmt = (*fmt).offset(1);
            a = a * 10 as libc::c_int + (*fresh343 as libc::c_int - '0' as i32);
            if !(digit(**fmt as libc::c_int) != 0
                && a
                    <= ((if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                        < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        MAX_SIZET_0
                    } else {
                        2147483647 as libc::c_int as size_t
                    }) as libc::c_int - 9 as libc::c_int) / 10 as libc::c_int)
            {
                break;
            }
        }
        return a;
    };
}
unsafe extern "C" fn getnumlimit(
    mut h: *mut Header,
    mut fmt: *mut *const libc::c_char,
    mut df: libc::c_int,
) -> libc::c_int {
    let mut sz = getnum(fmt, df);
    if sz > MAXINTSIZE || sz <= 0 as libc::c_int {
        return luaL_error(
            (*h).L,
            b"integral size (%d) out of limits [1,%d]\0" as *const u8
                as *const libc::c_char,
            sz,
            MAXINTSIZE,
        );
    }
    return sz;
}
unsafe extern "C" fn initheader(mut L: *mut lua_State, mut h: *mut Header) {
    (*h).L = L;
    (*h).islittle = nativeendian.little as libc::c_int;
    (*h).maxalign = 1 as libc::c_int;
}
unsafe extern "C" fn getoption(
    mut h: *mut Header,
    mut fmt: *mut *const libc::c_char,
    mut size: *mut libc::c_int,
) -> KOption {
    let fresh344 = *fmt;
    *fmt = (*fmt).offset(1);
    let mut opt = *fresh344 as libc::c_int;
    *size = 0 as libc::c_int;
    match opt {
        98 => {
            *size = ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                as libc::c_int;
            return Kint;
        }
        66 => {
            *size = ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                as libc::c_int;
            return Kuint;
        }
        104 => {
            *size = ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                as libc::c_int;
            return Kint;
        }
        72 => {
            *size = ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                as libc::c_int;
            return Kuint;
        }
        108 => {
            *size = ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                as libc::c_int;
            return Kint;
        }
        76 => {
            *size = ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                as libc::c_int;
            return Kuint;
        }
        106 => {
            *size = ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
                as libc::c_int;
            return Kint;
        }
        74 => {
            *size = ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
                as libc::c_int;
            return Kuint;
        }
        84 => {
            *size = ::core::mem::size_of::<size_t>() as libc::c_ulong as libc::c_int;
            return Kuint;
        }
        102 => {
            *size = ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
                as libc::c_int;
            return Kfloat;
        }
        100 => {
            *size = ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                as libc::c_int;
            return Kfloat;
        }
        110 => {
            *size = ::core::mem::size_of::<lua_Number>() as libc::c_ulong as libc::c_int;
            return Kfloat;
        }
        105 => {
            *size = getnumlimit(
                h,
                fmt,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            );
            return Kint;
        }
        73 => {
            *size = getnumlimit(
                h,
                fmt,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            );
            return Kuint;
        }
        115 => {
            *size = getnumlimit(
                h,
                fmt,
                ::core::mem::size_of::<size_t>() as libc::c_ulong as libc::c_int,
            );
            return Kstring;
        }
        99 => {
            *size = getnum(fmt, -(1 as libc::c_int));
            if *size == -(1 as libc::c_int) {
                luaL_error(
                    (*h).L,
                    b"missing size for format option 'c'\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return Kchar;
        }
        122 => return Kzstr,
        120 => {
            *size = 1 as libc::c_int;
            return Kpadding;
        }
        88 => return Kpaddalign,
        32 => {}
        60 => {
            (*h).islittle = 1 as libc::c_int;
        }
        62 => {
            (*h).islittle = 0 as libc::c_int;
        }
        61 => {
            (*h).islittle = nativeendian.little as libc::c_int;
        }
        33 => {
            (*h).maxalign = getnumlimit(h, fmt, MAXALIGN as libc::c_int);
        }
        _ => {
            luaL_error(
                (*h).L,
                b"invalid format option '%c'\0" as *const u8 as *const libc::c_char,
                opt,
            );
        }
    }
    return Knop;
}
unsafe extern "C" fn getdetails(
    mut h: *mut Header,
    mut totalsize: size_t,
    mut fmt: *mut *const libc::c_char,
    mut psize: *mut libc::c_int,
    mut ntoalign: *mut libc::c_int,
) -> KOption {
    let mut opt = getoption(h, fmt, psize);
    let mut align = *psize;
    if opt as libc::c_uint == Kpaddalign as libc::c_int as libc::c_uint {
        if **fmt as libc::c_int == '\0' as i32
            || getoption(h, fmt, &mut align) as libc::c_uint
                == Kchar as libc::c_int as libc::c_uint || align == 0 as libc::c_int
        {
            luaL_argerror(
                (*h).L,
                1 as libc::c_int,
                b"invalid next option for option 'X'\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if align <= 1 as libc::c_int
        || opt as libc::c_uint == Kchar as libc::c_int as libc::c_uint
    {
        *ntoalign = 0 as libc::c_int;
    } else {
        if align > (*h).maxalign {
            align = (*h).maxalign;
        }
        if align & align - 1 as libc::c_int != 0 as libc::c_int {
            luaL_argerror(
                (*h).L,
                1 as libc::c_int,
                b"format asks for alignment not power of 2\0" as *const u8
                    as *const libc::c_char,
            );
        }
        *ntoalign = align
            - (totalsize & (align - 1 as libc::c_int) as libc::c_ulong) as libc::c_int
            & align - 1 as libc::c_int;
    }
    return opt;
}
unsafe extern "C" fn packint(
    mut b: *mut luaL_Buffer,
    mut n: lua_Unsigned,
    mut islittle: libc::c_int,
    mut size: libc::c_int,
    mut neg: libc::c_int,
) {
    let mut buff = luaL_prepbuffsize(b, size as size_t);
    let mut i: libc::c_int = 0;
    *buff
        .offset(
            (if islittle != 0 { 0 as libc::c_int } else { size - 1 as libc::c_int })
                as isize,
        ) = (n & MC as libc::c_ulonglong) as libc::c_char;
    i = 1 as libc::c_int;
    while i < size {
        n >>= NB;
        *buff
            .offset(
                (if islittle != 0 { i } else { size - 1 as libc::c_int - i }) as isize,
            ) = (n & MC as libc::c_ulonglong) as libc::c_char;
        i += 1;
    }
    if neg != 0 && size > SZINT as libc::c_int {
        i = SZINT as libc::c_int;
        while i < size {
            *buff
                .offset(
                    (if islittle != 0 { i } else { size - 1 as libc::c_int - i })
                        as isize,
                ) = MC as libc::c_char;
            i += 1;
        }
    }
    let ref mut fresh345 = luaL_addsize!(b, size);
    *fresh345 = (*fresh345 as libc::c_ulong).wrapping_add(luaL_addsize!(b, size))
        as size_t as size_t;
}
unsafe extern "C" fn copywithendian(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: libc::c_int,
    mut islittle: libc::c_int,
) {
    if islittle == nativeendian.little as libc::c_int {
        loop {
            let fresh346 = size;
            size = size - 1;
            if !(fresh346 != 0 as libc::c_int) {
                break;
            }
            let fresh347 = src;
            src = src.offset(1);
            let fresh348 = dest;
            dest = dest.offset(1);
            ::core::ptr::write_volatile(fresh348, *fresh347);
        }
    } else {
        dest = dest.offset((size - 1 as libc::c_int) as isize);
        loop {
            let fresh349 = size;
            size = size - 1;
            if !(fresh349 != 0 as libc::c_int) {
                break;
            }
            let fresh350 = src;
            src = src.offset(1);
            let fresh351 = dest;
            dest = dest.offset(-1);
            ::core::ptr::write_volatile(fresh351, *fresh350);
        }
    };
}
unsafe extern "C" fn str_pack(mut L: *mut lua_State) -> libc::c_int {
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut h = Header {
        L: 0 as *mut lua_State,
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt = luaL_checkstring!(L, 1);
    let mut arg = 1 as libc::c_int;
    let mut totalsize = 0 as libc::c_int as size_t;
    initheader(L, &mut h);
    lua_pushnil(L);
    luaL_buffinit(L, &mut b);
    while *fmt as libc::c_int != '\0' as i32 {
        let mut size: libc::c_int = 0;
        let mut ntoalign: libc::c_int = 0;
        let mut opt = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
        totalsize = (totalsize as libc::c_ulong)
            .wrapping_add((ntoalign + size) as libc::c_ulong) as size_t as size_t;
        loop {
            let fresh352 = ntoalign;
            ntoalign = ntoalign - 1;
            if !(fresh352 > 0 as libc::c_int) {
                break;
            }
        }
        arg += 1;
        match opt as libc::c_uint {
            0 => {
                let mut n = luaL_checkinteger(L, arg);
                if size < SZINT as libc::c_int {
                    let mut lim = (1 as libc::c_int as lua_Integer)
                        << size * NB - 1 as libc::c_int;
                }
                packint(
                    &mut b,
                    n as lua_Unsigned,
                    h.islittle,
                    size,
                    (n < 0 as libc::c_int as libc::c_longlong) as libc::c_int,
                );
            }
            1 => {
                let mut n_0 = luaL_checkinteger(L, arg);
                size < SZINT as libc::c_int;
                packint(&mut b, n_0 as lua_Unsigned, h.islittle, size, 0 as libc::c_int);
            }
            2 => {
                let mut u = Ftypes { f: 0. };
                let mut buff = luaL_prepbuffsize(&mut b, size as size_t);
                let mut n_1 = luaL_checknumber(L, arg);
                if size as libc::c_ulong
                    == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
                {
                    ::core::ptr::write_volatile(
                        &mut u.f as *mut libc::c_float,
                        n_1 as libc::c_float,
                    );
                } else if size as libc::c_ulong
                    == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                {
                    ::core::ptr::write_volatile(&mut u.d as *mut libc::c_double, n_1);
                } else {
                    ::core::ptr::write_volatile(&mut u.n as *mut lua_Number, n_1);
                }
                copywithendian(
                    buff as *mut libc::c_char,
                    (u.buff).as_mut_ptr(),
                    size,
                    h.islittle,
                );
                let ref mut fresh353 = luaL_addsize!(& b, size);
                *fresh353 = (*fresh353 as libc::c_ulong)
                    .wrapping_add(luaL_addsize!(& b, size)) as size_t as size_t;
            }
            3 => {
                let mut len: size_t = 0;
                let mut s = luaL_checklstring(L, arg, &mut len);
                luaL_addlstring(&mut b, s, len);
                loop {
                    let fresh354 = len;
                    len = len.wrapping_add(1);
                    if !(fresh354 < size as size_t) {
                        break;
                    }
                }
            }
            4 => {
                let mut len_0: size_t = 0;
                let mut s_0 = luaL_checklstring(L, arg, &mut len_0);
                packint(
                    &mut b,
                    len_0 as lua_Unsigned,
                    h.islittle,
                    size,
                    0 as libc::c_int,
                );
                luaL_addlstring(&mut b, s_0, len_0);
                totalsize = (totalsize as libc::c_ulong).wrapping_add(len_0) as size_t
                    as size_t;
            }
            5 => {
                let mut len_1: size_t = 0;
                let mut s_1 = luaL_checklstring(L, arg, &mut len_1);
                luaL_addlstring(&mut b, s_1, len_1);
                totalsize = (totalsize as libc::c_ulong)
                    .wrapping_add(len_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
            }
            6 | 7 | 8 => {
                arg -= 1;
            }
            _ => {}
        }
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_packsize(mut L: *mut lua_State) -> libc::c_int {
    let mut h = Header {
        L: 0 as *mut lua_State,
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt = luaL_checkstring!(L, 1);
    let mut totalsize = 0 as libc::c_int as size_t;
    initheader(L, &mut h);
    while *fmt as libc::c_int != '\0' as i32 {
        let mut size: libc::c_int = 0;
        let mut ntoalign: libc::c_int = 0;
        let mut opt = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
        size += ntoalign;
        totalsize = (totalsize as libc::c_ulong).wrapping_add(size as libc::c_ulong)
            as size_t as size_t;
        match opt as libc::c_uint {
            4 | 5 => {
                luaL_argerror(
                    L,
                    1 as libc::c_int,
                    b"variable-length format\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
    }
    lua_pushinteger(L, totalsize as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn unpackint(
    mut L: *mut lua_State,
    mut str: *const libc::c_char,
    mut islittle: libc::c_int,
    mut size: libc::c_int,
    mut issigned: libc::c_int,
) -> lua_Integer {
    let mut res = 0 as libc::c_int as lua_Unsigned;
    let mut i: libc::c_int = 0;
    let mut limit = if size <= SZINT as libc::c_int {
        size
    } else {
        SZINT as libc::c_int
    };
    i = limit - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        res <<= NB;
        res
            |= *str
                .offset(
                    (if islittle != 0 { i } else { size - 1 as libc::c_int - i })
                        as isize,
                ) as libc::c_uchar as lua_Unsigned;
        i -= 1;
    }
    if size < SZINT as libc::c_int {
        if issigned != 0 {
            let mut mask = (1 as libc::c_int as lua_Unsigned)
                << size * NB - 1 as libc::c_int;
            res = (res ^ mask).wrapping_sub(mask);
        }
    } else if size > SZINT as libc::c_int {
        let mut mask_0 = if issigned == 0
            || res as lua_Integer >= 0 as libc::c_int as libc::c_longlong
        {
            0 as libc::c_int
        } else {
            MC
        };
        i = limit;
        while i < size {
            if *str
                .offset(
                    (if islittle != 0 { i } else { size - 1 as libc::c_int - i })
                        as isize,
                ) as libc::c_uchar as libc::c_int != mask_0
            {
                luaL_error(
                    L,
                    b"%d-byte integer does not fit into Lua Integer\0" as *const u8
                        as *const libc::c_char,
                    size,
                );
            }
            i += 1;
        }
    }
    return res as lua_Integer;
}
unsafe extern "C" fn str_unpack(mut L: *mut lua_State) -> libc::c_int {
    let mut h = Header {
        L: 0 as *mut lua_State,
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt = luaL_checkstring!(L, 1);
    let mut ld: size_t = 0;
    let mut data = luaL_checklstring(L, 2 as libc::c_int, &mut ld);
    let mut pos = (posrelat(
        luaL_optinteger(L, 3 as libc::c_int, 1 as libc::c_int as lua_Integer),
        ld,
    ) as size_t)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut n = 0 as libc::c_int;
    initheader(L, &mut h);
    while *fmt as libc::c_int != '\0' as i32 {
        let mut size: libc::c_int = 0;
        let mut ntoalign: libc::c_int = 0;
        let mut opt = getdetails(&mut h, pos, &mut fmt, &mut size, &mut ntoalign);
        if (ntoalign as size_t).wrapping_add(size as libc::c_ulong) > !pos
            || pos
                .wrapping_add(ntoalign as libc::c_ulong)
                .wrapping_add(size as libc::c_ulong) > ld
        {
            luaL_argerror(
                L,
                2 as libc::c_int,
                b"data string too short\0" as *const u8 as *const libc::c_char,
            );
        }
        pos = (pos as libc::c_ulong).wrapping_add(ntoalign as libc::c_ulong) as size_t
            as size_t;
        luaL_checkstack(
            L,
            2 as libc::c_int,
            b"too many results\0" as *const u8 as *const libc::c_char,
        );
        n += 1;
        match opt as libc::c_uint {
            0 | 1 => {
                let mut res = unpackint(
                    L,
                    data.offset(pos as isize),
                    h.islittle,
                    size,
                    (opt as libc::c_uint == Kint as libc::c_int as libc::c_uint)
                        as libc::c_int,
                );
                lua_pushinteger(L, res);
            }
            2 => {
                let mut u = Ftypes { f: 0. };
                let mut num: lua_Number = 0.;
                copywithendian(
                    (u.buff).as_mut_ptr(),
                    data.offset(pos as isize),
                    size,
                    h.islittle,
                );
                if size as libc::c_ulong
                    == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
                {
                    num = u.f as lua_Number;
                } else if size as libc::c_ulong
                    == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                {
                    num = u.d;
                } else {
                    num = u.n;
                }
                lua_pushnumber(L, num);
            }
            3 => {
                lua_pushlstring(L, data.offset(pos as isize), size as size_t);
            }
            4 => {
                let mut len = unpackint(
                    L,
                    data.offset(pos as isize),
                    h.islittle,
                    size,
                    0 as libc::c_int,
                ) as size_t;
                lua_pushlstring(L, data.offset(pos as isize).offset(size as isize), len);
                pos = (pos as libc::c_ulong).wrapping_add(len) as size_t as size_t;
            }
            5 => {
                let mut len_0 = strlen(data.offset(pos as isize)) as libc::c_int
                    as size_t;
                lua_pushlstring(L, data.offset(pos as isize), len_0);
                pos = (pos as libc::c_ulong)
                    .wrapping_add(len_0.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
            }
            7 | 6 | 8 => {
                n -= 1;
            }
            _ => {}
        }
        pos = (pos as libc::c_ulong).wrapping_add(size as libc::c_ulong) as size_t
            as size_t;
    }
    lua_pushinteger(
        L,
        pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as lua_Integer,
    );
    return n + 1 as libc::c_int;
}
static mut strlib: [luaL_Reg; 18] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"byte\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_byte as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"char\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_char as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"dump\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_dump as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"find\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_find as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"format\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_format as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gmatch\0" as *const u8 as *const libc::c_char,
                func: Some(gmatch as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gsub\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_gsub as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"len\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_len as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"lower\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_lower as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"match\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_match as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rep\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_rep as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"reverse\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_reverse as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sub\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_sub as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"upper\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_upper as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pack\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_pack as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"packsize\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_packsize as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"unpack\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_unpack as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
unsafe extern "C" fn createmetatable(mut L: *mut lua_State) {
    lua_createtable(L, 0 as libc::c_int, 1 as libc::c_int);
    lua_pushliteral!(L, "")(L, b"\0" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, -(2 as libc::c_int));
    lua_setmetatable(L, -(2 as libc::c_int));
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    lua_pushvalue(L, -(2 as libc::c_int));
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_string(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 18]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    createmetatable(L);
    return 1 as libc::c_int;
}
pub const TAB_R: libc::c_int = 1 as libc::c_int;
pub const TAB_W: libc::c_int = 2 as libc::c_int;
pub const TAB_L: libc::c_int = 4 as libc::c_int;
unsafe extern "C" fn checkfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    lua_pushstring(L, key);
    return (lua_rawget(L, -n) != LUA_TNIL) as libc::c_int;
}
unsafe extern "C" fn checktab(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut what: libc::c_int,
) {
    if lua_type(L, arg) != LUA_TTABLE {
        let mut n = 1 as libc::c_int;
        if lua_getmetatable(L, arg) != 0
            && (what & TAB_R == 0
                || {
                    n += 1;
                    checkfield(L, b"__index\0" as *const u8 as *const libc::c_char, n)
                        != 0
                })
            && (what & TAB_W == 0
                || {
                    n += 1;
                    checkfield(L, b"__newindex\0" as *const u8 as *const libc::c_char, n)
                        != 0
                })
            && (what & TAB_L == 0
                || {
                    n += 1;
                    checkfield(L, b"__len\0" as *const u8 as *const libc::c_char, n) != 0
                })
        {
            lua_pop!(L, n)(L, lua_pop!(L, n));
        } else {
            luaL_checktype(L, arg, LUA_TTABLE);
        }
    }
}
unsafe extern "C" fn tinsert(mut L: *mut lua_State) -> libc::c_int {
    let mut e = aux_getn!(L, 1, TAB_RW) + 1 as libc::c_int as libc::c_longlong;
    let mut pos: lua_Integer = 0;
    match lua_gettop(L) {
        2 => {
            pos = e;
        }
        3 => {
            let mut i: lua_Integer = 0;
            pos = luaL_checkinteger(L, 2 as libc::c_int);
            i = e;
            while i > pos {
                lua_geti(L, 1 as libc::c_int, i - 1 as libc::c_int as libc::c_longlong);
                lua_seti(L, 1 as libc::c_int, i);
                i -= 1;
            }
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments to 'insert'\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    lua_seti(L, 1 as libc::c_int, pos);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tremove(mut L: *mut lua_State) -> libc::c_int {
    let mut size = aux_getn!(L, 1, TAB_RW);
    let mut pos = luaL_optinteger(L, 2 as libc::c_int, size);
    pos != size;
    lua_geti(L, 1 as libc::c_int, pos);
    while pos < size {
        lua_geti(L, 1 as libc::c_int, pos + 1 as libc::c_int as libc::c_longlong);
        lua_seti(L, 1 as libc::c_int, pos);
        pos += 1;
    }
    lua_pushnil(L);
    lua_seti(L, 1 as libc::c_int, pos);
    return 1 as libc::c_int;
}
unsafe extern "C" fn tmove(mut L: *mut lua_State) -> libc::c_int {
    let mut f = luaL_checkinteger(L, 2 as libc::c_int);
    let mut e = luaL_checkinteger(L, 3 as libc::c_int);
    let mut t = luaL_checkinteger(L, 4 as libc::c_int);
    let mut tt = if lua_isnoneornil!(L, 5) == 0 {
        5 as libc::c_int
    } else {
        1 as libc::c_int
    };
    checktab(L, 1 as libc::c_int, TAB_R);
    checktab(L, tt, TAB_W);
    if e >= f {
        let mut n: lua_Integer = 0;
        let mut i: lua_Integer = 0;
        n = e - f + 1 as libc::c_int as libc::c_longlong;
        if t > e || t <= f
            || tt != 1 as libc::c_int
                && lua_compare(L, 1 as libc::c_int, tt, LUA_OPEQ) == 0
        {
            i = 0 as libc::c_int as lua_Integer;
            while i < n {
                lua_geti(L, 1 as libc::c_int, f + i);
                lua_seti(L, tt, t + i);
                i += 1;
            }
        } else {
            i = n - 1 as libc::c_int as libc::c_longlong;
            while i >= 0 as libc::c_int as libc::c_longlong {
                lua_geti(L, 1 as libc::c_int, f + i);
                lua_seti(L, tt, t + i);
                i -= 1;
            }
        }
    }
    lua_pushvalue(L, tt);
    return 1 as libc::c_int;
}
unsafe extern "C" fn addfield(
    mut L: *mut lua_State,
    mut b: *mut luaL_Buffer,
    mut i: lua_Integer,
) {
    lua_geti(L, 1 as libc::c_int, i);
    if lua_isstring(L, -(1 as libc::c_int)) == 0 {
        luaL_error(
            L,
            b"invalid value (%s) at index %d in table for 'concat'\0" as *const u8
                as *const libc::c_char,
            luaL_typename!(L, - 1),
            i,
        );
    }
    luaL_addvalue(b);
}
unsafe extern "C" fn tconcat(mut L: *mut lua_State) -> libc::c_int {
    let mut b = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut last = aux_getn!(L, 1, TAB_R);
    let mut lsep: size_t = 0;
    let mut sep = luaL_optlstring(
        L,
        2 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        &mut lsep,
    );
    let mut i = luaL_optinteger(L, 3 as libc::c_int, 1 as libc::c_int as lua_Integer);
    last = luaL_optinteger(L, 4 as libc::c_int, last);
    luaL_buffinit(L, &mut b);
    while i < last {
        addfield(L, &mut b, i);
        luaL_addlstring(&mut b, sep, lsep);
        i += 1;
    }
    if i == last {
        addfield(L, &mut b, i);
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
unsafe extern "C" fn pack(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n = lua_gettop(L);
    lua_createtable(L, n, 1 as libc::c_int);
    lua_insert!(L, 1)(L, lua_insert!(L, 1), lua_insert!(L, 1));
    i = n;
    while i >= 1 as libc::c_int {
        lua_seti(L, 1 as libc::c_int, i as lua_Integer);
        i -= 1;
    }
    lua_pushinteger(L, n as lua_Integer);
    lua_setfield(L, 1 as libc::c_int, b"n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn unpack(mut L: *mut lua_State) -> libc::c_int {
    let mut n: lua_Unsigned = 0;
    let mut i = luaL_optinteger(L, 2 as libc::c_int, 1 as libc::c_int as lua_Integer);
    let mut e = if lua_type(L, 3 as libc::c_int) <= 0 as libc::c_int {
        luaL_len(L, 1 as libc::c_int)
    } else {
        luaL_checkinteger(L, 3 as libc::c_int)
    };
    if i > e {
        return 0 as libc::c_int;
    }
    n = (e as lua_Unsigned).wrapping_sub(i as libc::c_ulonglong);
    if n >= INT_MAX as libc::c_uint as libc::c_ulonglong
        || {
            n = n.wrapping_add(1);
            lua_checkstack(L, n as libc::c_int) == 0
        }
    {
        return luaL_error(
            L,
            b"too many results to unpack\0" as *const u8 as *const libc::c_char,
        );
    }
    while i < e {
        lua_geti(L, 1 as libc::c_int, i);
        i += 1;
    }
    lua_geti(L, 1 as libc::c_int, e);
    return n as libc::c_int;
}
unsafe extern "C" fn l_randomizePivot() -> libc::c_uint {
    let mut c = clock();
    let mut t = time(NULL as *mut time_t);
    let mut buff: [libc::c_uint; 4] = [0; 4];
    let mut i: libc::c_uint = 0;
    let mut rnd = 0 as libc::c_int as libc::c_uint;
    memcpy(
        buff.as_mut_ptr() as *mut libc::c_void,
        &mut c as *mut clock_t as *const libc::c_void,
        sof!(c).wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    memcpy(
        buff.as_mut_ptr().offset(sof!(c) as isize) as *mut libc::c_void,
        &mut t as *mut time_t as *const libc::c_void,
        sof!(t).wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < sof!(buff) {
        rnd = rnd.wrapping_add(buff[i as usize]);
        i = i.wrapping_add(1);
    }
    return rnd;
}
pub const RANLIMIT: libc::c_uint = 100 as libc::c_uint;
unsafe extern "C" fn set2(mut L: *mut lua_State, mut i: IdxT, mut j: IdxT) {
    lua_seti(L, 1 as libc::c_int, i as lua_Integer);
    lua_seti(L, 1 as libc::c_int, j as lua_Integer);
}
unsafe extern "C" fn sort_comp(
    mut L: *mut lua_State,
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    if lua_isnil!(L, 2) != 0 {
        return lua_compare(L, a, b, LUA_OPLT)
    } else {
        let mut res: libc::c_int = 0;
        lua_pushvalue(L, 2 as libc::c_int);
        lua_pushvalue(L, a - 1 as libc::c_int);
        lua_pushvalue(L, b - 2 as libc::c_int);
        lua_call!(
            L, 2, 1
        )(
            L,
            lua_call!(L, 2, 1),
            lua_call!(L, 2, 1),
            lua_call!(L, 2, 1),
            ::core::mem::transmute::<
                libc::intptr_t,
                lua_KFunction,
            >(NULL as libc::intptr_t),
        );
        res = lua_toboolean(L, -(1 as libc::c_int));
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        return res;
    };
}
unsafe extern "C" fn partition(
    mut L: *mut lua_State,
    mut lo: IdxT,
    mut up: IdxT,
) -> IdxT {
    let mut i = lo;
    let mut j = up.wrapping_sub(1 as libc::c_int as libc::c_uint);
    loop {
        loop {
            i = i.wrapping_add(1);
            lua_geti(L, 1 as libc::c_int, i as lua_Integer);
            if !(sort_comp(L, -(1 as libc::c_int), -(2 as libc::c_int)) != 0) {
                break;
            }
            if i == up.wrapping_sub(1 as libc::c_int as libc::c_uint) {
                luaL_error(
                    L,
                    b"invalid order function for sorting\0" as *const u8
                        as *const libc::c_char,
                );
            }
            lua_pop!(L, 1)(L, lua_pop!(L, 1));
        }
        loop {
            j = j.wrapping_sub(1);
            lua_geti(L, 1 as libc::c_int, j as lua_Integer);
            if !(sort_comp(L, -(3 as libc::c_int), -(1 as libc::c_int)) != 0) {
                break;
            }
            if j < i {
                luaL_error(
                    L,
                    b"invalid order function for sorting\0" as *const u8
                        as *const libc::c_char,
                );
            }
            lua_pop!(L, 1)(L, lua_pop!(L, 1));
        }
        if j < i {
            lua_pop!(L, 1)(L, lua_pop!(L, 1));
            set2(L, up.wrapping_sub(1 as libc::c_int as libc::c_uint), i);
            return i;
        }
        set2(L, i, j);
    };
}
unsafe extern "C" fn choosePivot(
    mut lo: IdxT,
    mut up: IdxT,
    mut rnd: libc::c_uint,
) -> IdxT {
    let mut r4 = up.wrapping_sub(lo).wrapping_div(4 as libc::c_int as libc::c_uint);
    let mut p = rnd
        .wrapping_rem(r4.wrapping_mul(2 as libc::c_int as libc::c_uint))
        .wrapping_add(lo.wrapping_add(r4));
    return p;
}
unsafe extern "C" fn auxsort(
    mut L: *mut lua_State,
    mut lo: IdxT,
    mut up: IdxT,
    mut rnd: libc::c_uint,
) {
    while lo < up {
        let mut p: IdxT = 0;
        let mut n: IdxT = 0;
        lua_geti(L, 1 as libc::c_int, lo as lua_Integer);
        lua_geti(L, 1 as libc::c_int, up as lua_Integer);
        if sort_comp(L, -(1 as libc::c_int), -(2 as libc::c_int)) != 0 {
            set2(L, lo, up);
        } else {
            lua_pop!(L, 2)(L, lua_pop!(L, 2));
        }
        if up.wrapping_sub(lo) == 1 as libc::c_int as libc::c_uint {
            return;
        }
        if up.wrapping_sub(lo) < RANLIMIT || rnd == 0 as libc::c_int as libc::c_uint {
            p = lo.wrapping_add(up).wrapping_div(2 as libc::c_int as libc::c_uint);
        } else {
            p = choosePivot(lo, up, rnd);
        }
        lua_geti(L, 1 as libc::c_int, p as lua_Integer);
        lua_geti(L, 1 as libc::c_int, lo as lua_Integer);
        if sort_comp(L, -(2 as libc::c_int), -(1 as libc::c_int)) != 0 {
            set2(L, p, lo);
        } else {
            lua_pop!(L, 1)(L, lua_pop!(L, 1));
            lua_geti(L, 1 as libc::c_int, up as lua_Integer);
            if sort_comp(L, -(1 as libc::c_int), -(2 as libc::c_int)) != 0 {
                set2(L, p, up);
            } else {
                lua_pop!(L, 2)(L, lua_pop!(L, 2));
            }
        }
        if up.wrapping_sub(lo) == 2 as libc::c_int as libc::c_uint {
            return;
        }
        lua_geti(L, 1 as libc::c_int, p as lua_Integer);
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_geti(
            L,
            1 as libc::c_int,
            up.wrapping_sub(1 as libc::c_int as libc::c_uint) as lua_Integer,
        );
        set2(L, p, up.wrapping_sub(1 as libc::c_int as libc::c_uint));
        p = partition(L, lo, up);
        if p.wrapping_sub(lo) < up.wrapping_sub(p) {
            auxsort(L, lo, p.wrapping_sub(1 as libc::c_int as libc::c_uint), rnd);
            n = p.wrapping_sub(lo);
            lo = p.wrapping_add(1 as libc::c_int as libc::c_uint);
        } else {
            auxsort(L, p.wrapping_add(1 as libc::c_int as libc::c_uint), up, rnd);
            n = up.wrapping_sub(p);
            up = p.wrapping_sub(1 as libc::c_int as libc::c_uint);
        }
        if up.wrapping_sub(lo).wrapping_div(128 as libc::c_int as libc::c_uint) > n {
            rnd = l_randomizePivot();
        }
    }
}
unsafe extern "C" fn sort(mut L: *mut lua_State) -> libc::c_int {
    let mut n = aux_getn!(L, 1, TAB_RW);
    if n > 1 as libc::c_int as libc::c_longlong {
        if lua_isnoneornil!(L, 2) == 0 {
            luaL_checktype(L, 2 as libc::c_int, LUA_TFUNCTION);
        }
        lua_settop(L, 2 as libc::c_int);
        auxsort(
            L,
            1 as libc::c_int as IdxT,
            n as IdxT,
            0 as libc::c_int as libc::c_uint,
        );
    }
    return 0 as libc::c_int;
}
static mut tab_funcs: [luaL_Reg; 8] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"concat\0" as *const u8 as *const libc::c_char,
                func: Some(
                    tconcat as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"insert\0" as *const u8 as *const libc::c_char,
                func: Some(
                    tinsert as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pack\0" as *const u8 as *const libc::c_char,
                func: Some(pack as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"unpack\0" as *const u8 as *const libc::c_char,
                func: Some(unpack as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"remove\0" as *const u8 as *const libc::c_char,
                func: Some(
                    tremove as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"move\0" as *const u8 as *const libc::c_char,
                func: Some(tmove as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sort\0" as *const u8 as *const libc::c_char,
                func: Some(sort as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaopen_table(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    return 1 as libc::c_int;
}
pub const MAXUNICODE: libc::c_int = 0x10ffff as libc::c_int;
unsafe extern "C" fn u_posrelat(mut pos: lua_Integer, mut len: size_t) -> lua_Integer {
    if pos >= 0 as libc::c_int as libc::c_longlong {
        return pos
    } else if (0 as libc::c_uint as libc::c_ulong).wrapping_sub(pos as size_t) > len {
        return 0 as libc::c_int as lua_Integer
    } else {
        return len as lua_Integer + pos + 1 as libc::c_int as libc::c_longlong
    };
}
unsafe extern "C" fn utf8_decode(
    mut o: *const libc::c_char,
    mut val: *mut libc::c_int,
) -> *const libc::c_char {
    static mut limits: [libc::c_uint; 4] = [
        0xff as libc::c_int as libc::c_uint,
        0x7f as libc::c_int as libc::c_uint,
        0x7ff as libc::c_int as libc::c_uint,
        0xffff as libc::c_int as libc::c_uint,
    ];
    let mut s = o as *const libc::c_uchar;
    let mut c = *s.offset(0 as libc::c_int as isize) as libc::c_uint;
    let mut res = 0 as libc::c_int as libc::c_uint;
    if c < 0x80 as libc::c_int as libc::c_uint {
        res = c;
    } else {
        let mut count = 0 as libc::c_int;
        while c & 0x40 as libc::c_int as libc::c_uint != 0 {
            count += 1;
            let mut cc = *s.offset(count as isize) as libc::c_int;
            if cc & 0xc0 as libc::c_int != 0x80 as libc::c_int {
                return NULL as *const libc::c_char;
            }
            res = res << 6 as libc::c_int | (cc & 0x3f as libc::c_int) as libc::c_uint;
            c <<= 1 as libc::c_int;
        }
        res |= (c & 0x7f as libc::c_int as libc::c_uint) << count * 5 as libc::c_int;
        if count > 3 as libc::c_int || res > MAXUNICODE as libc::c_uint
            || res <= limits[count as usize]
        {
            return NULL as *const libc::c_char;
        }
        s = s.offset(count as isize);
    }
    if !val.is_null() {
        *val = res as libc::c_int;
    }
    return (s as *const libc::c_char).offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn utflen(mut L: *mut lua_State) -> libc::c_int {
    let mut n = 0 as libc::c_int;
    let mut len: size_t = 0;
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut posi = u_posrelat(
        luaL_optinteger(L, 2 as libc::c_int, 1 as libc::c_int as lua_Integer),
        len,
    );
    let mut posj = u_posrelat(
        luaL_optinteger(L, 3 as libc::c_int, -(1 as libc::c_int) as lua_Integer),
        len,
    );
    while posi <= posj {
        let mut s1 = utf8_decode(s.offset(posi as isize), NULL as *mut libc::c_int);
        if s1.is_null() {
            lua_pushnil(L);
            lua_pushinteger(L, posi + 1 as libc::c_int as libc::c_longlong);
            return 2 as libc::c_int;
        }
        posi = s1.offset_from(s) as libc::c_long as lua_Integer;
        n += 1;
    }
    lua_pushinteger(L, n as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn codepoint(mut L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut posi = u_posrelat(
        luaL_optinteger(L, 2 as libc::c_int, 1 as libc::c_int as lua_Integer),
        len,
    );
    let mut pose = u_posrelat(luaL_optinteger(L, 3 as libc::c_int, posi), len);
    let mut n: libc::c_int = 0;
    let mut se = 0 as *const libc::c_char;
    if posi > pose {
        return 0 as libc::c_int;
    }
    if pose - posi >= INT_MAX as libc::c_longlong {
        return luaL_error(
            L,
            b"string slice too long\0" as *const u8 as *const libc::c_char,
        );
    }
    n = (pose - posi) as libc::c_int + 1 as libc::c_int;
    luaL_checkstack(
        L,
        n,
        b"string slice too long\0" as *const u8 as *const libc::c_char,
    );
    n = 0 as libc::c_int;
    se = s.offset(pose as isize);
    s = s.offset((posi - 1 as libc::c_int as libc::c_longlong) as isize);
    while s < se {
        let mut code: libc::c_int = 0;
        s = utf8_decode(s, &mut code);
        if s.is_null() {
            return luaL_error(
                L,
                b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
            );
        }
        lua_pushinteger(L, code as lua_Integer);
        n += 1;
    }
    return n;
}
unsafe extern "C" fn pushutfchar(mut L: *mut lua_State, mut arg: libc::c_int) {
    let mut code = luaL_checkinteger(L, arg);
    lua_pushfstring(
        L,
        b"%U\0" as *const u8 as *const libc::c_char,
        code as libc::c_long,
    );
}
unsafe extern "C" fn utfchar(mut L: *mut lua_State) -> libc::c_int {
    let mut n = lua_gettop(L);
    if n == 1 as libc::c_int {
        pushutfchar(L, 1 as libc::c_int);
    } else {
        let mut i: libc::c_int = 0;
        let mut b = luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            initb: [0; 8192],
        };
        luaL_buffinit(L, &mut b);
        i = 1 as libc::c_int;
        while i <= n {
            pushutfchar(L, i);
            luaL_addvalue(&mut b);
            i += 1;
        }
        luaL_pushresult(&mut b);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn byteoffset(mut L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut n = luaL_checkinteger(L, 2 as libc::c_int);
    let mut posi = (if n >= 0 as libc::c_int as libc::c_longlong {
        1 as libc::c_int as libc::c_ulong
    } else {
        len.wrapping_add(1 as libc::c_int as libc::c_ulong)
    }) as lua_Integer;
    posi = u_posrelat(luaL_optinteger(L, 3 as libc::c_int, posi), len);
    if n == 0 as libc::c_int as libc::c_longlong {
        while posi > 0 as libc::c_int as libc::c_longlong && iscont!(s + posi) != 0 {
            posi -= 1;
        }
    } else {
        if iscont!(s + posi) != 0 {
            return luaL_error(
                L,
                b"initial position is a continuation byte\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if n < 0 as libc::c_int as libc::c_longlong {
            while n < 0 as libc::c_int as libc::c_longlong
                && posi > 0 as libc::c_int as libc::c_longlong
            {
                loop {
                    posi -= 1;
                    if !(posi > 0 as libc::c_int as libc::c_longlong
                        && iscont!(s + posi) != 0)
                    {
                        break;
                    }
                }
                n += 1;
            }
        } else {
            n -= 1;
            while n > 0 as libc::c_int as libc::c_longlong && posi < len as lua_Integer {
                loop {
                    posi += 1;
                    if !(iscont!(s + posi) != 0) {
                        break;
                    }
                }
                n -= 1;
            }
        }
    }
    if n == 0 as libc::c_int as libc::c_longlong {
        lua_pushinteger(L, posi + 1 as libc::c_int as libc::c_longlong);
    } else {
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn iter_aux(mut L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut n = lua_tointeger!(L, 2) - 1 as libc::c_int as libc::c_longlong;
    if n < 0 as libc::c_int as libc::c_longlong {
        n = 0 as libc::c_int as lua_Integer;
    } else if n < len as lua_Integer {
        n += 1;
        while iscont!(s + n) != 0 {
            n += 1;
        }
    }
    if n >= len as lua_Integer {
        return 0 as libc::c_int
    } else {
        let mut code: libc::c_int = 0;
        let mut next = utf8_decode(s.offset(n as isize), &mut code);
        if next.is_null() || iscont!(next) != 0 {
            return luaL_error(
                L,
                b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
            );
        }
        lua_pushinteger(L, n + 1 as libc::c_int as libc::c_longlong);
        lua_pushinteger(L, code as lua_Integer);
        return 2 as libc::c_int;
    };
}
unsafe extern "C" fn iter_codes(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkstring!(L, 1)(L, luaL_checkstring!(L, 1), NULL as *mut size_t);
    lua_pushcfunction!(
        L, iter_aux
    )(L, lua_pushcfunction!(L, iter_aux), lua_pushcfunction!(L, iter_aux));
    lua_pushvalue(L, 1 as libc::c_int);
    lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
    return 3 as libc::c_int;
}
pub const UTF8PATT: [libc::c_char; 15] = unsafe {
    *::core::mem::transmute::<
        &[u8; 15],
        &[libc::c_char; 15],
    >(b"[\0-\x7F\xC2-\xF4][\x80-\xBF]*\0")
};
static mut funcs: [luaL_Reg; 7] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"offset\0" as *const u8 as *const libc::c_char,
                func: Some(
                    byteoffset as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"codepoint\0" as *const u8 as *const libc::c_char,
                func: Some(
                    codepoint as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"char\0" as *const u8 as *const libc::c_char,
                func: Some(
                    utfchar as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"len\0" as *const u8 as *const libc::c_char,
                func: Some(utflen as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"codes\0" as *const u8 as *const libc::c_char,
                func: Some(
                    iter_codes as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"charpattern\0" as *const u8 as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaopen_utf8(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 7]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    lua_pushlstring(
        L,
        UTF8PATT.as_ptr(),
        (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"charpattern\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
static mut loadedlibs: [luaL_Reg; 11] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"_G\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_base as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: LUA_LOADLIBNAME.as_ptr(),
                func: Some(
                    luaopen_package
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: LUA_COLIBNAME.as_ptr(),
                func: Some(
                    luaopen_coroutine
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: LUA_TABLIBNAME.as_ptr(),
                func: Some(
                    luaopen_table as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: LUA_IOLIBNAME.as_ptr(),
                func: Some(
                    luaopen_io as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: LUA_OSLIBNAME.as_ptr(),
                func: Some(
                    luaopen_os as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: LUA_STRLIBNAME.as_ptr(),
                func: Some(
                    luaopen_string as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: LUA_MATHLIBNAME.as_ptr(),
                func: Some(
                    luaopen_math as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: LUA_UTF8LIBNAME.as_ptr(),
                func: Some(
                    luaopen_utf8 as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: LUA_DBLIBNAME.as_ptr(),
                func: Some(
                    luaopen_debug as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: NULL as *const libc::c_char,
                func: ::core::mem::transmute::<
                    libc::intptr_t,
                    lua_CFunction,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn luaL_openlibs(mut L: *mut lua_State) {
    let mut lib = 0 as *const luaL_Reg;
    lib = loadedlibs.as_ptr();
    while ((*lib).func).is_some() {
        luaL_requiref(L, (*lib).name, (*lib).func, 1 as libc::c_int);
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
        lib = lib.offset(1);
    }
}
pub const LUA_PROMPT: [libc::c_char; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"> \0")
};
pub const LUA_PROMPT2: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b">> \0")
};
pub const LUA_PROGNAME: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"lua\0")
};
pub const LUA_MAXINPUT: libc::c_int = 512 as libc::c_int;
static mut globalL: *mut lua_State = NULL as *mut lua_State;
static mut progname: *const libc::c_char = LUA_PROGNAME.as_ptr();
unsafe extern "C" fn lstop(mut L: *mut lua_State, mut ar: *mut lua_Debug) {
    lua_sethook(
        L,
        ::core::mem::transmute::<libc::intptr_t, lua_Hook>(NULL as libc::intptr_t),
        0 as libc::c_int,
        0 as libc::c_int,
    );
    luaL_error(L, b"interrupted!\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn laction(mut i: libc::c_int) {
    signal(
        i,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(SIG_DFL as libc::intptr_t),
    );
    lua_sethook(
        globalL,
        Some(lstop as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()),
        LUA_MASKCALL | LUA_MASKRET | LUA_MASKCOUNT,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn print_usage(mut badoption: *const libc::c_char) {
    *badoption.offset(1 as libc::c_int as isize) as libc::c_int == 'e' as i32
        || *badoption.offset(1 as libc::c_int as isize) as libc::c_int == 'l' as i32;
}
unsafe extern "C" fn l_message(
    mut pname: *const libc::c_char,
    mut msg: *const libc::c_char,
) {
    !pname.is_null();
}
unsafe extern "C" fn report(
    mut L: *mut lua_State,
    mut status: libc::c_int,
) -> libc::c_int {
    if status != LUA_OK {
        let mut msg = lua_tostring!(L, - 1);
        l_message(progname, msg);
        lua_pop!(L, 1)(L, lua_pop!(L, 1));
    }
    return status;
}
unsafe extern "C" fn msghandler(mut L: *mut lua_State) -> libc::c_int {
    let mut msg = lua_tostring!(L, 1);
    if msg.is_null() {
        if luaL_callmeta(
            L,
            1 as libc::c_int,
            b"__tostring\0" as *const u8 as *const libc::c_char,
        ) != 0 && lua_type(L, -(1 as libc::c_int)) == LUA_TSTRING
        {
            return 1 as libc::c_int
        } else {
            msg = lua_pushfstring(
                L,
                b"(error object is a %s value)\0" as *const u8 as *const libc::c_char,
                luaL_typename!(L, 1),
            );
        }
    }
    luaL_traceback(L, L, msg, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn docall(
    mut L: *mut lua_State,
    mut narg: libc::c_int,
    mut nres: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut base = lua_gettop(L) - narg;
    lua_pushcfunction!(
        L, msghandler
    )(L, lua_pushcfunction!(L, msghandler), lua_pushcfunction!(L, msghandler));
    lua_insert!(L, base)(L, lua_insert!(L, base), lua_insert!(L, base));
    globalL = L;
    signal(SIGINT, Some(laction as unsafe extern "C" fn(libc::c_int) -> ()));
    status = lua_pcall!(L, narg, nres, base);
    signal(
        SIGINT,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(SIG_DFL as libc::intptr_t),
    );
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return status;
}
unsafe extern "C" fn print_version() {
    fwrite(
        b"Lua 5.3.6  Copyright (C) 1994-2020 Lua.org, PUC-Rio\0" as *const u8
            as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        strlen(
            b"Lua 5.3.6  Copyright (C) 1994-2020 Lua.org, PUC-Rio\0" as *const u8
                as *const libc::c_char,
        ),
        stdout,
    );
    fwrite(
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stdout,
    );
}
unsafe extern "C" fn createargtable(
    mut L: *mut lua_State,
    mut argv: *mut *mut libc::c_char,
    mut argc: libc::c_int,
    mut script: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut narg: libc::c_int = 0;
    if script == argc {
        script = 0 as libc::c_int;
    }
    narg = argc - (script + 1 as libc::c_int);
    lua_createtable(L, narg, script + 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < argc {
        lua_pushstring(L, *argv.offset(i as isize));
        lua_rawseti(L, -(2 as libc::c_int), (i - script) as lua_Integer);
        i += 1;
    }
    lua_setglobal(L, b"arg\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn dochunk(
    mut L: *mut lua_State,
    mut status: libc::c_int,
) -> libc::c_int {
    if status == LUA_OK {
        status = docall(L, 0 as libc::c_int, 0 as libc::c_int);
    }
    return report(L, status);
}
unsafe extern "C" fn dofile(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) -> libc::c_int {
    return dochunk(L, luaL_loadfile!(L, name));
}
unsafe extern "C" fn dostring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut name: *const libc::c_char,
) -> libc::c_int {
    return dochunk(L, luaL_loadbuffer!(L, s, strlen(s), name));
}
unsafe extern "C" fn dolibrary(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    lua_getglobal(L, b"require\0" as *const u8 as *const libc::c_char);
    lua_pushstring(L, name);
    status = docall(L, 1 as libc::c_int, 1 as libc::c_int);
    if status == LUA_OK {
        lua_setglobal(L, name);
    }
    return report(L, status);
}
unsafe extern "C" fn get_prompt(
    mut L: *mut lua_State,
    mut firstline: libc::c_int,
) -> *const libc::c_char {
    let mut p = 0 as *const libc::c_char;
    lua_getglobal(
        L,
        if firstline != 0 {
            b"_PROMPT\0" as *const u8 as *const libc::c_char
        } else {
            b"_PROMPT2\0" as *const u8 as *const libc::c_char
        },
    );
    p = lua_tostring!(L, - 1);
    if p.is_null() {
        p = if firstline != 0 { LUA_PROMPT.as_ptr() } else { LUA_PROMPT2.as_ptr() };
    }
    return p;
}
pub const EOFMARK: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"<eof>\0")
};
pub const marklen: libc::c_ulong = (::core::mem::size_of::<[libc::c_char; 6]>()
    as libc::c_ulong)
    .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
unsafe extern "C" fn incomplete(
    mut L: *mut lua_State,
    mut status: libc::c_int,
) -> libc::c_int {
    if status == LUA_ERRSYNTAX {
        let mut lmsg: size_t = 0;
        let mut msg = lua_tolstring(L, -(1 as libc::c_int), &mut lmsg);
        if lmsg >= marklen
            && strcmp(
                msg.offset(lmsg as isize).offset(-(marklen as isize)),
                EOFMARK.as_ptr(),
            ) == 0 as libc::c_int
        {
            lua_pop!(L, 1)(L, lua_pop!(L, 1));
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pushline(
    mut L: *mut lua_State,
    mut firstline: libc::c_int,
) -> libc::c_int {
    let mut buffer: [libc::c_char; 512] = [0; 512];
    let mut b = buffer.as_mut_ptr();
    let mut l: size_t = 0;
    let mut prmt = get_prompt(L, firstline);
    let mut readstatus = lua_readline!(L, b, prmt);
    if readstatus == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    lua_pop!(L, 1)(L, lua_pop!(L, 1));
    l = strlen(b);
    if l > 0 as libc::c_int as libc::c_ulong
        && *b.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\n' as i32
    {
        l = l.wrapping_sub(1);
        *b.offset(l as isize) = '\0' as i32 as libc::c_char;
    }
    if firstline != 0
        && *b.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
    {
        lua_pushfstring(
            L,
            b"return %s\0" as *const u8 as *const libc::c_char,
            b.offset(1 as libc::c_int as isize),
        );
    } else {
        lua_pushlstring(L, b, l);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn addreturn(mut L: *mut lua_State) -> libc::c_int {
    let mut line = lua_tostring!(L, - 1);
    let mut retline = lua_pushfstring(
        L,
        b"return %s;\0" as *const u8 as *const libc::c_char,
        line,
    );
    let mut status = luaL_loadbuffer!(L, retline, strlen(retline), "=stdin");
    if status == LUA_OK {
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        *line.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32;
    } else {
        lua_pop!(L, 2)(L, lua_pop!(L, 2));
    }
    return status;
}
unsafe extern "C" fn multiline(mut L: *mut lua_State) -> libc::c_int {
    loop {
        let mut len: size_t = 0;
        let mut line = lua_tolstring(L, 1 as libc::c_int, &mut len);
        let mut status = luaL_loadbuffer!(L, line, len, "=stdin");
        if incomplete(L, status) == 0 || pushline(L, 0 as libc::c_int) == 0 {
            return status;
        }
        lua_pushliteral!(L, "\n")(L, b"\n\0" as *const u8 as *const libc::c_char);
        lua_insert!(L, - 2)(L, lua_insert!(L, - 2), lua_insert!(L, - 2));
        lua_concat(L, 3 as libc::c_int);
    };
}
unsafe extern "C" fn loadline(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    lua_settop(L, 0 as libc::c_int);
    if pushline(L, 1 as libc::c_int) == 0 {
        return -(1 as libc::c_int);
    }
    status = addreturn(L);
    if status != LUA_OK {
        status = multiline(L);
    }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return status;
}
unsafe extern "C" fn l_print(mut L: *mut lua_State) {
    let mut n = lua_gettop(L);
    if n > 0 as libc::c_int {
        luaL_checkstack(
            L,
            LUA_MINSTACK,
            b"too many results to print\0" as *const u8 as *const libc::c_char,
        );
        lua_getglobal(L, b"print\0" as *const u8 as *const libc::c_char);
        lua_insert!(L, 1)(L, lua_insert!(L, 1), lua_insert!(L, 1));
        if lua_pcall!(L, n, 0, 0) != LUA_OK {
            l_message(
                progname,
                lua_pushfstring(
                    L,
                    b"error calling 'print' (%s)\0" as *const u8 as *const libc::c_char,
                    lua_tostring!(L, - 1),
                ),
            );
        }
    }
}
unsafe extern "C" fn doREPL(mut L: *mut lua_State) {
    let mut status: libc::c_int = 0;
    let mut oldprogname = progname;
    progname = NULL as *const libc::c_char;
    loop {
        status = loadline(L);
        if !(status != -(1 as libc::c_int)) {
            break;
        }
        if status == LUA_OK {
            status = docall(L, 0 as libc::c_int, LUA_MULTRET);
        }
        if status == LUA_OK {
            l_print(L);
        } else {
            report(L, status);
        }
    }
    lua_settop(L, 0 as libc::c_int);
    fwrite(
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stdout,
    );
    progname = oldprogname;
}
unsafe extern "C" fn pushargs(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if lua_getglobal(L, b"arg\0" as *const u8 as *const libc::c_char) != LUA_TTABLE {
        luaL_error(L, b"'arg' is not a table\0" as *const u8 as *const libc::c_char);
    }
    n = luaL_len(L, -(1 as libc::c_int)) as libc::c_int;
    luaL_checkstack(
        L,
        n + 3 as libc::c_int,
        b"too many arguments to script\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i <= n {
        lua_rawgeti(L, -i, i as lua_Integer);
        i += 1;
    }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return n;
}
unsafe extern "C" fn handle_script(
    mut L: *mut lua_State,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut fname: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    if strcmp(fname, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        && strcmp(
            *argv.offset(-(1 as libc::c_int) as isize),
            b"--\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        fname = NULL as *const libc::c_char;
    }
    status = luaL_loadfile!(L, fname);
    if status == LUA_OK {
        let mut n = pushargs(L);
        status = docall(L, n, LUA_MULTRET);
    }
    return report(L, status);
}
pub const has_error: libc::c_int = 1 as libc::c_int;
pub const has_i: libc::c_int = 2 as libc::c_int;
pub const has_v: libc::c_int = 4 as libc::c_int;
pub const has_e: libc::c_int = 8 as libc::c_int;
pub const has_E: libc::c_int = 16 as libc::c_int;
unsafe extern "C" fn collectargs(
    mut argv: *mut *mut libc::c_char,
    mut first: *mut libc::c_int,
) -> libc::c_int {
    let mut args = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while !(*argv.offset(i as isize)).is_null() {
        *first = i;
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            != '-' as i32
        {
            return args;
        }
        let mut current_block_22: u64;
        match *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int
        {
            45 => {
                if *(*argv.offset(i as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int != '\0' as i32
                {
                    return has_error;
                }
                *first = i + 1 as libc::c_int;
                return args;
            }
            0 => return args,
            69 => {
                if *(*argv.offset(i as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int != '\0' as i32
                {
                    return has_error;
                }
                args |= has_E;
                current_block_22 = 2668756484064249700;
            }
            105 => {
                args |= has_i;
                current_block_22 = 15706559463154051555;
            }
            118 => {
                current_block_22 = 15706559463154051555;
            }
            101 => {
                args |= has_e;
                current_block_22 = 459704395062247846;
            }
            108 => {
                current_block_22 = 459704395062247846;
            }
            _ => return has_error,
        }
        match current_block_22 {
            15706559463154051555 => {
                if *(*argv.offset(i as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int != '\0' as i32
                {
                    return has_error;
                }
                args |= has_v;
            }
            459704395062247846 => {
                if *(*argv.offset(i as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                {
                    i += 1;
                    if (*argv.offset(i as isize)).is_null()
                        || *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                            as libc::c_int == '-' as i32
                    {
                        return has_error;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    *first = i;
    return args;
}
unsafe extern "C" fn runargs(
    mut L: *mut lua_State,
    mut argv: *mut *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < n {
        let mut option = *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int;
        if option == 'e' as i32 || option == 'l' as i32 {
            let mut status: libc::c_int = 0;
            let mut extra: *const libc::c_char = (*argv.offset(i as isize))
                .offset(2 as libc::c_int as isize);
            if *extra as libc::c_int == '\0' as i32 {
                i += 1;
                extra = *argv.offset(i as isize);
            }
            status = if option == 'e' as i32 {
                dostring(
                    L,
                    extra,
                    b"=(command line)\0" as *const u8 as *const libc::c_char,
                )
            } else {
                dolibrary(L, extra)
            };
            if status != LUA_OK {
                return 0 as libc::c_int;
            }
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn handle_luainit(mut L: *mut lua_State) -> libc::c_int {
    let mut name = b"=LUA_INIT_5_3\0" as *const u8 as *const libc::c_char;
    let mut init: *const libc::c_char = getenv(name.offset(1 as libc::c_int as isize));
    if init.is_null() {
        name = b"=LUA_INIT\0" as *const u8 as *const libc::c_char;
        init = getenv(name.offset(1 as libc::c_int as isize));
    }
    if init.is_null() {
        return LUA_OK
    } else if *init.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32 {
        return dofile(L, init.offset(1 as libc::c_int as isize))
    } else {
        return dostring(L, init, name)
    };
}
unsafe extern "C" fn pmain(mut L: *mut lua_State) -> libc::c_int {
    let mut argc = lua_tointeger!(L, 1) as libc::c_int;
    let mut argv = lua_touserdata(L, 2 as libc::c_int) as *mut *mut libc::c_char;
    let mut script: libc::c_int = 0;
    let mut args = collectargs(argv, &mut script);
    luaL_checkversion!(L)(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES);
    if !(*argv.offset(0 as libc::c_int as isize)).is_null()
        && *(*argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != 0
    {
        progname = *argv.offset(0 as libc::c_int as isize);
    }
    if args == has_error {
        print_usage(*argv.offset(script as isize));
        return 0 as libc::c_int;
    }
    if args & has_v != 0 {
        print_version();
    }
    if args & has_E != 0 {
        lua_pushboolean(L, 1 as libc::c_int);
        lua_setfield(
            L,
            LUA_REGISTRYINDEX,
            b"LUA_NOENV\0" as *const u8 as *const libc::c_char,
        );
    }
    luaL_openlibs(L);
    createargtable(L, argv, argc, script);
    if args & has_E == 0 {
        if handle_luainit(L) != LUA_OK {
            return 0 as libc::c_int;
        }
    }
    if runargs(L, argv, script) == 0 {
        return 0 as libc::c_int;
    }
    if script < argc && handle_script(L, argv.offset(script as isize)) != LUA_OK {
        return 0 as libc::c_int;
    }
    if args & has_i != 0 {
        doREPL(L);
    } else if script == argc && args & (has_e | has_v) == 0 {
        print_version();
        doREPL(L);
    }
    lua_pushboolean(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut L = luaL_newstate();
    if L.is_null() {
        l_message(
            *argv.offset(0 as libc::c_int as isize),
            b"cannot create state: not enough memory\0" as *const u8
                as *const libc::c_char,
        );
        return EXIT_FAILURE;
    }
    lua_pushcfunction!(
        L, & pmain
    )(L, lua_pushcfunction!(L, & pmain), lua_pushcfunction!(L, & pmain));
    lua_pushinteger(L, argc as lua_Integer);
    lua_pushlightuserdata(L, argv as *mut libc::c_void);
    status = lua_pcall!(L, 2, 1, 0);
    result = lua_toboolean(L, -(1 as libc::c_int));
    report(L, status);
    lua_close(L);
    return if result != 0 && status == LUA_OK { EXIT_SUCCESS } else { EXIT_FAILURE };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
