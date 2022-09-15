# Lua 5.3 in Rust - HACK-3870

# How to get started

  * Install rust (needs to be a nightly)
    * curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    * rustup default nightly
  * `cargo b` to build
  * `cargo test` to run the tests

# Working on stuff

 * Send Alex (aorlenko/khvzak) your github username to be added to the repo to push branches (so you don't have to work from a fork).
 * Please don't push to master unless the tests pass!
 * Not everything needs porting. E.g. rust supplies printf already, so you don't need to port this
 * Note `cstr!` from `macros.rs`, lua uses C strings (0 byte terminated) rust doesn't, this macro helps with that.
   * You can replace byte terminated strings with this macro in vscode using the following find and replace regex: `b(".*)\\0" as \*const u8 as \*const c_char` and `cstr!($1")`

# Project tracking

  https://docs.google.com/document/d/1NmWf19Xf47-Y99b6yKMzoyr2vY6hWXaxNHi1I4nL_vA/


# Examples

## Tests

https://www.lua.org/tests/

Start at `tests/tests.rs`

This sets up a lua environment and then runs the lua 5.3 tests.

# C to rust transpiler

https://github.com/immunant/c2rust

See `onelua.rs` - lua C code transpiled, could be helpful, but kinda hell code

Lots of C preprocessor + macros

Does not include the lua standard libraries at the moment

## How to port files

See `build/` and specifically `build/build.rs` - this includes the mapping of all C files built and what rust code to link them with.

Once things are ported, the `.c` files can be removed (but the `.h` files cannot). Any `.c` code in `build/` is still to be moved and should be deleted once moved.

## Using rust code from C

Search for anything with `[no_mangle]`.

E.g. from lstate.rs

    /*
    ** set GCdebt to a new value keeping the value (totalbytes + GCdebt)
    ** invariant (and avoiding underflows in 'totalbytes')
    */
    #[no_mangle]
    pub unsafe extern "C" fn luaE_setdebt(g: *mut global_State, debt: l_mem) {
        let tb = gettotalbytes(g);
        debug_assert!(tb > 0);
        (*g).totalbytes = tb as isize - debt;
        (*g).GCdebt = debt;
    }

## Using C code from rust

Lua C API https://www.lua.org/pil/24.html

See in lgc.rs

    extern "C" {
        pub fn luaC_upvalbarrier_(L: *mut lua_State, uv: *mut UpVal);
        pub fn luaC_barrierback_(L: *mut lua_State, t: *mut Table);
        pub fn luaC_fix(L: *mut lua_State, o: *mut GCObject);
        pub fn luaC_newobj(L: *mut lua_State, tt: c_int, sz: size_t) -> *mut GCObject;
        pub fn luaC_step(L: *mut lua_State);
        pub fn luaC_freeallobjects(L: *mut lua_State);
        pub fn luaC_fullgc(L: *mut lua_State, isemergency: c_int);
        pub fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject);
    }

## Refactoring

Take some of the ready made `.rs` rust code and make more idiomatic.

  * Remove raw pointers
  * Use methods
  * Stop calculating hashes and use rust native
  * Use primitives like vectors
  * Remove libc

## Exceptions

Lua uses exceptions a lot (both in lua + in the C api). C by it's nature doesn't have exceptions, so the C api uses `setjmp` and `longjmp`.

Replace with Rust native implementation of exceptions.

## Wrapping

Lua uses integer number overflow and wraparound explcitily. In rust this will cause an exception.

