#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals
)]
#![feature(c_variadic)]

#[macro_use]
pub(crate) mod macros;

#[macro_use]
pub(crate) mod lstate;
#[macro_use]
pub(crate) mod lobject;

pub(crate) mod lbaselib;
pub(crate) mod ldebug;
pub(crate) mod ldo;
pub(crate) mod ldump;
pub(crate) mod lfunc;
pub(crate) mod lgc;
pub(crate) mod libs;
pub(crate) mod linit;
pub(crate) mod llex;
pub(crate) mod llimits;
pub(crate) mod lmathlib;
pub(crate) mod lmem;
pub(crate) mod loadlib;
pub(crate) mod lopcodes;
pub(crate) mod loslib;
pub(crate) mod lparser;
pub(crate) mod lstring;
pub(crate) mod ltable;
pub(crate) mod ltablib;
pub(crate) mod ltm;
pub(crate) mod lundump;
pub(crate) mod lutf8lib;
pub(crate) mod lvm;
pub(crate) mod lzio;

pub mod lapi;
pub mod lauxlib;
pub mod lualib;
pub mod types;
