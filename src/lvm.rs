/*
** Lua virtual machine
*/

use std::ptr;

use libc::{c_char, c_int, c_uint, c_void, memcpy, strcoll, strlen};

use crate::ldebug::{luaG_ordererror, luaG_runerror, luaG_traceexec, luaG_typeerror};
use crate::ldo::{luaD_call, luaD_checkstack, luaD_poscall, luaD_precall};
use crate::lfunc::{luaF_close, luaF_findupval, luaF_newLclosure, UpVal};
use crate::lgc::{luaC_barrierback, luaC_condGC, luaC_upvalbarrier};
use crate::llimits::{Instruction, LUAI_MAXSHORTLEN};
use crate::lobject::{
    bvalue, chgfltvalue, chgivalue, clLvalue, fltvalue, fvalue, gcvalue, getproto, getstr, hvalue,
    ivalue, l_isfalse, luaO_fb2int, luaO_nilobject_, luaO_str2num, luaO_tostring, nvalue, pvalue,
    setbvalue, setclLvalue, setfltvalue, sethvalue, setivalue, setnilvalue, setobj, setsvalue,
    svalue, tsslen, tsvalue, ttisfloat, ttisfunction, ttisinteger, ttisnil, ttisnumber,
    ttisshrstring, ttisstring, ttistable, ttnov, ttype, uvalue, vslen, LClosure, Proto, StkId,
    TString, TValue, Table, LUA_TLCF, LUA_TLNGSTR, LUA_TNUMFLT, LUA_TNUMINT, LUA_TSHRSTR,
};
use crate::lopcodes::{
    getBMode, getCMode, GETARG_Ax, GETARG_Bx, GETARG_sBx, OpArgK, OpArgR, GETARG_A, GETARG_B,
    GETARG_C, GET_OPCODE, INDEXK, ISK, LFIELDS_PER_FLUSH, OP_ADD, OP_BAND, OP_BNOT, OP_BOR,
    OP_BXOR, OP_CALL, OP_CLOSURE, OP_CONCAT, OP_DIV, OP_EQ, OP_EXTRAARG, OP_FORLOOP, OP_FORPREP,
    OP_GETTABLE, OP_GETTABUP, OP_GETUPVAL, OP_IDIV, OP_JMP, OP_LE, OP_LEN, OP_LOADBOOL, OP_LOADK,
    OP_LOADKX, OP_LOADNIL, OP_LT, OP_MOD, OP_MOVE, OP_MUL, OP_NEWTABLE, OP_NOT, OP_POW, OP_RETURN,
    OP_SELF, OP_SETLIST, OP_SETTABLE, OP_SETTABUP, OP_SETUPVAL, OP_SHL, OP_SHR, OP_SUB,
    OP_TAILCALL, OP_TEST, OP_TESTSET, OP_TFORCALL, OP_TFORLOOP, OP_UNM, OP_VARARG,
};
use crate::lstate::{isLua, lua_State, CIST_FRESH, CIST_LEQ, CIST_TAIL};
use crate::lstring::{eqshrstr, luaS_createlngstrobj, luaS_eqlngstr, luaS_newlstr};
use crate::ltable::{
    invalidateTMcache, luaH_get, luaH_getn, luaH_getstr, luaH_new, luaH_newkey, luaH_resize,
    luaH_resizearray, luaH_setint,
};
use crate::ltm::{
    fasttm, luaT_callTM, luaT_callorderTM, luaT_gettmbyobj, luaT_trybinTM, TM_ADD, TM_BAND,
    TM_BNOT, TM_BOR, TM_BXOR, TM_CONCAT, TM_DIV, TM_EQ, TM_IDIV, TM_INDEX, TM_LE, TM_LEN, TM_LT,
    TM_MOD, TM_MUL, TM_NEWINDEX, TM_POW, TM_SHL, TM_SHR, TM_SUB, TM_UNM,
};
use crate::types::{
    lua_Integer, lua_Number, lua_Unsigned, LUA_MASKCOUNT, LUA_MASKLINE, LUA_MULTRET, LUA_TBOOLEAN,
    LUA_TLIGHTUSERDATA, LUA_TNIL, LUA_TNUMBER, LUA_TTABLE, LUA_TUSERDATA,
};

pub unsafe fn tonumber(o: *const TValue, n: *mut lua_Number) -> c_int {
    if ttisfloat(o) {
        *n = fltvalue(o);
        1
    } else {
        luaV_tonumber_(o, n)
    }
}

pub unsafe fn tointeger(o: *const TValue, i: *mut lua_Integer) -> c_int {
    if ttisinteger(o) {
        *i = ivalue(o);
        1
    } else {
        luaV_tointeger(o, i, 0 /* LUA_FLOORN2I */)
    }
}

// #define intop(op,v1,v2) l_castU2S(l_castS2U(v1) op l_castS2U(v2))

pub unsafe fn luaV_rawequalobj(t1: *const TValue, t2: *const TValue) -> c_int {
    luaV_equalobj(ptr::null_mut(), t1, t2)
}

/*
** fast track for 'gettable': if 't' is a table and 't[k]' is not nil,
** return 1 with 'slot' pointing to 't[k]' (final result).  Otherwise,
** return 0 (meaning it will have to check metamethod) with 'slot'
** pointing to a nil 't[k]' (if 't' is a table) or NULL (otherwise).
** 'f' is the raw get function to use.
*/
#[inline(always)]
unsafe fn luaV_fastget(
    _L: *mut lua_State,
    t: *const TValue,
    k: *const TValue,
    slot: &mut *const TValue,
    f: unsafe extern "C" fn(*mut Table, *const TValue) -> *const TValue,
) -> bool {
    if !ttistable(t) {
        /* not a table; 'slot' is NULL and result is 0 */
        *slot = ptr::null();
        false
    } else {
        /* else, do raw access */
        *slot = f(hvalue(t), k);
        !ttisnil(*slot)
    }
}

// TODO: Fixme!
#[inline(always)]
unsafe fn luaV_fastget_s(
    _L: *mut lua_State,
    t: *const TValue,
    k: *const TString,
    slot: &mut *const TValue,
    f: unsafe extern "C" fn(*mut Table, *const TString) -> *const TValue,
) -> bool {
    if !ttistable(t) {
        /* not a table; 'slot' is NULL and result is 0 */
        *slot = ptr::null();
        false
    } else {
        /* else, do raw access */
        *slot = f(hvalue(t), k);
        !ttisnil(*slot)
    }
}

/*
** standard implementation for 'gettable'
*/
// #define luaV_gettable(L,t,k,v) { const TValue *slot; \
//     if (luaV_fastget(L,t,k,slot,luaH_get)) { setobj2s(L, v, slot); } \
//     else luaV_finishget(L,t,k,v,slot); }

/*
** Fast track for set table. If 't' is a table and 't[k]' is not nil,
** call GC barrier, do a raw 't[k]=v', and return true; otherwise,
** return false with 'slot' equal to NULL (if 't' is not a table) or
** 'nil'. (This is needed by 'luaV_finishget'.) Note that, if the macro
** returns true, there is no need to 'invalidateTMcache', because the
** call is not creating a new entry.
*/
// #define luaV_fastset(L,t,k,slot,f,v) \
//   (!ttistable(t) \
//    ? (slot = NULL, 0) \
//    : (slot = f(hvalue(t), k), \
//      ttisnil(slot) ? 0 \
//      : (luaC_barrierback(L, hvalue(t), v), \
//         setobj2t(L, cast(TValue *,slot), v), \
//         1)))
#[inline(always)]
unsafe fn luaV_fastset(
    L: *mut lua_State,
    t: *const TValue,
    k: *const TValue,
    slot: &mut *const TValue,
    f: unsafe extern "C" fn(*mut Table, *const TValue) -> *const TValue,
    v: *const TValue,
) -> bool {
    if !ttistable(t) {
        *slot = ptr::null();
        false
    } else {
        *slot = f(hvalue(t), k);
        if ttisnil(*slot) {
            false
        } else {
            luaC_barrierback(L, hvalue(t), v);
            setobj(L, *slot as *mut TValue, v);
            true
        }
    }
}

