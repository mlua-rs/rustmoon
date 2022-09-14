use crate::lauxlib::{luaL_Reg, luaL_Buffer};
use crate::lstate::lua_State;
use crate::types::{lua_Integer, lua_KContext, lua_KFunction, lua_Number, lua_Unsigned};
use crate::lapi::{
    lua_createtable, lua_gettop, lua_pushnil, lua_pushvalue, lua_setfield, lua_pushstring, lua_pushinteger
};
use libc::{c_int, c_uint, c_char, c_long, c_longlong, c_ulong, c_ulonglong, c_void, clock_t, size_t};

pub type IdxT = c_uint;
pub type time_t = c_long;


extern "C" {
    fn memcpy(_: *mut c_void, _: *const c_void, _: c_ulong) -> *mut c_void;
    fn lua_settop(L: *mut lua_State, idx: c_int);
    fn lua_rotate(L: *mut lua_State, idx: c_int, n: c_int);
    fn lua_checkstack(L: *mut lua_State, n: c_int) -> c_int;
    fn lua_isstring(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_typename(L: *mut lua_State, tp: c_int) -> *const c_char;
    fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_compare(
        L: *mut lua_State,
        idx1: c_int,
        idx2: c_int,
        op: c_int,
    ) -> c_int;
    fn lua_geti(L: *mut lua_State, idx: c_int, n: lua_Integer) -> c_int;
    fn lua_rawget(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_getmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    fn lua_seti(L: *mut lua_State, idx: c_int, n: lua_Integer);
    fn lua_callk(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    );
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t);
    fn luaL_argerror(
        L: *mut lua_State,
        arg: c_int,
        extramsg: *const c_char,
    ) -> c_int;
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: c_int,
        def: *const c_char,
        l: *mut size_t,
    ) -> *const c_char;
    fn luaL_addvalue(B: *mut luaL_Buffer);
    fn luaL_checkinteger(L: *mut lua_State, arg: c_int) -> lua_Integer;
    fn luaL_optinteger(L: *mut lua_State, arg: c_int, def: lua_Integer) -> lua_Integer;
    fn luaL_checktype(L: *mut lua_State, arg: c_int, t: c_int);
    fn luaL_error(L: *mut lua_State, fmt: *const c_char, _: ...) -> c_int;
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_len(L: *mut lua_State, idx: c_int) -> lua_Integer;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: c_int);
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *const c_char, l: size_t);
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn clock() -> clock_t;
    fn time(_: *mut time_t) -> time_t;
}

unsafe extern "C" fn checkfield(
    L: *mut lua_State,
    key: *const c_char,
    n: c_int,
) -> c_int {
    lua_pushstring(L, key);
    return (lua_rawget(L, -n) != 0 as c_int) as c_int;
}

