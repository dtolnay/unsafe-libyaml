#![feature(extern_types)]
#![allow(
    non_camel_case_types,
    non_snake_case,
    unreachable_code,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
)]

pub mod libc {
    pub use std::os::raw::{
        c_char, c_int, c_long, c_schar, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    };
}

pub mod api;
pub mod dumper;
pub mod emitter;
pub mod loader;
pub mod parser;
pub mod reader;
pub mod scanner;
pub mod writer;
