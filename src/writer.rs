use crate::externs::__assert_fail;
use crate::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    pub data: unnamed_yaml_event_s_data,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_event_s_data {
    pub stream_start: unnamed_yaml_event_s_data_stream_start,
    pub document_start: unnamed_yaml_event_s_data_document_start,
    pub document_end: unnamed_yaml_event_s_data_document_end,
    pub alias: unnamed_yaml_event_s_data_alias,
    pub scalar: unnamed_yaml_event_s_data_scalar,
    pub sequence_start: unnamed_yaml_event_s_data_sequence_start,
    pub mapping_start: unnamed_yaml_event_s_data_mapping_start,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_event_s_data_mapping_start {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_event_s_data_sequence_start {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_event_s_data_scalar {
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
pub struct unnamed_yaml_event_s_data_alias {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_event_s_data_document_end {
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_event_s_data_document_start {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: unnamed_yaml_event_s_data_document_start_tag_directives,
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_event_s_data_document_start_tag_directives {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_event_s_data_stream_start {
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
    pub data: unnamed_yaml_node_s_data,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_node_s_data {
    pub scalar: unnamed_yaml_node_s_data_scalar,
    pub sequence: unnamed_yaml_node_s_data_sequence,
    pub mapping: unnamed_yaml_node_s_data_mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_node_s_data_mapping {
    pub pairs: unnamed_yaml_node_s_data_mapping_pairs,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_node_s_data_mapping_pairs {
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
pub struct unnamed_yaml_node_s_data_sequence {
    pub items: unnamed_yaml_node_s_data_sequence_items,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_node_s_data_sequence_items {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_node_s_data_scalar {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: unnamed_yaml_document_s_nodes,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: unnamed_yaml_document_s_tag_directives,
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_document_s_tag_directives {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_document_s_nodes {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_document_t = yaml_document_s;
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
    pub output: unnamed_yaml_emitter_s_output,
    pub buffer: unnamed_yaml_emitter_s_buffer,
    pub raw_buffer: unnamed_yaml_emitter_s_raw_buffer,
    pub encoding: yaml_encoding_t,
    pub canonical: libc::c_int,
    pub best_indent: libc::c_int,
    pub best_width: libc::c_int,
    pub unicode: libc::c_int,
    pub line_break: yaml_break_t,
    pub states: unnamed_yaml_emitter_s_states,
    pub state: yaml_emitter_state_t,
    pub events: unnamed_yaml_emitter_s_events,
    pub indents: unnamed_yaml_emitter_s_indents,
    pub tag_directives: unnamed_yaml_emitter_s_tag_directives,
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
    pub anchor_data: unnamed_yaml_emitter_s_anchor_data,
    pub tag_data: unnamed_yaml_emitter_s_tag_data,
    pub scalar_data: unnamed_yaml_emitter_s_scalar_data,
    pub opened: libc::c_int,
    pub closed: libc::c_int,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: libc::c_int,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_emitter_s_scalar_data {
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
pub struct unnamed_yaml_emitter_s_tag_data {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_emitter_s_anchor_data {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_emitter_s_tag_directives {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_emitter_s_indents {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_emitter_s_events {
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_emitter_s_states {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_emitter_s_raw_buffer {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_emitter_s_buffer {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_emitter_s_output {
    pub string: unnamed_yaml_emitter_s_output_string,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_yaml_emitter_s_output_string {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
    pub size_written: *mut size_t,
}
pub type yaml_emitter_t = yaml_emitter_s;
unsafe extern "C" fn yaml_emitter_set_writer_error(
    mut emitter: *mut yaml_emitter_t,
    mut problem: *const libc::c_char,
) -> libc::c_int {
    (*emitter).error = YAML_WRITER_ERROR;
    let ref mut fresh0 = (*emitter).problem;
    *fresh0 = problem;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_flush(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"writer.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int yaml_emitter_flush(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    if ((*emitter).write_handler).is_some() {} else {
        __assert_fail(
            b"emitter->write_handler\0" as *const u8 as *const libc::c_char,
            b"writer.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int yaml_emitter_flush(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    if (*emitter).encoding as u64 != 0 {} else {
        __assert_fail(
            b"emitter->encoding\0" as *const u8 as *const libc::c_char,
            b"writer.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int yaml_emitter_flush(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh1 = (*emitter).buffer.last;
    *fresh1 = (*emitter).buffer.pointer;
    let ref mut fresh2 = (*emitter).buffer.pointer;
    *fresh2 = (*emitter).buffer.start;
    if (*emitter).buffer.start == (*emitter).buffer.last {
        return 1 as libc::c_int;
    }
    if (*emitter).encoding as libc::c_uint
        == YAML_UTF8_ENCODING as libc::c_int as libc::c_uint
    {
        if ((*emitter).write_handler)
            .expect(
                "non-null function pointer",
            )(
            (*emitter).write_handler_data,
            (*emitter).buffer.start,
            ((*emitter).buffer.last).offset_from((*emitter).buffer.start) as libc::c_long
                as size_t,
        ) != 0
        {
            let ref mut fresh3 = (*emitter).buffer.last;
            *fresh3 = (*emitter).buffer.start;
            let ref mut fresh4 = (*emitter).buffer.pointer;
            *fresh4 = (*emitter).buffer.start;
            return 1 as libc::c_int;
        } else {
            return yaml_emitter_set_writer_error(
                emitter,
                b"write error\0" as *const u8 as *const libc::c_char,
            )
        }
    }
    low = if (*emitter).encoding as libc::c_uint
        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    high = if (*emitter).encoding as libc::c_uint
        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    while (*emitter).buffer.pointer != (*emitter).buffer.last {
        let mut octet: libc::c_uchar = 0;
        let mut width: libc::c_uint = 0;
        let mut value: libc::c_uint = 0;
        let mut k: size_t = 0;
        octet = *((*emitter).buffer.pointer).offset(0 as libc::c_int as isize);
        width = (if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
            1 as libc::c_int
        } else if octet as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
            2 as libc::c_int
        } else if octet as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
            3 as libc::c_int
        } else if octet as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        value = (if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
            octet as libc::c_int & 0x7f as libc::c_int
        } else if octet as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
            octet as libc::c_int & 0x1f as libc::c_int
        } else if octet as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
            octet as libc::c_int & 0xf as libc::c_int
        } else if octet as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
            octet as libc::c_int & 0x7 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        k = 1 as libc::c_int as size_t;
        while k < width as libc::c_ulong {
            octet = *((*emitter).buffer.pointer).offset(k as isize);
            value = (value << 6 as libc::c_int)
                .wrapping_add(
                    (octet as libc::c_int & 0x3f as libc::c_int) as libc::c_uint,
                );
            k = k.wrapping_add(1);
        }
        let ref mut fresh5 = (*emitter).buffer.pointer;
        *fresh5 = (*fresh5).offset(width as isize);
        if value < 0x10000 as libc::c_int as libc::c_uint {
            *((*emitter).raw_buffer.last)
                .offset(high as isize) = (value >> 8 as libc::c_int) as libc::c_uchar;
            *((*emitter).raw_buffer.last)
                .offset(
                    low as isize,
                ) = (value & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            let ref mut fresh6 = (*emitter).raw_buffer.last;
            *fresh6 = (*fresh6).offset(2 as libc::c_int as isize);
        } else {
            value = value.wrapping_sub(0x10000 as libc::c_int as libc::c_uint);
            *((*emitter).raw_buffer.last)
                .offset(
                    high as isize,
                ) = (0xd8 as libc::c_int as libc::c_uint)
                .wrapping_add(value >> 18 as libc::c_int) as libc::c_uchar;
            *((*emitter).raw_buffer.last)
                .offset(
                    low as isize,
                ) = (value >> 10 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_uchar;
            *((*emitter).raw_buffer.last)
                .offset(
                    (high + 2 as libc::c_int) as isize,
                ) = (0xdc as libc::c_int as libc::c_uint)
                .wrapping_add(
                    value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
                ) as libc::c_uchar;
            *((*emitter).raw_buffer.last)
                .offset(
                    (low + 2 as libc::c_int) as isize,
                ) = (value & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            let ref mut fresh7 = (*emitter).raw_buffer.last;
            *fresh7 = (*fresh7).offset(4 as libc::c_int as isize);
        }
    }
    if ((*emitter).write_handler)
        .expect(
            "non-null function pointer",
        )(
        (*emitter).write_handler_data,
        (*emitter).raw_buffer.start,
        ((*emitter).raw_buffer.last).offset_from((*emitter).raw_buffer.start)
            as libc::c_long as size_t,
    ) != 0
    {
        let ref mut fresh8 = (*emitter).buffer.last;
        *fresh8 = (*emitter).buffer.start;
        let ref mut fresh9 = (*emitter).buffer.pointer;
        *fresh9 = (*emitter).buffer.start;
        let ref mut fresh10 = (*emitter).raw_buffer.last;
        *fresh10 = (*emitter).raw_buffer.start;
        let ref mut fresh11 = (*emitter).raw_buffer.pointer;
        *fresh11 = (*emitter).raw_buffer.start;
        return 1 as libc::c_int;
    } else {
        return yaml_emitter_set_writer_error(
            emitter,
            b"write error\0" as *const u8 as *const libc::c_char,
        )
    };
}