unsafe extern "C" fn checktab(L: *mut lua_State, arg: c_int, what: c_int) {
    if lua_type(L, arg) != 5 as c_int {
        let mut n: c_int = 1 as c_int;
        if lua_getmetatable(L, arg) != 0
            && (what & 1 as c_int == 0 || {
                n += 1;
                checkfield(L, b"__index\0" as *const u8 as *const c_char, n) != 0
            })
            && (what & 2 as c_int == 0 || {
                n += 1;
                checkfield(L, b"__newindex\0" as *const u8 as *const c_char, n) != 0
            })
            && (what & 4 as c_int == 0 || {
                n += 1;
                checkfield(L, b"__len\0" as *const u8 as *const c_char, n) != 0
            })
        {
            lua_settop(L, -n - 1 as c_int);
        } else {
            luaL_checktype(L, arg, 5 as c_int);
        }
    }
}
unsafe extern "C" fn tinsert(L: *mut lua_State) -> c_int {
    checktab(
        L,
        1 as c_int,
        1 as c_int | 2 as c_int | 4 as c_int,
    );
    let e: lua_Integer = luaL_len(L, 1 as c_int) + 1 as c_int as c_longlong;
    let pos;
    match lua_gettop(L) {
        2 => {
            pos = e;
        }
        3 => {
            pos = luaL_checkinteger(L, 2 as c_int);
            (1 as c_int as c_longlong <= pos && pos <= e
                || luaL_argerror(
                    L,
                    2 as c_int,
                    b"position out of bounds\0" as *const u8 as *const c_char,
                ) != 0) as c_int;
            let mut i: lua_Integer = e;
            while i > pos {
                lua_geti(
                    L,
                    1 as c_int,
                    i - 1 as c_int as c_longlong,
                );
                lua_seti(L, 1 as c_int, i);
                i -= 1;
            }
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments to 'insert'\0" as *const u8 as *const c_char,
            );
        }
    }
    lua_seti(L, 1 as c_int, pos);
    return 0 as c_int;
}
unsafe extern "C" fn tremove(L: *mut lua_State) -> c_int {
    checktab(
        L,
        1 as c_int,
        1 as c_int | 2 as c_int | 4 as c_int,
    );
    let size: lua_Integer = luaL_len(L, 1 as c_int);
    let mut pos: lua_Integer = luaL_optinteger(L, 2 as c_int, size);
    if pos != size {
        (1 as c_int as c_longlong <= pos
            && pos <= size + 1 as c_int as c_longlong
            || luaL_argerror(
                L,
                1 as c_int,
                b"position out of bounds\0" as *const u8 as *const c_char,
            ) != 0) as c_int;
    }
    lua_geti(L, 1 as c_int, pos);
    while pos < size {
        lua_geti(
            L,
            1 as c_int,
            pos + 1 as c_int as c_longlong,
        );
        lua_seti(L, 1 as c_int, pos);
        pos += 1;
    }
    lua_pushnil(L);
    lua_seti(L, 1 as c_int, pos);
    return 1 as c_int;
}
unsafe extern "C" fn tmove(L: *mut lua_State) -> c_int {
    let f: lua_Integer = luaL_checkinteger(L, 2 as c_int);
    let e: lua_Integer = luaL_checkinteger(L, 3 as c_int);
    let t: lua_Integer = luaL_checkinteger(L, 4 as c_int);
    let tt: c_int = if !(lua_type(L, 5 as c_int) <= 0 as c_int) {
        5 as c_int
    } else {
        1 as c_int
    };
    checktab(L, 1 as c_int, 1 as c_int);
    checktab(L, tt, 2 as c_int);
    if e >= f {
        let mut i = 0 as c_int as lua_Integer;
        (f > 0 as c_int as c_longlong
            || e < 0x7fffffffffffffff as c_longlong + f
            || luaL_argerror(
                L,
                3 as c_int,
                b"too many elements to move\0" as *const u8 as *const c_char,
            ) != 0) as c_int;
        let n: lua_Integer = e - f + 1 as c_int as c_longlong;
        (t <= 0x7fffffffffffffff as c_longlong - n + 1 as c_int as c_longlong
            || luaL_argerror(
                L,
                4 as c_int,
                b"destination wrap around\0" as *const u8 as *const c_char,
            ) != 0) as c_int;
        if t > e
            || t <= f
            || tt != 1 as c_int && lua_compare(L, 1 as c_int, tt, 0 as c_int) == 0
        {
            while i < n {
                lua_geti(L, 1 as c_int, f + i);
                lua_seti(L, tt, t + i);
                i += 1;
            }
        } else {
            i = n - 1 as c_int as c_longlong;
            while i >= 0 as c_int as c_longlong {
                lua_geti(L, 1 as c_int, f + i);
                lua_seti(L, tt, t + i);
                i -= 1;
            }
        }
    }
    lua_pushvalue(L, tt);
    return 1 as c_int;
}
unsafe extern "C" fn addfield(L: *mut lua_State, b: *mut luaL_Buffer, i: lua_Integer) {
    lua_geti(L, 1 as c_int, i);
    if lua_isstring(L, -(1 as c_int)) == 0 {
        luaL_error(
            L,
            b"invalid value (%s) at index %d in table for 'concat'\0" as *const u8
                as *const c_char,
            lua_typename(L, lua_type(L, -(1 as c_int))),
            i,
        );
    }
    luaL_addvalue(b);
}
unsafe extern "C" fn tconcat(L: *mut lua_State) -> c_int {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    checktab(L, 1 as c_int, 1 as c_int | 4 as c_int);
    let mut last: lua_Integer = luaL_len(L, 1 as c_int);
    let mut lsep: size_t = 0;
    let sep: *const c_char = luaL_optlstring(
        L,
        2 as c_int,
        b"\0" as *const u8 as *const c_char,
        &mut lsep,
    );
    let mut i: lua_Integer = luaL_optinteger(L, 3 as c_int, 1 as c_int as lua_Integer);
    last = luaL_optinteger(L, 4 as c_int, last);
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
    return 1 as c_int;
}
unsafe extern "C" fn pack(L: *mut lua_State) -> c_int {
    let n: c_int = lua_gettop(L);
    lua_createtable(L, n, 1 as c_int);
    lua_rotate(L, 1 as c_int, 1 as c_int);
    let mut i: c_int = n;
    while i >= 1 as c_int {
        lua_seti(L, 1 as c_int, i as lua_Integer);
        i -= 1;
    }
    lua_pushinteger(L, n as lua_Integer);
    lua_setfield(
        L,
        1 as c_int,
        b"n\0" as *const u8 as *const c_char,
    );
    return 1 as c_int;
}
unsafe extern "C" fn unpack(L: *mut lua_State) -> c_int {
    let mut i: lua_Integer = luaL_optinteger(L, 2 as c_int, 1 as c_int as lua_Integer);
    let e: lua_Integer = if lua_type(L, 3 as c_int) <= 0 as c_int {
        luaL_len(L, 1 as c_int)
    } else {
        luaL_checkinteger(L, 3 as c_int)
    };
    if i > e {
        return 0 as c_int;
    }
    let mut n = (e as lua_Unsigned).wrapping_sub(i as c_ulonglong);
    if n >= 2147483647 as c_int as c_uint as c_ulonglong || {
        n = n.wrapping_add(1);
        lua_checkstack(L, n as c_int) == 0
    } {
        return luaL_error(
            L,
            b"too many results to unpack\0" as *const u8 as *const c_char,
        );
    }
    while i < e {
        lua_geti(L, 1 as c_int, i);
        i += 1;
    }
    lua_geti(L, 1 as c_int, e);
    return n as c_int;
}

