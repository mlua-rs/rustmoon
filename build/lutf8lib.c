/*
** $Id: lutf8lib.c,v 1.16.1.1 2017/04/19 17:29:57 roberto Exp $
** Standard library for UTF-8 manipulation
** See Copyright Notice in lua.h
*/

#define lutf8lib_c
#define LUA_LIB

#include "lprefix.h"


#include <assert.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>

#include "lua.h"

#include "lauxlib.h"
#include "lualib.h"

#define MAXUNICODE	0x10FFFF

#define iscont(p)	((*(p) & 0xC0) == 0x80)

// begin functions moved to rust
int codepoint (lua_State *L);
lua_Integer u_posrelat (lua_Integer pos, size_t len);
const char *utf8_decode (const char *o, int *val);
int utflen (lua_State *L);
int utfchar (lua_State *L);
int byteoffset (lua_State *L);
int iter_codes (lua_State *L);
// end functions moved to rust


/* pattern to match a single UTF-8 character */
#define UTF8PATT	"[\0-\x7F\xC2-\xF4][\x80-\xBF]*"


