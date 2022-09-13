/*
** $Id: lapi.c,v 2.259.1.2 2017/12/06 18:35:12 roberto Exp $
** Lua API
** See Copyright Notice in lua.h
*/

#define lapi_c
#define LUA_CORE

#include "lprefix.h"


#include <stdarg.h>
#include <string.h>

#include "lua.h"

#include "lapi.h"
#include "ldebug.h"
#include "ldo.h"
#include "lfunc.h"
#include "lgc.h"
#include "lmem.h"
#include "lobject.h"
#include "lstate.h"
#include "lstring.h"
#include "ltable.h"
#include "ltm.h"
#include "lundump.h"
#include "lvm.h"



const char lua_ident[] =
  "$LuaVersion: " LUA_COPYRIGHT " $"
  "$LuaAuthors: " LUA_AUTHORS " $";


/* value at a non-valid index */
#define NONVALIDVALUE		cast(TValue *, luaO_nilobject)

/* corresponding test */
#define isvalid(o)	((o) != luaO_nilobject)

/* test for pseudo index */
#define ispseudo(i)		((i) <= LUA_REGISTRYINDEX)

/* test for upvalue */
#define isupvalue(i)		((i) < LUA_REGISTRYINDEX)

/* test for valid but not pseudo index */
#define isstackindex(i, o)	(isvalid(o) && !ispseudo(i))

#define api_checkvalidindex(l,o)  api_check(l, isvalid(o), "invalid index")

#define api_checkstackindex(l, i, o)  \
	api_check(l, isstackindex(i, o), "index not in the stack")


// FIXME - static 
TValue *index2addr (lua_State *L, int idx) {
  CallInfo *ci = L->ci;
  if (idx > 0) {
    TValue *o = ci->func + idx;
    api_check(L, idx <= ci->top - (ci->func + 1), "unacceptable index");
    if (o >= L->top) return NONVALIDVALUE;
    else return o;
  }
  else if (!ispseudo(idx)) {  /* negative index */
    api_check(L, idx != 0 && -idx <= L->top - (ci->func + 1), "invalid index");
    return L->top + idx;
  }
  else if (idx == LUA_REGISTRYINDEX)
    return &G(L)->l_registry;
  else {  /* upvalues */
    idx = LUA_REGISTRYINDEX - idx;
    api_check(L, idx <= MAXUPVAL + 1, "upvalue index too large");
    if (ttislcf(ci->func))  /* light C function? */
      return NONVALIDVALUE;  /* it has no upvalues */
    else {
      CClosure *func = clCvalue(ci->func);
      return (idx <= func->nupvalues) ? &func->upvalue[idx-1] : NONVALIDVALUE;
    }
  }
}










LUA_API const lua_Number *lua_version (lua_State *L) {
  static const lua_Number version = LUA_VERSION_NUM;
  if (L == NULL) return &version;
  else return G(L)->version;
}



/*
** basic stack manipulation
*/








/*
** access functions (stack -> C)
*/





/*
** push functions (C -> stack)
*/









/*
** 'load' and 'call' functions (run Lua code)
*/









/*
** Garbage-collection function
*/

LUA_API int lua_gc (lua_State *L, int what, int data) {
  int res = 0;
  global_State *g;
  lua_lock(L);
  g = G(L);
  switch (what) {
    case LUA_GCSTOP: {
      g->gcrunning = 0;
      break;
    }
    case LUA_GCRESTART: {
      luaE_setdebt(g, 0);
      g->gcrunning = 1;
      break;
    }
    case LUA_GCCOLLECT: {
      luaC_fullgc(L, 0);
      break;
    }
    case LUA_GCCOUNT: {
      /* GC values are expressed in Kbytes: #bytes/2^10 */
      res = cast_int(gettotalbytes(g) >> 10);
      break;
    }
    case LUA_GCCOUNTB: {
      res = cast_int(gettotalbytes(g) & 0x3ff);
      break;
    }
    case LUA_GCSTEP: {
      l_mem debt = 1;  /* =1 to signal that it did an actual step */
      lu_byte oldrunning = g->gcrunning;
      g->gcrunning = 1;  /* allow GC to run */
      if (data == 0) {
        luaE_setdebt(g, -GCSTEPSIZE);  /* to do a "small" step */
        luaC_step(L);
      }
      else {  /* add 'data' to total debt */
        debt = cast(l_mem, data) * 1024 + g->GCdebt;
        luaE_setdebt(g, debt);
        luaC_checkGC(L);
      }
      g->gcrunning = oldrunning;  /* restore previous state */
      if (debt > 0 && g->gcstate == GCSpause)  /* end of cycle? */
        res = 1;  /* signal it */
      break;
    }
    case LUA_GCSETPAUSE: {
      res = g->gcpause;
      g->gcpause = data;
      break;
    }
    case LUA_GCSETSTEPMUL: {
      res = g->gcstepmul;
      if (data < 40) data = 40;  /* avoid ridiculous low values (and 0) */
      g->gcstepmul = data;
      break;
    }
    case LUA_GCISRUNNING: {
      res = g->gcrunning;
      break;
    }
    default: res = -1;  /* invalid option */
  }
  lua_unlock(L);
  return res;
}



/*
** miscellaneous functions
*/


LUA_API int lua_error (lua_State *L) {
  lua_lock(L);
  api_checknelems(L, 1);
  luaG_errormsg(L);
  /* code unreachable; will unlock when control actually leaves the kernel */
  return 0;  /* to avoid warnings */
}


