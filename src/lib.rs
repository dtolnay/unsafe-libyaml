#![no_std]
#![allow(non_camel_case_types)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_ptr_alignment,
    clippy::cast_sign_loss,
    clippy::collapsible_if,
    clippy::if_not_else,
    clippy::let_underscore_drop,
    clippy::manual_swap,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::nonminimal_bool,
    clippy::ptr_as_ptr,
    clippy::redundant_else,
    clippy::similar_names,
    clippy::single_match,
    clippy::single_match_else,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

extern crate alloc;

use core::mem::size_of;

pub mod libc {
    pub use core::ffi::c_void;
    pub use core::primitive::{
        i32 as c_int, i64 as c_long, i8 as c_char, i8 as c_schar, u16 as c_ushort, u32 as c_uint,
        u64 as c_ulong, u8 as c_uchar,
    };
}

#[macro_use]
pub mod externs {
    use crate::libc;
    use alloc::alloc::{self as rust, Layout};
    use core::mem::{self, MaybeUninit};
    use core::ptr;
    use core::slice;

    const HEADER: usize = mem::size_of::<usize>();

    // `max_align_t` may be bigger than this, but libyaml does not use `long
    // double` or u128.
    const MALLOC_ALIGN: usize = mem::align_of::<usize>();

    pub unsafe fn malloc(size: libc::c_ulong) -> *mut libc::c_void {
        let size = HEADER + size as usize;
        let layout = Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
        let memory = rust::alloc(layout);
        memory.cast::<usize>().write(size);
        memory.add(HEADER).cast()
    }

    pub unsafe fn realloc(ptr: *mut libc::c_void, new_size: libc::c_ulong) -> *mut libc::c_void {
        let mut memory = ptr.cast::<u8>().sub(HEADER);
        let size = memory.cast::<usize>().read();
        let layout = Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
        let new_size = HEADER + new_size as usize;
        memory = rust::realloc(memory, layout, new_size);
        memory.cast::<usize>().write(new_size);
        memory.add(HEADER).cast()
    }

    pub unsafe fn free(ptr: *mut libc::c_void) {
        let memory = ptr.cast::<u8>().sub(HEADER);
        let size = memory.cast::<usize>().read();
        let layout = Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
        rust::dealloc(memory, layout);
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
        let lhs = slice::from_raw_parts(lhs.cast::<u8>(), strlen(lhs) as usize);
        let rhs = slice::from_raw_parts(rhs.cast::<u8>(), strlen(rhs) as usize);
        lhs.cmp(rhs) as libc::c_int
    }

    pub unsafe fn strdup(src: *const libc::c_char) -> *mut libc::c_char {
        let len = strlen(src);
        let dest = malloc(len + 1);
        memcpy(dest, src.cast(), len + 1);
        dest.cast()
    }

    pub unsafe fn strlen(str: *const libc::c_char) -> libc::c_ulong {
        let mut end = str;
        while *end != 0 {
            end = end.add(1);
        }
        end.offset_from(str) as libc::c_ulong
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

    macro_rules! __assert {
        (false $(,)?) => {
            $crate::externs::__assert_fail(stringify!(false), file!(), line!())
        };
        ($assertion:expr $(,)?) => {
            if !$assertion {
                $crate::externs::__assert_fail(stringify!($assertion), file!(), line!());
            }
        };
    }

    pub(crate) unsafe fn __assert_fail(
        __assertion: &'static str,
        __file: &'static str,
        __line: u32,
    ) -> ! {
        struct Abort;
        impl Drop for Abort {
            fn drop(&mut self) {
                panic!();
            }
        }
        let _abort_on_panic = Abort;
        panic!("{}:{}: Assertion `{}` failed.", __file, __line, __assertion);
    }
}

mod fmt {
    use crate::yaml::yaml_char_t;
    use core::fmt::{self, Write};
    use core::ptr;

    pub struct WriteToPtr {
        ptr: *mut yaml_char_t,
    }

    impl WriteToPtr {
        pub unsafe fn new(ptr: *mut yaml_char_t) -> Self {
            WriteToPtr { ptr }
        }

