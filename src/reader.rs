use crate::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
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
unsafe extern "C" fn yaml_parser_set_reader_error(
    mut parser: *mut yaml_parser_t,
    mut problem: *const libc::c_char,
    mut offset: size_t,
    mut value: libc::c_int,
) -> libc::c_int {
    (*parser).error = YAML_READER_ERROR;
    let ref mut fresh0 = (*parser).problem;
    *fresh0 = problem;
    (*parser).problem_offset = offset;
    (*parser).problem_value = value;
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_determine_encoding(
    mut parser: *mut yaml_parser_t,
) -> libc::c_int {
    while (*parser).eof == 0
        && (((*parser).raw_buffer.last).offset_from((*parser).raw_buffer.pointer)
            as libc::c_long) < 3 as libc::c_int as libc::c_long
    {
        if yaml_parser_update_raw_buffer(parser) == 0 {
            return 0 as libc::c_int;
        }
    }
    if ((*parser).raw_buffer.last).offset_from((*parser).raw_buffer.pointer)
        as libc::c_long >= 2 as libc::c_int as libc::c_long
        && memcmp(
            (*parser).raw_buffer.pointer as *const libc::c_void,
            b"\xFF\xFE\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        (*parser).encoding = YAML_UTF16LE_ENCODING;
        let ref mut fresh1 = (*parser).raw_buffer.pointer;
        *fresh1 = (*fresh1).offset(2 as libc::c_int as isize);
        let ref mut fresh2 = (*parser).offset;
        *fresh2 = (*fresh2 as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    } else if ((*parser).raw_buffer.last).offset_from((*parser).raw_buffer.pointer)
            as libc::c_long >= 2 as libc::c_int as libc::c_long
            && memcmp(
                (*parser).raw_buffer.pointer as *const libc::c_void,
                b"\xFE\xFF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) == 0
        {
        (*parser).encoding = YAML_UTF16BE_ENCODING;
        let ref mut fresh3 = (*parser).raw_buffer.pointer;
        *fresh3 = (*fresh3).offset(2 as libc::c_int as isize);
        let ref mut fresh4 = (*parser).offset;
        *fresh4 = (*fresh4 as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    } else if ((*parser).raw_buffer.last).offset_from((*parser).raw_buffer.pointer)
            as libc::c_long >= 3 as libc::c_int as libc::c_long
            && memcmp(
                (*parser).raw_buffer.pointer as *const libc::c_void,
                b"\xEF\xBB\xBF\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            ) == 0
        {
        (*parser).encoding = YAML_UTF8_ENCODING;
        let ref mut fresh5 = (*parser).raw_buffer.pointer;
        *fresh5 = (*fresh5).offset(3 as libc::c_int as isize);
        let ref mut fresh6 = (*parser).offset;
        *fresh6 = (*fresh6 as libc::c_ulong)
            .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
    } else {
        (*parser).encoding = YAML_UTF8_ENCODING;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_update_raw_buffer(
    mut parser: *mut yaml_parser_t,
) -> libc::c_int {
    let mut size_read: size_t = 0 as libc::c_int as size_t;
    if (*parser).raw_buffer.start == (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.last == (*parser).raw_buffer.end
    {
        return 1 as libc::c_int;
    }
    if (*parser).eof != 0 {
        return 1 as libc::c_int;
    }
    if (*parser).raw_buffer.start < (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.pointer < (*parser).raw_buffer.last
    {
        memmove(
            (*parser).raw_buffer.start as *mut libc::c_void,
            (*parser).raw_buffer.pointer as *const libc::c_void,
            ((*parser).raw_buffer.last).offset_from((*parser).raw_buffer.pointer)
                as libc::c_long as libc::c_ulong,
        );
    }
    let ref mut fresh7 = (*parser).raw_buffer.last;
    *fresh7 = (*fresh7)
        .offset(
            -(((*parser).raw_buffer.pointer).offset_from((*parser).raw_buffer.start)
                as libc::c_long as isize),
        );
    let ref mut fresh8 = (*parser).raw_buffer.pointer;
    *fresh8 = (*parser).raw_buffer.start;
    if ((*parser).read_handler)
        .expect(
            "non-null function pointer",
        )(
        (*parser).read_handler_data,
        (*parser).raw_buffer.last,
        ((*parser).raw_buffer.end).offset_from((*parser).raw_buffer.last) as libc::c_long
            as size_t,
        &mut size_read,
    ) == 0
    {
        return yaml_parser_set_reader_error(
            parser,
            b"input error\0" as *const u8 as *const libc::c_char,
            (*parser).offset,
            -(1 as libc::c_int),
        );
    }
    let ref mut fresh9 = (*parser).raw_buffer.last;
    *fresh9 = (*fresh9).offset(size_read as isize);
    if size_read == 0 {
        (*parser).eof = 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_update_buffer(
    mut parser: *mut yaml_parser_t,
    mut length: size_t,
) -> libc::c_int {
    let mut first: libc::c_int = 1 as libc::c_int;
    if ((*parser).read_handler).is_some() {} else {
        __assert_fail(
            b"parser->read_handler\0" as *const u8 as *const libc::c_char,
            b"reader.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int yaml_parser_update_buffer(yaml_parser_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    if (*parser).eof != 0 && (*parser).raw_buffer.pointer == (*parser).raw_buffer.last {
        return 1 as libc::c_int;
    }
    if (*parser).unread >= length {
        return 1 as libc::c_int;
    }
    if (*parser).encoding as u64 == 0 {
        if yaml_parser_determine_encoding(parser) == 0 {
            return 0 as libc::c_int;
        }
    }
    if (*parser).buffer.start < (*parser).buffer.pointer
        && (*parser).buffer.pointer < (*parser).buffer.last
    {
        let mut size: size_t = ((*parser).buffer.last)
            .offset_from((*parser).buffer.pointer) as libc::c_long as size_t;
        memmove(
            (*parser).buffer.start as *mut libc::c_void,
            (*parser).buffer.pointer as *const libc::c_void,
            size,
        );
        let ref mut fresh10 = (*parser).buffer.pointer;
        *fresh10 = (*parser).buffer.start;
        let ref mut fresh11 = (*parser).buffer.last;
        *fresh11 = ((*parser).buffer.start).offset(size as isize);
    } else if (*parser).buffer.pointer == (*parser).buffer.last {
        let ref mut fresh12 = (*parser).buffer.pointer;
        *fresh12 = (*parser).buffer.start;
        let ref mut fresh13 = (*parser).buffer.last;
        *fresh13 = (*parser).buffer.start;
    }
    while (*parser).unread < length {
        if first == 0 || (*parser).raw_buffer.pointer == (*parser).raw_buffer.last {
            if yaml_parser_update_raw_buffer(parser) == 0 {
                return 0 as libc::c_int;
            }
        }
        first = 0 as libc::c_int;
        while (*parser).raw_buffer.pointer != (*parser).raw_buffer.last {
            let mut value: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut value2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut incomplete: libc::c_int = 0 as libc::c_int;
            let mut octet: libc::c_uchar = 0;
            let mut width: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut low: libc::c_int = 0;
            let mut high: libc::c_int = 0;
            let mut k: size_t = 0;
            let mut raw_unread: size_t = ((*parser).raw_buffer.last)
                .offset_from((*parser).raw_buffer.pointer) as libc::c_long as size_t;
            match (*parser).encoding as libc::c_uint {
                1 => {
                    octet = *((*parser).raw_buffer.pointer)
                        .offset(0 as libc::c_int as isize);
                    width = (if octet as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if octet as libc::c_int & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                        2 as libc::c_int
                    } else if octet as libc::c_int & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                        3 as libc::c_int
                    } else if octet as libc::c_int & 0xf8 as libc::c_int
                            == 0xf0 as libc::c_int
                        {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint;
                    if width == 0 {
                        return yaml_parser_set_reader_error(
                            parser,
                            b"invalid leading UTF-8 octet\0" as *const u8
                                as *const libc::c_char,
                            (*parser).offset,
                            octet as libc::c_int,
                        );
                    }
                    if width as libc::c_ulong > raw_unread {
                        if (*parser).eof != 0 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-8 octet sequence\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                -(1 as libc::c_int),
                            );
                        }
                        incomplete = 1 as libc::c_int;
                    } else {
                        value = (if octet as libc::c_int & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            octet as libc::c_int & 0x7f as libc::c_int
                        } else if octet as libc::c_int & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                            octet as libc::c_int & 0x1f as libc::c_int
                        } else if octet as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                            octet as libc::c_int & 0xf as libc::c_int
                        } else if octet as libc::c_int & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                            octet as libc::c_int & 0x7 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as libc::c_uint;
                        k = 1 as libc::c_int as size_t;
                        while k < width as libc::c_ulong {
                            octet = *((*parser).raw_buffer.pointer).offset(k as isize);
                            if octet as libc::c_int & 0xc0 as libc::c_int
                                != 0x80 as libc::c_int
                            {
                                return yaml_parser_set_reader_error(
                                    parser,
                                    b"invalid trailing UTF-8 octet\0" as *const u8
                                        as *const libc::c_char,
                                    ((*parser).offset).wrapping_add(k),
                                    octet as libc::c_int,
                                );
                            }
                            value = (value << 6 as libc::c_int)
                                .wrapping_add(
                                    (octet as libc::c_int & 0x3f as libc::c_int) as libc::c_uint,
                                );
                            k = k.wrapping_add(1);
                        }
                        if !(width == 1 as libc::c_int as libc::c_uint
                            || width == 2 as libc::c_int as libc::c_uint
                                && value >= 0x80 as libc::c_int as libc::c_uint
                            || width == 3 as libc::c_int as libc::c_uint
                                && value >= 0x800 as libc::c_int as libc::c_uint
                            || width == 4 as libc::c_int as libc::c_uint
                                && value >= 0x10000 as libc::c_int as libc::c_uint)
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid length of a UTF-8 sequence\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                -(1 as libc::c_int),
                            );
                        }
                        if value >= 0xd800 as libc::c_int as libc::c_uint
                            && value <= 0xdfff as libc::c_int as libc::c_uint
                            || value > 0x10ffff as libc::c_int as libc::c_uint
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid Unicode character\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                value as libc::c_int,
                            );
                        }
                    }
                }
                2 | 3 => {
                    low = if (*parser).encoding as libc::c_uint
                        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
                    {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    high = if (*parser).encoding as libc::c_uint
                        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    if raw_unread < 2 as libc::c_int as libc::c_ulong {
                        if (*parser).eof != 0 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-16 character\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                -(1 as libc::c_int),
                            );
                        }
                        incomplete = 1 as libc::c_int;
                    } else {
                        value = (*((*parser).raw_buffer.pointer).offset(low as isize)
                            as libc::c_int
                            + ((*((*parser).raw_buffer.pointer).offset(high as isize)
                                as libc::c_int) << 8 as libc::c_int)) as libc::c_uint;
                        if value & 0xfc00 as libc::c_int as libc::c_uint
                            == 0xdc00 as libc::c_int as libc::c_uint
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"unexpected low surrogate area\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                value as libc::c_int,
                            );
                        }
                        if value & 0xfc00 as libc::c_int as libc::c_uint
                            == 0xd800 as libc::c_int as libc::c_uint
                        {
                            width = 4 as libc::c_int as libc::c_uint;
                            if raw_unread < 4 as libc::c_int as libc::c_ulong {
                                if (*parser).eof != 0 {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"incomplete UTF-16 surrogate pair\0" as *const u8
                                            as *const libc::c_char,
                                        (*parser).offset,
                                        -(1 as libc::c_int),
                                    );
                                }
                                incomplete = 1 as libc::c_int;
                            } else {
                                value2 = (*((*parser).raw_buffer.pointer)
                                    .offset((low + 2 as libc::c_int) as isize) as libc::c_int
                                    + ((*((*parser).raw_buffer.pointer)
                                        .offset((high + 2 as libc::c_int) as isize) as libc::c_int)
                                        << 8 as libc::c_int)) as libc::c_uint;
                                if value2 & 0xfc00 as libc::c_int as libc::c_uint
                                    != 0xdc00 as libc::c_int as libc::c_uint
                                {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"expected low surrogate area\0" as *const u8
                                            as *const libc::c_char,
                                        ((*parser).offset)
                                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                                        value2 as libc::c_int,
                                    );
                                }
                                value = (0x10000 as libc::c_int as libc::c_uint)
                                    .wrapping_add(
                                        (value & 0x3ff as libc::c_int as libc::c_uint)
                                            << 10 as libc::c_int,
                                    )
                                    .wrapping_add(
                                        value2 & 0x3ff as libc::c_int as libc::c_uint,
                                    );
                            }
                        } else {
                            width = 2 as libc::c_int as libc::c_uint;
                        }
                    }
                }
                _ => {}
            }
            if incomplete != 0 {
                break;
            }
            if !(value == 0x9 as libc::c_int as libc::c_uint
                || value == 0xa as libc::c_int as libc::c_uint
                || value == 0xd as libc::c_int as libc::c_uint
                || value >= 0x20 as libc::c_int as libc::c_uint
                    && value <= 0x7e as libc::c_int as libc::c_uint
                || value == 0x85 as libc::c_int as libc::c_uint
                || value >= 0xa0 as libc::c_int as libc::c_uint
                    && value <= 0xd7ff as libc::c_int as libc::c_uint
                || value >= 0xe000 as libc::c_int as libc::c_uint
                    && value <= 0xfffd as libc::c_int as libc::c_uint
                || value >= 0x10000 as libc::c_int as libc::c_uint
                    && value <= 0x10ffff as libc::c_int as libc::c_uint)
            {
                return yaml_parser_set_reader_error(
                    parser,
                    b"control characters are not allowed\0" as *const u8
                        as *const libc::c_char,
                    (*parser).offset,
                    value as libc::c_int,
                );
            }
            let ref mut fresh14 = (*parser).raw_buffer.pointer;
            *fresh14 = (*fresh14).offset(width as isize);
            let ref mut fresh15 = (*parser).offset;
            *fresh15 = (*fresh15 as libc::c_ulong).wrapping_add(width as libc::c_ulong)
                as size_t as size_t;
            if value <= 0x7f as libc::c_int as libc::c_uint {
                let ref mut fresh16 = (*parser).buffer.last;
                let fresh17 = *fresh16;
                *fresh16 = (*fresh16).offset(1);
                *fresh17 = value as yaml_char_t;
            } else if value <= 0x7ff as libc::c_int as libc::c_uint {
                let ref mut fresh18 = (*parser).buffer.last;
                let fresh19 = *fresh18;
                *fresh18 = (*fresh18).offset(1);
                *fresh19 = (0xc0 as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 6 as libc::c_int) as yaml_char_t;
                let ref mut fresh20 = (*parser).buffer.last;
                let fresh21 = *fresh20;
                *fresh20 = (*fresh20).offset(1);
                *fresh21 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(value & 0x3f as libc::c_int as libc::c_uint)
                    as yaml_char_t;
            } else if value <= 0xffff as libc::c_int as libc::c_uint {
                let ref mut fresh22 = (*parser).buffer.last;
                let fresh23 = *fresh22;
                *fresh22 = (*fresh22).offset(1);
                *fresh23 = (0xe0 as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 12 as libc::c_int) as yaml_char_t;
                let ref mut fresh24 = (*parser).buffer.last;
                let fresh25 = *fresh24;
                *fresh24 = (*fresh24).offset(1);
                *fresh25 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        value >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint,
                    ) as yaml_char_t;
                let ref mut fresh26 = (*parser).buffer.last;
                let fresh27 = *fresh26;
                *fresh26 = (*fresh26).offset(1);
                *fresh27 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(value & 0x3f as libc::c_int as libc::c_uint)
                    as yaml_char_t;
            } else {
                let ref mut fresh28 = (*parser).buffer.last;
                let fresh29 = *fresh28;
                *fresh28 = (*fresh28).offset(1);
                *fresh29 = (0xf0 as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 18 as libc::c_int) as yaml_char_t;
                let ref mut fresh30 = (*parser).buffer.last;
                let fresh31 = *fresh30;
                *fresh30 = (*fresh30).offset(1);
                *fresh31 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        value >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint,
                    ) as yaml_char_t;
                let ref mut fresh32 = (*parser).buffer.last;
                let fresh33 = *fresh32;
                *fresh32 = (*fresh32).offset(1);
                *fresh33 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        value >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint,
                    ) as yaml_char_t;
                let ref mut fresh34 = (*parser).buffer.last;
                let fresh35 = *fresh34;
                *fresh34 = (*fresh34).offset(1);
                *fresh35 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(value & 0x3f as libc::c_int as libc::c_uint)
                    as yaml_char_t;
            }
            let ref mut fresh36 = (*parser).unread;
            *fresh36 = (*fresh36).wrapping_add(1);
        }
        if (*parser).eof != 0 {
            let ref mut fresh37 = (*parser).buffer.last;
            let fresh38 = *fresh37;
            *fresh37 = (*fresh37).offset(1);
            *fresh38 = '\0' as i32 as yaml_char_t;
            let ref mut fresh39 = (*parser).unread;
            *fresh39 = (*fresh39).wrapping_add(1);
            return 1 as libc::c_int;
        }
    }
    if (*parser).offset
        >= (!(0 as libc::c_int as size_t))
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        return yaml_parser_set_reader_error(
            parser,
            b"input is too long\0" as *const u8 as *const libc::c_char,
            (*parser).offset,
            -(1 as libc::c_int),
        );
    }
    return 1 as libc::c_int;
}
