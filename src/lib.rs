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

pub(crate) mod lapi;
pub(crate) mod ldebug;
pub(crate) mod ldo;
pub(crate) mod lfunc;
pub(crate) mod lgc;
pub(crate) mod llex;
pub(crate) mod llimits;
pub(crate) mod lmem;
pub(crate) mod lopcodes;
pub(crate) mod lstring;
pub(crate) mod ltable;
pub(crate) mod ltemp;
pub(crate) mod ltm;
pub(crate) mod lvm;
pub(crate) mod lzio;
pub(crate) mod types;