// #define luaV_settable(L,t,k,v) { const TValue *slot; \
//     if (!luaV_fastset(L,t,k,slot,luaH_get,v)) \
//       luaV_finishset(L,t,k,v,slot); }

// Limit for table tag-method chains (to avoid loops)
const MAXTAGLOOP: usize = 2000;

/*
** Check whether some integers may not fit in a float, that is, whether
** (maxinteger >> NBM) > 0 (that implies (1 << NBM) <= maxinteger).
** (The shifts are done in parts to avoid shifting by more than the size
** of an integer. In a worst case, NBM == 113 for long double and
** sizeof(integer) == 32.)
*/
const fn l_intfitsf(i: lua_Integer) -> bool {
    const NBM: lua_Integer = lua_Number::MANTISSA_DIGITS as lua_Integer;
    -(1 << NBM) <= i && i <= (1 << NBM)
}

/*
** Try to convert a value to a float. The float case is already handled
** by the macro 'tonumber'.
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> c_int {
    let mut v = TValue::new();
    if ttisinteger(obj) {
        *n = ivalue(obj) as lua_Number;
        return 1;
    } else if ttisstring(obj) && luaO_str2num(svalue(obj), &mut v) == vslen(obj) + 1 {
        /* string convertible to number? */
        *n = nvalue(&v); /* convert result of 'luaO_str2num' to a float */
        return 1;
    } else {
        return 0; /* conversion failed */
    };
}

/*
** try to convert a value to an integer, rounding according to 'mode':
** mode == 0: accepts only integral values
** mode == 1: takes the floor of the number
** mode == 2: takes the ceil of the number
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_tointeger(
    mut obj: *const TValue,
    p: *mut lua_Integer,
    mode: c_int,
) -> c_int {
    let mut v = TValue::new();
    loop {
        if ttisfloat(obj) {
            let n = fltvalue(obj);
            let mut f = n.floor();
            if n != f {
                /* not an integral value? */
                if mode == 0 {
                    return 0; /* fails if mode demands integral value */
                } else if mode > 1 {
                    /* needs ceil? */
                    /* convert floor to ceil (remember: n != f) */
                    f += 1.0;
                }
            }
            return if f >= (lua_Integer::MIN as lua_Number) && f < -(lua_Integer::MIN as lua_Number)
            {
                *p = f as lua_Integer;
                1
            } else {
                0
            };
        } else if ttisinteger(obj) {
            *p = ivalue(obj);
            return 1;
        } else if ttisstring(obj) && luaO_str2num(svalue(obj), &mut v) == vslen(obj) + 1 {
            /* convert result from 'luaO_str2num' to an integer */
            obj = &v;
            continue;
        }
        break;
    }
    return 0; /* conversion failed */
}

/*
** Try to convert a 'for' limit to an integer, preserving the
** semantics of the loop.
** (The following explanation assumes a non-negative step; it is valid
** for negative steps mutatis mutandis.)
** If the limit can be converted to an integer, rounding down, that is
** it.
** Otherwise, check whether the limit can be converted to a number.  If
** the number is too large, it is OK to set the limit as LUA_MAXINTEGER,
** which means no limit.  If the number is too negative, the loop
** should not run, because any initial integer value is larger than the
** limit. So, it sets the limit to LUA_MININTEGER. 'stopnow' corrects
** the extreme case when the initial value is LUA_MININTEGER, in which
** case the LUA_MININTEGER limit would still run the loop once.
*/
unsafe fn forlimit(
    obj: *const TValue,
    p: *mut lua_Integer,
    step: lua_Integer,
    stopnow: *mut c_int,
) -> c_int {
    *stopnow = 0; /* usually, let loops run */
    if luaV_tointeger(obj, p, if step < 0 { 2 } else { 1 }) == 0 {
        /* not fit in integer? */
        let mut n = 0.; /* try to convert to float */
        if tonumber(obj, &mut n) == 0 {
            /* cannot convert to float? */
            return 0; /* not a number */
        }
        if 0. < n {
            /* if true, float is larger than max integer */
            *p = lua_Integer::MAX;
            if step < 0 {
                *stopnow = 1;
            }
        } else {
            /* float is smaller than min integer */
            *p = lua_Integer::MIN;
            if step >= 0 {
                *stopnow = 1;
            }
        }
    }
    return 1;
}

/*
** Finish the table access 'val = t[key]'.
** if 'slot' is NULL, 't' is not a table; otherwise, 'slot' points to
** t[k] entry (which must be nil).
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_finishget(
    L: *mut lua_State,
    mut t: *const TValue,
    key: *mut TValue,
    val: StkId,
    mut slot: *const TValue,
) {
    let mut tm; /* metamethod */
    for _ in 0..MAXTAGLOOP {
        if slot.is_null() {
            /* 't' is not a table? */
            debug_assert!(!ttistable(t));
            tm = luaT_gettmbyobj(L, t, TM_INDEX);
            if ttisnil(tm) {
                luaG_typeerror(L, t, cstr!("index")); /* no metamethod */
            }
            /* else will try the metamethod */
        } else {
            /* 't' is a table */
            debug_assert!(ttisnil(slot));
            tm = fasttm(L, (*hvalue(t)).metatable, TM_INDEX);
            if tm.is_null() {
                /* no metamethod? */
                setnilvalue(val); /* result is nil */
                return;
            }
            /* else will try the metamethod */
        }
        if ttisfunction(tm) {
            /* is metamethod a function? */
            luaT_callTM(L, tm, t, key, val, 1); /* call it */
            return;
        }
        t = tm; /* else try to access 'tm[key]' */
        if luaV_fastget(L, t, key, &mut slot, luaH_get) {
            /* fast track? */
            setobj(L, val, slot); /* done */
            return;
        }
        /* else repeat (tail call 'luaV_finishget') */
    }
    luaG_runerror(L, cstr!("'__index' chain too long; possible loop"));
}

/*
** Finish a table assignment 't[key] = val'.
** If 'slot' is NULL, 't' is not a table.  Otherwise, 'slot' points
** to the entry 't[key]', or to 'luaO_nilobject' if there is no such
** entry.  (The value at 'slot' must be nil, otherwise 'luaV_fastset'
** would have done the job.)
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_finishset(
    L: *mut lua_State,
    mut t: *const TValue,
    key: *mut TValue,
    val: StkId,
    mut slot: *const TValue,
) {
    for _ in 0..MAXTAGLOOP {
        let tm; /* '__newindex' metamethod */
        if !slot.is_null() {
            /* is 't' a table? */
            let h = hvalue(t); /* save 't' table */
            debug_assert!(ttisnil(slot)); /* old value must be nil */
            tm = fasttm(L, (*h).metatable, TM_NEWINDEX); /* get metamethod */
            if tm.is_null() {
                /* no metamethod? */
                if slot == &luaO_nilobject_ as *const TValue {
                    /* no previous entry? */
                    slot = luaH_newkey(L, h, key); /* create one */
                }
                /* no metamethod and (now) there is an entry with given key */
                setobj(L, slot as *mut TValue, val); /* set its new value */
                invalidateTMcache(h);
                luaC_barrierback(L, h, val);
                return;
            }
            /* else will try the metamethod */
        } else {
            /* not a table; check metamethod */
            tm = luaT_gettmbyobj(L, t, TM_NEWINDEX);
            if ttisnil(tm) {
                luaG_typeerror(L, t, cstr!("index"));
            }
        }
        /* try the metamethod */
        if ttisfunction(tm) {
            luaT_callTM(L, tm, t, key, val, 0);
            return;
        }
        t = tm; /* else repeat assignment over 'tm' */
        if luaV_fastset(L, t, key, &mut slot, luaH_get, val) {
            /* done */
            return;
        }
    }
    luaG_runerror(L, cstr!("'__newindex' chain too long; possible loop"));
}

