use crate::lapi::{
    lua_callk, lua_checkstack, lua_compare, lua_createtable, lua_geti, lua_getmetatable,
    lua_gettop, lua_isstring, lua_pushinteger, lua_pushnil, lua_pushstring, lua_pushvalue,
    lua_rawget, lua_rotate, lua_setfield, lua_seti, lua_settop, lua_toboolean, lua_type,
    lua_typename,
};
use crate::lauxlib::{
    luaL_Buffer, luaL_Reg, luaL_addlstring, luaL_addvalue, luaL_argerror, luaL_buffinit,
    luaL_checkinteger, luaL_checktype, luaL_checkversion_, luaL_error, luaL_len, luaL_optinteger,
    luaL_optlstring, luaL_pushresult, luaL_setfuncs,
};
use crate::lstate::lua_State;
use crate::types::{lua_Integer, lua_KContext, lua_Number, lua_Unsigned, LUA_TTABLE};
use libc::{
    c_char, c_int, c_long, c_longlong, c_uint, c_ulong, c_ulonglong, c_void, clock_t, memcpy,
    size_t,
};

pub type IdxT = c_uint;
pub type time_t = c_long;

pub const TAB_R: c_int = 1;
pub const TAB_W: c_int = 2;
pub const TAB_L: c_int = 4;

extern "C" {
    fn clock() -> clock_t;
    fn time(_: *mut time_t) -> time_t;
}

unsafe fn checkfield(L: *mut lua_State, key: *const c_char, n: c_int) -> c_int {
    lua_pushstring(L, key);
    return (lua_rawget(L, -n) != 0) as c_int;
}

unsafe fn checktab(L: *mut lua_State, arg: c_int, what: c_int) {
    if lua_type(L, arg) != LUA_TTABLE {
        let mut n: c_int = 1;
        if lua_getmetatable(L, arg) != 0
            && (what & TAB_R == 0 || {
                n += 1;
                checkfield(L, cstr!("__index"), n) != 0
            })
            && (what & TAB_W == 0 || {
                n += 1;
                checkfield(L, cstr!("__newindex"), n) != 0
            })
            && (what & TAB_L == 0 || {
                n += 1;
                checkfield(L, cstr!("__len"), n) != 0
            })
        {
            lua_settop(L, -n - 1);
        } else {
            luaL_checktype(L, arg, 5);
        }
    }
}
unsafe extern "C" fn tinsert(L: *mut lua_State) -> c_int {
    checktab(L, 1, 1 | 2 | 4);
    let e: lua_Integer = luaL_len(L, 1) + 1;
    let pos;
    match lua_gettop(L) {
        2 => {
            pos = e;
        }
        3 => {
            pos = luaL_checkinteger(L, 2);
            let _ =
                1 <= pos && pos <= e || luaL_argerror(L, 2, cstr!("position out of bounds")) != 0;
            let mut i: lua_Integer = e;
            while i > pos {
                lua_geti(L, 1, i - 1);
                lua_seti(L, 1, i);
                i -= 1;
            }
        }
        _ => {
            luaL_error(L, cstr!("wrong number of arguments to 'insert'"));
        }
    }
    lua_seti(L, 1, pos);
    return 0;
}
unsafe extern "C" fn tremove(L: *mut lua_State) -> c_int {
    checktab(L, 1, 1 | 2 | 4);
    let size: lua_Integer = luaL_len(L, 1);
    let mut pos: lua_Integer = luaL_optinteger(L, 2, size);
    if pos != size {
        let _ = 1 <= pos && pos <= size + 1
            || luaL_argerror(L, 1, cstr!("position out of bounds")) != 0;
    }
    lua_geti(L, 1, pos);
    while pos < size {
        lua_geti(L, 1, pos + 1);
        lua_seti(L, 1, pos);
        pos += 1;
    }
    lua_pushnil(L);
    lua_seti(L, 1, pos);
    return 1;
}
unsafe extern "C" fn tmove(L: *mut lua_State) -> c_int {
    let f: lua_Integer = luaL_checkinteger(L, 2);
    let e: lua_Integer = luaL_checkinteger(L, 3);
    let t: lua_Integer = luaL_checkinteger(L, 4);
    let tt: c_int = if !(lua_type(L, 5) <= 0) { 5 } else { 1 };
    checktab(L, 1, 1);
    checktab(L, tt, 2);
    if e >= f {
        let mut i = 0 as lua_Integer;
        let _ = f > 0
            || e < lua_Integer::MAX + f
            || luaL_argerror(L, 3, cstr!("too many elements to move")) != 0;
        let n: lua_Integer = e - f + 1;
        let _ = t <= lua_Integer::MAX - n + 1
            || luaL_argerror(L, 4, cstr!("destination wrap around")) != 0;
        if t > e || t <= f || tt != 1 && lua_compare(L, 1, tt, 0) == 0 {
            while i < n {
                lua_geti(L, 1, f + i);
                lua_seti(L, tt, t + i);
                i += 1;
            }
        } else {
            i = n - 1;
            while i >= 0 {
                lua_geti(L, 1, f + i);
                lua_seti(L, tt, t + i);
                i -= 1;
            }
        }
    }
    lua_pushvalue(L, tt);
    return 1;
}
unsafe fn addfield(L: *mut lua_State, b: *mut luaL_Buffer, i: lua_Integer) {
    lua_geti(L, 1, i);
    if lua_isstring(L, -(1)) == 0 {
        luaL_error(
            L,
            cstr!("invalid value (%s) at index %d in table for 'concat'"),
            lua_typename(L, lua_type(L, -(1))),
            i,
        );
    }
    luaL_addvalue(b);
}
unsafe extern "C" fn tconcat(L: *mut lua_State) -> c_int {
    let mut b: luaL_Buffer = luaL_Buffer::new();
    checktab(L, 1, 1 | 4);
    let mut last: lua_Integer = luaL_len(L, 1);
    let mut lsep: size_t = 0;
    let sep: *const c_char = luaL_optlstring(L, 2, cstr!(""), &mut lsep);
    let mut i: lua_Integer = luaL_optinteger(L, 3, 1 as lua_Integer);
    last = luaL_optinteger(L, 4, last);
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
    return 1;
}
unsafe extern "C" fn pack(L: *mut lua_State) -> c_int {
    let n: c_int = lua_gettop(L);
    lua_createtable(L, n, 1);
    lua_rotate(L, 1, 1);
    let mut i: c_int = n;
    while i >= 1 {
        lua_seti(L, 1, i as lua_Integer);
        i -= 1;
    }
    lua_pushinteger(L, n as lua_Integer);
    lua_setfield(L, 1, cstr!("n"));
    return 1;
}
unsafe extern "C" fn unpack(L: *mut lua_State) -> c_int {
    let mut i: lua_Integer = luaL_optinteger(L, 2, 1 as lua_Integer);
    let e: lua_Integer = if lua_type(L, 3) <= 0 {
        luaL_len(L, 1)
    } else {
        luaL_checkinteger(L, 3)
    };
    if i > e {
        return 0;
    }
    let mut n = (e as lua_Unsigned).wrapping_sub(i as c_ulonglong);
    if n >= c_int::MAX as c_ulonglong || {
        n = n.wrapping_add(1);
        lua_checkstack(L, n as c_int) == 0
    } {
        luaL_error(L, cstr!("too many results to unpack"));
    }
    while i < e {
        lua_geti(L, 1, i);
        i += 1;
    }
    lua_geti(L, 1, e);
    return n as c_int;
}

