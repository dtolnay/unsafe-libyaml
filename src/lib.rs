#![allow(
    non_camel_case_types,
    non_snake_case,
    unreachable_code,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables
)]

use std::mem;

pub mod libc {
    pub use std::os::raw::{
        c_char, c_int, c_long, c_schar, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    };
}

pub mod externs {
    use crate::libc;
    use std::alloc::Layout;
    use std::ffi::CStr;
    use std::io::{self, Write};
    use std::mem::{self, MaybeUninit};
    use std::process;
    use std::ptr;
    use std::slice;

    const HEADER: usize = mem::size_of::<usize>();

    // `max_align_t` may be bigger than this, but libyaml does not use `long
    // double` or u128.
    const MALLOC_ALIGN: usize = mem::align_of::<usize>();

    pub unsafe fn malloc(size: libc::c_ulong) -> *mut libc::c_void {
        let size = HEADER + size as usize;
        let layout = Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
        let memory = std::alloc::alloc(layout);
        memory.cast::<usize>().write(size);
        memory.add(HEADER).cast()
    }

    pub unsafe fn realloc(ptr: *mut libc::c_void, new_size: libc::c_ulong) -> *mut libc::c_void {
        let mut memory = ptr.cast::<u8>().sub(HEADER);
        let size = memory.cast::<usize>().read();
        let layout = Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
        let new_size = HEADER + new_size as usize;
        memory = std::alloc::realloc(memory, layout, new_size);
        memory.cast::<usize>().write(new_size);
        memory.add(HEADER).cast()
    }

    pub unsafe fn free(ptr: *mut libc::c_void) {
        let memory = ptr.cast::<u8>().sub(HEADER);
        let size = memory.cast::<usize>().read();
        let layout = Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
        std::alloc::dealloc(memory, layout);
    }

    pub unsafe fn memcmp(
        lhs: *const libc::c_void,
        rhs: *const libc::c_void,
        count: libc::c_ulong,
    ) -> libc::c_int {
        let lhs = slice::from_raw_parts(lhs.cast::<u8>(), count as usize);
        let rhs = slice::from_raw_parts(rhs.cast::<u8>(), count as usize);
        lhs.cmp(rhs) as libc::c_int
    }

    pub unsafe fn memcpy(
        dest: *mut libc::c_void,
        src: *const libc::c_void,
        count: libc::c_ulong,
    ) -> *mut libc::c_void {
        ptr::copy_nonoverlapping(
            src.cast::<MaybeUninit<u8>>(),
            dest.cast::<MaybeUninit<u8>>(),
            count as usize,
        );
        dest
    }

    pub unsafe fn memmove(
        dest: *mut libc::c_void,
        src: *const libc::c_void,
        count: libc::c_ulong,
    ) -> *mut libc::c_void {
        ptr::copy(
            src.cast::<MaybeUninit<u8>>(),
            dest.cast::<MaybeUninit<u8>>(),
            count as usize,
        );
        dest
    }

    pub unsafe fn memset(
        dest: *mut libc::c_void,
        ch: libc::c_int,
        count: libc::c_ulong,
    ) -> *mut libc::c_void {
        ptr::write_bytes(dest.cast::<u8>(), ch as u8, count as usize);
        dest
    }

    pub unsafe fn strcmp(lhs: *const libc::c_char, rhs: *const libc::c_char) -> libc::c_int {
        let lhs = CStr::from_ptr(lhs);
        let rhs = CStr::from_ptr(rhs);
        lhs.cmp(rhs) as libc::c_int
    }

    pub unsafe fn strdup(src: *const libc::c_char) -> *mut libc::c_char {
        let len = strlen(src);
        let dest = malloc(len + 1);
        memcpy(dest, src.cast(), len + 1);
        dest.cast()
    }

    pub unsafe fn strlen(str: *const libc::c_char) -> libc::c_ulong {
        CStr::from_ptr(str).to_bytes().len() as libc::c_ulong
    }

    pub unsafe fn strncmp(
        lhs: *const libc::c_char,
        rhs: *const libc::c_char,
        mut count: libc::c_ulong,
    ) -> libc::c_int {
        let mut lhs = lhs.cast::<u8>();
        let mut rhs = rhs.cast::<u8>();
        while count > 0 && *lhs != 0 && *lhs == *rhs {
            lhs = lhs.add(1);
            rhs = rhs.add(1);
            count -= 1;
        }
        if count == 0 {
            0
        } else {
            (*lhs).cmp(&*rhs) as libc::c_int
        }
    }

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

trait PointerExt: Sized {
    fn c_offset_from(self, origin: Self) -> isize;
}

impl<T> PointerExt for *const T {
    fn c_offset_from(self, origin: *const T) -> isize {
        (self as isize - origin as isize) / mem::size_of::<T>() as isize
    }
}

mod yaml;

pub mod api;
pub mod dumper;
pub mod emitter;
pub mod loader;
pub mod parser;
pub mod reader;
pub mod scanner;
pub mod writer;

pub use crate::yaml::*;