/*
** Compare two strings 'ls' x 'rs', returning an integer smaller-equal-
** -larger than zero if 'ls' is smaller-equal-larger than 'rs'.
** The code is a little tricky because it allows '\0' in the strings
** and it uses 'strcoll' (to respect locales) for each segments
** of the strings.
*/
unsafe fn l_strcmp(ls: *const TString, rs: *const TString) -> c_int {
    let mut l = getstr(ls);
    let mut ll = tsslen(ls);
    let mut r = getstr(rs);
    let mut lr = tsslen(rs);
    loop {
        /* for each segment */
        let temp = strcoll(l, r);
        if temp != 0 {
            /* not equal? */
            return temp;
        } else {
            /* strings are equal up to a '\0' */
            let mut len = strlen(l); /* index of first '\0' in both strings */
            if len == lr {
                /* 'rs' is finished? */
                return (!(len == ll)) as c_int; /* check 'ls' */
            } else if len == ll {
                /* 'ls' is finished? */
                return -1; /* 'ls' is smaller than 'rs' ('rs' is not finished) */
            }
            /* both strings longer than 'len'; go on comparing after the '\0' */
            len += 1;
            l = l.add(len);
            ll -= len;
            r = r.add(len);
            lr -= len;
        }
    }
}

/*
** Check whether integer 'i' is less than float 'f'. If 'i' has an
** exact representation as a float ('l_intfitsf'), compare numbers as
** floats. Otherwise, if 'f' is outside the range for integers, result
** is trivial. Otherwise, compare them as integers. (When 'i' has no
** float representation, either 'f' is "far away" from 'i' or 'f' has
** no precision left for a fractional part; either way, how 'f' is
** truncated is irrelevant.) When 'f' is NaN, comparisons must result
** in false.
*/
#[inline(always)]
fn LTintfloat(i: lua_Integer, f: lua_Number) -> bool {
    if !l_intfitsf(i) {
        if f >= -(lua_Integer::MIN as lua_Number) {
            /* -minint == maxint + 1 */
            return true; /* f >= maxint + 1 > i */
        } else if f > (lua_Integer::MIN as lua_Number) {
            /* minint < f <= maxint ? */
            return i < (f as lua_Integer); /* compare them as integers */
        } else {
            /* f <= minint <= i (or 'f' is NaN)  -->  not(i < f) */
            return false;
        }
    }
    (i as lua_Number) < f /* compare them as floats */
}

/*
** Check whether integer 'i' is less than or equal to float 'f'.
** See comments on previous function.
*/
#[inline(always)]
fn LEintfloat(i: lua_Integer, f: lua_Number) -> bool {
    if !l_intfitsf(i) {
        if f >= -(lua_Integer::MIN as lua_Number) {
            /* -minint == maxint + 1 */
            return true; /* f >= maxint + 1 > i */
        } else if f >= (lua_Integer::MIN as lua_Number) {
            /* minint <= f <= maxint ? */
            return i <= (f as lua_Integer); /* compare them as integers */
        } else {
            /* f < minint <= i (or 'f' is NaN)  -->  not(i <= f) */
            return false;
        }
    }
    (i as lua_Number) <= f /* compare them as floats */
}

/*
** Return 'l < r', for numbers.
*/
unsafe fn LTnum(l: *const TValue, r: *const TValue) -> bool {
    if ttisinteger(l) {
        let li = ivalue(l);
        if ttisinteger(r) {
            return li < ivalue(r); /* both are integers */
        } else {
            /* 'l' is int and 'r' is float */
            return LTintfloat(li, fltvalue(r)); /* l < r ? */
        }
    } else {
        let lf = fltvalue(l); /* 'l' must be float */
        if ttisfloat(r) {
            return lf < fltvalue(r); /* both are float */
        } else if lf.is_nan() {
            /* 'r' is int and 'l' is float */
            return false; /* NaN < i is always false */
        } else {
            /* without NaN, (l < r)  <-->  not(r <= l) */
            return !LEintfloat(ivalue(r), lf); /* not (r <= l) ? */
        }
    };
}

/*
** Return 'l <= r', for numbers.
*/
unsafe fn LEnum(l: *const TValue, r: *const TValue) -> bool {
    if ttisinteger(l) {
        let li = ivalue(l);
        if ttisinteger(r) {
            return li <= ivalue(r); /* both are integers */
        } else {
            /* 'l' is int and 'r' is float */
            return LEintfloat(li, fltvalue(r)); /* l <= r ? */
        }
    } else {
        let lf = fltvalue(l); /* 'l' must be float */
        if ttisfloat(r) {
            return lf <= fltvalue(r); /* both are float */
        } else if lf.is_nan() {
            /* 'r' is int and 'l' is float */
            return false; /*  NaN <= i is always false */
        } else {
            /* without NaN, (l <= r)  <-->  not(r < l) */
            return !LTintfloat(ivalue(r), lf); /* not (r < l) ? */
        }
    };
}

/*
** Main operation less than; return 'l < r'.
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_lessthan(
    L: *mut lua_State,
    l: *const TValue,
    r: *const TValue,
) -> c_int {
    if ttisnumber(l) && ttisnumber(r) {
        /* both operands are numbers? */
        return LTnum(l, r) as c_int;
    } else if ttisstring(l) && ttisstring(r) {
        /* both are strings? */
        return (l_strcmp(tsvalue(l), tsvalue(r)) < 0) as c_int;
    } else {
        let res = luaT_callorderTM(L, l, r, TM_LT);
        if res < 0 {
            /* no metamethod? */
            luaG_ordererror(L, l, r); /* error */
        }
        return res;
    }
}

/*
** Main operation less than or equal to; return 'l <= r'. If it needs
** a metamethod and there is no '__le', try '__lt', based on
** l <= r iff !(r < l) (assuming a total order). If the metamethod
** yields during this substitution, the continuation has to know
** about it (to negate the result of r<l); bit CIST_LEQ in the call
** status keeps that information.
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_lessequal(
    L: *mut lua_State,
    l: *const TValue,
    r: *const TValue,
) -> c_int {
    if ttisnumber(l) && ttisnumber(r) {
        /* both operands are numbers? */
        return LEnum(l, r) as c_int;
    } else if ttisstring(l) && ttisstring(r) {
        /* both are strings? */
        return (l_strcmp(tsvalue(l), tsvalue(r)) <= 0) as c_int;
    }
    /* try 'le' */
    let res = luaT_callorderTM(L, l, r, TM_LE);
    if res >= 0 {
        return res;
    }
    /* try 'lt': */
    (*(*L).ci).callstatus |= CIST_LEQ; /* mark it is doing 'lt' for 'le' */
    let res = luaT_callorderTM(L, r, l, TM_LT);
    (*(*L).ci).callstatus ^= CIST_LEQ; /* clear mark */
    if res < 0 {
        luaG_ordererror(L, l, r);
    }
    return (res == 0) as c_int;
}