unsafe fn l_randomizePivot() -> c_uint {
    let mut c: clock_t = clock();
    let mut t: time_t = time(0 as *mut time_t);
    let mut buff: [c_uint; 4] = [0; 4];
    let mut rnd: c_uint = 0;
    memcpy(
        buff.as_mut_ptr() as *mut c_void,
        &mut c as *mut clock_t as *const c_void,
        (::std::mem::size_of::<clock_t>() as usize)
            .wrapping_div(::std::mem::size_of::<c_uint>() as usize)
            .wrapping_mul(::std::mem::size_of::<c_uint>() as usize),
    );
    memcpy(
        buff.as_mut_ptr().offset(
            (::std::mem::size_of::<clock_t>() as c_ulong)
                .wrapping_div(::std::mem::size_of::<c_uint>() as c_ulong) as isize,
        ) as *mut c_void,
        &mut t as *mut time_t as *const c_void,
        (::std::mem::size_of::<time_t>() as usize)
            .wrapping_div(::std::mem::size_of::<c_uint>() as usize)
            .wrapping_mul(::std::mem::size_of::<c_uint>() as usize),
    );
    let mut i: c_int = 0;
    while (i as c_ulong)
        < (::std::mem::size_of::<[c_uint; 4]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<c_uint>() as c_ulong)
    {
        rnd = rnd.wrapping_add(buff[i as usize]);
        i = i.wrapping_add(1);
    }
    return rnd;
}

unsafe fn set2(L: *mut lua_State, i: IdxT, j: IdxT) {
    lua_seti(L, 1, i as lua_Integer);
    lua_seti(L, 1, j as lua_Integer);
}

