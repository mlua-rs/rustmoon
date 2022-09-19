use std::env;
use std::ptr;

use rustmoon::cstr;
use rustmoon::lapi::lua_call;
use rustmoon::lauxlib::{luaL_loadfilex, luaL_loadstring, luaL_newstate};
use rustmoon::lstate::lua_close;
use rustmoon::lualib::luaL_openlibs;
use rustmoon::types::LUA_OK;

#[test]
fn test_all() {
    std::thread::Builder::new()
        .stack_size(8388608)
        .spawn(|| unsafe {
            let state = luaL_newstate();
            luaL_openlibs(state);

            // Skip non-portable tests
            if luaL_loadstring(state, cstr!("_port=true")) != LUA_OK {
                panic!("Failed to load Lua code");
            }
            lua_call(state, 0, 0);

            // Run the tests suit
            env::set_current_dir("tests").unwrap();
            if luaL_loadfilex(state, cstr!("all.lua"), ptr::null()) != LUA_OK {
                panic!("Failed to load `all.lua`");
            }
            lua_call(state, 0, 0);

            lua_close(state);
        })
        .unwrap()
        .join()
        .unwrap()
}