        pub fn write_fmt(&mut self, args: fmt::Arguments) {
            let _ = Write::write_fmt(self, args);
        }
    }

    impl Write for WriteToPtr {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            unsafe {
                ptr::copy_nonoverlapping(s.as_ptr(), self.ptr, s.len());
                self.ptr = self.ptr.add(s.len());
            }
            Ok(())
        }
    }
}

trait PointerExt: Sized {
    fn c_offset_from(self, origin: Self) -> isize;
}

impl<T> PointerExt for *const T {
    fn c_offset_from(self, origin: *const T) -> isize {
        (self as isize - origin as isize) / size_of::<T>() as isize
    }
}

impl<T> PointerExt for *mut T {
    fn c_offset_from(self, origin: *mut T) -> isize {
        (self as isize - origin as isize) / size_of::<T>() as isize
    }
}

mod api;
mod dumper;
mod emitter;
mod loader;
mod parser;
mod reader;
mod scanner;
mod writer;
mod yaml;

pub use crate::api::{
    yaml_alias_event_initialize, yaml_document_add_mapping, yaml_document_add_scalar,
    yaml_document_add_sequence, yaml_document_append_mapping_pair,
    yaml_document_append_sequence_item, yaml_document_delete, yaml_document_end_event_initialize,
    yaml_document_get_node, yaml_document_get_root_node, yaml_document_initialize,
    yaml_document_start_event_initialize, yaml_emitter_delete, yaml_emitter_initialize,
    yaml_emitter_set_break, yaml_emitter_set_canonical, yaml_emitter_set_encoding,
    yaml_emitter_set_indent, yaml_emitter_set_output, yaml_emitter_set_output_string,
    yaml_emitter_set_unicode, yaml_emitter_set_width, yaml_event_delete, yaml_free,
    yaml_get_version, yaml_get_version_string, yaml_malloc, yaml_mapping_end_event_initialize,
    yaml_mapping_start_event_initialize, yaml_parser_delete, yaml_parser_initialize,
    yaml_parser_set_encoding, yaml_parser_set_input, yaml_parser_set_input_string,
    yaml_queue_extend, yaml_realloc, yaml_scalar_event_initialize,
    yaml_sequence_end_event_initialize, yaml_sequence_start_event_initialize, yaml_stack_extend,
    yaml_strdup, yaml_stream_end_event_initialize, yaml_stream_start_event_initialize,
    yaml_string_extend, yaml_string_join, yaml_token_delete,
};
pub use crate::dumper::{yaml_emitter_close, yaml_emitter_dump, yaml_emitter_open};
pub use crate::emitter::yaml_emitter_emit;
pub use crate::loader::yaml_parser_load;
pub use crate::parser::yaml_parser_parse;
pub use crate::reader::yaml_parser_update_buffer;
pub use crate::scanner::{yaml_parser_fetch_more_tokens, yaml_parser_scan};
pub use crate::writer::yaml_emitter_flush;
pub use crate::yaml::{
    yaml_alias_data_t, yaml_anchors_t, yaml_break_t, yaml_document_t, yaml_emitter_state_t,
    yaml_emitter_t, yaml_encoding_t, yaml_error_type_t, yaml_event_t, yaml_event_type_t,
    yaml_mapping_style_t, yaml_mark_t, yaml_node_item_t, yaml_node_pair_t, yaml_node_t,
    yaml_node_type_t, yaml_parser_state_t, yaml_parser_t, yaml_read_handler_t, yaml_scalar_style_t,
    yaml_sequence_style_t, yaml_simple_key_t, yaml_string_t, yaml_tag_directive_t, yaml_token_t,
    yaml_token_type_t, yaml_version_directive_t, yaml_write_handler_t, YAML_ALIAS_EVENT,
    YAML_ALIAS_TOKEN, YAML_ANCHOR_TOKEN, YAML_ANY_BREAK, YAML_ANY_ENCODING, YAML_ANY_MAPPING_STYLE,
    YAML_ANY_SCALAR_STYLE, YAML_ANY_SEQUENCE_STYLE, YAML_BLOCK_END_TOKEN, YAML_BLOCK_ENTRY_TOKEN,
    YAML_BLOCK_MAPPING_START_TOKEN, YAML_BLOCK_MAPPING_STYLE, YAML_BLOCK_SEQUENCE_START_TOKEN,
    YAML_BLOCK_SEQUENCE_STYLE, YAML_COMPOSER_ERROR, YAML_CRLN_BREAK, YAML_CR_BREAK,
    YAML_DOCUMENT_END_EVENT, YAML_DOCUMENT_END_TOKEN, YAML_DOCUMENT_START_EVENT,
    YAML_DOCUMENT_START_TOKEN, YAML_DOUBLE_QUOTED_SCALAR_STYLE, YAML_EMITTER_ERROR,
    YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE, YAML_EMIT_BLOCK_MAPPING_KEY_STATE,
    YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE, YAML_EMIT_BLOCK_MAPPING_VALUE_STATE,
    YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE, YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE,
    YAML_EMIT_DOCUMENT_CONTENT_STATE, YAML_EMIT_DOCUMENT_END_STATE, YAML_EMIT_DOCUMENT_START_STATE,
    YAML_EMIT_END_STATE, YAML_EMIT_FIRST_DOCUMENT_START_STATE,
    YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE, YAML_EMIT_FLOW_MAPPING_KEY_STATE,
    YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE, YAML_EMIT_FLOW_MAPPING_VALUE_STATE,
    YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE, YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE,
    YAML_EMIT_STREAM_START_STATE, YAML_FLOW_ENTRY_TOKEN, YAML_FLOW_MAPPING_END_TOKEN,
    YAML_FLOW_MAPPING_START_TOKEN, YAML_FLOW_MAPPING_STYLE, YAML_FLOW_SEQUENCE_END_TOKEN,
    YAML_FLOW_SEQUENCE_START_TOKEN, YAML_FLOW_SEQUENCE_STYLE, YAML_FOLDED_SCALAR_STYLE,
    YAML_KEY_TOKEN, YAML_LITERAL_SCALAR_STYLE, YAML_LN_BREAK, YAML_MAPPING_END_EVENT,
    YAML_MAPPING_NODE, YAML_MAPPING_START_EVENT, YAML_MEMORY_ERROR, YAML_NO_ERROR, YAML_NO_EVENT,
    YAML_NO_NODE, YAML_NO_TOKEN, YAML_PARSER_ERROR, YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE,
    YAML_PARSE_BLOCK_MAPPING_KEY_STATE, YAML_PARSE_BLOCK_MAPPING_VALUE_STATE,
    YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE, YAML_PARSE_BLOCK_NODE_STATE,
    YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE, YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE,
    YAML_PARSE_DOCUMENT_CONTENT_STATE, YAML_PARSE_DOCUMENT_END_STATE,
    YAML_PARSE_DOCUMENT_START_STATE, YAML_PARSE_END_STATE,
    YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE, YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE,
    YAML_PARSE_FLOW_MAPPING_KEY_STATE, YAML_PARSE_FLOW_MAPPING_VALUE_STATE,
    YAML_PARSE_FLOW_NODE_STATE, YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE, YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE,
    YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE, YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE,
    YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE, YAML_PARSE_STREAM_START_STATE,
    YAML_PLAIN_SCALAR_STYLE, YAML_READER_ERROR, YAML_SCALAR_EVENT, YAML_SCALAR_NODE,
    YAML_SCALAR_TOKEN, YAML_SCANNER_ERROR, YAML_SEQUENCE_END_EVENT, YAML_SEQUENCE_NODE,
    YAML_SEQUENCE_START_EVENT, YAML_SINGLE_QUOTED_SCALAR_STYLE, YAML_STREAM_END_EVENT,
    YAML_STREAM_END_TOKEN, YAML_STREAM_START_EVENT, YAML_STREAM_START_TOKEN,
    YAML_TAG_DIRECTIVE_TOKEN, YAML_TAG_TOKEN, YAML_UTF16BE_ENCODING, YAML_UTF16LE_ENCODING,
    YAML_UTF8_ENCODING, YAML_VALUE_TOKEN, YAML_VERSION_DIRECTIVE_TOKEN, YAML_WRITER_ERROR,
};
