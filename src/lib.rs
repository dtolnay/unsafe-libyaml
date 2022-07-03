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

pub mod externs {
    use crate::libc;
    use std::ffi::CStr;
    use std::io::{self, Write};
    use std::process;

    pub unsafe fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> ! {
        let _ = writeln!(
            io::stderr(),
            "{}:{}: {}: Assertion `{}` failed.",
            CStr::from_ptr(__file).to_string_lossy(),
            __line,
            CStr::from_ptr(__function).to_string_lossy(),
            CStr::from_ptr(__assertion).to_string_lossy(),
        );
        process::abort();
    }
}

mod yaml;

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

pub use crate::yaml::*;
