pub use self::{
    yaml_break_t::*, yaml_emitter_state_t::*, yaml_encoding_t::*, yaml_error_type_t::*,
    yaml_event_type_t::*, yaml_mapping_style_t::*, yaml_node_type_t::*, yaml_parser_state_t::*,
    yaml_scalar_style_t::*, yaml_sequence_style_t::*, yaml_token_type_t::*,
};
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
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_encoding_t {
    YAML_ANY_ENCODING = 0,
    YAML_UTF8_ENCODING = 1,
    YAML_UTF16LE_ENCODING = 2,
    YAML_UTF16BE_ENCODING = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_break_t {
    YAML_ANY_BREAK = 0,
    YAML_CR_BREAK = 1,
    YAML_LN_BREAK = 2,
    YAML_CRLN_BREAK = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_error_type_t {
    YAML_NO_ERROR = 0,
    YAML_MEMORY_ERROR = 1,
    YAML_READER_ERROR = 2,
    YAML_SCANNER_ERROR = 3,
    YAML_PARSER_ERROR = 4,
    YAML_COMPOSER_ERROR = 5,
    YAML_WRITER_ERROR = 6,
    YAML_EMITTER_ERROR = 7,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_mark_t {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_scalar_style_t {
    YAML_ANY_SCALAR_STYLE = 0,
    YAML_PLAIN_SCALAR_STYLE = 1,
    YAML_SINGLE_QUOTED_SCALAR_STYLE = 2,
    YAML_DOUBLE_QUOTED_SCALAR_STYLE = 3,
    YAML_LITERAL_SCALAR_STYLE = 4,
    YAML_FOLDED_SCALAR_STYLE = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_sequence_style_t {
    YAML_ANY_SEQUENCE_STYLE = 0,
    YAML_BLOCK_SEQUENCE_STYLE = 1,
    YAML_FLOW_SEQUENCE_STYLE = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_mapping_style_t {
    YAML_ANY_MAPPING_STYLE = 0,
    YAML_BLOCK_MAPPING_STYLE = 1,
    YAML_FLOW_MAPPING_STYLE = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_token_type_t {
    YAML_NO_TOKEN = 0,
    YAML_STREAM_START_TOKEN = 1,
    YAML_STREAM_END_TOKEN = 2,
    YAML_VERSION_DIRECTIVE_TOKEN = 3,
    YAML_TAG_DIRECTIVE_TOKEN = 4,
    YAML_DOCUMENT_START_TOKEN = 5,
    YAML_DOCUMENT_END_TOKEN = 6,
    YAML_BLOCK_SEQUENCE_START_TOKEN = 7,
    YAML_BLOCK_MAPPING_START_TOKEN = 8,
    YAML_BLOCK_END_TOKEN = 9,
    YAML_FLOW_SEQUENCE_START_TOKEN = 10,
    YAML_FLOW_SEQUENCE_END_TOKEN = 11,
    YAML_FLOW_MAPPING_START_TOKEN = 12,
    YAML_FLOW_MAPPING_END_TOKEN = 13,
    YAML_BLOCK_ENTRY_TOKEN = 14,
    YAML_FLOW_ENTRY_TOKEN = 15,
    YAML_KEY_TOKEN = 16,
    YAML_VALUE_TOKEN = 17,
    YAML_ALIAS_TOKEN = 18,
    YAML_ANCHOR_TOKEN = 19,
    YAML_TAG_TOKEN = 20,
    YAML_SCALAR_TOKEN = 21,
}
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
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_event_type_t {
    YAML_NO_EVENT = 0,
    YAML_STREAM_START_EVENT = 1,
    YAML_STREAM_END_EVENT = 2,
    YAML_DOCUMENT_START_EVENT = 3,
    YAML_DOCUMENT_END_EVENT = 4,
    YAML_ALIAS_EVENT = 5,
    YAML_SCALAR_EVENT = 6,
    YAML_SEQUENCE_START_EVENT = 7,
    YAML_SEQUENCE_END_EVENT = 8,
    YAML_MAPPING_START_EVENT = 9,
    YAML_MAPPING_END_EVENT = 10,
}
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
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_node_type_t {
    YAML_NO_NODE = 0,
    YAML_SCALAR_NODE = 1,
    YAML_SEQUENCE_NODE = 2,
    YAML_MAPPING_NODE = 3,
}
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
pub type yaml_read_handler_t = unsafe fn(
    data: *mut libc::c_void,
    buffer: *mut libc::c_uchar,
    size: size_t,
    size_read: *mut size_t,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_simple_key_t {
    pub possible: libc::c_int,
    pub required: libc::c_int,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_parser_state_t {
    YAML_PARSE_STREAM_START_STATE = 0,
    YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE = 1,
    YAML_PARSE_DOCUMENT_START_STATE = 2,
    YAML_PARSE_DOCUMENT_CONTENT_STATE = 3,
    YAML_PARSE_DOCUMENT_END_STATE = 4,
    YAML_PARSE_BLOCK_NODE_STATE = 5,
    YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE = 6,
    YAML_PARSE_FLOW_NODE_STATE = 7,
    YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE = 8,
    YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE = 9,
    YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE = 10,
    YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE = 11,
    YAML_PARSE_BLOCK_MAPPING_KEY_STATE = 12,
    YAML_PARSE_BLOCK_MAPPING_VALUE_STATE = 13,
    YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE = 14,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE = 15,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE = 16,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE = 17,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE = 18,
    YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE = 19,
    YAML_PARSE_FLOW_MAPPING_KEY_STATE = 20,
    YAML_PARSE_FLOW_MAPPING_VALUE_STATE = 21,
    YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE = 22,
    YAML_PARSE_END_STATE = 23,
}
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
    unsafe fn(data: *mut libc::c_void, buffer: *mut libc::c_uchar, size: size_t) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_emitter_state_t {
    YAML_EMIT_STREAM_START_STATE = 0,
    YAML_EMIT_FIRST_DOCUMENT_START_STATE = 1,
    YAML_EMIT_DOCUMENT_START_STATE = 2,
    YAML_EMIT_DOCUMENT_CONTENT_STATE = 3,
    YAML_EMIT_DOCUMENT_END_STATE = 4,
    YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE = 5,
    YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE = 6,
    YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE = 7,
    YAML_EMIT_FLOW_MAPPING_KEY_STATE = 8,
    YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE = 9,
    YAML_EMIT_FLOW_MAPPING_VALUE_STATE = 10,
    YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE = 11,
    YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE = 12,
    YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE = 13,
    YAML_EMIT_BLOCK_MAPPING_KEY_STATE = 14,
    YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE = 15,
    YAML_EMIT_BLOCK_MAPPING_VALUE_STATE = 16,
    YAML_EMIT_END_STATE = 17,
}
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
pub(crate) struct yaml_string_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
}
