fn main() {
    cc::Build::new()
        .define("LUA_USE_APICHECK", None)
        .define("LUA_USE_CTYPE", None)
        .define("LUA_USE_POSIX", None)
        // Lua core
        .file("build/lauxlib.c")
        .file("build/lcode.c")
        .file("build/lgc.c")
        .file("build/llex.c")
        .file("build/lparser.c")
        .file("build/lvm.c")
        // Non core libs
        .file("build/lbaselib.c")
        .file("build/lbitlib.c")
        .file("build/lcorolib.c")
        .file("build/ldblib.c")
        .file("build/liolib.c")
        .file("build/loslib.c")
        .file("build/lstrlib.c")
        .file("build/ltablib.c")
        .file("build/lua.c")
        .compile("lua");
}