unsafe extern "C" fn l_randomizePivot() -> c_uint {
    let mut c: clock_t = clock();
    let mut t: time_t = time(0 as *mut time_t);
    let mut buff: [c_uint; 4] = [0; 4];
    let mut rnd: c_uint = 0 as c_int as c_uint;
    memcpy(
        buff.as_mut_ptr() as *mut c_void,
        &mut c as *mut clock_t as *const c_void,
        (::std::mem::size_of::<clock_t>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<c_uint>() as c_ulong)
            .wrapping_mul(::std::mem::size_of::<c_uint>() as c_ulong),
    );
    memcpy(
        buff.as_mut_ptr().offset(
            (::std::mem::size_of::<clock_t>() as c_ulong)
                .wrapping_div(::std::mem::size_of::<c_uint>() as c_ulong)
                as isize,
        ) as *mut c_void,
        &mut t as *mut time_t as *const c_void,
        (::std::mem::size_of::<time_t>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<c_uint>() as c_ulong)
            .wrapping_mul(::std::mem::size_of::<c_uint>() as c_ulong),
    );
    let mut i = 0 as c_int as c_uint;
    while (i as c_ulong)
        < (::std::mem::size_of::<[c_uint; 4]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<c_uint>() as c_ulong)
    {
        rnd = rnd.wrapping_add(buff[i as usize]);
        i = i.wrapping_add(1);
    }
    return rnd;
}