LUA_API int lua_next (lua_State *L, int idx) {
  StkId t;
  int more;
  lua_lock(L);
  t = index2addr(L, idx);
  api_check(L, ttistable(t), "table expected");
  more = luaH_next(L, hvalue(t), L->top - 1);
  if (more) {
    api_incr_top(L);
  }
  else  /* no more elements */
    L->top -= 1;  /* remove key */
  lua_unlock(L);
  return more;
}


LUA_API void lua_concat (lua_State *L, int n) {
  lua_lock(L);
  api_checknelems(L, n);
  if (n >= 2) {
    luaV_concat(L, n);
  }
  else if (n == 0) {  /* push empty string */
    setsvalue2s(L, L->top, luaS_newlstr(L, "", 0));
    api_incr_top(L);
  }
  /* else n == 1; nothing to do */
  luaC_checkGC(L);
  lua_unlock(L);
}


LUA_API void lua_len (lua_State *L, int idx) {
  StkId t;
  lua_lock(L);
  t = index2addr(L, idx);
  luaV_objlen(L, L->top, t);
  api_incr_top(L);
  lua_unlock(L);
}


LUA_API lua_Alloc lua_getallocf (lua_State *L, void **ud) {
  lua_Alloc f;
  lua_lock(L);
  if (ud) *ud = G(L)->ud;
  f = G(L)->frealloc;
  lua_unlock(L);
  return f;
}


LUA_API void lua_setallocf (lua_State *L, lua_Alloc f, void *ud) {
  lua_lock(L);
  G(L)->ud = ud;
  G(L)->frealloc = f;
  lua_unlock(L);
}


LUA_API void *lua_newuserdata (lua_State *L, size_t size) {
  Udata *u;
  lua_lock(L);
  u = luaS_newudata(L, size);
  setuvalue(L, L->top, u);
  api_incr_top(L);
  luaC_checkGC(L);
  lua_unlock(L);
  return getudatamem(u);
}



static const char *aux_upvalue (StkId fi, int n, TValue **val,
                                CClosure **owner, UpVal **uv) {
  switch (ttype(fi)) {
    case LUA_TCCL: {  /* C closure */
      CClosure *f = clCvalue(fi);
      if (!(1 <= n && n <= f->nupvalues)) return NULL;
      *val = &f->upvalue[n-1];
      if (owner) *owner = f;
      return "";
    }
    case LUA_TLCL: {  /* Lua closure */
      LClosure *f = clLvalue(fi);
      TString *name;
      Proto *p = f->p;
      if (!(1 <= n && n <= p->sizeupvalues)) return NULL;
      *val = f->upvals[n-1]->v;
      if (uv) *uv = f->upvals[n - 1];
      name = p->upvalues[n-1].name;
      return (name == NULL) ? "(*no name)" : getstr(name);
    }
    default: return NULL;  /* not a closure */
  }
}


LUA_API const char *lua_getupvalue (lua_State *L, int funcindex, int n) {
  const char *name;
  TValue *val = NULL;  /* to avoid warnings */
  lua_lock(L);
  name = aux_upvalue(index2addr(L, funcindex), n, &val, NULL, NULL);
  if (name) {
    setobj2s(L, L->top, val);
    api_incr_top(L);
  }
  lua_unlock(L);
  return name;
}


LUA_API const char *lua_setupvalue (lua_State *L, int funcindex, int n) {
  const char *name;
  TValue *val = NULL;  /* to avoid warnings */
  CClosure *owner = NULL;
  UpVal *uv = NULL;
  StkId fi;
  lua_lock(L);
  fi = index2addr(L, funcindex);
  api_checknelems(L, 1);
  name = aux_upvalue(fi, n, &val, &owner, &uv);
  if (name) {
    L->top--;
    setobj(L, val, L->top);
    if (owner) { luaC_barrier(L, owner, L->top); }
    else if (uv) { luaC_upvalbarrier(L, uv); }
  }
  lua_unlock(L);
  return name;
}


static UpVal **getupvalref (lua_State *L, int fidx, int n) {
  LClosure *f;
  StkId fi = index2addr(L, fidx);
  api_check(L, ttisLclosure(fi), "Lua function expected");
  f = clLvalue(fi);
  api_check(L, (1 <= n && n <= f->p->sizeupvalues), "invalid upvalue index");
  return &f->upvals[n - 1];  /* get its upvalue pointer */
}


LUA_API void *lua_upvalueid (lua_State *L, int fidx, int n) {
  StkId fi = index2addr(L, fidx);
  switch (ttype(fi)) {
    case LUA_TLCL: {  /* lua closure */
      return *getupvalref(L, fidx, n);
    }
    case LUA_TCCL: {  /* C closure */
      CClosure *f = clCvalue(fi);
      api_check(L, 1 <= n && n <= f->nupvalues, "invalid upvalue index");
      return &f->upvalue[n - 1];
    }
    default: {
      api_check(L, 0, "closure expected");
      return NULL;
    }
  }
}


LUA_API void lua_upvaluejoin (lua_State *L, int fidx1, int n1,
                                            int fidx2, int n2) {
  UpVal **up1 = getupvalref(L, fidx1, n1);
  UpVal **up2 = getupvalref(L, fidx2, n2);
  if (*up1 == *up2)
    return;
  luaC_upvdeccount(L, *up1);
  *up1 = *up2;
  (*up1)->refcount++;
  if (upisopen(*up1)) (*up1)->u.open.touched = 1;
  luaC_upvalbarrier(L, *up1);
}


