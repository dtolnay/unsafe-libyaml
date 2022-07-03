#![feature(extern_types)]
#![allow(
    non_camel_case_types,
    non_snake_case,
    unreachable_code,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables
)]

pub mod libc {
    pub use std::os::raw::{
        c_char, c_int, c_long, c_schar, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    };
}

#[rustfmt::skip]
pub mod api;
#[rustfmt::skip]
pub mod dumper;
#[rustfmt::skip]
pub mod emitter;
#[rustfmt::skip]
pub mod loader;
#[rustfmt::skip]
pub mod parser;
#[rustfmt::skip]
pub mod reader;
#[rustfmt::skip]
pub mod scanner;
#[rustfmt::skip]
pub mod writer;
