use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn yaml_queue_extend(
        start: *mut *mut libc::c_void,
        head: *mut *mut libc::c_void,
        tail: *mut *mut libc::c_void,
        end: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn yaml_strdup(_: *const yaml_char_t) -> *mut yaml_char_t;
    fn yaml_free(ptr: *mut libc::c_void);
    fn yaml_stack_extend(
        start: *mut *mut libc::c_void,
        top: *mut *mut libc::c_void,
        end: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn yaml_event_delete(event: *mut yaml_event_t);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> libc::c_int;
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
    pub data: C2RustUnnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub stream_start: C2RustUnnamed_7,
    pub document_start: C2RustUnnamed_5,
    pub document_end: C2RustUnnamed_4,
    pub alias: C2RustUnnamed_3,
    pub scalar: C2RustUnnamed_2,
    pub sequence_start: C2RustUnnamed_1,
    pub mapping_start: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_6,
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub data: C2RustUnnamed_8,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub scalar: C2RustUnnamed_13,
    pub sequence: C2RustUnnamed_11,
    pub mapping: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub pairs: C2RustUnnamed_10,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
    pub items: C2RustUnnamed_12,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_15,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_14,
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
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
    pub output: C2RustUnnamed_25,
    pub buffer: C2RustUnnamed_24,
    pub raw_buffer: C2RustUnnamed_23,
    pub encoding: yaml_encoding_t,
    pub canonical: libc::c_int,
    pub best_indent: libc::c_int,
    pub best_width: libc::c_int,
    pub unicode: libc::c_int,
    pub line_break: yaml_break_t,
    pub states: C2RustUnnamed_22,
    pub state: yaml_emitter_state_t,
    pub events: C2RustUnnamed_21,
    pub indents: C2RustUnnamed_20,
    pub tag_directives: C2RustUnnamed_19,
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
    pub anchor_data: C2RustUnnamed_18,
    pub tag_data: C2RustUnnamed_17,
    pub scalar_data: C2RustUnnamed_16,
    pub opened: libc::c_int,
    pub closed: libc::c_int,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: libc::c_int,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
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
pub struct C2RustUnnamed_17 {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
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
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub string: C2RustUnnamed_26,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
    pub size_written: *mut size_t,
}
pub type yaml_emitter_t = yaml_emitter_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_string_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
}
unsafe extern "C" fn yaml_emitter_set_emitter_error(
    mut emitter: *mut yaml_emitter_t,
    mut problem: *const libc::c_char,
) -> libc::c_int {
    (*emitter).error = YAML_EMITTER_ERROR;
    let ref mut fresh0 = (*emitter).problem;
    *fresh0 = problem;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_emit(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    if if (*emitter).events.tail != (*emitter).events.end
        || yaml_queue_extend(
            &mut (*emitter).events.start as *mut *mut yaml_event_t
                as *mut *mut libc::c_void,
            &mut (*emitter).events.head as *mut *mut yaml_event_t
                as *mut *mut libc::c_void,
            &mut (*emitter).events.tail as *mut *mut yaml_event_t
                as *mut *mut libc::c_void,
            &mut (*emitter).events.end as *mut *mut yaml_event_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh1 = (*emitter).events.tail;
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).offset(1);
        *fresh2 = *event;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        yaml_event_delete(event);
        return 0 as libc::c_int;
    }
    while yaml_emitter_need_more_events(emitter) == 0 {
        if yaml_emitter_analyze_event(emitter, (*emitter).events.head) == 0 {
            return 0 as libc::c_int;
        }
        if yaml_emitter_state_machine(emitter, (*emitter).events.head) == 0 {
            return 0 as libc::c_int;
        }
        let ref mut fresh3 = (*emitter).events.head;
        let fresh4 = *fresh3;
        *fresh3 = (*fresh3).offset(1);
        yaml_event_delete(fresh4);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_need_more_events(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    let mut level: libc::c_int = 0 as libc::c_int;
    let mut accumulate: libc::c_int = 0 as libc::c_int;
    let mut event: *mut yaml_event_t = 0 as *mut yaml_event_t;
    if (*emitter).events.head == (*emitter).events.tail {
        return 1 as libc::c_int;
    }
    match (*(*emitter).events.head).type_0 as libc::c_uint {
        3 => {
            accumulate = 1 as libc::c_int;
        }
        7 => {
            accumulate = 2 as libc::c_int;
        }
        9 => {
            accumulate = 3 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    }
    if ((*emitter).events.tail).offset_from((*emitter).events.head) as libc::c_long
        > accumulate as libc::c_long
    {
        return 0 as libc::c_int;
    }
    event = (*emitter).events.head;
    while event != (*emitter).events.tail {
        match (*event).type_0 as libc::c_uint {
            1 | 3 | 7 | 9 => {
                level += 1 as libc::c_int;
            }
            2 | 4 | 8 | 10 => {
                level -= 1 as libc::c_int;
            }
            _ => {}
        }
        if level == 0 {
            return 0 as libc::c_int;
        }
        event = event.offset(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_append_tag_directive(
    mut emitter: *mut yaml_emitter_t,
    mut value: yaml_tag_directive_t,
    mut allow_duplicates: libc::c_int,
) -> libc::c_int {
    let mut tag_directive: *mut yaml_tag_directive_t = 0 as *mut yaml_tag_directive_t;
    let mut copy: yaml_tag_directive_t = {
        let mut init = yaml_tag_directive_s {
            handle: 0 as *mut yaml_char_t,
            prefix: 0 as *mut yaml_char_t,
        };
        init
    };
    tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        if strcmp(
            value.handle as *mut libc::c_char,
            (*tag_directive).handle as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            if allow_duplicates != 0 {
                return 1 as libc::c_int;
            }
            return yaml_emitter_set_emitter_error(
                emitter,
                b"duplicate %TAG directive\0" as *const u8 as *const libc::c_char,
            );
        }
        tag_directive = tag_directive.offset(1);
    }
    copy.handle = yaml_strdup(value.handle);
    copy.prefix = yaml_strdup(value.prefix);
    if (copy.handle).is_null() || (copy.prefix).is_null() {
        (*emitter).error = YAML_MEMORY_ERROR;
    } else if !(if (*emitter).tag_directives.top != (*emitter).tag_directives.end
            || yaml_stack_extend(
                &mut (*emitter).tag_directives.start as *mut *mut yaml_tag_directive_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).tag_directives.top as *mut *mut yaml_tag_directive_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).tag_directives.end as *mut *mut yaml_tag_directive_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh5 = (*emitter).tag_directives.top;
            let fresh6 = *fresh5;
            *fresh5 = (*fresh5).offset(1);
            *fresh6 = copy;
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0)
        {
        return 1 as libc::c_int
    }
    yaml_free(copy.handle as *mut libc::c_void);
    yaml_free(copy.prefix as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_increase_indent(
    mut emitter: *mut yaml_emitter_t,
    mut flow: libc::c_int,
    mut indentless: libc::c_int,
) -> libc::c_int {
    if if (*emitter).indents.top != (*emitter).indents.end
        || yaml_stack_extend(
            &mut (*emitter).indents.start as *mut *mut libc::c_int
                as *mut *mut libc::c_void,
            &mut (*emitter).indents.top as *mut *mut libc::c_int
                as *mut *mut libc::c_void,
            &mut (*emitter).indents.end as *mut *mut libc::c_int
                as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh7 = (*emitter).indents.top;
        let fresh8 = *fresh7;
        *fresh7 = (*fresh7).offset(1);
        *fresh8 = (*emitter).indent;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    if (*emitter).indent < 0 as libc::c_int {
        (*emitter)
            .indent = if flow != 0 { (*emitter).best_indent } else { 0 as libc::c_int };
    } else if indentless == 0 {
        (*emitter).indent += (*emitter).best_indent;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_state_machine(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    match (*emitter).state as libc::c_uint {
        0 => return yaml_emitter_emit_stream_start(emitter, event),
        1 => return yaml_emitter_emit_document_start(emitter, event, 1 as libc::c_int),
        2 => return yaml_emitter_emit_document_start(emitter, event, 0 as libc::c_int),
        3 => return yaml_emitter_emit_document_content(emitter, event),
        4 => return yaml_emitter_emit_document_end(emitter, event),
        5 => {
            return yaml_emitter_emit_flow_sequence_item(emitter, event, 1 as libc::c_int);
        }
        6 => {
            return yaml_emitter_emit_flow_sequence_item(emitter, event, 0 as libc::c_int);
        }
        7 => return yaml_emitter_emit_flow_mapping_key(emitter, event, 1 as libc::c_int),
        8 => return yaml_emitter_emit_flow_mapping_key(emitter, event, 0 as libc::c_int),
        9 => {
            return yaml_emitter_emit_flow_mapping_value(emitter, event, 1 as libc::c_int);
        }
        10 => {
            return yaml_emitter_emit_flow_mapping_value(emitter, event, 0 as libc::c_int);
        }
        11 => {
            return yaml_emitter_emit_block_sequence_item(
                emitter,
                event,
                1 as libc::c_int,
            );
        }
        12 => {
            return yaml_emitter_emit_block_sequence_item(
                emitter,
                event,
                0 as libc::c_int,
            );
        }
        13 => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 1 as libc::c_int);
        }
        14 => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 0 as libc::c_int);
        }
        15 => {
            return yaml_emitter_emit_block_mapping_value(
                emitter,
                event,
                1 as libc::c_int,
            );
        }
        16 => {
            return yaml_emitter_emit_block_mapping_value(
                emitter,
                event,
                0 as libc::c_int,
            );
        }
        17 => {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"expected nothing after STREAM-END\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_emit_stream_start(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    (*emitter).open_ended = 0 as libc::c_int;
    if (*event).type_0 as libc::c_uint
        == YAML_STREAM_START_EVENT as libc::c_int as libc::c_uint
    {
        if (*emitter).encoding as u64 == 0 {
            (*emitter).encoding = (*event).data.stream_start.encoding;
        }
        if (*emitter).encoding as u64 == 0 {
            (*emitter).encoding = YAML_UTF8_ENCODING;
        }
        if (*emitter).best_indent < 2 as libc::c_int
            || (*emitter).best_indent > 9 as libc::c_int
        {
            (*emitter).best_indent = 2 as libc::c_int;
        }
        if (*emitter).best_width >= 0 as libc::c_int
            && (*emitter).best_width <= (*emitter).best_indent * 2 as libc::c_int
        {
            (*emitter).best_width = 80 as libc::c_int;
        }
        if (*emitter).best_width < 0 as libc::c_int {
            (*emitter).best_width = 2147483647 as libc::c_int;
        }
        if (*emitter).line_break as u64 == 0 {
            (*emitter).line_break = YAML_LN_BREAK;
        }
        (*emitter).indent = -(1 as libc::c_int);
        (*emitter).line = 0 as libc::c_int;
        (*emitter).column = 0 as libc::c_int;
        (*emitter).whitespace = 1 as libc::c_int;
        (*emitter).indention = 1 as libc::c_int;
        if (*emitter).encoding as libc::c_uint
            != YAML_UTF8_ENCODING as libc::c_int as libc::c_uint
        {
            if yaml_emitter_write_bom(emitter) == 0 {
                return 0 as libc::c_int;
            }
        }
        (*emitter).state = YAML_EMIT_FIRST_DOCUMENT_START_STATE;
        return 1 as libc::c_int;
    }
    return yaml_emitter_set_emitter_error(
        emitter,
        b"expected STREAM-START\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn yaml_emitter_emit_document_start(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: libc::c_int,
) -> libc::c_int {
    if (*event).type_0 as libc::c_uint
        == YAML_DOCUMENT_START_EVENT as libc::c_int as libc::c_uint
    {
        let mut default_tag_directives: [yaml_tag_directive_t; 3] = [
            {
                let mut init = yaml_tag_directive_s {
                    handle: b"!\0" as *const u8 as *const libc::c_char
                        as *mut yaml_char_t,
                    prefix: b"!\0" as *const u8 as *const libc::c_char
                        as *mut yaml_char_t,
                };
                init
            },
            {
                let mut init = yaml_tag_directive_s {
                    handle: b"!!\0" as *const u8 as *const libc::c_char
                        as *mut yaml_char_t,
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
        let mut tag_directive: *mut yaml_tag_directive_t = 0
            as *mut yaml_tag_directive_t;
        let mut implicit: libc::c_int = 0;
        if !((*event).data.document_start.version_directive).is_null() {
            if yaml_emitter_analyze_version_directive(
                emitter,
                *(*event).data.document_start.version_directive,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        tag_directive = (*event).data.document_start.tag_directives.start;
        while tag_directive != (*event).data.document_start.tag_directives.end {
            if yaml_emitter_analyze_tag_directive(emitter, *tag_directive) == 0 {
                return 0 as libc::c_int;
            }
            if yaml_emitter_append_tag_directive(
                emitter,
                *tag_directive,
                0 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            tag_directive = tag_directive.offset(1);
        }
        tag_directive = default_tag_directives.as_mut_ptr();
        while !((*tag_directive).handle).is_null() {
            if yaml_emitter_append_tag_directive(
                emitter,
                *tag_directive,
                1 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            tag_directive = tag_directive.offset(1);
        }
        implicit = (*event).data.document_start.implicit;
        if first == 0 || (*emitter).canonical != 0 {
            implicit = 0 as libc::c_int;
        }
        if (!((*event).data.document_start.version_directive).is_null()
            || (*event).data.document_start.tag_directives.start
                != (*event).data.document_start.tag_directives.end)
            && (*emitter).open_ended != 0
        {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as libc::c_int;
            }
        }
        (*emitter).open_ended = 0 as libc::c_int;
        if !((*event).data.document_start.version_directive).is_null() {
            implicit = 0 as libc::c_int;
            if yaml_emitter_write_indicator(
                emitter,
                b"%YAML\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            if (*(*event).data.document_start.version_directive).minor
                == 1 as libc::c_int
            {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"1.1\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if yaml_emitter_write_indicator(
                    emitter,
                    b"1.2\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) == 0
                {
                return 0 as libc::c_int
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as libc::c_int;
            }
        }
        if (*event).data.document_start.tag_directives.start
            != (*event).data.document_start.tag_directives.end
        {
            implicit = 0 as libc::c_int;
            tag_directive = (*event).data.document_start.tag_directives.start;
            while tag_directive != (*event).data.document_start.tag_directives.end {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"%TAG\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                if yaml_emitter_write_tag_handle(
                    emitter,
                    (*tag_directive).handle,
                    strlen((*tag_directive).handle as *mut libc::c_char),
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                if yaml_emitter_write_tag_content(
                    emitter,
                    (*tag_directive).prefix,
                    strlen((*tag_directive).prefix as *mut libc::c_char),
                    1 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                tag_directive = tag_directive.offset(1);
            }
        }
        if yaml_emitter_check_empty_document(emitter) != 0 {
            implicit = 0 as libc::c_int;
        }
        if implicit == 0 {
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as libc::c_int;
            }
            if yaml_emitter_write_indicator(
                emitter,
                b"---\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            if (*emitter).canonical != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
            }
        }
        (*emitter).state = YAML_EMIT_DOCUMENT_CONTENT_STATE;
        (*emitter).open_ended = 0 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if (*event).type_0 as libc::c_uint
            == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint
        {
            if (*emitter).open_ended == 2 as libc::c_int {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"...\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                (*emitter).open_ended = 0 as libc::c_int;
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
            }
            if yaml_emitter_flush(emitter) == 0 {
                return 0 as libc::c_int;
            }
            (*emitter).state = YAML_EMIT_END_STATE;
            return 1 as libc::c_int;
        }
    }
    return yaml_emitter_set_emitter_error(
        emitter,
        b"expected DOCUMENT-START or STREAM-END\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn yaml_emitter_emit_document_content(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &mut (*emitter).states.start as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.top as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.end as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh9 = (*emitter).states.top;
        let fresh10 = *fresh9;
        *fresh9 = (*fresh9).offset(1);
        *fresh10 = YAML_EMIT_DOCUMENT_END_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_document_end(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    if (*event).type_0 as libc::c_uint
        == YAML_DOCUMENT_END_EVENT as libc::c_int as libc::c_uint
    {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as libc::c_int;
        }
        if (*event).data.document_end.implicit == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            (*emitter).open_ended = 0 as libc::c_int;
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as libc::c_int;
            }
        } else if (*emitter).open_ended == 0 {
            (*emitter).open_ended = 1 as libc::c_int;
        }
        if yaml_emitter_flush(emitter) == 0 {
            return 0 as libc::c_int;
        }
        (*emitter).state = YAML_EMIT_DOCUMENT_START_STATE;
        while !((*emitter).tag_directives.start == (*emitter).tag_directives.top) {
            let ref mut fresh11 = (*emitter).tag_directives.top;
            *fresh11 = (*fresh11).offset(-1);
            let mut tag_directive: yaml_tag_directive_t = **fresh11;
            yaml_free(tag_directive.handle as *mut libc::c_void);
            yaml_free(tag_directive.prefix as *mut libc::c_void);
        }
        return 1 as libc::c_int;
    }
    return yaml_emitter_set_emitter_error(
        emitter,
        b"expected DOCUMENT-END\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn yaml_emitter_emit_flow_sequence_item(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: libc::c_int,
) -> libc::c_int {
    if first != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b"[\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if yaml_emitter_increase_indent(emitter, 1 as libc::c_int, 0 as libc::c_int) == 0
        {
            return 0 as libc::c_int;
        }
        let ref mut fresh12 = (*emitter).flow_level;
        *fresh12 += 1;
    }
    if (*event).type_0 as libc::c_uint
        == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint
    {
        let ref mut fresh13 = (*emitter).flow_level;
        *fresh13 -= 1;
        let ref mut fresh14 = (*emitter).indents.top;
        *fresh14 = (*fresh14).offset(-1);
        (*emitter).indent = **fresh14;
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b",\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as libc::c_int;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b"]\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        let ref mut fresh15 = (*emitter).states.top;
        *fresh15 = (*fresh15).offset(-1);
        (*emitter).state = **fresh15;
        return 1 as libc::c_int;
    }
    if first == 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b",\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as libc::c_int;
        }
    }
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &mut (*emitter).states.start as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.top as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.end as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh16 = (*emitter).states.top;
        let fresh17 = *fresh16;
        *fresh16 = (*fresh16).offset(1);
        *fresh17 = YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_flow_mapping_key(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: libc::c_int,
) -> libc::c_int {
    if first != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b"{\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if yaml_emitter_increase_indent(emitter, 1 as libc::c_int, 0 as libc::c_int) == 0
        {
            return 0 as libc::c_int;
        }
        let ref mut fresh18 = (*emitter).flow_level;
        *fresh18 += 1;
    }
    if (*event).type_0 as libc::c_uint
        == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint
    {
        let ref mut fresh19 = (*emitter).flow_level;
        *fresh19 -= 1;
        let ref mut fresh20 = (*emitter).indents.top;
        *fresh20 = (*fresh20).offset(-1);
        (*emitter).indent = **fresh20;
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b",\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as libc::c_int;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b"}\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        let ref mut fresh21 = (*emitter).states.top;
        *fresh21 = (*fresh21).offset(-1);
        (*emitter).state = **fresh21;
        return 1 as libc::c_int;
    }
    if first == 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b",\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as libc::c_int;
        }
    }
    if (*emitter).canonical == 0 && yaml_emitter_check_simple_key(emitter) != 0 {
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                &mut (*emitter).states.start as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).states.top as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).states.end as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh22 = (*emitter).states.top;
            let fresh23 = *fresh22;
            *fresh22 = (*fresh22).offset(1);
            *fresh23 = YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE;
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        return yaml_emitter_emit_node(
            emitter,
            event,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
        );
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"?\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                &mut (*emitter).states.start as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).states.top as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).states.end as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh24 = (*emitter).states.top;
            let fresh25 = *fresh24;
            *fresh24 = (*fresh24).offset(1);
            *fresh25 = YAML_EMIT_FLOW_MAPPING_VALUE_STATE;
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        return yaml_emitter_emit_node(
            emitter,
            event,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        );
    };
}
unsafe extern "C" fn yaml_emitter_emit_flow_mapping_value(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut simple: libc::c_int,
) -> libc::c_int {
    if simple != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    } else {
        if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as libc::c_int;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &mut (*emitter).states.start as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.top as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.end as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh26 = (*emitter).states.top;
        let fresh27 = *fresh26;
        *fresh26 = (*fresh26).offset(1);
        *fresh27 = YAML_EMIT_FLOW_MAPPING_KEY_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_block_sequence_item(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: libc::c_int,
) -> libc::c_int {
    if first != 0 {
        if yaml_emitter_increase_indent(
            emitter,
            0 as libc::c_int,
            ((*emitter).mapping_context != 0 && (*emitter).indention == 0) as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if (*event).type_0 as libc::c_uint
        == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint
    {
        let ref mut fresh28 = (*emitter).indents.top;
        *fresh28 = (*fresh28).offset(-1);
        (*emitter).indent = **fresh28;
        let ref mut fresh29 = (*emitter).states.top;
        *fresh29 = (*fresh29).offset(-1);
        (*emitter).state = **fresh29;
        return 1 as libc::c_int;
    }
    if yaml_emitter_write_indent(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"-\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &mut (*emitter).states.start as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.top as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.end as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh30 = (*emitter).states.top;
        let fresh31 = *fresh30;
        *fresh30 = (*fresh30).offset(1);
        *fresh31 = YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_block_mapping_key(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: libc::c_int,
) -> libc::c_int {
    if first != 0 {
        if yaml_emitter_increase_indent(emitter, 0 as libc::c_int, 0 as libc::c_int) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if (*event).type_0 as libc::c_uint
        == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint
    {
        let ref mut fresh32 = (*emitter).indents.top;
        *fresh32 = (*fresh32).offset(-1);
        (*emitter).indent = **fresh32;
        let ref mut fresh33 = (*emitter).states.top;
        *fresh33 = (*fresh33).offset(-1);
        (*emitter).state = **fresh33;
        return 1 as libc::c_int;
    }
    if yaml_emitter_write_indent(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_check_simple_key(emitter) != 0 {
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                &mut (*emitter).states.start as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).states.top as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).states.end as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh34 = (*emitter).states.top;
            let fresh35 = *fresh34;
            *fresh34 = (*fresh34).offset(1);
            *fresh35 = YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE;
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        return yaml_emitter_emit_node(
            emitter,
            event,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
        );
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"?\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                &mut (*emitter).states.start as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).states.top as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
                &mut (*emitter).states.end as *mut *mut yaml_emitter_state_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let ref mut fresh36 = (*emitter).states.top;
            let fresh37 = *fresh36;
            *fresh36 = (*fresh36).offset(1);
            *fresh37 = YAML_EMIT_BLOCK_MAPPING_VALUE_STATE;
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        return yaml_emitter_emit_node(
            emitter,
            event,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        );
    };
}
unsafe extern "C" fn yaml_emitter_emit_block_mapping_value(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut simple: libc::c_int,
) -> libc::c_int {
    if simple != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    } else {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as libc::c_int;
        }
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &mut (*emitter).states.start as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.top as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
            &mut (*emitter).states.end as *mut *mut yaml_emitter_state_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh38 = (*emitter).states.top;
        let fresh39 = *fresh38;
        *fresh38 = (*fresh38).offset(1);
        *fresh39 = YAML_EMIT_BLOCK_MAPPING_KEY_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_node(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut root: libc::c_int,
    mut sequence: libc::c_int,
    mut mapping: libc::c_int,
    mut simple_key: libc::c_int,
) -> libc::c_int {
    (*emitter).root_context = root;
    (*emitter).sequence_context = sequence;
    (*emitter).mapping_context = mapping;
    (*emitter).simple_key_context = simple_key;
    match (*event).type_0 as libc::c_uint {
        5 => return yaml_emitter_emit_alias(emitter, event),
        6 => return yaml_emitter_emit_scalar(emitter, event),
        7 => return yaml_emitter_emit_sequence_start(emitter, event),
        9 => return yaml_emitter_emit_mapping_start(emitter, event),
        _ => {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"expected SCALAR, SEQUENCE-START, MAPPING-START, or ALIAS\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn yaml_emitter_emit_alias(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if (*emitter).simple_key_context != 0 {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                let ref mut fresh40 = (*emitter).buffer.pointer;
                let fresh41 = *fresh40;
                *fresh40 = (*fresh40).offset(1);
                *fresh41 = ' ' as i32 as yaml_char_t;
                let ref mut fresh42 = (*emitter).column;
                *fresh42 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    let ref mut fresh43 = (*emitter).states.top;
    *fresh43 = (*fresh43).offset(-1);
    (*emitter).state = **fresh43;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_emit_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_select_scalar_style(emitter, event) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_increase_indent(emitter, 1 as libc::c_int, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_process_scalar(emitter) == 0 {
        return 0 as libc::c_int;
    }
    let ref mut fresh44 = (*emitter).indents.top;
    *fresh44 = (*fresh44).offset(-1);
    (*emitter).indent = **fresh44;
    let ref mut fresh45 = (*emitter).states.top;
    *fresh45 = (*fresh45).offset(-1);
    (*emitter).state = **fresh45;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_emit_sequence_start(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if (*emitter).flow_level != 0 || (*emitter).canonical != 0
        || (*event).data.sequence_start.style as libc::c_uint
            == YAML_FLOW_SEQUENCE_STYLE as libc::c_int as libc::c_uint
        || yaml_emitter_check_empty_sequence(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_emit_mapping_start(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if (*emitter).flow_level != 0 || (*emitter).canonical != 0
        || (*event).data.mapping_start.style as libc::c_uint
            == YAML_FLOW_MAPPING_STYLE as libc::c_int as libc::c_uint
        || yaml_emitter_check_empty_mapping(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_check_empty_document(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_check_empty_sequence(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    if (((*emitter).events.tail).offset_from((*emitter).events.head) as libc::c_long)
        < 2 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    return ((*((*emitter).events.head).offset(0 as libc::c_int as isize)).type_0
        as libc::c_uint == YAML_SEQUENCE_START_EVENT as libc::c_int as libc::c_uint
        && (*((*emitter).events.head).offset(1 as libc::c_int as isize)).type_0
            as libc::c_uint == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_check_empty_mapping(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    if (((*emitter).events.tail).offset_from((*emitter).events.head) as libc::c_long)
        < 2 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    return ((*((*emitter).events.head).offset(0 as libc::c_int as isize)).type_0
        as libc::c_uint == YAML_MAPPING_START_EVENT as libc::c_int as libc::c_uint
        && (*((*emitter).events.head).offset(1 as libc::c_int as isize)).type_0
            as libc::c_uint == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_check_simple_key(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    let mut event: *mut yaml_event_t = (*emitter).events.head;
    let mut length: size_t = 0 as libc::c_int as size_t;
    match (*event).type_0 as libc::c_uint {
        5 => {
            length = (length as libc::c_ulong)
                .wrapping_add((*emitter).anchor_data.anchor_length) as size_t as size_t;
        }
        6 => {
            if (*emitter).scalar_data.multiline != 0 {
                return 0 as libc::c_int;
            }
            length = (length as libc::c_ulong)
                .wrapping_add(
                    ((*emitter).anchor_data.anchor_length)
                        .wrapping_add((*emitter).tag_data.handle_length)
                        .wrapping_add((*emitter).tag_data.suffix_length)
                        .wrapping_add((*emitter).scalar_data.length),
                ) as size_t as size_t;
        }
        7 => {
            if yaml_emitter_check_empty_sequence(emitter) == 0 {
                return 0 as libc::c_int;
            }
            length = (length as libc::c_ulong)
                .wrapping_add(
                    ((*emitter).anchor_data.anchor_length)
                        .wrapping_add((*emitter).tag_data.handle_length)
                        .wrapping_add((*emitter).tag_data.suffix_length),
                ) as size_t as size_t;
        }
        9 => {
            if yaml_emitter_check_empty_mapping(emitter) == 0 {
                return 0 as libc::c_int;
            }
            length = (length as libc::c_ulong)
                .wrapping_add(
                    ((*emitter).anchor_data.anchor_length)
                        .wrapping_add((*emitter).tag_data.handle_length)
                        .wrapping_add((*emitter).tag_data.suffix_length),
                ) as size_t as size_t;
        }
        _ => return 0 as libc::c_int,
    }
    if length > 128 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_select_scalar_style(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut style: yaml_scalar_style_t = (*event).data.scalar.style;
    let mut no_tag: libc::c_int = (((*emitter).tag_data.handle).is_null()
        && ((*emitter).tag_data.suffix).is_null()) as libc::c_int;
    if no_tag != 0 && (*event).data.scalar.plain_implicit == 0
        && (*event).data.scalar.quoted_implicit == 0
    {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"neither tag nor implicit flags are specified\0" as *const u8
                as *const libc::c_char,
        );
    }
    if style as libc::c_uint == YAML_ANY_SCALAR_STYLE as libc::c_int as libc::c_uint {
        style = YAML_PLAIN_SCALAR_STYLE;
    }
    if (*emitter).canonical != 0 {
        style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
    }
    if (*emitter).simple_key_context != 0 && (*emitter).scalar_data.multiline != 0 {
        style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
    }
    if style as libc::c_uint == YAML_PLAIN_SCALAR_STYLE as libc::c_int as libc::c_uint {
        if (*emitter).flow_level != 0 && (*emitter).scalar_data.flow_plain_allowed == 0
            || (*emitter).flow_level == 0
                && (*emitter).scalar_data.block_plain_allowed == 0
        {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
        if (*emitter).scalar_data.length == 0
            && ((*emitter).flow_level != 0 || (*emitter).simple_key_context != 0)
        {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
        if no_tag != 0 && (*event).data.scalar.plain_implicit == 0 {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
    }
    if style as libc::c_uint
        == YAML_SINGLE_QUOTED_SCALAR_STYLE as libc::c_int as libc::c_uint
    {
        if (*emitter).scalar_data.single_quoted_allowed == 0 {
            style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
        }
    }
    if style as libc::c_uint == YAML_LITERAL_SCALAR_STYLE as libc::c_int as libc::c_uint
        || style as libc::c_uint
            == YAML_FOLDED_SCALAR_STYLE as libc::c_int as libc::c_uint
    {
        if (*emitter).scalar_data.block_allowed == 0 || (*emitter).flow_level != 0
            || (*emitter).simple_key_context != 0
        {
            style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
        }
    }
    if no_tag != 0 && (*event).data.scalar.quoted_implicit == 0
        && style as libc::c_uint
            != YAML_PLAIN_SCALAR_STYLE as libc::c_int as libc::c_uint
    {
        let ref mut fresh46 = (*emitter).tag_data.handle;
        *fresh46 = b"!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t;
        (*emitter).tag_data.handle_length = 1 as libc::c_int as size_t;
    }
    (*emitter).scalar_data.style = style;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_process_anchor(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    if ((*emitter).anchor_data.anchor).is_null() {
        return 1 as libc::c_int;
    }
    if yaml_emitter_write_indicator(
        emitter,
        if (*emitter).anchor_data.alias != 0 {
            b"*\0" as *const u8 as *const libc::c_char
        } else {
            b"&\0" as *const u8 as *const libc::c_char
        },
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return yaml_emitter_write_anchor(
        emitter,
        (*emitter).anchor_data.anchor,
        (*emitter).anchor_data.anchor_length,
    );
}
unsafe extern "C" fn yaml_emitter_process_tag(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    if ((*emitter).tag_data.handle).is_null() && ((*emitter).tag_data.suffix).is_null() {
        return 1 as libc::c_int;
    }
    if !((*emitter).tag_data.handle).is_null() {
        if yaml_emitter_write_tag_handle(
            emitter,
            (*emitter).tag_data.handle,
            (*emitter).tag_data.handle_length,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if !((*emitter).tag_data.suffix).is_null() {
            if yaml_emitter_write_tag_content(
                emitter,
                (*emitter).tag_data.suffix,
                (*emitter).tag_data.suffix_length,
                0 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"!<\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if yaml_emitter_write_tag_content(
            emitter,
            (*emitter).tag_data.suffix,
            (*emitter).tag_data.suffix_length,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if yaml_emitter_write_indicator(
            emitter,
            b">\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_process_scalar(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    match (*emitter).scalar_data.style as libc::c_uint {
        1 => {
            return yaml_emitter_write_plain_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
                ((*emitter).simple_key_context == 0) as libc::c_int,
            );
        }
        2 => {
            return yaml_emitter_write_single_quoted_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
                ((*emitter).simple_key_context == 0) as libc::c_int,
            );
        }
        3 => {
            return yaml_emitter_write_double_quoted_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
                ((*emitter).simple_key_context == 0) as libc::c_int,
            );
        }
        4 => {
            return yaml_emitter_write_literal_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
            );
        }
        5 => {
            return yaml_emitter_write_folded_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
            );
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_version_directive(
    mut emitter: *mut yaml_emitter_t,
    mut version_directive: yaml_version_directive_t,
) -> libc::c_int {
    if version_directive.major != 1 as libc::c_int
        || version_directive.minor != 1 as libc::c_int
            && version_directive.minor != 2 as libc::c_int
    {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"incompatible %YAML directive\0" as *const u8 as *const libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_tag_directive(
    mut emitter: *mut yaml_emitter_t,
    mut tag_directive: yaml_tag_directive_t,
) -> libc::c_int {
    let mut handle: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    let mut prefix: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    let mut handle_length: size_t = 0;
    let mut prefix_length: size_t = 0;
    handle_length = strlen(tag_directive.handle as *mut libc::c_char);
    prefix_length = strlen(tag_directive.prefix as *mut libc::c_char);
    handle.start = tag_directive.handle;
    handle.end = (tag_directive.handle).offset(handle_length as isize);
    handle.pointer = tag_directive.handle;
    prefix.start = tag_directive.prefix;
    prefix.end = (tag_directive.prefix).offset(prefix_length as isize);
    prefix.pointer = tag_directive.prefix;
    if handle.start == handle.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must not be empty\0" as *const u8 as *const libc::c_char,
        );
    }
    if *(handle.start).offset(0 as libc::c_int as isize) as libc::c_int != '!' as i32 {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must start with '!'\0" as *const u8 as *const libc::c_char,
        );
    }
    if *(handle.end).offset(-(1 as libc::c_int) as isize) as libc::c_int != '!' as i32 {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must end with '!'\0" as *const u8 as *const libc::c_char,
        );
    }
    handle.pointer = (handle.pointer).offset(1);
    while handle.pointer < (handle.end).offset(-(1 as libc::c_int as isize)) {
        if !(*(handle.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            >= '0' as i32 as yaml_char_t as libc::c_int
            && *(handle.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                <= '9' as i32 as yaml_char_t as libc::c_int
            || *(handle.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *(handle.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'Z' as i32 as yaml_char_t as libc::c_int
            || *(handle.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *(handle.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'z' as i32 as yaml_char_t as libc::c_int
            || *(handle.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '_' as i32
            || *(handle.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32)
        {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"tag handle must contain alphanumerical characters only\0" as *const u8
                    as *const libc::c_char,
            );
        }
        handle
            .pointer = (handle.pointer)
            .offset(
                (if *(handle.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(handle.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                    {
                    2 as libc::c_int
                } else if *(handle.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                    {
                    3 as libc::c_int
                } else if *(handle.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                    {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            );
    }
    if prefix.start == prefix.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag prefix must not be empty\0" as *const u8 as *const libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_anchor(
    mut emitter: *mut yaml_emitter_t,
    mut anchor: *mut yaml_char_t,
    mut alias: libc::c_int,
) -> libc::c_int {
    let mut anchor_length: size_t = 0;
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    anchor_length = strlen(anchor as *mut libc::c_char);
    string.start = anchor;
    string.end = anchor.offset(anchor_length as isize);
    string.pointer = anchor;
    if string.start == string.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            if alias != 0 {
                b"alias value must not be empty\0" as *const u8 as *const libc::c_char
            } else {
                b"anchor value must not be empty\0" as *const u8 as *const libc::c_char
            },
        );
    }
    while string.pointer != string.end {
        if !(*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            >= '0' as i32 as yaml_char_t as libc::c_int
            && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                <= '9' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'Z' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'z' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '_' as i32
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32)
        {
            return yaml_emitter_set_emitter_error(
                emitter,
                if alias != 0 {
                    b"alias value must contain alphanumerical characters only\0"
                        as *const u8 as *const libc::c_char
                } else {
                    b"anchor value must contain alphanumerical characters only\0"
                        as *const u8 as *const libc::c_char
                },
            );
        }
        string
            .pointer = (string.pointer)
            .offset(
                (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                    {
                    2 as libc::c_int
                } else if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                    {
                    3 as libc::c_int
                } else if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                    {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            );
    }
    let ref mut fresh47 = (*emitter).anchor_data.anchor;
    *fresh47 = string.start;
    (*emitter)
        .anchor_data
        .anchor_length = (string.end).offset_from(string.start) as libc::c_long
        as size_t;
    (*emitter).anchor_data.alias = alias;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_tag(
    mut emitter: *mut yaml_emitter_t,
    mut tag: *mut yaml_char_t,
) -> libc::c_int {
    let mut tag_length: size_t = 0;
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    let mut tag_directive: *mut yaml_tag_directive_t = 0 as *mut yaml_tag_directive_t;
    tag_length = strlen(tag as *mut libc::c_char);
    string.start = tag;
    string.end = tag.offset(tag_length as isize);
    string.pointer = tag;
    if string.start == string.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag value must not be empty\0" as *const u8 as *const libc::c_char,
        );
    }
    tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        let mut prefix_length: size_t = strlen(
            (*tag_directive).prefix as *mut libc::c_char,
        );
        if prefix_length
            < (string.end).offset_from(string.start) as libc::c_long as size_t
            && strncmp(
                (*tag_directive).prefix as *mut libc::c_char,
                string.start as *mut libc::c_char,
                prefix_length,
            ) == 0 as libc::c_int
        {
            let ref mut fresh48 = (*emitter).tag_data.handle;
            *fresh48 = (*tag_directive).handle;
            (*emitter)
                .tag_data
                .handle_length = strlen((*tag_directive).handle as *mut libc::c_char);
            let ref mut fresh49 = (*emitter).tag_data.suffix;
            *fresh49 = (string.start).offset(prefix_length as isize);
            (*emitter)
                .tag_data
                .suffix_length = ((string.end).offset_from(string.start) as libc::c_long
                as libc::c_ulong)
                .wrapping_sub(prefix_length);
            return 1 as libc::c_int;
        }
        tag_directive = tag_directive.offset(1);
    }
    let ref mut fresh50 = (*emitter).tag_data.suffix;
    *fresh50 = string.start;
    (*emitter)
        .tag_data
        .suffix_length = (string.end).offset_from(string.start) as libc::c_long
        as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    let mut block_indicators: libc::c_int = 0 as libc::c_int;
    let mut flow_indicators: libc::c_int = 0 as libc::c_int;
    let mut line_breaks: libc::c_int = 0 as libc::c_int;
    let mut special_characters: libc::c_int = 0 as libc::c_int;
    let mut leading_space: libc::c_int = 0 as libc::c_int;
    let mut leading_break: libc::c_int = 0 as libc::c_int;
    let mut trailing_space: libc::c_int = 0 as libc::c_int;
    let mut trailing_break: libc::c_int = 0 as libc::c_int;
    let mut break_space: libc::c_int = 0 as libc::c_int;
    let mut space_break: libc::c_int = 0 as libc::c_int;
    let mut preceded_by_whitespace: libc::c_int = 0 as libc::c_int;
    let mut followed_by_whitespace: libc::c_int = 0 as libc::c_int;
    let mut previous_space: libc::c_int = 0 as libc::c_int;
    let mut previous_break: libc::c_int = 0 as libc::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    let ref mut fresh51 = (*emitter).scalar_data.value;
    *fresh51 = value;
    (*emitter).scalar_data.length = length;
    if string.start == string.end {
        (*emitter).scalar_data.multiline = 0 as libc::c_int;
        (*emitter).scalar_data.flow_plain_allowed = 0 as libc::c_int;
        (*emitter).scalar_data.block_plain_allowed = 1 as libc::c_int;
        (*emitter).scalar_data.single_quoted_allowed = 1 as libc::c_int;
        (*emitter).scalar_data.block_allowed = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
        == '-' as i32 as yaml_char_t as libc::c_int
        && *(string.pointer).offset(1 as libc::c_int as isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && *(string.pointer).offset(2 as libc::c_int as isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
            && *(string.pointer).offset(1 as libc::c_int as isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
            && *(string.pointer).offset(2 as libc::c_int as isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
    {
        block_indicators = 1 as libc::c_int;
        flow_indicators = 1 as libc::c_int;
    }
    preceded_by_whitespace = 1 as libc::c_int;
    followed_by_whitespace = (*(string.pointer)
        .offset(
            (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int == 0 as libc::c_int
            {
                1 as libc::c_int
            } else {
                (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else {
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                    })
                })
            }) as isize,
        ) as libc::c_int == ' ' as i32 as yaml_char_t as libc::c_int
        || *(string.pointer)
            .offset(
                (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                            {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                        })
                    })
                }) as isize,
            ) as libc::c_int == '\t' as i32 as yaml_char_t as libc::c_int
        || (*(string.pointer)
            .offset(
                (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                            {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                        })
                    })
                }) as isize,
            ) as libc::c_int == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer)
                .offset(
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            })
                        })
                    }) as isize,
                ) as libc::c_int == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer)
                .offset(
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            })
                        })
                    }) as isize,
                ) as libc::c_int == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset(
                        ((if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                    {
                                        4 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                })
                            })
                        }) + 1 as libc::c_int) as isize,
                    ) as libc::c_int == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer)
                .offset(
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            })
                        })
                    }) as isize,
                ) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset(
                        ((if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                    {
                                        4 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                })
                            })
                        }) + 1 as libc::c_int) as isize,
                    ) as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset(
                        ((if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                    {
                                        4 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                })
                            })
                        }) + 2 as libc::c_int) as isize,
                    ) as libc::c_int == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer)
                .offset(
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            })
                        })
                    }) as isize,
                ) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset(
                        ((if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                    {
                                        4 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                })
                            })
                        }) + 1 as libc::c_int) as isize,
                    ) as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset(
                        ((if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                    {
                                        4 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                })
                            })
                        }) + 2 as libc::c_int) as isize,
                    ) as libc::c_int == -87i32 as yaml_char_t as libc::c_int
            || *(string.pointer)
                .offset(
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            })
                        })
                    }) as isize,
                ) as libc::c_int == '\0' as i32 as yaml_char_t as libc::c_int))
        as libc::c_int;
    while string.pointer != string.end {
        if string.start == string.pointer {
            if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '#' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == ',' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '[' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == ']' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '{' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '}' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '&' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '*' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '!' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '|' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '>' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\'' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '"' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '%' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '@' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '`' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1 as libc::c_int;
                block_indicators = 1 as libc::c_int;
            }
            if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '?' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == ':' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1 as libc::c_int;
                if followed_by_whitespace != 0 {
                    block_indicators = 1 as libc::c_int;
                }
            }
            if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32 as yaml_char_t as libc::c_int
                && followed_by_whitespace != 0
            {
                flow_indicators = 1 as libc::c_int;
                block_indicators = 1 as libc::c_int;
            }
        } else {
            if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '?' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '[' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == ']' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '{' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '}' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1 as libc::c_int;
            }
            if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == ':' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1 as libc::c_int;
                if followed_by_whitespace != 0 {
                    block_indicators = 1 as libc::c_int;
                }
            }
            if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '#' as i32 as yaml_char_t as libc::c_int
                && preceded_by_whitespace != 0
            {
                flow_indicators = 1 as libc::c_int;
                block_indicators = 1 as libc::c_int;
            }
        }
        if !(*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == 0xa as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                >= 0x20 as libc::c_int
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    <= 0x7e as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == 0xc2 as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int >= 0xa0 as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                > 0xc2 as libc::c_int
                && (*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int)
                    < 0xed as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == 0xed as libc::c_int
                && (*(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int) < 0xa0 as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == 0xee as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == 0xef as libc::c_int
                && !(*(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == 0xbb as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == 0xbf as libc::c_int)
                && !(*(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == 0xbf as libc::c_int
                    && (*(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == 0xbe as libc::c_int
                        || *(string.pointer)
                            .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int == 0xbf as libc::c_int)))
            || !(*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                <= '\u{7f}' as i32 as yaml_char_t as libc::c_int)
                && (*emitter).unicode == 0
        {
            special_characters = 1 as libc::c_int;
        }
        if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -87i32 as yaml_char_t as libc::c_int
        {
            line_breaks = 1 as libc::c_int;
        }
        if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if string.start == string.pointer {
                leading_space = 1 as libc::c_int;
            }
            if (string.pointer)
                .offset(
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            })
                        })
                    }) as isize,
                ) == string.end
            {
                trailing_space = 1 as libc::c_int;
            }
            if previous_break != 0 {
                break_space = 1 as libc::c_int;
            }
            previous_space = 1 as libc::c_int;
            previous_break = 0 as libc::c_int;
        } else if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -87i32 as yaml_char_t as libc::c_int
            {
            if string.start == string.pointer {
                leading_break = 1 as libc::c_int;
            }
            if (string.pointer)
                .offset(
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            })
                        })
                    }) as isize,
                ) == string.end
            {
                trailing_break = 1 as libc::c_int;
            }
            if previous_space != 0 {
                space_break = 1 as libc::c_int;
            }
            previous_space = 0 as libc::c_int;
            previous_break = 1 as libc::c_int;
        } else {
            previous_space = 0 as libc::c_int;
            previous_break = 0 as libc::c_int;
        }
        preceded_by_whitespace = (*(string.pointer).offset(0 as libc::c_int as isize)
            as libc::c_int == ' ' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
            || (*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -87i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int)) as libc::c_int;
        string
            .pointer = (string.pointer)
            .offset(
                (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                    {
                    2 as libc::c_int
                } else if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                    {
                    3 as libc::c_int
                } else if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                    {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            );
        if string.pointer != string.end {
            followed_by_whitespace = (*(string.pointer)
                .offset(
                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                        as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            })
                        })
                    }) as isize,
                ) as libc::c_int == ' ' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer)
                    .offset(
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                    {
                                        4 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                })
                            })
                        }) as isize,
                    ) as libc::c_int == '\t' as i32 as yaml_char_t as libc::c_int
                || (*(string.pointer)
                    .offset(
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else {
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                    {
                                        4 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                })
                            })
                        }) as isize,
                    ) as libc::c_int == '\r' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer)
                        .offset(
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                {
                                    2 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                    {
                                        3 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        })
                                    })
                                })
                            }) as isize,
                        ) as libc::c_int == '\n' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer)
                        .offset(
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                {
                                    2 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                    {
                                        3 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        })
                                    })
                                })
                            }) as isize,
                        ) as libc::c_int == -62i32 as yaml_char_t as libc::c_int
                        && *(string.pointer)
                            .offset(
                                ((if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                                {
                                    1 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                    {
                                        2 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else {
                                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                            {
                                                4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            })
                                        })
                                    })
                                }) + 1 as libc::c_int) as isize,
                            ) as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                    || *(string.pointer)
                        .offset(
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                {
                                    2 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                    {
                                        3 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        })
                                    })
                                })
                            }) as isize,
                        ) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                        && *(string.pointer)
                            .offset(
                                ((if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                                {
                                    1 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                    {
                                        2 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else {
                                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                            {
                                                4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            })
                                        })
                                    })
                                }) + 1 as libc::c_int) as isize,
                            ) as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                        && *(string.pointer)
                            .offset(
                                ((if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                                {
                                    1 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                    {
                                        2 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else {
                                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                            {
                                                4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            })
                                        })
                                    })
                                }) + 2 as libc::c_int) as isize,
                            ) as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                    || *(string.pointer)
                        .offset(
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                {
                                    2 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                    {
                                        3 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        })
                                    })
                                })
                            }) as isize,
                        ) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                        && *(string.pointer)
                            .offset(
                                ((if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                                {
                                    1 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                    {
                                        2 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else {
                                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                            {
                                                4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            })
                                        })
                                    })
                                }) + 1 as libc::c_int) as isize,
                            ) as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                        && *(string.pointer)
                            .offset(
                                ((if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                                {
                                    1 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                    {
                                        2 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else {
                                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                            {
                                                4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            })
                                        })
                                    })
                                }) + 2 as libc::c_int) as isize,
                            ) as libc::c_int == -87i32 as yaml_char_t as libc::c_int
                    || *(string.pointer)
                        .offset(
                            (if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else {
                                (if *(string.pointer).offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                {
                                    2 as libc::c_int
                                } else {
                                    (if *(string.pointer).offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                    {
                                        3 as libc::c_int
                                    } else {
                                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                                            as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        })
                                    })
                                })
                            }) as isize,
                        ) as libc::c_int == '\0' as i32 as yaml_char_t as libc::c_int))
                as libc::c_int;
        }
    }
    (*emitter).scalar_data.multiline = line_breaks;
    (*emitter).scalar_data.flow_plain_allowed = 1 as libc::c_int;
    (*emitter).scalar_data.block_plain_allowed = 1 as libc::c_int;
    (*emitter).scalar_data.single_quoted_allowed = 1 as libc::c_int;
    (*emitter).scalar_data.block_allowed = 1 as libc::c_int;
    if leading_space != 0 || leading_break != 0 || trailing_space != 0
        || trailing_break != 0
    {
        (*emitter).scalar_data.flow_plain_allowed = 0 as libc::c_int;
        (*emitter).scalar_data.block_plain_allowed = 0 as libc::c_int;
    }
    if trailing_space != 0 {
        (*emitter).scalar_data.block_allowed = 0 as libc::c_int;
    }
    if break_space != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0 as libc::c_int;
        (*emitter).scalar_data.block_plain_allowed = 0 as libc::c_int;
        (*emitter).scalar_data.single_quoted_allowed = 0 as libc::c_int;
    }
    if space_break != 0 || special_characters != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0 as libc::c_int;
        (*emitter).scalar_data.block_plain_allowed = 0 as libc::c_int;
        (*emitter).scalar_data.single_quoted_allowed = 0 as libc::c_int;
        (*emitter).scalar_data.block_allowed = 0 as libc::c_int;
    }
    if line_breaks != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0 as libc::c_int;
        (*emitter).scalar_data.block_plain_allowed = 0 as libc::c_int;
    }
    if flow_indicators != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0 as libc::c_int;
    }
    if block_indicators != 0 {
        (*emitter).scalar_data.block_plain_allowed = 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_event(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let ref mut fresh52 = (*emitter).anchor_data.anchor;
    *fresh52 = 0 as *mut yaml_char_t;
    (*emitter).anchor_data.anchor_length = 0 as libc::c_int as size_t;
    let ref mut fresh53 = (*emitter).tag_data.handle;
    *fresh53 = 0 as *mut yaml_char_t;
    (*emitter).tag_data.handle_length = 0 as libc::c_int as size_t;
    let ref mut fresh54 = (*emitter).tag_data.suffix;
    *fresh54 = 0 as *mut yaml_char_t;
    (*emitter).tag_data.suffix_length = 0 as libc::c_int as size_t;
    let ref mut fresh55 = (*emitter).scalar_data.value;
    *fresh55 = 0 as *mut yaml_char_t;
    (*emitter).scalar_data.length = 0 as libc::c_int as size_t;
    match (*event).type_0 as libc::c_uint {
        5 => {
            if yaml_emitter_analyze_anchor(
                emitter,
                (*event).data.alias.anchor,
                1 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        6 => {
            if !((*event).data.scalar.anchor).is_null() {
                if yaml_emitter_analyze_anchor(
                    emitter,
                    (*event).data.scalar.anchor,
                    0 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            }
            if !((*event).data.scalar.tag).is_null()
                && ((*emitter).canonical != 0
                    || (*event).data.scalar.plain_implicit == 0
                        && (*event).data.scalar.quoted_implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.scalar.tag) == 0 {
                    return 0 as libc::c_int;
                }
            }
            if yaml_emitter_analyze_scalar(
                emitter,
                (*event).data.scalar.value,
                (*event).data.scalar.length,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        7 => {
            if !((*event).data.sequence_start.anchor).is_null() {
                if yaml_emitter_analyze_anchor(
                    emitter,
                    (*event).data.sequence_start.anchor,
                    0 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            }
            if !((*event).data.sequence_start.tag).is_null()
                && ((*emitter).canonical != 0
                    || (*event).data.sequence_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.sequence_start.tag)
                    == 0
                {
                    return 0 as libc::c_int;
                }
            }
            return 1 as libc::c_int;
        }
        9 => {
            if !((*event).data.mapping_start.anchor).is_null() {
                if yaml_emitter_analyze_anchor(
                    emitter,
                    (*event).data.mapping_start.anchor,
                    0 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            }
            if !((*event).data.mapping_start.tag).is_null()
                && ((*emitter).canonical != 0
                    || (*event).data.mapping_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.mapping_start.tag)
                    == 0
                {
                    return 0 as libc::c_int;
                }
            }
            return 1 as libc::c_int;
        }
        _ => return 1 as libc::c_int,
    };
}
unsafe extern "C" fn yaml_emitter_write_bom(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    if !(((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
    {
        return 0 as libc::c_int;
    }
    let ref mut fresh56 = (*emitter).buffer.pointer;
    let fresh57 = *fresh56;
    *fresh56 = (*fresh56).offset(1);
    *fresh57 = -17i32 as yaml_char_t;
    let ref mut fresh58 = (*emitter).buffer.pointer;
    let fresh59 = *fresh58;
    *fresh58 = (*fresh58).offset(1);
    *fresh59 = -69i32 as yaml_char_t;
    let ref mut fresh60 = (*emitter).buffer.pointer;
    let fresh61 = *fresh60;
    *fresh60 = (*fresh60).offset(1);
    *fresh61 = -65i32 as yaml_char_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_indent(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    let mut indent: libc::c_int = if (*emitter).indent >= 0 as libc::c_int {
        (*emitter).indent
    } else {
        0 as libc::c_int
    };
    if (*emitter).indention == 0 || (*emitter).column > indent
        || (*emitter).column == indent && (*emitter).whitespace == 0
    {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                if (*emitter).line_break as libc::c_uint
                    == YAML_CR_BREAK as libc::c_int as libc::c_uint
                {
                    let ref mut fresh62 = (*emitter).buffer.pointer;
                    let fresh63 = *fresh62;
                    *fresh62 = (*fresh62).offset(1);
                    *fresh63 = '\r' as i32 as yaml_char_t;
                } else {
                    if (*emitter).line_break as libc::c_uint
                        == YAML_LN_BREAK as libc::c_int as libc::c_uint
                    {
                        let ref mut fresh64 = (*emitter).buffer.pointer;
                        let fresh65 = *fresh64;
                        *fresh64 = (*fresh64).offset(1);
                        *fresh65 = '\n' as i32 as yaml_char_t;
                    } else {
                        if (*emitter).line_break as libc::c_uint
                            == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh66 = (*emitter).buffer.pointer;
                            let fresh67 = *fresh66;
                            *fresh66 = (*fresh66).offset(1);
                            *fresh67 = '\r' as i32 as yaml_char_t;
                            let ref mut fresh68 = (*emitter).buffer.pointer;
                            let fresh69 = *fresh68;
                            *fresh68 = (*fresh68).offset(1);
                            *fresh69 = '\n' as i32 as yaml_char_t;
                        } else {};
                    };
                };
                (*emitter).column = 0 as libc::c_int;
                let ref mut fresh70 = (*emitter).line;
                *fresh70 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while (*emitter).column < indent {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                let ref mut fresh71 = (*emitter).buffer.pointer;
                let fresh72 = *fresh71;
                *fresh71 = (*fresh71).offset(1);
                *fresh72 = ' ' as i32 as yaml_char_t;
                let ref mut fresh73 = (*emitter).column;
                *fresh73 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    (*emitter).whitespace = 1 as libc::c_int;
    (*emitter).indention = 1 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_indicator(
    mut emitter: *mut yaml_emitter_t,
    mut indicator: *const libc::c_char,
    mut need_whitespace: libc::c_int,
    mut is_whitespace: libc::c_int,
    mut is_indention: libc::c_int,
) -> libc::c_int {
    let mut indicator_length: size_t = 0;
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    indicator_length = strlen(indicator);
    string.start = indicator as *mut yaml_char_t;
    string.end = (indicator as *mut yaml_char_t).offset(indicator_length as isize);
    string.pointer = indicator as *mut yaml_char_t;
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                let ref mut fresh74 = (*emitter).buffer.pointer;
                let fresh75 = *fresh74;
                *fresh74 = (*fresh74).offset(1);
                *fresh75 = ' ' as i32 as yaml_char_t;
                let ref mut fresh76 = (*emitter).column;
                *fresh76 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while string.pointer != string.end {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as libc::c_int & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    let fresh77 = string.pointer;
                    string.pointer = (string.pointer).offset(1);
                    let ref mut fresh78 = (*emitter).buffer.pointer;
                    let fresh79 = *fresh78;
                    *fresh78 = (*fresh78).offset(1);
                    *fresh79 = *fresh77;
                } else {
                    if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh80 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh81 = (*emitter).buffer.pointer;
                        let fresh82 = *fresh81;
                        *fresh81 = (*fresh81).offset(1);
                        *fresh82 = *fresh80;
                        let fresh83 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh84 = (*emitter).buffer.pointer;
                        let fresh85 = *fresh84;
                        *fresh84 = (*fresh84).offset(1);
                        *fresh85 = *fresh83;
                    } else {
                        if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            let fresh86 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh87 = (*emitter).buffer.pointer;
                            let fresh88 = *fresh87;
                            *fresh87 = (*fresh87).offset(1);
                            *fresh88 = *fresh86;
                            let fresh89 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh90 = (*emitter).buffer.pointer;
                            let fresh91 = *fresh90;
                            *fresh90 = (*fresh90).offset(1);
                            *fresh91 = *fresh89;
                            let fresh92 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh93 = (*emitter).buffer.pointer;
                            let fresh94 = *fresh93;
                            *fresh93 = (*fresh93).offset(1);
                            *fresh94 = *fresh92;
                        } else {
                            if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                let fresh95 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh96 = (*emitter).buffer.pointer;
                                let fresh97 = *fresh96;
                                *fresh96 = (*fresh96).offset(1);
                                *fresh97 = *fresh95;
                                let fresh98 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh99 = (*emitter).buffer.pointer;
                                let fresh100 = *fresh99;
                                *fresh99 = (*fresh99).offset(1);
                                *fresh100 = *fresh98;
                                let fresh101 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh102 = (*emitter).buffer.pointer;
                                let fresh103 = *fresh102;
                                *fresh102 = (*fresh102).offset(1);
                                *fresh103 = *fresh101;
                                let fresh104 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh105 = (*emitter).buffer.pointer;
                                let fresh106 = *fresh105;
                                *fresh105 = (*fresh105).offset(1);
                                *fresh106 = *fresh104;
                            } else {};
                        };
                    };
                };
                let ref mut fresh107 = (*emitter).column;
                *fresh107 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    (*emitter).whitespace = is_whitespace;
    (*emitter)
        .indention = ((*emitter).indention != 0 && is_indention != 0) as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_anchor(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    while string.pointer != string.end {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as libc::c_int & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    let fresh108 = string.pointer;
                    string.pointer = (string.pointer).offset(1);
                    let ref mut fresh109 = (*emitter).buffer.pointer;
                    let fresh110 = *fresh109;
                    *fresh109 = (*fresh109).offset(1);
                    *fresh110 = *fresh108;
                } else {
                    if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh111 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh112 = (*emitter).buffer.pointer;
                        let fresh113 = *fresh112;
                        *fresh112 = (*fresh112).offset(1);
                        *fresh113 = *fresh111;
                        let fresh114 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh115 = (*emitter).buffer.pointer;
                        let fresh116 = *fresh115;
                        *fresh115 = (*fresh115).offset(1);
                        *fresh116 = *fresh114;
                    } else {
                        if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            let fresh117 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh118 = (*emitter).buffer.pointer;
                            let fresh119 = *fresh118;
                            *fresh118 = (*fresh118).offset(1);
                            *fresh119 = *fresh117;
                            let fresh120 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh121 = (*emitter).buffer.pointer;
                            let fresh122 = *fresh121;
                            *fresh121 = (*fresh121).offset(1);
                            *fresh122 = *fresh120;
                            let fresh123 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh124 = (*emitter).buffer.pointer;
                            let fresh125 = *fresh124;
                            *fresh124 = (*fresh124).offset(1);
                            *fresh125 = *fresh123;
                        } else {
                            if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                let fresh126 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh127 = (*emitter).buffer.pointer;
                                let fresh128 = *fresh127;
                                *fresh127 = (*fresh127).offset(1);
                                *fresh128 = *fresh126;
                                let fresh129 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh130 = (*emitter).buffer.pointer;
                                let fresh131 = *fresh130;
                                *fresh130 = (*fresh130).offset(1);
                                *fresh131 = *fresh129;
                                let fresh132 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh133 = (*emitter).buffer.pointer;
                                let fresh134 = *fresh133;
                                *fresh133 = (*fresh133).offset(1);
                                *fresh134 = *fresh132;
                                let fresh135 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh136 = (*emitter).buffer.pointer;
                                let fresh137 = *fresh136;
                                *fresh136 = (*fresh136).offset(1);
                                *fresh137 = *fresh135;
                            } else {};
                        };
                    };
                };
                let ref mut fresh138 = (*emitter).column;
                *fresh138 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    (*emitter).whitespace = 0 as libc::c_int;
    (*emitter).indention = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_tag_handle(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if (*emitter).whitespace == 0 {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                let ref mut fresh139 = (*emitter).buffer.pointer;
                let fresh140 = *fresh139;
                *fresh139 = (*fresh139).offset(1);
                *fresh140 = ' ' as i32 as yaml_char_t;
                let ref mut fresh141 = (*emitter).column;
                *fresh141 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while string.pointer != string.end {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as libc::c_int & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    let fresh142 = string.pointer;
                    string.pointer = (string.pointer).offset(1);
                    let ref mut fresh143 = (*emitter).buffer.pointer;
                    let fresh144 = *fresh143;
                    *fresh143 = (*fresh143).offset(1);
                    *fresh144 = *fresh142;
                } else {
                    if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh145 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh146 = (*emitter).buffer.pointer;
                        let fresh147 = *fresh146;
                        *fresh146 = (*fresh146).offset(1);
                        *fresh147 = *fresh145;
                        let fresh148 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh149 = (*emitter).buffer.pointer;
                        let fresh150 = *fresh149;
                        *fresh149 = (*fresh149).offset(1);
                        *fresh150 = *fresh148;
                    } else {
                        if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            let fresh151 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh152 = (*emitter).buffer.pointer;
                            let fresh153 = *fresh152;
                            *fresh152 = (*fresh152).offset(1);
                            *fresh153 = *fresh151;
                            let fresh154 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh155 = (*emitter).buffer.pointer;
                            let fresh156 = *fresh155;
                            *fresh155 = (*fresh155).offset(1);
                            *fresh156 = *fresh154;
                            let fresh157 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh158 = (*emitter).buffer.pointer;
                            let fresh159 = *fresh158;
                            *fresh158 = (*fresh158).offset(1);
                            *fresh159 = *fresh157;
                        } else {
                            if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                let fresh160 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh161 = (*emitter).buffer.pointer;
                                let fresh162 = *fresh161;
                                *fresh161 = (*fresh161).offset(1);
                                *fresh162 = *fresh160;
                                let fresh163 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh164 = (*emitter).buffer.pointer;
                                let fresh165 = *fresh164;
                                *fresh164 = (*fresh164).offset(1);
                                *fresh165 = *fresh163;
                                let fresh166 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh167 = (*emitter).buffer.pointer;
                                let fresh168 = *fresh167;
                                *fresh167 = (*fresh167).offset(1);
                                *fresh168 = *fresh166;
                                let fresh169 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh170 = (*emitter).buffer.pointer;
                                let fresh171 = *fresh170;
                                *fresh170 = (*fresh170).offset(1);
                                *fresh171 = *fresh169;
                            } else {};
                        };
                    };
                };
                let ref mut fresh172 = (*emitter).column;
                *fresh172 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    (*emitter).whitespace = 0 as libc::c_int;
    (*emitter).indention = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_tag_content(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
    mut need_whitespace: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                let ref mut fresh173 = (*emitter).buffer.pointer;
                let fresh174 = *fresh173;
                *fresh173 = (*fresh173).offset(1);
                *fresh174 = ' ' as i32 as yaml_char_t;
                let ref mut fresh175 = (*emitter).column;
                *fresh175 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while string.pointer != string.end {
        if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            >= '0' as i32 as yaml_char_t as libc::c_int
            && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                <= '9' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'Z' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'z' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '_' as i32
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == ';' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '/' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '?' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == ':' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '@' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '&' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '=' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '+' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '$' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '_' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '~' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '*' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\'' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '(' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == ')' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '[' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == ']' as i32 as yaml_char_t as libc::c_int
        {
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh176 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh177 = (*emitter).buffer.pointer;
                        let fresh178 = *fresh177;
                        *fresh177 = (*fresh177).offset(1);
                        *fresh178 = *fresh176;
                    } else {
                        if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            let fresh179 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh180 = (*emitter).buffer.pointer;
                            let fresh181 = *fresh180;
                            *fresh180 = (*fresh180).offset(1);
                            *fresh181 = *fresh179;
                            let fresh182 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh183 = (*emitter).buffer.pointer;
                            let fresh184 = *fresh183;
                            *fresh183 = (*fresh183).offset(1);
                            *fresh184 = *fresh182;
                        } else {
                            if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh185 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh186 = (*emitter).buffer.pointer;
                                let fresh187 = *fresh186;
                                *fresh186 = (*fresh186).offset(1);
                                *fresh187 = *fresh185;
                                let fresh188 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh189 = (*emitter).buffer.pointer;
                                let fresh190 = *fresh189;
                                *fresh189 = (*fresh189).offset(1);
                                *fresh190 = *fresh188;
                                let fresh191 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh192 = (*emitter).buffer.pointer;
                                let fresh193 = *fresh192;
                                *fresh192 = (*fresh192).offset(1);
                                *fresh193 = *fresh191;
                            } else {
                                if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    let fresh194 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh195 = (*emitter).buffer.pointer;
                                    let fresh196 = *fresh195;
                                    *fresh195 = (*fresh195).offset(1);
                                    *fresh196 = *fresh194;
                                    let fresh197 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh198 = (*emitter).buffer.pointer;
                                    let fresh199 = *fresh198;
                                    *fresh198 = (*fresh198).offset(1);
                                    *fresh199 = *fresh197;
                                    let fresh200 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh201 = (*emitter).buffer.pointer;
                                    let fresh202 = *fresh201;
                                    *fresh201 = (*fresh201).offset(1);
                                    *fresh202 = *fresh200;
                                    let fresh203 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh204 = (*emitter).buffer.pointer;
                                    let fresh205 = *fresh204;
                                    *fresh204 = (*fresh204).offset(1);
                                    *fresh205 = *fresh203;
                                } else {};
                            };
                        };
                    };
                    let ref mut fresh206 = (*emitter).column;
                    *fresh206 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
        } else {
            let mut width: libc::c_int = if *(string.pointer)
                .offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                {
                2 as libc::c_int
            } else if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                {
                3 as libc::c_int
            } else if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut value_0: libc::c_uint = 0;
            loop {
                let fresh207 = width;
                width = width - 1;
                if !(fresh207 != 0) {
                    break;
                }
                let fresh208 = string.pointer;
                string.pointer = (string.pointer).offset(1);
                value_0 = *fresh208 as libc::c_uint;
                if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        let ref mut fresh209 = (*emitter).buffer.pointer;
                        let fresh210 = *fresh209;
                        *fresh209 = (*fresh209).offset(1);
                        *fresh210 = '%' as i32 as yaml_char_t;
                        let ref mut fresh211 = (*emitter).column;
                        *fresh211 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
                if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        let ref mut fresh212 = (*emitter).buffer.pointer;
                        let fresh213 = *fresh212;
                        *fresh212 = (*fresh212).offset(1);
                        *fresh213 = (value_0 >> 4 as libc::c_int)
                            .wrapping_add(
                                (if (value_0 >> 4 as libc::c_int)
                                    < 10 as libc::c_int as libc::c_uint
                                {
                                    '0' as i32
                                } else {
                                    'A' as i32 - 10 as libc::c_int
                                }) as libc::c_uint,
                            ) as yaml_char_t;
                        let ref mut fresh214 = (*emitter).column;
                        *fresh214 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
                if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        let ref mut fresh215 = (*emitter).buffer.pointer;
                        let fresh216 = *fresh215;
                        *fresh215 = (*fresh215).offset(1);
                        *fresh216 = (value_0 & 0xf as libc::c_int as libc::c_uint)
                            .wrapping_add(
                                (if (value_0 & 0xf as libc::c_int as libc::c_uint)
                                    < 10 as libc::c_int as libc::c_uint
                                {
                                    '0' as i32
                                } else {
                                    'A' as i32 - 10 as libc::c_int
                                }) as libc::c_uint,
                            ) as yaml_char_t;
                        let ref mut fresh217 = (*emitter).column;
                        *fresh217 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
            }
        }
    }
    (*emitter).whitespace = 0 as libc::c_int;
    (*emitter).indention = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_plain_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
    mut allow_breaks: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    let mut spaces: libc::c_int = 0 as libc::c_int;
    let mut breaks: libc::c_int = 0 as libc::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if (*emitter).whitespace == 0 && (length != 0 || (*emitter).flow_level != 0) {
        if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
            && {
                let ref mut fresh218 = (*emitter).buffer.pointer;
                let fresh219 = *fresh218;
                *fresh218 = (*fresh218).offset(1);
                *fresh219 = ' ' as i32 as yaml_char_t;
                let ref mut fresh220 = (*emitter).column;
                *fresh220 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while string.pointer != string.end {
        if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if allow_breaks != 0 && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && !(*(string.pointer).offset(1 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                string
                    .pointer = (string.pointer)
                    .offset(
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                            2 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                            3 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                            {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    );
            } else if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        if *string.pointer as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            let fresh221 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh222 = (*emitter).buffer.pointer;
                            let fresh223 = *fresh222;
                            *fresh222 = (*fresh222).offset(1);
                            *fresh223 = *fresh221;
                        } else {
                            if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                let fresh224 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh225 = (*emitter).buffer.pointer;
                                let fresh226 = *fresh225;
                                *fresh225 = (*fresh225).offset(1);
                                *fresh226 = *fresh224;
                                let fresh227 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh228 = (*emitter).buffer.pointer;
                                let fresh229 = *fresh228;
                                *fresh228 = (*fresh228).offset(1);
                                *fresh229 = *fresh227;
                            } else {
                                if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                    == 0xe0 as libc::c_int
                                {
                                    let fresh230 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh231 = (*emitter).buffer.pointer;
                                    let fresh232 = *fresh231;
                                    *fresh231 = (*fresh231).offset(1);
                                    *fresh232 = *fresh230;
                                    let fresh233 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh234 = (*emitter).buffer.pointer;
                                    let fresh235 = *fresh234;
                                    *fresh234 = (*fresh234).offset(1);
                                    *fresh235 = *fresh233;
                                    let fresh236 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh237 = (*emitter).buffer.pointer;
                                    let fresh238 = *fresh237;
                                    *fresh237 = (*fresh237).offset(1);
                                    *fresh238 = *fresh236;
                                } else {
                                    if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                        == 0xf0 as libc::c_int
                                    {
                                        let fresh239 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh240 = (*emitter).buffer.pointer;
                                        let fresh241 = *fresh240;
                                        *fresh240 = (*fresh240).offset(1);
                                        *fresh241 = *fresh239;
                                        let fresh242 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh243 = (*emitter).buffer.pointer;
                                        let fresh244 = *fresh243;
                                        *fresh243 = (*fresh243).offset(1);
                                        *fresh244 = *fresh242;
                                        let fresh245 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh246 = (*emitter).buffer.pointer;
                                        let fresh247 = *fresh246;
                                        *fresh246 = (*fresh246).offset(1);
                                        *fresh247 = *fresh245;
                                        let fresh248 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh249 = (*emitter).buffer.pointer;
                                        let fresh250 = *fresh249;
                                        *fresh249 = (*fresh249).offset(1);
                                        *fresh250 = *fresh248;
                                    } else {};
                                };
                            };
                        };
                        let ref mut fresh251 = (*emitter).column;
                        *fresh251 += 1;
                        1 as libc::c_int != 0
                    })
                {
                return 0 as libc::c_int
            }
            spaces = 1 as libc::c_int;
        } else if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -87i32 as yaml_char_t as libc::c_int
            {
            if breaks == 0
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        if (*emitter).line_break as libc::c_uint
                            == YAML_CR_BREAK as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh252 = (*emitter).buffer.pointer;
                            let fresh253 = *fresh252;
                            *fresh252 = (*fresh252).offset(1);
                            *fresh253 = '\r' as i32 as yaml_char_t;
                        } else {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh254 = (*emitter).buffer.pointer;
                                let fresh255 = *fresh254;
                                *fresh254 = (*fresh254).offset(1);
                                *fresh255 = '\n' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as libc::c_uint
                                    == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                                {
                                    let ref mut fresh256 = (*emitter).buffer.pointer;
                                    let fresh257 = *fresh256;
                                    *fresh256 = (*fresh256).offset(1);
                                    *fresh257 = '\r' as i32 as yaml_char_t;
                                    let ref mut fresh258 = (*emitter).buffer.pointer;
                                    let fresh259 = *fresh258;
                                    *fresh258 = (*fresh258).offset(1);
                                    *fresh259 = '\n' as i32 as yaml_char_t;
                                } else {};
                            };
                        };
                        (*emitter).column = 0 as libc::c_int;
                        let ref mut fresh260 = (*emitter).line;
                        *fresh260 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
            }
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    ((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh261 = (*emitter).buffer.pointer;
                                let fresh262 = *fresh261;
                                *fresh261 = (*fresh261).offset(1);
                                *fresh262 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as libc::c_uint
                                    == YAML_LN_BREAK as libc::c_int as libc::c_uint
                                {
                                    let ref mut fresh263 = (*emitter).buffer.pointer;
                                    let fresh264 = *fresh263;
                                    *fresh263 = (*fresh263).offset(1);
                                    *fresh264 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as libc::c_uint
                                        == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                                    {
                                        let ref mut fresh265 = (*emitter).buffer.pointer;
                                        let fresh266 = *fresh265;
                                        *fresh265 = (*fresh265).offset(1);
                                        *fresh266 = '\r' as i32 as yaml_char_t;
                                        let ref mut fresh267 = (*emitter).buffer.pointer;
                                        let fresh268 = *fresh267;
                                        *fresh267 = (*fresh267).offset(1);
                                        *fresh268 = '\n' as i32 as yaml_char_t;
                                    } else {};
                                };
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let ref mut fresh269 = (*emitter).line;
                            *fresh269 += 1;
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    string.pointer = (string.pointer).offset(1);
                    1 as libc::c_int
                } else {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh270 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh271 = (*emitter).buffer.pointer;
                        let fresh272 = *fresh271;
                        *fresh271 = (*fresh271).offset(1);
                        *fresh272 = *fresh270;
                    } else {
                        if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            let fresh273 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh274 = (*emitter).buffer.pointer;
                            let fresh275 = *fresh274;
                            *fresh274 = (*fresh274).offset(1);
                            *fresh275 = *fresh273;
                            let fresh276 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh277 = (*emitter).buffer.pointer;
                            let fresh278 = *fresh277;
                            *fresh277 = (*fresh277).offset(1);
                            *fresh278 = *fresh276;
                        } else {
                            if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh279 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh280 = (*emitter).buffer.pointer;
                                let fresh281 = *fresh280;
                                *fresh280 = (*fresh280).offset(1);
                                *fresh281 = *fresh279;
                                let fresh282 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh283 = (*emitter).buffer.pointer;
                                let fresh284 = *fresh283;
                                *fresh283 = (*fresh283).offset(1);
                                *fresh284 = *fresh282;
                                let fresh285 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh286 = (*emitter).buffer.pointer;
                                let fresh287 = *fresh286;
                                *fresh286 = (*fresh286).offset(1);
                                *fresh287 = *fresh285;
                            } else {
                                if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    let fresh288 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh289 = (*emitter).buffer.pointer;
                                    let fresh290 = *fresh289;
                                    *fresh289 = (*fresh289).offset(1);
                                    *fresh290 = *fresh288;
                                    let fresh291 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh292 = (*emitter).buffer.pointer;
                                    let fresh293 = *fresh292;
                                    *fresh292 = (*fresh292).offset(1);
                                    *fresh293 = *fresh291;
                                    let fresh294 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh295 = (*emitter).buffer.pointer;
                                    let fresh296 = *fresh295;
                                    *fresh295 = (*fresh295).offset(1);
                                    *fresh296 = *fresh294;
                                    let fresh297 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh298 = (*emitter).buffer.pointer;
                                    let fresh299 = *fresh298;
                                    *fresh298 = (*fresh298).offset(1);
                                    *fresh299 = *fresh297;
                                } else {};
                            };
                        };
                    };
                    (*emitter).column = 0 as libc::c_int;
                    let ref mut fresh300 = (*emitter).line;
                    *fresh300 += 1;
                    1 as libc::c_int
                }) != 0)
            {
                return 0 as libc::c_int;
            }
            (*emitter).indention = 1 as libc::c_int;
            breaks = 1 as libc::c_int;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
            }
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh301 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh302 = (*emitter).buffer.pointer;
                        let fresh303 = *fresh302;
                        *fresh302 = (*fresh302).offset(1);
                        *fresh303 = *fresh301;
                    } else {
                        if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            let fresh304 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh305 = (*emitter).buffer.pointer;
                            let fresh306 = *fresh305;
                            *fresh305 = (*fresh305).offset(1);
                            *fresh306 = *fresh304;
                            let fresh307 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh308 = (*emitter).buffer.pointer;
                            let fresh309 = *fresh308;
                            *fresh308 = (*fresh308).offset(1);
                            *fresh309 = *fresh307;
                        } else {
                            if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh310 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh311 = (*emitter).buffer.pointer;
                                let fresh312 = *fresh311;
                                *fresh311 = (*fresh311).offset(1);
                                *fresh312 = *fresh310;
                                let fresh313 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh314 = (*emitter).buffer.pointer;
                                let fresh315 = *fresh314;
                                *fresh314 = (*fresh314).offset(1);
                                *fresh315 = *fresh313;
                                let fresh316 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh317 = (*emitter).buffer.pointer;
                                let fresh318 = *fresh317;
                                *fresh317 = (*fresh317).offset(1);
                                *fresh318 = *fresh316;
                            } else {
                                if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    let fresh319 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh320 = (*emitter).buffer.pointer;
                                    let fresh321 = *fresh320;
                                    *fresh320 = (*fresh320).offset(1);
                                    *fresh321 = *fresh319;
                                    let fresh322 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh323 = (*emitter).buffer.pointer;
                                    let fresh324 = *fresh323;
                                    *fresh323 = (*fresh323).offset(1);
                                    *fresh324 = *fresh322;
                                    let fresh325 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh326 = (*emitter).buffer.pointer;
                                    let fresh327 = *fresh326;
                                    *fresh326 = (*fresh326).offset(1);
                                    *fresh327 = *fresh325;
                                    let fresh328 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh329 = (*emitter).buffer.pointer;
                                    let fresh330 = *fresh329;
                                    *fresh329 = (*fresh329).offset(1);
                                    *fresh330 = *fresh328;
                                } else {};
                            };
                        };
                    };
                    let ref mut fresh331 = (*emitter).column;
                    *fresh331 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            (*emitter).indention = 0 as libc::c_int;
            spaces = 0 as libc::c_int;
            breaks = 0 as libc::c_int;
        }
    }
    (*emitter).whitespace = 0 as libc::c_int;
    (*emitter).indention = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_single_quoted_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
    mut allow_breaks: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    let mut spaces: libc::c_int = 0 as libc::c_int;
    let mut breaks: libc::c_int = 0 as libc::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if yaml_emitter_write_indicator(
        emitter,
        b"'\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    while string.pointer != string.end {
        if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if allow_breaks != 0 && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != (string.end).offset(-(1 as libc::c_int as isize))
                && !(*(string.pointer).offset(1 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                string
                    .pointer = (string.pointer)
                    .offset(
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                            2 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                            3 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                            {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    );
            } else if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        if *string.pointer as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            let fresh332 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh333 = (*emitter).buffer.pointer;
                            let fresh334 = *fresh333;
                            *fresh333 = (*fresh333).offset(1);
                            *fresh334 = *fresh332;
                        } else {
                            if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                let fresh335 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh336 = (*emitter).buffer.pointer;
                                let fresh337 = *fresh336;
                                *fresh336 = (*fresh336).offset(1);
                                *fresh337 = *fresh335;
                                let fresh338 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh339 = (*emitter).buffer.pointer;
                                let fresh340 = *fresh339;
                                *fresh339 = (*fresh339).offset(1);
                                *fresh340 = *fresh338;
                            } else {
                                if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                    == 0xe0 as libc::c_int
                                {
                                    let fresh341 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh342 = (*emitter).buffer.pointer;
                                    let fresh343 = *fresh342;
                                    *fresh342 = (*fresh342).offset(1);
                                    *fresh343 = *fresh341;
                                    let fresh344 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh345 = (*emitter).buffer.pointer;
                                    let fresh346 = *fresh345;
                                    *fresh345 = (*fresh345).offset(1);
                                    *fresh346 = *fresh344;
                                    let fresh347 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh348 = (*emitter).buffer.pointer;
                                    let fresh349 = *fresh348;
                                    *fresh348 = (*fresh348).offset(1);
                                    *fresh349 = *fresh347;
                                } else {
                                    if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                        == 0xf0 as libc::c_int
                                    {
                                        let fresh350 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh351 = (*emitter).buffer.pointer;
                                        let fresh352 = *fresh351;
                                        *fresh351 = (*fresh351).offset(1);
                                        *fresh352 = *fresh350;
                                        let fresh353 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh354 = (*emitter).buffer.pointer;
                                        let fresh355 = *fresh354;
                                        *fresh354 = (*fresh354).offset(1);
                                        *fresh355 = *fresh353;
                                        let fresh356 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh357 = (*emitter).buffer.pointer;
                                        let fresh358 = *fresh357;
                                        *fresh357 = (*fresh357).offset(1);
                                        *fresh358 = *fresh356;
                                        let fresh359 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh360 = (*emitter).buffer.pointer;
                                        let fresh361 = *fresh360;
                                        *fresh360 = (*fresh360).offset(1);
                                        *fresh361 = *fresh359;
                                    } else {};
                                };
                            };
                        };
                        let ref mut fresh362 = (*emitter).column;
                        *fresh362 += 1;
                        1 as libc::c_int != 0
                    })
                {
                return 0 as libc::c_int
            }
            spaces = 1 as libc::c_int;
        } else if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -87i32 as yaml_char_t as libc::c_int
            {
            if breaks == 0
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        if (*emitter).line_break as libc::c_uint
                            == YAML_CR_BREAK as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh363 = (*emitter).buffer.pointer;
                            let fresh364 = *fresh363;
                            *fresh363 = (*fresh363).offset(1);
                            *fresh364 = '\r' as i32 as yaml_char_t;
                        } else {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh365 = (*emitter).buffer.pointer;
                                let fresh366 = *fresh365;
                                *fresh365 = (*fresh365).offset(1);
                                *fresh366 = '\n' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as libc::c_uint
                                    == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                                {
                                    let ref mut fresh367 = (*emitter).buffer.pointer;
                                    let fresh368 = *fresh367;
                                    *fresh367 = (*fresh367).offset(1);
                                    *fresh368 = '\r' as i32 as yaml_char_t;
                                    let ref mut fresh369 = (*emitter).buffer.pointer;
                                    let fresh370 = *fresh369;
                                    *fresh369 = (*fresh369).offset(1);
                                    *fresh370 = '\n' as i32 as yaml_char_t;
                                } else {};
                            };
                        };
                        (*emitter).column = 0 as libc::c_int;
                        let ref mut fresh371 = (*emitter).line;
                        *fresh371 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
            }
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    ((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh372 = (*emitter).buffer.pointer;
                                let fresh373 = *fresh372;
                                *fresh372 = (*fresh372).offset(1);
                                *fresh373 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as libc::c_uint
                                    == YAML_LN_BREAK as libc::c_int as libc::c_uint
                                {
                                    let ref mut fresh374 = (*emitter).buffer.pointer;
                                    let fresh375 = *fresh374;
                                    *fresh374 = (*fresh374).offset(1);
                                    *fresh375 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as libc::c_uint
                                        == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                                    {
                                        let ref mut fresh376 = (*emitter).buffer.pointer;
                                        let fresh377 = *fresh376;
                                        *fresh376 = (*fresh376).offset(1);
                                        *fresh377 = '\r' as i32 as yaml_char_t;
                                        let ref mut fresh378 = (*emitter).buffer.pointer;
                                        let fresh379 = *fresh378;
                                        *fresh378 = (*fresh378).offset(1);
                                        *fresh379 = '\n' as i32 as yaml_char_t;
                                    } else {};
                                };
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let ref mut fresh380 = (*emitter).line;
                            *fresh380 += 1;
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    string.pointer = (string.pointer).offset(1);
                    1 as libc::c_int
                } else {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh381 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh382 = (*emitter).buffer.pointer;
                        let fresh383 = *fresh382;
                        *fresh382 = (*fresh382).offset(1);
                        *fresh383 = *fresh381;
                    } else {
                        if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            let fresh384 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh385 = (*emitter).buffer.pointer;
                            let fresh386 = *fresh385;
                            *fresh385 = (*fresh385).offset(1);
                            *fresh386 = *fresh384;
                            let fresh387 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh388 = (*emitter).buffer.pointer;
                            let fresh389 = *fresh388;
                            *fresh388 = (*fresh388).offset(1);
                            *fresh389 = *fresh387;
                        } else {
                            if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh390 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh391 = (*emitter).buffer.pointer;
                                let fresh392 = *fresh391;
                                *fresh391 = (*fresh391).offset(1);
                                *fresh392 = *fresh390;
                                let fresh393 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh394 = (*emitter).buffer.pointer;
                                let fresh395 = *fresh394;
                                *fresh394 = (*fresh394).offset(1);
                                *fresh395 = *fresh393;
                                let fresh396 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh397 = (*emitter).buffer.pointer;
                                let fresh398 = *fresh397;
                                *fresh397 = (*fresh397).offset(1);
                                *fresh398 = *fresh396;
                            } else {
                                if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    let fresh399 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh400 = (*emitter).buffer.pointer;
                                    let fresh401 = *fresh400;
                                    *fresh400 = (*fresh400).offset(1);
                                    *fresh401 = *fresh399;
                                    let fresh402 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh403 = (*emitter).buffer.pointer;
                                    let fresh404 = *fresh403;
                                    *fresh403 = (*fresh403).offset(1);
                                    *fresh404 = *fresh402;
                                    let fresh405 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh406 = (*emitter).buffer.pointer;
                                    let fresh407 = *fresh406;
                                    *fresh406 = (*fresh406).offset(1);
                                    *fresh407 = *fresh405;
                                    let fresh408 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh409 = (*emitter).buffer.pointer;
                                    let fresh410 = *fresh409;
                                    *fresh409 = (*fresh409).offset(1);
                                    *fresh410 = *fresh408;
                                } else {};
                            };
                        };
                    };
                    (*emitter).column = 0 as libc::c_int;
                    let ref mut fresh411 = (*emitter).line;
                    *fresh411 += 1;
                    1 as libc::c_int
                }) != 0)
            {
                return 0 as libc::c_int;
            }
            (*emitter).indention = 1 as libc::c_int;
            breaks = 1 as libc::c_int;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
            }
            if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\'' as i32 as yaml_char_t as libc::c_int
            {
                if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        let ref mut fresh412 = (*emitter).buffer.pointer;
                        let fresh413 = *fresh412;
                        *fresh412 = (*fresh412).offset(1);
                        *fresh413 = '\'' as i32 as yaml_char_t;
                        let ref mut fresh414 = (*emitter).column;
                        *fresh414 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
            }
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh415 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh416 = (*emitter).buffer.pointer;
                        let fresh417 = *fresh416;
                        *fresh416 = (*fresh416).offset(1);
                        *fresh417 = *fresh415;
                    } else {
                        if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            let fresh418 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh419 = (*emitter).buffer.pointer;
                            let fresh420 = *fresh419;
                            *fresh419 = (*fresh419).offset(1);
                            *fresh420 = *fresh418;
                            let fresh421 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh422 = (*emitter).buffer.pointer;
                            let fresh423 = *fresh422;
                            *fresh422 = (*fresh422).offset(1);
                            *fresh423 = *fresh421;
                        } else {
                            if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh424 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh425 = (*emitter).buffer.pointer;
                                let fresh426 = *fresh425;
                                *fresh425 = (*fresh425).offset(1);
                                *fresh426 = *fresh424;
                                let fresh427 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh428 = (*emitter).buffer.pointer;
                                let fresh429 = *fresh428;
                                *fresh428 = (*fresh428).offset(1);
                                *fresh429 = *fresh427;
                                let fresh430 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh431 = (*emitter).buffer.pointer;
                                let fresh432 = *fresh431;
                                *fresh431 = (*fresh431).offset(1);
                                *fresh432 = *fresh430;
                            } else {
                                if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    let fresh433 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh434 = (*emitter).buffer.pointer;
                                    let fresh435 = *fresh434;
                                    *fresh434 = (*fresh434).offset(1);
                                    *fresh435 = *fresh433;
                                    let fresh436 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh437 = (*emitter).buffer.pointer;
                                    let fresh438 = *fresh437;
                                    *fresh437 = (*fresh437).offset(1);
                                    *fresh438 = *fresh436;
                                    let fresh439 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh440 = (*emitter).buffer.pointer;
                                    let fresh441 = *fresh440;
                                    *fresh440 = (*fresh440).offset(1);
                                    *fresh441 = *fresh439;
                                    let fresh442 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh443 = (*emitter).buffer.pointer;
                                    let fresh444 = *fresh443;
                                    *fresh443 = (*fresh443).offset(1);
                                    *fresh444 = *fresh442;
                                } else {};
                            };
                        };
                    };
                    let ref mut fresh445 = (*emitter).column;
                    *fresh445 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            (*emitter).indention = 0 as libc::c_int;
            spaces = 0 as libc::c_int;
            breaks = 0 as libc::c_int;
        }
    }
    if breaks != 0 {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as libc::c_int;
        }
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"'\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    (*emitter).whitespace = 0 as libc::c_int;
    (*emitter).indention = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_double_quoted_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
    mut allow_breaks: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    let mut spaces: libc::c_int = 0 as libc::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if yaml_emitter_write_indicator(
        emitter,
        b"\"\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    while string.pointer != string.end {
        if !(*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == 0xa as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                >= 0x20 as libc::c_int
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    <= 0x7e as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == 0xc2 as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int >= 0xa0 as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                > 0xc2 as libc::c_int
                && (*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int)
                    < 0xed as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == 0xed as libc::c_int
                && (*(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int) < 0xa0 as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == 0xee as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == 0xef as libc::c_int
                && !(*(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == 0xbb as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == 0xbf as libc::c_int)
                && !(*(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == 0xbf as libc::c_int
                    && (*(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == 0xbe as libc::c_int
                        || *(string.pointer)
                            .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int == 0xbf as libc::c_int)))
            || (*emitter).unicode == 0
                && !(*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    <= '\u{7f}' as i32 as yaml_char_t as libc::c_int)
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -17i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -69i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -65i32 as yaml_char_t as libc::c_int
            || (*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -87i32 as yaml_char_t as libc::c_int)
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '"' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\\' as i32 as yaml_char_t as libc::c_int
        {
            let mut octet: libc::c_uchar = 0;
            let mut width: libc::c_uint = 0;
            let mut value_0: libc::c_uint = 0;
            let mut k: libc::c_int = 0;
            octet = *(string.pointer).offset(0 as libc::c_int as isize);
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
            value_0 = (if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
            {
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
            k = 1 as libc::c_int;
            while k < width as libc::c_int {
                octet = *(string.pointer).offset(k as isize);
                value_0 = (value_0 << 6 as libc::c_int)
                    .wrapping_add(
                        (octet as libc::c_int & 0x3f as libc::c_int) as libc::c_uint,
                    );
                k += 1;
            }
            string.pointer = (string.pointer).offset(width as isize);
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && {
                    let ref mut fresh446 = (*emitter).buffer.pointer;
                    let fresh447 = *fresh446;
                    *fresh446 = (*fresh446).offset(1);
                    *fresh447 = '\\' as i32 as yaml_char_t;
                    let ref mut fresh448 = (*emitter).column;
                    *fresh448 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            match value_0 {
                0 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh449 = (*emitter).buffer.pointer;
                            let fresh450 = *fresh449;
                            *fresh449 = (*fresh449).offset(1);
                            *fresh450 = '0' as i32 as yaml_char_t;
                            let ref mut fresh451 = (*emitter).column;
                            *fresh451 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                7 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh452 = (*emitter).buffer.pointer;
                            let fresh453 = *fresh452;
                            *fresh452 = (*fresh452).offset(1);
                            *fresh453 = 'a' as i32 as yaml_char_t;
                            let ref mut fresh454 = (*emitter).column;
                            *fresh454 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                8 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh455 = (*emitter).buffer.pointer;
                            let fresh456 = *fresh455;
                            *fresh455 = (*fresh455).offset(1);
                            *fresh456 = 'b' as i32 as yaml_char_t;
                            let ref mut fresh457 = (*emitter).column;
                            *fresh457 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                9 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh458 = (*emitter).buffer.pointer;
                            let fresh459 = *fresh458;
                            *fresh458 = (*fresh458).offset(1);
                            *fresh459 = 't' as i32 as yaml_char_t;
                            let ref mut fresh460 = (*emitter).column;
                            *fresh460 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                10 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh461 = (*emitter).buffer.pointer;
                            let fresh462 = *fresh461;
                            *fresh461 = (*fresh461).offset(1);
                            *fresh462 = 'n' as i32 as yaml_char_t;
                            let ref mut fresh463 = (*emitter).column;
                            *fresh463 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                11 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh464 = (*emitter).buffer.pointer;
                            let fresh465 = *fresh464;
                            *fresh464 = (*fresh464).offset(1);
                            *fresh465 = 'v' as i32 as yaml_char_t;
                            let ref mut fresh466 = (*emitter).column;
                            *fresh466 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                12 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh467 = (*emitter).buffer.pointer;
                            let fresh468 = *fresh467;
                            *fresh467 = (*fresh467).offset(1);
                            *fresh468 = 'f' as i32 as yaml_char_t;
                            let ref mut fresh469 = (*emitter).column;
                            *fresh469 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                13 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh470 = (*emitter).buffer.pointer;
                            let fresh471 = *fresh470;
                            *fresh470 = (*fresh470).offset(1);
                            *fresh471 = 'r' as i32 as yaml_char_t;
                            let ref mut fresh472 = (*emitter).column;
                            *fresh472 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                27 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh473 = (*emitter).buffer.pointer;
                            let fresh474 = *fresh473;
                            *fresh473 = (*fresh473).offset(1);
                            *fresh474 = 'e' as i32 as yaml_char_t;
                            let ref mut fresh475 = (*emitter).column;
                            *fresh475 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                34 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh476 = (*emitter).buffer.pointer;
                            let fresh477 = *fresh476;
                            *fresh476 = (*fresh476).offset(1);
                            *fresh477 = '"' as i32 as yaml_char_t;
                            let ref mut fresh478 = (*emitter).column;
                            *fresh478 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                92 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh479 = (*emitter).buffer.pointer;
                            let fresh480 = *fresh479;
                            *fresh479 = (*fresh479).offset(1);
                            *fresh480 = '\\' as i32 as yaml_char_t;
                            let ref mut fresh481 = (*emitter).column;
                            *fresh481 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                133 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh482 = (*emitter).buffer.pointer;
                            let fresh483 = *fresh482;
                            *fresh482 = (*fresh482).offset(1);
                            *fresh483 = 'N' as i32 as yaml_char_t;
                            let ref mut fresh484 = (*emitter).column;
                            *fresh484 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                160 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh485 = (*emitter).buffer.pointer;
                            let fresh486 = *fresh485;
                            *fresh485 = (*fresh485).offset(1);
                            *fresh486 = '_' as i32 as yaml_char_t;
                            let ref mut fresh487 = (*emitter).column;
                            *fresh487 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                8232 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh488 = (*emitter).buffer.pointer;
                            let fresh489 = *fresh488;
                            *fresh488 = (*fresh488).offset(1);
                            *fresh489 = 'L' as i32 as yaml_char_t;
                            let ref mut fresh490 = (*emitter).column;
                            *fresh490 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                8233 => {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh491 = (*emitter).buffer.pointer;
                            let fresh492 = *fresh491;
                            *fresh491 = (*fresh491).offset(1);
                            *fresh492 = 'P' as i32 as yaml_char_t;
                            let ref mut fresh493 = (*emitter).column;
                            *fresh493 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                _ => {
                    if value_0 <= 0xff as libc::c_int as libc::c_uint {
                        if !((((*emitter).buffer.pointer)
                            .offset(5 as libc::c_int as isize) < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let ref mut fresh494 = (*emitter).buffer.pointer;
                                let fresh495 = *fresh494;
                                *fresh494 = (*fresh494).offset(1);
                                *fresh495 = 'x' as i32 as yaml_char_t;
                                let ref mut fresh496 = (*emitter).column;
                                *fresh496 += 1;
                                1 as libc::c_int != 0
                            })
                        {
                            return 0 as libc::c_int;
                        }
                        width = 2 as libc::c_int as libc::c_uint;
                    } else if value_0 <= 0xffff as libc::c_int as libc::c_uint {
                        if !((((*emitter).buffer.pointer)
                            .offset(5 as libc::c_int as isize) < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let ref mut fresh497 = (*emitter).buffer.pointer;
                                let fresh498 = *fresh497;
                                *fresh497 = (*fresh497).offset(1);
                                *fresh498 = 'u' as i32 as yaml_char_t;
                                let ref mut fresh499 = (*emitter).column;
                                *fresh499 += 1;
                                1 as libc::c_int != 0
                            })
                        {
                            return 0 as libc::c_int;
                        }
                        width = 4 as libc::c_int as libc::c_uint;
                    } else {
                        if !((((*emitter).buffer.pointer)
                            .offset(5 as libc::c_int as isize) < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let ref mut fresh500 = (*emitter).buffer.pointer;
                                let fresh501 = *fresh500;
                                *fresh500 = (*fresh500).offset(1);
                                *fresh501 = 'U' as i32 as yaml_char_t;
                                let ref mut fresh502 = (*emitter).column;
                                *fresh502 += 1;
                                1 as libc::c_int != 0
                            })
                        {
                            return 0 as libc::c_int;
                        }
                        width = 8 as libc::c_int as libc::c_uint;
                    }
                    k = width
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        .wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_int;
                    while k >= 0 as libc::c_int {
                        let mut digit: libc::c_int = (value_0 >> k
                            & 0xf as libc::c_int as libc::c_uint) as libc::c_int;
                        if !((((*emitter).buffer.pointer)
                            .offset(5 as libc::c_int as isize) < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let ref mut fresh503 = (*emitter).buffer.pointer;
                                let fresh504 = *fresh503;
                                *fresh503 = (*fresh503).offset(1);
                                *fresh504 = (digit
                                    + (if digit < 10 as libc::c_int {
                                        '0' as i32
                                    } else {
                                        'A' as i32 - 10 as libc::c_int
                                    })) as yaml_char_t;
                                let ref mut fresh505 = (*emitter).column;
                                *fresh505 += 1;
                                1 as libc::c_int != 0
                            })
                        {
                            return 0 as libc::c_int;
                        }
                        k -= 4 as libc::c_int;
                    }
                }
            }
            spaces = 0 as libc::c_int;
        } else if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
            {
            if allow_breaks != 0 && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != (string.end).offset(-(1 as libc::c_int as isize))
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                if *(string.pointer).offset(1 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            let ref mut fresh506 = (*emitter).buffer.pointer;
                            let fresh507 = *fresh506;
                            *fresh506 = (*fresh506).offset(1);
                            *fresh507 = '\\' as i32 as yaml_char_t;
                            let ref mut fresh508 = (*emitter).column;
                            *fresh508 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                string
                    .pointer = (string.pointer)
                    .offset(
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                            2 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                            3 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                            {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    );
            } else if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        if *string.pointer as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            let fresh509 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh510 = (*emitter).buffer.pointer;
                            let fresh511 = *fresh510;
                            *fresh510 = (*fresh510).offset(1);
                            *fresh511 = *fresh509;
                        } else {
                            if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                let fresh512 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh513 = (*emitter).buffer.pointer;
                                let fresh514 = *fresh513;
                                *fresh513 = (*fresh513).offset(1);
                                *fresh514 = *fresh512;
                                let fresh515 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh516 = (*emitter).buffer.pointer;
                                let fresh517 = *fresh516;
                                *fresh516 = (*fresh516).offset(1);
                                *fresh517 = *fresh515;
                            } else {
                                if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                    == 0xe0 as libc::c_int
                                {
                                    let fresh518 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh519 = (*emitter).buffer.pointer;
                                    let fresh520 = *fresh519;
                                    *fresh519 = (*fresh519).offset(1);
                                    *fresh520 = *fresh518;
                                    let fresh521 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh522 = (*emitter).buffer.pointer;
                                    let fresh523 = *fresh522;
                                    *fresh522 = (*fresh522).offset(1);
                                    *fresh523 = *fresh521;
                                    let fresh524 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh525 = (*emitter).buffer.pointer;
                                    let fresh526 = *fresh525;
                                    *fresh525 = (*fresh525).offset(1);
                                    *fresh526 = *fresh524;
                                } else {
                                    if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                        == 0xf0 as libc::c_int
                                    {
                                        let fresh527 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh528 = (*emitter).buffer.pointer;
                                        let fresh529 = *fresh528;
                                        *fresh528 = (*fresh528).offset(1);
                                        *fresh529 = *fresh527;
                                        let fresh530 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh531 = (*emitter).buffer.pointer;
                                        let fresh532 = *fresh531;
                                        *fresh531 = (*fresh531).offset(1);
                                        *fresh532 = *fresh530;
                                        let fresh533 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh534 = (*emitter).buffer.pointer;
                                        let fresh535 = *fresh534;
                                        *fresh534 = (*fresh534).offset(1);
                                        *fresh535 = *fresh533;
                                        let fresh536 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh537 = (*emitter).buffer.pointer;
                                        let fresh538 = *fresh537;
                                        *fresh537 = (*fresh537).offset(1);
                                        *fresh538 = *fresh536;
                                    } else {};
                                };
                            };
                        };
                        let ref mut fresh539 = (*emitter).column;
                        *fresh539 += 1;
                        1 as libc::c_int != 0
                    })
                {
                return 0 as libc::c_int
            }
            spaces = 1 as libc::c_int;
        } else {
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh540 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh541 = (*emitter).buffer.pointer;
                        let fresh542 = *fresh541;
                        *fresh541 = (*fresh541).offset(1);
                        *fresh542 = *fresh540;
                    } else {
                        if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            let fresh543 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh544 = (*emitter).buffer.pointer;
                            let fresh545 = *fresh544;
                            *fresh544 = (*fresh544).offset(1);
                            *fresh545 = *fresh543;
                            let fresh546 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh547 = (*emitter).buffer.pointer;
                            let fresh548 = *fresh547;
                            *fresh547 = (*fresh547).offset(1);
                            *fresh548 = *fresh546;
                        } else {
                            if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh549 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh550 = (*emitter).buffer.pointer;
                                let fresh551 = *fresh550;
                                *fresh550 = (*fresh550).offset(1);
                                *fresh551 = *fresh549;
                                let fresh552 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh553 = (*emitter).buffer.pointer;
                                let fresh554 = *fresh553;
                                *fresh553 = (*fresh553).offset(1);
                                *fresh554 = *fresh552;
                                let fresh555 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh556 = (*emitter).buffer.pointer;
                                let fresh557 = *fresh556;
                                *fresh556 = (*fresh556).offset(1);
                                *fresh557 = *fresh555;
                            } else {
                                if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    let fresh558 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh559 = (*emitter).buffer.pointer;
                                    let fresh560 = *fresh559;
                                    *fresh559 = (*fresh559).offset(1);
                                    *fresh560 = *fresh558;
                                    let fresh561 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh562 = (*emitter).buffer.pointer;
                                    let fresh563 = *fresh562;
                                    *fresh562 = (*fresh562).offset(1);
                                    *fresh563 = *fresh561;
                                    let fresh564 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh565 = (*emitter).buffer.pointer;
                                    let fresh566 = *fresh565;
                                    *fresh565 = (*fresh565).offset(1);
                                    *fresh566 = *fresh564;
                                    let fresh567 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh568 = (*emitter).buffer.pointer;
                                    let fresh569 = *fresh568;
                                    *fresh568 = (*fresh568).offset(1);
                                    *fresh569 = *fresh567;
                                } else {};
                            };
                        };
                    };
                    let ref mut fresh570 = (*emitter).column;
                    *fresh570 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            spaces = 0 as libc::c_int;
        }
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"\"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    (*emitter).whitespace = 0 as libc::c_int;
    (*emitter).indention = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_block_scalar_hints(
    mut emitter: *mut yaml_emitter_t,
    mut string: yaml_string_t,
) -> libc::c_int {
    let mut indent_hint: [libc::c_char; 2] = [0; 2];
    let mut chomp_hint: *const libc::c_char = 0 as *const libc::c_char;
    if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
        == ' ' as i32 as yaml_char_t as libc::c_int
        || (*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -87i32 as yaml_char_t as libc::c_int)
    {
        indent_hint[0 as libc::c_int
            as usize] = ('0' as i32
            + (*emitter).best_indent as libc::c_char as libc::c_int) as libc::c_char;
        indent_hint[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        if yaml_emitter_write_indicator(
            emitter,
            indent_hint.as_mut_ptr(),
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    (*emitter).open_ended = 0 as libc::c_int;
    string.pointer = string.end;
    if string.start == string.pointer {
        chomp_hint = b"-\0" as *const u8 as *const libc::c_char;
    } else {
        loop {
            string.pointer = (string.pointer).offset(-1);
            if !(*string.pointer as libc::c_int & 0xc0 as libc::c_int
                == 0x80 as libc::c_int)
            {
                break;
            }
        }
        if !(*(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -87i32 as yaml_char_t as libc::c_int)
        {
            chomp_hint = b"-\0" as *const u8 as *const libc::c_char;
        } else if string.start == string.pointer {
            chomp_hint = b"+\0" as *const u8 as *const libc::c_char;
            (*emitter).open_ended = 2 as libc::c_int;
        } else {
            loop {
                string.pointer = (string.pointer).offset(-1);
                if !(*string.pointer as libc::c_int & 0xc0 as libc::c_int
                    == 0x80 as libc::c_int)
                {
                    break;
                }
            }
            if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer)
                        .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int == -87i32 as yaml_char_t as libc::c_int
            {
                chomp_hint = b"+\0" as *const u8 as *const libc::c_char;
                (*emitter).open_ended = 2 as libc::c_int;
            }
        }
    }
    if !chomp_hint.is_null() {
        if yaml_emitter_write_indicator(
            emitter,
            chomp_hint,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_literal_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    let mut breaks: libc::c_int = 1 as libc::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if yaml_emitter_write_indicator(
        emitter,
        b"|\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 {
        return 0 as libc::c_int;
    }
    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
        && {
            if (*emitter).line_break as libc::c_uint
                == YAML_CR_BREAK as libc::c_int as libc::c_uint
            {
                let ref mut fresh571 = (*emitter).buffer.pointer;
                let fresh572 = *fresh571;
                *fresh571 = (*fresh571).offset(1);
                *fresh572 = '\r' as i32 as yaml_char_t;
            } else {
                if (*emitter).line_break as libc::c_uint
                    == YAML_LN_BREAK as libc::c_int as libc::c_uint
                {
                    let ref mut fresh573 = (*emitter).buffer.pointer;
                    let fresh574 = *fresh573;
                    *fresh573 = (*fresh573).offset(1);
                    *fresh574 = '\n' as i32 as yaml_char_t;
                } else {
                    if (*emitter).line_break as libc::c_uint
                        == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                    {
                        let ref mut fresh575 = (*emitter).buffer.pointer;
                        let fresh576 = *fresh575;
                        *fresh575 = (*fresh575).offset(1);
                        *fresh576 = '\r' as i32 as yaml_char_t;
                        let ref mut fresh577 = (*emitter).buffer.pointer;
                        let fresh578 = *fresh577;
                        *fresh577 = (*fresh577).offset(1);
                        *fresh578 = '\n' as i32 as yaml_char_t;
                    } else {};
                };
            };
            (*emitter).column = 0 as libc::c_int;
            let ref mut fresh579 = (*emitter).line;
            *fresh579 += 1;
            1 as libc::c_int != 0
        })
    {
        return 0 as libc::c_int;
    }
    (*emitter).indention = 1 as libc::c_int;
    (*emitter).whitespace = 1 as libc::c_int;
    while string.pointer != string.end {
        if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -87i32 as yaml_char_t as libc::c_int
        {
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    ((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh580 = (*emitter).buffer.pointer;
                                let fresh581 = *fresh580;
                                *fresh580 = (*fresh580).offset(1);
                                *fresh581 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as libc::c_uint
                                    == YAML_LN_BREAK as libc::c_int as libc::c_uint
                                {
                                    let ref mut fresh582 = (*emitter).buffer.pointer;
                                    let fresh583 = *fresh582;
                                    *fresh582 = (*fresh582).offset(1);
                                    *fresh583 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as libc::c_uint
                                        == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                                    {
                                        let ref mut fresh584 = (*emitter).buffer.pointer;
                                        let fresh585 = *fresh584;
                                        *fresh584 = (*fresh584).offset(1);
                                        *fresh585 = '\r' as i32 as yaml_char_t;
                                        let ref mut fresh586 = (*emitter).buffer.pointer;
                                        let fresh587 = *fresh586;
                                        *fresh586 = (*fresh586).offset(1);
                                        *fresh587 = '\n' as i32 as yaml_char_t;
                                    } else {};
                                };
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let ref mut fresh588 = (*emitter).line;
                            *fresh588 += 1;
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    string.pointer = (string.pointer).offset(1);
                    1 as libc::c_int
                } else {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh589 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh590 = (*emitter).buffer.pointer;
                        let fresh591 = *fresh590;
                        *fresh590 = (*fresh590).offset(1);
                        *fresh591 = *fresh589;
                    } else {
                        if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            let fresh592 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh593 = (*emitter).buffer.pointer;
                            let fresh594 = *fresh593;
                            *fresh593 = (*fresh593).offset(1);
                            *fresh594 = *fresh592;
                            let fresh595 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh596 = (*emitter).buffer.pointer;
                            let fresh597 = *fresh596;
                            *fresh596 = (*fresh596).offset(1);
                            *fresh597 = *fresh595;
                        } else {
                            if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh598 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh599 = (*emitter).buffer.pointer;
                                let fresh600 = *fresh599;
                                *fresh599 = (*fresh599).offset(1);
                                *fresh600 = *fresh598;
                                let fresh601 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh602 = (*emitter).buffer.pointer;
                                let fresh603 = *fresh602;
                                *fresh602 = (*fresh602).offset(1);
                                *fresh603 = *fresh601;
                                let fresh604 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh605 = (*emitter).buffer.pointer;
                                let fresh606 = *fresh605;
                                *fresh605 = (*fresh605).offset(1);
                                *fresh606 = *fresh604;
                            } else {
                                if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    let fresh607 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh608 = (*emitter).buffer.pointer;
                                    let fresh609 = *fresh608;
                                    *fresh608 = (*fresh608).offset(1);
                                    *fresh609 = *fresh607;
                                    let fresh610 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh611 = (*emitter).buffer.pointer;
                                    let fresh612 = *fresh611;
                                    *fresh611 = (*fresh611).offset(1);
                                    *fresh612 = *fresh610;
                                    let fresh613 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh614 = (*emitter).buffer.pointer;
                                    let fresh615 = *fresh614;
                                    *fresh614 = (*fresh614).offset(1);
                                    *fresh615 = *fresh613;
                                    let fresh616 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh617 = (*emitter).buffer.pointer;
                                    let fresh618 = *fresh617;
                                    *fresh617 = (*fresh617).offset(1);
                                    *fresh618 = *fresh616;
                                } else {};
                            };
                        };
                    };
                    (*emitter).column = 0 as libc::c_int;
                    let ref mut fresh619 = (*emitter).line;
                    *fresh619 += 1;
                    1 as libc::c_int
                }) != 0)
            {
                return 0 as libc::c_int;
            }
            (*emitter).indention = 1 as libc::c_int;
            breaks = 1 as libc::c_int;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
            }
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh620 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh621 = (*emitter).buffer.pointer;
                        let fresh622 = *fresh621;
                        *fresh621 = (*fresh621).offset(1);
                        *fresh622 = *fresh620;
                    } else {
                        if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            let fresh623 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh624 = (*emitter).buffer.pointer;
                            let fresh625 = *fresh624;
                            *fresh624 = (*fresh624).offset(1);
                            *fresh625 = *fresh623;
                            let fresh626 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh627 = (*emitter).buffer.pointer;
                            let fresh628 = *fresh627;
                            *fresh627 = (*fresh627).offset(1);
                            *fresh628 = *fresh626;
                        } else {
                            if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh629 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh630 = (*emitter).buffer.pointer;
                                let fresh631 = *fresh630;
                                *fresh630 = (*fresh630).offset(1);
                                *fresh631 = *fresh629;
                                let fresh632 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh633 = (*emitter).buffer.pointer;
                                let fresh634 = *fresh633;
                                *fresh633 = (*fresh633).offset(1);
                                *fresh634 = *fresh632;
                                let fresh635 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh636 = (*emitter).buffer.pointer;
                                let fresh637 = *fresh636;
                                *fresh636 = (*fresh636).offset(1);
                                *fresh637 = *fresh635;
                            } else {
                                if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    let fresh638 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh639 = (*emitter).buffer.pointer;
                                    let fresh640 = *fresh639;
                                    *fresh639 = (*fresh639).offset(1);
                                    *fresh640 = *fresh638;
                                    let fresh641 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh642 = (*emitter).buffer.pointer;
                                    let fresh643 = *fresh642;
                                    *fresh642 = (*fresh642).offset(1);
                                    *fresh643 = *fresh641;
                                    let fresh644 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh645 = (*emitter).buffer.pointer;
                                    let fresh646 = *fresh645;
                                    *fresh645 = (*fresh645).offset(1);
                                    *fresh646 = *fresh644;
                                    let fresh647 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh648 = (*emitter).buffer.pointer;
                                    let fresh649 = *fresh648;
                                    *fresh648 = (*fresh648).offset(1);
                                    *fresh649 = *fresh647;
                                } else {};
                            };
                        };
                    };
                    let ref mut fresh650 = (*emitter).column;
                    *fresh650 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            (*emitter).indention = 0 as libc::c_int;
            breaks = 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_write_folded_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: 0 as *mut yaml_char_t,
        end: 0 as *mut yaml_char_t,
        pointer: 0 as *mut yaml_char_t,
    };
    let mut breaks: libc::c_int = 1 as libc::c_int;
    let mut leading_spaces: libc::c_int = 1 as libc::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if yaml_emitter_write_indicator(
        emitter,
        b">\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 {
        return 0 as libc::c_int;
    }
    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
        && {
            if (*emitter).line_break as libc::c_uint
                == YAML_CR_BREAK as libc::c_int as libc::c_uint
            {
                let ref mut fresh651 = (*emitter).buffer.pointer;
                let fresh652 = *fresh651;
                *fresh651 = (*fresh651).offset(1);
                *fresh652 = '\r' as i32 as yaml_char_t;
            } else {
                if (*emitter).line_break as libc::c_uint
                    == YAML_LN_BREAK as libc::c_int as libc::c_uint
                {
                    let ref mut fresh653 = (*emitter).buffer.pointer;
                    let fresh654 = *fresh653;
                    *fresh653 = (*fresh653).offset(1);
                    *fresh654 = '\n' as i32 as yaml_char_t;
                } else {
                    if (*emitter).line_break as libc::c_uint
                        == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                    {
                        let ref mut fresh655 = (*emitter).buffer.pointer;
                        let fresh656 = *fresh655;
                        *fresh655 = (*fresh655).offset(1);
                        *fresh656 = '\r' as i32 as yaml_char_t;
                        let ref mut fresh657 = (*emitter).buffer.pointer;
                        let fresh658 = *fresh657;
                        *fresh657 = (*fresh657).offset(1);
                        *fresh658 = '\n' as i32 as yaml_char_t;
                    } else {};
                };
            };
            (*emitter).column = 0 as libc::c_int;
            let ref mut fresh659 = (*emitter).line;
            *fresh659 += 1;
            1 as libc::c_int != 0
        })
    {
        return 0 as libc::c_int;
    }
    (*emitter).indention = 1 as libc::c_int;
    (*emitter).whitespace = 1 as libc::c_int;
    while string.pointer != string.end {
        if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer)
                    .offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int == -87i32 as yaml_char_t as libc::c_int
        {
            if breaks == 0 && leading_spaces == 0
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let mut k: libc::c_int = 0 as libc::c_int;
                while *(string.pointer).offset(k as isize) as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).offset(k as isize) as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).offset(k as isize) as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).offset((k + 1 as libc::c_int) as isize)
                            as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).offset(k as isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).offset((k + 1 as libc::c_int) as isize)
                            as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).offset((k + 2 as libc::c_int) as isize)
                            as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).offset(k as isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).offset((k + 1 as libc::c_int) as isize)
                            as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).offset((k + 2 as libc::c_int) as isize)
                            as libc::c_int == -87i32 as yaml_char_t as libc::c_int
                {
                    k
                        += if *(string.pointer).offset(k as isize) as libc::c_int
                            & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).offset(k as isize) as libc::c_int
                                & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                            2 as libc::c_int
                        } else if *(string.pointer).offset(k as isize) as libc::c_int
                                & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                            3 as libc::c_int
                        } else if *(string.pointer).offset(k as isize) as libc::c_int
                                & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                            {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                }
                if !(*(string.pointer).offset(k as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).offset(k as isize) as libc::c_int
                        == '\t' as i32 as yaml_char_t as libc::c_int
                    || (*(string.pointer).offset(k as isize) as libc::c_int
                        == '\r' as i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).offset(k as isize) as libc::c_int
                            == '\n' as i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).offset(k as isize) as libc::c_int
                            == -62i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).offset((k + 1 as libc::c_int) as isize)
                                as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).offset(k as isize) as libc::c_int
                            == -30i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).offset((k + 1 as libc::c_int) as isize)
                                as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).offset((k + 2 as libc::c_int) as isize)
                                as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).offset(k as isize) as libc::c_int
                            == -30i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).offset((k + 1 as libc::c_int) as isize)
                                as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).offset((k + 2 as libc::c_int) as isize)
                                as libc::c_int == -87i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).offset(k as isize) as libc::c_int
                            == '\0' as i32 as yaml_char_t as libc::c_int))
                {
                    if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh660 = (*emitter).buffer.pointer;
                                let fresh661 = *fresh660;
                                *fresh660 = (*fresh660).offset(1);
                                *fresh661 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as libc::c_uint
                                    == YAML_LN_BREAK as libc::c_int as libc::c_uint
                                {
                                    let ref mut fresh662 = (*emitter).buffer.pointer;
                                    let fresh663 = *fresh662;
                                    *fresh662 = (*fresh662).offset(1);
                                    *fresh663 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as libc::c_uint
                                        == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                                    {
                                        let ref mut fresh664 = (*emitter).buffer.pointer;
                                        let fresh665 = *fresh664;
                                        *fresh664 = (*fresh664).offset(1);
                                        *fresh665 = '\r' as i32 as yaml_char_t;
                                        let ref mut fresh666 = (*emitter).buffer.pointer;
                                        let fresh667 = *fresh666;
                                        *fresh666 = (*fresh666).offset(1);
                                        *fresh667 = '\n' as i32 as yaml_char_t;
                                    } else {};
                                };
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let ref mut fresh668 = (*emitter).line;
                            *fresh668 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
            }
            if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                && (if *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    ((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh669 = (*emitter).buffer.pointer;
                                let fresh670 = *fresh669;
                                *fresh669 = (*fresh669).offset(1);
                                *fresh670 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as libc::c_uint
                                    == YAML_LN_BREAK as libc::c_int as libc::c_uint
                                {
                                    let ref mut fresh671 = (*emitter).buffer.pointer;
                                    let fresh672 = *fresh671;
                                    *fresh671 = (*fresh671).offset(1);
                                    *fresh672 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as libc::c_uint
                                        == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                                    {
                                        let ref mut fresh673 = (*emitter).buffer.pointer;
                                        let fresh674 = *fresh673;
                                        *fresh673 = (*fresh673).offset(1);
                                        *fresh674 = '\r' as i32 as yaml_char_t;
                                        let ref mut fresh675 = (*emitter).buffer.pointer;
                                        let fresh676 = *fresh675;
                                        *fresh675 = (*fresh675).offset(1);
                                        *fresh676 = '\n' as i32 as yaml_char_t;
                                    } else {};
                                };
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let ref mut fresh677 = (*emitter).line;
                            *fresh677 += 1;
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    string.pointer = (string.pointer).offset(1);
                    1 as libc::c_int
                } else {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh678 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        let ref mut fresh679 = (*emitter).buffer.pointer;
                        let fresh680 = *fresh679;
                        *fresh679 = (*fresh679).offset(1);
                        *fresh680 = *fresh678;
                    } else {
                        if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            let fresh681 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh682 = (*emitter).buffer.pointer;
                            let fresh683 = *fresh682;
                            *fresh682 = (*fresh682).offset(1);
                            *fresh683 = *fresh681;
                            let fresh684 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh685 = (*emitter).buffer.pointer;
                            let fresh686 = *fresh685;
                            *fresh685 = (*fresh685).offset(1);
                            *fresh686 = *fresh684;
                        } else {
                            if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh687 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh688 = (*emitter).buffer.pointer;
                                let fresh689 = *fresh688;
                                *fresh688 = (*fresh688).offset(1);
                                *fresh689 = *fresh687;
                                let fresh690 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh691 = (*emitter).buffer.pointer;
                                let fresh692 = *fresh691;
                                *fresh691 = (*fresh691).offset(1);
                                *fresh692 = *fresh690;
                                let fresh693 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh694 = (*emitter).buffer.pointer;
                                let fresh695 = *fresh694;
                                *fresh694 = (*fresh694).offset(1);
                                *fresh695 = *fresh693;
                            } else {
                                if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    let fresh696 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh697 = (*emitter).buffer.pointer;
                                    let fresh698 = *fresh697;
                                    *fresh697 = (*fresh697).offset(1);
                                    *fresh698 = *fresh696;
                                    let fresh699 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh700 = (*emitter).buffer.pointer;
                                    let fresh701 = *fresh700;
                                    *fresh700 = (*fresh700).offset(1);
                                    *fresh701 = *fresh699;
                                    let fresh702 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh703 = (*emitter).buffer.pointer;
                                    let fresh704 = *fresh703;
                                    *fresh703 = (*fresh703).offset(1);
                                    *fresh704 = *fresh702;
                                    let fresh705 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh706 = (*emitter).buffer.pointer;
                                    let fresh707 = *fresh706;
                                    *fresh706 = (*fresh706).offset(1);
                                    *fresh707 = *fresh705;
                                } else {};
                            };
                        };
                    };
                    (*emitter).column = 0 as libc::c_int;
                    let ref mut fresh708 = (*emitter).line;
                    *fresh708 += 1;
                    1 as libc::c_int
                }) != 0)
            {
                return 0 as libc::c_int;
            }
            (*emitter).indention = 1 as libc::c_int;
            breaks = 1 as libc::c_int;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                leading_spaces = (*(string.pointer).offset(0 as libc::c_int as isize)
                    as libc::c_int == ' ' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                        == '\t' as i32 as yaml_char_t as libc::c_int) as libc::c_int;
            }
            if breaks == 0
                && *(string.pointer).offset(0 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                && !(*(string.pointer).offset(1 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int)
                && (*emitter).column > (*emitter).best_width
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                string
                    .pointer = (string.pointer)
                    .offset(
                        (if *(string.pointer).offset(0 as libc::c_int as isize)
                            as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                            {
                            2 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                            {
                            3 as libc::c_int
                        } else if *(string.pointer).offset(0 as libc::c_int as isize)
                                as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                            {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    );
            } else if !((((*emitter).buffer.pointer).offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end || yaml_emitter_flush(emitter) != 0)
                    && {
                        if *string.pointer as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            let fresh709 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            let ref mut fresh710 = (*emitter).buffer.pointer;
                            let fresh711 = *fresh710;
                            *fresh710 = (*fresh710).offset(1);
                            *fresh711 = *fresh709;
                        } else {
                            if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                let fresh712 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh713 = (*emitter).buffer.pointer;
                                let fresh714 = *fresh713;
                                *fresh713 = (*fresh713).offset(1);
                                *fresh714 = *fresh712;
                                let fresh715 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                let ref mut fresh716 = (*emitter).buffer.pointer;
                                let fresh717 = *fresh716;
                                *fresh716 = (*fresh716).offset(1);
                                *fresh717 = *fresh715;
                            } else {
                                if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                                    == 0xe0 as libc::c_int
                                {
                                    let fresh718 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh719 = (*emitter).buffer.pointer;
                                    let fresh720 = *fresh719;
                                    *fresh719 = (*fresh719).offset(1);
                                    *fresh720 = *fresh718;
                                    let fresh721 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh722 = (*emitter).buffer.pointer;
                                    let fresh723 = *fresh722;
                                    *fresh722 = (*fresh722).offset(1);
                                    *fresh723 = *fresh721;
                                    let fresh724 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    let ref mut fresh725 = (*emitter).buffer.pointer;
                                    let fresh726 = *fresh725;
                                    *fresh725 = (*fresh725).offset(1);
                                    *fresh726 = *fresh724;
                                } else {
                                    if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                                        == 0xf0 as libc::c_int
                                    {
                                        let fresh727 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh728 = (*emitter).buffer.pointer;
                                        let fresh729 = *fresh728;
                                        *fresh728 = (*fresh728).offset(1);
                                        *fresh729 = *fresh727;
                                        let fresh730 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh731 = (*emitter).buffer.pointer;
                                        let fresh732 = *fresh731;
                                        *fresh731 = (*fresh731).offset(1);
                                        *fresh732 = *fresh730;
                                        let fresh733 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh734 = (*emitter).buffer.pointer;
                                        let fresh735 = *fresh734;
                                        *fresh734 = (*fresh734).offset(1);
                                        *fresh735 = *fresh733;
                                        let fresh736 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        let ref mut fresh737 = (*emitter).buffer.pointer;
                                        let fresh738 = *fresh737;
                                        *fresh737 = (*fresh737).offset(1);
                                        *fresh738 = *fresh736;
                                    } else {};
                                };
                            };
                        };
                        let ref mut fresh739 = (*emitter).column;
                        *fresh739 += 1;
                        1 as libc::c_int != 0
                    })
                {
                return 0 as libc::c_int
            }
            (*emitter).indention = 0 as libc::c_int;
            breaks = 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
