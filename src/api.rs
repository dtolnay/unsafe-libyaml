use crate::externs::{free, malloc, memcpy, memmove, memset, realloc, strdup, strlen};
use crate::yaml::{
    Unnamed_16, Unnamed_17, Unnamed_26, Unnamed_27, Unnamed_28, Unnamed_29, Unnamed_30, Unnamed_31,
    Unnamed_32, Unnamed_33, Unnamed_34, Unnamed_35,
};
use crate::{
    libc, size_t, yaml_break_t, yaml_char_t, yaml_document_t, yaml_emitter_state_t, yaml_emitter_t,
    yaml_encoding_t, yaml_event_t, yaml_mapping_style_t, yaml_mark_t, yaml_node_item_t,
    yaml_node_pair_t, yaml_node_t, yaml_parser_state_t, yaml_parser_t, yaml_read_handler_t,
    yaml_scalar_style_t, yaml_sequence_style_t, yaml_simple_key_t, yaml_tag_directive_t,
    yaml_token_t, yaml_version_directive_t, yaml_write_handler_t, PointerExt, YAML_ALIAS_EVENT,
    YAML_DOCUMENT_END_EVENT, YAML_DOCUMENT_START_EVENT, YAML_MAPPING_END_EVENT, YAML_MAPPING_NODE,
    YAML_MAPPING_START_EVENT, YAML_MEMORY_ERROR, YAML_NO_ERROR, YAML_SCALAR_EVENT,
    YAML_SCALAR_NODE, YAML_SEQUENCE_END_EVENT, YAML_SEQUENCE_NODE, YAML_SEQUENCE_START_EVENT,
    YAML_STREAM_END_EVENT, YAML_STREAM_START_EVENT,
};
use core::mem::{size_of, MaybeUninit};
use core::ptr::{self, addr_of_mut};
pub unsafe fn yaml_get_version_string() -> *const libc::c_char {
    b"0.2.5\0" as *const u8 as *const libc::c_char
}
pub unsafe fn yaml_get_version(
    major: *mut libc::c_int,
    minor: *mut libc::c_int,
    patch: *mut libc::c_int,
) {
    *major = 0_i32;
    *minor = 2_i32;
    *patch = 5_i32;
}
pub unsafe fn yaml_malloc(size: size_t) -> *mut libc::c_void {
    malloc(size)
}
pub unsafe fn yaml_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void {
    if !ptr.is_null() {
        realloc(ptr, size)
    } else {
        malloc(size)
    }
}
pub unsafe fn yaml_free(ptr: *mut libc::c_void) {
    if !ptr.is_null() {
        free(ptr);
    }
}
pub unsafe fn yaml_strdup(str: *const yaml_char_t) -> *mut yaml_char_t {
    if str.is_null() {
        return ptr::null_mut::<yaml_char_t>();
    }
    strdup(str as *mut libc::c_char) as *mut yaml_char_t
}
pub unsafe fn yaml_string_extend(
    start: *mut *mut yaml_char_t,
    pointer: *mut *mut yaml_char_t,
    end: *mut *mut yaml_char_t,
) -> libc::c_int {
    let new_start: *mut yaml_char_t = yaml_realloc(
        *start as *mut libc::c_void,
        ((*end).c_offset_from(*start) as libc::c_long * 2_i64) as size_t,
    ) as *mut yaml_char_t;
    if new_start.is_null() {
        return 0_i32;
    }
    memset(
        new_start.wrapping_offset((*end).c_offset_from(*start) as libc::c_long as isize)
            as *mut libc::c_void,
        0_i32,
        (*end).c_offset_from(*start) as libc::c_long as libc::c_ulong,
    );
    *pointer = new_start.wrapping_offset((*pointer).c_offset_from(*start) as libc::c_long as isize);
    *end =
        new_start.wrapping_offset(((*end).c_offset_from(*start) as libc::c_long * 2_i64) as isize);
    *start = new_start;
    1_i32
}
pub unsafe fn yaml_string_join(
    a_start: *mut *mut yaml_char_t,
    a_pointer: *mut *mut yaml_char_t,
    a_end: *mut *mut yaml_char_t,
    b_start: *mut *mut yaml_char_t,
    b_pointer: *mut *mut yaml_char_t,
    _b_end: *mut *mut yaml_char_t,
) -> libc::c_int {
    if *b_start == *b_pointer {
        return 1_i32;
    }
    while (*a_end).c_offset_from(*a_pointer) as libc::c_long
        <= (*b_pointer).c_offset_from(*b_start) as libc::c_long
    {
        if yaml_string_extend(a_start, a_pointer, a_end) == 0 {
            return 0_i32;
        }
    }
    memcpy(
        *a_pointer as *mut libc::c_void,
        *b_start as *const libc::c_void,
        (*b_pointer).c_offset_from(*b_start) as libc::c_long as libc::c_ulong,
    );
    *a_pointer =
        (*a_pointer).wrapping_offset((*b_pointer).c_offset_from(*b_start) as libc::c_long as isize);
    1_i32
}
pub unsafe fn yaml_stack_extend(
    start: *mut *mut libc::c_void,
    top: *mut *mut libc::c_void,
    end: *mut *mut libc::c_void,
) -> libc::c_int {
    if (*end as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
        >= (2147483647_i32 / 2_i32) as libc::c_long
    {
        return 0_i32;
    }
    let new_start: *mut libc::c_void = yaml_realloc(
        *start,
        ((*end as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
            * 2_i64) as size_t,
    );
    if new_start.is_null() {
        return 0_i32;
    }
    *top = (new_start as *mut libc::c_char).wrapping_offset(
        (*top as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
            as isize,
    ) as *mut libc::c_void;
    *end = (new_start as *mut libc::c_char).wrapping_offset(
        ((*end as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
            * 2_i64) as isize,
    ) as *mut libc::c_void;
    *start = new_start;
    1_i32
}
pub unsafe fn yaml_queue_extend(
    start: *mut *mut libc::c_void,
    head: *mut *mut libc::c_void,
    tail: *mut *mut libc::c_void,
    end: *mut *mut libc::c_void,
) -> libc::c_int {
    if *start == *head && *tail == *end {
        let new_start: *mut libc::c_void = yaml_realloc(
            *start,
            ((*end as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
                * 2_i64) as size_t,
        );
        if new_start.is_null() {
            return 0_i32;
        }
        *head = (new_start as *mut libc::c_char).wrapping_offset(
            (*head as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
                as isize,
        ) as *mut libc::c_void;
        *tail = (new_start as *mut libc::c_char).wrapping_offset(
            (*tail as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
                as isize,
        ) as *mut libc::c_void;
        *end = (new_start as *mut libc::c_char).wrapping_offset(
            ((*end as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
                * 2_i64) as isize,
        ) as *mut libc::c_void;
        *start = new_start;
    }
    if *tail == *end {
        if *head != *tail {
            memmove(
                *start,
                *head,
                (*tail as *mut libc::c_char).c_offset_from(*head as *mut libc::c_char)
                    as libc::c_long as libc::c_ulong,
            );
        }
        *tail = (*start as *mut libc::c_char).wrapping_offset(
            (*tail as *mut libc::c_char).c_offset_from(*head as *mut libc::c_char) as libc::c_long
                as isize,
        ) as *mut libc::c_void;
        *head = *start;
    }
    1_i32
}
pub unsafe fn yaml_parser_initialize(mut parser: *mut yaml_parser_t) -> libc::c_int {
    __assert!(!parser.is_null());
    memset(
        parser as *mut libc::c_void,
        0_i32,
        size_of::<yaml_parser_t>() as libc::c_ulong,
    );
    let fresh0 = addr_of_mut!((*parser).raw_buffer.start);
    *fresh0 = yaml_malloc(16384_u64) as *mut yaml_char_t;
    if !(if !(*fresh0).is_null() {
        let fresh1 = addr_of_mut!((*parser).raw_buffer.pointer);
        *fresh1 = (*parser).raw_buffer.start;
        let fresh2 = addr_of_mut!((*parser).raw_buffer.last);
        *fresh2 = *fresh1;
        let fresh3 = addr_of_mut!((*parser).raw_buffer.end);
        *fresh3 = ((*parser).raw_buffer.start).wrapping_offset(16384_isize);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
        let fresh4 = addr_of_mut!((*parser).buffer.start);
        *fresh4 = yaml_malloc((16384_i32 * 3_i32) as size_t) as *mut yaml_char_t;
        if !(if !(*fresh4).is_null() {
            let fresh5 = addr_of_mut!((*parser).buffer.pointer);
            *fresh5 = (*parser).buffer.start;
            let fresh6 = addr_of_mut!((*parser).buffer.last);
            *fresh6 = *fresh5;
            let fresh7 = addr_of_mut!((*parser).buffer.end);
            *fresh7 = ((*parser).buffer.start).wrapping_offset((16384_i32 * 3_i32) as isize);
            1_i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0)
        {
            let fresh8 = addr_of_mut!((*parser).tokens.start);
            *fresh8 = yaml_malloc((16_u64).wrapping_mul(size_of::<yaml_token_t>() as libc::c_ulong))
                as *mut yaml_token_t;
            if !(if !(*fresh8).is_null() {
                let fresh9 = addr_of_mut!((*parser).tokens.tail);
                *fresh9 = (*parser).tokens.start;
                let fresh10 = addr_of_mut!((*parser).tokens.head);
                *fresh10 = *fresh9;
                let fresh11 = addr_of_mut!((*parser).tokens.end);
                *fresh11 = ((*parser).tokens.start).wrapping_offset(16_isize);
                1_i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0_i32
            } == 0)
            {
                let fresh12 = addr_of_mut!((*parser).indents.start);
                *fresh12 =
                    yaml_malloc((16_u64).wrapping_mul(size_of::<libc::c_int>() as libc::c_ulong))
                        as *mut libc::c_int;
                if !(if !(*fresh12).is_null() {
                    let fresh13 = addr_of_mut!((*parser).indents.top);
                    *fresh13 = (*parser).indents.start;
                    let fresh14 = addr_of_mut!((*parser).indents.end);
                    *fresh14 = ((*parser).indents.start).wrapping_offset(16_isize);
                    1_i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0_i32
                } == 0)
                {
                    let fresh15 = addr_of_mut!((*parser).simple_keys.start);
                    *fresh15 = yaml_malloc(
                        (16_u64).wrapping_mul(size_of::<yaml_simple_key_t>() as libc::c_ulong),
                    ) as *mut yaml_simple_key_t;
                    if !(if !(*fresh15).is_null() {
                        let fresh16 = addr_of_mut!((*parser).simple_keys.top);
                        *fresh16 = (*parser).simple_keys.start;
                        let fresh17 = addr_of_mut!((*parser).simple_keys.end);
                        *fresh17 = ((*parser).simple_keys.start).wrapping_offset(16_isize);
                        1_i32
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0_i32
                    } == 0)
                    {
                        let fresh18 = addr_of_mut!((*parser).states.start);
                        *fresh18 = yaml_malloc(
                            (16_u64)
                                .wrapping_mul(size_of::<yaml_parser_state_t>() as libc::c_ulong),
                        ) as *mut yaml_parser_state_t;
                        if !(if !(*fresh18).is_null() {
                            let fresh19 = addr_of_mut!((*parser).states.top);
                            *fresh19 = (*parser).states.start;
                            let fresh20 = addr_of_mut!((*parser).states.end);
                            *fresh20 = ((*parser).states.start).wrapping_offset(16_isize);
                            1_i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0_i32
                        } == 0)
                        {
                            let fresh21 = addr_of_mut!((*parser).marks.start);
                            *fresh21 = yaml_malloc(
                                (16_u64).wrapping_mul(size_of::<yaml_mark_t>() as libc::c_ulong),
                            ) as *mut yaml_mark_t;
                            if !(if !(*fresh21).is_null() {
                                let fresh22 = addr_of_mut!((*parser).marks.top);
                                *fresh22 = (*parser).marks.start;
                                let fresh23 = addr_of_mut!((*parser).marks.end);
                                *fresh23 = ((*parser).marks.start).wrapping_offset(16_isize);
                                1_i32
                            } else {
                                (*parser).error = YAML_MEMORY_ERROR;
                                0_i32
                            } == 0)
                            {
                                let fresh24 = addr_of_mut!((*parser).tag_directives.start);
                                *fresh24 =
                                    yaml_malloc((16_u64).wrapping_mul(
                                        size_of::<yaml_tag_directive_t>() as libc::c_ulong,
                                    ))
                                        as *mut yaml_tag_directive_t;
                                if !(if !(*fresh24).is_null() {
                                    let fresh25 = addr_of_mut!((*parser).tag_directives.top);
                                    *fresh25 = (*parser).tag_directives.start;
                                    let fresh26 = addr_of_mut!((*parser).tag_directives.end);
                                    *fresh26 =
                                        ((*parser).tag_directives.start).wrapping_offset(16_isize);
                                    1_i32
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0_i32
                                } == 0)
                                {
                                    return 1_i32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free((*parser).raw_buffer.start as *mut libc::c_void);
    let fresh27 = addr_of_mut!((*parser).raw_buffer.end);
    *fresh27 = ptr::null_mut::<libc::c_uchar>();
    let fresh28 = addr_of_mut!((*parser).raw_buffer.pointer);
    *fresh28 = *fresh27;
    let fresh29 = addr_of_mut!((*parser).raw_buffer.start);
    *fresh29 = *fresh28;
    yaml_free((*parser).buffer.start as *mut libc::c_void);
    let fresh30 = addr_of_mut!((*parser).buffer.end);
    *fresh30 = ptr::null_mut::<yaml_char_t>();
    let fresh31 = addr_of_mut!((*parser).buffer.pointer);
    *fresh31 = *fresh30;
    let fresh32 = addr_of_mut!((*parser).buffer.start);
    *fresh32 = *fresh31;
    yaml_free((*parser).tokens.start as *mut libc::c_void);
    let fresh33 = addr_of_mut!((*parser).tokens.end);
    *fresh33 = ptr::null_mut::<yaml_token_t>();
    let fresh34 = addr_of_mut!((*parser).tokens.tail);
    *fresh34 = *fresh33;
    let fresh35 = addr_of_mut!((*parser).tokens.head);
    *fresh35 = *fresh34;
    let fresh36 = addr_of_mut!((*parser).tokens.start);
    *fresh36 = *fresh35;
    yaml_free((*parser).indents.start as *mut libc::c_void);
    let fresh37 = addr_of_mut!((*parser).indents.end);
    *fresh37 = ptr::null_mut::<libc::c_int>();
    let fresh38 = addr_of_mut!((*parser).indents.top);
    *fresh38 = *fresh37;
    let fresh39 = addr_of_mut!((*parser).indents.start);
    *fresh39 = *fresh38;
    yaml_free((*parser).simple_keys.start as *mut libc::c_void);
    let fresh40 = addr_of_mut!((*parser).simple_keys.end);
    *fresh40 = ptr::null_mut::<yaml_simple_key_t>();
    let fresh41 = addr_of_mut!((*parser).simple_keys.top);
    *fresh41 = *fresh40;
    let fresh42 = addr_of_mut!((*parser).simple_keys.start);
    *fresh42 = *fresh41;
    yaml_free((*parser).states.start as *mut libc::c_void);
    let fresh43 = addr_of_mut!((*parser).states.end);
    *fresh43 = ptr::null_mut::<yaml_parser_state_t>();
    let fresh44 = addr_of_mut!((*parser).states.top);
    *fresh44 = *fresh43;
    let fresh45 = addr_of_mut!((*parser).states.start);
    *fresh45 = *fresh44;
    yaml_free((*parser).marks.start as *mut libc::c_void);
    let fresh46 = addr_of_mut!((*parser).marks.end);
    *fresh46 = ptr::null_mut::<yaml_mark_t>();
    let fresh47 = addr_of_mut!((*parser).marks.top);
    *fresh47 = *fresh46;
    let fresh48 = addr_of_mut!((*parser).marks.start);
    *fresh48 = *fresh47;
    yaml_free((*parser).tag_directives.start as *mut libc::c_void);
    let fresh49 = addr_of_mut!((*parser).tag_directives.end);
    *fresh49 = ptr::null_mut::<yaml_tag_directive_t>();
    let fresh50 = addr_of_mut!((*parser).tag_directives.top);
    *fresh50 = *fresh49;
    let fresh51 = addr_of_mut!((*parser).tag_directives.start);
    *fresh51 = *fresh50;
    0_i32
}
pub unsafe fn yaml_parser_delete(parser: *mut yaml_parser_t) {
    __assert!(!parser.is_null());
    yaml_free((*parser).raw_buffer.start as *mut libc::c_void);
    let fresh52 = addr_of_mut!((*parser).raw_buffer.end);
    *fresh52 = ptr::null_mut::<libc::c_uchar>();
    let fresh53 = addr_of_mut!((*parser).raw_buffer.pointer);
    *fresh53 = *fresh52;
    let fresh54 = addr_of_mut!((*parser).raw_buffer.start);
    *fresh54 = *fresh53;
    yaml_free((*parser).buffer.start as *mut libc::c_void);
    let fresh55 = addr_of_mut!((*parser).buffer.end);
    *fresh55 = ptr::null_mut::<yaml_char_t>();
    let fresh56 = addr_of_mut!((*parser).buffer.pointer);
    *fresh56 = *fresh55;
    let fresh57 = addr_of_mut!((*parser).buffer.start);
    *fresh57 = *fresh56;
    while !((*parser).tokens.head == (*parser).tokens.tail) {
        let fresh58 = addr_of_mut!((*parser).tokens.head);
        let fresh59 = *fresh58;
        *fresh58 = (*fresh58).wrapping_offset(1);
        yaml_token_delete(fresh59);
    }
    yaml_free((*parser).tokens.start as *mut libc::c_void);
    let fresh60 = addr_of_mut!((*parser).tokens.end);
    *fresh60 = ptr::null_mut::<yaml_token_t>();
    let fresh61 = addr_of_mut!((*parser).tokens.tail);
    *fresh61 = *fresh60;
    let fresh62 = addr_of_mut!((*parser).tokens.head);
    *fresh62 = *fresh61;
    let fresh63 = addr_of_mut!((*parser).tokens.start);
    *fresh63 = *fresh62;
    yaml_free((*parser).indents.start as *mut libc::c_void);
    let fresh64 = addr_of_mut!((*parser).indents.end);
    *fresh64 = ptr::null_mut::<libc::c_int>();
    let fresh65 = addr_of_mut!((*parser).indents.top);
    *fresh65 = *fresh64;
    let fresh66 = addr_of_mut!((*parser).indents.start);
    *fresh66 = *fresh65;
    yaml_free((*parser).simple_keys.start as *mut libc::c_void);
    let fresh67 = addr_of_mut!((*parser).simple_keys.end);
    *fresh67 = ptr::null_mut::<yaml_simple_key_t>();
    let fresh68 = addr_of_mut!((*parser).simple_keys.top);
    *fresh68 = *fresh67;
    let fresh69 = addr_of_mut!((*parser).simple_keys.start);
    *fresh69 = *fresh68;
    yaml_free((*parser).states.start as *mut libc::c_void);
    let fresh70 = addr_of_mut!((*parser).states.end);
    *fresh70 = ptr::null_mut::<yaml_parser_state_t>();
    let fresh71 = addr_of_mut!((*parser).states.top);
    *fresh71 = *fresh70;
    let fresh72 = addr_of_mut!((*parser).states.start);
    *fresh72 = *fresh71;
    yaml_free((*parser).marks.start as *mut libc::c_void);
    let fresh73 = addr_of_mut!((*parser).marks.end);
    *fresh73 = ptr::null_mut::<yaml_mark_t>();
    let fresh74 = addr_of_mut!((*parser).marks.top);
    *fresh74 = *fresh73;
    let fresh75 = addr_of_mut!((*parser).marks.start);
    *fresh75 = *fresh74;
    while !((*parser).tag_directives.start == (*parser).tag_directives.top) {
        let fresh76 = addr_of_mut!((*parser).tag_directives.top);
        *fresh76 = (*fresh76).wrapping_offset(-1);
        let tag_directive: yaml_tag_directive_t = **fresh76;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    yaml_free((*parser).tag_directives.start as *mut libc::c_void);
    let fresh77 = addr_of_mut!((*parser).tag_directives.end);
    *fresh77 = ptr::null_mut::<yaml_tag_directive_t>();
    let fresh78 = addr_of_mut!((*parser).tag_directives.top);
    *fresh78 = *fresh77;
    let fresh79 = addr_of_mut!((*parser).tag_directives.start);
    *fresh79 = *fresh78;
    memset(
        parser as *mut libc::c_void,
        0_i32,
        size_of::<yaml_parser_t>() as libc::c_ulong,
    );
}
unsafe fn yaml_string_read_handler(
    data: *mut libc::c_void,
    buffer: *mut libc::c_uchar,
    mut size: size_t,
    size_read: *mut size_t,
) -> libc::c_int {
    let parser: *mut yaml_parser_t = data as *mut yaml_parser_t;
    if (*parser).input.string.current == (*parser).input.string.end {
        *size_read = 0_u64;
        return 1_i32;
    }
    if size
        > ((*parser).input.string.end).c_offset_from((*parser).input.string.current) as libc::c_long
            as size_t
    {
        size = ((*parser).input.string.end).c_offset_from((*parser).input.string.current)
            as libc::c_long as size_t;
    }
    memcpy(
        buffer as *mut libc::c_void,
        (*parser).input.string.current as *const libc::c_void,
        size,
    );
    let fresh80 = addr_of_mut!((*parser).input.string.current);
    *fresh80 = (*fresh80).wrapping_offset(size as isize);
    *size_read = size;
    1_i32
}
pub unsafe fn yaml_parser_set_input_string(
    parser: *mut yaml_parser_t,
    input: *const libc::c_uchar,
    size: size_t,
) {
    __assert!(!parser.is_null());
    __assert!(((*parser).read_handler).is_none());
    __assert!(!input.is_null());
    let fresh81 = addr_of_mut!((*parser).read_handler);
    *fresh81 = Some(
        yaml_string_read_handler
            as unsafe fn(*mut libc::c_void, *mut libc::c_uchar, size_t, *mut size_t) -> libc::c_int,
    );
    let fresh82 = addr_of_mut!((*parser).read_handler_data);
    *fresh82 = parser as *mut libc::c_void;
    let fresh83 = addr_of_mut!((*parser).input.string.start);
    *fresh83 = input;
    let fresh84 = addr_of_mut!((*parser).input.string.current);
    *fresh84 = input;
    let fresh85 = addr_of_mut!((*parser).input.string.end);
    *fresh85 = input.wrapping_offset(size as isize);
}
pub unsafe fn yaml_parser_set_input(
    parser: *mut yaml_parser_t,
    handler: Option<yaml_read_handler_t>,
    data: *mut libc::c_void,
) {
    __assert!(!parser.is_null());
    __assert!(((*parser).read_handler).is_none());
    __assert!(handler.is_some());
    let fresh89 = addr_of_mut!((*parser).read_handler);
    *fresh89 = handler;
    let fresh90 = addr_of_mut!((*parser).read_handler_data);
    *fresh90 = data;
}
pub unsafe fn yaml_parser_set_encoding(mut parser: *mut yaml_parser_t, encoding: yaml_encoding_t) {
    __assert!(!parser.is_null());
    __assert!((*parser).encoding as u64 == 0);
    (*parser).encoding = encoding;
}
pub unsafe fn yaml_emitter_initialize(mut emitter: *mut yaml_emitter_t) -> libc::c_int {
    __assert!(!emitter.is_null());
    memset(
        emitter as *mut libc::c_void,
        0_i32,
        size_of::<yaml_emitter_t>() as libc::c_ulong,
    );
    let fresh91 = addr_of_mut!((*emitter).buffer.start);
    *fresh91 = yaml_malloc(16384_u64) as *mut yaml_char_t;
    if !(if !(*fresh91).is_null() {
        let fresh92 = addr_of_mut!((*emitter).buffer.pointer);
        *fresh92 = (*emitter).buffer.start;
        let fresh93 = addr_of_mut!((*emitter).buffer.last);
        *fresh93 = *fresh92;
        let fresh94 = addr_of_mut!((*emitter).buffer.end);
        *fresh94 = ((*emitter).buffer.start).wrapping_offset(16384_isize);
        1_i32
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
        let fresh95 = addr_of_mut!((*emitter).raw_buffer.start);
        *fresh95 = yaml_malloc((16384_i32 * 2_i32 + 2_i32) as size_t) as *mut yaml_char_t;
        if !(if !(*fresh95).is_null() {
            let fresh96 = addr_of_mut!((*emitter).raw_buffer.pointer);
            *fresh96 = (*emitter).raw_buffer.start;
            let fresh97 = addr_of_mut!((*emitter).raw_buffer.last);
            *fresh97 = *fresh96;
            let fresh98 = addr_of_mut!((*emitter).raw_buffer.end);
            *fresh98 =
                ((*emitter).raw_buffer.start).wrapping_offset((16384_i32 * 2_i32 + 2_i32) as isize);
            1_i32
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0)
        {
            let fresh99 = addr_of_mut!((*emitter).states.start);
            *fresh99 = yaml_malloc(
                (16_u64).wrapping_mul(size_of::<yaml_emitter_state_t>() as libc::c_ulong),
            ) as *mut yaml_emitter_state_t;
            if !(if !(*fresh99).is_null() {
                let fresh100 = addr_of_mut!((*emitter).states.top);
                *fresh100 = (*emitter).states.start;
                let fresh101 = addr_of_mut!((*emitter).states.end);
                *fresh101 = ((*emitter).states.start).wrapping_offset(16_isize);
                1_i32
            } else {
                (*emitter).error = YAML_MEMORY_ERROR;
                0_i32
            } == 0)
            {
                let fresh102 = addr_of_mut!((*emitter).events.start);
                *fresh102 =
                    yaml_malloc((16_u64).wrapping_mul(size_of::<yaml_event_t>() as libc::c_ulong))
                        as *mut yaml_event_t;
                if !(if !(*fresh102).is_null() {
                    let fresh103 = addr_of_mut!((*emitter).events.tail);
                    *fresh103 = (*emitter).events.start;
                    let fresh104 = addr_of_mut!((*emitter).events.head);
                    *fresh104 = *fresh103;
                    let fresh105 = addr_of_mut!((*emitter).events.end);
                    *fresh105 = ((*emitter).events.start).wrapping_offset(16_isize);
                    1_i32
                } else {
                    (*emitter).error = YAML_MEMORY_ERROR;
                    0_i32
                } == 0)
                {
                    let fresh106 = addr_of_mut!((*emitter).indents.start);
                    *fresh106 = yaml_malloc(
                        (16_u64).wrapping_mul(size_of::<libc::c_int>() as libc::c_ulong),
                    ) as *mut libc::c_int;
                    if !(if !(*fresh106).is_null() {
                        let fresh107 = addr_of_mut!((*emitter).indents.top);
                        *fresh107 = (*emitter).indents.start;
                        let fresh108 = addr_of_mut!((*emitter).indents.end);
                        *fresh108 = ((*emitter).indents.start).wrapping_offset(16_isize);
                        1_i32
                    } else {
                        (*emitter).error = YAML_MEMORY_ERROR;
                        0_i32
                    } == 0)
                    {
                        let fresh109 = addr_of_mut!((*emitter).tag_directives.start);
                        *fresh109 = yaml_malloc(
                            (16_u64)
                                .wrapping_mul(size_of::<yaml_tag_directive_t>() as libc::c_ulong),
                        ) as *mut yaml_tag_directive_t;
                        if !(if !(*fresh109).is_null() {
                            let fresh110 = addr_of_mut!((*emitter).tag_directives.top);
                            *fresh110 = (*emitter).tag_directives.start;
                            let fresh111 = addr_of_mut!((*emitter).tag_directives.end);
                            *fresh111 = ((*emitter).tag_directives.start).wrapping_offset(16_isize);
                            1_i32
                        } else {
                            (*emitter).error = YAML_MEMORY_ERROR;
                            0_i32
                        } == 0)
                        {
                            return 1_i32;
                        }
                    }
                }
            }
        }
    }
    yaml_free((*emitter).buffer.start as *mut libc::c_void);
    let fresh112 = addr_of_mut!((*emitter).buffer.end);
    *fresh112 = ptr::null_mut::<yaml_char_t>();
    let fresh113 = addr_of_mut!((*emitter).buffer.pointer);
    *fresh113 = *fresh112;
    let fresh114 = addr_of_mut!((*emitter).buffer.start);
    *fresh114 = *fresh113;
    yaml_free((*emitter).raw_buffer.start as *mut libc::c_void);
    let fresh115 = addr_of_mut!((*emitter).raw_buffer.end);
    *fresh115 = ptr::null_mut::<libc::c_uchar>();
    let fresh116 = addr_of_mut!((*emitter).raw_buffer.pointer);
    *fresh116 = *fresh115;
    let fresh117 = addr_of_mut!((*emitter).raw_buffer.start);
    *fresh117 = *fresh116;
    yaml_free((*emitter).states.start as *mut libc::c_void);
    let fresh118 = addr_of_mut!((*emitter).states.end);
    *fresh118 = ptr::null_mut::<yaml_emitter_state_t>();
    let fresh119 = addr_of_mut!((*emitter).states.top);
    *fresh119 = *fresh118;
    let fresh120 = addr_of_mut!((*emitter).states.start);
    *fresh120 = *fresh119;
    yaml_free((*emitter).events.start as *mut libc::c_void);
    let fresh121 = addr_of_mut!((*emitter).events.end);
    *fresh121 = ptr::null_mut::<yaml_event_t>();
    let fresh122 = addr_of_mut!((*emitter).events.tail);
    *fresh122 = *fresh121;
    let fresh123 = addr_of_mut!((*emitter).events.head);
    *fresh123 = *fresh122;
    let fresh124 = addr_of_mut!((*emitter).events.start);
    *fresh124 = *fresh123;
    yaml_free((*emitter).indents.start as *mut libc::c_void);
    let fresh125 = addr_of_mut!((*emitter).indents.end);
    *fresh125 = ptr::null_mut::<libc::c_int>();
    let fresh126 = addr_of_mut!((*emitter).indents.top);
    *fresh126 = *fresh125;
    let fresh127 = addr_of_mut!((*emitter).indents.start);
    *fresh127 = *fresh126;
    yaml_free((*emitter).tag_directives.start as *mut libc::c_void);
    let fresh128 = addr_of_mut!((*emitter).tag_directives.end);
    *fresh128 = ptr::null_mut::<yaml_tag_directive_t>();
    let fresh129 = addr_of_mut!((*emitter).tag_directives.top);
    *fresh129 = *fresh128;
    let fresh130 = addr_of_mut!((*emitter).tag_directives.start);
    *fresh130 = *fresh129;
    0_i32
}
pub unsafe fn yaml_emitter_delete(emitter: *mut yaml_emitter_t) {
    __assert!(!emitter.is_null());
    yaml_free((*emitter).buffer.start as *mut libc::c_void);
    let fresh131 = addr_of_mut!((*emitter).buffer.end);
    *fresh131 = ptr::null_mut::<yaml_char_t>();
    let fresh132 = addr_of_mut!((*emitter).buffer.pointer);
    *fresh132 = *fresh131;
    let fresh133 = addr_of_mut!((*emitter).buffer.start);
    *fresh133 = *fresh132;
    yaml_free((*emitter).raw_buffer.start as *mut libc::c_void);
    let fresh134 = addr_of_mut!((*emitter).raw_buffer.end);
    *fresh134 = ptr::null_mut::<libc::c_uchar>();
    let fresh135 = addr_of_mut!((*emitter).raw_buffer.pointer);
    *fresh135 = *fresh134;
    let fresh136 = addr_of_mut!((*emitter).raw_buffer.start);
    *fresh136 = *fresh135;
    yaml_free((*emitter).states.start as *mut libc::c_void);
    let fresh137 = addr_of_mut!((*emitter).states.end);
    *fresh137 = ptr::null_mut::<yaml_emitter_state_t>();
    let fresh138 = addr_of_mut!((*emitter).states.top);
    *fresh138 = *fresh137;
    let fresh139 = addr_of_mut!((*emitter).states.start);
    *fresh139 = *fresh138;
    while !((*emitter).events.head == (*emitter).events.tail) {
        let fresh140 = addr_of_mut!((*emitter).events.head);
        let fresh141 = *fresh140;
        *fresh140 = (*fresh140).wrapping_offset(1);
        yaml_event_delete(fresh141);
    }
    yaml_free((*emitter).events.start as *mut libc::c_void);
    let fresh142 = addr_of_mut!((*emitter).events.end);
    *fresh142 = ptr::null_mut::<yaml_event_t>();
    let fresh143 = addr_of_mut!((*emitter).events.tail);
    *fresh143 = *fresh142;
    let fresh144 = addr_of_mut!((*emitter).events.head);
    *fresh144 = *fresh143;
    let fresh145 = addr_of_mut!((*emitter).events.start);
    *fresh145 = *fresh144;
    yaml_free((*emitter).indents.start as *mut libc::c_void);
    let fresh146 = addr_of_mut!((*emitter).indents.end);
    *fresh146 = ptr::null_mut::<libc::c_int>();
    let fresh147 = addr_of_mut!((*emitter).indents.top);
    *fresh147 = *fresh146;
    let fresh148 = addr_of_mut!((*emitter).indents.start);
    *fresh148 = *fresh147;
    while !((*emitter).tag_directives.start == (*emitter).tag_directives.top) {
        let fresh149 = addr_of_mut!((*emitter).tag_directives.top);
        *fresh149 = (*fresh149).wrapping_offset(-1);
        let tag_directive: yaml_tag_directive_t = **fresh149;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    yaml_free((*emitter).tag_directives.start as *mut libc::c_void);
    let fresh150 = addr_of_mut!((*emitter).tag_directives.end);
    *fresh150 = ptr::null_mut::<yaml_tag_directive_t>();
    let fresh151 = addr_of_mut!((*emitter).tag_directives.top);
    *fresh151 = *fresh150;
    let fresh152 = addr_of_mut!((*emitter).tag_directives.start);
    *fresh152 = *fresh151;
    yaml_free((*emitter).anchors as *mut libc::c_void);
    memset(
        emitter as *mut libc::c_void,
        0_i32,
        size_of::<yaml_emitter_t>() as libc::c_ulong,
    );
}
unsafe fn yaml_string_write_handler(
    data: *mut libc::c_void,
    buffer: *mut libc::c_uchar,
    size: size_t,
) -> libc::c_int {
    let emitter: *mut yaml_emitter_t = data as *mut yaml_emitter_t;
    if ((*emitter).output.string.size).wrapping_sub(*(*emitter).output.string.size_written) < size {
        memcpy(
            ((*emitter).output.string.buffer)
                .wrapping_offset(*(*emitter).output.string.size_written as isize)
                as *mut libc::c_void,
            buffer as *const libc::c_void,
            ((*emitter).output.string.size).wrapping_sub(*(*emitter).output.string.size_written),
        );
        *(*emitter).output.string.size_written = (*emitter).output.string.size;
        return 0_i32;
    }
    memcpy(
        ((*emitter).output.string.buffer)
            .wrapping_offset(*(*emitter).output.string.size_written as isize)
            as *mut libc::c_void,
        buffer as *const libc::c_void,
        size,
    );
    let fresh153 = addr_of_mut!((*(*emitter).output.string.size_written));
    *fresh153 = (*fresh153 as libc::c_ulong).wrapping_add(size) as size_t as size_t;
    1_i32
}
pub unsafe fn yaml_emitter_set_output_string(
    mut emitter: *mut yaml_emitter_t,
    output: *mut libc::c_uchar,
    size: size_t,
    size_written: *mut size_t,
) {
    __assert!(!emitter.is_null());
    __assert!(((*emitter).write_handler).is_none());
    __assert!(!output.is_null());
    let fresh154 = addr_of_mut!((*emitter).write_handler);
    *fresh154 = Some(
        yaml_string_write_handler
            as unsafe fn(*mut libc::c_void, *mut libc::c_uchar, size_t) -> libc::c_int,
    );
    let fresh155 = addr_of_mut!((*emitter).write_handler_data);
    *fresh155 = emitter as *mut libc::c_void;
    let fresh156 = addr_of_mut!((*emitter).output.string.buffer);
    *fresh156 = output;
    (*emitter).output.string.size = size;
    let fresh157 = addr_of_mut!((*emitter).output.string.size_written);
    *fresh157 = size_written;
    *size_written = 0_u64;
}
pub unsafe fn yaml_emitter_set_output(
    emitter: *mut yaml_emitter_t,
    handler: Option<yaml_write_handler_t>,
    data: *mut libc::c_void,
) {
    __assert!(!emitter.is_null());
    __assert!(((*emitter).write_handler).is_none());
    __assert!(handler.is_some());
    let fresh161 = addr_of_mut!((*emitter).write_handler);
    *fresh161 = handler;
    let fresh162 = addr_of_mut!((*emitter).write_handler_data);
    *fresh162 = data;
}
pub unsafe fn yaml_emitter_set_encoding(
    mut emitter: *mut yaml_emitter_t,
    encoding: yaml_encoding_t,
) {
    __assert!(!emitter.is_null());
    __assert!((*emitter).encoding as u64 == 0);
    (*emitter).encoding = encoding;
}
pub unsafe fn yaml_emitter_set_canonical(mut emitter: *mut yaml_emitter_t, canonical: libc::c_int) {
    __assert!(!emitter.is_null());
    (*emitter).canonical = (canonical != 0_i32) as libc::c_int;
}
pub unsafe fn yaml_emitter_set_indent(mut emitter: *mut yaml_emitter_t, indent: libc::c_int) {
    __assert!(!emitter.is_null());
    (*emitter).best_indent = if 1_i32 < indent && indent < 10_i32 {
        indent
    } else {
        2_i32
    };
}
pub unsafe fn yaml_emitter_set_width(mut emitter: *mut yaml_emitter_t, width: libc::c_int) {
    __assert!(!emitter.is_null());
    (*emitter).best_width = if width >= 0_i32 { width } else { -1_i32 };
}
pub unsafe fn yaml_emitter_set_unicode(mut emitter: *mut yaml_emitter_t, unicode: libc::c_int) {
    __assert!(!emitter.is_null());
    (*emitter).unicode = (unicode != 0_i32) as libc::c_int;
}
pub unsafe fn yaml_emitter_set_break(mut emitter: *mut yaml_emitter_t, line_break: yaml_break_t) {
    __assert!(!emitter.is_null());
    (*emitter).line_break = line_break;
}
pub unsafe fn yaml_token_delete(token: *mut yaml_token_t) {
    __assert!(!token.is_null());
    match (*token).type_0 as libc::c_uint {
        4 => {
            yaml_free((*token).data.tag_directive.handle as *mut libc::c_void);
            yaml_free((*token).data.tag_directive.prefix as *mut libc::c_void);
        }
        18 => {
            yaml_free((*token).data.alias.value as *mut libc::c_void);
        }
        19 => {
            yaml_free((*token).data.anchor.value as *mut libc::c_void);
        }
        20 => {
            yaml_free((*token).data.tag.handle as *mut libc::c_void);
            yaml_free((*token).data.tag.suffix as *mut libc::c_void);
        }
        21 => {
            yaml_free((*token).data.scalar.value as *mut libc::c_void);
        }
        _ => {}
    }
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
}
unsafe fn yaml_check_utf8(start: *const yaml_char_t, length: size_t) -> libc::c_int {
    let end: *const yaml_char_t = start.wrapping_offset(length as isize);
    let mut pointer: *const yaml_char_t = start;
    while pointer < end {
        let mut octet: libc::c_uchar;
        let mut value: libc::c_uint;
        let mut k: size_t;
        octet = *pointer.wrapping_offset(0_isize);
        let width: libc::c_uint = (if octet as libc::c_int & 0x80_i32 == 0_i32 {
            1_i32
        } else if octet as libc::c_int & 0xe0_i32 == 0xc0_i32 {
            2_i32
        } else if octet as libc::c_int & 0xf0_i32 == 0xe0_i32 {
            3_i32
        } else if octet as libc::c_int & 0xf8_i32 == 0xf0_i32 {
            4_i32
        } else {
            0_i32
        }) as libc::c_uint;
        value = (if octet as libc::c_int & 0x80_i32 == 0_i32 {
            octet as libc::c_int & 0x7f_i32
        } else if octet as libc::c_int & 0xe0_i32 == 0xc0_i32 {
            octet as libc::c_int & 0x1f_i32
        } else if octet as libc::c_int & 0xf0_i32 == 0xe0_i32 {
            octet as libc::c_int & 0xf_i32
        } else if octet as libc::c_int & 0xf8_i32 == 0xf0_i32 {
            octet as libc::c_int & 0x7_i32
        } else {
            0_i32
        }) as libc::c_uint;
        if width == 0 {
            return 0_i32;
        }
        if pointer.wrapping_offset(width as isize) > end {
            return 0_i32;
        }
        k = 1_u64;
        while k < width as libc::c_ulong {
            octet = *pointer.wrapping_offset(k as isize);
            if octet as libc::c_int & 0xc0_i32 != 0x80_i32 {
                return 0_i32;
            }
            value =
                (value << 6_i32).wrapping_add((octet as libc::c_int & 0x3f_i32) as libc::c_uint);
            k = k.wrapping_add(1);
        }
        if !(width == 1_u32
            || width == 2_u32 && value >= 0x80_i32 as libc::c_uint
            || width == 3_u32 && value >= 0x800_i32 as libc::c_uint
            || width == 4_u32 && value >= 0x10000_i32 as libc::c_uint)
        {
            return 0_i32;
        }
        pointer = pointer.wrapping_offset(width as isize);
    }
    1_i32
}
pub unsafe fn yaml_stream_start_event_initialize(
    mut event: *mut yaml_event_t,
    encoding: yaml_encoding_t,
) -> libc::c_int {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_STREAM_START_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.stream_start.encoding = encoding;
    1_i32
}
pub unsafe fn yaml_stream_end_event_initialize(mut event: *mut yaml_event_t) -> libc::c_int {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_STREAM_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1_i32
}
pub unsafe fn yaml_document_start_event_initialize(
    mut event: *mut yaml_event_t,
    version_directive: *mut yaml_version_directive_t,
    tag_directives_start: *mut yaml_tag_directive_t,
    tag_directives_end: *mut yaml_tag_directive_t,
    implicit: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut context: Unnamed_17 = Unnamed_17 {
        error: YAML_NO_ERROR,
    };
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut version_directive_copy: *mut yaml_version_directive_t =
        ptr::null_mut::<yaml_version_directive_t>();
    let mut tag_directives_copy: Unnamed_16 = Unnamed_16 {
        start: ptr::null_mut::<yaml_tag_directive_t>(),
        end: ptr::null_mut::<yaml_tag_directive_t>(),
        top: ptr::null_mut::<yaml_tag_directive_t>(),
    };
    let mut value = yaml_tag_directive_t {
        handle: ptr::null_mut::<yaml_char_t>(),
        prefix: ptr::null_mut::<yaml_char_t>(),
    };
    __assert!(!event.is_null());
    __assert!(
        !tag_directives_start.is_null() && !tag_directives_end.is_null()
            || tag_directives_start == tag_directives_end
    );
    if !version_directive.is_null() {
        version_directive_copy = yaml_malloc(size_of::<yaml_version_directive_t>() as libc::c_ulong)
            as *mut yaml_version_directive_t;
        if version_directive_copy.is_null() {
            current_block = 14964981520188694172;
        } else {
            (*version_directive_copy).major = (*version_directive).major;
            (*version_directive_copy).minor = (*version_directive).minor;
            current_block = 1394248824506584008;
        }
    } else {
        current_block = 1394248824506584008;
    }
    match current_block {
        1394248824506584008 => {
            if tag_directives_start != tag_directives_end {
                let mut tag_directive: *mut yaml_tag_directive_t;
                tag_directives_copy.start = yaml_malloc(
                    (16_u64).wrapping_mul(size_of::<yaml_tag_directive_t>() as libc::c_ulong),
                ) as *mut yaml_tag_directive_t;
                if if !(tag_directives_copy.start).is_null() {
                    tag_directives_copy.top = tag_directives_copy.start;
                    tag_directives_copy.end = (tag_directives_copy.start).wrapping_offset(16_isize);
                    1_i32
                } else {
                    context.error = YAML_MEMORY_ERROR;
                    0_i32
                } == 0
                {
                    current_block = 14964981520188694172;
                } else {
                    tag_directive = tag_directives_start;
                    loop {
                        if !(tag_directive != tag_directives_end) {
                            current_block = 16203760046146113240;
                            break;
                        }
                        __assert!(!((*tag_directive).handle).is_null());
                        __assert!(!((*tag_directive).prefix).is_null());
                        if yaml_check_utf8(
                            (*tag_directive).handle,
                            strlen((*tag_directive).handle as *mut libc::c_char),
                        ) == 0
                        {
                            current_block = 14964981520188694172;
                            break;
                        }
                        if yaml_check_utf8(
                            (*tag_directive).prefix,
                            strlen((*tag_directive).prefix as *mut libc::c_char),
                        ) == 0
                        {
                            current_block = 14964981520188694172;
                            break;
                        }
                        value.handle = yaml_strdup((*tag_directive).handle);
                        value.prefix = yaml_strdup((*tag_directive).prefix);
                        if (value.handle).is_null() || (value.prefix).is_null() {
                            current_block = 14964981520188694172;
                            break;
                        }
                        if if tag_directives_copy.top != tag_directives_copy.end
                            || yaml_stack_extend(
                                addr_of_mut!(tag_directives_copy.start) as *mut *mut libc::c_void,
                                addr_of_mut!(tag_directives_copy.top) as *mut *mut libc::c_void,
                                addr_of_mut!(tag_directives_copy.end) as *mut *mut libc::c_void,
                            ) != 0
                        {
                            let fresh163 = tag_directives_copy.top;
                            tag_directives_copy.top = (tag_directives_copy.top).wrapping_offset(1);
                            *fresh163 = value;
                            1_i32
                        } else {
                            context.error = YAML_MEMORY_ERROR;
                            0_i32
                        } == 0
                        {
                            current_block = 14964981520188694172;
                            break;
                        }
                        value.handle = ptr::null_mut::<yaml_char_t>();
                        value.prefix = ptr::null_mut::<yaml_char_t>();
                        tag_directive = tag_directive.wrapping_offset(1);
                    }
                }
            } else {
                current_block = 16203760046146113240;
            }
            match current_block {
                14964981520188694172 => {}
                _ => {
                    memset(
                        event as *mut libc::c_void,
                        0_i32,
                        size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    (*event).type_0 = YAML_DOCUMENT_START_EVENT;
                    (*event).start_mark = mark;
                    (*event).end_mark = mark;
                    let fresh164 = addr_of_mut!((*event).data.document_start.version_directive);
                    *fresh164 = version_directive_copy;
                    let fresh165 = addr_of_mut!((*event).data.document_start.tag_directives.start);
                    *fresh165 = tag_directives_copy.start;
                    let fresh166 = addr_of_mut!((*event).data.document_start.tag_directives.end);
                    *fresh166 = tag_directives_copy.top;
                    (*event).data.document_start.implicit = implicit;
                    return 1_i32;
                }
            }
        }
        _ => {}
    }
    yaml_free(version_directive_copy as *mut libc::c_void);
    while !(tag_directives_copy.start == tag_directives_copy.top) {
        tag_directives_copy.top = (tag_directives_copy.top).wrapping_offset(-1);
        let value_0: yaml_tag_directive_t = *tag_directives_copy.top;
        yaml_free(value_0.handle as *mut libc::c_void);
        yaml_free(value_0.prefix as *mut libc::c_void);
    }
    yaml_free(tag_directives_copy.start as *mut libc::c_void);
    tag_directives_copy.end = ptr::null_mut::<yaml_tag_directive_t>();
    tag_directives_copy.top = tag_directives_copy.end;
    tag_directives_copy.start = tag_directives_copy.top;
    yaml_free(value.handle as *mut libc::c_void);
    yaml_free(value.prefix as *mut libc::c_void);
    0_i32
}
pub unsafe fn yaml_document_end_event_initialize(
    mut event: *mut yaml_event_t,
    implicit: libc::c_int,
) -> libc::c_int {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_DOCUMENT_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.document_end.implicit = implicit;
    1_i32
}
pub unsafe fn yaml_alias_event_initialize(
    mut event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
) -> libc::c_int {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    __assert!(!anchor.is_null());
    if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
        return 0_i32;
    }
    let anchor_copy: *mut yaml_char_t = yaml_strdup(anchor);
    if anchor_copy.is_null() {
        return 0_i32;
    }
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_ALIAS_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    let fresh167 = addr_of_mut!((*event).data.alias.anchor);
    *fresh167 = anchor_copy;
    1_i32
}
pub unsafe fn yaml_scalar_event_initialize(
    mut event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    value: *const yaml_char_t,
    mut length: libc::c_int,
    plain_implicit: libc::c_int,
    quoted_implicit: libc::c_int,
    style: yaml_scalar_style_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut anchor_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut value_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    __assert!(!event.is_null());
    __assert!(!value.is_null());
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
            current_block = 16285396129609901221;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 16285396129609901221;
            } else {
                current_block = 8515828400728868193;
            }
        }
    } else {
        current_block = 8515828400728868193;
    }
    match current_block {
        8515828400728868193 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0 {
                    current_block = 16285396129609901221;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 16285396129609901221;
                    } else {
                        current_block = 12800627514080957624;
                    }
                }
            } else {
                current_block = 12800627514080957624;
            }
            match current_block {
                16285396129609901221 => {}
                _ => {
                    if length < 0_i32 {
                        length = strlen(value as *mut libc::c_char) as libc::c_int;
                    }
                    if !(yaml_check_utf8(value, length as size_t) == 0) {
                        value_copy = yaml_malloc((length + 1_i32) as size_t) as *mut yaml_char_t;
                        if !value_copy.is_null() {
                            memcpy(
                                value_copy as *mut libc::c_void,
                                value as *const libc::c_void,
                                length as libc::c_ulong,
                            );
                            *value_copy.wrapping_offset(length as isize) =
                                '\0' as i32 as yaml_char_t;
                            memset(
                                event as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_0 = YAML_SCALAR_EVENT;
                            (*event).start_mark = mark;
                            (*event).end_mark = mark;
                            let fresh168 = addr_of_mut!((*event).data.scalar.anchor);
                            *fresh168 = anchor_copy;
                            let fresh169 = addr_of_mut!((*event).data.scalar.tag);
                            *fresh169 = tag_copy;
                            let fresh170 = addr_of_mut!((*event).data.scalar.value);
                            *fresh170 = value_copy;
                            (*event).data.scalar.length = length as size_t;
                            (*event).data.scalar.plain_implicit = plain_implicit;
                            (*event).data.scalar.quoted_implicit = quoted_implicit;
                            (*event).data.scalar.style = style;
                            return 1_i32;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    yaml_free(value_copy as *mut libc::c_void);
    0_i32
}
pub unsafe fn yaml_sequence_start_event_initialize(
    mut event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    implicit: libc::c_int,
    style: yaml_sequence_style_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut anchor_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    __assert!(!event.is_null());
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
            current_block = 8817775685815971442;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 8817775685815971442;
            } else {
                current_block = 11006700562992250127;
            }
        }
    } else {
        current_block = 11006700562992250127;
    }
    match current_block {
        11006700562992250127 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0 {
                    current_block = 8817775685815971442;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 8817775685815971442;
                    } else {
                        current_block = 7651349459974463963;
                    }
                }
            } else {
                current_block = 7651349459974463963;
            }
            match current_block {
                8817775685815971442 => {}
                _ => {
                    memset(
                        event as *mut libc::c_void,
                        0_i32,
                        size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    (*event).type_0 = YAML_SEQUENCE_START_EVENT;
                    (*event).start_mark = mark;
                    (*event).end_mark = mark;
                    let fresh171 = addr_of_mut!((*event).data.sequence_start.anchor);
                    *fresh171 = anchor_copy;
                    let fresh172 = addr_of_mut!((*event).data.sequence_start.tag);
                    *fresh172 = tag_copy;
                    (*event).data.sequence_start.implicit = implicit;
                    (*event).data.sequence_start.style = style;
                    return 1_i32;
                }
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    0_i32
}
pub unsafe fn yaml_sequence_end_event_initialize(mut event: *mut yaml_event_t) -> libc::c_int {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_SEQUENCE_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1_i32
}
pub unsafe fn yaml_mapping_start_event_initialize(
    mut event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    implicit: libc::c_int,
    style: yaml_mapping_style_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut anchor_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    __assert!(!event.is_null());
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
            current_block = 14748279734549812740;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 14748279734549812740;
            } else {
                current_block = 11006700562992250127;
            }
        }
    } else {
        current_block = 11006700562992250127;
    }
    match current_block {
        11006700562992250127 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0 {
                    current_block = 14748279734549812740;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 14748279734549812740;
                    } else {
                        current_block = 7651349459974463963;
                    }
                }
            } else {
                current_block = 7651349459974463963;
            }
            match current_block {
                14748279734549812740 => {}
                _ => {
                    memset(
                        event as *mut libc::c_void,
                        0_i32,
                        size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    (*event).type_0 = YAML_MAPPING_START_EVENT;
                    (*event).start_mark = mark;
                    (*event).end_mark = mark;
                    let fresh173 = addr_of_mut!((*event).data.mapping_start.anchor);
                    *fresh173 = anchor_copy;
                    let fresh174 = addr_of_mut!((*event).data.mapping_start.tag);
                    *fresh174 = tag_copy;
                    (*event).data.mapping_start.implicit = implicit;
                    (*event).data.mapping_start.style = style;
                    return 1_i32;
                }
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    0_i32
}
pub unsafe fn yaml_mapping_end_event_initialize(mut event: *mut yaml_event_t) -> libc::c_int {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_MAPPING_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1_i32
}
pub unsafe fn yaml_event_delete(event: *mut yaml_event_t) {
    let mut tag_directive: *mut yaml_tag_directive_t;
    __assert!(!event.is_null());
    match (*event).type_0 as libc::c_uint {
        3 => {
            yaml_free((*event).data.document_start.version_directive as *mut libc::c_void);
            tag_directive = (*event).data.document_start.tag_directives.start;
            while tag_directive != (*event).data.document_start.tag_directives.end {
                yaml_free((*tag_directive).handle as *mut libc::c_void);
                yaml_free((*tag_directive).prefix as *mut libc::c_void);
                tag_directive = tag_directive.wrapping_offset(1);
            }
            yaml_free((*event).data.document_start.tag_directives.start as *mut libc::c_void);
        }
        5 => {
            yaml_free((*event).data.alias.anchor as *mut libc::c_void);
        }
        6 => {
            yaml_free((*event).data.scalar.anchor as *mut libc::c_void);
            yaml_free((*event).data.scalar.tag as *mut libc::c_void);
            yaml_free((*event).data.scalar.value as *mut libc::c_void);
        }
        7 => {
            yaml_free((*event).data.sequence_start.anchor as *mut libc::c_void);
            yaml_free((*event).data.sequence_start.tag as *mut libc::c_void);
        }
        9 => {
            yaml_free((*event).data.mapping_start.anchor as *mut libc::c_void);
            yaml_free((*event).data.mapping_start.tag as *mut libc::c_void);
        }
        _ => {}
    }
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
}
pub unsafe fn yaml_document_initialize(
    mut document: *mut yaml_document_t,
    version_directive: *mut yaml_version_directive_t,
    tag_directives_start: *mut yaml_tag_directive_t,
    tag_directives_end: *mut yaml_tag_directive_t,
    start_implicit: libc::c_int,
    end_implicit: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut context: Unnamed_28 = Unnamed_28 {
        error: YAML_NO_ERROR,
    };
    let mut nodes: Unnamed_27 = Unnamed_27 {
        start: ptr::null_mut::<yaml_node_t>(),
        end: ptr::null_mut::<yaml_node_t>(),
        top: ptr::null_mut::<yaml_node_t>(),
    };
    let mut version_directive_copy: *mut yaml_version_directive_t =
        ptr::null_mut::<yaml_version_directive_t>();
    let mut tag_directives_copy: Unnamed_26 = Unnamed_26 {
        start: ptr::null_mut::<yaml_tag_directive_t>(),
        end: ptr::null_mut::<yaml_tag_directive_t>(),
        top: ptr::null_mut::<yaml_tag_directive_t>(),
    };
    let mut value = yaml_tag_directive_t {
        handle: ptr::null_mut::<yaml_char_t>(),
        prefix: ptr::null_mut::<yaml_char_t>(),
    };
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!document.is_null());
    __assert!(
        !tag_directives_start.is_null() && !tag_directives_end.is_null()
            || tag_directives_start == tag_directives_end
    );
    nodes.start = yaml_malloc((16_u64).wrapping_mul(size_of::<yaml_node_t>() as libc::c_ulong))
        as *mut yaml_node_t;
    if !(if !(nodes.start).is_null() {
        nodes.top = nodes.start;
        nodes.end = (nodes.start).wrapping_offset(16_isize);
        1_i32
    } else {
        context.error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
        if !version_directive.is_null() {
            version_directive_copy =
                yaml_malloc(size_of::<yaml_version_directive_t>() as libc::c_ulong)
                    as *mut yaml_version_directive_t;
            if version_directive_copy.is_null() {
                current_block = 8142820162064489797;
            } else {
                (*version_directive_copy).major = (*version_directive).major;
                (*version_directive_copy).minor = (*version_directive).minor;
                current_block = 7746791466490516765;
            }
        } else {
            current_block = 7746791466490516765;
        }
        match current_block {
            8142820162064489797 => {}
            _ => {
                if tag_directives_start != tag_directives_end {
                    let mut tag_directive: *mut yaml_tag_directive_t;
                    tag_directives_copy.start =
                        yaml_malloc(
                            (16_u64)
                                .wrapping_mul(size_of::<yaml_tag_directive_t>() as libc::c_ulong),
                        ) as *mut yaml_tag_directive_t;
                    if if !(tag_directives_copy.start).is_null() {
                        tag_directives_copy.top = tag_directives_copy.start;
                        tag_directives_copy.end =
                            (tag_directives_copy.start).wrapping_offset(16_isize);
                        1_i32
                    } else {
                        context.error = YAML_MEMORY_ERROR;
                        0_i32
                    } == 0
                    {
                        current_block = 8142820162064489797;
                    } else {
                        tag_directive = tag_directives_start;
                        loop {
                            if !(tag_directive != tag_directives_end) {
                                current_block = 14818589718467733107;
                                break;
                            }
                            __assert!(!((*tag_directive).handle).is_null());
                            __assert!(!((*tag_directive).prefix).is_null());
                            if yaml_check_utf8(
                                (*tag_directive).handle,
                                strlen((*tag_directive).handle as *mut libc::c_char),
                            ) == 0
                            {
                                current_block = 8142820162064489797;
                                break;
                            }
                            if yaml_check_utf8(
                                (*tag_directive).prefix,
                                strlen((*tag_directive).prefix as *mut libc::c_char),
                            ) == 0
                            {
                                current_block = 8142820162064489797;
                                break;
                            }
                            value.handle = yaml_strdup((*tag_directive).handle);
                            value.prefix = yaml_strdup((*tag_directive).prefix);
                            if (value.handle).is_null() || (value.prefix).is_null() {
                                current_block = 8142820162064489797;
                                break;
                            }
                            if if tag_directives_copy.top != tag_directives_copy.end
                                || yaml_stack_extend(
                                    addr_of_mut!(tag_directives_copy.start)
                                        as *mut *mut libc::c_void,
                                    addr_of_mut!(tag_directives_copy.top) as *mut *mut libc::c_void,
                                    addr_of_mut!(tag_directives_copy.end) as *mut *mut libc::c_void,
                                ) != 0
                            {
                                let fresh175 = tag_directives_copy.top;
                                tag_directives_copy.top =
                                    (tag_directives_copy.top).wrapping_offset(1);
                                *fresh175 = value;
                                1_i32
                            } else {
                                context.error = YAML_MEMORY_ERROR;
                                0_i32
                            } == 0
                            {
                                current_block = 8142820162064489797;
                                break;
                            }
                            value.handle = ptr::null_mut::<yaml_char_t>();
                            value.prefix = ptr::null_mut::<yaml_char_t>();
                            tag_directive = tag_directive.wrapping_offset(1);
                        }
                    }
                } else {
                    current_block = 14818589718467733107;
                }
                match current_block {
                    8142820162064489797 => {}
                    _ => {
                        memset(
                            document as *mut libc::c_void,
                            0_i32,
                            size_of::<yaml_document_t>() as libc::c_ulong,
                        );
                        let fresh176 = addr_of_mut!((*document).nodes.start);
                        *fresh176 = nodes.start;
                        let fresh177 = addr_of_mut!((*document).nodes.end);
                        *fresh177 = nodes.end;
                        let fresh178 = addr_of_mut!((*document).nodes.top);
                        *fresh178 = nodes.start;
                        let fresh179 = addr_of_mut!((*document).version_directive);
                        *fresh179 = version_directive_copy;
                        let fresh180 = addr_of_mut!((*document).tag_directives.start);
                        *fresh180 = tag_directives_copy.start;
                        let fresh181 = addr_of_mut!((*document).tag_directives.end);
                        *fresh181 = tag_directives_copy.top;
                        (*document).start_implicit = start_implicit;
                        (*document).end_implicit = end_implicit;
                        (*document).start_mark = mark;
                        (*document).end_mark = mark;
                        return 1_i32;
                    }
                }
            }
        }
    }
    yaml_free(nodes.start as *mut libc::c_void);
    nodes.end = ptr::null_mut::<yaml_node_t>();
    nodes.top = nodes.end;
    nodes.start = nodes.top;
    yaml_free(version_directive_copy as *mut libc::c_void);
    while !(tag_directives_copy.start == tag_directives_copy.top) {
        tag_directives_copy.top = (tag_directives_copy.top).wrapping_offset(-1);
        let value_0: yaml_tag_directive_t = *tag_directives_copy.top;
        yaml_free(value_0.handle as *mut libc::c_void);
        yaml_free(value_0.prefix as *mut libc::c_void);
    }
    yaml_free(tag_directives_copy.start as *mut libc::c_void);
    tag_directives_copy.end = ptr::null_mut::<yaml_tag_directive_t>();
    tag_directives_copy.top = tag_directives_copy.end;
    tag_directives_copy.start = tag_directives_copy.top;
    yaml_free(value.handle as *mut libc::c_void);
    yaml_free(value.prefix as *mut libc::c_void);
    0_i32
}
pub unsafe fn yaml_document_delete(document: *mut yaml_document_t) {
    let mut tag_directive: *mut yaml_tag_directive_t;
    __assert!(!document.is_null());
    while !((*document).nodes.start == (*document).nodes.top) {
        let fresh182 = addr_of_mut!((*document).nodes.top);
        *fresh182 = (*fresh182).wrapping_offset(-1);
        let mut node: yaml_node_t = **fresh182;
        yaml_free(node.tag as *mut libc::c_void);
        match node.type_0 as libc::c_uint {
            1 => {
                yaml_free(node.data.scalar.value as *mut libc::c_void);
            }
            2 => {
                yaml_free(node.data.sequence.items.start as *mut libc::c_void);
                node.data.sequence.items.end = ptr::null_mut::<yaml_node_item_t>();
                node.data.sequence.items.top = node.data.sequence.items.end;
                node.data.sequence.items.start = node.data.sequence.items.top;
            }
            3 => {
                yaml_free(node.data.mapping.pairs.start as *mut libc::c_void);
                node.data.mapping.pairs.end = ptr::null_mut::<yaml_node_pair_t>();
                node.data.mapping.pairs.top = node.data.mapping.pairs.end;
                node.data.mapping.pairs.start = node.data.mapping.pairs.top;
            }
            _ => {
                __assert!(false);
            }
        }
    }
    yaml_free((*document).nodes.start as *mut libc::c_void);
    let fresh183 = addr_of_mut!((*document).nodes.end);
    *fresh183 = ptr::null_mut::<yaml_node_t>();
    let fresh184 = addr_of_mut!((*document).nodes.top);
    *fresh184 = *fresh183;
    let fresh185 = addr_of_mut!((*document).nodes.start);
    *fresh185 = *fresh184;
    yaml_free((*document).version_directive as *mut libc::c_void);
    tag_directive = (*document).tag_directives.start;
    while tag_directive != (*document).tag_directives.end {
        yaml_free((*tag_directive).handle as *mut libc::c_void);
        yaml_free((*tag_directive).prefix as *mut libc::c_void);
        tag_directive = tag_directive.wrapping_offset(1);
    }
    yaml_free((*document).tag_directives.start as *mut libc::c_void);
    memset(
        document as *mut libc::c_void,
        0_i32,
        size_of::<yaml_document_t>() as libc::c_ulong,
    );
}
pub unsafe fn yaml_document_get_node(
    document: *mut yaml_document_t,
    index: libc::c_int,
) -> *mut yaml_node_t {
    __assert!(!document.is_null());
    if index > 0_i32
        && ((*document).nodes.start).wrapping_offset(index as isize) <= (*document).nodes.top
    {
        return ((*document).nodes.start)
            .wrapping_offset(index as isize)
            .wrapping_offset(-(1_isize));
    }
    ptr::null_mut::<yaml_node_t>()
}
pub unsafe fn yaml_document_get_root_node(document: *mut yaml_document_t) -> *mut yaml_node_t {
    __assert!(!document.is_null());
    if (*document).nodes.top != (*document).nodes.start {
        return (*document).nodes.start;
    }
    ptr::null_mut::<yaml_node_t>()
}
pub unsafe fn yaml_document_add_scalar(
    document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    value: *const yaml_char_t,
    mut length: libc::c_int,
    style: yaml_scalar_style_t,
) -> libc::c_int {
    let mut context: Unnamed_29 = Unnamed_29 {
        error: YAML_NO_ERROR,
    };
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut value_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut node = MaybeUninit::<yaml_node_t>::uninit();
    let node = node.as_mut_ptr();
    __assert!(!document.is_null());
    __assert!(!value.is_null());
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char as *mut yaml_char_t;
    }
    if !(yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0) {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            if length < 0_i32 {
                length = strlen(value as *mut libc::c_char) as libc::c_int;
            }
            if !(yaml_check_utf8(value, length as size_t) == 0) {
                value_copy = yaml_malloc((length + 1_i32) as size_t) as *mut yaml_char_t;
                if !value_copy.is_null() {
                    memcpy(
                        value_copy as *mut libc::c_void,
                        value as *const libc::c_void,
                        length as libc::c_ulong,
                    );
                    *value_copy.wrapping_offset(length as isize) = '\0' as i32 as yaml_char_t;
                    memset(
                        node as *mut libc::c_void,
                        0_i32,
                        size_of::<yaml_node_t>() as libc::c_ulong,
                    );
                    (*node).type_0 = YAML_SCALAR_NODE;
                    (*node).tag = tag_copy;
                    (*node).start_mark = mark;
                    (*node).end_mark = mark;
                    (*node).data.scalar.value = value_copy;
                    (*node).data.scalar.length = length as size_t;
                    (*node).data.scalar.style = style;
                    if !(if (*document).nodes.top != (*document).nodes.end
                        || yaml_stack_extend(
                            addr_of_mut!((*document).nodes.start) as *mut *mut libc::c_void,
                            addr_of_mut!((*document).nodes.top) as *mut *mut libc::c_void,
                            addr_of_mut!((*document).nodes.end) as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh186 = addr_of_mut!((*document).nodes.top);
                        let fresh187 = *fresh186;
                        *fresh186 = (*fresh186).wrapping_offset(1);
                        ptr::copy_nonoverlapping(node, fresh187, 1);
                        1_i32
                    } else {
                        context.error = YAML_MEMORY_ERROR;
                        0_i32
                    } == 0)
                    {
                        return ((*document).nodes.top).c_offset_from((*document).nodes.start)
                            as libc::c_long as libc::c_int;
                    }
                }
            }
        }
    }
    yaml_free(tag_copy as *mut libc::c_void);
    yaml_free(value_copy as *mut libc::c_void);
    0_i32
}
pub unsafe fn yaml_document_add_sequence(
    document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    style: yaml_sequence_style_t,
) -> libc::c_int {
    let mut context: Unnamed_31 = Unnamed_31 {
        error: YAML_NO_ERROR,
    };
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut items: Unnamed_30 = Unnamed_30 {
        start: ptr::null_mut::<yaml_node_item_t>(),
        end: ptr::null_mut::<yaml_node_item_t>(),
        top: ptr::null_mut::<yaml_node_item_t>(),
    };
    let mut node = MaybeUninit::<yaml_node_t>::uninit();
    let node = node.as_mut_ptr();
    __assert!(!document.is_null());
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:seq\0" as *const u8 as *const libc::c_char as *mut yaml_char_t;
    }
    if !(yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0) {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            items.start =
                yaml_malloc((16_u64).wrapping_mul(size_of::<yaml_node_item_t>() as libc::c_ulong))
                    as *mut yaml_node_item_t;
            if !(if !(items.start).is_null() {
                items.top = items.start;
                items.end = (items.start).wrapping_offset(16_isize);
                1_i32
            } else {
                context.error = YAML_MEMORY_ERROR;
                0_i32
            } == 0)
            {
                memset(
                    node as *mut libc::c_void,
                    0_i32,
                    size_of::<yaml_node_t>() as libc::c_ulong,
                );
                (*node).type_0 = YAML_SEQUENCE_NODE;
                (*node).tag = tag_copy;
                (*node).start_mark = mark;
                (*node).end_mark = mark;
                (*node).data.sequence.items.start = items.start;
                (*node).data.sequence.items.end = items.end;
                (*node).data.sequence.items.top = items.start;
                (*node).data.sequence.style = style;
                if !(if (*document).nodes.top != (*document).nodes.end
                    || yaml_stack_extend(
                        addr_of_mut!((*document).nodes.start) as *mut *mut libc::c_void,
                        addr_of_mut!((*document).nodes.top) as *mut *mut libc::c_void,
                        addr_of_mut!((*document).nodes.end) as *mut *mut libc::c_void,
                    ) != 0
                {
                    let fresh188 = addr_of_mut!((*document).nodes.top);
                    let fresh189 = *fresh188;
                    *fresh188 = (*fresh188).wrapping_offset(1);
                    ptr::copy_nonoverlapping(node, fresh189, 1);
                    1_i32
                } else {
                    context.error = YAML_MEMORY_ERROR;
                    0_i32
                } == 0)
                {
                    return ((*document).nodes.top).c_offset_from((*document).nodes.start)
                        as libc::c_long as libc::c_int;
                }
            }
        }
    }
    yaml_free(items.start as *mut libc::c_void);
    items.end = ptr::null_mut::<yaml_node_item_t>();
    items.top = items.end;
    items.start = items.top;
    yaml_free(tag_copy as *mut libc::c_void);
    0_i32
}
pub unsafe fn yaml_document_add_mapping(
    document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    style: yaml_mapping_style_t,
) -> libc::c_int {
    let mut context: Unnamed_33 = Unnamed_33 {
        error: YAML_NO_ERROR,
    };
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut pairs: Unnamed_32 = Unnamed_32 {
        start: ptr::null_mut::<yaml_node_pair_t>(),
        end: ptr::null_mut::<yaml_node_pair_t>(),
        top: ptr::null_mut::<yaml_node_pair_t>(),
    };
    let mut node = MaybeUninit::<yaml_node_t>::uninit();
    let node = node.as_mut_ptr();
    __assert!(!document.is_null());
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:map\0" as *const u8 as *const libc::c_char as *mut yaml_char_t;
    }
    if !(yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0) {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            pairs.start =
                yaml_malloc((16_u64).wrapping_mul(size_of::<yaml_node_pair_t>() as libc::c_ulong))
                    as *mut yaml_node_pair_t;
            if !(if !(pairs.start).is_null() {
                pairs.top = pairs.start;
                pairs.end = (pairs.start).wrapping_offset(16_isize);
                1_i32
            } else {
                context.error = YAML_MEMORY_ERROR;
                0_i32
            } == 0)
            {
                memset(
                    node as *mut libc::c_void,
                    0_i32,
                    size_of::<yaml_node_t>() as libc::c_ulong,
                );
                (*node).type_0 = YAML_MAPPING_NODE;
                (*node).tag = tag_copy;
                (*node).start_mark = mark;
                (*node).end_mark = mark;
                (*node).data.mapping.pairs.start = pairs.start;
                (*node).data.mapping.pairs.end = pairs.end;
                (*node).data.mapping.pairs.top = pairs.start;
                (*node).data.mapping.style = style;
                if !(if (*document).nodes.top != (*document).nodes.end
                    || yaml_stack_extend(
                        addr_of_mut!((*document).nodes.start) as *mut *mut libc::c_void,
                        addr_of_mut!((*document).nodes.top) as *mut *mut libc::c_void,
                        addr_of_mut!((*document).nodes.end) as *mut *mut libc::c_void,
                    ) != 0
                {
                    let fresh190 = addr_of_mut!((*document).nodes.top);
                    let fresh191 = *fresh190;
                    *fresh190 = (*fresh190).wrapping_offset(1);
                    ptr::copy_nonoverlapping(node, fresh191, 1);
                    1_i32
                } else {
                    context.error = YAML_MEMORY_ERROR;
                    0_i32
                } == 0)
                {
                    return ((*document).nodes.top).c_offset_from((*document).nodes.start)
                        as libc::c_long as libc::c_int;
                }
            }
        }
    }
    yaml_free(pairs.start as *mut libc::c_void);
    pairs.end = ptr::null_mut::<yaml_node_pair_t>();
    pairs.top = pairs.end;
    pairs.start = pairs.top;
    yaml_free(tag_copy as *mut libc::c_void);
    0_i32
}
pub unsafe fn yaml_document_append_sequence_item(
    document: *mut yaml_document_t,
    sequence: libc::c_int,
    item: libc::c_int,
) -> libc::c_int {
    let mut context: Unnamed_34 = Unnamed_34 {
        error: YAML_NO_ERROR,
    };
    __assert!(!document.is_null());
    __assert!(
        sequence > 0_i32
            && ((*document).nodes.start).wrapping_offset(sequence as isize)
                <= (*document).nodes.top
    );
    __assert!(
        (*((*document).nodes.start).wrapping_offset((sequence - 1_i32) as isize)).type_0
            as libc::c_uint
            == YAML_SEQUENCE_NODE as libc::c_int as libc::c_uint
    );
    __assert!(
        item > 0_i32
            && ((*document).nodes.start).wrapping_offset(item as isize) <= (*document).nodes.top
    );
    if if (*((*document).nodes.start).wrapping_offset((sequence - 1_i32) as isize))
        .data
        .sequence
        .items
        .top
        != (*((*document).nodes.start).wrapping_offset((sequence - 1_i32) as isize))
            .data
            .sequence
            .items
            .end
        || yaml_stack_extend(
            addr_of_mut!(
                (*((*document).nodes.start).wrapping_offset((sequence - 1_i32) as isize))
                    .data
                    .sequence
                    .items
                    .start
            ) as *mut *mut libc::c_void,
            addr_of_mut!(
                (*((*document).nodes.start).wrapping_offset((sequence - 1_i32) as isize))
                    .data
                    .sequence
                    .items
                    .top
            ) as *mut *mut libc::c_void,
            addr_of_mut!(
                (*((*document).nodes.start).wrapping_offset((sequence - 1_i32) as isize))
                    .data
                    .sequence
                    .items
                    .end
            ) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh192 = addr_of_mut!(
            (*((*document).nodes.start).wrapping_offset((sequence - 1_i32) as isize))
                .data
                .sequence
                .items
                .top
        );
        let fresh193 = *fresh192;
        *fresh192 = (*fresh192).wrapping_offset(1);
        *fresh193 = item;
        1_i32
    } else {
        context.error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}
pub unsafe fn yaml_document_append_mapping_pair(
    document: *mut yaml_document_t,
    mapping: libc::c_int,
    key: libc::c_int,
    value: libc::c_int,
) -> libc::c_int {
    let mut context: Unnamed_35 = Unnamed_35 {
        error: YAML_NO_ERROR,
    };
    __assert!(!document.is_null());
    __assert!(
        mapping > 0_i32
            && ((*document).nodes.start).wrapping_offset(mapping as isize) <= (*document).nodes.top
    );
    __assert!(
        (*((*document).nodes.start).wrapping_offset((mapping - 1_i32) as isize)).type_0
            as libc::c_uint
            == YAML_MAPPING_NODE as libc::c_int as libc::c_uint
    );
    __assert!(
        key > 0_i32
            && ((*document).nodes.start).wrapping_offset(key as isize) <= (*document).nodes.top
    );
    __assert!(
        value > 0_i32
            && ((*document).nodes.start).wrapping_offset(value as isize) <= (*document).nodes.top
    );
    let pair = yaml_node_pair_t { key, value };
    if if (*((*document).nodes.start).wrapping_offset((mapping - 1_i32) as isize))
        .data
        .mapping
        .pairs
        .top
        != (*((*document).nodes.start).wrapping_offset((mapping - 1_i32) as isize))
            .data
            .mapping
            .pairs
            .end
        || yaml_stack_extend(
            addr_of_mut!(
                (*((*document).nodes.start).wrapping_offset((mapping - 1_i32) as isize))
                    .data
                    .mapping
                    .pairs
                    .start
            ) as *mut *mut libc::c_void,
            addr_of_mut!(
                (*((*document).nodes.start).wrapping_offset((mapping - 1_i32) as isize))
                    .data
                    .mapping
                    .pairs
                    .top
            ) as *mut *mut libc::c_void,
            addr_of_mut!(
                (*((*document).nodes.start).wrapping_offset((mapping - 1_i32) as isize))
                    .data
                    .mapping
                    .pairs
                    .end
            ) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh194 = addr_of_mut!(
            (*((*document).nodes.start).wrapping_offset((mapping - 1_i32) as isize))
                .data
                .mapping
                .pairs
                .top
        );
        let fresh195 = *fresh194;
        *fresh194 = (*fresh194).wrapping_offset(1);
        *fresh195 = pair;
        1_i32
    } else {
        context.error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}
