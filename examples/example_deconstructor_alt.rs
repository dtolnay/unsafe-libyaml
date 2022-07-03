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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn yaml_event_delete(event: *mut yaml_event_t);
    fn yaml_document_add_scalar(
        document: *mut yaml_document_t,
        tag: *const yaml_char_t,
        value: *const yaml_char_t,
        length: libc::c_int,
        style: yaml_scalar_style_t,
    ) -> libc::c_int;
    fn yaml_document_add_sequence(
        document: *mut yaml_document_t,
        tag: *const yaml_char_t,
        style: yaml_sequence_style_t,
    ) -> libc::c_int;
    fn yaml_document_append_sequence_item(
        document: *mut yaml_document_t,
        sequence: libc::c_int,
        item: libc::c_int,
    ) -> libc::c_int;
    fn yaml_document_append_mapping_pair(
        document: *mut yaml_document_t,
        mapping: libc::c_int,
        key: libc::c_int,
        value: libc::c_int,
    ) -> libc::c_int;
    fn yaml_document_delete(document: *mut yaml_document_t);
    fn yaml_document_add_mapping(
        document: *mut yaml_document_t,
        tag: *const yaml_char_t,
        style: yaml_mapping_style_t,
    ) -> libc::c_int;
    fn yaml_document_initialize(
        document: *mut yaml_document_t,
        version_directive: *mut yaml_version_directive_t,
        tag_directives_start: *mut yaml_tag_directive_t,
        tag_directives_end: *mut yaml_tag_directive_t,
        start_implicit: libc::c_int,
        end_implicit: libc::c_int,
    ) -> libc::c_int;
    fn yaml_emitter_set_canonical(emitter: *mut yaml_emitter_t, canonical: libc::c_int);
    fn yaml_parser_parse(
        parser: *mut yaml_parser_t,
        event: *mut yaml_event_t,
    ) -> libc::c_int;
    fn yaml_parser_set_input_file(parser: *mut yaml_parser_t, file: *mut FILE);
    fn yaml_emitter_set_output_file(emitter: *mut yaml_emitter_t, file: *mut FILE);
    fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> libc::c_int;
    fn yaml_emitter_delete(emitter: *mut yaml_emitter_t);
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> libc::c_int;
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_emitter_set_unicode(emitter: *mut yaml_emitter_t, unicode: libc::c_int);
    fn yaml_emitter_open(emitter: *mut yaml_emitter_t) -> libc::c_int;
    fn yaml_emitter_close(emitter: *mut yaml_emitter_t) -> libc::c_int;
    fn yaml_emitter_dump(
        emitter: *mut yaml_emitter_t,
        document: *mut yaml_document_t,
    ) -> libc::c_int;
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
pub type yaml_break_e = libc::c_uint;
pub const YAML_CRLN_BREAK: yaml_break_e = 3;
pub const YAML_LN_BREAK: yaml_break_e = 2;
pub const YAML_CR_BREAK: yaml_break_e = 1;
pub const YAML_ANY_BREAK: yaml_break_e = 0;
pub type yaml_break_t = yaml_break_e;
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
pub type yaml_write_handler_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_uchar,
    size_t,
) -> libc::c_int;
pub type yaml_emitter_state_e = libc::c_uint;
pub const YAML_EMIT_END_STATE: yaml_emitter_state_e = 17;
pub const YAML_EMIT_BLOCK_MAPPING_VALUE_STATE: yaml_emitter_state_e = 16;
pub const YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_e = 15;
pub const YAML_EMIT_BLOCK_MAPPING_KEY_STATE: yaml_emitter_state_e = 14;
pub const YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_e = 13;
pub const YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE: yaml_emitter_state_e = 12;
pub const YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_e = 11;
pub const YAML_EMIT_FLOW_MAPPING_VALUE_STATE: yaml_emitter_state_e = 10;
pub const YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_e = 9;
pub const YAML_EMIT_FLOW_MAPPING_KEY_STATE: yaml_emitter_state_e = 8;
pub const YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_e = 7;
pub const YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE: yaml_emitter_state_e = 6;
pub const YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_e = 5;
pub const YAML_EMIT_DOCUMENT_END_STATE: yaml_emitter_state_e = 4;
pub const YAML_EMIT_DOCUMENT_CONTENT_STATE: yaml_emitter_state_e = 3;
pub const YAML_EMIT_DOCUMENT_START_STATE: yaml_emitter_state_e = 2;
pub const YAML_EMIT_FIRST_DOCUMENT_START_STATE: yaml_emitter_state_e = 1;
pub const YAML_EMIT_STREAM_START_STATE: yaml_emitter_state_e = 0;
pub type yaml_emitter_state_t = yaml_emitter_state_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_anchors_s {
    pub references: libc::c_int,
    pub anchor: libc::c_int,
    pub serialized: libc::c_int,
}
pub type yaml_anchors_t = yaml_anchors_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_emitter_s {
    pub error: yaml_error_type_t,
    pub problem: *const libc::c_char,
    pub write_handler: Option::<yaml_write_handler_t>,
    pub write_handler_data: *mut libc::c_void,
    pub output: C2RustUnnamed_44,
    pub buffer: C2RustUnnamed_43,
    pub raw_buffer: C2RustUnnamed_42,
    pub encoding: yaml_encoding_t,
    pub canonical: libc::c_int,
    pub best_indent: libc::c_int,
    pub best_width: libc::c_int,
    pub unicode: libc::c_int,
    pub line_break: yaml_break_t,
    pub states: C2RustUnnamed_41,
    pub state: yaml_emitter_state_t,
    pub events: C2RustUnnamed_40,
    pub indents: C2RustUnnamed_39,
    pub tag_directives: C2RustUnnamed_38,
    pub indent: libc::c_int,
    pub flow_level: libc::c_int,
    pub root_context: libc::c_int,
    pub sequence_context: libc::c_int,
    pub mapping_context: libc::c_int,
    pub simple_key_context: libc::c_int,
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub whitespace: libc::c_int,
    pub indention: libc::c_int,
    pub open_ended: libc::c_int,
    pub anchor_data: C2RustUnnamed_37,
    pub tag_data: C2RustUnnamed_36,
    pub scalar_data: C2RustUnnamed_35,
    pub opened: libc::c_int,
    pub closed: libc::c_int,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: libc::c_int,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub multiline: libc::c_int,
    pub flow_plain_allowed: libc::c_int,
    pub block_plain_allowed: libc::c_int,
    pub single_quoted_allowed: libc::c_int,
    pub block_allowed: libc::c_int,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_43 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_44 {
    pub string: C2RustUnnamed_45,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
    pub size_written: *mut size_t,
}
pub type yaml_emitter_t = yaml_emitter_s;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut help: libc::c_int = 0 as libc::c_int;
    let mut canonical: libc::c_int = 0 as libc::c_int;
    let mut unicode: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut done: libc::c_int = 0 as libc::c_int;
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
    let mut emitter: yaml_emitter_t = yaml_emitter_t {
        error: YAML_NO_ERROR,
        problem: 0 as *const libc::c_char,
        write_handler: None,
        write_handler_data: 0 as *mut libc::c_void,
        output: C2RustUnnamed_44 {
            string: C2RustUnnamed_45 {
                buffer: 0 as *mut libc::c_uchar,
                size: 0,
                size_written: 0 as *mut size_t,
            },
        },
        buffer: C2RustUnnamed_43 {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
            last: 0 as *mut yaml_char_t,
        },
        raw_buffer: C2RustUnnamed_42 {
            start: 0 as *mut libc::c_uchar,
            end: 0 as *mut libc::c_uchar,
            pointer: 0 as *mut libc::c_uchar,
            last: 0 as *mut libc::c_uchar,
        },
        encoding: YAML_ANY_ENCODING,
        canonical: 0,
        best_indent: 0,
        best_width: 0,
        unicode: 0,
        line_break: YAML_ANY_BREAK,
        states: C2RustUnnamed_41 {
            start: 0 as *mut yaml_emitter_state_t,
            end: 0 as *mut yaml_emitter_state_t,
            top: 0 as *mut yaml_emitter_state_t,
        },
        state: YAML_EMIT_STREAM_START_STATE,
        events: C2RustUnnamed_40 {
            start: 0 as *mut yaml_event_t,
            end: 0 as *mut yaml_event_t,
            head: 0 as *mut yaml_event_t,
            tail: 0 as *mut yaml_event_t,
        },
        indents: C2RustUnnamed_39 {
            start: 0 as *mut libc::c_int,
            end: 0 as *mut libc::c_int,
            top: 0 as *mut libc::c_int,
        },
        tag_directives: C2RustUnnamed_38 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        },
        indent: 0,
        flow_level: 0,
        root_context: 0,
        sequence_context: 0,
        mapping_context: 0,
        simple_key_context: 0,
        line: 0,
        column: 0,
        whitespace: 0,
        indention: 0,
        open_ended: 0,
        anchor_data: C2RustUnnamed_37 {
            anchor: 0 as *mut yaml_char_t,
            anchor_length: 0,
            alias: 0,
        },
        tag_data: C2RustUnnamed_36 {
            handle: 0 as *mut yaml_char_t,
            handle_length: 0,
            suffix: 0 as *mut yaml_char_t,
            suffix_length: 0,
        },
        scalar_data: C2RustUnnamed_35 {
            value: 0 as *mut yaml_char_t,
            length: 0,
            multiline: 0,
            flow_plain_allowed: 0,
            block_plain_allowed: 0,
            single_quoted_allowed: 0,
            block_allowed: 0,
            style: YAML_ANY_SCALAR_STYLE,
        },
        opened: 0,
        closed: 0,
        anchors: 0 as *mut yaml_anchors_t,
        last_anchor_id: 0,
        document: 0 as *mut yaml_document_t,
    };
    let mut input_event: yaml_event_t = yaml_event_t {
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
    let mut output_document: yaml_document_t = yaml_document_t {
        nodes: C2RustUnnamed_23 {
            start: 0 as *mut yaml_node_t,
            end: 0 as *mut yaml_node_t,
            top: 0 as *mut yaml_node_t,
        },
        version_directive: 0 as *mut yaml_version_directive_t,
        tag_directives: C2RustUnnamed_22 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
        },
        start_implicit: 0,
        end_implicit: 0,
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
    let mut root: libc::c_int = 0;
    memset(
        &mut parser as *mut yaml_parser_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_parser_t>() as libc::c_ulong,
    );
    memset(
        &mut emitter as *mut yaml_emitter_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_emitter_t>() as libc::c_ulong,
    );
    memset(
        &mut input_event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    memset(
        &mut output_document as *mut yaml_document_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_document_t>() as libc::c_ulong,
    );
    k = 1 as libc::c_int;
    while k < argc {
        if strcmp(*argv.offset(k as isize), b"-h\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--help\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            help = 1 as libc::c_int;
        } else if strcmp(
                *argv.offset(k as isize),
                b"-c\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(k as isize),
                    b"--canonical\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
            canonical = 1 as libc::c_int;
        } else if strcmp(
                *argv.offset(k as isize),
                b"-u\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(k as isize),
                    b"--unicode\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
            unicode = 1 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"Unrecognized option: %s\nTry `%s --help` for more information.\n\0"
                    as *const u8 as *const libc::c_char,
                *argv.offset(k as isize),
                *argv.offset(0 as libc::c_int as isize),
            );
            return 1 as libc::c_int;
        }
        k += 1;
    }
    if help != 0 {
        printf(
            b"%s <input\nor\n%s -h | --help\nDeconstruct a YAML stream\n\nOptions:\n-h, --help\t\tdisplay this help and exit\n-c, --canonical\t\toutput in the canonical YAML format\n-u, --unicode\t\toutput unescaped non-ASCII characters\n\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
            *argv.offset(0 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    }
    if yaml_parser_initialize(&mut parser) == 0 {
        fprintf(
            stderr,
            b"Could not initialize the parser object\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if yaml_emitter_initialize(&mut emitter) == 0 {
        yaml_parser_delete(&mut parser);
        fprintf(
            stderr,
            b"Could not inialize the emitter object\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    yaml_parser_set_input_file(&mut parser, stdin);
    yaml_emitter_set_output_file(&mut emitter, stdout);
    yaml_emitter_set_canonical(&mut emitter, canonical);
    yaml_emitter_set_unicode(&mut emitter, unicode);
    if !(yaml_emitter_open(&mut emitter) == 0) {
        if yaml_document_initialize(
            &mut output_document,
            0 as *mut yaml_version_directive_t,
            0 as *mut yaml_tag_directive_t,
            0 as *mut yaml_tag_directive_t,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            current_block = 6124167286404711642;
        } else {
            root = yaml_document_add_sequence(
                &mut output_document,
                0 as *const yaml_char_t,
                YAML_BLOCK_SEQUENCE_STYLE,
            );
            if root == 0 {
                current_block = 6124167286404711642;
            } else {
                's_207: loop {
                    if !(done == 0) {
                        current_block = 9602112577253180622;
                        break;
                    }
                    let mut properties: libc::c_int = 0;
                    let mut key: libc::c_int = 0;
                    let mut value: libc::c_int = 0;
                    let mut map: libc::c_int = 0;
                    let mut seq: libc::c_int = 0;
                    if yaml_parser_parse(&mut parser, &mut input_event) == 0 {
                        current_block = 478310419506789813;
                        break;
                    }
                    if input_event.type_0 as libc::c_uint
                        == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint
                    {
                        done = 1 as libc::c_int;
                    }
                    properties = yaml_document_add_mapping(
                        &mut output_document,
                        0 as *const yaml_char_t,
                        YAML_BLOCK_MAPPING_STYLE,
                    );
                    if properties == 0 {
                        current_block = 6124167286404711642;
                        break;
                    }
                    if yaml_document_append_sequence_item(
                        &mut output_document,
                        root,
                        properties,
                    ) == 0
                    {
                        current_block = 6124167286404711642;
                        break;
                    }
                    match input_event.type_0 as libc::c_uint {
                        1 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"STREAM-START\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if input_event.data.stream_start.encoding as u64 != 0 {
                                let mut encoding: yaml_encoding_t = input_event
                                    .data
                                    .stream_start
                                    .encoding;
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"encoding\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    (if encoding as libc::c_uint
                                        == YAML_UTF8_ENCODING as libc::c_int as libc::c_uint
                                    {
                                        b"utf-8\0" as *const u8 as *const libc::c_char
                                    } else if encoding as libc::c_uint
                                            == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
                                        {
                                        b"utf-16-le\0" as *const u8 as *const libc::c_char
                                    } else if encoding as libc::c_uint
                                            == YAML_UTF16BE_ENCODING as libc::c_int as libc::c_uint
                                        {
                                        b"utf-16-be\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"unknown\0" as *const u8 as *const libc::c_char
                                    }) as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                        }
                        2 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"STREAM-END\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                        }
                        3 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"DOCUMENT-START\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if !(input_event.data.document_start.version_directive)
                                .is_null()
                            {
                                let mut version: *mut yaml_version_directive_t = input_event
                                    .data
                                    .document_start
                                    .version_directive;
                                let mut number: [libc::c_char; 64] = [0; 64];
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"version\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                map = yaml_document_add_mapping(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    YAML_FLOW_MAPPING_STYLE,
                                );
                                if map == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    map,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"major\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                sprintf(
                                    number.as_mut_ptr(),
                                    b"%d\0" as *const u8 as *const libc::c_char,
                                    (*version).major,
                                );
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    b"tag:yaml.org,2002:int\0" as *const u8
                                        as *const libc::c_char as *mut yaml_char_t,
                                    number.as_mut_ptr() as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    map,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"minor\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                sprintf(
                                    number.as_mut_ptr(),
                                    b"%d\0" as *const u8 as *const libc::c_char,
                                    (*version).minor,
                                );
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    b"tag:yaml.org,2002:int\0" as *const u8
                                        as *const libc::c_char as *mut yaml_char_t,
                                    number.as_mut_ptr() as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    map,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                            if input_event.data.document_start.tag_directives.start
                                != input_event.data.document_start.tag_directives.end
                            {
                                let mut tag: *mut yaml_tag_directive_t = 0
                                    as *mut yaml_tag_directive_t;
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"tags\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                seq = yaml_document_add_sequence(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    YAML_BLOCK_SEQUENCE_STYLE,
                                );
                                if seq == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    seq,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                tag = input_event.data.document_start.tag_directives.start;
                                while tag
                                    != input_event.data.document_start.tag_directives.end
                                {
                                    map = yaml_document_add_mapping(
                                        &mut output_document,
                                        0 as *const yaml_char_t,
                                        YAML_FLOW_MAPPING_STYLE,
                                    );
                                    if map == 0 {
                                        current_block = 6124167286404711642;
                                        break 's_207;
                                    }
                                    if yaml_document_append_sequence_item(
                                        &mut output_document,
                                        seq,
                                        map,
                                    ) == 0
                                    {
                                        current_block = 6124167286404711642;
                                        break 's_207;
                                    }
                                    key = yaml_document_add_scalar(
                                        &mut output_document,
                                        0 as *const yaml_char_t,
                                        b"handle\0" as *const u8 as *const libc::c_char
                                            as *mut yaml_char_t,
                                        -(1 as libc::c_int),
                                        YAML_PLAIN_SCALAR_STYLE,
                                    );
                                    if key == 0 {
                                        current_block = 6124167286404711642;
                                        break 's_207;
                                    }
                                    value = yaml_document_add_scalar(
                                        &mut output_document,
                                        0 as *const yaml_char_t,
                                        (*tag).handle,
                                        -(1 as libc::c_int),
                                        YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                    );
                                    if value == 0 {
                                        current_block = 6124167286404711642;
                                        break 's_207;
                                    }
                                    if yaml_document_append_mapping_pair(
                                        &mut output_document,
                                        map,
                                        key,
                                        value,
                                    ) == 0
                                    {
                                        current_block = 6124167286404711642;
                                        break 's_207;
                                    }
                                    key = yaml_document_add_scalar(
                                        &mut output_document,
                                        0 as *const yaml_char_t,
                                        b"prefix\0" as *const u8 as *const libc::c_char
                                            as *mut yaml_char_t,
                                        -(1 as libc::c_int),
                                        YAML_PLAIN_SCALAR_STYLE,
                                    );
                                    if key == 0 {
                                        current_block = 6124167286404711642;
                                        break 's_207;
                                    }
                                    value = yaml_document_add_scalar(
                                        &mut output_document,
                                        0 as *const yaml_char_t,
                                        (*tag).prefix,
                                        -(1 as libc::c_int),
                                        YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                    );
                                    if value == 0 {
                                        current_block = 6124167286404711642;
                                        break 's_207;
                                    }
                                    if yaml_document_append_mapping_pair(
                                        &mut output_document,
                                        map,
                                        key,
                                        value,
                                    ) == 0
                                    {
                                        current_block = 6124167286404711642;
                                        break 's_207;
                                    }
                                    tag = tag.offset(1);
                                }
                            }
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"implicit\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                b"tag:yaml.org,2002:bool\0" as *const u8
                                    as *const libc::c_char as *mut yaml_char_t,
                                (if input_event.data.document_start.implicit != 0 {
                                    b"true\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"false\0" as *const u8 as *const libc::c_char
                                }) as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                        }
                        4 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"DOCUMENT-END\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"implicit\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                b"tag:yaml.org,2002:bool\0" as *const u8
                                    as *const libc::c_char as *mut yaml_char_t,
                                (if input_event.data.document_end.implicit != 0 {
                                    b"true\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"false\0" as *const u8 as *const libc::c_char
                                }) as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                        }
                        5 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"ALIAS\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"anchor\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                input_event.data.alias.anchor,
                                -(1 as libc::c_int),
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                        }
                        6 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"SCALAR\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if !(input_event.data.scalar.anchor).is_null() {
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"anchor\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    input_event.data.scalar.anchor,
                                    -(1 as libc::c_int),
                                    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                            if !(input_event.data.scalar.tag).is_null() {
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"tag\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    input_event.data.scalar.tag,
                                    -(1 as libc::c_int),
                                    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"value\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                input_event.data.scalar.value,
                                input_event.data.scalar.length as libc::c_int,
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"version\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            map = yaml_document_add_mapping(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                YAML_FLOW_MAPPING_STYLE,
                            );
                            if map == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                map,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"plain\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                b"tag:yaml.org,2002:bool\0" as *const u8
                                    as *const libc::c_char as *mut yaml_char_t,
                                (if input_event.data.scalar.plain_implicit != 0 {
                                    b"true\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"false\0" as *const u8 as *const libc::c_char
                                }) as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                map,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"quoted\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                b"tag:yaml.org,2002:bool\0" as *const u8
                                    as *const libc::c_char as *mut yaml_char_t,
                                (if input_event.data.scalar.quoted_implicit != 0 {
                                    b"true\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"false\0" as *const u8 as *const libc::c_char
                                }) as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                map,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if input_event.data.scalar.style as u64 != 0 {
                                let mut style: yaml_scalar_style_t = input_event
                                    .data
                                    .scalar
                                    .style;
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"style\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    (if style as libc::c_uint
                                        == YAML_PLAIN_SCALAR_STYLE as libc::c_int as libc::c_uint
                                    {
                                        b"plain\0" as *const u8 as *const libc::c_char
                                    } else if style as libc::c_uint
                                            == YAML_SINGLE_QUOTED_SCALAR_STYLE as libc::c_int
                                                as libc::c_uint
                                        {
                                        b"single-quoted\0" as *const u8 as *const libc::c_char
                                    } else if style as libc::c_uint
                                            == YAML_DOUBLE_QUOTED_SCALAR_STYLE as libc::c_int
                                                as libc::c_uint
                                        {
                                        b"double-quoted\0" as *const u8 as *const libc::c_char
                                    } else if style as libc::c_uint
                                            == YAML_LITERAL_SCALAR_STYLE as libc::c_int as libc::c_uint
                                        {
                                        b"literal\0" as *const u8 as *const libc::c_char
                                    } else if style as libc::c_uint
                                            == YAML_FOLDED_SCALAR_STYLE as libc::c_int as libc::c_uint
                                        {
                                        b"folded\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"unknown\0" as *const u8 as *const libc::c_char
                                    }) as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                        }
                        7 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"SEQUENCE-START\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if !(input_event.data.sequence_start.anchor).is_null() {
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"anchor\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    input_event.data.sequence_start.anchor,
                                    -(1 as libc::c_int),
                                    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                            if !(input_event.data.sequence_start.tag).is_null() {
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"tag\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    input_event.data.sequence_start.tag,
                                    -(1 as libc::c_int),
                                    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"implicit\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                b"tag:yaml.org,2002:bool\0" as *const u8
                                    as *const libc::c_char as *mut yaml_char_t,
                                (if input_event.data.sequence_start.implicit != 0 {
                                    b"true\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"false\0" as *const u8 as *const libc::c_char
                                }) as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if input_event.data.sequence_start.style as u64 != 0 {
                                let mut style_0: yaml_sequence_style_t = input_event
                                    .data
                                    .sequence_start
                                    .style;
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"style\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    (if style_0 as libc::c_uint
                                        == YAML_BLOCK_SEQUENCE_STYLE as libc::c_int as libc::c_uint
                                    {
                                        b"block\0" as *const u8 as *const libc::c_char
                                    } else if style_0 as libc::c_uint
                                            == YAML_FLOW_SEQUENCE_STYLE as libc::c_int as libc::c_uint
                                        {
                                        b"flow\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"unknown\0" as *const u8 as *const libc::c_char
                                    }) as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                        }
                        8 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"SEQUENCE-END\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                        }
                        9 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"MAPPING-START\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if !(input_event.data.mapping_start.anchor).is_null() {
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"anchor\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    input_event.data.mapping_start.anchor,
                                    -(1 as libc::c_int),
                                    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                            if !(input_event.data.mapping_start.tag).is_null() {
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"tag\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    input_event.data.mapping_start.tag,
                                    -(1 as libc::c_int),
                                    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"implicit\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                b"tag:yaml.org,2002:bool\0" as *const u8
                                    as *const libc::c_char as *mut yaml_char_t,
                                (if input_event.data.mapping_start.implicit != 0 {
                                    b"true\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"false\0" as *const u8 as *const libc::c_char
                                }) as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if input_event.data.mapping_start.style as u64 != 0 {
                                let mut style_1: yaml_mapping_style_t = input_event
                                    .data
                                    .mapping_start
                                    .style;
                                key = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    b"style\0" as *const u8 as *const libc::c_char
                                        as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if key == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                value = yaml_document_add_scalar(
                                    &mut output_document,
                                    0 as *const yaml_char_t,
                                    (if style_1 as libc::c_uint
                                        == YAML_BLOCK_MAPPING_STYLE as libc::c_int as libc::c_uint
                                    {
                                        b"block\0" as *const u8 as *const libc::c_char
                                    } else if style_1 as libc::c_uint
                                            == YAML_FLOW_MAPPING_STYLE as libc::c_int as libc::c_uint
                                        {
                                        b"flow\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"unknown\0" as *const u8 as *const libc::c_char
                                    }) as *mut yaml_char_t,
                                    -(1 as libc::c_int),
                                    YAML_PLAIN_SCALAR_STYLE,
                                );
                                if value == 0 {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                                if yaml_document_append_mapping_pair(
                                    &mut output_document,
                                    properties,
                                    key,
                                    value,
                                ) == 0
                                {
                                    current_block = 6124167286404711642;
                                    break;
                                }
                            }
                        }
                        10 => {
                            key = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"type\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if key == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            value = yaml_document_add_scalar(
                                &mut output_document,
                                0 as *const yaml_char_t,
                                b"MAPPING-END\0" as *const u8 as *const libc::c_char
                                    as *mut yaml_char_t,
                                -(1 as libc::c_int),
                                YAML_PLAIN_SCALAR_STYLE,
                            );
                            if value == 0 {
                                current_block = 6124167286404711642;
                                break;
                            }
                            if yaml_document_append_mapping_pair(
                                &mut output_document,
                                properties,
                                key,
                                value,
                            ) == 0
                            {
                                current_block = 6124167286404711642;
                                break;
                            }
                        }
                        _ => {}
                    }
                    yaml_event_delete(&mut input_event);
                }
                match current_block {
                    6124167286404711642 => {}
                    _ => {
                        match current_block {
                            478310419506789813 => {
                                match parser.error as libc::c_uint {
                                    1 => {
                                        fprintf(
                                            stderr,
                                            b"Memory error: Not enough memory for parsing\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    2 => {
                                        if parser.problem_value != -(1 as libc::c_int) {
                                            fprintf(
                                                stderr,
                                                b"Reader error: %s: #%X at %zd\n\0" as *const u8
                                                    as *const libc::c_char,
                                                parser.problem,
                                                parser.problem_value,
                                                parser.problem_offset,
                                            );
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"Reader error: %s at %zd\n\0" as *const u8
                                                    as *const libc::c_char,
                                                parser.problem,
                                                parser.problem_offset,
                                            );
                                        }
                                    }
                                    3 => {
                                        if !(parser.context).is_null() {
                                            fprintf(
                                                stderr,
                                                b"Scanner error: %s at line %lu, column %lu\n%s at line %lu, column %lu\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                parser.context,
                                                (parser.context_mark.line)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                (parser.context_mark.column)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                parser.problem,
                                                (parser.problem_mark.line)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                (parser.problem_mark.column)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            );
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"Scanner error: %s at line %lu, column %lu\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                parser.problem,
                                                (parser.problem_mark.line)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                (parser.problem_mark.column)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            );
                                        }
                                    }
                                    4 => {
                                        if !(parser.context).is_null() {
                                            fprintf(
                                                stderr,
                                                b"Parser error: %s at line %lu, column %lu\n%s at line %lu, column %lu\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                parser.context,
                                                (parser.context_mark.line)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                (parser.context_mark.column)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                parser.problem,
                                                (parser.problem_mark.line)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                (parser.problem_mark.column)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            );
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"Parser error: %s at line %lu, column %lu\n\0" as *const u8
                                                    as *const libc::c_char,
                                                parser.problem,
                                                (parser.problem_mark.line)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                (parser.problem_mark.column)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            );
                                        }
                                    }
                                    _ => {
                                        fprintf(
                                            stderr,
                                            b"Internal error\n\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                }
                                yaml_event_delete(&mut input_event);
                                yaml_document_delete(&mut output_document);
                                yaml_parser_delete(&mut parser);
                                yaml_emitter_delete(&mut emitter);
                                return 1 as libc::c_int;
                            }
                            _ => {
                                if !(yaml_emitter_dump(&mut emitter, &mut output_document)
                                    == 0)
                                {
                                    if !(yaml_emitter_close(&mut emitter) == 0) {
                                        yaml_parser_delete(&mut parser);
                                        yaml_emitter_delete(&mut emitter);
                                        return 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                        current_block = 16626164785316884790;
                    }
                }
            }
        }
        match current_block {
            16626164785316884790 => {}
            _ => {
                fprintf(
                    stderr,
                    b"Memory error: Not enough memory for creating a document\n\0"
                        as *const u8 as *const libc::c_char,
                );
                yaml_event_delete(&mut input_event);
                yaml_document_delete(&mut output_document);
                yaml_parser_delete(&mut parser);
                yaml_emitter_delete(&mut emitter);
                return 1 as libc::c_int;
            }
        }
    }
    match emitter.error as libc::c_uint {
        1 => {
            fprintf(
                stderr,
                b"Memory error: Not enough memory for emitting\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        6 => {
            fprintf(
                stderr,
                b"Writer error: %s\n\0" as *const u8 as *const libc::c_char,
                emitter.problem,
            );
        }
        7 => {
            fprintf(
                stderr,
                b"Emitter error: %s\n\0" as *const u8 as *const libc::c_char,
                emitter.problem,
            );
        }
        _ => {
            fprintf(stderr, b"Internal error\n\0" as *const u8 as *const libc::c_char);
        }
    }
    yaml_event_delete(&mut input_event);
    yaml_document_delete(&mut output_document);
    yaml_parser_delete(&mut parser);
    yaml_emitter_delete(&mut emitter);
    return 1 as libc::c_int;
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
