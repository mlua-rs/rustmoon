use libc::{c_char, c_int};

extern "C" {
    pub fn lua_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

fn main() {
    let mut args: Vec<*mut c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(lua_main(
            (args.len() - 1) as c_int,
            args.as_mut_ptr() as *mut *mut c_char,
        ) as i32)
    }
}

extern crate rustmoon;
