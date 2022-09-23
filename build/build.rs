fn main() {
    cc::Build::new()
        .define("LUA_USE_APICHECK", None)
        .define("LUA_USE_CTYPE", None)
        .define("LUA_USE_POSIX", None)
        // Lua core
        .file("build/lparser.c")
        // Non core libs
        .file("build/ldblib.c")
        .file("build/liolib.c")
        .file("build/lstrlib.c")
        .file("build/lua.c")
        .compile("lua");
}