#[no_mangle]
pub unsafe extern "C" fn set2(L: *mut lua_State, i: IdxT, j: IdxT) {
    lua_seti(L, 1 as c_int, i as lua_Integer);
    lua_seti(L, 1 as c_int, j as lua_Integer);
}

unsafe extern "C" fn sort_comp(L: *mut lua_State, a: c_int, b: c_int) -> c_int {
    if lua_type(L, 2 as c_int) == 0 as c_int {
        return lua_compare(L, a, b, 1 as c_int);
    } else {
        lua_pushvalue(L, 2 as c_int);
        lua_pushvalue(L, a - 1 as c_int);
        lua_pushvalue(L, b - 2 as c_int);
        lua_callk(
            L,
            2 as c_int,
            1 as c_int,
            0 as c_int as lua_KContext,
            None,
        );
        let res = lua_toboolean(L, -(1 as c_int));
        lua_settop(L, -(1 as c_int) - 1 as c_int);
        return res;
    };
}
unsafe extern "C" fn partition(L: *mut lua_State, lo: IdxT, up: IdxT) -> IdxT {
    let mut i: IdxT = lo;
    let mut j: IdxT = up.wrapping_sub(1 as c_int as c_uint);
    loop {
        loop {
            i = i.wrapping_add(1);
            lua_geti(L, 1 as c_int, i as lua_Integer);
            if !(sort_comp(L, -(1 as c_int), -(2 as c_int)) != 0) {
                break;
            }
            if i == up.wrapping_sub(1 as c_int as c_uint) {
                luaL_error(
                    L,
                    b"invalid order function for sorting\0" as *const u8 as *const c_char,
                );
            }
            lua_settop(L, -(1 as c_int) - 1 as c_int);
        }
        loop {
            j = j.wrapping_sub(1);
            lua_geti(L, 1 as c_int, j as lua_Integer);
            if !(sort_comp(L, -(3 as c_int), -(1 as c_int)) != 0) {
                break;
            }
            if j < i {
                luaL_error(
                    L,
                    b"invalid order function for sorting\0" as *const u8 as *const c_char,
                );
            }
            lua_settop(L, -(1 as c_int) - 1 as c_int);
        }
        if j < i {
            lua_settop(L, -(1 as c_int) - 1 as c_int);
            set2(L, up.wrapping_sub(1 as c_int as c_uint), i);
            return i;
        }
        set2(L, i, j);
    }
}

