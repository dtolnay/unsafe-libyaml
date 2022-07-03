#![feature(extern_types)]
#![allow(
    non_camel_case_types,
    non_snake_case,
    unused_assignments,
    unused_mut,
)]

use unsafe_libyaml::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn yaml_parser_parse(
        parser: *mut yaml_parser_t,
        event: *mut yaml_event_t,
    ) -> libc::c_int;
    fn yaml_parser_set_input_file(parser: *mut yaml_parser_t, file: *mut FILE);
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> libc::c_int;
    fn yaml_event_delete(event: *mut yaml_event_t);
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn abort() -> !;
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
pub type yaml_event_type_e = libc::c_uint;
pub const YAML_MAPPING_END_EVENT: yaml_event_type_e = 10;
pub const YAML_MAPPING_START_EVENT: yaml_event_type_e = 9;
pub const YAML_SEQUENCE_END_EVENT: yaml_event_type_e = 8;
pub const YAML_SEQUENCE_START_EVENT: yaml_event_type_e = 7;
pub const YAML_SCALAR_EVENT: yaml_event_type_e = 6;
pub const YAML_ALIAS_EVENT: yaml_event_type_e = 5;
pub const YAML_DOCUMENT_END_EVENT: yaml_event_type_e = 4;
pub const YAML_DOCUMENT_START_EVENT: yaml_event_type_e = 3;
pub const YAML_STREAM_END_EVENT: yaml_event_type_e = 2;
pub const YAML_STREAM_START_EVENT: yaml_event_type_e = 1;
pub const YAML_NO_EVENT: yaml_event_type_e = 0;
pub type yaml_event_type_t = yaml_event_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_event_s {
    pub type_0: yaml_event_type_t,
    pub data: C2RustUnnamed_7,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub stream_start: C2RustUnnamed_15,
    pub document_start: C2RustUnnamed_13,
    pub document_end: C2RustUnnamed_12,
    pub alias: C2RustUnnamed_11,
    pub scalar: C2RustUnnamed_10,
    pub sequence_start: C2RustUnnamed_9,
    pub mapping_start: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub plain_implicit: libc::c_int,
    pub quoted_implicit: libc::c_int,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_14,
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_event_t = yaml_event_s;
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
    pub data: C2RustUnnamed_16,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub scalar: C2RustUnnamed_21,
    pub sequence: C2RustUnnamed_19,
    pub mapping: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub pairs: C2RustUnnamed_18,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
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
pub struct C2RustUnnamed_19 {
    pub items: C2RustUnnamed_20,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_23,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_22,
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
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
    pub input: C2RustUnnamed_33,
    pub eof: libc::c_int,
    pub buffer: C2RustUnnamed_32,
    pub unread: size_t,
    pub raw_buffer: C2RustUnnamed_31,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: libc::c_int,
    pub stream_end_produced: libc::c_int,
    pub flow_level: libc::c_int,
    pub tokens: C2RustUnnamed_30,
    pub tokens_parsed: size_t,
    pub token_available: libc::c_int,
    pub indents: C2RustUnnamed_29,
    pub indent: libc::c_int,
    pub simple_key_allowed: libc::c_int,
    pub simple_keys: C2RustUnnamed_28,
    pub states: C2RustUnnamed_27,
    pub state: yaml_parser_state_t,
    pub marks: C2RustUnnamed_26,
    pub tag_directives: C2RustUnnamed_25,
    pub aliases: C2RustUnnamed_24,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub string: C2RustUnnamed_34,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub start: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
    pub current: *const libc::c_uchar,
}
pub type yaml_parser_t = yaml_parser_s;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut input: *mut FILE = 0 as *mut FILE;
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
        input: C2RustUnnamed_33 {
            string: C2RustUnnamed_34 {
                start: 0 as *const libc::c_uchar,
                end: 0 as *const libc::c_uchar,
                current: 0 as *const libc::c_uchar,
            },
        },
        eof: 0,
        buffer: C2RustUnnamed_32 {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
            last: 0 as *mut yaml_char_t,
        },
        unread: 0,
        raw_buffer: C2RustUnnamed_31 {
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
        tokens: C2RustUnnamed_30 {
            start: 0 as *mut yaml_token_t,
            end: 0 as *mut yaml_token_t,
            head: 0 as *mut yaml_token_t,
            tail: 0 as *mut yaml_token_t,
        },
        tokens_parsed: 0,
        token_available: 0,
        indents: C2RustUnnamed_29 {
            start: 0 as *mut libc::c_int,
            end: 0 as *mut libc::c_int,
            top: 0 as *mut libc::c_int,
        },
        indent: 0,
        simple_key_allowed: 0,
        simple_keys: C2RustUnnamed_28 {
            start: 0 as *mut yaml_simple_key_t,
            end: 0 as *mut yaml_simple_key_t,
            top: 0 as *mut yaml_simple_key_t,
        },
        states: C2RustUnnamed_27 {
            start: 0 as *mut yaml_parser_state_t,
            end: 0 as *mut yaml_parser_state_t,
            top: 0 as *mut yaml_parser_state_t,
        },
        state: YAML_PARSE_STREAM_START_STATE,
        marks: C2RustUnnamed_26 {
            start: 0 as *mut yaml_mark_t,
            end: 0 as *mut yaml_mark_t,
            top: 0 as *mut yaml_mark_t,
        },
        tag_directives: C2RustUnnamed_25 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        },
        aliases: C2RustUnnamed_24 {
            start: 0 as *mut yaml_alias_data_t,
            end: 0 as *mut yaml_alias_data_t,
            top: 0 as *mut yaml_alias_data_t,
        },
        document: 0 as *mut yaml_document_t,
    };
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed_7 {
            stream_start: C2RustUnnamed_15 {
                encoding: YAML_ANY_ENCODING,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    let mut flow: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut foundfile: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if strncmp(
            *argv.offset(i as isize),
            b"--flow\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            if i + 1 as libc::c_int == argc {
                return usage(1 as libc::c_int);
            }
            i += 1;
            if strncmp(
                *argv.offset(i as isize),
                b"keep\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                flow = 0 as libc::c_int;
            } else if strncmp(
                    *argv.offset(i as isize),
                    b"on\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                flow = 1 as libc::c_int;
            } else if strncmp(
                    *argv.offset(i as isize),
                    b"off\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                flow = -(1 as libc::c_int);
            } else {
                return usage(1 as libc::c_int)
            }
        } else if strncmp(
                *argv.offset(i as isize),
                b"--help\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            return usage(0 as libc::c_int)
        } else {
            if strncmp(
                *argv.offset(i as isize),
                b"-h\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                return usage(0 as libc::c_int)
            } else {
                if foundfile == 0 {
                    input = fopen(
                        *argv.offset(i as isize),
                        b"rb\0" as *const u8 as *const libc::c_char,
                    );
                    foundfile = 1 as libc::c_int;
                } else {
                    return usage(1 as libc::c_int)
                }
            }
        }
        i += 1;
    }
    if foundfile == 0 {
        input = stdin;
    }
    if !input.is_null() {} else {
        __assert_fail(
            b"input\0" as *const u8 as *const libc::c_char,
            b"run-parser-test-suite.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    if yaml_parser_initialize(&mut parser) == 0 {
        fprintf(
            stderr,
            b"Could not initialize the parser object\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    yaml_parser_set_input_file(&mut parser, input);
    loop {
        let mut type_0: yaml_event_type_t = YAML_NO_EVENT;
        if yaml_parser_parse(&mut parser, &mut event) == 0 {
            if parser.problem_mark.line != 0 || parser.problem_mark.column != 0 {
                fprintf(
                    stderr,
                    b"Parse error: %s\nLine: %lu Column: %lu\n\0" as *const u8
                        as *const libc::c_char,
                    parser.problem,
                    (parser.problem_mark.line)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    (parser.problem_mark.column)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            } else {
                fprintf(
                    stderr,
                    b"Parse error: %s\n\0" as *const u8 as *const libc::c_char,
                    parser.problem,
                );
            }
            return 1 as libc::c_int;
        }
        type_0 = event.type_0;
        if type_0 as libc::c_uint == YAML_NO_EVENT as libc::c_int as libc::c_uint {
            printf(b"???\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_STREAM_START_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"+STR\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"-STR\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_DOCUMENT_START_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"+DOC\0" as *const u8 as *const libc::c_char);
            if event.data.document_start.implicit == 0 {
                printf(b" ---\0" as *const u8 as *const libc::c_char);
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_DOCUMENT_END_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"-DOC\0" as *const u8 as *const libc::c_char);
            if event.data.document_end.implicit == 0 {
                printf(b" ...\0" as *const u8 as *const libc::c_char);
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_MAPPING_START_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"+MAP\0" as *const u8 as *const libc::c_char);
            if flow == 0 as libc::c_int
                && event.data.mapping_start.style as libc::c_uint
                    == YAML_FLOW_MAPPING_STYLE as libc::c_int as libc::c_uint
            {
                printf(b" {}\0" as *const u8 as *const libc::c_char);
            } else if flow == 1 as libc::c_int {
                printf(b" {}\0" as *const u8 as *const libc::c_char);
            }
            if !(event.data.mapping_start.anchor).is_null() {
                printf(
                    b" &%s\0" as *const u8 as *const libc::c_char,
                    event.data.mapping_start.anchor,
                );
            }
            if !(event.data.mapping_start.tag).is_null() {
                printf(
                    b" <%s>\0" as *const u8 as *const libc::c_char,
                    event.data.mapping_start.tag,
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"-MAP\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_SEQUENCE_START_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"+SEQ\0" as *const u8 as *const libc::c_char);
            if flow == 0 as libc::c_int
                && event.data.sequence_start.style as libc::c_uint
                    == YAML_FLOW_SEQUENCE_STYLE as libc::c_int as libc::c_uint
            {
                printf(b" []\0" as *const u8 as *const libc::c_char);
            } else if flow == 1 as libc::c_int {
                printf(b" []\0" as *const u8 as *const libc::c_char);
            }
            if !(event.data.sequence_start.anchor).is_null() {
                printf(
                    b" &%s\0" as *const u8 as *const libc::c_char,
                    event.data.sequence_start.anchor,
                );
            }
            if !(event.data.sequence_start.tag).is_null() {
                printf(
                    b" <%s>\0" as *const u8 as *const libc::c_char,
                    event.data.sequence_start.tag,
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"-SEQ\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_SCALAR_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"=VAL\0" as *const u8 as *const libc::c_char);
            if !(event.data.scalar.anchor).is_null() {
                printf(
                    b" &%s\0" as *const u8 as *const libc::c_char,
                    event.data.scalar.anchor,
                );
            }
            if !(event.data.scalar.tag).is_null() {
                printf(
                    b" <%s>\0" as *const u8 as *const libc::c_char,
                    event.data.scalar.tag,
                );
            }
            match event.data.scalar.style as libc::c_uint {
                1 => {
                    printf(b" :\0" as *const u8 as *const libc::c_char);
                }
                2 => {
                    printf(b" '\0" as *const u8 as *const libc::c_char);
                }
                3 => {
                    printf(b" \"\0" as *const u8 as *const libc::c_char);
                }
                4 => {
                    printf(b" |\0" as *const u8 as *const libc::c_char);
                }
                5 => {
                    printf(b" >\0" as *const u8 as *const libc::c_char);
                }
                0 => {
                    abort();
                }
                _ => {}
            }
            print_escaped(event.data.scalar.value, event.data.scalar.length);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_ALIAS_EVENT as libc::c_int as libc::c_uint
            {
            printf(
                b"=ALI *%s\n\0" as *const u8 as *const libc::c_char,
                event.data.alias.anchor,
            );
        } else {
            abort();
        }
        yaml_event_delete(&mut event);
        if type_0 as libc::c_uint == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint
        {
            break;
        }
    }
    if fclose(input) == 0 {} else {
        __assert_fail(
            b"!fclose(input)\0" as *const u8 as *const libc::c_char,
            b"run-parser-test-suite.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    yaml_parser_delete(&mut parser);
    fflush(stdout);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_escaped(mut str: *mut yaml_char_t, mut length: size_t) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < length {
        c = *str.offset(i as isize) as libc::c_char;
        if c as libc::c_int == '\\' as i32 {
            printf(b"\\\\\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\0' as i32 {
            printf(b"\\0\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\u{8}' as i32 {
            printf(b"\\b\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\n' as i32 {
            printf(b"\\n\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\r' as i32 {
            printf(b"\\r\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\t' as i32 {
            printf(b"\\t\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"%c\0" as *const u8 as *const libc::c_char, c as libc::c_int);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut ret: libc::c_int) -> libc::c_int {
    fprintf(
        stderr,
        b"Usage: libyaml-parser [--flow (on|off|keep)] [<input-file>]\n\0" as *const u8
            as *const libc::c_char,
    );
    return ret;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