/*
** Main operation for equality of Lua values; return 't1 == t2'.
** L == NULL means raw equality (no metamethods)
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_equalobj(
    L: *mut lua_State,
    t1: *const TValue,
    t2: *const TValue,
) -> c_int {
    if ttype(t1) != ttype(t2) {
        /* not the same variant? */
        if ttnov(t1) != ttnov(t2) || ttnov(t1) != LUA_TNUMBER {
            return 0; /* only numbers can be equal with different variants */
        } else {
            /* two numbers with different variants */
            let mut i1 = 0;
            let mut i2 = 0;
            return (tointeger(t1, &mut i1) != 0 && tointeger(t2, &mut i2) != 0 && i1 == i2)
                as c_int;
        }
    }
    /* values have same type and same variant */
    let mut tm;
    match ttype(t1) {
        LUA_TNIL => return 1,
        LUA_TNUMINT => return (ivalue(t1) == ivalue(t2)) as c_int,
        LUA_TNUMFLT => return (fltvalue(t1) == fltvalue(t2)) as c_int,
        LUA_TBOOLEAN => return (bvalue(t1) == bvalue(t2)) as c_int,
        LUA_TLIGHTUSERDATA => return (pvalue(t1) == pvalue(t2)) as c_int,
        LUA_TLCF => return (fvalue(t1) == fvalue(t2)) as c_int,
        LUA_TSHRSTR => return eqshrstr(tsvalue(t1), tsvalue(t2)) as c_int,
        LUA_TLNGSTR => return luaS_eqlngstr(tsvalue(t1), tsvalue(t2)) as c_int,
        LUA_TUSERDATA => {
            if uvalue(t1) == uvalue(t2) {
                return 1;
            } else if L.is_null() {
                return 0;
            }
            tm = fasttm(L, (*uvalue(t1)).metatable, TM_EQ);
            if tm.is_null() {
                tm = fasttm(L, (*uvalue(t2)).metatable, TM_EQ);
            }
            /* will try TM */
        }
        LUA_TTABLE => {
            if hvalue(t1) == hvalue(t2) {
                return 1;
            } else if L.is_null() {
                return 0;
            }
            tm = fasttm(L, (*hvalue(t1)).metatable, TM_EQ);
            if tm.is_null() {
                tm = fasttm(L, (*hvalue(t2)).metatable, TM_EQ);
            }
        }
        _ => return (gcvalue(t1) == gcvalue(t2)) as c_int,
    }
    if tm.is_null() {
        /* no TM? */
        return 0; /* objects are different */
    }
    luaT_callTM(L, tm, t1, t2, (*L).top, 1); /* call TM */
    return (!l_isfalse((*L).top)) as c_int;
}

/* macro used by 'luaV_concat' to ensure that element at 'o' is a string */
unsafe fn tostring(L: *mut lua_State, o: *mut TValue) -> bool {
    if ttisnumber(o) {
        luaO_tostring(L, o);
        return true;
    }
    ttisstring(o)
}

unsafe fn isemptystr(o: *const TValue) -> bool {
    ttisshrstring(o) && (*tsvalue(o)).shrlen == 0
}

/* copy strings in stack from top - n up to top - 1 to buffer */
unsafe extern "C" fn copy2buff(top: StkId, mut n: c_int, buff: *mut c_char) {
    let mut tl = 0; /* size already copied */
    loop {
        let l = vslen(top.sub(n as usize)); /* length of string being copied */
        memcpy(
            buff.add(tl as usize) as *mut c_void,
            svalue(top.sub(n as usize)) as *const c_void,
            l,
        );
        tl += l;
        n -= 1;
        if n == 0 {
            break;
        }
    }
}

/*
** Main operation for concatenation: concat 'total' values in the stack,
** from 'L->top - total' up to 'L->top - 1'.
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_concat(L: *mut lua_State, mut total: c_int) {
    debug_assert!(total >= 2);
    loop {
        let top = (*L).top;
        let mut n = 2; /* number of elements handled in this pass (at least 2) */
        if !(ttisstring(top.sub(2)) || ttisnumber(top.sub(2))) || !tostring(L, top.sub(1)) {
            luaT_trybinTM(L, top.sub(2), top.sub(1), top.sub(2), TM_CONCAT);
        } else if isemptystr(top.sub(1)) {
            /* second operand is empty? */
            tostring(L, top.sub(2)); /* result is first operand */
        } else if isemptystr(top.sub(2)) {
            /* first operand is an empty string? */
            setobj(L, top.sub(2), top.sub(1)); /* result is second op. */
        } else {
            /* at least two non-empty string values; get as many as possible */
            let mut tl = vslen(top.sub(1));
            let ts;
            /* collect total length and number of strings */
            n = 1;
            while n < total && tostring(L, top.sub(n as usize + 1)) {
                let l = vslen(top.sub(n as usize + 1));
                if l >= usize::MAX - tl {
                    luaG_runerror(L, cstr!("string length overflow"));
                }
                tl += l;
                n += 1;
            }
            if tl <= LUAI_MAXSHORTLEN {
                /* is result a short string? */
                let mut buff = [0; LUAI_MAXSHORTLEN];
                copy2buff(top, n, buff.as_mut_ptr()); /* copy strings to buffer */
                ts = luaS_newlstr(L, buff.as_mut_ptr(), tl);
            } else {
                /* long string; copy strings directly to final result */
                ts = luaS_createlngstrobj(L, tl);
                copy2buff(top, n, getstr(ts));
            }
            setsvalue(L, top.sub(n as usize), ts); /* create result */
        }
        total -= n - 1; /* got 'n' strings to create 1 new */
        (*L).top = ((*L).top).sub(n as usize - 1); /* popped 'n' strings and pushed one */
        /* repeat until only 1 result left */
        if !(total > 1) {
            break;
        }
    }
}

/*
** Main operation 'ra' = #rb'.
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_objlen(L: *mut lua_State, ra: StkId, rb: *const TValue) {
    let tm;
    match ttype(rb) {
        LUA_TTABLE => {
            let h = hvalue(rb);
            tm = fasttm(L, (*h).metatable, TM_LEN);
            if tm.is_null() {
                setivalue(ra, luaH_getn(h) as lua_Integer); /* primitive len */
                return;
            }
            /* metamethod? call it after the switch */
        }
        LUA_TSHRSTR => {
            setivalue(ra, (*tsvalue(rb)).shrlen as lua_Integer);
            return;
        }
        LUA_TLNGSTR => {
            setivalue(ra, (*tsvalue(rb)).u.lnglen as lua_Integer);
            return;
        }
        _ => {
            /* try metamethod */
            tm = luaT_gettmbyobj(L, rb, TM_LEN);
            if ttisnil(tm) {
                /* no metamethod? */
                luaG_typeerror(L, rb, cstr!("get length of"));
            }
        }
    }
    luaT_callTM(L, tm, rb, rb, ra, 1);
}

/*
** Integer division; return 'm // n', that is, floor(m/n).
** C division truncates its result (rounds towards zero).
** 'floor(q) == trunc(q)' when 'q >= 0' or when 'q' is integer,
** otherwise 'floor(q) == trunc(q) - 1'.
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_div(
    L: *mut lua_State,
    m: lua_Integer,
    n: lua_Integer,
) -> lua_Integer {
    if (n as lua_Unsigned).wrapping_add(1) <= 1 {
        /* special cases: -1 or 0 */
        if n == 0 {
            luaG_runerror(L, cstr!("attempt to divide by zero"));
        }
        return 0i64.wrapping_sub(m); /* n == -1; avoid overflow with 0x80000...//-1 */
    } else {
        let mut q = m / n; /* perform C division */
        if m ^ n < 0 && m % n != 0 {
            /* 'm/n' would be negative non-integer? */
            q -= 1; /* correct result for different rounding */
        }
        return q;
    };
}