unsafe fn sort_comp(L: *mut lua_State, a: c_int, b: c_int) -> c_int {
    if lua_type(L, 2) == 0 {
        return lua_compare(L, a, b, 1);
    } else {
        lua_pushvalue(L, 2);
        lua_pushvalue(L, a - 1);
        lua_pushvalue(L, b - 2);
        lua_callk(L, 2, 1, 0 as lua_KContext, None);
        let res = lua_toboolean(L, -(1));
        lua_settop(L, -(1) - 1);
        return res;
    };
}
unsafe fn partition(L: *mut lua_State, lo: IdxT, up: IdxT) -> IdxT {
    let mut i: IdxT = lo;
    let mut j: IdxT = up.wrapping_sub(1);
    loop {
        loop {
            i = i.wrapping_add(1);
            lua_geti(L, 1, i as lua_Integer);
            if !(sort_comp(L, -(1), -(2)) != 0) {
                break;
            }
            if i == up.wrapping_sub(1) {
                luaL_error(L, cstr!("invalid order function for sorting"));
            }
            lua_settop(L, -(1) - 1);
        }
        loop {
            j = j.wrapping_sub(1);
            lua_geti(L, 1, j as lua_Integer);
            if !(sort_comp(L, -(3), -(1)) != 0) {
                break;
            }
            if j < i {
                luaL_error(L, cstr!("invalid order function for sorting"));
            }
            lua_settop(L, -(1) - 1);
        }
        if j < i {
            lua_settop(L, -(1) - 1);
            set2(L, up.wrapping_sub(1), i);
            return i;
        }
        set2(L, i, j);
    }
}

unsafe fn choosePivot(lo: IdxT, up: IdxT, rnd: c_uint) -> IdxT {
    let r4: IdxT = up.wrapping_sub(lo).wrapping_div(4);
    let p: IdxT = rnd
        .wrapping_rem(r4.wrapping_mul(2))
        .wrapping_add(lo.wrapping_add(r4));
    return p;
}
unsafe fn auxsort(L: *mut lua_State, mut lo: IdxT, mut up: IdxT, mut rnd: c_uint) {
    while lo < up {
        let mut p: IdxT;
        let n: IdxT;
        lua_geti(L, 1, lo as lua_Integer);
        lua_geti(L, 1, up as lua_Integer);
        if sort_comp(L, -(1), -(2)) != 0 {
            set2(L, lo, up);
        } else {
            lua_settop(L, -(2) - 1);
        }
        if up.wrapping_sub(lo) == 1 {
            return;
        }
        if up.wrapping_sub(lo) < 100 || rnd == 0 {
            p = lo.wrapping_add(up).wrapping_div(2);
        } else {
            p = choosePivot(lo, up, rnd);
        }
        lua_geti(L, 1, p as lua_Integer);
        lua_geti(L, 1, lo as lua_Integer);
        if sort_comp(L, -(2), -(1)) != 0 {
            set2(L, p, lo);
        } else {
            lua_settop(L, -(1) - 1);
            lua_geti(L, 1, up as lua_Integer);
            if sort_comp(L, -(1), -(2)) != 0 {
                set2(L, p, up);
            } else {
                lua_settop(L, -(2) - 1);
            }
        }
        if up.wrapping_sub(lo) == 2 {
            return;
        }
        lua_geti(L, 1, p as lua_Integer);
        lua_pushvalue(L, -(1));
        lua_geti(L, 1, up.wrapping_sub(1) as lua_Integer);
        set2(L, p, up.wrapping_sub(1));
        p = partition(L, lo, up);
        if p.wrapping_sub(lo) < up.wrapping_sub(p) {
            auxsort(L, lo, p.wrapping_sub(1), rnd);
            n = p.wrapping_sub(lo);
            lo = p.wrapping_add(1);
        } else {
            auxsort(L, p.wrapping_add(1), up, rnd);
            n = up.wrapping_sub(p);
            up = p.wrapping_sub(1);
        }
        if up.wrapping_sub(lo).wrapping_div(128) > n {
            rnd = l_randomizePivot();
        }
    }
}

unsafe extern "C" fn sort(L: *mut lua_State) -> c_int {
    checktab(L, 1, 1 | 2 | 4);
    let n: lua_Integer = luaL_len(L, 1);
    if n > 1 {
        let _ = n < c_int::MAX as c_longlong || luaL_argerror(L, 1, cstr!("array too big")) != 0;
        if !(lua_type(L, 2) <= 0) {
            luaL_checktype(L, 2, 6);
        }
        lua_settop(L, 2);
        auxsort(L, 1 as IdxT, n as IdxT, 0);
    }
    return 0;
}

static mut tab_funcs: [luaL_Reg; 8] = {
    [
        {
            let init = luaL_Reg {
                name: cstr!("concat"),
                func: Some(tconcat as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("insert"),
                func: Some(tinsert as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("pack"),
                func: Some(pack as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("unpack"),
                func: Some(unpack as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("remove"),
                func: Some(tremove as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("move"),
                func: Some(tmove as unsafe extern "C" fn(*mut lua_State) -> c_int),
            };
            init
        },
        {
            let init = luaL_Reg {
                name: cstr!("sort"),
                func: Some(sort),
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
        503 as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as c_ulong)
            .wrapping_mul(16 as c_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as c_ulong)
            .try_into()
            .unwrap(),
    );
    lua_createtable(
        L,
        0,
        (::std::mem::size_of::<[luaL_Reg; 8]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as c_ulong)
            .wrapping_sub(1 as c_int as c_ulong) as c_int,
    );
    luaL_setfuncs(L, tab_funcs.as_ptr(), 0);
    return 1;
}