unsafe extern "C" fn choosePivot(lo: IdxT, up: IdxT, rnd: c_uint) -> IdxT {
    let r4: IdxT = up
        .wrapping_sub(lo)
        .wrapping_div(4 as c_int as c_uint);
    let p: IdxT = rnd
        .wrapping_rem(r4.wrapping_mul(2 as c_int as c_uint))
        .wrapping_add(lo.wrapping_add(r4));
    return p;
}
unsafe extern "C" fn auxsort(L: *mut lua_State, mut lo: IdxT, mut up: IdxT, mut rnd: c_uint) {
    while lo < up {
        let mut p: IdxT;
        let n: IdxT;
        lua_geti(L, 1 as c_int, lo as lua_Integer);
        lua_geti(L, 1 as c_int, up as lua_Integer);
        if sort_comp(L, -(1 as c_int), -(2 as c_int)) != 0 {
            set2(L, lo, up);
        } else {
            lua_settop(L, -(2 as c_int) - 1 as c_int);
        }
        if up.wrapping_sub(lo) == 1 as c_int as c_uint {
            return;
        }
        if up.wrapping_sub(lo) < 100 as c_uint || rnd == 0 as c_int as c_uint {
            p = lo
                .wrapping_add(up)
                .wrapping_div(2 as c_int as c_uint);
        } else {
            p = choosePivot(lo, up, rnd);
        }
        lua_geti(L, 1 as c_int, p as lua_Integer);
        lua_geti(L, 1 as c_int, lo as lua_Integer);
        if sort_comp(L, -(2 as c_int), -(1 as c_int)) != 0 {
            set2(L, p, lo);
        } else {
            lua_settop(L, -(1 as c_int) - 1 as c_int);
            lua_geti(L, 1 as c_int, up as lua_Integer);
            if sort_comp(L, -(1 as c_int), -(2 as c_int)) != 0 {
                set2(L, p, up);
            } else {
                lua_settop(L, -(2 as c_int) - 1 as c_int);
            }
        }
        if up.wrapping_sub(lo) == 2 as c_int as c_uint {
            return;
        }
        lua_geti(L, 1 as c_int, p as lua_Integer);
        lua_pushvalue(L, -(1 as c_int));
        lua_geti(
            L,
            1 as c_int,
            up.wrapping_sub(1 as c_int as c_uint) as lua_Integer,
        );
        set2(L, p, up.wrapping_sub(1 as c_int as c_uint));
        p = partition(L, lo, up);
        if p.wrapping_sub(lo) < up.wrapping_sub(p) {
            auxsort(L, lo, p.wrapping_sub(1 as c_int as c_uint), rnd);
            n = p.wrapping_sub(lo);
            lo = p.wrapping_add(1 as c_int as c_uint);
        } else {
            auxsort(L, p.wrapping_add(1 as c_int as c_uint), up, rnd);
            n = up.wrapping_sub(p);
            up = p.wrapping_sub(1 as c_int as c_uint);
        }
        if up
            .wrapping_sub(lo)
            .wrapping_div(128 as c_int as c_uint)
            > n
        {
            rnd = l_randomizePivot();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sort(L: *mut lua_State) -> c_int {
    checktab(
        L,
        1 as c_int,
        1 as c_int | 2 as c_int | 4 as c_int,
    );
    let n: lua_Integer = luaL_len(L, 1 as c_int);
    if n > 1 as c_int as c_longlong {
        (n < 2147483647 as c_int as c_longlong
            || luaL_argerror(
                L,
                1 as c_int,
                b"array too big\0" as *const u8 as *const c_char,
            ) != 0) as c_int;
        if !(lua_type(L, 2 as c_int) <= 0 as c_int) {
            luaL_checktype(L, 2 as c_int, 6 as c_int);
        }
        lua_settop(L, 2 as c_int);
        auxsort(
            L,
            1 as c_int as IdxT,
            n as IdxT,
            0 as c_int as c_uint,
        );
    }
    return 0 as c_int;
}

static mut tab_funcs: [luaL_Reg; 8] = {
    [
        {
            let init = luaL_Reg {
                name: b"concat\0" as *const u8 as *const c_char,
                func: Some(tconcat as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"insert\0" as *const u8 as *const c_char,
                func: Some(tinsert as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"pack\0" as *const u8 as *const c_char,
                func: Some(pack as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"unpack\0" as *const u8 as *const c_char,
                func: Some(unpack as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"remove\0" as *const u8 as *const c_char,
                func: Some(tremove as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"move\0" as *const u8 as *const c_char,
                func: Some(tmove as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: b"sort\0" as *const u8 as *const c_char,
                func: Some(sort as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: 0 as *const c_char,
                func: None,
            };
            init
        },
    ]
};

#[no_mangle]
pub unsafe extern "C" fn luaopen_table(L: *mut lua_State) -> c_int {
    luaL_checkversion_(
        L,
        503 as c_int as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as c_ulong)
            .wrapping_mul(16 as c_int as c_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as c_ulong)
            .try_into()
            .unwrap(),
    );
    lua_createtable(
        L,
        0 as c_int,
        (::std::mem::size_of::<[luaL_Reg; 8]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as c_ulong)
            .wrapping_sub(1 as c_int as c_ulong) as c_int,
    );
    luaL_setfuncs(L, tab_funcs.as_ptr(), 0 as c_int);
    return 1 as c_int;
}