/*
** Integer modulus; return 'm % n'. (Assume that C '%' with
** negative operands follows C99 behavior. See previous comment
** about luaV_div.)
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_mod(
    L: *mut lua_State,
    m: lua_Integer,
    n: lua_Integer,
) -> lua_Integer {
    if (n as lua_Unsigned).wrapping_add(1) <= 1 {
        /* special cases: -1 or 0 */
        if n == 0 {
            luaG_runerror(L, cstr!("attempt to perform 'n%%0'"));
        }
        return 0; /* m % -1 == 0; avoid overflow with 0x80000...%-1 */
    } else {
        let mut r = m % n;
        if r != 0 && m ^ n < 0 {
            /* 'm/n' would be non-integer negative? */
            r += n; /* correct result for different rounding */
        }
        return r;
    }
}

/*
** Shift left operation. (Shift right just negates 'y'.)
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_shiftl(x: lua_Integer, mut y: lua_Integer) -> lua_Integer {
    if y < 0 {
        y *= -1;
        /* shift right? */
        if y >= lua_Integer::BITS as lua_Integer {
            return 0;
        } else {
            return ((x as lua_Unsigned) >> (y as lua_Unsigned)) as lua_Integer;
        }
    } else {
        /* shift right? */
        if y >= lua_Integer::BITS as lua_Integer {
            return 0;
        } else {
            // TODO: check intop
            return x << y;
        }
    }
}

/*
** check whether cached closure in prototype 'p' may be reused, that is,
** whether there is a cached closure with the same upvalues needed by
** new closure to be created.
*/
unsafe extern "C" fn getcached(
    p: *mut Proto,
    encup: *mut *mut UpVal,
    base: StkId,
) -> *mut LClosure {
    let c = (*p).cache;
    if !c.is_null() {
        /* is there a cached closure? */
        let nup = (*p).sizeupvalues as usize;
        let uv = (*p).upvalues;
        for i in 0..nup {
            /* check whether it has right upvalues */
            let v = if (*uv.add(i)).instack != 0 {
                base.add((*uv.add(i)).idx as usize)
            } else {
                (**encup.add((*uv.add(i as usize)).idx as usize)).v
            };
            if (**((*c).upvals).as_mut_ptr().add(i)).v != v {
                return ptr::null_mut(); /* wrong upvalue; cannot reuse closure */
            }
        }
    }
    return c; /* return cached closure (or NULL if no cached closure) */
}

/*
** create a new Lua closure, push it in the stack, and initialize
** its upvalues. Note that the closure is not cached if prototype is
** already black (which means that 'cache' was already cleared by the
** GC).
*/
unsafe extern "C" fn pushclosure(
    L: *mut lua_State,
    p: *mut Proto,
    encup: *mut *mut UpVal,
    base: StkId,
    ra: StkId,
) {
    let nup = (*p).sizeupvalues;
    let uv = (*p).upvalues;
    let ncl = luaF_newLclosure(L, nup);
    (*ncl).p = p;
    setclLvalue(L, ra, ncl); /* anchor new closure in stack */
    for i in 0..nup as usize {
        /* fill in its upvalues */
        if (*uv.add(i)).instack != 0 {
            /* upvalue refers to local variable? */
            *((*ncl).upvals).as_mut_ptr().add(i) =
                luaF_findupval(L, base.add((*uv.add(i)).idx as usize));
        } else {
            /* get upvalue from enclosing function */
            *((*ncl).upvals).as_mut_ptr().add(i) = *encup.add((*uv.add(i)).idx as usize);
        }
        (**((*ncl).upvals).as_mut_ptr().add(i)).refcount += 1;
        /* new closure is white, so we do not need a barrier here */
    }
    if !isblack!(p) {
        /* cache will not break GC invariant? */
        (*p).cache = ncl; /* save it on cache for reuse */
    }
}

/*
** finish execution of an opcode interrupted by an yield
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_finishOp(L: *mut lua_State) {
    let ci = (*L).ci;
    let base = (*ci).u.l.base;
    let inst = *((*ci).u.l.savedpc).sub(1); /* interrupted instruction */
    let op = GET_OPCODE(inst);
    match op {
        /* finish its execution */
        OP_ADD | OP_SUB | OP_MUL | OP_DIV | OP_IDIV | OP_BAND | OP_BOR | OP_BXOR | OP_SHL
        | OP_SHR | OP_MOD | OP_POW | OP_UNM | OP_BNOT | OP_LEN | OP_GETTABUP | OP_GETTABLE
        | OP_SELF => {
            (*L).top = (*L).top.sub(1);
            setobj(L, base.add(GETARG_A(inst) as usize), (*L).top);
        }
        OP_LE | OP_LT | OP_EQ => {
            let mut res = !l_isfalse((*L).top.sub(1));
            (*L).top = (*L).top.sub(1);
            if (*ci).callstatus & CIST_LEQ != 0 {
                /* "<=" using "<" instead? */
                debug_assert!(op == OP_LE);
                (*ci).callstatus ^= CIST_LEQ; /* clear mark */
                res = !res; /* negate result */
            }
            debug_assert!(GET_OPCODE(*(*ci).u.l.savedpc) == OP_JMP);
            if res != (GETARG_A(inst) != 0) {
                /* condition failed? */
                (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1); /* skip jump instruction */
            }
        }
        OP_CONCAT => {
            let top = ((*L).top).sub(1); /* top when 'luaT_trybinTM' was called */
            let b = GETARG_B(inst); /* first element to concatenate */
            let total = top.sub(1).offset_from(base.add(b as usize)); /* yet to concatenate */
            setobj(L, top.sub(2), top); /* put TM result in proper position */
            if total > 1 {
                /* are there elements to concat? */
                (*L).top = top.sub(1); /* top is one after last element (at top-2) */
                luaV_concat(L, total as c_int); /* concat them (may yield again) */
            }
            /* move final result to final position */
            setobj(
                L,
                (*ci).u.l.base.add(GETARG_A(inst) as usize),
                (*L).top.sub(1),
            );
            (*L).top = (*ci).top; /* restore top */
        }
        OP_TFORCALL => {
            debug_assert!(GET_OPCODE(*(*ci).u.l.savedpc) == OP_TFORLOOP);
            (*L).top = (*ci).top; /* correct top */
        }
        OP_CALL => {
            if GETARG_C(inst) - 1 >= 0 {
                /* nresults >= 0? */
                (*L).top = (*ci).top; /* adjust results */
            }
        }
        OP_TAILCALL | OP_SETTABUP | OP_SETTABLE => {}
        _ => unreachable!(),
    };
}

/*
**
** Function 'luaV_execute': main interpreter loop
**
*/

/*
** some macros for common tasks in 'luaV_execute'
*/
#[inline(always)]
unsafe fn RA(base: *mut TValue, i: Instruction) -> *mut TValue {
    base.add(GETARG_A(i) as usize)
}

#[inline(always)]
unsafe fn RB(base: *mut TValue, i: Instruction) -> *mut TValue {
    debug_assert!(getBMode(GET_OPCODE(i)) == OpArgR);
    base.add(GETARG_B(i) as usize)
}

#[inline(always)]
unsafe fn RC(base: *mut TValue, i: Instruction) -> *mut TValue {
    debug_assert!(getCMode(GET_OPCODE(i)) == OpArgR);
    base.add(GETARG_C(i) as usize)
}

#[inline(always)]
unsafe fn RKB(base: *mut TValue, k: *mut TValue, i: Instruction) -> *mut TValue {
    debug_assert!(getBMode(GET_OPCODE(i)) == OpArgK);
    if ISK(GETARG_B(i) as c_uint) {
        k.add(INDEXK(GETARG_B(i) as c_uint) as usize)
    } else {
        base.add(GETARG_B(i) as usize)
    }
}

