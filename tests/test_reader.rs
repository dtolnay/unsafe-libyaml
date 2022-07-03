#![feature(extern_types)]
#![allow(
    non_camel_case_types,
    non_snake_case,
    unused_assignments,
    unused_mut,
)]

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn yaml_parser_set_input_string(
        parser: *mut yaml_parser_t,
        input: *const libc::c_uchar,
        size: size_t,
    );
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn yaml_parser_update_buffer(
        parser: *mut yaml_parser_t,
        length: size_t,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type yaml_char_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_version_directive_s {
    pub major: libc::c_int,
    pub minor: libc::c_int,
}
pub type yaml_version_directive_t = yaml_version_directive_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_tag_directive_s {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
pub type yaml_tag_directive_t = yaml_tag_directive_s;
pub type yaml_encoding_e = libc::c_uint;
pub const YAML_UTF16BE_ENCODING: yaml_encoding_e = 3;
pub const YAML_UTF16LE_ENCODING: yaml_encoding_e = 2;
pub const YAML_UTF8_ENCODING: yaml_encoding_e = 1;
pub const YAML_ANY_ENCODING: yaml_encoding_e = 0;
pub type yaml_encoding_t = yaml_encoding_e;
pub type yaml_error_type_e = libc::c_uint;
pub const YAML_EMITTER_ERROR: yaml_error_type_e = 7;
pub const YAML_WRITER_ERROR: yaml_error_type_e = 6;
pub const YAML_COMPOSER_ERROR: yaml_error_type_e = 5;
pub const YAML_PARSER_ERROR: yaml_error_type_e = 4;
pub const YAML_SCANNER_ERROR: yaml_error_type_e = 3;
pub const YAML_READER_ERROR: yaml_error_type_e = 2;
pub const YAML_MEMORY_ERROR: yaml_error_type_e = 1;
pub const YAML_NO_ERROR: yaml_error_type_e = 0;
pub type yaml_error_type_t = yaml_error_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_mark_s {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}
pub type yaml_mark_t = yaml_mark_s;
pub type yaml_scalar_style_e = libc::c_uint;
pub const YAML_FOLDED_SCALAR_STYLE: yaml_scalar_style_e = 5;
pub const YAML_LITERAL_SCALAR_STYLE: yaml_scalar_style_e = 4;
pub const YAML_DOUBLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 3;
pub const YAML_SINGLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 2;
pub const YAML_PLAIN_SCALAR_STYLE: yaml_scalar_style_e = 1;
pub const YAML_ANY_SCALAR_STYLE: yaml_scalar_style_e = 0;
pub type yaml_scalar_style_t = yaml_scalar_style_e;
pub type yaml_sequence_style_e = libc::c_uint;
pub const YAML_FLOW_SEQUENCE_STYLE: yaml_sequence_style_e = 2;
pub const YAML_BLOCK_SEQUENCE_STYLE: yaml_sequence_style_e = 1;
pub const YAML_ANY_SEQUENCE_STYLE: yaml_sequence_style_e = 0;
pub type yaml_sequence_style_t = yaml_sequence_style_e;
pub type yaml_mapping_style_e = libc::c_uint;
pub const YAML_FLOW_MAPPING_STYLE: yaml_mapping_style_e = 2;
pub const YAML_BLOCK_MAPPING_STYLE: yaml_mapping_style_e = 1;
pub const YAML_ANY_MAPPING_STYLE: yaml_mapping_style_e = 0;
pub type yaml_mapping_style_t = yaml_mapping_style_e;
pub type yaml_token_type_e = libc::c_uint;
pub const YAML_SCALAR_TOKEN: yaml_token_type_e = 21;
pub const YAML_TAG_TOKEN: yaml_token_type_e = 20;
pub const YAML_ANCHOR_TOKEN: yaml_token_type_e = 19;
pub const YAML_ALIAS_TOKEN: yaml_token_type_e = 18;
pub const YAML_VALUE_TOKEN: yaml_token_type_e = 17;
pub const YAML_KEY_TOKEN: yaml_token_type_e = 16;
pub const YAML_FLOW_ENTRY_TOKEN: yaml_token_type_e = 15;
pub const YAML_BLOCK_ENTRY_TOKEN: yaml_token_type_e = 14;
pub const YAML_FLOW_MAPPING_END_TOKEN: yaml_token_type_e = 13;
pub const YAML_FLOW_MAPPING_START_TOKEN: yaml_token_type_e = 12;
pub const YAML_FLOW_SEQUENCE_END_TOKEN: yaml_token_type_e = 11;
pub const YAML_FLOW_SEQUENCE_START_TOKEN: yaml_token_type_e = 10;
pub const YAML_BLOCK_END_TOKEN: yaml_token_type_e = 9;
pub const YAML_BLOCK_MAPPING_START_TOKEN: yaml_token_type_e = 8;
pub const YAML_BLOCK_SEQUENCE_START_TOKEN: yaml_token_type_e = 7;
pub const YAML_DOCUMENT_END_TOKEN: yaml_token_type_e = 6;
pub const YAML_DOCUMENT_START_TOKEN: yaml_token_type_e = 5;
pub const YAML_TAG_DIRECTIVE_TOKEN: yaml_token_type_e = 4;
pub const YAML_VERSION_DIRECTIVE_TOKEN: yaml_token_type_e = 3;
pub const YAML_STREAM_END_TOKEN: yaml_token_type_e = 2;
pub const YAML_STREAM_START_TOKEN: yaml_token_type_e = 1;
pub const YAML_NO_TOKEN: yaml_token_type_e = 0;
pub type yaml_token_type_t = yaml_token_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_token_s {
    pub type_0: yaml_token_type_t,
    pub data: C2RustUnnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub stream_start: C2RustUnnamed_6,
    pub alias: C2RustUnnamed_5,
    pub anchor: C2RustUnnamed_4,
    pub tag: C2RustUnnamed_3,
    pub scalar: C2RustUnnamed_2,
    pub version_directive: C2RustUnnamed_1,
    pub tag_directive: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub major: libc::c_int,
    pub minor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub handle: *mut yaml_char_t,
    pub suffix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_token_t = yaml_token_s;
pub type yaml_node_type_e = libc::c_uint;
pub const YAML_MAPPING_NODE: yaml_node_type_e = 3;
pub const YAML_SEQUENCE_NODE: yaml_node_type_e = 2;
pub const YAML_SCALAR_NODE: yaml_node_type_e = 1;
pub const YAML_NO_NODE: yaml_node_type_e = 0;
pub type yaml_node_type_t = yaml_node_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_node_s {
    pub type_0: yaml_node_type_t,
    pub tag: *mut yaml_char_t,
    pub data: C2RustUnnamed_7,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub scalar: C2RustUnnamed_12,
    pub sequence: C2RustUnnamed_10,
    pub mapping: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub pairs: C2RustUnnamed_9,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
pub type yaml_node_pair_t = yaml_node_pair_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_node_pair_s {
    pub key: libc::c_int,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub items: C2RustUnnamed_11,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_14,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_13,
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_document_t = yaml_document_s;
pub type yaml_read_handler_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_uchar,
    size_t,
    *mut size_t,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_simple_key_s {
    pub possible: libc::c_int,
    pub required: libc::c_int,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}
pub type yaml_simple_key_t = yaml_simple_key_s;
pub type yaml_parser_state_e = libc::c_uint;
pub const YAML_PARSE_END_STATE: yaml_parser_state_e = 23;
pub const YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE: yaml_parser_state_e = 22;
pub const YAML_PARSE_FLOW_MAPPING_VALUE_STATE: yaml_parser_state_e = 21;
pub const YAML_PARSE_FLOW_MAPPING_KEY_STATE: yaml_parser_state_e = 20;
pub const YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE: yaml_parser_state_e = 19;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE: yaml_parser_state_e = 18;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE: yaml_parser_state_e = 17;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE: yaml_parser_state_e = 16;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 15;
pub const YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_e = 14;
pub const YAML_PARSE_BLOCK_MAPPING_VALUE_STATE: yaml_parser_state_e = 13;
pub const YAML_PARSE_BLOCK_MAPPING_KEY_STATE: yaml_parser_state_e = 12;
pub const YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_parser_state_e = 11;
pub const YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 10;
pub const YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 9;
pub const YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_e = 8;
pub const YAML_PARSE_FLOW_NODE_STATE: yaml_parser_state_e = 7;
pub const YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE: yaml_parser_state_e = 6;
pub const YAML_PARSE_BLOCK_NODE_STATE: yaml_parser_state_e = 5;
pub const YAML_PARSE_DOCUMENT_END_STATE: yaml_parser_state_e = 4;
pub const YAML_PARSE_DOCUMENT_CONTENT_STATE: yaml_parser_state_e = 3;
pub const YAML_PARSE_DOCUMENT_START_STATE: yaml_parser_state_e = 2;
pub const YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE: yaml_parser_state_e = 1;
pub const YAML_PARSE_STREAM_START_STATE: yaml_parser_state_e = 0;
pub type yaml_parser_state_t = yaml_parser_state_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_alias_data_s {
    pub anchor: *mut yaml_char_t,
    pub index: libc::c_int,
    pub mark: yaml_mark_t,
}
pub type yaml_alias_data_t = yaml_alias_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_parser_s {
    pub error: yaml_error_type_t,
    pub problem: *const libc::c_char,
    pub problem_offset: size_t,
    pub problem_value: libc::c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *const libc::c_char,
    pub context_mark: yaml_mark_t,
    pub read_handler: Option::<yaml_read_handler_t>,
    pub read_handler_data: *mut libc::c_void,
    pub input: C2RustUnnamed_24,
    pub eof: libc::c_int,
    pub buffer: C2RustUnnamed_23,
    pub unread: size_t,
    pub raw_buffer: C2RustUnnamed_22,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: libc::c_int,
    pub stream_end_produced: libc::c_int,
    pub flow_level: libc::c_int,
    pub tokens: C2RustUnnamed_21,
    pub tokens_parsed: size_t,
    pub token_available: libc::c_int,
    pub indents: C2RustUnnamed_20,
    pub indent: libc::c_int,
    pub simple_key_allowed: libc::c_int,
    pub simple_keys: C2RustUnnamed_19,
    pub states: C2RustUnnamed_18,
    pub state: yaml_parser_state_t,
    pub marks: C2RustUnnamed_17,
    pub tag_directives: C2RustUnnamed_16,
    pub aliases: C2RustUnnamed_15,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub string: C2RustUnnamed_25,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub start: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
    pub current: *const libc::c_uchar,
}
pub type yaml_parser_t = yaml_parser_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_case {
    pub title: *mut libc::c_char,
    pub test: *mut libc::c_char,
    pub result: libc::c_int,
}
#[no_mangle]
pub static mut utf8_sequences: [test_case; 43] = [
    {
        let mut init = test_case {
            title: b"a simple test\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"'test' is '\xD0\xBF\xD1\x80\xD0\xBE\xD0\xB2\xD0\xB5\xD1\x80\xD0\xBA\xD0\xB0' in Russian!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"an empty line\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-0 is a control character\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\0!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-80 is a control character\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xC2\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-800 is valid\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xE0\xA0\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-10000 is valid\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xF0\x90\x80\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"5 bytes sequences are not allowed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xF8\x88\x80\x80\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"6 bytes sequences are not allowed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xFC\x84\x80\x80\x80\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-7f is a control character\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\x7F!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-7FF is valid\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xDF\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-FFFF is a control character\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xEF\xBF\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-1FFFFF is too large\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xF7\xBF\xBF\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-3FFFFFF is 5 bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xFB\xBF\xBF\xBF\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-7FFFFFFF is 6 bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xFD\xBF\xBF\xBF\xBF\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-D7FF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            test: b"\xED\x9F\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-E000\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            test: b"\xEE\x80\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-FFFD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            test: b"\xEF\xBF\xBD!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-10FFFF\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xF4\x8F\xBF\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"u-110000\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xF4\x90\x80\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"first continuation byte\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\x80!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"last continuation byte\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xBF!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"2 continuation bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\x80\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"3 continuation bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\x80\xBF\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"4 continuation bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\x80\xBF\x80\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"5 continuation bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\x80\xBF\x80\xBF\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"6 continuation bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\x80\xBF\x80\xBF\x80\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"7 continuation bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\x80\xBF\x80\xBF\x80\xBF\x80!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"sequence of all 64 possible continuation bytes\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\x80|\x81|\x82|\x83|\x84|\x85|\x86|\x87|\x88|\x89|\x8A|\x8B|\x8C|\x8D|\x8E|\x8F|\x90|\x91|\x92|\x93|\x94|\x95|\x96|\x97|\x98|\x99|\x9A|\x9B|\x9C|\x9D|\x9E|\x9F|\xA0|\xA1|\xA2|\xA3|\xA4|\xA5|\xA6|\xA7|\xA8|\xA9|\xAA|\xAB|\xAC|\xAD|\xAE|\xAF|\xB0|\xB1|\xB2|\xB3|\xB4|\xB5|\xB6|\xB7|\xB8|\xB9|\xBA|\xBB|\xBC|\xBD|\xBE|\xBF!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"32 first bytes of 2-byte sequences {0xc0-0xdf}\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xC0 |\xC1 |\xC2 |\xC3 |\xC4 |\xC5 |\xC6 |\xC7 |\xC8 |\xC9 |\xCA |\xCB |\xCC |\xCD |\xCE |\xCF |\xD0 |\xD1 |\xD2 |\xD3 |\xD4 |\xD5 |\xD6 |\xD7 |\xD8 |\xD9 |\xDA |\xDB |\xDC |\xDD |\xDE |\xDF !\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"16 first bytes of 3-byte sequences {0xe0-0xef}\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xE0 |\xE1 |\xE2 |\xE3 |\xE4 |\xE5 |\xE6 |\xE7 |\xE8 |\xE9 |\xEA |\xEB |\xEC |\xED |\xEE |\xEF !\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"8 first bytes of 4-byte sequences {0xf0-0xf7}\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xF0 |\xF1 |\xF2 |\xF3 |\xF4 |\xF5 |\xF6 |\xF7 !\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"4 first bytes of 5-byte sequences {0xf8-0xfb}\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xF8 |\xF9 |\xFA |\xFB !\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"2 first bytes of 6-byte sequences {0xfc-0xfd}\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xFC |\xFD !\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"sequences with last byte missing {u-0}\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xC0|\xE0\x80|\xF0\x80\x80|\xF8\x80\x80\x80|\xFC\x80\x80\x80\x80!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"sequences with last byte missing {u-...FF}\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xDF|\xEF\xBF|\xF7\xBF\xBF|\xFB\xBF\xBF\xBF|\xFD\xBF\xBF\xBF\xBF!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"impossible bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xFE|\xFF|\xFE\xFE\xFF\xFF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"overlong sequences {u-2f}\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xC0\xAF|\xE0\x80\xAF|\xF0\x80\x80\xAF|\xF8\x80\x80\x80\xAF|\xFC\x80\x80\x80\x80\xAF!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"maximum overlong sequences\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xC1\xBF|\xE0\x9F\xBF|\xF0\x8F\xBF\xBF|\xF8\x87\xBF\xBF\xBF|\xFC\x83\xBF\xBF\xBF\xBF!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"overlong representation of the NUL character\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            test: b"\xC0\x80|\xE0\x80\x80|\xF0\x80\x80\x80|\xF8\x80\x80\x80\x80|\xFC\x80\x80\x80\x80\x80!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"single UTF-16 surrogates\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xED\xA0\x80|\xED\xAD\xBF|\xED\xAE\x80|\xED\xAF\xBF|\xED\xB0\x80|\xED\xBE\x80|\xED\xBF\xBF!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"paired UTF-16 surrogates\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xED\xA0\x80\xED\xB0\x80|\xED\xA0\x80\xED\xBF\xBF|\xED\xAD\xBF\xED\xB0\x80|\xED\xAD\xBF\xED\xBF\xBF|\xED\xAE\x80\xED\xB0\x80|\xED\xAE\x80\xED\xBF\xBF|\xED\xAF\xBF\xED\xB0\x80|\xED\xAF\xBF\xED\xBF\xBF!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"other illegal code positions\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xEF\xBF\xBE|\xEF\xBF\xBF!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: 0 as *const libc::c_char as *mut libc::c_char,
            test: 0 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut boms: [test_case; 5] = [
    {
        let mut init = test_case {
            title: b"no bom (utf-8)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"Hi is \xD0\x9F\xD1\x80\xD0\xB8\xD0\xB2\xD0\xB5\xD1\x82!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"bom (utf-8)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xEF\xBB\xBFHi is \xD0\x9F\xD1\x80\xD0\xB8\xD0\xB2\xD0\xB5\xD1\x82!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"bom (utf-16-le)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xFF\xFEH\0i\0 \0i\0s\0 \0\x1F\x04@\x048\x042\x045\x04B\x04!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: b"bom (utf-16-be)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test: b"\xFE\xFF\0H\0i\0 \0i\0s\0 \x04\x1F\x04@\x048\x042\x045\x04B!\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            result: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = test_case {
            title: 0 as *const libc::c_char as *mut libc::c_char,
            test: 0 as *const libc::c_char as *mut libc::c_char,
            result: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut bom_original: *mut libc::c_char = b"Hi is \xD0\x9F\xD1\x80\xD0\xB8\xD0\xB2\xD0\xB5\xD1\x82\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn check_utf8_sequences() -> libc::c_int {
    let mut parser: yaml_parser_t = yaml_parser_t {
        error: YAML_NO_ERROR,
        problem: 0 as *const libc::c_char,
        problem_offset: 0,
        problem_value: 0,
        problem_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        context: 0 as *const libc::c_char,
        context_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        read_handler: None,
        read_handler_data: 0 as *mut libc::c_void,
        input: C2RustUnnamed_24 {
            string: C2RustUnnamed_25 {
                start: 0 as *const libc::c_uchar,
                end: 0 as *const libc::c_uchar,
                current: 0 as *const libc::c_uchar,
            },
        },
        eof: 0,
        buffer: C2RustUnnamed_23 {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
            last: 0 as *mut yaml_char_t,
        },
        unread: 0,
        raw_buffer: C2RustUnnamed_22 {
            start: 0 as *mut libc::c_uchar,
            end: 0 as *mut libc::c_uchar,
            pointer: 0 as *mut libc::c_uchar,
            last: 0 as *mut libc::c_uchar,
        },
        encoding: YAML_ANY_ENCODING,
        offset: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        stream_start_produced: 0,
        stream_end_produced: 0,
        flow_level: 0,
        tokens: C2RustUnnamed_21 {
            start: 0 as *mut yaml_token_t,
            end: 0 as *mut yaml_token_t,
            head: 0 as *mut yaml_token_t,
            tail: 0 as *mut yaml_token_t,
        },
        tokens_parsed: 0,
        token_available: 0,
        indents: C2RustUnnamed_20 {
            start: 0 as *mut libc::c_int,
            end: 0 as *mut libc::c_int,
            top: 0 as *mut libc::c_int,
        },
        indent: 0,
        simple_key_allowed: 0,
        simple_keys: C2RustUnnamed_19 {
            start: 0 as *mut yaml_simple_key_t,
            end: 0 as *mut yaml_simple_key_t,
            top: 0 as *mut yaml_simple_key_t,
        },
        states: C2RustUnnamed_18 {
            start: 0 as *mut yaml_parser_state_t,
            end: 0 as *mut yaml_parser_state_t,
            top: 0 as *mut yaml_parser_state_t,
        },
        state: YAML_PARSE_STREAM_START_STATE,
        marks: C2RustUnnamed_17 {
            start: 0 as *mut yaml_mark_t,
            end: 0 as *mut yaml_mark_t,
            top: 0 as *mut yaml_mark_t,
        },
        tag_directives: C2RustUnnamed_16 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        },
        aliases: C2RustUnnamed_15 {
            start: 0 as *mut yaml_alias_data_t,
            end: 0 as *mut yaml_alias_data_t,
            top: 0 as *mut yaml_alias_data_t,
        },
        document: 0 as *mut yaml_document_t,
    };
    let mut failed: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    printf(b"checking utf-8 sequences...\n\0" as *const u8 as *const libc::c_char);
    k = 0 as libc::c_int;
    while !(utf8_sequences[k as usize].test).is_null() {
        let mut title: *mut libc::c_char = utf8_sequences[k as usize].title;
        let mut check: libc::c_int = utf8_sequences[k as usize].result;
        let mut result: libc::c_int = 0;
        let mut start: *mut libc::c_char = utf8_sequences[k as usize].test;
        let mut end: *mut libc::c_char = start;
        printf(b"\t%s:\n\0" as *const u8 as *const libc::c_char, title);
        loop {
            while *end as libc::c_int != '|' as i32 && *end as libc::c_int != '!' as i32
            {
                end = end.offset(1);
            }
            yaml_parser_initialize(&mut parser);
            yaml_parser_set_input_string(
                &mut parser,
                start as *mut libc::c_uchar,
                end.offset_from(start) as libc::c_long as size_t,
            );
            result = yaml_parser_update_buffer(
                &mut parser,
                end.offset_from(start) as libc::c_long as size_t,
            );
            if result != check {
                printf(b"\t\t- \0" as *const u8 as *const libc::c_char);
                failed += 1;
            } else {
                printf(b"\t\t+ \0" as *const u8 as *const libc::c_char);
            }
            if parser.error as u64 == 0 {
                printf(b"(no error)\n\0" as *const u8 as *const libc::c_char);
            } else if parser.error as libc::c_uint
                    == YAML_READER_ERROR as libc::c_int as libc::c_uint
                {
                if parser.problem_value != -(1 as libc::c_int) {
                    printf(
                        b"(reader error: %s: #%X at %ld)\n\0" as *const u8
                            as *const libc::c_char,
                        parser.problem,
                        parser.problem_value,
                        parser.problem_offset as libc::c_long,
                    );
                } else {
                    printf(
                        b"(reader error: %s at %ld)\n\0" as *const u8
                            as *const libc::c_char,
                        parser.problem,
                        parser.problem_offset as libc::c_long,
                    );
                }
            }
            if *end as libc::c_int == '!' as i32 {
                break;
            }
            end = end.offset(1);
            start = end;
            yaml_parser_delete(&mut parser);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        k += 1;
    }
    printf(
        b"checking utf-8 sequences: %d fail(s)\n\0" as *const u8 as *const libc::c_char,
        failed,
    );
    return failed;
}
#[no_mangle]
pub unsafe extern "C" fn check_boms() -> libc::c_int {
    let mut parser: yaml_parser_t = yaml_parser_t {
        error: YAML_NO_ERROR,
        problem: 0 as *const libc::c_char,
        problem_offset: 0,
        problem_value: 0,
        problem_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        context: 0 as *const libc::c_char,
        context_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        read_handler: None,
        read_handler_data: 0 as *mut libc::c_void,
        input: C2RustUnnamed_24 {
            string: C2RustUnnamed_25 {
                start: 0 as *const libc::c_uchar,
                end: 0 as *const libc::c_uchar,
                current: 0 as *const libc::c_uchar,
            },
        },
        eof: 0,
        buffer: C2RustUnnamed_23 {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
            last: 0 as *mut yaml_char_t,
        },
        unread: 0,
        raw_buffer: C2RustUnnamed_22 {
            start: 0 as *mut libc::c_uchar,
            end: 0 as *mut libc::c_uchar,
            pointer: 0 as *mut libc::c_uchar,
            last: 0 as *mut libc::c_uchar,
        },
        encoding: YAML_ANY_ENCODING,
        offset: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        stream_start_produced: 0,
        stream_end_produced: 0,
        flow_level: 0,
        tokens: C2RustUnnamed_21 {
            start: 0 as *mut yaml_token_t,
            end: 0 as *mut yaml_token_t,
            head: 0 as *mut yaml_token_t,
            tail: 0 as *mut yaml_token_t,
        },
        tokens_parsed: 0,
        token_available: 0,
        indents: C2RustUnnamed_20 {
            start: 0 as *mut libc::c_int,
            end: 0 as *mut libc::c_int,
            top: 0 as *mut libc::c_int,
        },
        indent: 0,
        simple_key_allowed: 0,
        simple_keys: C2RustUnnamed_19 {
            start: 0 as *mut yaml_simple_key_t,
            end: 0 as *mut yaml_simple_key_t,
            top: 0 as *mut yaml_simple_key_t,
        },
        states: C2RustUnnamed_18 {
            start: 0 as *mut yaml_parser_state_t,
            end: 0 as *mut yaml_parser_state_t,
            top: 0 as *mut yaml_parser_state_t,
        },
        state: YAML_PARSE_STREAM_START_STATE,
        marks: C2RustUnnamed_17 {
            start: 0 as *mut yaml_mark_t,
            end: 0 as *mut yaml_mark_t,
            top: 0 as *mut yaml_mark_t,
        },
        tag_directives: C2RustUnnamed_16 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        },
        aliases: C2RustUnnamed_15 {
            start: 0 as *mut yaml_alias_data_t,
            end: 0 as *mut yaml_alias_data_t,
            top: 0 as *mut yaml_alias_data_t,
        },
        document: 0 as *mut yaml_document_t,
    };
    let mut failed: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    printf(b"checking boms...\n\0" as *const u8 as *const libc::c_char);
    k = 0 as libc::c_int;
    while !(boms[k as usize].test).is_null() {
        let mut title: *mut libc::c_char = boms[k as usize].title;
        let mut check: libc::c_int = boms[k as usize].result;
        let mut result: libc::c_int = 0;
        let mut start: *mut libc::c_char = boms[k as usize].test;
        let mut end: *mut libc::c_char = start;
        while *end as libc::c_int != '!' as i32 {
            end = end.offset(1);
        }
        printf(b"\t%s: \0" as *const u8 as *const libc::c_char, title);
        yaml_parser_initialize(&mut parser);
        yaml_parser_set_input_string(
            &mut parser,
            start as *mut libc::c_uchar,
            end.offset_from(start) as libc::c_long as size_t,
        );
        result = yaml_parser_update_buffer(
            &mut parser,
            end.offset_from(start) as libc::c_long as size_t,
        );
        if result == 0 {
            printf(
                b"- (reader error: %s at %ld)\n\0" as *const u8 as *const libc::c_char,
                parser.problem,
                parser.problem_offset as libc::c_long,
            );
            failed += 1;
        } else if parser.unread != check as libc::c_ulong {
            printf(
                b"- (length=%ld while expected length=%d)\n\0" as *const u8
                    as *const libc::c_char,
                parser.unread as libc::c_long,
                check,
            );
            failed += 1;
        } else if memcmp(
                parser.buffer.start as *const libc::c_void,
                bom_original as *const libc::c_void,
                check as libc::c_ulong,
            ) != 0 as libc::c_int
            {
            printf(
                b"- (value '%s' does not equal to the original value '%s')\n\0"
                    as *const u8 as *const libc::c_char,
                parser.buffer.start,
                bom_original,
            );
            failed += 1;
        } else {
            printf(b"+\n\0" as *const u8 as *const libc::c_char);
        }
        yaml_parser_delete(&mut parser);
        k += 1;
    }
    printf(b"checking boms: %d fail(s)\n\0" as *const u8 as *const libc::c_char, failed);
    return failed;
}
#[no_mangle]
pub unsafe extern "C" fn check_long_utf8() -> libc::c_int {
    let mut parser: yaml_parser_t = yaml_parser_t {
        error: YAML_NO_ERROR,
        problem: 0 as *const libc::c_char,
        problem_offset: 0,
        problem_value: 0,
        problem_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        context: 0 as *const libc::c_char,
        context_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        read_handler: None,
        read_handler_data: 0 as *mut libc::c_void,
        input: C2RustUnnamed_24 {
            string: C2RustUnnamed_25 {
                start: 0 as *const libc::c_uchar,
                end: 0 as *const libc::c_uchar,
                current: 0 as *const libc::c_uchar,
            },
        },
        eof: 0,
        buffer: C2RustUnnamed_23 {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
            last: 0 as *mut yaml_char_t,
        },
        unread: 0,
        raw_buffer: C2RustUnnamed_22 {
            start: 0 as *mut libc::c_uchar,
            end: 0 as *mut libc::c_uchar,
            pointer: 0 as *mut libc::c_uchar,
            last: 0 as *mut libc::c_uchar,
        },
        encoding: YAML_ANY_ENCODING,
        offset: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        stream_start_produced: 0,
        stream_end_produced: 0,
        flow_level: 0,
        tokens: C2RustUnnamed_21 {
            start: 0 as *mut yaml_token_t,
            end: 0 as *mut yaml_token_t,
            head: 0 as *mut yaml_token_t,
            tail: 0 as *mut yaml_token_t,
        },
        tokens_parsed: 0,
        token_available: 0,
        indents: C2RustUnnamed_20 {
            start: 0 as *mut libc::c_int,
            end: 0 as *mut libc::c_int,
            top: 0 as *mut libc::c_int,
        },
        indent: 0,
        simple_key_allowed: 0,
        simple_keys: C2RustUnnamed_19 {
            start: 0 as *mut yaml_simple_key_t,
            end: 0 as *mut yaml_simple_key_t,
            top: 0 as *mut yaml_simple_key_t,
        },
        states: C2RustUnnamed_18 {
            start: 0 as *mut yaml_parser_state_t,
            end: 0 as *mut yaml_parser_state_t,
            top: 0 as *mut yaml_parser_state_t,
        },
        state: YAML_PARSE_STREAM_START_STATE,
        marks: C2RustUnnamed_17 {
            start: 0 as *mut yaml_mark_t,
            end: 0 as *mut yaml_mark_t,
            top: 0 as *mut yaml_mark_t,
        },
        tag_directives: C2RustUnnamed_16 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        },
        aliases: C2RustUnnamed_15 {
            start: 0 as *mut yaml_alias_data_t,
            end: 0 as *mut yaml_alias_data_t,
            top: 0 as *mut yaml_alias_data_t,
        },
        document: 0 as *mut yaml_document_t,
    };
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut failed: libc::c_int = 0 as libc::c_int;
    let mut ch0: libc::c_uchar = 0;
    let mut ch1: libc::c_uchar = 0;
    let mut buffer: *mut libc::c_uchar = malloc(
        (3 as libc::c_int + 100000 as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer\0" as *const u8 as *const libc::c_char,
            b"test-reader.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"int check_long_utf8(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"checking a long utf8 sequence...\n\0" as *const u8 as *const libc::c_char);
    let fresh0 = k;
    k = k + 1;
    *buffer.offset(fresh0 as isize) = -17i32 as libc::c_uchar;
    let fresh1 = k;
    k = k + 1;
    *buffer.offset(fresh1 as isize) = -69i32 as libc::c_uchar;
    let fresh2 = k;
    k = k + 1;
    *buffer.offset(fresh2 as isize) = -65i32 as libc::c_uchar;
    j = 0 as libc::c_int;
    while j < 100000 as libc::c_int {
        if j % 2 as libc::c_int != 0 {
            let fresh3 = k;
            k = k + 1;
            *buffer.offset(fresh3 as isize) = -48i32 as libc::c_uchar;
            let fresh4 = k;
            k = k + 1;
            *buffer.offset(fresh4 as isize) = -112i32 as libc::c_uchar;
        } else {
            let fresh5 = k;
            k = k + 1;
            *buffer.offset(fresh5 as isize) = -48i32 as libc::c_uchar;
            let fresh6 = k;
            k = k + 1;
            *buffer.offset(fresh6 as isize) = -81i32 as libc::c_uchar;
        }
        j += 1;
    }
    yaml_parser_initialize(&mut parser);
    yaml_parser_set_input_string(
        &mut parser,
        buffer,
        (3 as libc::c_int + 100000 as libc::c_int * 2 as libc::c_int) as size_t,
    );
    k = 0 as libc::c_int;
    while k < 100000 as libc::c_int {
        if parser.unread == 0 {
            if yaml_parser_update_buffer(&mut parser, 1 as libc::c_int as size_t) == 0 {
                printf(
                    b"\treader error: %s at %ld\n\0" as *const u8 as *const libc::c_char,
                    parser.problem,
                    parser.problem_offset as libc::c_long,
                );
                failed = 1 as libc::c_int;
                break;
            }
        }
        if parser.unread == 0 {
            printf(
                b"\tnot enough characters at %d\n\0" as *const u8 as *const libc::c_char,
                k,
            );
            failed = 1 as libc::c_int;
            break;
        } else {
            if k % 2 as libc::c_int != 0 {
                ch0 = -48i32 as libc::c_uchar;
                ch1 = -112i32 as libc::c_uchar;
            } else {
                ch0 = -48i32 as libc::c_uchar;
                ch1 = -81i32 as libc::c_uchar;
            }
            if *(parser.buffer.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                != ch0 as libc::c_int
                || *(parser.buffer.pointer).offset(1 as libc::c_int as isize)
                    as libc::c_int != ch1 as libc::c_int
            {
                printf(
                    b"\tincorrect UTF-8 sequence: %X %X instead of %X %X\n\0"
                        as *const u8 as *const libc::c_char,
                    *(parser.buffer.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int,
                    *(parser.buffer.pointer).offset(1 as libc::c_int as isize)
                        as libc::c_int,
                    ch0 as libc::c_int,
                    ch1 as libc::c_int,
                );
                failed = 1 as libc::c_int;
                break;
            } else {
                parser
                    .buffer
                    .pointer = (parser.buffer.pointer).offset(2 as libc::c_int as isize);
                parser
                    .unread = (parser.unread as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
                k += 1;
            }
        }
    }
    if failed == 0 {
        if yaml_parser_update_buffer(&mut parser, 1 as libc::c_int as size_t) == 0 {
            printf(
                b"\treader error: %s at %ld\n\0" as *const u8 as *const libc::c_char,
                parser.problem,
                parser.problem_offset as libc::c_long,
            );
            failed = 1 as libc::c_int;
        } else if *(parser.buffer.pointer).offset(0 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
            {
            printf(
                b"\texpected NUL, found %X (eof=%d, unread=%ld)\n\0" as *const u8
                    as *const libc::c_char,
                *(parser.buffer.pointer).offset(0 as libc::c_int as isize)
                    as libc::c_int,
                parser.eof,
                parser.unread as libc::c_long,
            );
            failed = 1 as libc::c_int;
        }
    }
    yaml_parser_delete(&mut parser);
    free(buffer as *mut libc::c_void);
    printf(
        b"checking a long utf8 sequence: %d fail(s)\n\0" as *const u8
            as *const libc::c_char,
        failed,
    );
    return failed;
}
#[no_mangle]
pub unsafe extern "C" fn check_long_utf16() -> libc::c_int {
    let mut parser: yaml_parser_t = yaml_parser_t {
        error: YAML_NO_ERROR,
        problem: 0 as *const libc::c_char,
        problem_offset: 0,
        problem_value: 0,
        problem_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        context: 0 as *const libc::c_char,
        context_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        read_handler: None,
        read_handler_data: 0 as *mut libc::c_void,
        input: C2RustUnnamed_24 {
            string: C2RustUnnamed_25 {
                start: 0 as *const libc::c_uchar,
                end: 0 as *const libc::c_uchar,
                current: 0 as *const libc::c_uchar,
            },
        },
        eof: 0,
        buffer: C2RustUnnamed_23 {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
            last: 0 as *mut yaml_char_t,
        },
        unread: 0,
        raw_buffer: C2RustUnnamed_22 {
            start: 0 as *mut libc::c_uchar,
            end: 0 as *mut libc::c_uchar,
            pointer: 0 as *mut libc::c_uchar,
            last: 0 as *mut libc::c_uchar,
        },
        encoding: YAML_ANY_ENCODING,
        offset: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        stream_start_produced: 0,
        stream_end_produced: 0,
        flow_level: 0,
        tokens: C2RustUnnamed_21 {
            start: 0 as *mut yaml_token_t,
            end: 0 as *mut yaml_token_t,
            head: 0 as *mut yaml_token_t,
            tail: 0 as *mut yaml_token_t,
        },
        tokens_parsed: 0,
        token_available: 0,
        indents: C2RustUnnamed_20 {
            start: 0 as *mut libc::c_int,
            end: 0 as *mut libc::c_int,
            top: 0 as *mut libc::c_int,
        },
        indent: 0,
        simple_key_allowed: 0,
        simple_keys: C2RustUnnamed_19 {
            start: 0 as *mut yaml_simple_key_t,
            end: 0 as *mut yaml_simple_key_t,
            top: 0 as *mut yaml_simple_key_t,
        },
        states: C2RustUnnamed_18 {
            start: 0 as *mut yaml_parser_state_t,
            end: 0 as *mut yaml_parser_state_t,
            top: 0 as *mut yaml_parser_state_t,
        },
        state: YAML_PARSE_STREAM_START_STATE,
        marks: C2RustUnnamed_17 {
            start: 0 as *mut yaml_mark_t,
            end: 0 as *mut yaml_mark_t,
            top: 0 as *mut yaml_mark_t,
        },
        tag_directives: C2RustUnnamed_16 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        },
        aliases: C2RustUnnamed_15 {
            start: 0 as *mut yaml_alias_data_t,
            end: 0 as *mut yaml_alias_data_t,
            top: 0 as *mut yaml_alias_data_t,
        },
        document: 0 as *mut yaml_document_t,
    };
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut failed: libc::c_int = 0 as libc::c_int;
    let mut ch0: libc::c_uchar = 0;
    let mut ch1: libc::c_uchar = 0;
    let mut buffer: *mut libc::c_uchar = malloc(
        (2 as libc::c_int + 100000 as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if !buffer.is_null() {} else {
        __assert_fail(
            b"buffer\0" as *const u8 as *const libc::c_char,
            b"test-reader.c\0" as *const u8 as *const libc::c_char,
            287 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"int check_long_utf16(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"checking a long utf16 sequence...\n\0" as *const u8 as *const libc::c_char);
    let fresh7 = k;
    k = k + 1;
    *buffer.offset(fresh7 as isize) = -1i32 as libc::c_uchar;
    let fresh8 = k;
    k = k + 1;
    *buffer.offset(fresh8 as isize) = -2i32 as libc::c_uchar;
    j = 0 as libc::c_int;
    while j < 100000 as libc::c_int {
        if j % 2 as libc::c_int != 0 {
            let fresh9 = k;
            k = k + 1;
            *buffer.offset(fresh9 as isize) = '\u{10}' as i32 as libc::c_uchar;
            let fresh10 = k;
            k = k + 1;
            *buffer.offset(fresh10 as isize) = '\u{4}' as i32 as libc::c_uchar;
        } else {
            let fresh11 = k;
            k = k + 1;
            *buffer.offset(fresh11 as isize) = '/' as i32 as libc::c_uchar;
            let fresh12 = k;
            k = k + 1;
            *buffer.offset(fresh12 as isize) = '\u{4}' as i32 as libc::c_uchar;
        }
        j += 1;
    }
    yaml_parser_initialize(&mut parser);
    yaml_parser_set_input_string(
        &mut parser,
        buffer,
        (2 as libc::c_int + 100000 as libc::c_int * 2 as libc::c_int) as size_t,
    );
    k = 0 as libc::c_int;
    while k < 100000 as libc::c_int {
        if parser.unread == 0 {
            if yaml_parser_update_buffer(&mut parser, 1 as libc::c_int as size_t) == 0 {
                printf(
                    b"\treader error: %s at %ld\n\0" as *const u8 as *const libc::c_char,
                    parser.problem,
                    parser.problem_offset as libc::c_long,
                );
                failed = 1 as libc::c_int;
                break;
            }
        }
        if parser.unread == 0 {
            printf(
                b"\tnot enough characters at %d\n\0" as *const u8 as *const libc::c_char,
                k,
            );
            failed = 1 as libc::c_int;
            break;
        } else {
            if k % 2 as libc::c_int != 0 {
                ch0 = -48i32 as libc::c_uchar;
                ch1 = -112i32 as libc::c_uchar;
            } else {
                ch0 = -48i32 as libc::c_uchar;
                ch1 = -81i32 as libc::c_uchar;
            }
            if *(parser.buffer.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                != ch0 as libc::c_int
                || *(parser.buffer.pointer).offset(1 as libc::c_int as isize)
                    as libc::c_int != ch1 as libc::c_int
            {
                printf(
                    b"\tincorrect UTF-8 sequence: %X %X instead of %X %X\n\0"
                        as *const u8 as *const libc::c_char,
                    *(parser.buffer.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int,
                    *(parser.buffer.pointer).offset(1 as libc::c_int as isize)
                        as libc::c_int,
                    ch0 as libc::c_int,
                    ch1 as libc::c_int,
                );
                failed = 1 as libc::c_int;
                break;
            } else {
                parser
                    .buffer
                    .pointer = (parser.buffer.pointer).offset(2 as libc::c_int as isize);
                parser
                    .unread = (parser.unread as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
                k += 1;
            }
        }
    }
    if failed == 0 {
        if yaml_parser_update_buffer(&mut parser, 1 as libc::c_int as size_t) == 0 {
            printf(
                b"\treader error: %s at %ld\n\0" as *const u8 as *const libc::c_char,
                parser.problem,
                parser.problem_offset as libc::c_long,
            );
            failed = 1 as libc::c_int;
        } else if *(parser.buffer.pointer).offset(0 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
            {
            printf(
                b"\texpected NUL, found %X (eof=%d, unread=%ld)\n\0" as *const u8
                    as *const libc::c_char,
                *(parser.buffer.pointer).offset(0 as libc::c_int as isize)
                    as libc::c_int,
                parser.eof,
                parser.unread as libc::c_long,
            );
            failed = 1 as libc::c_int;
        }
    }
    yaml_parser_delete(&mut parser);
    free(buffer as *mut libc::c_void);
    printf(
        b"checking a long utf16 sequence: %d fail(s)\n\0" as *const u8
            as *const libc::c_char,
        failed,
    );
    return failed;
}
unsafe fn main_0() -> libc::c_int {
    return check_utf8_sequences() + check_boms() + check_long_utf8()
        + check_long_utf16();
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
