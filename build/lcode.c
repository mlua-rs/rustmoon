/*
** $Id: lcode.c,v 2.112.1.1 2017/04/19 17:20:42 roberto Exp $
** Code generator for Lua
** See Copyright Notice in lua.h
*/

#define lcode_c
#define LUA_CORE

#include "lprefix.h"


#include <math.h>
#include <stdlib.h>

#include "lua.h"

#include "lcode.h"
#include "ldebug.h"
#include "ldo.h"
#include "lgc.h"
#include "llex.h"
#include "lmem.h"
#include "lobject.h"
#include "lopcodes.h"
#include "lparser.h"
#include "lstring.h"
#include "ltable.h"
#include "lvm.h"


/* Maximum number of registers in a Lua function (must fit in 8 bits) */
#define MAXREGS		255


#define hasjumps(e)	((e)->t != (e)->f)
extern int tonumeral(const expdesc *e, TValue *v);
void luaK_nil (FuncState *fs, int from, int n);
extern int getjump (FuncState *fs, int pc);
extern void fixjump (FuncState *fs, int pc, int dest);
void luaK_concat (FuncState *fs, int *l1, int l2);
int luaK_jump (FuncState *fs);
void luaK_ret (FuncState *fs, int first, int nret);
extern int condjump (FuncState *fs, OpCode op, int A, int B, int C);
int luaK_getlabel (FuncState *fs);
extern Instruction *getjumpcontrol (FuncState *fs, int pc);
extern int patchtestreg (FuncState *fs, int node, int reg);
extern void removevalues (FuncState *fs, int list);
extern void patchlistaux (FuncState *fs, int list, int vtarget, int reg, int dtarget);
extern void dischargejpc (FuncState *fs);
void luaK_patchtohere (FuncState *fs, int list);
void luaK_patchlist (FuncState *fs, int list, int target);
void luaK_patchclose (FuncState *fs, int list, int level);
extern int luaK_code (FuncState *fs, Instruction i);
int luaK_codeABC (FuncState *fs, OpCode o, int a, int b, int c);
int luaK_codeABx (FuncState *fs, OpCode o, int a, unsigned int bc);
extern int codeextraarg (FuncState *fs, int a);
int luaK_codek (FuncState *fs, int reg, int k);
void luaK_checkstack (FuncState *fs, int n);
void luaK_reserveregs (FuncState *fs, int n);
extern void freereg (FuncState *fs, int reg);
extern void freeexp (FuncState *fs, expdesc *e);
extern void freeexps (FuncState *fs, expdesc *e1, expdesc *e2);
extern int addk (FuncState *fs, TValue *key, TValue *v);
int luaK_stringK (FuncState *fs, TString *s);
int luaK_intK (FuncState *fs, lua_Integer n);
extern int luaK_numberK (FuncState *fs, lua_Number r);
extern int boolK (FuncState *fs, int b);
extern int nilK (FuncState *fs);
void luaK_setreturns (FuncState *fs, expdesc *e, int nresults);
void luaK_setoneret (FuncState *fs, expdesc *e);
void luaK_dischargevars (FuncState *fs, expdesc *e);
extern void discharge2reg (FuncState *fs, expdesc *e, int reg);
void discharge2anyreg (FuncState *fs, expdesc *e);
int code_loadbool (FuncState *fs, int A, int b, int jump);
int need_value (FuncState *fs, int list);
void exp2reg (FuncState *fs, expdesc *e, int reg);
void luaK_exp2nextreg (FuncState *fs, expdesc *e);
int luaK_exp2anyreg (FuncState *fs, expdesc *e);
void luaK_exp2anyregup (FuncState *fs, expdesc *e);
void luaK_exp2val (FuncState *fs, expdesc *e);
int luaK_exp2RK (FuncState *fs, expdesc *e);
void luaK_storevar (FuncState *fs, expdesc *var, expdesc *ex);
void luaK_self (FuncState *fs, expdesc *e, expdesc *key);
extern void negatecondition (FuncState *fs, expdesc *e);
extern int jumponcond (FuncState *fs, expdesc *e, int cond);
void luaK_goiftrue (FuncState *fs, expdesc *e);
void luaK_goiffalse (FuncState *fs, expdesc *e);
void codenot (FuncState *fs, expdesc *e);
void luaK_indexed (FuncState *fs, expdesc *t, expdesc *k);
int validop (int op, TValue *v1, TValue *v2);
int constfolding (FuncState *fs, int op, expdesc *e1,
                                                const expdesc *e2);
void codeunexpval (FuncState *fs, OpCode op, expdesc *e, int line);
void codebinexpval (FuncState *fs, OpCode op,
                           expdesc *e1, expdesc *e2, int line);
void codecomp (FuncState *fs, BinOpr opr, expdesc *e1, expdesc *e2);
void luaK_prefix (FuncState *fs, UnOpr op, expdesc *e, int line);
void luaK_infix (FuncState *fs, BinOpr op, expdesc *v);
void luaK_posfix (FuncState *fs, BinOpr op,
                  expdesc *e1, expdesc *e2, int line);


/*
** Change line information associated with current position.
*/
void luaK_fixline (FuncState *fs, int line) {
  fs->f->lineinfo[fs->pc - 1] = line;
}


/*
** Emit a SETLIST instruction.
** 'base' is register that keeps table;
** 'nelems' is #table plus those to be stored now;
** 'tostore' is number of values (in registers 'base + 1',...) to add to
** table (or LUA_MULTRET to add up to stack top).
*/
void luaK_setlist (FuncState *fs, int base, int nelems, int tostore) {
  int c =  (nelems - 1)/LFIELDS_PER_FLUSH + 1;
  int b = (tostore == LUA_MULTRET) ? 0 : tostore;
  lua_assert(tostore != 0 && tostore <= LFIELDS_PER_FLUSH);
  if (c <= MAXARG_C)
    luaK_codeABC(fs, OP_SETLIST, base, b, c);
  else if (c <= MAXARG_Ax) {
    luaK_codeABC(fs, OP_SETLIST, base, b, 0);
    codeextraarg(fs, c);
  }
  else
    luaX_syntaxerror(fs->ls, "constructor too long");
  fs->freereg = base + 1;  /* free registers with list values */
}

