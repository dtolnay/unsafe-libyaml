use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> libc::c_int;
    fn yaml_emitter_dump(
        emitter: *mut yaml_emitter_t,
        document: *mut yaml_document_t,
    ) -> libc::c_int;
    fn yaml_emitter_close(emitter: *mut yaml_emitter_t) -> libc::c_int;
    fn yaml_emitter_open(emitter: *mut yaml_emitter_t) -> libc::c_int;
    fn yaml_emitter_set_unicode(emitter: *mut yaml_emitter_t, unicode: libc::c_int);
    fn yaml_emitter_set_canonical(emitter: *mut yaml_emitter_t, canonical: libc::c_int);
    fn yaml_emitter_set_output_string(
        emitter: *mut yaml_emitter_t,
        output: *mut libc::c_uchar,
        size: size_t,
        size_written: *mut size_t,
    );
    fn yaml_emitter_delete(emitter: *mut yaml_emitter_t);
    fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> libc::c_int;
    fn yaml_parser_load(
        parser: *mut yaml_parser_t,
        document: *mut yaml_document_t,
    ) -> libc::c_int;
    fn yaml_parser_set_input_file(parser: *mut yaml_parser_t, file: *mut FILE);
    fn yaml_parser_set_input_string(
        parser: *mut yaml_parser_t,
        input: *const libc::c_uchar,
        size: size_t,
    );
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> libc::c_int;
    fn yaml_document_append_mapping_pair(
        document: *mut yaml_document_t,
        mapping: libc::c_int,
        key: libc::c_int,
        value: libc::c_int,
    ) -> libc::c_int;
    fn yaml_document_append_sequence_item(
        document: *mut yaml_document_t,
        sequence: libc::c_int,
        item: libc::c_int,
    ) -> libc::c_int;
    fn yaml_document_add_mapping(
        document: *mut yaml_document_t,
        tag: *const yaml_char_t,
        style: yaml_mapping_style_t,
    ) -> libc::c_int;
    fn yaml_document_add_sequence(
        document: *mut yaml_document_t,
        tag: *const yaml_char_t,
        style: yaml_sequence_style_t,
    ) -> libc::c_int;
    fn yaml_document_add_scalar(
        document: *mut yaml_document_t,
        tag: *const yaml_char_t,
        value: *const yaml_char_t,
        length: libc::c_int,
        style: yaml_scalar_style_t,
    ) -> libc::c_int;
    fn yaml_document_get_root_node(document: *mut yaml_document_t) -> *mut yaml_node_t;
    fn yaml_document_get_node(
        document: *mut yaml_document_t,
        index: libc::c_int,
    ) -> *mut yaml_node_t;
    fn yaml_document_delete(document: *mut yaml_document_t);
    fn yaml_document_initialize(
        document: *mut yaml_document_t,
        version_directive: *mut yaml_version_directive_t,
        tag_directives_start: *mut yaml_tag_directive_t,
        tag_directives_end: *mut yaml_tag_directive_t,
        start_implicit: libc::c_int,
        end_implicit: libc::c_int,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memmove(
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
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
#[no_mangle]
pub unsafe extern "C" fn copy_document(
    mut document_to: *mut yaml_document_t,
    mut document_from: *mut yaml_document_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut node: *mut yaml_node_t = 0 as *mut yaml_node_t;
    let mut item: *mut yaml_node_item_t = 0 as *mut yaml_node_item_t;
    let mut pair: *mut yaml_node_pair_t = 0 as *mut yaml_node_pair_t;
    if yaml_document_initialize(
        document_to,
        (*document_from).version_directive,
        (*document_from).tag_directives.start,
        (*document_from).tag_directives.end,
        (*document_from).start_implicit,
        (*document_from).end_implicit,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    node = (*document_from).nodes.start;
    loop {
        if !(node < (*document_from).nodes.top) {
            current_block = 9606288038608642794;
            break;
        }
        match (*node).type_0 as libc::c_uint {
            1 => {
                if yaml_document_add_scalar(
                    document_to,
                    (*node).tag,
                    (*node).data.scalar.value,
                    (*node).data.scalar.length as libc::c_int,
                    (*node).data.scalar.style,
                ) == 0
                {
                    current_block = 13132497681902965791;
                    break;
                }
            }
            2 => {
                if yaml_document_add_sequence(
                    document_to,
                    (*node).tag,
                    (*node).data.sequence.style,
                ) == 0
                {
                    current_block = 13132497681902965791;
                    break;
                }
            }
            3 => {
                if yaml_document_add_mapping(
                    document_to,
                    (*node).tag,
                    (*node).data.mapping.style,
                ) == 0
                {
                    current_block = 13132497681902965791;
                    break;
                }
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                    44 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 56],
                        &[libc::c_char; 56],
                    >(b"int copy_document(yaml_document_t *, yaml_document_t *)\0"))
                        .as_ptr(),
                );
            }
        }
        node = node.offset(1);
    }
    match current_block {
        9606288038608642794 => {
            node = (*document_from).nodes.start;
            's_79: loop {
                if !(node < (*document_from).nodes.top) {
                    current_block = 4761528863920922185;
                    break;
                }
                match (*node).type_0 as libc::c_uint {
                    2 => {
                        item = (*node).data.sequence.items.start;
                        while item < (*node).data.sequence.items.top {
                            if yaml_document_append_sequence_item(
                                document_to,
                                (node.offset_from((*document_from).nodes.start)
                                    as libc::c_long + 1 as libc::c_int as libc::c_long)
                                    as libc::c_int,
                                *item,
                            ) == 0
                            {
                                current_block = 13132497681902965791;
                                break 's_79;
                            }
                            item = item.offset(1);
                        }
                    }
                    3 => {
                        pair = (*node).data.mapping.pairs.start;
                        while pair < (*node).data.mapping.pairs.top {
                            if yaml_document_append_mapping_pair(
                                document_to,
                                (node.offset_from((*document_from).nodes.start)
                                    as libc::c_long + 1 as libc::c_int as libc::c_long)
                                    as libc::c_int,
                                (*pair).key,
                                (*pair).value,
                            ) == 0
                            {
                                current_block = 13132497681902965791;
                                break 's_79;
                            }
                            pair = pair.offset(1);
                        }
                    }
                    _ => {}
                }
                node = node.offset(1);
            }
            match current_block {
                13132497681902965791 => {}
                _ => return 1 as libc::c_int,
            }
        }
        _ => {}
    }
    yaml_document_delete(document_to);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn compare_nodes(
    mut document1: *mut yaml_document_t,
    mut index1: libc::c_int,
    mut document2: *mut yaml_document_t,
    mut index2: libc::c_int,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut node1: *mut yaml_node_t = 0 as *mut yaml_node_t;
    let mut node2: *mut yaml_node_t = 0 as *mut yaml_node_t;
    let fresh0 = level;
    level = level + 1;
    if fresh0 > 1000 as libc::c_int {
        return 0 as libc::c_int;
    }
    node1 = yaml_document_get_node(document1, index1);
    node2 = yaml_document_get_node(document2, index2);
    if !node1.is_null() {} else {
        __assert_fail(
            b"node1\0" as *const u8 as *const libc::c_char,
            b"run-dumper.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int compare_nodes(yaml_document_t *, int, yaml_document_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if !node2.is_null() {} else {
        __assert_fail(
            b"node2\0" as *const u8 as *const libc::c_char,
            b"run-dumper.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int compare_nodes(yaml_document_t *, int, yaml_document_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if (*node1).type_0 as libc::c_uint != (*node2).type_0 as libc::c_uint {
        return 0 as libc::c_int;
    }
    if strcmp((*node1).tag as *mut libc::c_char, (*node2).tag as *mut libc::c_char)
        != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    match (*node1).type_0 as libc::c_uint {
        1 => {
            if (*node1).data.scalar.length != (*node2).data.scalar.length {
                return 0 as libc::c_int;
            }
            if strncmp(
                (*node1).data.scalar.value as *mut libc::c_char,
                (*node2).data.scalar.value as *mut libc::c_char,
                (*node1).data.scalar.length,
            ) != 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
        2 => {
            if ((*node1).data.sequence.items.top)
                .offset_from((*node1).data.sequence.items.start) as libc::c_long
                != ((*node2).data.sequence.items.top)
                    .offset_from((*node2).data.sequence.items.start) as libc::c_long
            {
                return 0 as libc::c_int;
            }
            k = 0 as libc::c_int;
            while (k as libc::c_long)
                < ((*node1).data.sequence.items.top)
                    .offset_from((*node1).data.sequence.items.start) as libc::c_long
            {
                if compare_nodes(
                    document1,
                    *((*node1).data.sequence.items.start).offset(k as isize),
                    document2,
                    *((*node2).data.sequence.items.start).offset(k as isize),
                    level,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                k += 1;
            }
        }
        3 => {
            if ((*node1).data.mapping.pairs.top)
                .offset_from((*node1).data.mapping.pairs.start) as libc::c_long
                != ((*node2).data.mapping.pairs.top)
                    .offset_from((*node2).data.mapping.pairs.start) as libc::c_long
            {
                return 0 as libc::c_int;
            }
            k = 0 as libc::c_int;
            while (k as libc::c_long)
                < ((*node1).data.mapping.pairs.top)
                    .offset_from((*node1).data.mapping.pairs.start) as libc::c_long
            {
                if compare_nodes(
                    document1,
                    (*((*node1).data.mapping.pairs.start).offset(k as isize)).key,
                    document2,
                    (*((*node2).data.mapping.pairs.start).offset(k as isize)).key,
                    level,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                if compare_nodes(
                    document1,
                    (*((*node1).data.mapping.pairs.start).offset(k as isize)).value,
                    document2,
                    (*((*node2).data.mapping.pairs.start).offset(k as isize)).value,
                    level,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                k += 1;
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"int compare_nodes(yaml_document_t *, int, yaml_document_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn compare_documents(
    mut document1: *mut yaml_document_t,
    mut document2: *mut yaml_document_t,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    if !((*document1).version_directive).is_null()
        && ((*document2).version_directive).is_null()
        || ((*document1).version_directive).is_null()
            && !((*document2).version_directive).is_null()
        || !((*document1).version_directive).is_null()
            && !((*document2).version_directive).is_null()
            && ((*(*document1).version_directive).major
                != (*(*document2).version_directive).major
                || (*(*document1).version_directive).minor
                    != (*(*document2).version_directive).minor)
    {
        return 0 as libc::c_int;
    }
    if ((*document1).tag_directives.end).offset_from((*document1).tag_directives.start)
        as libc::c_long
        != ((*document2).tag_directives.end)
            .offset_from((*document2).tag_directives.start) as libc::c_long
    {
        return 0 as libc::c_int;
    }
    k = 0 as libc::c_int;
    while (k as libc::c_long)
        < ((*document1).tag_directives.end)
            .offset_from((*document1).tag_directives.start) as libc::c_long
    {
        if strcmp(
            (*((*document1).tag_directives.start).offset(k as isize)).handle
                as *mut libc::c_char,
            (*((*document2).tag_directives.start).offset(k as isize)).handle
                as *mut libc::c_char,
        ) != 0 as libc::c_int
            || strcmp(
                (*((*document1).tag_directives.start).offset(k as isize)).prefix
                    as *mut libc::c_char,
                (*((*document2).tag_directives.start).offset(k as isize)).prefix
                    as *mut libc::c_char,
            ) != 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        k += 1;
    }
    if ((*document1).nodes.top).offset_from((*document1).nodes.start) as libc::c_long
        != ((*document2).nodes.top).offset_from((*document2).nodes.start) as libc::c_long
    {
        return 0 as libc::c_int;
    }
    if (*document1).nodes.top != (*document1).nodes.start {
        if compare_nodes(
            document1,
            1 as libc::c_int,
            document2,
            1 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_output(
    mut name: *mut libc::c_char,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut data: [libc::c_char; 65536] = [0; 65536];
    let mut data_size: size_t = 1 as libc::c_int as size_t;
    let mut total_size: size_t = 0 as libc::c_int as size_t;
    if count >= 0 as libc::c_int {
        printf(
            b"FAILED (at the document #%d)\nSOURCE:\n\0" as *const u8
                as *const libc::c_char,
            count + 1 as libc::c_int,
        );
    }
    file = fopen(name, b"rb\0" as *const u8 as *const libc::c_char);
    if !file.is_null() {} else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            b"run-dumper.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int print_output(char *, unsigned char *, size_t, int)\0"))
                .as_ptr(),
        );
    }
    while data_size > 0 as libc::c_int as libc::c_ulong {
        data_size = fread(
            data.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            65536 as libc::c_int as libc::c_ulong,
            file,
        );
        if ferror(file) == 0 {} else {
            __assert_fail(
                b"!ferror(file)\0" as *const u8 as *const libc::c_char,
                b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                178 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int print_output(char *, unsigned char *, size_t, int)\0"))
                    .as_ptr(),
            );
        }
        if data_size == 0 {
            break;
        }
        if fwrite(
            data.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            data_size,
            stdout,
        ) == data_size
        {} else {
            __assert_fail(
                b"fwrite(data, 1, data_size, stdout) == data_size\0" as *const u8
                    as *const libc::c_char,
                b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                180 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int print_output(char *, unsigned char *, size_t, int)\0"))
                    .as_ptr(),
            );
        }
        total_size = (total_size as libc::c_ulong).wrapping_add(data_size) as size_t
            as size_t;
        if feof(file) != 0 {
            break;
        }
    }
    fclose(file);
    printf(
        b"#### (length: %ld)\n\0" as *const u8 as *const libc::c_char,
        total_size as libc::c_long,
    );
    printf(
        b"OUTPUT:\n%s#### (length: %ld)\n\0" as *const u8 as *const libc::c_char,
        buffer,
        size as libc::c_long,
    );
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut number: libc::c_int = 0;
    let mut canonical: libc::c_int = 0 as libc::c_int;
    let mut unicode: libc::c_int = 0 as libc::c_int;
    number = 1 as libc::c_int;
    while number < argc {
        if strcmp(
            *argv.offset(number as isize),
            b"-c\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            canonical = 1 as libc::c_int;
        } else if strcmp(
                *argv.offset(number as isize),
                b"-u\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
            unicode = 1 as libc::c_int;
        } else if *(*argv.offset(number as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32
            {
            printf(
                b"Unknown option: '%s'\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(number as isize),
            );
            return 0 as libc::c_int;
        }
        if *(*argv.offset(number as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
        {
            if number < argc - 1 as libc::c_int {
                memmove(
                    argv.offset(number as isize) as *mut libc::c_void,
                    argv.offset(number as isize).offset(1 as libc::c_int as isize)
                        as *const libc::c_void,
                    ((argc - number - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                );
            }
            argc -= 1;
        } else {
            number += 1;
        }
    }
    if argc < 2 as libc::c_int {
        printf(
            b"Usage: %s [-c] [-u] file1.yaml ...\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    }
    number = 1 as libc::c_int;
    while number < argc {
        let mut file: *mut FILE = 0 as *mut FILE;
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
        let mut document: yaml_document_t = yaml_document_t {
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
        let mut buffer: [libc::c_uchar; 65537] = [0; 65537];
        let mut written: size_t = 0 as libc::c_int as size_t;
        let mut documents: [yaml_document_t; 16] = [yaml_document_t {
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
        }; 16];
        let mut document_number: size_t = 0 as libc::c_int as size_t;
        let mut done: libc::c_int = 0 as libc::c_int;
        let mut count: libc::c_int = 0 as libc::c_int;
        let mut error: libc::c_int = 0 as libc::c_int;
        let mut k: libc::c_int = 0;
        memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (65536 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
        memset(
            documents.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<yaml_document_t>() as libc::c_ulong),
        );
        printf(
            b"[%d] Loading, dumping, and loading again '%s': \0" as *const u8
                as *const libc::c_char,
            number,
            *argv.offset(number as isize),
        );
        fflush(stdout);
        file = fopen(
            *argv.offset(number as isize),
            b"rb\0" as *const u8 as *const libc::c_char,
        );
        if !file.is_null() {} else {
            __assert_fail(
                b"file\0" as *const u8 as *const libc::c_char,
                b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                247 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        if yaml_parser_initialize(&mut parser) != 0 {} else {
            __assert_fail(
                b"yaml_parser_initialize(&parser)\0" as *const u8 as *const libc::c_char,
                b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                249 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        yaml_parser_set_input_file(&mut parser, file);
        if yaml_emitter_initialize(&mut emitter) != 0 {} else {
            __assert_fail(
                b"yaml_emitter_initialize(&emitter)\0" as *const u8
                    as *const libc::c_char,
                b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                251 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        if canonical != 0 {
            yaml_emitter_set_canonical(&mut emitter, 1 as libc::c_int);
        }
        if unicode != 0 {
            yaml_emitter_set_unicode(&mut emitter, 1 as libc::c_int);
        }
        yaml_emitter_set_output_string(
            &mut emitter,
            buffer.as_mut_ptr(),
            65536 as libc::c_int as size_t,
            &mut written,
        );
        yaml_emitter_open(&mut emitter);
        while done == 0 {
            if yaml_parser_load(&mut parser, &mut document) == 0 {
                error = 1 as libc::c_int;
                break;
            } else {
                done = (yaml_document_get_root_node(&mut document)).is_null()
                    as libc::c_int;
                if done == 0 {
                    if document_number < 16 as libc::c_int as libc::c_ulong {} else {
                        __assert_fail(
                            b"document_number < MAX_DOCUMENTS\0" as *const u8
                                as *const libc::c_char,
                            b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                            270 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 23],
                                &[libc::c_char; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                    let fresh1 = document_number;
                    document_number = document_number.wrapping_add(1);
                    if copy_document(
                        &mut *documents.as_mut_ptr().offset(fresh1 as isize),
                        &mut document,
                    ) != 0
                    {} else {
                        __assert_fail(
                            b"copy_document(&(documents[document_number++]), &document)\0"
                                as *const u8 as *const libc::c_char,
                            b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                            271 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 23],
                                &[libc::c_char; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                    if yaml_emitter_dump(&mut emitter, &mut document) != 0
                        || yaml_emitter_flush(&mut emitter) != 0
                            && print_output(
                                *argv.offset(number as isize),
                                buffer.as_mut_ptr(),
                                written,
                                count,
                            ) != 0
                    {} else {
                        __assert_fail(
                            b"yaml_emitter_dump(&emitter, &document) || (yaml_emitter_flush(&emitter) && print_output(argv[number], buffer, written, count))\0"
                                as *const u8 as *const libc::c_char,
                            b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                            273 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 23],
                                &[libc::c_char; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                    count += 1;
                } else {
                    yaml_document_delete(&mut document);
                }
            }
        }
        yaml_parser_delete(&mut parser);
        if fclose(file) == 0 {} else {
            __assert_fail(
                b"!fclose(file)\0" as *const u8 as *const libc::c_char,
                b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        yaml_emitter_close(&mut emitter);
        yaml_emitter_delete(&mut emitter);
        if error == 0 {
            done = 0 as libc::c_int;
            count = done;
            if yaml_parser_initialize(&mut parser) != 0 {} else {
                __assert_fail(
                    b"yaml_parser_initialize(&parser)\0" as *const u8
                        as *const libc::c_char,
                    b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                    289 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
            yaml_parser_set_input_string(&mut parser, buffer.as_mut_ptr(), written);
            while done == 0 {
                if yaml_parser_load(&mut parser, &mut document) != 0
                    || print_output(
                        *argv.offset(number as isize),
                        buffer.as_mut_ptr(),
                        written,
                        count,
                    ) != 0
                {} else {
                    __assert_fail(
                        b"yaml_parser_load(&parser, &document) || print_output(argv[number], buffer, written, count)\0"
                            as *const u8 as *const libc::c_char,
                        b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                        294 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 23],
                            &[libc::c_char; 23],
                        >(b"int main(int, char **)\0"))
                            .as_ptr(),
                    );
                }
                done = (yaml_document_get_root_node(&mut document)).is_null()
                    as libc::c_int;
                if done == 0 {
                    if compare_documents(
                        documents.as_mut_ptr().offset(count as isize),
                        &mut document,
                    ) != 0
                        || print_output(
                            *argv.offset(number as isize),
                            buffer.as_mut_ptr(),
                            written,
                            count,
                        ) != 0
                    {} else {
                        __assert_fail(
                            b"compare_documents(documents+count, &document) || print_output(argv[number], buffer, written, count)\0"
                                as *const u8 as *const libc::c_char,
                            b"run-dumper.c\0" as *const u8 as *const libc::c_char,
                            297 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 23],
                                &[libc::c_char; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                    count += 1;
                }
                yaml_document_delete(&mut document);
            }
            yaml_parser_delete(&mut parser);
        }
        k = 0 as libc::c_int;
        while (k as libc::c_ulong) < document_number {
            yaml_document_delete(documents.as_mut_ptr().offset(k as isize));
            k += 1;
        }
        printf(
            b"PASSED (length: %ld)\n\0" as *const u8 as *const libc::c_char,
            written as libc::c_long,
        );
        print_output(
            *argv.offset(number as isize),
            buffer.as_mut_ptr(),
            written,
            -(1 as libc::c_int),
        );
        number += 1;
    }
    return 0 as libc::c_int;
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
