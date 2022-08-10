fn main() {
    cc::Build::new()
        .define("LUA_USE_APICHECK", None)
        .define("LUA_USE_CTYPE", None)
        .define("LUA_USE_POSIX", None)
        .file("build/lapi.c")
        .file("build/lauxlib.c")
        .file("build/lbaselib.c")
        .file("build/lbitlib.c")
        .file("build/lcode.c")
        .file("build/lcorolib.c")
        .file("build/ldblib.c")
        .file("build/ldebug.c")
        .file("build/ldo.c")
        .file("build/ldump.c")
        .file("build/lfunc.c")
        .file("build/lgc.c")
        .file("build/linit.c")
        .file("build/liolib.c")
        .file("build/llex.c")
        .file("build/lmathlib.c")
        .file("build/lmem.c")
        .file("build/loadlib.c")
        .file("build/lobject.c")
        .file("build/lopcodes.c")
        .file("build/loslib.c")
        .file("build/lparser.c")
        .file("build/lstate.c")
        .file("build/lstring.c")
        .file("build/lstrlib.c")
        .file("build/ltable.c")
        .file("build/ltablib.c")
        .file("build/ltm.c")
        .file("build/lua.c")
        .file("build/lundump.c")
        .file("build/lutf8lib.c")
        .file("build/lvm.c")
        .compile("lua");
}
