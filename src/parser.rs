use crate::externs::__assert_fail;
use crate::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn yaml_malloc(size: size_t) -> *mut libc::c_void;
    fn yaml_free(ptr: *mut libc::c_void);
    fn yaml_parser_fetch_more_tokens(parser: *mut yaml_parser_t) -> libc::c_int;
    fn yaml_stack_extend(
        start: *mut *mut libc::c_void,
        top: *mut *mut libc::c_void,
        end: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn yaml_strdup(_: *const yaml_char_t) -> *mut yaml_char_t;
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
    pub data: Unnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed {
    pub stream_start: Unnamed_6,
    pub alias: Unnamed_5,
    pub anchor: Unnamed_4,
    pub tag: Unnamed_3,
    pub scalar: Unnamed_2,
    pub version_directive: Unnamed_1,
    pub tag_directive: Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_0 {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_1 {
    pub major: libc::c_int,
    pub minor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_2 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_3 {
    pub handle: *mut yaml_char_t,
    pub suffix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_4 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_5 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_6 {
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
    pub data: Unnamed_7,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed_7 {
    pub stream_start: Unnamed_15,
    pub document_start: Unnamed_13,
    pub document_end: Unnamed_12,
    pub alias: Unnamed_11,
    pub scalar: Unnamed_10,
    pub sequence_start: Unnamed_9,
    pub mapping_start: Unnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_8 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_9 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_10 {
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
pub struct Unnamed_11 {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_12 {
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_13 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: Unnamed_14,
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_14 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_15 {
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
    pub data: Unnamed_16,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed_16 {
    pub scalar: Unnamed_21,
    pub sequence: Unnamed_19,
    pub mapping: Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_17 {
    pub pairs: Unnamed_18,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_18 {
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
pub struct Unnamed_19 {
    pub items: Unnamed_20,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_20 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_21 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: Unnamed_23,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: Unnamed_22,
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_22 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_23 {
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
    pub input: Unnamed_33,
    pub eof: libc::c_int,
    pub buffer: Unnamed_32,
    pub unread: size_t,
    pub raw_buffer: Unnamed_31,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: libc::c_int,
    pub stream_end_produced: libc::c_int,
    pub flow_level: libc::c_int,
    pub tokens: Unnamed_30,
    pub tokens_parsed: size_t,
    pub token_available: libc::c_int,
    pub indents: Unnamed_29,
    pub indent: libc::c_int,
    pub simple_key_allowed: libc::c_int,
    pub simple_keys: Unnamed_28,
    pub states: Unnamed_27,
    pub state: yaml_parser_state_t,
    pub marks: Unnamed_26,
    pub tag_directives: Unnamed_25,
    pub aliases: Unnamed_24,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_24 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_25 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_26 {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_27 {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_28 {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_29 {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_30 {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_31 {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_32 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed_33 {
    pub string: Unnamed_34,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_34 {
    pub start: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
    pub current: *const libc::c_uchar,
}
pub type yaml_parser_t = yaml_parser_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_35 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_36 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_parse(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"parser.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int yaml_parser_parse(yaml_parser_t *, yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"parser.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int yaml_parser_parse(yaml_parser_t *, yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    if (*parser).stream_end_produced != 0 || (*parser).error as libc::c_uint != 0
        || (*parser).state as libc::c_uint
            == YAML_PARSE_END_STATE as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return yaml_parser_state_machine(parser, event);
}
unsafe extern "C" fn yaml_parser_set_parser_error(
    mut parser: *mut yaml_parser_t,
    mut problem: *const libc::c_char,
    mut problem_mark: yaml_mark_t,
) -> libc::c_int {
    (*parser).error = YAML_PARSER_ERROR;
    let ref mut fresh0 = (*parser).problem;
    *fresh0 = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_set_parser_error_context(
    mut parser: *mut yaml_parser_t,
    mut context: *const libc::c_char,
    mut context_mark: yaml_mark_t,
    mut problem: *const libc::c_char,
    mut problem_mark: yaml_mark_t,
) -> libc::c_int {
    (*parser).error = YAML_PARSER_ERROR;
    let ref mut fresh1 = (*parser).context;
    *fresh1 = context;
    (*parser).context_mark = context_mark;
    let ref mut fresh2 = (*parser).problem;
    *fresh2 = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_state_machine(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    match (*parser).state as libc::c_uint {
        0 => return yaml_parser_parse_stream_start(parser, event),
        1 => return yaml_parser_parse_document_start(parser, event, 1 as libc::c_int),
        2 => return yaml_parser_parse_document_start(parser, event, 0 as libc::c_int),
        3 => return yaml_parser_parse_document_content(parser, event),
        4 => return yaml_parser_parse_document_end(parser, event),
        5 => {
            return yaml_parser_parse_node(
                parser,
                event,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
        6 => {
            return yaml_parser_parse_node(
                parser,
                event,
                1 as libc::c_int,
                1 as libc::c_int,
            );
        }
        7 => {
            return yaml_parser_parse_node(
                parser,
                event,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
        8 => {
            return yaml_parser_parse_block_sequence_entry(
                parser,
                event,
                1 as libc::c_int,
            );
        }
        9 => {
            return yaml_parser_parse_block_sequence_entry(
                parser,
                event,
                0 as libc::c_int,
            );
        }
        10 => return yaml_parser_parse_indentless_sequence_entry(parser, event),
        11 => return yaml_parser_parse_block_mapping_key(parser, event, 1 as libc::c_int),
        12 => return yaml_parser_parse_block_mapping_key(parser, event, 0 as libc::c_int),
        13 => return yaml_parser_parse_block_mapping_value(parser, event),
        14 => {
            return yaml_parser_parse_flow_sequence_entry(parser, event, 1 as libc::c_int);
        }
        15 => {
            return yaml_parser_parse_flow_sequence_entry(parser, event, 0 as libc::c_int);
        }
        16 => return yaml_parser_parse_flow_sequence_entry_mapping_key(parser, event),
        17 => return yaml_parser_parse_flow_sequence_entry_mapping_value(parser, event),
        18 => return yaml_parser_parse_flow_sequence_entry_mapping_end(parser, event),
        19 => return yaml_parser_parse_flow_mapping_key(parser, event, 1 as libc::c_int),
        20 => return yaml_parser_parse_flow_mapping_key(parser, event, 0 as libc::c_int),
        21 => {
            return yaml_parser_parse_flow_mapping_value(parser, event, 0 as libc::c_int);
        }
        22 => {
            return yaml_parser_parse_flow_mapping_value(parser, event, 1 as libc::c_int);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_parse_stream_start(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint
        != YAML_STREAM_START_TOKEN as libc::c_int as libc::c_uint
    {
        return yaml_parser_set_parser_error(
            parser,
            b"did not find expected <stream-start>\0" as *const u8
                as *const libc::c_char,
            (*token).start_mark,
        );
    }
    (*parser).state = YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE;
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_STREAM_START_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).start_mark;
    (*event).data.stream_start.encoding = (*token).data.stream_start.encoding;
    (*parser).token_available = 0 as libc::c_int;
    let ref mut fresh3 = (*parser).tokens_parsed;
    *fresh3 = (*fresh3).wrapping_add(1);
    (*parser)
        .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
        == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
    let ref mut fresh4 = (*parser).tokens.head;
    *fresh4 = (*fresh4).offset(1);
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_parse_document_start(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut implicit: libc::c_int,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    let mut version_directive: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut tag_directives: Unnamed_35 = {
        let mut init = Unnamed_35 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
        };
        init
    };
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if implicit == 0 {
        while (*token).type_0 as libc::c_uint
            == YAML_DOCUMENT_END_TOKEN as libc::c_int as libc::c_uint
        {
            (*parser).token_available = 0 as libc::c_int;
            let ref mut fresh5 = (*parser).tokens_parsed;
            *fresh5 = (*fresh5).wrapping_add(1);
            (*parser)
                .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
                == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
            let ref mut fresh6 = (*parser).tokens.head;
            *fresh6 = (*fresh6).offset(1);
            token = if (*parser).token_available != 0
                || yaml_parser_fetch_more_tokens(parser) != 0
            {
                (*parser).tokens.head
            } else {
                0 as *mut yaml_token_t
            };
            if token.is_null() {
                return 0 as libc::c_int;
            }
        }
    }
    if implicit != 0
        && (*token).type_0 as libc::c_uint
            != YAML_VERSION_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_0 as libc::c_uint
            != YAML_TAG_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_0 as libc::c_uint
            != YAML_DOCUMENT_START_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_0 as libc::c_uint
            != YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint
    {
        if yaml_parser_process_directives(
            parser,
            0 as *mut *mut yaml_version_directive_t,
            0 as *mut *mut yaml_tag_directive_t,
            0 as *mut *mut yaml_tag_directive_t,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if if (*parser).states.top != (*parser).states.end
            || yaml_stack_extend(
                &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
                &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
                &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh7 = (*parser).states.top;
            let fresh8 = *fresh7;
            *fresh7 = (*fresh7).offset(1);
            *fresh8 = YAML_PARSE_DOCUMENT_END_STATE;
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        (*parser).state = YAML_PARSE_BLOCK_NODE_STATE;
        memset(
            event as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_0 = YAML_DOCUMENT_START_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).start_mark;
        let ref mut fresh9 = (*event).data.document_start.version_directive;
        *fresh9 = 0 as *mut yaml_version_directive_t;
        let ref mut fresh10 = (*event).data.document_start.tag_directives.start;
        *fresh10 = 0 as *mut yaml_tag_directive_t;
        let ref mut fresh11 = (*event).data.document_start.tag_directives.end;
        *fresh11 = 0 as *mut yaml_tag_directive_t;
        (*event).data.document_start.implicit = 1 as libc::c_int;
        return 1 as libc::c_int;
    } else if (*token).type_0 as libc::c_uint
            != YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint
        {
        let mut start_mark: yaml_mark_t = yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        };
        let mut end_mark: yaml_mark_t = yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        };
        start_mark = (*token).start_mark;
        if yaml_parser_process_directives(
            parser,
            &mut version_directive,
            &mut tag_directives.start,
            &mut tag_directives.end,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if !token.is_null() {
            if (*token).type_0 as libc::c_uint
                != YAML_DOCUMENT_START_TOKEN as libc::c_int as libc::c_uint
            {
                yaml_parser_set_parser_error(
                    parser,
                    b"did not find expected <document start>\0" as *const u8
                        as *const libc::c_char,
                    (*token).start_mark,
                );
            } else if !(if (*parser).states.top != (*parser).states.end
                    || yaml_stack_extend(
                        &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let ref mut fresh12 = (*parser).states.top;
                    let fresh13 = *fresh12;
                    *fresh12 = (*fresh12).offset(1);
                    *fresh13 = YAML_PARSE_DOCUMENT_END_STATE;
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                (*parser).state = YAML_PARSE_DOCUMENT_CONTENT_STATE;
                end_mark = (*token).end_mark;
                memset(
                    event as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                );
                (*event).type_0 = YAML_DOCUMENT_START_EVENT;
                (*event).start_mark = start_mark;
                (*event).end_mark = end_mark;
                let ref mut fresh14 = (*event).data.document_start.version_directive;
                *fresh14 = version_directive;
                let ref mut fresh15 = (*event).data.document_start.tag_directives.start;
                *fresh15 = tag_directives.start;
                let ref mut fresh16 = (*event).data.document_start.tag_directives.end;
                *fresh16 = tag_directives.end;
                (*event).data.document_start.implicit = 0 as libc::c_int;
                (*parser).token_available = 0 as libc::c_int;
                let ref mut fresh17 = (*parser).tokens_parsed;
                *fresh17 = (*fresh17).wrapping_add(1);
                (*parser)
                    .stream_end_produced = ((*(*parser).tokens.head).type_0
                    as libc::c_uint
                    == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let ref mut fresh18 = (*parser).tokens.head;
                *fresh18 = (*fresh18).offset(1);
                version_directive = 0 as *mut yaml_version_directive_t;
                tag_directives.end = 0 as *mut yaml_tag_directive_t;
                tag_directives.start = tag_directives.end;
                return 1 as libc::c_int;
            }
        }
        yaml_free(version_directive as *mut libc::c_void);
        while tag_directives.start != tag_directives.end {
            yaml_free(
                (*(tag_directives.end).offset(-(1 as libc::c_int) as isize)).handle
                    as *mut libc::c_void,
            );
            yaml_free(
                (*(tag_directives.end).offset(-(1 as libc::c_int) as isize)).prefix
                    as *mut libc::c_void,
            );
            tag_directives.end = (tag_directives.end).offset(-1);
        }
        yaml_free(tag_directives.start as *mut libc::c_void);
        return 0 as libc::c_int;
    } else {
        (*parser).state = YAML_PARSE_END_STATE;
        memset(
            event as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_0 = YAML_STREAM_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh19 = (*parser).tokens_parsed;
        *fresh19 = (*fresh19).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh20 = (*parser).tokens.head;
        *fresh20 = (*fresh20).offset(1);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn yaml_parser_parse_document_content(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint
        == YAML_VERSION_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
        || (*token).type_0 as libc::c_uint
            == YAML_TAG_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
        || (*token).type_0 as libc::c_uint
            == YAML_DOCUMENT_START_TOKEN as libc::c_int as libc::c_uint
        || (*token).type_0 as libc::c_uint
            == YAML_DOCUMENT_END_TOKEN as libc::c_int as libc::c_uint
        || (*token).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint
    {
        let ref mut fresh21 = (*parser).states.top;
        *fresh21 = (*fresh21).offset(-1);
        (*parser).state = **fresh21;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    } else {
        return yaml_parser_parse_node(parser, event, 1 as libc::c_int, 0 as libc::c_int)
    };
}
unsafe extern "C" fn yaml_parser_parse_document_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut implicit: libc::c_int = 1 as libc::c_int;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    end_mark = (*token).start_mark;
    start_mark = end_mark;
    if (*token).type_0 as libc::c_uint
        == YAML_DOCUMENT_END_TOKEN as libc::c_int as libc::c_uint
    {
        end_mark = (*token).end_mark;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh22 = (*parser).tokens_parsed;
        *fresh22 = (*fresh22).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh23 = (*parser).tokens.head;
        *fresh23 = (*fresh23).offset(1);
        implicit = 0 as libc::c_int;
    }
    while !((*parser).tag_directives.start == (*parser).tag_directives.top) {
        let ref mut fresh24 = (*parser).tag_directives.top;
        *fresh24 = (*fresh24).offset(-1);
        let mut tag_directive: yaml_tag_directive_t = **fresh24;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    (*parser).state = YAML_PARSE_DOCUMENT_START_STATE;
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_DOCUMENT_END_EVENT;
    (*event).start_mark = start_mark;
    (*event).end_mark = end_mark;
    (*event).data.document_end.implicit = implicit;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_parse_node(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut block: libc::c_int,
    mut indentless_sequence: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    let mut anchor: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_handle: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_suffix: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut tag_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut implicit: libc::c_int = 0;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint == YAML_ALIAS_TOKEN as libc::c_int as libc::c_uint
    {
        let ref mut fresh25 = (*parser).states.top;
        *fresh25 = (*fresh25).offset(-1);
        (*parser).state = **fresh25;
        memset(
            event as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_0 = YAML_ALIAS_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        let ref mut fresh26 = (*event).data.alias.anchor;
        *fresh26 = (*token).data.alias.value;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh27 = (*parser).tokens_parsed;
        *fresh27 = (*fresh27).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh28 = (*parser).tokens.head;
        *fresh28 = (*fresh28).offset(1);
        return 1 as libc::c_int;
    } else {
        end_mark = (*token).start_mark;
        start_mark = end_mark;
        if (*token).type_0 as libc::c_uint
            == YAML_ANCHOR_TOKEN as libc::c_int as libc::c_uint
        {
            anchor = (*token).data.anchor.value;
            start_mark = (*token).start_mark;
            end_mark = (*token).end_mark;
            (*parser).token_available = 0 as libc::c_int;
            let ref mut fresh29 = (*parser).tokens_parsed;
            *fresh29 = (*fresh29).wrapping_add(1);
            (*parser)
                .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
                == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
            let ref mut fresh30 = (*parser).tokens.head;
            *fresh30 = (*fresh30).offset(1);
            token = if (*parser).token_available != 0
                || yaml_parser_fetch_more_tokens(parser) != 0
            {
                (*parser).tokens.head
            } else {
                0 as *mut yaml_token_t
            };
            if token.is_null() {
                current_block = 17786380918591080555;
            } else if (*token).type_0 as libc::c_uint
                    == YAML_TAG_TOKEN as libc::c_int as libc::c_uint
                {
                tag_handle = (*token).data.tag.handle;
                tag_suffix = (*token).data.tag.suffix;
                tag_mark = (*token).start_mark;
                end_mark = (*token).end_mark;
                (*parser).token_available = 0 as libc::c_int;
                let ref mut fresh31 = (*parser).tokens_parsed;
                *fresh31 = (*fresh31).wrapping_add(1);
                (*parser)
                    .stream_end_produced = ((*(*parser).tokens.head).type_0
                    as libc::c_uint
                    == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let ref mut fresh32 = (*parser).tokens.head;
                *fresh32 = (*fresh32).offset(1);
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    current_block = 17786380918591080555;
                } else {
                    current_block = 11743904203796629665;
                }
            } else {
                current_block = 11743904203796629665;
            }
        } else if (*token).type_0 as libc::c_uint
                == YAML_TAG_TOKEN as libc::c_int as libc::c_uint
            {
            tag_handle = (*token).data.tag.handle;
            tag_suffix = (*token).data.tag.suffix;
            tag_mark = (*token).start_mark;
            start_mark = tag_mark;
            end_mark = (*token).end_mark;
            (*parser).token_available = 0 as libc::c_int;
            let ref mut fresh33 = (*parser).tokens_parsed;
            *fresh33 = (*fresh33).wrapping_add(1);
            (*parser)
                .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
                == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
            let ref mut fresh34 = (*parser).tokens.head;
            *fresh34 = (*fresh34).offset(1);
            token = if (*parser).token_available != 0
                || yaml_parser_fetch_more_tokens(parser) != 0
            {
                (*parser).tokens.head
            } else {
                0 as *mut yaml_token_t
            };
            if token.is_null() {
                current_block = 17786380918591080555;
            } else if (*token).type_0 as libc::c_uint
                    == YAML_ANCHOR_TOKEN as libc::c_int as libc::c_uint
                {
                anchor = (*token).data.anchor.value;
                end_mark = (*token).end_mark;
                (*parser).token_available = 0 as libc::c_int;
                let ref mut fresh35 = (*parser).tokens_parsed;
                *fresh35 = (*fresh35).wrapping_add(1);
                (*parser)
                    .stream_end_produced = ((*(*parser).tokens.head).type_0
                    as libc::c_uint
                    == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let ref mut fresh36 = (*parser).tokens.head;
                *fresh36 = (*fresh36).offset(1);
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    current_block = 17786380918591080555;
                } else {
                    current_block = 11743904203796629665;
                }
            } else {
                current_block = 11743904203796629665;
            }
        } else {
            current_block = 11743904203796629665;
        }
        match current_block {
            11743904203796629665 => {
                if !tag_handle.is_null() {
                    if *tag_handle == 0 {
                        tag = tag_suffix;
                        yaml_free(tag_handle as *mut libc::c_void);
                        tag_suffix = 0 as *mut yaml_char_t;
                        tag_handle = tag_suffix;
                        current_block = 9437013279121998969;
                    } else {
                        let mut tag_directive: *mut yaml_tag_directive_t = 0
                            as *mut yaml_tag_directive_t;
                        tag_directive = (*parser).tag_directives.start;
                        loop {
                            if !(tag_directive != (*parser).tag_directives.top) {
                                current_block = 17728966195399430138;
                                break;
                            }
                            if strcmp(
                                (*tag_directive).handle as *mut libc::c_char,
                                tag_handle as *mut libc::c_char,
                            ) == 0 as libc::c_int
                            {
                                let mut prefix_len: size_t = strlen(
                                    (*tag_directive).prefix as *mut libc::c_char,
                                );
                                let mut suffix_len: size_t = strlen(
                                    tag_suffix as *mut libc::c_char,
                                );
                                tag = yaml_malloc(
                                    prefix_len
                                        .wrapping_add(suffix_len)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as *mut yaml_char_t;
                                if tag.is_null() {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    current_block = 17786380918591080555;
                                    break;
                                } else {
                                    memcpy(
                                        tag as *mut libc::c_void,
                                        (*tag_directive).prefix as *const libc::c_void,
                                        prefix_len,
                                    );
                                    memcpy(
                                        tag.offset(prefix_len as isize) as *mut libc::c_void,
                                        tag_suffix as *const libc::c_void,
                                        suffix_len,
                                    );
                                    *tag
                                        .offset(
                                            prefix_len.wrapping_add(suffix_len) as isize,
                                        ) = '\0' as i32 as yaml_char_t;
                                    yaml_free(tag_handle as *mut libc::c_void);
                                    yaml_free(tag_suffix as *mut libc::c_void);
                                    tag_suffix = 0 as *mut yaml_char_t;
                                    tag_handle = tag_suffix;
                                    current_block = 17728966195399430138;
                                    break;
                                }
                            } else {
                                tag_directive = tag_directive.offset(1);
                            }
                        }
                        match current_block {
                            17786380918591080555 => {}
                            _ => {
                                if tag.is_null() {
                                    yaml_parser_set_parser_error_context(
                                        parser,
                                        b"while parsing a node\0" as *const u8
                                            as *const libc::c_char,
                                        start_mark,
                                        b"found undefined tag handle\0" as *const u8
                                            as *const libc::c_char,
                                        tag_mark,
                                    );
                                    current_block = 17786380918591080555;
                                } else {
                                    current_block = 9437013279121998969;
                                }
                            }
                        }
                    }
                } else {
                    current_block = 9437013279121998969;
                }
                match current_block {
                    17786380918591080555 => {}
                    _ => {
                        implicit = (tag.is_null() || *tag == 0) as libc::c_int;
                        if indentless_sequence != 0
                            && (*token).type_0 as libc::c_uint
                                == YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_0 = YAML_SEQUENCE_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let ref mut fresh37 = (*event).data.sequence_start.anchor;
                            *fresh37 = anchor;
                            let ref mut fresh38 = (*event).data.sequence_start.tag;
                            *fresh38 = tag;
                            (*event).data.sequence_start.implicit = implicit;
                            (*event)
                                .data
                                .sequence_start
                                .style = YAML_BLOCK_SEQUENCE_STYLE;
                            return 1 as libc::c_int;
                        } else if (*token).type_0 as libc::c_uint
                                == YAML_SCALAR_TOKEN as libc::c_int as libc::c_uint
                            {
                            let mut plain_implicit: libc::c_int = 0 as libc::c_int;
                            let mut quoted_implicit: libc::c_int = 0 as libc::c_int;
                            end_mark = (*token).end_mark;
                            if (*token).data.scalar.style as libc::c_uint
                                == YAML_PLAIN_SCALAR_STYLE as libc::c_int as libc::c_uint
                                && tag.is_null()
                                || !tag.is_null()
                                    && strcmp(
                                        tag as *mut libc::c_char,
                                        b"!\0" as *const u8 as *const libc::c_char,
                                    ) == 0 as libc::c_int
                            {
                                plain_implicit = 1 as libc::c_int;
                            } else if tag.is_null() {
                                quoted_implicit = 1 as libc::c_int;
                            }
                            let ref mut fresh39 = (*parser).states.top;
                            *fresh39 = (*fresh39).offset(-1);
                            (*parser).state = **fresh39;
                            memset(
                                event as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_0 = YAML_SCALAR_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let ref mut fresh40 = (*event).data.scalar.anchor;
                            *fresh40 = anchor;
                            let ref mut fresh41 = (*event).data.scalar.tag;
                            *fresh41 = tag;
                            let ref mut fresh42 = (*event).data.scalar.value;
                            *fresh42 = (*token).data.scalar.value;
                            (*event).data.scalar.length = (*token).data.scalar.length;
                            (*event).data.scalar.plain_implicit = plain_implicit;
                            (*event).data.scalar.quoted_implicit = quoted_implicit;
                            (*event).data.scalar.style = (*token).data.scalar.style;
                            (*parser).token_available = 0 as libc::c_int;
                            let ref mut fresh43 = (*parser).tokens_parsed;
                            *fresh43 = (*fresh43).wrapping_add(1);
                            (*parser)
                                .stream_end_produced = ((*(*parser).tokens.head).type_0
                                as libc::c_uint
                                == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint)
                                as libc::c_int;
                            let ref mut fresh44 = (*parser).tokens.head;
                            *fresh44 = (*fresh44).offset(1);
                            return 1 as libc::c_int;
                        } else if (*token).type_0 as libc::c_uint
                                == YAML_FLOW_SEQUENCE_START_TOKEN as libc::c_int
                                    as libc::c_uint
                            {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_0 = YAML_SEQUENCE_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let ref mut fresh45 = (*event).data.sequence_start.anchor;
                            *fresh45 = anchor;
                            let ref mut fresh46 = (*event).data.sequence_start.tag;
                            *fresh46 = tag;
                            (*event).data.sequence_start.implicit = implicit;
                            (*event)
                                .data
                                .sequence_start
                                .style = YAML_FLOW_SEQUENCE_STYLE;
                            return 1 as libc::c_int;
                        } else if (*token).type_0 as libc::c_uint
                                == YAML_FLOW_MAPPING_START_TOKEN as libc::c_int
                                    as libc::c_uint
                            {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_0 = YAML_MAPPING_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let ref mut fresh47 = (*event).data.mapping_start.anchor;
                            *fresh47 = anchor;
                            let ref mut fresh48 = (*event).data.mapping_start.tag;
                            *fresh48 = tag;
                            (*event).data.mapping_start.implicit = implicit;
                            (*event).data.mapping_start.style = YAML_FLOW_MAPPING_STYLE;
                            return 1 as libc::c_int;
                        } else if block != 0
                                && (*token).type_0 as libc::c_uint
                                    == YAML_BLOCK_SEQUENCE_START_TOKEN as libc::c_int
                                        as libc::c_uint
                            {
                            end_mark = (*token).end_mark;
                            (*parser)
                                .state = YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_0 = YAML_SEQUENCE_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let ref mut fresh49 = (*event).data.sequence_start.anchor;
                            *fresh49 = anchor;
                            let ref mut fresh50 = (*event).data.sequence_start.tag;
                            *fresh50 = tag;
                            (*event).data.sequence_start.implicit = implicit;
                            (*event)
                                .data
                                .sequence_start
                                .style = YAML_BLOCK_SEQUENCE_STYLE;
                            return 1 as libc::c_int;
                        } else if block != 0
                                && (*token).type_0 as libc::c_uint
                                    == YAML_BLOCK_MAPPING_START_TOKEN as libc::c_int
                                        as libc::c_uint
                            {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_0 = YAML_MAPPING_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let ref mut fresh51 = (*event).data.mapping_start.anchor;
                            *fresh51 = anchor;
                            let ref mut fresh52 = (*event).data.mapping_start.tag;
                            *fresh52 = tag;
                            (*event).data.mapping_start.implicit = implicit;
                            (*event).data.mapping_start.style = YAML_BLOCK_MAPPING_STYLE;
                            return 1 as libc::c_int;
                        } else if !anchor.is_null() || !tag.is_null() {
                            let mut value: *mut yaml_char_t = yaml_malloc(
                                1 as libc::c_int as size_t,
                            ) as *mut yaml_char_t;
                            if value.is_null() {
                                (*parser).error = YAML_MEMORY_ERROR;
                            } else {
                                *value
                                    .offset(
                                        0 as libc::c_int as isize,
                                    ) = '\0' as i32 as yaml_char_t;
                                let ref mut fresh53 = (*parser).states.top;
                                *fresh53 = (*fresh53).offset(-1);
                                (*parser).state = **fresh53;
                                memset(
                                    event as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                                );
                                (*event).type_0 = YAML_SCALAR_EVENT;
                                (*event).start_mark = start_mark;
                                (*event).end_mark = end_mark;
                                let ref mut fresh54 = (*event).data.scalar.anchor;
                                *fresh54 = anchor;
                                let ref mut fresh55 = (*event).data.scalar.tag;
                                *fresh55 = tag;
                                let ref mut fresh56 = (*event).data.scalar.value;
                                *fresh56 = value;
                                (*event).data.scalar.length = 0 as libc::c_int as size_t;
                                (*event).data.scalar.plain_implicit = implicit;
                                (*event).data.scalar.quoted_implicit = 0 as libc::c_int;
                                (*event).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
                                return 1 as libc::c_int;
                            }
                        } else {
                            yaml_parser_set_parser_error_context(
                                parser,
                                if block != 0 {
                                    b"while parsing a block node\0" as *const u8
                                        as *const libc::c_char
                                } else {
                                    b"while parsing a flow node\0" as *const u8
                                        as *const libc::c_char
                                },
                                start_mark,
                                b"did not find expected node content\0" as *const u8
                                    as *const libc::c_char,
                                (*token).start_mark,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        yaml_free(anchor as *mut libc::c_void);
        yaml_free(tag_handle as *mut libc::c_void);
        yaml_free(tag_suffix as *mut libc::c_void);
        yaml_free(tag as *mut libc::c_void);
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn yaml_parser_parse_block_sequence_entry(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut first: libc::c_int,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    if first != 0 {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if if (*parser).marks.top != (*parser).marks.end
            || yaml_stack_extend(
                &mut (*parser).marks.start as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.top as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.end as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh57 = (*parser).marks.top;
            let fresh58 = *fresh57;
            *fresh57 = (*fresh57).offset(1);
            *fresh58 = (*token).start_mark;
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh59 = (*parser).tokens_parsed;
        *fresh59 = (*fresh59).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh60 = (*parser).tokens.head;
        *fresh60 = (*fresh60).offset(1);
    }
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint
        == YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint
    {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh61 = (*parser).tokens_parsed;
        *fresh61 = (*fresh61).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh62 = (*parser).tokens.head;
        *fresh62 = (*fresh62).offset(1);
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as libc::c_int;
        }
        if (*token).type_0 as libc::c_uint
            != YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let ref mut fresh63 = (*parser).states.top;
                let fresh64 = *fresh63;
                *fresh63 = (*fresh63).offset(1);
                *fresh64 = YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE;
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
            return yaml_parser_parse_node(
                parser,
                event,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        } else {
            (*parser).state = YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else if (*token).type_0 as libc::c_uint
            == YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
        let ref mut fresh65 = (*parser).states.top;
        *fresh65 = (*fresh65).offset(-1);
        (*parser).state = **fresh65;
        let ref mut fresh66 = (*parser).marks.top;
        *fresh66 = (*fresh66).offset(-1);
        memset(
            event as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_0 = YAML_SEQUENCE_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh67 = (*parser).tokens_parsed;
        *fresh67 = (*fresh67).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh68 = (*parser).tokens.head;
        *fresh68 = (*fresh68).offset(1);
        return 1 as libc::c_int;
    } else {
        let ref mut fresh69 = (*parser).marks.top;
        *fresh69 = (*fresh69).offset(-1);
        return yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block collection\0" as *const u8 as *const libc::c_char,
            **fresh69,
            b"did not find expected '-' indicator\0" as *const u8 as *const libc::c_char,
            (*token).start_mark,
        );
    };
}
unsafe extern "C" fn yaml_parser_parse_indentless_sequence_entry(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint
        == YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint
    {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh70 = (*parser).tokens_parsed;
        *fresh70 = (*fresh70).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh71 = (*parser).tokens.head;
        *fresh71 = (*fresh71).offset(1);
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as libc::c_int;
        }
        if (*token).type_0 as libc::c_uint
            != YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_KEY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let ref mut fresh72 = (*parser).states.top;
                let fresh73 = *fresh72;
                *fresh72 = (*fresh72).offset(1);
                *fresh73 = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
            return yaml_parser_parse_node(
                parser,
                event,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        } else {
            (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else {
        let ref mut fresh74 = (*parser).states.top;
        *fresh74 = (*fresh74).offset(-1);
        (*parser).state = **fresh74;
        memset(
            event as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_0 = YAML_SEQUENCE_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).start_mark;
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn yaml_parser_parse_block_mapping_key(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut first: libc::c_int,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    if first != 0 {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if if (*parser).marks.top != (*parser).marks.end
            || yaml_stack_extend(
                &mut (*parser).marks.start as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.top as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.end as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh75 = (*parser).marks.top;
            let fresh76 = *fresh75;
            *fresh75 = (*fresh75).offset(1);
            *fresh76 = (*token).start_mark;
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh77 = (*parser).tokens_parsed;
        *fresh77 = (*fresh77).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh78 = (*parser).tokens.head;
        *fresh78 = (*fresh78).offset(1);
    }
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint == YAML_KEY_TOKEN as libc::c_int as libc::c_uint {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh79 = (*parser).tokens_parsed;
        *fresh79 = (*fresh79).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh80 = (*parser).tokens.head;
        *fresh80 = (*fresh80).offset(1);
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as libc::c_int;
        }
        if (*token).type_0 as libc::c_uint
            != YAML_KEY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let ref mut fresh81 = (*parser).states.top;
                let fresh82 = *fresh81;
                *fresh81 = (*fresh81).offset(1);
                *fresh82 = YAML_PARSE_BLOCK_MAPPING_VALUE_STATE;
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
            return yaml_parser_parse_node(
                parser,
                event,
                1 as libc::c_int,
                1 as libc::c_int,
            );
        } else {
            (*parser).state = YAML_PARSE_BLOCK_MAPPING_VALUE_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else if (*token).type_0 as libc::c_uint
            == YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
        let ref mut fresh83 = (*parser).states.top;
        *fresh83 = (*fresh83).offset(-1);
        (*parser).state = **fresh83;
        let ref mut fresh84 = (*parser).marks.top;
        *fresh84 = (*fresh84).offset(-1);
        memset(
            event as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_0 = YAML_MAPPING_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh85 = (*parser).tokens_parsed;
        *fresh85 = (*fresh85).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh86 = (*parser).tokens.head;
        *fresh86 = (*fresh86).offset(1);
        return 1 as libc::c_int;
    } else {
        let ref mut fresh87 = (*parser).marks.top;
        *fresh87 = (*fresh87).offset(-1);
        return yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block mapping\0" as *const u8 as *const libc::c_char,
            **fresh87,
            b"did not find expected key\0" as *const u8 as *const libc::c_char,
            (*token).start_mark,
        );
    };
}
unsafe extern "C" fn yaml_parser_parse_block_mapping_value(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint == YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
    {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh88 = (*parser).tokens_parsed;
        *fresh88 = (*fresh88).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh89 = (*parser).tokens.head;
        *fresh89 = (*fresh89).offset(1);
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as libc::c_int;
        }
        if (*token).type_0 as libc::c_uint
            != YAML_KEY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let ref mut fresh90 = (*parser).states.top;
                let fresh91 = *fresh90;
                *fresh90 = (*fresh90).offset(1);
                *fresh91 = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
            return yaml_parser_parse_node(
                parser,
                event,
                1 as libc::c_int,
                1 as libc::c_int,
            );
        } else {
            (*parser).state = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else {
        (*parser).state = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    };
}
unsafe extern "C" fn yaml_parser_parse_flow_sequence_entry(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut first: libc::c_int,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    if first != 0 {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if if (*parser).marks.top != (*parser).marks.end
            || yaml_stack_extend(
                &mut (*parser).marks.start as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.top as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.end as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh92 = (*parser).marks.top;
            let fresh93 = *fresh92;
            *fresh92 = (*fresh92).offset(1);
            *fresh93 = (*token).start_mark;
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh94 = (*parser).tokens_parsed;
        *fresh94 = (*fresh94).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh95 = (*parser).tokens.head;
        *fresh95 = (*fresh95).offset(1);
    }
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint
        != YAML_FLOW_SEQUENCE_END_TOKEN as libc::c_int as libc::c_uint
    {
        if first == 0 {
            if (*token).type_0 as libc::c_uint
                == YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
            {
                (*parser).token_available = 0 as libc::c_int;
                let ref mut fresh96 = (*parser).tokens_parsed;
                *fresh96 = (*fresh96).wrapping_add(1);
                (*parser)
                    .stream_end_produced = ((*(*parser).tokens.head).type_0
                    as libc::c_uint
                    == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let ref mut fresh97 = (*parser).tokens.head;
                *fresh97 = (*fresh97).offset(1);
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    return 0 as libc::c_int;
                }
            } else {
                let ref mut fresh98 = (*parser).marks.top;
                *fresh98 = (*fresh98).offset(-1);
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow sequence\0" as *const u8
                        as *const libc::c_char,
                    **fresh98,
                    b"did not find expected ',' or ']'\0" as *const u8
                        as *const libc::c_char,
                    (*token).start_mark,
                );
            }
        }
        if (*token).type_0 as libc::c_uint
            == YAML_KEY_TOKEN as libc::c_int as libc::c_uint
        {
            (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE;
            memset(
                event as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
            );
            (*event).type_0 = YAML_MAPPING_START_EVENT;
            (*event).start_mark = (*token).start_mark;
            (*event).end_mark = (*token).end_mark;
            let ref mut fresh99 = (*event).data.mapping_start.anchor;
            *fresh99 = 0 as *mut yaml_char_t;
            let ref mut fresh100 = (*event).data.mapping_start.tag;
            *fresh100 = 0 as *mut yaml_char_t;
            (*event).data.mapping_start.implicit = 1 as libc::c_int;
            (*event).data.mapping_start.style = YAML_FLOW_MAPPING_STYLE;
            (*parser).token_available = 0 as libc::c_int;
            let ref mut fresh101 = (*parser).tokens_parsed;
            *fresh101 = (*fresh101).wrapping_add(1);
            (*parser)
                .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
                == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
            let ref mut fresh102 = (*parser).tokens.head;
            *fresh102 = (*fresh102).offset(1);
            return 1 as libc::c_int;
        } else {
            if (*token).type_0 as libc::c_uint
                != YAML_FLOW_SEQUENCE_END_TOKEN as libc::c_int as libc::c_uint
            {
                if if (*parser).states.top != (*parser).states.end
                    || yaml_stack_extend(
                        &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let ref mut fresh103 = (*parser).states.top;
                    let fresh104 = *fresh103;
                    *fresh103 = (*fresh103).offset(1);
                    *fresh104 = YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE;
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0
                {
                    return 0 as libc::c_int;
                }
                return yaml_parser_parse_node(
                    parser,
                    event,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
        }
    }
    let ref mut fresh105 = (*parser).states.top;
    *fresh105 = (*fresh105).offset(-1);
    (*parser).state = **fresh105;
    let ref mut fresh106 = (*parser).marks.top;
    *fresh106 = (*fresh106).offset(-1);
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_SEQUENCE_END_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).end_mark;
    (*parser).token_available = 0 as libc::c_int;
    let ref mut fresh107 = (*parser).tokens_parsed;
    *fresh107 = (*fresh107).wrapping_add(1);
    (*parser)
        .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
        == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
    let ref mut fresh108 = (*parser).tokens.head;
    *fresh108 = (*fresh108).offset(1);
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_parse_flow_sequence_entry_mapping_key(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_0 as libc::c_uint
            != YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_0 as libc::c_uint
            != YAML_FLOW_SEQUENCE_END_TOKEN as libc::c_int as libc::c_uint
    {
        if if (*parser).states.top != (*parser).states.end
            || yaml_stack_extend(
                &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
                &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
                &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh109 = (*parser).states.top;
            let fresh110 = *fresh109;
            *fresh109 = (*fresh109).offset(1);
            *fresh110 = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE;
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        return yaml_parser_parse_node(parser, event, 0 as libc::c_int, 0 as libc::c_int);
    } else {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh111 = (*parser).tokens_parsed;
        *fresh111 = (*fresh111).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh112 = (*parser).tokens.head;
        *fresh112 = (*fresh112).offset(1);
        (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE;
        return yaml_parser_process_empty_scalar(parser, event, mark);
    };
}
unsafe extern "C" fn yaml_parser_parse_flow_sequence_entry_mapping_value(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint == YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
    {
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh113 = (*parser).tokens_parsed;
        *fresh113 = (*fresh113).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh114 = (*parser).tokens.head;
        *fresh114 = (*fresh114).offset(1);
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as libc::c_int;
        }
        if (*token).type_0 as libc::c_uint
            != YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_FLOW_SEQUENCE_END_TOKEN as libc::c_int as libc::c_uint
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let ref mut fresh115 = (*parser).states.top;
                let fresh116 = *fresh115;
                *fresh115 = (*fresh115).offset(1);
                *fresh116 = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE;
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
            return yaml_parser_parse_node(
                parser,
                event,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE;
    return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
}
unsafe extern "C" fn yaml_parser_parse_flow_sequence_entry_mapping_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE;
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_MAPPING_END_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).start_mark;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_parse_flow_mapping_key(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut first: libc::c_int,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    if first != 0 {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if if (*parser).marks.top != (*parser).marks.end
            || yaml_stack_extend(
                &mut (*parser).marks.start as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.top as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.end as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh117 = (*parser).marks.top;
            let fresh118 = *fresh117;
            *fresh117 = (*fresh117).offset(1);
            *fresh118 = (*token).start_mark;
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh119 = (*parser).tokens_parsed;
        *fresh119 = (*fresh119).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh120 = (*parser).tokens.head;
        *fresh120 = (*fresh120).offset(1);
    }
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if (*token).type_0 as libc::c_uint
        != YAML_FLOW_MAPPING_END_TOKEN as libc::c_int as libc::c_uint
    {
        if first == 0 {
            if (*token).type_0 as libc::c_uint
                == YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
            {
                (*parser).token_available = 0 as libc::c_int;
                let ref mut fresh121 = (*parser).tokens_parsed;
                *fresh121 = (*fresh121).wrapping_add(1);
                (*parser)
                    .stream_end_produced = ((*(*parser).tokens.head).type_0
                    as libc::c_uint
                    == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let ref mut fresh122 = (*parser).tokens.head;
                *fresh122 = (*fresh122).offset(1);
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    return 0 as libc::c_int;
                }
            } else {
                let ref mut fresh123 = (*parser).marks.top;
                *fresh123 = (*fresh123).offset(-1);
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow mapping\0" as *const u8
                        as *const libc::c_char,
                    **fresh123,
                    b"did not find expected ',' or '}'\0" as *const u8
                        as *const libc::c_char,
                    (*token).start_mark,
                );
            }
        }
        if (*token).type_0 as libc::c_uint
            == YAML_KEY_TOKEN as libc::c_int as libc::c_uint
        {
            (*parser).token_available = 0 as libc::c_int;
            let ref mut fresh124 = (*parser).tokens_parsed;
            *fresh124 = (*fresh124).wrapping_add(1);
            (*parser)
                .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
                == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
            let ref mut fresh125 = (*parser).tokens.head;
            *fresh125 = (*fresh125).offset(1);
            token = if (*parser).token_available != 0
                || yaml_parser_fetch_more_tokens(parser) != 0
            {
                (*parser).tokens.head
            } else {
                0 as *mut yaml_token_t
            };
            if token.is_null() {
                return 0 as libc::c_int;
            }
            if (*token).type_0 as libc::c_uint
                != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
                && (*token).type_0 as libc::c_uint
                    != YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
                && (*token).type_0 as libc::c_uint
                    != YAML_FLOW_MAPPING_END_TOKEN as libc::c_int as libc::c_uint
            {
                if if (*parser).states.top != (*parser).states.end
                    || yaml_stack_extend(
                        &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let ref mut fresh126 = (*parser).states.top;
                    let fresh127 = *fresh126;
                    *fresh126 = (*fresh126).offset(1);
                    *fresh127 = YAML_PARSE_FLOW_MAPPING_VALUE_STATE;
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0
                {
                    return 0 as libc::c_int;
                }
                return yaml_parser_parse_node(
                    parser,
                    event,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            } else {
                (*parser).state = YAML_PARSE_FLOW_MAPPING_VALUE_STATE;
                return yaml_parser_process_empty_scalar(
                    parser,
                    event,
                    (*token).start_mark,
                );
            }
        } else {
            if (*token).type_0 as libc::c_uint
                != YAML_FLOW_MAPPING_END_TOKEN as libc::c_int as libc::c_uint
            {
                if if (*parser).states.top != (*parser).states.end
                    || yaml_stack_extend(
                        &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let ref mut fresh128 = (*parser).states.top;
                    let fresh129 = *fresh128;
                    *fresh128 = (*fresh128).offset(1);
                    *fresh129 = YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE;
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0
                {
                    return 0 as libc::c_int;
                }
                return yaml_parser_parse_node(
                    parser,
                    event,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
        }
    }
    let ref mut fresh130 = (*parser).states.top;
    *fresh130 = (*fresh130).offset(-1);
    (*parser).state = **fresh130;
    let ref mut fresh131 = (*parser).marks.top;
    *fresh131 = (*fresh131).offset(-1);
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_MAPPING_END_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).end_mark;
    (*parser).token_available = 0 as libc::c_int;
    let ref mut fresh132 = (*parser).tokens_parsed;
    *fresh132 = (*fresh132).wrapping_add(1);
    (*parser)
        .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
        == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
    let ref mut fresh133 = (*parser).tokens.head;
    *fresh133 = (*fresh133).offset(1);
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_parse_flow_mapping_value(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut empty: libc::c_int,
) -> libc::c_int {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if empty != 0 {
        (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    }
    if (*token).type_0 as libc::c_uint == YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
    {
        (*parser).token_available = 0 as libc::c_int;
        let ref mut fresh134 = (*parser).tokens_parsed;
        *fresh134 = (*fresh134).wrapping_add(1);
        (*parser)
            .stream_end_produced = ((*(*parser).tokens.head).type_0 as libc::c_uint
            == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint) as libc::c_int;
        let ref mut fresh135 = (*parser).tokens.head;
        *fresh135 = (*fresh135).offset(1);
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as libc::c_int;
        }
        if (*token).type_0 as libc::c_uint
            != YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_0 as libc::c_uint
                != YAML_FLOW_MAPPING_END_TOKEN as libc::c_int as libc::c_uint
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let ref mut fresh136 = (*parser).states.top;
                let fresh137 = *fresh136;
                *fresh136 = (*fresh136).offset(1);
                *fresh137 = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
            return yaml_parser_parse_node(
                parser,
                event,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
    return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
}
unsafe extern "C" fn yaml_parser_process_empty_scalar(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut mark: yaml_mark_t,
) -> libc::c_int {
    let mut value: *mut yaml_char_t = 0 as *mut yaml_char_t;
    value = yaml_malloc(1 as libc::c_int as size_t) as *mut yaml_char_t;
    if value.is_null() {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0 as libc::c_int;
    }
    *value.offset(0 as libc::c_int as isize) = '\0' as i32 as yaml_char_t;
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_SCALAR_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    let ref mut fresh138 = (*event).data.scalar.anchor;
    *fresh138 = 0 as *mut yaml_char_t;
    let ref mut fresh139 = (*event).data.scalar.tag;
    *fresh139 = 0 as *mut yaml_char_t;
    let ref mut fresh140 = (*event).data.scalar.value;
    *fresh140 = value;
    (*event).data.scalar.length = 0 as libc::c_int as size_t;
    (*event).data.scalar.plain_implicit = 1 as libc::c_int;
    (*event).data.scalar.quoted_implicit = 0 as libc::c_int;
    (*event).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_process_directives(
    mut parser: *mut yaml_parser_t,
    mut version_directive_ref: *mut *mut yaml_version_directive_t,
    mut tag_directives_start_ref: *mut *mut yaml_tag_directive_t,
    mut tag_directives_end_ref: *mut *mut yaml_tag_directive_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut default_tag_directives: [yaml_tag_directive_t; 3] = [
        {
            let mut init = yaml_tag_directive_s {
                handle: b"!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
                prefix: b"!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
            };
            init
        },
        {
            let mut init = yaml_tag_directive_s {
                handle: b"!!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
                prefix: b"tag:yaml.org,2002:\0" as *const u8 as *const libc::c_char
                    as *mut yaml_char_t,
            };
            init
        },
        {
            let mut init = yaml_tag_directive_s {
                handle: 0 as *mut yaml_char_t,
                prefix: 0 as *mut yaml_char_t,
            };
            init
        },
    ];
    let mut default_tag_directive: *mut yaml_tag_directive_t = 0
        as *mut yaml_tag_directive_t;
    let mut version_directive: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut tag_directives: Unnamed_36 = {
        let mut init = Unnamed_36 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        };
        init
    };
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    tag_directives
        .start = yaml_malloc(
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<yaml_tag_directive_t>() as libc::c_ulong),
    ) as *mut yaml_tag_directive_t;
    if !(if !(tag_directives.start).is_null() {
        tag_directives.top = tag_directives.start;
        tag_directives.end = (tag_directives.start).offset(16 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if !token.is_null() {
            loop {
                if !((*token).type_0 as libc::c_uint
                    == YAML_VERSION_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
                    || (*token).type_0 as libc::c_uint
                        == YAML_TAG_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint)
                {
                    current_block = 16924917904204750491;
                    break;
                }
                if (*token).type_0 as libc::c_uint
                    == YAML_VERSION_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
                {
                    if !version_directive.is_null() {
                        yaml_parser_set_parser_error(
                            parser,
                            b"found duplicate %YAML directive\0" as *const u8
                                as *const libc::c_char,
                            (*token).start_mark,
                        );
                        current_block = 17143798186130252483;
                        break;
                    } else if (*token).data.version_directive.major != 1 as libc::c_int
                            || (*token).data.version_directive.minor != 1 as libc::c_int
                                && (*token).data.version_directive.minor != 2 as libc::c_int
                        {
                        yaml_parser_set_parser_error(
                            parser,
                            b"found incompatible YAML document\0" as *const u8
                                as *const libc::c_char,
                            (*token).start_mark,
                        );
                        current_block = 17143798186130252483;
                        break;
                    } else {
                        version_directive = yaml_malloc(
                            ::std::mem::size_of::<yaml_version_directive_t>()
                                as libc::c_ulong,
                        ) as *mut yaml_version_directive_t;
                        if version_directive.is_null() {
                            (*parser).error = YAML_MEMORY_ERROR;
                            current_block = 17143798186130252483;
                            break;
                        } else {
                            (*version_directive)
                                .major = (*token).data.version_directive.major;
                            (*version_directive)
                                .minor = (*token).data.version_directive.minor;
                        }
                    }
                } else if (*token).type_0 as libc::c_uint
                        == YAML_TAG_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
                    {
                    let mut value: yaml_tag_directive_t = yaml_tag_directive_t {
                        handle: 0 as *mut yaml_char_t,
                        prefix: 0 as *mut yaml_char_t,
                    };
                    value.handle = (*token).data.tag_directive.handle;
                    value.prefix = (*token).data.tag_directive.prefix;
                    if yaml_parser_append_tag_directive(
                        parser,
                        value,
                        0 as libc::c_int,
                        (*token).start_mark,
                    ) == 0
                    {
                        current_block = 17143798186130252483;
                        break;
                    }
                    if if tag_directives.top != tag_directives.end
                        || yaml_stack_extend(
                            &mut tag_directives.start as *mut *mut yaml_tag_directive_t
                                as *mut *mut libc::c_void,
                            &mut tag_directives.top as *mut *mut yaml_tag_directive_t
                                as *mut *mut libc::c_void,
                            &mut tag_directives.end as *mut *mut yaml_tag_directive_t
                                as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh141 = tag_directives.top;
                        tag_directives.top = (tag_directives.top).offset(1);
                        *fresh141 = value;
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0
                    {
                        current_block = 17143798186130252483;
                        break;
                    }
                }
                (*parser).token_available = 0 as libc::c_int;
                let ref mut fresh142 = (*parser).tokens_parsed;
                *fresh142 = (*fresh142).wrapping_add(1);
                (*parser)
                    .stream_end_produced = ((*(*parser).tokens.head).type_0
                    as libc::c_uint
                    == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let ref mut fresh143 = (*parser).tokens.head;
                *fresh143 = (*fresh143).offset(1);
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    current_block = 17143798186130252483;
                    break;
                }
            }
            match current_block {
                17143798186130252483 => {}
                _ => {
                    default_tag_directive = default_tag_directives.as_mut_ptr();
                    loop {
                        if ((*default_tag_directive).handle).is_null() {
                            current_block = 18377268871191777778;
                            break;
                        }
                        if yaml_parser_append_tag_directive(
                            parser,
                            *default_tag_directive,
                            1 as libc::c_int,
                            (*token).start_mark,
                        ) == 0
                        {
                            current_block = 17143798186130252483;
                            break;
                        }
                        default_tag_directive = default_tag_directive.offset(1);
                    }
                    match current_block {
                        17143798186130252483 => {}
                        _ => {
                            if !version_directive_ref.is_null() {
                                *version_directive_ref = version_directive;
                            }
                            if !tag_directives_start_ref.is_null() {
                                if tag_directives.start == tag_directives.top {
                                    *tag_directives_end_ref = 0 as *mut yaml_tag_directive_t;
                                    *tag_directives_start_ref = *tag_directives_end_ref;
                                    yaml_free(tag_directives.start as *mut libc::c_void);
                                    tag_directives.end = 0 as *mut yaml_tag_directive_t;
                                    tag_directives.top = tag_directives.end;
                                    tag_directives.start = tag_directives.top;
                                } else {
                                    *tag_directives_start_ref = tag_directives.start;
                                    *tag_directives_end_ref = tag_directives.top;
                                }
                            } else {
                                yaml_free(tag_directives.start as *mut libc::c_void);
                                tag_directives.end = 0 as *mut yaml_tag_directive_t;
                                tag_directives.top = tag_directives.end;
                                tag_directives.start = tag_directives.top;
                            }
                            if version_directive_ref.is_null() {
                                yaml_free(version_directive as *mut libc::c_void);
                            }
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_free(version_directive as *mut libc::c_void);
    while !(tag_directives.start == tag_directives.top) {
        tag_directives.top = (tag_directives.top).offset(-1);
        let mut tag_directive: yaml_tag_directive_t = *tag_directives.top;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    yaml_free(tag_directives.start as *mut libc::c_void);
    tag_directives.end = 0 as *mut yaml_tag_directive_t;
    tag_directives.top = tag_directives.end;
    tag_directives.start = tag_directives.top;
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_append_tag_directive(
    mut parser: *mut yaml_parser_t,
    mut value: yaml_tag_directive_t,
    mut allow_duplicates: libc::c_int,
    mut mark: yaml_mark_t,
) -> libc::c_int {
    let mut tag_directive: *mut yaml_tag_directive_t = 0 as *mut yaml_tag_directive_t;
    let mut copy: yaml_tag_directive_t = {
        let mut init = yaml_tag_directive_s {
            handle: 0 as *mut yaml_char_t,
            prefix: 0 as *mut yaml_char_t,
        };
        init
    };
    tag_directive = (*parser).tag_directives.start;
    while tag_directive != (*parser).tag_directives.top {
        if strcmp(
            value.handle as *mut libc::c_char,
            (*tag_directive).handle as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            if allow_duplicates != 0 {
                return 1 as libc::c_int;
            }
            return yaml_parser_set_parser_error(
                parser,
                b"found duplicate %TAG directive\0" as *const u8 as *const libc::c_char,
                mark,
            );
        }
        tag_directive = tag_directive.offset(1);
    }
    copy.handle = yaml_strdup(value.handle);
    copy.prefix = yaml_strdup(value.prefix);
    if (copy.handle).is_null() || (copy.prefix).is_null() {
        (*parser).error = YAML_MEMORY_ERROR;
    } else if !(if (*parser).tag_directives.top != (*parser).tag_directives.end
            || yaml_stack_extend(
                &mut (*parser).tag_directives.start as *mut *mut yaml_tag_directive_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tag_directives.top as *mut *mut yaml_tag_directive_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tag_directives.end as *mut *mut yaml_tag_directive_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh144 = (*parser).tag_directives.top;
            let fresh145 = *fresh144;
            *fresh144 = (*fresh144).offset(1);
            *fresh145 = copy;
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0)
        {
        return 1 as libc::c_int
    }
    yaml_free(copy.handle as *mut libc::c_void);
    yaml_free(copy.prefix as *mut libc::c_void);
    return 0 as libc::c_int;
}