#[inline(always)]
unsafe fn RKC(base: *mut TValue, k: *mut TValue, i: Instruction) -> *mut TValue {
    debug_assert!(getCMode(GET_OPCODE(i)) == OpArgK);
    if ISK(GETARG_C(i) as c_uint) {
        k.add(INDEXK(GETARG_C(i) as c_uint) as usize)
    } else {
        base.add(GETARG_C(i) as usize)
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaV_execute(L: *mut lua_State) {
    let mut ci = (*L).ci;
    let mut cl: *mut LClosure;
    let mut k: *mut TValue;
    let mut base: StkId;
    (*ci).callstatus |= CIST_FRESH; /* fresh invocation of 'luaV_execute" */
    'newframe: loop {
        /* reentry point when frame changes (call/return) */
        debug_assert!(ci == (*L).ci);
        cl = clLvalue((*ci).func); // local reference to function's closure
        k = (*(*cl).p).k; // local reference to function's constant table
        base = (*ci).u.l.base; // local copy of function's base

        macro_rules! protect {
            ($f:stmt) => {
                $f
                base = (*ci).u.l.base;
            };
        }

        /* execute a jump instruction */
        macro_rules! dojump {
            ($ci:expr, $i:expr, $e:expr) => {{
                let a = GETARG_A($i);
                if a != 0 {
                    luaF_close(L, (*ci).u.l.base.add(a as usize - 1));
                }
                (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(GETARG_sBx($i) as usize + $e);
            }};
        }

        /*
         ** copy of 'luaV_gettable', but protecting the call to potential
         ** metamethod (which can reallocate the stack)
         */
        macro_rules! gettableProtected {
            ($t:expr, $k:expr, $v:expr) => {
                let mut slot: *const TValue = ptr::null();
                if luaV_fastget(L, $t, $k, &mut slot, luaH_get) {
                    setobj(L, $v, slot);
                } else {
                    protect!(luaV_finishget(L, $t, $k, $v, slot));
                }
            };
        }

        macro_rules! settableProtected {
            ($t:expr, $k:expr, $v:expr) => {
                let mut slot: *const TValue = ptr::null();
                if !luaV_fastset(L, $t, $k, &mut slot, luaH_get, $v) {
                    protect!(luaV_finishset(L, $t, $k, $v, slot));
                }
            };
        }

        macro_rules! checkGC {
            ($c:expr) => {
                luaC_condGC(
                    L,
                    || {
                        (*L).top = $c; /* limit of live values */
                    },
                    || {
                        protect!((*L).top = (*ci).top); /* restore top */
                    },
                );
            };
        }

        // main loop of interpreter
        loop {
            // fetch an instruction and prepare its execution
            let mut i = *(*ci).u.l.savedpc;
            (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1);
            if (*L).hookmask & (LUA_MASKLINE | LUA_MASKCOUNT) != 0 {
                protect!(luaG_traceexec(L));
            }
            let mut ra = RA(base, i); /* WARNING: any stack reallocation invalidates 'ra' */
            debug_assert!(base == (*ci).u.l.base);
            debug_assert!(
                // TODO: check this
                base <= (*L).top && (*L).top.offset_from((*L).stack) < (*L).stacksize as isize
            );

            /* for test instructions, execute the jump instruction that follows it */
            macro_rules! donextjump {
                ($ci:expr) => {
                    i = *(*ci).u.l.savedpc;
                    dojump!(ci, i, 1);
                };
            }

            match GET_OPCODE(i) {
                OP_MOVE => {
                    setobj(L, ra, RB(base, i));
                }
                OP_LOADK => {
                    let rb = k.add(GETARG_Bx(i) as usize);
                    setobj(L, ra, rb);
                }
                OP_LOADKX => {
                    debug_assert!(GET_OPCODE(*(*ci).u.l.savedpc) == OP_EXTRAARG);
                    let rb = k.add(GETARG_Ax(*(*ci).u.l.savedpc) as usize);
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1);
                    setobj(L, ra, rb);
                }
                OP_LOADBOOL => {
                    setbvalue(ra, GETARG_B(i) != 0);
                    if GETARG_C(i) != 0 {
                        /* skip next instruction (if C) */
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1);
                    }
                }
                OP_LOADNIL => {
                    let mut b = GETARG_B(i);
                    loop {
                        setnilvalue(ra);
                        ra = ra.add(1);
                        if b == 0 {
                            break;
                        }
                        b -= 1;
                    }
                }
                OP_GETUPVAL => {
                    let b = GETARG_B(i);
                    setobj(L, ra, (**(*cl).upvals.as_mut_ptr().add(b as usize)).v);
                }
                OP_GETTABUP => {
                    let b = GETARG_B(i);
                    let upval = (**(*cl).upvals.as_mut_ptr().add(b as usize)).v;
                    let rc = RKC(base, k, i);
                    gettableProtected!(upval, rc, ra);
                }
                OP_GETTABLE => {
                    let rb = RB(base, i);
                    let rc = RKC(base, k, i);
                    gettableProtected!(rb, rc, ra);
                }
                OP_SETTABUP => {
                    let a = GETARG_A(i);
                    let upval = (**(*cl).upvals.as_mut_ptr().add(a as usize)).v;
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    settableProtected!(upval, rb, rc);
                }
                OP_SETUPVAL => {
                    let uv = *(*cl).upvals.as_mut_ptr().add(GETARG_B(i) as usize);
                    setobj(L, (*uv).v, ra);
                    luaC_upvalbarrier(L, uv);
                }
                OP_SETTABLE => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    settableProtected!(ra, rb, rc);
                }
                OP_NEWTABLE => {
                    let b = GETARG_B(i);
                    let c = GETARG_C(i);
                    let t = luaH_new(L);
                    sethvalue(L, ra, t);
                    if b != 0 || c != 0 {
                        luaH_resize(L, t, luaO_fb2int(b) as c_uint, luaO_fb2int(c) as c_uint)
                    }
                    checkGC!(ra.add(1));
                }
                OP_SELF => {
                    let rb = RB(base, i);
                    let rc = RKC(base, k, i);
                    let key = tsvalue(rc); /* key must be a string */
                    setobj(L, ra.add(1), rb);
                    let mut aux: *const TValue = ptr::null();
                    if luaV_fastget_s(L, rb, key, &mut aux, luaH_getstr) {
                        setobj(L, ra, aux);
                    } else {
                        protect!(luaV_finishget(L, rb, rc, ra, aux));
                    }
                }
                OP_ADD => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut nb, mut nc) = (0., 0.);
                    if ttisinteger(rb) && ttisinteger(rc) {
                        let ib = ivalue(rb);
                        let ic = ivalue(rc);
                        setivalue(ra, ib.wrapping_add(ic));
                    } else if tonumber(rb, &mut nb) != 0 && tonumber(rc, &mut nc) != 0 {
                        setfltvalue(ra, nb + nc);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_ADD));
                    }
                }
                OP_SUB => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut nb, mut nc) = (0., 0.);
                    if ttisinteger(rb) && ttisinteger(rc) {
                        let ib = ivalue(rb);
                        let ic = ivalue(rc);
                        setivalue(ra, ib.wrapping_sub(ic));
                    } else if tonumber(rb, &mut nb) != 0 && tonumber(rc, &mut nc) != 0 {
                        setfltvalue(ra, nb - nc);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_SUB));
                    }
                }
                OP_MUL => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut nb, mut nc) = (0., 0.);
                    if ttisinteger(rb) && ttisinteger(rc) {
                        let ib = ivalue(rb);
                        let ic = ivalue(rc);
                        setivalue(ra, ib.wrapping_mul(ic));
                    } else if tonumber(rb, &mut nb) != 0 && tonumber(rc, &mut nc) != 0 {
                        setfltvalue(ra, nb * nc);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_MUL));
                    }
                }
                OP_DIV => {
                    /* float division (always with floats) */
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut nb, mut nc) = (0., 0.);
                    if tonumber(rb, &mut nb) != 0 && tonumber(rc, &mut nc) != 0 {
                        setfltvalue(ra, nb / nc);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_DIV));
                    }
                }
                OP_BAND => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut ib, mut ic) = (0, 0);
                    if tointeger(rb, &mut ib) != 0 && tointeger(rc, &mut ic) != 0 {
                        setivalue(ra, ib & ic);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_BAND));
                    }
                }
                OP_BOR => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut ib, mut ic) = (0, 0);
                    if tointeger(rb, &mut ib) != 0 && tointeger(rc, &mut ic) != 0 {
                        setivalue(ra, ib | ic);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_BOR));
                    }
                }
                OP_BXOR => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut ib, mut ic) = (0, 0);
                    if tointeger(rb, &mut ib) != 0 && tointeger(rc, &mut ic) != 0 {
                        setivalue(ra, ib ^ ic);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_BXOR));
                    }
                }
                OP_SHL => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut ib, mut ic) = (0, 0);
                    if tointeger(rb, &mut ib) != 0 && tointeger(rc, &mut ic) != 0 {
                        setivalue(ra, luaV_shiftl(ib, ic));
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_SHL));
                    }
                }
                OP_SHR => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut ib, mut ic) = (0, 0);
                    if tointeger(rb, &mut ib) != 0 && tointeger(rc, &mut ic) != 0 {
                        setivalue(ra, luaV_shiftl(ib, -ic));
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_SHR));
                    }
                }
                OP_MOD => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut nb, mut nc) = (0., 0.);
                    if ttisinteger(rb) && ttisinteger(rc) {
                        let ib = ivalue(rb);
                        let ic = ivalue(rc);
                        setivalue(ra, luaV_mod(L, ib, ic));
                    } else if tonumber(rb, &mut nb) != 0 && tonumber(rc, &mut nc) != 0 {
                        let mut m = nb % nc;
                        if m * nc < 0. {
                            m += nc;
                        }
                        setfltvalue(ra, m);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_MOD));
                    }
                }
                OP_IDIV => {
                    /* floor division */
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut nb, mut nc) = (0., 0.);
                    if ttisinteger(rb) && ttisinteger(rc) {
                        let ib = ivalue(rb);
                        let ic = ivalue(rc);
                        setivalue(ra, luaV_div(L, ib, ic));
                    } else if tonumber(rb, &mut nb) != 0 && tonumber(rc, &mut nc) != 0 {
                        setfltvalue(ra, (nb / nc).floor());
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_IDIV));
                    }
                }
                OP_POW => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    let (mut nb, mut nc) = (0., 0.);
                    if tonumber(rb, &mut nb) != 0 && tonumber(rc, &mut nc) != 0 {
                        setfltvalue(ra, nb.powf(nc));
                    } else {
                        protect!(luaT_trybinTM(L, rb, rc, ra, TM_POW));
                    }
                }
                OP_UNM => {
                    let rb = RB(base, i);
                    let mut nb = 0.;
                    if ttisinteger(rb) {
                        let ib = ivalue(rb);
                        setivalue(ra, 0i64.wrapping_sub(ib));
                    } else if tonumber(rb, &mut nb) != 0 {
                        setfltvalue(ra, -nb);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rb, ra, TM_UNM));
                    }
                }
                OP_BNOT => {
                    let rb = RB(base, i);
                    let mut ib = 0;
                    if tointeger(rb, &mut ib) != 0 {
                        setivalue(ra, !ib);
                    } else {
                        protect!(luaT_trybinTM(L, rb, rb, ra, TM_BNOT));
                    }
                }
                OP_NOT => {
                    let rb = RB(base, i);
                    let res = l_isfalse(rb); /* next assignment may change this value */
                    setbvalue(ra, res);
                }
                OP_LEN => {
                    protect!(luaV_objlen(L, ra, RB(base, i)));
                }
                OP_CONCAT => {
                    let b = GETARG_B(i);
                    let c = GETARG_C(i);
                    (*L).top = base.add(c as usize + 1); /* mark the end of concat operands */
                    protect!(luaV_concat(L, c - b + 1));
                    let ra = RA(base, i); /* 'luaV_concat' may invoke TMs and move the stack */
                    let rb = base.add(b as usize);
                    setobj(L, ra, rb);
                    checkGC!(if ra >= rb { ra.add(1) } else { rb });
                    (*L).top = (*ci).top; /* restore top */
                }
                OP_JMP => {
                    dojump!(ci, i, 0);
                }
                OP_EQ => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    protect!(if luaV_equalobj(L, rb, rc) != GETARG_A(i) {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1);
                    } else {
                        donextjump!(ci);
                    });
                }
                OP_LT => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    protect!(if luaV_lessthan(L, rb, rc) != GETARG_A(i) {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1);
                    } else {
                        donextjump!(ci);
                    });
                }
                OP_LE => {
                    let rb = RKB(base, k, i);
                    let rc = RKC(base, k, i);
                    protect!(if luaV_lessequal(L, rb, rc) != GETARG_A(i) {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1);
                    } else {
                        donextjump!(ci);
                    });
                }
                OP_TEST => {
                    let x = if GETARG_C(i) != 0 {
                        l_isfalse(ra)
                    } else {
                        !l_isfalse(ra)
                    };
                    if x {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1);
                    } else {
                        donextjump!(ci);
                    }
                }
                OP_TESTSET => {
                    let rb = RB(base, i);
                    let x = if GETARG_C(i) != 0 {
                        l_isfalse(rb)
                    } else {
                        !l_isfalse(rb)
                    };
                    if x {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1);
                    } else {
                        setobj(L, ra, rb);
                        donextjump!(ci);
                    }
                }
                OP_CALL => {
                    let b = GETARG_B(i);
                    let nresults = GETARG_C(i) - 1;
                    if b != 0 {
                        (*L).top = ra.add(b as usize);
                        /* else previous instruction set top */
                    }
                    if luaD_precall(L, ra, nresults) != 0 {
                        /* C function? */
                        if nresults >= 0 {
                            (*L).top = (*ci).top; /* adjust results */
                        }
                        protect!({}); /* update 'base' */
                    } else {
                        /* Lua function */
                        ci = (*L).ci;
                        continue 'newframe; /* restart luaV_execute over new Lua function */
                    }
                }
                OP_TAILCALL => {
                    let b = GETARG_B(i);
                    if b != 0 {
                        (*L).top = ra.add(b as usize);
                        /* else previous instruction set top */
                    }
                    debug_assert!(GETARG_C(i) - 1 == LUA_MULTRET);
                    if luaD_precall(L, ra, LUA_MULTRET) != 0 {
                        /* C function? */
                        protect!({}); /* update 'base' */
                    } else {
                        /* tail call: put called frame (n) in place of caller one (o) */
                        let nci = (*L).ci; /* called frame */
                        let oci = (*nci).previous; /* caller frame */
                        let nfunc = (*nci).func; /* called function */
                        let ofunc = (*oci).func; /* caller function */
                        /* last stack slot filled by 'precall' */
                        let lim = ((*nci).u.l.base).add((*getproto(nfunc)).numparams as usize);
                        /* close all upvalues from previous call */
                        if (*(*cl).p).sizep > 0 {
                            luaF_close(L, (*oci).u.l.base);
                        }
                        /* move new frame into old one */
                        let mut aux = 0;
                        while nfunc.add(aux) < lim {
                            setobj(L, ofunc.add(aux), nfunc.add(aux));
                            aux += 1;
                        }
                        (*oci).u.l.base = ofunc.offset(((*nci).u.l.base).offset_from(nfunc)); /* correct base */
                        /* correct top */
                        (*L).top = ofunc.offset(((*L).top).offset_from(nfunc));
                        (*oci).top = (*L).top;
                        (*oci).u.l.savedpc = (*nci).u.l.savedpc;
                        (*oci).callstatus |= CIST_TAIL; /* function was tail called */
                        /* remove new frame */
                        (*L).ci = oci;
                        ci = (*L).ci;
                        debug_assert!(
                            (*L).top
                                == (*oci)
                                    .u
                                    .l
                                    .base
                                    .add((*getproto(ofunc)).maxstacksize as usize)
                        );
                        continue 'newframe; /* restart luaV_execute over new Lua function */
                    }
                }
                OP_RETURN => {
                    let mut b = GETARG_B(i);
                    if (*(*cl).p).sizep > 0 {
                        luaF_close(L, base);
                    }
                    let x = if b != 0 {
                        b - 1
                    } else {
                        (*L).top.offset_from(ra) as c_int
                    };
                    b = luaD_poscall(L, ci, ra, x);
                    if (*ci).callstatus & CIST_FRESH != 0 {
                        /* local 'ci' still from callee */
                        return; /* external invocation: return */
                    }
                    /* invocation via reentry: continue execution */
                    ci = (*L).ci;
                    if b != 0 {
                        (*L).top = (*ci).top;
                    }
                    debug_assert!(isLua(ci));
                    debug_assert!(GET_OPCODE(*(*ci).u.l.savedpc.sub(1)) == OP_CALL);
                    continue 'newframe; /* restart luaV_execute over new Lua function */
                }
                OP_FORLOOP => {
                    if ttisinteger(ra) {
                        /* integer loop? */
                        let step = ivalue(ra.add(2));
                        let idx = (ivalue(ra) as lua_Unsigned).wrapping_add(step as lua_Unsigned)
                            as lua_Integer; /* increment index */
                        let limit = ivalue(ra.add(1));
                        let x = if 0 < step { idx <= limit } else { limit <= idx };
                        if x {
                            /* jump back */
                            (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(GETARG_sBx(i) as isize);
                            chgivalue(ra, idx); /* update internal index... */
                            setivalue(ra.add(3), idx); /* ...and external index */
                        }
                    } else {
                        /* floating loop */
                        let step = fltvalue(ra.add(2));
                        let idx = fltvalue(ra) + step; /* inc. index */
                        let limit = fltvalue(ra.add(1));
                        let x = if 0. < step {
                            idx <= limit
                        } else {
                            limit <= idx
                        };
                        if x {
                            /* jump back */
                            (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(GETARG_sBx(i) as isize);
                            chgfltvalue(ra, idx); /* update internal index... */
                            setfltvalue(ra.add(3), idx); /* ...and external index */
                        }
                    }
                }
                OP_FORPREP => {
                    let init = ra;
                    let plimit = ra.add(1);
                    let pstep = ra.add(2);
                    let (mut ilimit, mut stopnow) = (0, 0);
                    if ttisinteger(init)
                        && ttisinteger(pstep)
                        && forlimit(plimit, &mut ilimit, ivalue(pstep), &mut stopnow) != 0
                    {
                        /* all values are integer */
                        let initv = if stopnow != 0 { 0 } else { ivalue(init) };
                        setivalue(plimit, ilimit);
                        setivalue(
                            init,
                            (initv as lua_Unsigned).wrapping_sub(ivalue(pstep) as lua_Unsigned)
                                as lua_Integer,
                        );
                    } else {
                        /* try making all values floats */
                        let mut ninit = 0.;
                        let mut nlimit = 0.;
                        let mut nstep = 0.;
                        if tonumber(plimit, &mut nlimit) == 0 {
                            luaG_runerror(L, cstr!("'for' limit must be a number"));
                        }
                        setfltvalue(plimit, nlimit);
                        if tonumber(pstep, &mut nstep) == 0 {
                            luaG_runerror(L, cstr!("'for' step must be a number"));
                        }
                        setfltvalue(pstep, nstep);
                        if tonumber(init, &mut ninit) == 0 {
                            luaG_runerror(L, cstr!("'for' initial value must be a number"));
                        }
                        setfltvalue(init, ninit - nstep);
                    }
                    (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(GETARG_sBx(i) as isize);
                }
                OP_TFORCALL => {
                    let cb = ra.add(3); /* call base */
                    setobj(L, cb.add(2), ra.add(2));
                    setobj(L, cb.add(1), ra.add(1));
                    setobj(L, cb, ra);
                    (*L).top = cb.add(3); /* func. + 2 args (state and index) */
                    protect!(luaD_call(L, cb, GETARG_C(i)));
                    (*L).top = (*ci).top;
                    i = *((*ci).u.l.savedpc); /* go to next instruction */
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1);
                    ra = RA(base, i);
                    debug_assert!(GET_OPCODE(i) == OP_TFORLOOP);
                    if !ttisnil(ra.add(1)) {
                        /* continue loop? */
                        setobj(L, ra, ra.add(1)); /* save control variable */
                        /* jump back */
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(GETARG_sBx(i) as isize);
                    }
                }
                OP_TFORLOOP => {
                    if !ttisnil(ra.add(1)) {
                        /* continue loop? */
                        setobj(L, ra, ra.add(1)); /* save control variable */
                        /* jump back */
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(GETARG_sBx(i) as isize);
                    }
                }
                OP_SETLIST => {
                    let mut n = GETARG_B(i);
                    let mut c = GETARG_C(i);
                    if n == 0 {
                        n = (*L).top.offset_from(ra) as c_int - 1;
                    };
                    if c == 0 {
                        debug_assert!(GET_OPCODE(*(*ci).u.l.savedpc) == OP_EXTRAARG);
                        c = GETARG_Ax(*(*ci).u.l.savedpc);
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.add(1)
                    }
                    let h = hvalue(ra);
                    let mut last = ((c - 1) * LFIELDS_PER_FLUSH as c_int) + n;
                    if last as c_uint > (*h).sizearray {
                        /* needs more space? */
                        luaH_resizearray(L, h, last as c_uint); /* preallocate it at once */
                    }
                    while n > 0 {
                        let val = ra.add(n as usize);
                        luaH_setint(L, h, last as lua_Integer, val);
                        last -= 1;
                        luaC_barrierback(L, h, val);
                        n -= 1;
                    }
                    (*L).top = (*ci).top; /* correct top (in case of previous open call) */
                }
                OP_CLOSURE => {
                    let p = *((*(*cl).p).p).offset(GETARG_Bx(i) as isize);
                    let ncl = getcached(p, ((*cl).upvals).as_mut_ptr(), base); /* cached closure */
                    if ncl.is_null() {
                        /* no match? */
                        /* create a new one */
                        pushclosure(L, p, ((*cl).upvals).as_mut_ptr(), base, ra);
                    } else {
                        setclLvalue(L, ra, ncl); /* push cashed closure */
                    }
                    checkGC!(ra.add(1));
                }
                OP_VARARG => {
                    let mut b = GETARG_B(i) - 1; /* required results */
                    let mut n =
                        (base.offset_from((*ci).func) - (*(*cl).p).numparams as isize - 1) as c_int;
                    if n < 0 {
                        /* less arguments than parameters? */
                        n = 0; /* no vararg arguments */
                    }
                    if b < 0 {
                        /* B == 0? */
                        b = n; /* get all var. arguments */
                        protect!(luaD_checkstack(L, n));
                        ra = RA(base, i); /* previous call may change the stack */
                        (*L).top = ra.add(n as usize);
                    }
                    let mut j = 0;
                    while j < b && j < n {
                        setobj(L, ra.add(j as usize), base.sub((n - j) as usize));
                        j += 1;
                    }
                    while j < b {
                        /* complete required results with nil */
                        setnilvalue(ra.add(j as usize));
                        j += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
