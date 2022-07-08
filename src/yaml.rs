use crate::libc;
pub use core::primitive::{i64 as ptrdiff_t, u64 as size_t, u8 as yaml_char_t};
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_version_directive_t {
    pub major: libc::c_int,
    pub minor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_tag_directive_t {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
pub type yaml_encoding_t = libc::c_uint;
pub const YAML_UTF16BE_ENCODING: yaml_encoding_t = 3;
pub const YAML_UTF16LE_ENCODING: yaml_encoding_t = 2;
pub const YAML_UTF8_ENCODING: yaml_encoding_t = 1;
pub const YAML_ANY_ENCODING: yaml_encoding_t = 0;
pub type yaml_break_t = libc::c_uint;
pub const YAML_CRLN_BREAK: yaml_break_t = 3;
pub const YAML_LN_BREAK: yaml_break_t = 2;
pub const YAML_CR_BREAK: yaml_break_t = 1;
pub const YAML_ANY_BREAK: yaml_break_t = 0;
pub type yaml_error_type_t = libc::c_uint;
pub const YAML_EMITTER_ERROR: yaml_error_type_t = 7;
pub const YAML_WRITER_ERROR: yaml_error_type_t = 6;
pub const YAML_COMPOSER_ERROR: yaml_error_type_t = 5;
pub const YAML_PARSER_ERROR: yaml_error_type_t = 4;
pub const YAML_SCANNER_ERROR: yaml_error_type_t = 3;
pub const YAML_READER_ERROR: yaml_error_type_t = 2;
pub const YAML_MEMORY_ERROR: yaml_error_type_t = 1;
pub const YAML_NO_ERROR: yaml_error_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_mark_t {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}
pub type yaml_scalar_style_t = libc::c_uint;
pub const YAML_FOLDED_SCALAR_STYLE: yaml_scalar_style_t = 5;
pub const YAML_LITERAL_SCALAR_STYLE: yaml_scalar_style_t = 4;
pub const YAML_DOUBLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_t = 3;
pub const YAML_SINGLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_t = 2;
pub const YAML_PLAIN_SCALAR_STYLE: yaml_scalar_style_t = 1;
pub const YAML_ANY_SCALAR_STYLE: yaml_scalar_style_t = 0;
pub type yaml_sequence_style_t = libc::c_uint;
pub const YAML_FLOW_SEQUENCE_STYLE: yaml_sequence_style_t = 2;
pub const YAML_BLOCK_SEQUENCE_STYLE: yaml_sequence_style_t = 1;
pub const YAML_ANY_SEQUENCE_STYLE: yaml_sequence_style_t = 0;
pub type yaml_mapping_style_t = libc::c_uint;
pub const YAML_FLOW_MAPPING_STYLE: yaml_mapping_style_t = 2;
pub const YAML_BLOCK_MAPPING_STYLE: yaml_mapping_style_t = 1;
pub const YAML_ANY_MAPPING_STYLE: yaml_mapping_style_t = 0;
pub type yaml_token_type_t = libc::c_uint;
pub const YAML_SCALAR_TOKEN: yaml_token_type_t = 21;
pub const YAML_TAG_TOKEN: yaml_token_type_t = 20;
pub const YAML_ANCHOR_TOKEN: yaml_token_type_t = 19;
pub const YAML_ALIAS_TOKEN: yaml_token_type_t = 18;
pub const YAML_VALUE_TOKEN: yaml_token_type_t = 17;
pub const YAML_KEY_TOKEN: yaml_token_type_t = 16;
pub const YAML_FLOW_ENTRY_TOKEN: yaml_token_type_t = 15;
pub const YAML_BLOCK_ENTRY_TOKEN: yaml_token_type_t = 14;
pub const YAML_FLOW_MAPPING_END_TOKEN: yaml_token_type_t = 13;
pub const YAML_FLOW_MAPPING_START_TOKEN: yaml_token_type_t = 12;
pub const YAML_FLOW_SEQUENCE_END_TOKEN: yaml_token_type_t = 11;
pub const YAML_FLOW_SEQUENCE_START_TOKEN: yaml_token_type_t = 10;
pub const YAML_BLOCK_END_TOKEN: yaml_token_type_t = 9;
pub const YAML_BLOCK_MAPPING_START_TOKEN: yaml_token_type_t = 8;
pub const YAML_BLOCK_SEQUENCE_START_TOKEN: yaml_token_type_t = 7;
pub const YAML_DOCUMENT_END_TOKEN: yaml_token_type_t = 6;
pub const YAML_DOCUMENT_START_TOKEN: yaml_token_type_t = 5;
pub const YAML_TAG_DIRECTIVE_TOKEN: yaml_token_type_t = 4;
pub const YAML_VERSION_DIRECTIVE_TOKEN: yaml_token_type_t = 3;
pub const YAML_STREAM_END_TOKEN: yaml_token_type_t = 2;
pub const YAML_STREAM_START_TOKEN: yaml_token_type_t = 1;
pub const YAML_NO_TOKEN: yaml_token_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_token_t {
    pub type_: yaml_token_type_t,
    pub data: unnamed_yaml_token_t_data,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_token_t_data {
    pub stream_start: unnamed_yaml_token_t_data_stream_start,
    pub alias: unnamed_yaml_token_t_data_alias,
    pub anchor: unnamed_yaml_token_t_data_anchor,
    pub tag: unnamed_yaml_token_t_data_tag,
    pub scalar: unnamed_yaml_token_t_data_scalar,
    pub version_directive: unnamed_yaml_token_t_data_version_directive,
    pub tag_directive: unnamed_yaml_token_t_data_tag_directive,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_tag_directive {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_version_directive {
    pub major: libc::c_int,
    pub minor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_scalar {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_tag {
    pub handle: *mut yaml_char_t,
    pub suffix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_anchor {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_alias {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_stream_start {
    pub encoding: yaml_encoding_t,
}
pub type yaml_event_type_t = libc::c_uint;
pub const YAML_MAPPING_END_EVENT: yaml_event_type_t = 10;
pub const YAML_MAPPING_START_EVENT: yaml_event_type_t = 9;
pub const YAML_SEQUENCE_END_EVENT: yaml_event_type_t = 8;
pub const YAML_SEQUENCE_START_EVENT: yaml_event_type_t = 7;
pub const YAML_SCALAR_EVENT: yaml_event_type_t = 6;
pub const YAML_ALIAS_EVENT: yaml_event_type_t = 5;
pub const YAML_DOCUMENT_END_EVENT: yaml_event_type_t = 4;
pub const YAML_DOCUMENT_START_EVENT: yaml_event_type_t = 3;
pub const YAML_STREAM_END_EVENT: yaml_event_type_t = 2;
pub const YAML_STREAM_START_EVENT: yaml_event_type_t = 1;
pub const YAML_NO_EVENT: yaml_event_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_event_t {
    pub type_: yaml_event_type_t,
    pub data: unnamed_yaml_event_t_data,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_event_t_data {
    pub stream_start: unnamed_yaml_event_t_data_stream_start,
    pub document_start: unnamed_yaml_event_t_data_document_start,
    pub document_end: unnamed_yaml_event_t_data_document_end,
    pub alias: unnamed_yaml_event_t_data_alias,
    pub scalar: unnamed_yaml_event_t_data_scalar,
    pub sequence_start: unnamed_yaml_event_t_data_sequence_start,
    pub mapping_start: unnamed_yaml_event_t_data_mapping_start,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_mapping_start {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_sequence_start {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_scalar {
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
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_alias {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_document_end {
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_document_start {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: unnamed_yaml_event_t_data_document_start_tag_directives,
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_document_start_tag_directives {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_stream_start {
    pub encoding: yaml_encoding_t,
}
pub type yaml_node_type_t = libc::c_uint;
pub const YAML_MAPPING_NODE: yaml_node_type_t = 3;
pub const YAML_SEQUENCE_NODE: yaml_node_type_t = 2;
pub const YAML_SCALAR_NODE: yaml_node_type_t = 1;
pub const YAML_NO_NODE: yaml_node_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_node_t {
    pub type_: yaml_node_type_t,
    pub tag: *mut yaml_char_t,
    pub data: unnamed_yaml_node_t_data,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_node_t_data {
    pub scalar: unnamed_yaml_node_t_data_scalar,
    pub sequence: unnamed_yaml_node_t_data_sequence,
    pub mapping: unnamed_yaml_node_t_data_mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_node_t_data_mapping {
    pub pairs: unnamed_yaml_node_t_data_mapping_pairs,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_node_t_data_mapping_pairs {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_node_pair_t {
    pub key: libc::c_int,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_node_t_data_sequence {
    pub items: unnamed_yaml_node_t_data_sequence_items,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_node_t_data_sequence_items {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_node_t_data_scalar {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_document_t {
    pub nodes: unnamed_yaml_document_t_nodes,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: unnamed_yaml_document_t_tag_directives,
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_document_t_tag_directives {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_document_t_nodes {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_read_handler_t =
    unsafe fn(*mut libc::c_void, *mut libc::c_uchar, size_t, *mut size_t) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_simple_key_t {
    pub possible: libc::c_int,
    pub required: libc::c_int,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}
pub type yaml_parser_state_t = libc::c_uint;
pub const YAML_PARSE_END_STATE: yaml_parser_state_t = 23;
pub const YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE: yaml_parser_state_t = 22;
pub const YAML_PARSE_FLOW_MAPPING_VALUE_STATE: yaml_parser_state_t = 21;
pub const YAML_PARSE_FLOW_MAPPING_KEY_STATE: yaml_parser_state_t = 20;
pub const YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE: yaml_parser_state_t = 19;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE: yaml_parser_state_t = 18;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE: yaml_parser_state_t = 17;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE: yaml_parser_state_t = 16;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE: yaml_parser_state_t = 15;
pub const YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_t = 14;
pub const YAML_PARSE_BLOCK_MAPPING_VALUE_STATE: yaml_parser_state_t = 13;
pub const YAML_PARSE_BLOCK_MAPPING_KEY_STATE: yaml_parser_state_t = 12;
pub const YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_parser_state_t = 11;
pub const YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE: yaml_parser_state_t = 10;
pub const YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE: yaml_parser_state_t = 9;
pub const YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_t = 8;
pub const YAML_PARSE_FLOW_NODE_STATE: yaml_parser_state_t = 7;
pub const YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE: yaml_parser_state_t = 6;
pub const YAML_PARSE_BLOCK_NODE_STATE: yaml_parser_state_t = 5;
pub const YAML_PARSE_DOCUMENT_END_STATE: yaml_parser_state_t = 4;
pub const YAML_PARSE_DOCUMENT_CONTENT_STATE: yaml_parser_state_t = 3;
pub const YAML_PARSE_DOCUMENT_START_STATE: yaml_parser_state_t = 2;
pub const YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE: yaml_parser_state_t = 1;
pub const YAML_PARSE_STREAM_START_STATE: yaml_parser_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_alias_data_t {
    pub anchor: *mut yaml_char_t,
    pub index: libc::c_int,
    pub mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_parser_t {
    pub error: yaml_error_type_t,
    pub problem: *const libc::c_char,
    pub problem_offset: size_t,
    pub problem_value: libc::c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *const libc::c_char,
    pub context_mark: yaml_mark_t,
    pub read_handler: Option<yaml_read_handler_t>,
    pub read_handler_data: *mut libc::c_void,
    pub input: unnamed_yaml_parser_t_input,
    pub eof: libc::c_int,
    pub buffer: unnamed_yaml_parser_t_buffer,
    pub unread: size_t,
    pub raw_buffer: unnamed_yaml_parser_t_raw_buffer,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: libc::c_int,
    pub stream_end_produced: libc::c_int,
    pub flow_level: libc::c_int,
    pub tokens: unnamed_yaml_parser_t_tokens,
    pub tokens_parsed: size_t,
    pub token_available: libc::c_int,
    pub indents: unnamed_yaml_parser_t_indents,
    pub indent: libc::c_int,
    pub simple_key_allowed: libc::c_int,
    pub simple_keys: unnamed_yaml_parser_t_simple_keys,
    pub states: unnamed_yaml_parser_t_states,
    pub state: yaml_parser_state_t,
    pub marks: unnamed_yaml_parser_t_marks,
    pub tag_directives: unnamed_yaml_parser_t_tag_directives,
    pub aliases: unnamed_yaml_parser_t_aliases,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_aliases {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_tag_directives {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_marks {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_states {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_simple_keys {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_indents {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_tokens {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_raw_buffer {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_buffer {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_parser_t_input {
    pub string: unnamed_yaml_parser_t_input_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_parser_t_input_string {
    pub start: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
    pub current: *const libc::c_uchar,
}
pub type yaml_write_handler_t =
    unsafe fn(*mut libc::c_void, *mut libc::c_uchar, size_t) -> libc::c_int;
pub type yaml_emitter_state_t = libc::c_uint;
pub const YAML_EMIT_END_STATE: yaml_emitter_state_t = 17;
pub const YAML_EMIT_BLOCK_MAPPING_VALUE_STATE: yaml_emitter_state_t = 16;
pub const YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_t = 15;
pub const YAML_EMIT_BLOCK_MAPPING_KEY_STATE: yaml_emitter_state_t = 14;
pub const YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_t = 13;
pub const YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE: yaml_emitter_state_t = 12;
pub const YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_t = 11;
pub const YAML_EMIT_FLOW_MAPPING_VALUE_STATE: yaml_emitter_state_t = 10;
pub const YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_t = 9;
pub const YAML_EMIT_FLOW_MAPPING_KEY_STATE: yaml_emitter_state_t = 8;
pub const YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_t = 7;
pub const YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE: yaml_emitter_state_t = 6;
pub const YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_t = 5;
pub const YAML_EMIT_DOCUMENT_END_STATE: yaml_emitter_state_t = 4;
pub const YAML_EMIT_DOCUMENT_CONTENT_STATE: yaml_emitter_state_t = 3;
pub const YAML_EMIT_DOCUMENT_START_STATE: yaml_emitter_state_t = 2;
pub const YAML_EMIT_FIRST_DOCUMENT_START_STATE: yaml_emitter_state_t = 1;
pub const YAML_EMIT_STREAM_START_STATE: yaml_emitter_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_anchors_t {
    pub references: libc::c_int,
    pub anchor: libc::c_int,
    pub serialized: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_emitter_t {
    pub error: yaml_error_type_t,
    pub problem: *const libc::c_char,
    pub write_handler: Option<yaml_write_handler_t>,
    pub write_handler_data: *mut libc::c_void,
    pub output: unnamed_yaml_emitter_t_output,
    pub buffer: unnamed_yaml_emitter_t_buffer,
    pub raw_buffer: unnamed_yaml_emitter_t_raw_buffer,
    pub encoding: yaml_encoding_t,
    pub canonical: libc::c_int,
    pub best_indent: libc::c_int,
    pub best_width: libc::c_int,
    pub unicode: libc::c_int,
    pub line_break: yaml_break_t,
    pub states: unnamed_yaml_emitter_t_states,
    pub state: yaml_emitter_state_t,
    pub events: unnamed_yaml_emitter_t_events,
    pub indents: unnamed_yaml_emitter_t_indents,
    pub tag_directives: unnamed_yaml_emitter_t_tag_directives,
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
    pub anchor_data: unnamed_yaml_emitter_t_anchor_data,
    pub tag_data: unnamed_yaml_emitter_t_tag_data,
    pub scalar_data: unnamed_yaml_emitter_t_scalar_data,
    pub opened: libc::c_int,
    pub closed: libc::c_int,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: libc::c_int,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_scalar_data {
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
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_tag_data {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_anchor_data {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_tag_directives {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_indents {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_events {
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_states {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_raw_buffer {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_buffer {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_emitter_t_output {
    pub string: unnamed_yaml_emitter_t_output_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_emitter_t_output_string {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
    pub size_written: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_string_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
}
