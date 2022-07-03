use crate::externs::__assert_fail;
use crate::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn yaml_malloc(size: size_t) -> *mut libc::c_void;
    fn yaml_free(ptr: *mut libc::c_void);
    fn yaml_emitter_emit(
        emitter: *mut yaml_emitter_t,
        event: *mut yaml_event_t,
    ) -> libc::c_int;
    fn yaml_document_delete(document: *mut yaml_document_t);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    pub data: Unnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed {
    pub stream_start: Unnamed_7,
    pub document_start: Unnamed_5,
    pub document_end: Unnamed_4,
    pub alias: Unnamed_3,
    pub scalar: Unnamed_2,
    pub sequence_start: Unnamed_1,
    pub mapping_start: Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_0 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_1 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_2 {
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
pub struct Unnamed_3 {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_4 {
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_5 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: Unnamed_6,
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_6 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_7 {
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
    pub data: Unnamed_8,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed_8 {
    pub scalar: Unnamed_13,
    pub sequence: Unnamed_11,
    pub mapping: Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_9 {
    pub pairs: Unnamed_10,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_10 {
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
pub struct Unnamed_11 {
    pub items: Unnamed_12,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_12 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_13 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: Unnamed_15,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: Unnamed_14,
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
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
    pub output: Unnamed_25,
    pub buffer: Unnamed_24,
    pub raw_buffer: Unnamed_23,
    pub encoding: yaml_encoding_t,
    pub canonical: libc::c_int,
    pub best_indent: libc::c_int,
    pub best_width: libc::c_int,
    pub unicode: libc::c_int,
    pub line_break: yaml_break_t,
    pub states: Unnamed_22,
    pub state: yaml_emitter_state_t,
    pub events: Unnamed_21,
    pub indents: Unnamed_20,
    pub tag_directives: Unnamed_19,
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
    pub anchor_data: Unnamed_18,
    pub tag_data: Unnamed_17,
    pub scalar_data: Unnamed_16,
    pub opened: libc::c_int,
    pub closed: libc::c_int,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: libc::c_int,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_16 {
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
pub struct Unnamed_17 {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_18 {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_19 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_20 {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_21 {
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_22 {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_23 {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_24 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed_25 {
    pub string: Unnamed_26,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_26 {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
    pub size_written: *mut size_t,
}
pub type yaml_emitter_t = yaml_emitter_s;
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_open(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: Unnamed {
            stream_start: Unnamed_7 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"dumper.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"int yaml_emitter_open(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    if (*emitter).opened == 0 {} else {
        __assert_fail(
            b"!emitter->opened\0" as *const u8 as *const libc::c_char,
            b"dumper.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"int yaml_emitter_open(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_STREAM_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.stream_start.encoding = YAML_ANY_ENCODING;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    (*emitter).opened = 1 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_close(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: Unnamed {
            stream_start: Unnamed_7 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"dumper.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int yaml_emitter_close(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    if (*emitter).opened != 0 {} else {
        __assert_fail(
            b"emitter->opened\0" as *const u8 as *const libc::c_char,
            b"dumper.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int yaml_emitter_close(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    if (*emitter).closed != 0 {
        return 1 as libc::c_int;
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_STREAM_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    (*emitter).closed = 1 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_dump(
    mut emitter: *mut yaml_emitter_t,
    mut document: *mut yaml_document_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: Unnamed {
            stream_start: Unnamed_7 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"dumper.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"dumper.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh0 = (*emitter).document;
    *fresh0 = document;
    if (*emitter).opened == 0 {
        if yaml_emitter_open(emitter) == 0 {
            current_block = 5018439318894558507;
        } else {
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            if (*document).nodes.start == (*document).nodes.top {
                if !(yaml_emitter_close(emitter) == 0) {
                    yaml_emitter_delete_document_and_anchors(emitter);
                    return 1 as libc::c_int;
                }
            } else {
                if (*emitter).opened != 0 {} else {
                    __assert_fail(
                        b"emitter->opened\0" as *const u8 as *const libc::c_char,
                        b"dumper.c\0" as *const u8 as *const libc::c_char,
                        132 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 59],
                            &[libc::c_char; 59],
                        >(
                            b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                let ref mut fresh1 = (*emitter).anchors;
                *fresh1 = yaml_malloc(
                    (::std::mem::size_of::<yaml_anchors_t>() as libc::c_ulong)
                        .wrapping_mul(
                            ((*document).nodes.top).offset_from((*document).nodes.start)
                                as libc::c_long as libc::c_ulong,
                        ),
                ) as *mut yaml_anchors_t;
                if !((*emitter).anchors).is_null() {
                    memset(
                        (*emitter).anchors as *mut libc::c_void,
                        0 as libc::c_int,
                        (::std::mem::size_of::<yaml_anchors_t>() as libc::c_ulong)
                            .wrapping_mul(
                                ((*document).nodes.top).offset_from((*document).nodes.start)
                                    as libc::c_long as libc::c_ulong,
                            ),
                    );
                    memset(
                        &mut event as *mut yaml_event_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    event.type_0 = YAML_DOCUMENT_START_EVENT;
                    event.start_mark = mark;
                    event.end_mark = mark;
                    event
                        .data
                        .document_start
                        .version_directive = (*document).version_directive;
                    event
                        .data
                        .document_start
                        .tag_directives
                        .start = (*document).tag_directives.start;
                    event
                        .data
                        .document_start
                        .tag_directives
                        .end = (*document).tag_directives.end;
                    event.data.document_start.implicit = (*document).start_implicit;
                    if !(yaml_emitter_emit(emitter, &mut event) == 0) {
                        yaml_emitter_anchor_node(emitter, 1 as libc::c_int);
                        if !(yaml_emitter_dump_node(emitter, 1 as libc::c_int) == 0) {
                            memset(
                                &mut event as *mut yaml_event_t as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            event.type_0 = YAML_DOCUMENT_END_EVENT;
                            event.start_mark = mark;
                            event.end_mark = mark;
                            event.data.document_end.implicit = (*document).end_implicit;
                            if !(yaml_emitter_emit(emitter, &mut event) == 0) {
                                yaml_emitter_delete_document_and_anchors(emitter);
                                return 1 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    yaml_emitter_delete_document_and_anchors(emitter);
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_delete_document_and_anchors(
    mut emitter: *mut yaml_emitter_t,
) {
    let mut index: libc::c_int = 0;
    if ((*emitter).anchors).is_null() {
        yaml_document_delete((*emitter).document);
        let ref mut fresh2 = (*emitter).document;
        *fresh2 = 0 as *mut yaml_document_t;
        return;
    }
    index = 0 as libc::c_int;
    while ((*(*emitter).document).nodes.start).offset(index as isize)
        < (*(*emitter).document).nodes.top
    {
        let mut node: yaml_node_t = *((*(*emitter).document).nodes.start)
            .offset(index as isize);
        if (*((*emitter).anchors).offset(index as isize)).serialized == 0 {
            yaml_free(node.tag as *mut libc::c_void);
            if node.type_0 as libc::c_uint
                == YAML_SCALAR_NODE as libc::c_int as libc::c_uint
            {
                yaml_free(node.data.scalar.value as *mut libc::c_void);
            }
        }
        if node.type_0 as libc::c_uint
            == YAML_SEQUENCE_NODE as libc::c_int as libc::c_uint
        {
            yaml_free(node.data.sequence.items.start as *mut libc::c_void);
            node.data.sequence.items.end = 0 as *mut yaml_node_item_t;
            node.data.sequence.items.top = node.data.sequence.items.end;
            node.data.sequence.items.start = node.data.sequence.items.top;
        }
        if node.type_0 as libc::c_uint
            == YAML_MAPPING_NODE as libc::c_int as libc::c_uint
        {
            yaml_free(node.data.mapping.pairs.start as *mut libc::c_void);
            node.data.mapping.pairs.end = 0 as *mut yaml_node_pair_t;
            node.data.mapping.pairs.top = node.data.mapping.pairs.end;
            node.data.mapping.pairs.start = node.data.mapping.pairs.top;
        }
        index += 1;
    }
    yaml_free((*(*emitter).document).nodes.start as *mut libc::c_void);
    let ref mut fresh3 = (*(*emitter).document).nodes.end;
    *fresh3 = 0 as *mut yaml_node_t;
    let ref mut fresh4 = (*(*emitter).document).nodes.top;
    *fresh4 = *fresh3;
    let ref mut fresh5 = (*(*emitter).document).nodes.start;
    *fresh5 = *fresh4;
    yaml_free((*emitter).anchors as *mut libc::c_void);
    let ref mut fresh6 = (*emitter).anchors;
    *fresh6 = 0 as *mut yaml_anchors_t;
    (*emitter).last_anchor_id = 0 as libc::c_int;
    let ref mut fresh7 = (*emitter).document;
    *fresh7 = 0 as *mut yaml_document_t;
}
unsafe extern "C" fn yaml_emitter_anchor_node(
    mut emitter: *mut yaml_emitter_t,
    mut index: libc::c_int,
) {
    let mut node: *mut yaml_node_t = ((*(*emitter).document).nodes.start)
        .offset(index as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut item: *mut yaml_node_item_t = 0 as *mut yaml_node_item_t;
    let mut pair: *mut yaml_node_pair_t = 0 as *mut yaml_node_pair_t;
    let ref mut fresh8 = (*((*emitter).anchors)
        .offset((index - 1 as libc::c_int) as isize))
        .references;
    *fresh8 += 1;
    if (*((*emitter).anchors).offset((index - 1 as libc::c_int) as isize)).references
        == 1 as libc::c_int
    {
        match (*node).type_0 as libc::c_uint {
            2 => {
                item = (*node).data.sequence.items.start;
                while item < (*node).data.sequence.items.top {
                    yaml_emitter_anchor_node(emitter, *item);
                    item = item.offset(1);
                }
            }
            3 => {
                pair = (*node).data.mapping.pairs.start;
                while pair < (*node).data.mapping.pairs.top {
                    yaml_emitter_anchor_node(emitter, (*pair).key);
                    yaml_emitter_anchor_node(emitter, (*pair).value);
                    pair = pair.offset(1);
                }
            }
            _ => {}
        }
    } else if (*((*emitter).anchors).offset((index - 1 as libc::c_int) as isize))
            .references == 2 as libc::c_int
        {
        let ref mut fresh9 = (*emitter).last_anchor_id;
        *fresh9 += 1;
        (*((*emitter).anchors).offset((index - 1 as libc::c_int) as isize))
            .anchor = *fresh9;
    }
}
unsafe extern "C" fn yaml_emitter_generate_anchor(
    mut emitter: *mut yaml_emitter_t,
    mut anchor_id: libc::c_int,
) -> *mut yaml_char_t {
    let mut anchor: *mut yaml_char_t = yaml_malloc(16 as libc::c_int as size_t)
        as *mut yaml_char_t;
    if anchor.is_null() {
        return 0 as *mut yaml_char_t;
    }
    sprintf(
        anchor as *mut libc::c_char,
        b"id%03d\0" as *const u8 as *const libc::c_char,
        anchor_id,
    );
    return anchor;
}
unsafe extern "C" fn yaml_emitter_dump_node(
    mut emitter: *mut yaml_emitter_t,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut node: *mut yaml_node_t = ((*(*emitter).document).nodes.start)
        .offset(index as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut anchor_id: libc::c_int = (*((*emitter).anchors)
        .offset((index - 1 as libc::c_int) as isize))
        .anchor;
    let mut anchor: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if anchor_id != 0 {
        anchor = yaml_emitter_generate_anchor(emitter, anchor_id);
        if anchor.is_null() {
            return 0 as libc::c_int;
        }
    }
    if (*((*emitter).anchors).offset((index - 1 as libc::c_int) as isize)).serialized
        != 0
    {
        return yaml_emitter_dump_alias(emitter, anchor);
    }
    (*((*emitter).anchors).offset((index - 1 as libc::c_int) as isize))
        .serialized = 1 as libc::c_int;
    match (*node).type_0 as libc::c_uint {
        1 => return yaml_emitter_dump_scalar(emitter, node, anchor),
        2 => return yaml_emitter_dump_sequence(emitter, node, anchor),
        3 => return yaml_emitter_dump_mapping(emitter, node, anchor),
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"dumper.c\0" as *const u8 as *const libc::c_char,
                289 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"int yaml_emitter_dump_node(yaml_emitter_t *, int)\0"))
                    .as_ptr(),
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_dump_alias(
    mut emitter: *mut yaml_emitter_t,
    mut anchor: *mut yaml_char_t,
) -> libc::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: Unnamed {
            stream_start: Unnamed_7 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_ALIAS_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.alias.anchor = anchor;
    return yaml_emitter_emit(emitter, &mut event);
}
unsafe extern "C" fn yaml_emitter_dump_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
) -> libc::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: Unnamed {
            stream_start: Unnamed_7 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut plain_implicit: libc::c_int = (strcmp(
        (*node).tag as *mut libc::c_char,
        b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    let mut quoted_implicit: libc::c_int = (strcmp(
        (*node).tag as *mut libc::c_char,
        b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_SCALAR_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.scalar.anchor = anchor;
    event.data.scalar.tag = (*node).tag;
    event.data.scalar.value = (*node).data.scalar.value;
    event.data.scalar.length = (*node).data.scalar.length;
    event.data.scalar.plain_implicit = plain_implicit;
    event.data.scalar.quoted_implicit = quoted_implicit;
    event.data.scalar.style = (*node).data.scalar.style;
    return yaml_emitter_emit(emitter, &mut event);
}
unsafe extern "C" fn yaml_emitter_dump_sequence(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
) -> libc::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: Unnamed {
            stream_start: Unnamed_7 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut implicit: libc::c_int = (strcmp(
        (*node).tag as *mut libc::c_char,
        b"tag:yaml.org,2002:seq\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    let mut item: *mut yaml_node_item_t = 0 as *mut yaml_node_item_t;
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_SEQUENCE_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.sequence_start.anchor = anchor;
    event.data.sequence_start.tag = (*node).tag;
    event.data.sequence_start.implicit = implicit;
    event.data.sequence_start.style = (*node).data.sequence.style;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    item = (*node).data.sequence.items.start;
    while item < (*node).data.sequence.items.top {
        if yaml_emitter_dump_node(emitter, *item) == 0 {
            return 0 as libc::c_int;
        }
        item = item.offset(1);
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_SEQUENCE_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_dump_mapping(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
) -> libc::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: Unnamed {
            stream_start: Unnamed_7 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut implicit: libc::c_int = (strcmp(
        (*node).tag as *mut libc::c_char,
        b"tag:yaml.org,2002:map\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    let mut pair: *mut yaml_node_pair_t = 0 as *mut yaml_node_pair_t;
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_MAPPING_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.mapping_start.anchor = anchor;
    event.data.mapping_start.tag = (*node).tag;
    event.data.mapping_start.implicit = implicit;
    event.data.mapping_start.style = (*node).data.mapping.style;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    pair = (*node).data.mapping.pairs.start;
    while pair < (*node).data.mapping.pairs.top {
        if yaml_emitter_dump_node(emitter, (*pair).key) == 0 {
            return 0 as libc::c_int;
        }
        if yaml_emitter_dump_node(emitter, (*pair).value) == 0 {
            return 0 as libc::c_int;
        }
        pair = pair.offset(1);
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_MAPPING_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
