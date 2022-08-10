use libc::c_int;

pub type jmp_buf = [libc::c_int; 37];

/* chain list of long jump buffers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_longjmp {
    pub previous: *mut lua_longjmp,
    pub b: jmp_buf,
    pub status: c_int,
}
