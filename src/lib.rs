#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals
)]

#[macro_use]
pub(crate) mod lstate;

pub(crate) mod ldebug;
pub(crate) mod ldo;
pub(crate) mod lfunc;
pub(crate) mod lgc;
pub(crate) mod llimits;
pub(crate) mod lmem;
pub(crate) mod lobject;
pub(crate) mod lopcodes;
pub(crate) mod ltemp;
pub(crate) mod ltm;
pub(crate) mod lvm;
pub(crate) mod lzio;
pub(crate) mod types;
