use crate::api::{yaml_free, yaml_queue_extend, yaml_stack_extend, yaml_strdup};
use crate::externs::{strcmp, strlen, strncmp};
use crate::yaml::{size_t, yaml_char_t, yaml_string_t};
use crate::{
    libc, yaml_emitter_flush, yaml_emitter_t, yaml_event_delete, yaml_event_t, yaml_scalar_style_t,
    yaml_tag_directive_t, yaml_version_directive_t, PointerExt, YAML_ANY_SCALAR_STYLE,
    YAML_CRLN_BREAK, YAML_CR_BREAK, YAML_DOCUMENT_END_EVENT, YAML_DOCUMENT_START_EVENT,
    YAML_DOUBLE_QUOTED_SCALAR_STYLE, YAML_EMITTER_ERROR, YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE,
    YAML_EMIT_BLOCK_MAPPING_KEY_STATE, YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE,
    YAML_EMIT_BLOCK_MAPPING_VALUE_STATE, YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE,
    YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE, YAML_EMIT_DOCUMENT_CONTENT_STATE,
    YAML_EMIT_DOCUMENT_END_STATE, YAML_EMIT_DOCUMENT_START_STATE, YAML_EMIT_END_STATE,
    YAML_EMIT_FIRST_DOCUMENT_START_STATE, YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE,
    YAML_EMIT_FLOW_MAPPING_KEY_STATE, YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE,
    YAML_EMIT_FLOW_MAPPING_VALUE_STATE, YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE,
    YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE, YAML_FLOW_MAPPING_STYLE, YAML_FLOW_SEQUENCE_STYLE,
    YAML_FOLDED_SCALAR_STYLE, YAML_LITERAL_SCALAR_STYLE, YAML_LN_BREAK, YAML_MAPPING_END_EVENT,
    YAML_MAPPING_START_EVENT, YAML_MEMORY_ERROR, YAML_PLAIN_SCALAR_STYLE, YAML_SEQUENCE_END_EVENT,
    YAML_SEQUENCE_START_EVENT, YAML_SINGLE_QUOTED_SCALAR_STYLE, YAML_STREAM_END_EVENT,
    YAML_STREAM_START_EVENT, YAML_UTF8_ENCODING,
};
use core::ptr::{self, addr_of_mut};

macro_rules! FLUSH {
    ($emitter:expr) => {
        ((*$emitter).buffer.pointer).wrapping_offset(5_isize) < (*$emitter).buffer.end
            || yaml_emitter_flush($emitter) != 0
    };
}

macro_rules! PUT {
    ($emitter:expr, $value:expr) => {
        FLUSH!($emitter) && {
            let fresh40 = addr_of_mut!((*$emitter).buffer.pointer);
            let fresh41 = *fresh40;
            *fresh40 = (*fresh40).wrapping_offset(1);
            *fresh41 = $value as i32 as yaml_char_t;
            let fresh42 = addr_of_mut!((*$emitter).column);
            *fresh42 += 1;
            true
        }
    };
}

macro_rules! PUT_BREAK {
    ($emitter:expr) => {
        FLUSH!($emitter) && {
            if (*$emitter).line_break as libc::c_uint
                == YAML_CR_BREAK as libc::c_int as libc::c_uint
            {
                let fresh62 = addr_of_mut!((*$emitter).buffer.pointer);
                let fresh63 = *fresh62;
                *fresh62 = (*fresh62).wrapping_offset(1);
                *fresh63 = '\r' as i32 as yaml_char_t;
            } else if (*$emitter).line_break as libc::c_uint
                == YAML_LN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh64 = addr_of_mut!((*$emitter).buffer.pointer);
                let fresh65 = *fresh64;
                *fresh64 = (*fresh64).wrapping_offset(1);
                *fresh65 = '\n' as i32 as yaml_char_t;
            } else if (*$emitter).line_break as libc::c_uint
                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh66 = addr_of_mut!((*$emitter).buffer.pointer);
                let fresh67 = *fresh66;
                *fresh66 = (*fresh66).wrapping_offset(1);
                *fresh67 = '\r' as i32 as yaml_char_t;
                let fresh68 = addr_of_mut!((*$emitter).buffer.pointer);
                let fresh69 = *fresh68;
                *fresh68 = (*fresh68).wrapping_offset(1);
                *fresh69 = '\n' as i32 as yaml_char_t;
            };
            (*$emitter).column = 0_i32;
            let fresh70 = addr_of_mut!((*$emitter).line);
            *fresh70 += 1;
            true
        }
    };
}

macro_rules! WRITE {
    ($emitter:expr, $string:expr) => {
        FLUSH!($emitter) && {
            COPY!((*$emitter).buffer, $string);
            let fresh107 = addr_of_mut!((*$emitter).column);
            *fresh107 += 1;
            true
        }
    };
}

macro_rules! WRITE_BREAK {
    ($emitter:expr, $string:expr) => {
        FLUSH!($emitter)
            && (if *$string.pointer as libc::c_int == '\n' as i32 as yaml_char_t as libc::c_int {
                let _ = PUT_BREAK!($emitter);
                $string.pointer = $string.pointer.wrapping_offset(1);
                1_i32
            } else {
                COPY!((*$emitter).buffer, $string);
                (*$emitter).column = 0_i32;
                let fresh300 = addr_of_mut!((*$emitter).line);
                *fresh300 += 1;
                1_i32
            }) != 0
    };
}

unsafe fn yaml_emitter_set_emitter_error(
    mut emitter: *mut yaml_emitter_t,
    problem: *const libc::c_char,
) -> libc::c_int {
    (*emitter).error = YAML_EMITTER_ERROR;
    let fresh0 = addr_of_mut!((*emitter).problem);
    *fresh0 = problem;
    0_i32
}

/// Emit an event.
///
/// The event object may be generated using the yaml_parser_parse() function.
/// The emitter takes the responsibility for the event object and destroys its
/// content after it is emitted. The event object is destroyed even if the
/// function fails.
///
/// Returns 1 if the function succeeded, 0 on error.
#[must_use]
pub unsafe fn yaml_emitter_emit(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if ENQUEUE!(emitter, (*emitter).events, *event) == 0 {
        yaml_event_delete(event);
        return 0_i32;
    }
    while yaml_emitter_need_more_events(emitter) == 0 {
        if yaml_emitter_analyze_event(emitter, (*emitter).events.head) == 0 {
            return 0_i32;
        }
        if yaml_emitter_state_machine(emitter, (*emitter).events.head) == 0 {
            return 0_i32;
        }
        yaml_event_delete(addr_of_mut!(DEQUEUE!((*emitter).events)));
    }
    1_i32
}

unsafe fn yaml_emitter_need_more_events(emitter: *mut yaml_emitter_t) -> libc::c_int {
    let mut level: libc::c_int = 0_i32;
    let mut event: *mut yaml_event_t;
    if QUEUE_EMPTY!((*emitter).events) {
        return 1_i32;
    }
    let accumulate = match (*(*emitter).events.head).type_ as libc::c_uint {
        3 => 1_i32,
        7 => 2_i32,
        9 => 3_i32,
        _ => return 0_i32,
    };
    if ((*emitter).events.tail).c_offset_from((*emitter).events.head) as libc::c_long
        > accumulate as libc::c_long
    {
        return 0_i32;
    }
    event = (*emitter).events.head;
    while event != (*emitter).events.tail {
        match (*event).type_ as libc::c_uint {
            1 | 3 | 7 | 9 => {
                level += 1_i32;
            }
            2 | 4 | 8 | 10 => {
                level -= 1_i32;
            }
            _ => {}
        }
        if level == 0 {
            return 0_i32;
        }
        event = event.wrapping_offset(1);
    }
    1_i32
}

unsafe fn yaml_emitter_append_tag_directive(
    mut emitter: *mut yaml_emitter_t,
    value: yaml_tag_directive_t,
    allow_duplicates: libc::c_int,
) -> libc::c_int {
    let mut tag_directive: *mut yaml_tag_directive_t;
    let mut copy = yaml_tag_directive_t {
        handle: ptr::null_mut::<yaml_char_t>(),
        prefix: ptr::null_mut::<yaml_char_t>(),
    };
    tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        if strcmp(
            value.handle as *mut libc::c_char,
            (*tag_directive).handle as *mut libc::c_char,
        ) == 0_i32
        {
            if allow_duplicates != 0 {
                return 1_i32;
            }
            return yaml_emitter_set_emitter_error(
                emitter,
                b"duplicate %TAG directive\0" as *const u8 as *const libc::c_char,
            );
        }
        tag_directive = tag_directive.wrapping_offset(1);
    }
    copy.handle = yaml_strdup(value.handle);
    copy.prefix = yaml_strdup(value.prefix);
    if copy.handle.is_null() || copy.prefix.is_null() {
        (*emitter).error = YAML_MEMORY_ERROR;
    } else if !(PUSH!(emitter, (*emitter).tag_directives, copy) == 0) {
        return 1_i32;
    }
    yaml_free(copy.handle as *mut libc::c_void);
    yaml_free(copy.prefix as *mut libc::c_void);
    0_i32
}

unsafe fn yaml_emitter_increase_indent(
    mut emitter: *mut yaml_emitter_t,
    flow: libc::c_int,
    indentless: libc::c_int,
) -> libc::c_int {
    if PUSH!(emitter, (*emitter).indents, (*emitter).indent) == 0 {
        return 0_i32;
    }
    if (*emitter).indent < 0_i32 {
        (*emitter).indent = if flow != 0 {
            (*emitter).best_indent
        } else {
            0_i32
        };
    } else if indentless == 0 {
        (*emitter).indent += (*emitter).best_indent;
    }
    1_i32
}

unsafe fn yaml_emitter_state_machine(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    match (*emitter).state as libc::c_uint {
        0 => return yaml_emitter_emit_stream_start(emitter, event),
        1 => return yaml_emitter_emit_document_start(emitter, event, 1_i32),
        2 => return yaml_emitter_emit_document_start(emitter, event, 0_i32),
        3 => return yaml_emitter_emit_document_content(emitter, event),
        4 => return yaml_emitter_emit_document_end(emitter, event),
        5 => {
            return yaml_emitter_emit_flow_sequence_item(emitter, event, 1_i32);
        }
        6 => {
            return yaml_emitter_emit_flow_sequence_item(emitter, event, 0_i32);
        }
        7 => return yaml_emitter_emit_flow_mapping_key(emitter, event, 1_i32),
        8 => return yaml_emitter_emit_flow_mapping_key(emitter, event, 0_i32),
        9 => {
            return yaml_emitter_emit_flow_mapping_value(emitter, event, 1_i32);
        }
        10 => {
            return yaml_emitter_emit_flow_mapping_value(emitter, event, 0_i32);
        }
        11 => {
            return yaml_emitter_emit_block_sequence_item(emitter, event, 1_i32);
        }
        12 => {
            return yaml_emitter_emit_block_sequence_item(emitter, event, 0_i32);
        }
        13 => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 1_i32);
        }
        14 => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 0_i32);
        }
        15 => {
            return yaml_emitter_emit_block_mapping_value(emitter, event, 1_i32);
        }
        16 => {
            return yaml_emitter_emit_block_mapping_value(emitter, event, 0_i32);
        }
        17 => {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"expected nothing after STREAM-END\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    0_i32
}

unsafe fn yaml_emitter_emit_stream_start(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    (*emitter).open_ended = 0_i32;
    if (*event).type_ as libc::c_uint == YAML_STREAM_START_EVENT as libc::c_int as libc::c_uint {
        if (*emitter).encoding as u64 == 0 {
            (*emitter).encoding = (*event).data.stream_start.encoding;
        }
        if (*emitter).encoding as u64 == 0 {
            (*emitter).encoding = YAML_UTF8_ENCODING;
        }
        if (*emitter).best_indent < 2_i32 || (*emitter).best_indent > 9_i32 {
            (*emitter).best_indent = 2_i32;
        }
        if (*emitter).best_width >= 0_i32 && (*emitter).best_width <= (*emitter).best_indent * 2_i32
        {
            (*emitter).best_width = 80_i32;
        }
        if (*emitter).best_width < 0_i32 {
            (*emitter).best_width = 2147483647_i32;
        }
        if (*emitter).line_break as u64 == 0 {
            (*emitter).line_break = YAML_LN_BREAK;
        }
        (*emitter).indent = -1_i32;
        (*emitter).line = 0_i32;
        (*emitter).column = 0_i32;
        (*emitter).whitespace = 1_i32;
        (*emitter).indention = 1_i32;
        if (*emitter).encoding as libc::c_uint != YAML_UTF8_ENCODING as libc::c_int as libc::c_uint
        {
            if yaml_emitter_write_bom(emitter) == 0 {
                return 0_i32;
            }
        }
        (*emitter).state = YAML_EMIT_FIRST_DOCUMENT_START_STATE;
        return 1_i32;
    }
    yaml_emitter_set_emitter_error(
        emitter,
        b"expected STREAM-START\0" as *const u8 as *const libc::c_char,
    )
}

unsafe fn yaml_emitter_emit_document_start(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: libc::c_int,
) -> libc::c_int {
    if (*event).type_ as libc::c_uint == YAML_DOCUMENT_START_EVENT as libc::c_int as libc::c_uint {
        let mut default_tag_directives: [yaml_tag_directive_t; 3] = [
            yaml_tag_directive_t {
                handle: b"!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
                prefix: b"!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
            },
            yaml_tag_directive_t {
                handle: b"!!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
                prefix: b"tag:yaml.org,2002:\0" as *const u8 as *const libc::c_char
                    as *mut yaml_char_t,
            },
            yaml_tag_directive_t {
                handle: ptr::null_mut::<yaml_char_t>(),
                prefix: ptr::null_mut::<yaml_char_t>(),
            },
        ];
        let mut tag_directive: *mut yaml_tag_directive_t;
        let mut implicit: libc::c_int;
        if !((*event).data.document_start.version_directive).is_null() {
            if yaml_emitter_analyze_version_directive(
                emitter,
                *(*event).data.document_start.version_directive,
            ) == 0
            {
                return 0_i32;
            }
        }
        tag_directive = (*event).data.document_start.tag_directives.start;
        while tag_directive != (*event).data.document_start.tag_directives.end {
            if yaml_emitter_analyze_tag_directive(emitter, *tag_directive) == 0 {
                return 0_i32;
            }
            if yaml_emitter_append_tag_directive(emitter, *tag_directive, 0_i32) == 0 {
                return 0_i32;
            }
            tag_directive = tag_directive.wrapping_offset(1);
        }
        tag_directive = default_tag_directives.as_mut_ptr();
        while !((*tag_directive).handle).is_null() {
            if yaml_emitter_append_tag_directive(emitter, *tag_directive, 1_i32) == 0 {
                return 0_i32;
            }
            tag_directive = tag_directive.wrapping_offset(1);
        }
        implicit = (*event).data.document_start.implicit;
        if first == 0 || (*emitter).canonical != 0 {
            implicit = 0_i32;
        }
        if (!((*event).data.document_start.version_directive).is_null()
            || (*event).data.document_start.tag_directives.start
                != (*event).data.document_start.tag_directives.end)
            && (*emitter).open_ended != 0
        {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0" as *const u8 as *const libc::c_char,
                1_i32,
                0_i32,
                0_i32,
            ) == 0
            {
                return 0_i32;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0_i32;
            }
        }
        (*emitter).open_ended = 0_i32;
        if !((*event).data.document_start.version_directive).is_null() {
            implicit = 0_i32;
            if yaml_emitter_write_indicator(
                emitter,
                b"%YAML\0" as *const u8 as *const libc::c_char,
                1_i32,
                0_i32,
                0_i32,
            ) == 0
            {
                return 0_i32;
            }
            if (*(*event).data.document_start.version_directive).minor == 1_i32 {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"1.1\0" as *const u8 as *const libc::c_char,
                    1_i32,
                    0_i32,
                    0_i32,
                ) == 0
                {
                    return 0_i32;
                }
            } else if yaml_emitter_write_indicator(
                emitter,
                b"1.2\0" as *const u8 as *const libc::c_char,
                1_i32,
                0_i32,
                0_i32,
            ) == 0
            {
                return 0_i32;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0_i32;
            }
        }
        if (*event).data.document_start.tag_directives.start
            != (*event).data.document_start.tag_directives.end
        {
            implicit = 0_i32;
            tag_directive = (*event).data.document_start.tag_directives.start;
            while tag_directive != (*event).data.document_start.tag_directives.end {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"%TAG\0" as *const u8 as *const libc::c_char,
                    1_i32,
                    0_i32,
                    0_i32,
                ) == 0
                {
                    return 0_i32;
                }
                if yaml_emitter_write_tag_handle(
                    emitter,
                    (*tag_directive).handle,
                    strlen((*tag_directive).handle as *mut libc::c_char),
                ) == 0
                {
                    return 0_i32;
                }
                if yaml_emitter_write_tag_content(
                    emitter,
                    (*tag_directive).prefix,
                    strlen((*tag_directive).prefix as *mut libc::c_char),
                    1_i32,
                ) == 0
                {
                    return 0_i32;
                }
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                tag_directive = tag_directive.wrapping_offset(1);
            }
        }
        if yaml_emitter_check_empty_document(emitter) != 0 {
            implicit = 0_i32;
        }
        if implicit == 0 {
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0_i32;
            }
            if yaml_emitter_write_indicator(
                emitter,
                b"---\0" as *const u8 as *const libc::c_char,
                1_i32,
                0_i32,
                0_i32,
            ) == 0
            {
                return 0_i32;
            }
            if (*emitter).canonical != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
            }
        }
        (*emitter).state = YAML_EMIT_DOCUMENT_CONTENT_STATE;
        (*emitter).open_ended = 0_i32;
        return 1_i32;
    } else if (*event).type_ as libc::c_uint == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint
    {
        if (*emitter).open_ended == 2_i32 {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0" as *const u8 as *const libc::c_char,
                1_i32,
                0_i32,
                0_i32,
            ) == 0
            {
                return 0_i32;
            }
            (*emitter).open_ended = 0_i32;
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0_i32;
            }
        }
        if yaml_emitter_flush(emitter) == 0 {
            return 0_i32;
        }
        (*emitter).state = YAML_EMIT_END_STATE;
        return 1_i32;
    }
    yaml_emitter_set_emitter_error(
        emitter,
        b"expected DOCUMENT-START or STREAM-END\0" as *const u8 as *const libc::c_char,
    )
}

unsafe fn yaml_emitter_emit_document_content(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if PUSH!(emitter, (*emitter).states, YAML_EMIT_DOCUMENT_END_STATE) == 0 {
        return 0_i32;
    }
    yaml_emitter_emit_node(emitter, event, 1_i32, 0_i32, 0_i32, 0_i32)
}

unsafe fn yaml_emitter_emit_document_end(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if (*event).type_ as libc::c_uint == YAML_DOCUMENT_END_EVENT as libc::c_int as libc::c_uint {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0_i32;
        }
        if (*event).data.document_end.implicit == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0" as *const u8 as *const libc::c_char,
                1_i32,
                0_i32,
                0_i32,
            ) == 0
            {
                return 0_i32;
            }
            (*emitter).open_ended = 0_i32;
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0_i32;
            }
        } else if (*emitter).open_ended == 0 {
            (*emitter).open_ended = 1_i32;
        }
        if yaml_emitter_flush(emitter) == 0 {
            return 0_i32;
        }
        (*emitter).state = YAML_EMIT_DOCUMENT_START_STATE;
        while !STACK_EMPTY!((*emitter).tag_directives) {
            let tag_directive = POP!((*emitter).tag_directives);
            yaml_free(tag_directive.handle as *mut libc::c_void);
            yaml_free(tag_directive.prefix as *mut libc::c_void);
        }
        return 1_i32;
    }
    yaml_emitter_set_emitter_error(
        emitter,
        b"expected DOCUMENT-END\0" as *const u8 as *const libc::c_char,
    )
}

unsafe fn yaml_emitter_emit_flow_sequence_item(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: libc::c_int,
) -> libc::c_int {
    if first != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b"[\0" as *const u8 as *const libc::c_char,
            1_i32,
            1_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
        if yaml_emitter_increase_indent(emitter, 1_i32, 0_i32) == 0 {
            return 0_i32;
        }
        let fresh12 = addr_of_mut!((*emitter).flow_level);
        *fresh12 += 1;
    }
    if (*event).type_ as libc::c_uint == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint {
        let fresh13 = addr_of_mut!((*emitter).flow_level);
        *fresh13 -= 1;
        (*emitter).indent = POP!((*emitter).indents);
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b",\0" as *const u8 as *const libc::c_char,
                0_i32,
                0_i32,
                0_i32,
            ) == 0
            {
                return 0_i32;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0_i32;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b"]\0" as *const u8 as *const libc::c_char,
            0_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
        (*emitter).state = POP!((*emitter).states);
        return 1_i32;
    }
    if first == 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b",\0" as *const u8 as *const libc::c_char,
            0_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
    }
    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0_i32;
        }
    }
    if PUSH!(
        emitter,
        (*emitter).states,
        YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE
    ) == 0
    {
        return 0_i32;
    }
    yaml_emitter_emit_node(emitter, event, 0_i32, 1_i32, 0_i32, 0_i32)
}

unsafe fn yaml_emitter_emit_flow_mapping_key(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: libc::c_int,
) -> libc::c_int {
    if first != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b"{\0" as *const u8 as *const libc::c_char,
            1_i32,
            1_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
        if yaml_emitter_increase_indent(emitter, 1_i32, 0_i32) == 0 {
            return 0_i32;
        }
        let fresh18 = addr_of_mut!((*emitter).flow_level);
        *fresh18 += 1;
    }
    if (*event).type_ as libc::c_uint == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint {
        let fresh19 = addr_of_mut!((*emitter).flow_level);
        *fresh19 -= 1;
        (*emitter).indent = POP!((*emitter).indents);
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b",\0" as *const u8 as *const libc::c_char,
                0_i32,
                0_i32,
                0_i32,
            ) == 0
            {
                return 0_i32;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0_i32;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b"}\0" as *const u8 as *const libc::c_char,
            0_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
        (*emitter).state = POP!((*emitter).states);
        return 1_i32;
    }
    if first == 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b",\0" as *const u8 as *const libc::c_char,
            0_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
    }
    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0_i32;
        }
    }
    if (*emitter).canonical == 0 && yaml_emitter_check_simple_key(emitter) != 0 {
        if PUSH!(
            emitter,
            (*emitter).states,
            YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE
        ) == 0
        {
            return 0_i32;
        }
        yaml_emitter_emit_node(emitter, event, 0_i32, 0_i32, 1_i32, 1_i32)
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"?\0" as *const u8 as *const libc::c_char,
            1_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
        if PUSH!(
            emitter,
            (*emitter).states,
            YAML_EMIT_FLOW_MAPPING_VALUE_STATE
        ) == 0
        {
            return 0_i32;
        }
        yaml_emitter_emit_node(emitter, event, 0_i32, 0_i32, 1_i32, 0_i32)
    }
}

unsafe fn yaml_emitter_emit_flow_mapping_value(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    simple: libc::c_int,
) -> libc::c_int {
    if simple != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const libc::c_char,
            0_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
    } else {
        if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0_i32;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const libc::c_char,
            1_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
    }
    if PUSH!(emitter, (*emitter).states, YAML_EMIT_FLOW_MAPPING_KEY_STATE) == 0 {
        return 0_i32;
    }
    yaml_emitter_emit_node(emitter, event, 0_i32, 0_i32, 1_i32, 0_i32)
}

unsafe fn yaml_emitter_emit_block_sequence_item(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: libc::c_int,
) -> libc::c_int {
    if first != 0 {
        if yaml_emitter_increase_indent(
            emitter,
            0_i32,
            ((*emitter).mapping_context != 0 && (*emitter).indention == 0) as libc::c_int,
        ) == 0
        {
            return 0_i32;
        }
    }
    if (*event).type_ as libc::c_uint == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint {
        (*emitter).indent = POP!((*emitter).indents);
        (*emitter).state = POP!((*emitter).states);
        return 1_i32;
    }
    if yaml_emitter_write_indent(emitter) == 0 {
        return 0_i32;
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"-\0" as *const u8 as *const libc::c_char,
        1_i32,
        0_i32,
        1_i32,
    ) == 0
    {
        return 0_i32;
    }
    if PUSH!(
        emitter,
        (*emitter).states,
        YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE
    ) == 0
    {
        return 0_i32;
    }
    yaml_emitter_emit_node(emitter, event, 0_i32, 1_i32, 0_i32, 0_i32)
}

unsafe fn yaml_emitter_emit_block_mapping_key(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: libc::c_int,
) -> libc::c_int {
    if first != 0 {
        if yaml_emitter_increase_indent(emitter, 0_i32, 0_i32) == 0 {
            return 0_i32;
        }
    }
    if (*event).type_ as libc::c_uint == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint {
        (*emitter).indent = POP!((*emitter).indents);
        (*emitter).state = POP!((*emitter).states);
        return 1_i32;
    }
    if yaml_emitter_write_indent(emitter) == 0 {
        return 0_i32;
    }
    if yaml_emitter_check_simple_key(emitter) != 0 {
        if PUSH!(
            emitter,
            (*emitter).states,
            YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE
        ) == 0
        {
            return 0_i32;
        }
        yaml_emitter_emit_node(emitter, event, 0_i32, 0_i32, 1_i32, 1_i32)
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"?\0" as *const u8 as *const libc::c_char,
            1_i32,
            0_i32,
            1_i32,
        ) == 0
        {
            return 0_i32;
        }
        if PUSH!(
            emitter,
            (*emitter).states,
            YAML_EMIT_BLOCK_MAPPING_VALUE_STATE
        ) == 0
        {
            return 0_i32;
        }
        yaml_emitter_emit_node(emitter, event, 0_i32, 0_i32, 1_i32, 0_i32)
    }
}

unsafe fn yaml_emitter_emit_block_mapping_value(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    simple: libc::c_int,
) -> libc::c_int {
    if simple != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const libc::c_char,
            0_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
    } else {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0_i32;
        }
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const libc::c_char,
            1_i32,
            0_i32,
            1_i32,
        ) == 0
        {
            return 0_i32;
        }
    }
    if PUSH!(
        emitter,
        (*emitter).states,
        YAML_EMIT_BLOCK_MAPPING_KEY_STATE
    ) == 0
    {
        return 0_i32;
    }
    yaml_emitter_emit_node(emitter, event, 0_i32, 0_i32, 1_i32, 0_i32)
}

unsafe fn yaml_emitter_emit_node(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    root: libc::c_int,
    sequence: libc::c_int,
    mapping: libc::c_int,
    simple_key: libc::c_int,
) -> libc::c_int {
    (*emitter).root_context = root;
    (*emitter).sequence_context = sequence;
    (*emitter).mapping_context = mapping;
    (*emitter).simple_key_context = simple_key;
    match (*event).type_ as libc::c_uint {
        5 => yaml_emitter_emit_alias(emitter, event),
        6 => yaml_emitter_emit_scalar(emitter, event),
        7 => yaml_emitter_emit_sequence_start(emitter, event),
        9 => yaml_emitter_emit_mapping_start(emitter, event),
        _ => yaml_emitter_set_emitter_error(
            emitter,
            b"expected SCALAR, SEQUENCE-START, MAPPING-START, or ALIAS\0" as *const u8
                as *const libc::c_char,
        ),
    }
}

unsafe fn yaml_emitter_emit_alias(
    mut emitter: *mut yaml_emitter_t,
    _event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0_i32;
    }
    if (*emitter).simple_key_context != 0 {
        if !(PUT!(emitter, ' ')) {
            return 0_i32;
        }
    }
    (*emitter).state = POP!((*emitter).states);
    1_i32
}

unsafe fn yaml_emitter_emit_scalar(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_select_scalar_style(emitter, event) == 0 {
        return 0_i32;
    }
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0_i32;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0_i32;
    }
    if yaml_emitter_increase_indent(emitter, 1_i32, 0_i32) == 0 {
        return 0_i32;
    }
    if yaml_emitter_process_scalar(emitter) == 0 {
        return 0_i32;
    }
    (*emitter).indent = POP!((*emitter).indents);
    (*emitter).state = POP!((*emitter).states);
    1_i32
}

unsafe fn yaml_emitter_emit_sequence_start(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0_i32;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0_i32;
    }
    if (*emitter).flow_level != 0
        || (*emitter).canonical != 0
        || (*event).data.sequence_start.style as libc::c_uint
            == YAML_FLOW_SEQUENCE_STYLE as libc::c_int as libc::c_uint
        || yaml_emitter_check_empty_sequence(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE;
    }
    1_i32
}

unsafe fn yaml_emitter_emit_mapping_start(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0_i32;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0_i32;
    }
    if (*emitter).flow_level != 0
        || (*emitter).canonical != 0
        || (*event).data.mapping_start.style as libc::c_uint
            == YAML_FLOW_MAPPING_STYLE as libc::c_int as libc::c_uint
        || yaml_emitter_check_empty_mapping(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE;
    }
    1_i32
}

unsafe fn yaml_emitter_check_empty_document(_emitter: *mut yaml_emitter_t) -> libc::c_int {
    0_i32
}

unsafe fn yaml_emitter_check_empty_sequence(emitter: *mut yaml_emitter_t) -> libc::c_int {
    if (((*emitter).events.tail).c_offset_from((*emitter).events.head) as libc::c_long) < 2_i64 {
        return 0_i32;
    }
    ((*((*emitter).events.head)).type_ as libc::c_uint
        == YAML_SEQUENCE_START_EVENT as libc::c_int as libc::c_uint
        && (*((*emitter).events.head).wrapping_offset(1_isize)).type_ as libc::c_uint
            == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint) as libc::c_int
}

unsafe fn yaml_emitter_check_empty_mapping(emitter: *mut yaml_emitter_t) -> libc::c_int {
    if (((*emitter).events.tail).c_offset_from((*emitter).events.head) as libc::c_long) < 2_i64 {
        return 0_i32;
    }
    ((*((*emitter).events.head)).type_ as libc::c_uint
        == YAML_MAPPING_START_EVENT as libc::c_int as libc::c_uint
        && (*((*emitter).events.head).wrapping_offset(1_isize)).type_ as libc::c_uint
            == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint) as libc::c_int
}

unsafe fn yaml_emitter_check_simple_key(emitter: *mut yaml_emitter_t) -> libc::c_int {
    let event: *mut yaml_event_t = (*emitter).events.head;
    let mut length: size_t = 0_u64;
    match (*event).type_ as libc::c_uint {
        5 => {
            length = (length as libc::c_ulong).wrapping_add((*emitter).anchor_data.anchor_length)
                as size_t as size_t;
        }
        6 => {
            if (*emitter).scalar_data.multiline != 0 {
                return 0_i32;
            }
            length = (length as libc::c_ulong).wrapping_add(
                ((*emitter).anchor_data.anchor_length)
                    .wrapping_add((*emitter).tag_data.handle_length)
                    .wrapping_add((*emitter).tag_data.suffix_length)
                    .wrapping_add((*emitter).scalar_data.length),
            ) as size_t as size_t;
        }
        7 => {
            if yaml_emitter_check_empty_sequence(emitter) == 0 {
                return 0_i32;
            }
            length = (length as libc::c_ulong).wrapping_add(
                ((*emitter).anchor_data.anchor_length)
                    .wrapping_add((*emitter).tag_data.handle_length)
                    .wrapping_add((*emitter).tag_data.suffix_length),
            ) as size_t as size_t;
        }
        9 => {
            if yaml_emitter_check_empty_mapping(emitter) == 0 {
                return 0_i32;
            }
            length = (length as libc::c_ulong).wrapping_add(
                ((*emitter).anchor_data.anchor_length)
                    .wrapping_add((*emitter).tag_data.handle_length)
                    .wrapping_add((*emitter).tag_data.suffix_length),
            ) as size_t as size_t;
        }
        _ => return 0_i32,
    }
    if length > 128_u64 {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_emitter_select_scalar_style(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    let mut style: yaml_scalar_style_t = (*event).data.scalar.style;
    let no_tag: libc::c_int = (((*emitter).tag_data.handle).is_null()
        && ((*emitter).tag_data.suffix).is_null()) as libc::c_int;
    if no_tag != 0
        && (*event).data.scalar.plain_implicit == 0
        && (*event).data.scalar.quoted_implicit == 0
    {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"neither tag nor implicit flags are specified\0" as *const u8 as *const libc::c_char,
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
            || (*emitter).flow_level == 0 && (*emitter).scalar_data.block_plain_allowed == 0
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
    if style as libc::c_uint == YAML_SINGLE_QUOTED_SCALAR_STYLE as libc::c_int as libc::c_uint {
        if (*emitter).scalar_data.single_quoted_allowed == 0 {
            style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
        }
    }
    if style as libc::c_uint == YAML_LITERAL_SCALAR_STYLE as libc::c_int as libc::c_uint
        || style as libc::c_uint == YAML_FOLDED_SCALAR_STYLE as libc::c_int as libc::c_uint
    {
        if (*emitter).scalar_data.block_allowed == 0
            || (*emitter).flow_level != 0
            || (*emitter).simple_key_context != 0
        {
            style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
        }
    }
    if no_tag != 0
        && (*event).data.scalar.quoted_implicit == 0
        && style as libc::c_uint != YAML_PLAIN_SCALAR_STYLE as libc::c_int as libc::c_uint
    {
        let fresh46 = addr_of_mut!((*emitter).tag_data.handle);
        *fresh46 = b"!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t;
        (*emitter).tag_data.handle_length = 1_u64;
    }
    (*emitter).scalar_data.style = style;
    1_i32
}

unsafe fn yaml_emitter_process_anchor(emitter: *mut yaml_emitter_t) -> libc::c_int {
    if ((*emitter).anchor_data.anchor).is_null() {
        return 1_i32;
    }
    if yaml_emitter_write_indicator(
        emitter,
        if (*emitter).anchor_data.alias != 0 {
            b"*\0" as *const u8 as *const libc::c_char
        } else {
            b"&\0" as *const u8 as *const libc::c_char
        },
        1_i32,
        0_i32,
        0_i32,
    ) == 0
    {
        return 0_i32;
    }
    yaml_emitter_write_anchor(
        emitter,
        (*emitter).anchor_data.anchor,
        (*emitter).anchor_data.anchor_length,
    )
}

unsafe fn yaml_emitter_process_tag(emitter: *mut yaml_emitter_t) -> libc::c_int {
    if ((*emitter).tag_data.handle).is_null() && ((*emitter).tag_data.suffix).is_null() {
        return 1_i32;
    }
    if !((*emitter).tag_data.handle).is_null() {
        if yaml_emitter_write_tag_handle(
            emitter,
            (*emitter).tag_data.handle,
            (*emitter).tag_data.handle_length,
        ) == 0
        {
            return 0_i32;
        }
        if !((*emitter).tag_data.suffix).is_null() {
            if yaml_emitter_write_tag_content(
                emitter,
                (*emitter).tag_data.suffix,
                (*emitter).tag_data.suffix_length,
                0_i32,
            ) == 0
            {
                return 0_i32;
            }
        }
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"!<\0" as *const u8 as *const libc::c_char,
            1_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
        if yaml_emitter_write_tag_content(
            emitter,
            (*emitter).tag_data.suffix,
            (*emitter).tag_data.suffix_length,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
        if yaml_emitter_write_indicator(
            emitter,
            b">\0" as *const u8 as *const libc::c_char,
            0_i32,
            0_i32,
            0_i32,
        ) == 0
        {
            return 0_i32;
        }
    }
    1_i32
}

unsafe fn yaml_emitter_process_scalar(emitter: *mut yaml_emitter_t) -> libc::c_int {
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
    0_i32
}

unsafe fn yaml_emitter_analyze_version_directive(
    emitter: *mut yaml_emitter_t,
    version_directive: yaml_version_directive_t,
) -> libc::c_int {
    if version_directive.major != 1_i32
        || version_directive.minor != 1_i32 && version_directive.minor != 2_i32
    {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"incompatible %YAML directive\0" as *const u8 as *const libc::c_char,
        );
    }
    1_i32
}

unsafe fn yaml_emitter_analyze_tag_directive(
    emitter: *mut yaml_emitter_t,
    tag_directive: yaml_tag_directive_t,
) -> libc::c_int {
    let handle_length: size_t = strlen(tag_directive.handle as *mut libc::c_char);
    let prefix_length: size_t = strlen(tag_directive.prefix as *mut libc::c_char);
    let mut handle = STRING_ASSIGN!(tag_directive.handle, handle_length);
    let prefix = STRING_ASSIGN!(tag_directive.prefix, prefix_length);
    if handle.start == handle.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must not be empty\0" as *const u8 as *const libc::c_char,
        );
    }
    if *handle.start as libc::c_int != '!' as i32 {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must start with '!'\0" as *const u8 as *const libc::c_char,
        );
    }
    if *handle.end.wrapping_offset(-1_isize) as libc::c_int != '!' as i32 {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must end with '!'\0" as *const u8 as *const libc::c_char,
        );
    }
    handle.pointer = handle.pointer.wrapping_offset(1);
    while handle.pointer < handle.end.wrapping_offset(-(1_isize)) {
        if !IS_ALPHA!(handle) {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"tag handle must contain alphanumerical characters only\0" as *const u8
                    as *const libc::c_char,
            );
        }
        MOVE!(handle);
    }
    if prefix.start == prefix.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag prefix must not be empty\0" as *const u8 as *const libc::c_char,
        );
    }
    1_i32
}

unsafe fn yaml_emitter_analyze_anchor(
    mut emitter: *mut yaml_emitter_t,
    anchor: *mut yaml_char_t,
    alias: libc::c_int,
) -> libc::c_int {
    let anchor_length: size_t = strlen(anchor as *mut libc::c_char);
    let mut string = STRING_ASSIGN!(anchor, anchor_length);
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
        if !IS_ALPHA!(string) {
            return yaml_emitter_set_emitter_error(
                emitter,
                if alias != 0 {
                    b"alias value must contain alphanumerical characters only\0" as *const u8
                        as *const libc::c_char
                } else {
                    b"anchor value must contain alphanumerical characters only\0" as *const u8
                        as *const libc::c_char
                },
            );
        }
        MOVE!(string);
    }
    let fresh47 = addr_of_mut!((*emitter).anchor_data.anchor);
    *fresh47 = string.start;
    (*emitter).anchor_data.anchor_length =
        string.end.c_offset_from(string.start) as libc::c_long as size_t;
    (*emitter).anchor_data.alias = alias;
    1_i32
}

unsafe fn yaml_emitter_analyze_tag(
    mut emitter: *mut yaml_emitter_t,
    tag: *mut yaml_char_t,
) -> libc::c_int {
    let mut tag_directive: *mut yaml_tag_directive_t;
    let tag_length: size_t = strlen(tag as *mut libc::c_char);
    let string = STRING_ASSIGN!(tag, tag_length);
    if string.start == string.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag value must not be empty\0" as *const u8 as *const libc::c_char,
        );
    }
    tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        let prefix_length: size_t = strlen((*tag_directive).prefix as *mut libc::c_char);
        if prefix_length < string.end.c_offset_from(string.start) as libc::c_long as size_t
            && strncmp(
                (*tag_directive).prefix as *mut libc::c_char,
                string.start as *mut libc::c_char,
                prefix_length,
            ) == 0_i32
        {
            let fresh48 = addr_of_mut!((*emitter).tag_data.handle);
            *fresh48 = (*tag_directive).handle;
            (*emitter).tag_data.handle_length =
                strlen((*tag_directive).handle as *mut libc::c_char);
            let fresh49 = addr_of_mut!((*emitter).tag_data.suffix);
            *fresh49 = string.start.wrapping_offset(prefix_length as isize);
            (*emitter).tag_data.suffix_length = ((string.end).c_offset_from(string.start)
                as libc::c_long as libc::c_ulong)
                .wrapping_sub(prefix_length);
            return 1_i32;
        }
        tag_directive = tag_directive.wrapping_offset(1);
    }
    let fresh50 = addr_of_mut!((*emitter).tag_data.suffix);
    *fresh50 = string.start;
    (*emitter).tag_data.suffix_length =
        string.end.c_offset_from(string.start) as libc::c_long as size_t;
    1_i32
}

unsafe fn yaml_emitter_analyze_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut block_indicators: libc::c_int = 0_i32;
    let mut flow_indicators: libc::c_int = 0_i32;
    let mut line_breaks: libc::c_int = 0_i32;
    let mut special_characters: libc::c_int = 0_i32;
    let mut leading_space: libc::c_int = 0_i32;
    let mut leading_break: libc::c_int = 0_i32;
    let mut trailing_space: libc::c_int = 0_i32;
    let mut trailing_break: libc::c_int = 0_i32;
    let mut break_space: libc::c_int = 0_i32;
    let mut space_break: libc::c_int = 0_i32;
    let mut preceded_by_whitespace: libc::c_int;
    let mut followed_by_whitespace: libc::c_int;
    let mut previous_space: libc::c_int = 0_i32;
    let mut previous_break: libc::c_int = 0_i32;
    let mut string = STRING_ASSIGN!(value, length);
    let fresh51 = addr_of_mut!((*emitter).scalar_data.value);
    *fresh51 = value;
    (*emitter).scalar_data.length = length;
    if string.start == string.end {
        (*emitter).scalar_data.multiline = 0_i32;
        (*emitter).scalar_data.flow_plain_allowed = 0_i32;
        (*emitter).scalar_data.block_plain_allowed = 1_i32;
        (*emitter).scalar_data.single_quoted_allowed = 1_i32;
        (*emitter).scalar_data.block_allowed = 0_i32;
        return 1_i32;
    }
    if *string.pointer as libc::c_int == '-' as i32 as yaml_char_t as libc::c_int
        && *string.pointer.wrapping_offset(1_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && *string.pointer.wrapping_offset(2_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        || *string.pointer as libc::c_int == '.' as i32 as yaml_char_t as libc::c_int
            && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
            && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
    {
        block_indicators = 1_i32;
        flow_indicators = 1_i32;
    }
    preceded_by_whitespace = 1_i32;
    followed_by_whitespace = IS_BLANKZ_AT!(
        string,
        if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
            1_i32
        } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
            2_i32
        } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
            3_i32
        } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
            4_i32
        } else {
            0_i32
        }
    ) as libc::c_int;
    while string.pointer != string.end {
        if string.start == string.pointer {
            if *string.pointer as libc::c_int == '#' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == ',' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '[' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == ']' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '{' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '}' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '&' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '*' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '!' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '|' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '>' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '\'' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '"' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '%' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '@' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '`' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1_i32;
                block_indicators = 1_i32;
            }
            if *string.pointer as libc::c_int == '?' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == ':' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1_i32;
                if followed_by_whitespace != 0 {
                    block_indicators = 1_i32;
                }
            }
            if *string.pointer as libc::c_int == '-' as i32 as yaml_char_t as libc::c_int
                && followed_by_whitespace != 0
            {
                flow_indicators = 1_i32;
                block_indicators = 1_i32;
            }
        } else {
            if *string.pointer as libc::c_int == ',' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '?' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '[' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == ']' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '{' as i32 as yaml_char_t as libc::c_int
                || *string.pointer as libc::c_int == '}' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1_i32;
            }
            if *string.pointer as libc::c_int == ':' as i32 as yaml_char_t as libc::c_int {
                flow_indicators = 1_i32;
                if followed_by_whitespace != 0 {
                    block_indicators = 1_i32;
                }
            }
            if *string.pointer as libc::c_int == '#' as i32 as yaml_char_t as libc::c_int
                && preceded_by_whitespace != 0
            {
                flow_indicators = 1_i32;
                block_indicators = 1_i32;
            }
        }
        if !IS_PRINTABLE!(string) || !IS_ASCII!(string) && (*emitter).unicode == 0 {
            special_characters = 1_i32;
        }
        if IS_BREAK!(string) {
            line_breaks = 1_i32;
        }
        if IS_SPACE!(string) {
            if string.start == string.pointer {
                leading_space = 1_i32;
            }
            if string.pointer.wrapping_offset(
                (if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                    2_i32
                } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                    3_i32
                } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            ) == string.end
            {
                trailing_space = 1_i32;
            }
            if previous_break != 0 {
                break_space = 1_i32;
            }
            previous_space = 1_i32;
            previous_break = 0_i32;
        } else if IS_BREAK!(string) {
            if string.start == string.pointer {
                leading_break = 1_i32;
            }
            if string.pointer.wrapping_offset(
                (if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                    2_i32
                } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                    3_i32
                } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            ) == string.end
            {
                trailing_break = 1_i32;
            }
            if previous_space != 0 {
                space_break = 1_i32;
            }
            previous_space = 0_i32;
            previous_break = 1_i32;
        } else {
            previous_space = 0_i32;
            previous_break = 0_i32;
        }
        preceded_by_whitespace = IS_BLANKZ!(string) as libc::c_int;
        MOVE!(string);
        if string.pointer != string.end {
            followed_by_whitespace = IS_BLANKZ_AT!(
                string,
                if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                    2_i32
                } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                    3_i32
                } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                    4_i32
                } else {
                    0_i32
                }
            ) as libc::c_int;
        }
    }
    (*emitter).scalar_data.multiline = line_breaks;
    (*emitter).scalar_data.flow_plain_allowed = 1_i32;
    (*emitter).scalar_data.block_plain_allowed = 1_i32;
    (*emitter).scalar_data.single_quoted_allowed = 1_i32;
    (*emitter).scalar_data.block_allowed = 1_i32;
    if leading_space != 0 || leading_break != 0 || trailing_space != 0 || trailing_break != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0_i32;
        (*emitter).scalar_data.block_plain_allowed = 0_i32;
    }
    if trailing_space != 0 {
        (*emitter).scalar_data.block_allowed = 0_i32;
    }
    if break_space != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0_i32;
        (*emitter).scalar_data.block_plain_allowed = 0_i32;
        (*emitter).scalar_data.single_quoted_allowed = 0_i32;
    }
    if space_break != 0 || special_characters != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0_i32;
        (*emitter).scalar_data.block_plain_allowed = 0_i32;
        (*emitter).scalar_data.single_quoted_allowed = 0_i32;
        (*emitter).scalar_data.block_allowed = 0_i32;
    }
    if line_breaks != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0_i32;
        (*emitter).scalar_data.block_plain_allowed = 0_i32;
    }
    if flow_indicators != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0_i32;
    }
    if block_indicators != 0 {
        (*emitter).scalar_data.block_plain_allowed = 0_i32;
    }
    1_i32
}

unsafe fn yaml_emitter_analyze_event(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    let fresh52 = addr_of_mut!((*emitter).anchor_data.anchor);
    *fresh52 = ptr::null_mut::<yaml_char_t>();
    (*emitter).anchor_data.anchor_length = 0_u64;
    let fresh53 = addr_of_mut!((*emitter).tag_data.handle);
    *fresh53 = ptr::null_mut::<yaml_char_t>();
    (*emitter).tag_data.handle_length = 0_u64;
    let fresh54 = addr_of_mut!((*emitter).tag_data.suffix);
    *fresh54 = ptr::null_mut::<yaml_char_t>();
    (*emitter).tag_data.suffix_length = 0_u64;
    let fresh55 = addr_of_mut!((*emitter).scalar_data.value);
    *fresh55 = ptr::null_mut::<yaml_char_t>();
    (*emitter).scalar_data.length = 0_u64;
    match (*event).type_ as libc::c_uint {
        5 => {
            if yaml_emitter_analyze_anchor(emitter, (*event).data.alias.anchor, 1_i32) == 0 {
                return 0_i32;
            }
            1_i32
        }
        6 => {
            if !((*event).data.scalar.anchor).is_null() {
                if yaml_emitter_analyze_anchor(emitter, (*event).data.scalar.anchor, 0_i32) == 0 {
                    return 0_i32;
                }
            }
            if !((*event).data.scalar.tag).is_null()
                && ((*emitter).canonical != 0
                    || (*event).data.scalar.plain_implicit == 0
                        && (*event).data.scalar.quoted_implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.scalar.tag) == 0 {
                    return 0_i32;
                }
            }
            if yaml_emitter_analyze_scalar(
                emitter,
                (*event).data.scalar.value,
                (*event).data.scalar.length,
            ) == 0
            {
                return 0_i32;
            }
            1_i32
        }
        7 => {
            if !((*event).data.sequence_start.anchor).is_null() {
                if yaml_emitter_analyze_anchor(emitter, (*event).data.sequence_start.anchor, 0_i32)
                    == 0
                {
                    return 0_i32;
                }
            }
            if !((*event).data.sequence_start.tag).is_null()
                && ((*emitter).canonical != 0 || (*event).data.sequence_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.sequence_start.tag) == 0 {
                    return 0_i32;
                }
            }
            1_i32
        }
        9 => {
            if !((*event).data.mapping_start.anchor).is_null() {
                if yaml_emitter_analyze_anchor(emitter, (*event).data.mapping_start.anchor, 0_i32)
                    == 0
                {
                    return 0_i32;
                }
            }
            if !((*event).data.mapping_start.tag).is_null()
                && ((*emitter).canonical != 0 || (*event).data.mapping_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.mapping_start.tag) == 0 {
                    return 0_i32;
                }
            }
            1_i32
        }
        _ => 1_i32,
    }
}

unsafe fn yaml_emitter_write_bom(emitter: *mut yaml_emitter_t) -> libc::c_int {
    if !FLUSH!(emitter) {
        return 0_i32;
    }
    let fresh56 = addr_of_mut!((*emitter).buffer.pointer);
    let fresh57 = *fresh56;
    *fresh56 = (*fresh56).wrapping_offset(1);
    *fresh57 = -17i32 as yaml_char_t;
    let fresh58 = addr_of_mut!((*emitter).buffer.pointer);
    let fresh59 = *fresh58;
    *fresh58 = (*fresh58).wrapping_offset(1);
    *fresh59 = -69i32 as yaml_char_t;
    let fresh60 = addr_of_mut!((*emitter).buffer.pointer);
    let fresh61 = *fresh60;
    *fresh60 = (*fresh60).wrapping_offset(1);
    *fresh61 = -65i32 as yaml_char_t;
    1_i32
}

unsafe fn yaml_emitter_write_indent(mut emitter: *mut yaml_emitter_t) -> libc::c_int {
    let indent: libc::c_int = if (*emitter).indent >= 0_i32 {
        (*emitter).indent
    } else {
        0_i32
    };
    if (*emitter).indention == 0
        || (*emitter).column > indent
        || (*emitter).column == indent && (*emitter).whitespace == 0
    {
        if !(PUT_BREAK!(emitter)) {
            return 0_i32;
        }
    }
    while (*emitter).column < indent {
        if !(PUT!(emitter, ' ')) {
            return 0_i32;
        }
    }
    (*emitter).whitespace = 1_i32;
    (*emitter).indention = 1_i32;
    1_i32
}

unsafe fn yaml_emitter_write_indicator(
    mut emitter: *mut yaml_emitter_t,
    indicator: *const libc::c_char,
    need_whitespace: libc::c_int,
    is_whitespace: libc::c_int,
    is_indention: libc::c_int,
) -> libc::c_int {
    let indicator_length: size_t = strlen(indicator);
    let mut string = STRING_ASSIGN!(indicator as *mut yaml_char_t, indicator_length);
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !(PUT!(emitter, ' ')) {
            return 0_i32;
        }
    }
    while string.pointer != string.end {
        if !(WRITE!(emitter, string)) {
            return 0_i32;
        }
    }
    (*emitter).whitespace = is_whitespace;
    (*emitter).indention = ((*emitter).indention != 0 && is_indention != 0) as libc::c_int;
    1_i32
}

unsafe fn yaml_emitter_write_anchor(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut string = STRING_ASSIGN!(value, length);
    while string.pointer != string.end {
        if !(WRITE!(emitter, string)) {
            return 0_i32;
        }
    }
    (*emitter).whitespace = 0_i32;
    (*emitter).indention = 0_i32;
    1_i32
}

unsafe fn yaml_emitter_write_tag_handle(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut string = STRING_ASSIGN!(value, length);
    if (*emitter).whitespace == 0 {
        if !(PUT!(emitter, ' ')) {
            return 0_i32;
        }
    }
    while string.pointer != string.end {
        if !(WRITE!(emitter, string)) {
            return 0_i32;
        }
    }
    (*emitter).whitespace = 0_i32;
    (*emitter).indention = 0_i32;
    1_i32
}

unsafe fn yaml_emitter_write_tag_content(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    need_whitespace: libc::c_int,
) -> libc::c_int {
    let mut string = STRING_ASSIGN!(value, length);
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !(PUT!(emitter, ' ')) {
            return 0_i32;
        }
    }
    while string.pointer != string.end {
        if IS_ALPHA!(string)
            || *string.pointer as libc::c_int == ';' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '/' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '?' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == ':' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '@' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '&' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '=' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '+' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '$' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == ',' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '_' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '.' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '~' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '*' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '\'' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '(' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == ')' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '[' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == ']' as i32 as yaml_char_t as libc::c_int
        {
            if !(WRITE!(emitter, string)) {
                return 0_i32;
            }
        } else {
            let mut width: libc::c_int = if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                2_i32
            } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                3_i32
            } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                4_i32
            } else {
                0_i32
            };
            let mut value_0: libc::c_uint;
            loop {
                let fresh207 = width;
                width -= 1;
                if !(fresh207 != 0) {
                    break;
                }
                let fresh208 = string.pointer;
                string.pointer = string.pointer.wrapping_offset(1);
                value_0 = *fresh208 as libc::c_uint;
                if !(PUT!(emitter, '%')) {
                    return 0_i32;
                }
                if !(PUT!(
                    emitter,
                    (value_0 >> 4_i32).wrapping_add(if (value_0 >> 4_i32) < 10_u32 {
                        '0' as i32
                    } else {
                        'A' as i32 - 10_i32
                    } as libc::c_uint)
                )) {
                    return 0_i32;
                }
                if !(PUT!(
                    emitter,
                    (value_0 & 0xf_i32 as libc::c_uint).wrapping_add(if (value_0
                        & 0xf_i32 as libc::c_uint)
                        < 10_u32
                    {
                        '0' as i32
                    } else {
                        'A' as i32 - 10_i32
                    }
                        as libc::c_uint)
                )) {
                    return 0_i32;
                }
            }
        }
    }
    (*emitter).whitespace = 0_i32;
    (*emitter).indention = 0_i32;
    1_i32
}

unsafe fn yaml_emitter_write_plain_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: libc::c_int,
) -> libc::c_int {
    let mut spaces: libc::c_int = 0_i32;
    let mut breaks: libc::c_int = 0_i32;
    let mut string = STRING_ASSIGN!(value, length);
    if (*emitter).whitespace == 0 && (length != 0 || (*emitter).flow_level != 0) {
        if !(PUT!(emitter, ' ')) {
            return 0_i32;
        }
    }
    while string.pointer != string.end {
        if IS_SPACE!(string) {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && !IS_SPACE_AT!(string, 1)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                MOVE!(string);
            } else if !(WRITE!(emitter, string)) {
                return 0_i32;
            }
            spaces = 1_i32;
        } else if IS_BREAK!(string) {
            if breaks == 0
                && *string.pointer as libc::c_int == '\n' as i32 as yaml_char_t as libc::c_int
            {
                if !(PUT_BREAK!(emitter)) {
                    return 0_i32;
                }
            }
            if !(WRITE_BREAK!(emitter, string)) {
                return 0_i32;
            }
            (*emitter).indention = 1_i32;
            breaks = 1_i32;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
            }
            if !(WRITE!(emitter, string)) {
                return 0_i32;
            }
            (*emitter).indention = 0_i32;
            spaces = 0_i32;
            breaks = 0_i32;
        }
    }
    (*emitter).whitespace = 0_i32;
    (*emitter).indention = 0_i32;
    1_i32
}

unsafe fn yaml_emitter_write_single_quoted_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: libc::c_int,
) -> libc::c_int {
    let mut spaces: libc::c_int = 0_i32;
    let mut breaks: libc::c_int = 0_i32;
    let mut string = STRING_ASSIGN!(value, length);
    if yaml_emitter_write_indicator(
        emitter,
        b"'\0" as *const u8 as *const libc::c_char,
        1_i32,
        0_i32,
        0_i32,
    ) == 0
    {
        return 0_i32;
    }
    while string.pointer != string.end {
        if IS_SPACE!(string) {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.wrapping_offset(-(1_isize))
                && !IS_SPACE_AT!(string, 1)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                MOVE!(string);
            } else if !(WRITE!(emitter, string)) {
                return 0_i32;
            }
            spaces = 1_i32;
        } else if IS_BREAK!(string) {
            if breaks == 0
                && *string.pointer as libc::c_int == '\n' as i32 as yaml_char_t as libc::c_int
            {
                if !(PUT_BREAK!(emitter)) {
                    return 0_i32;
                }
            }
            if !(WRITE_BREAK!(emitter, string)) {
                return 0_i32;
            }
            (*emitter).indention = 1_i32;
            breaks = 1_i32;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
            }
            if *string.pointer as libc::c_int == '\'' as i32 as yaml_char_t as libc::c_int {
                if !(PUT!(emitter, '\'')) {
                    return 0_i32;
                }
            }
            if !(WRITE!(emitter, string)) {
                return 0_i32;
            }
            (*emitter).indention = 0_i32;
            spaces = 0_i32;
            breaks = 0_i32;
        }
    }
    if breaks != 0 {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0_i32;
        }
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"'\0" as *const u8 as *const libc::c_char,
        0_i32,
        0_i32,
        0_i32,
    ) == 0
    {
        return 0_i32;
    }
    (*emitter).whitespace = 0_i32;
    (*emitter).indention = 0_i32;
    1_i32
}

unsafe fn yaml_emitter_write_double_quoted_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: libc::c_int,
) -> libc::c_int {
    let mut spaces: libc::c_int = 0_i32;
    let mut string = STRING_ASSIGN!(value, length);
    if yaml_emitter_write_indicator(
        emitter,
        b"\"\0" as *const u8 as *const libc::c_char,
        1_i32,
        0_i32,
        0_i32,
    ) == 0
    {
        return 0_i32;
    }
    while string.pointer != string.end {
        if !IS_PRINTABLE!(string)
            || (*emitter).unicode == 0 && !IS_ASCII!(string)
            || IS_BOM!(string)
            || IS_BREAK!(string)
            || *string.pointer as libc::c_int == '"' as i32 as yaml_char_t as libc::c_int
            || *string.pointer as libc::c_int == '\\' as i32 as yaml_char_t as libc::c_int
        {
            let mut octet: libc::c_uchar;
            let mut width: libc::c_uint;
            let mut value_0: libc::c_uint;
            let mut k: libc::c_int;
            octet = *string.pointer;
            width = (if octet as libc::c_int & 0x80_i32 == 0_i32 {
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
            value_0 = (if octet as libc::c_int & 0x80_i32 == 0_i32 {
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
            k = 1_i32;
            while k < width as libc::c_int {
                octet = *string.pointer.wrapping_offset(k as isize);
                value_0 = (value_0 << 6_i32)
                    .wrapping_add((octet as libc::c_int & 0x3f_i32) as libc::c_uint);
                k += 1;
            }
            string.pointer = string.pointer.wrapping_offset(width as isize);
            if !(PUT!(emitter, '\\')) {
                return 0_i32;
            }
            match value_0 {
                0 => {
                    if !(PUT!(emitter, '0')) {
                        return 0_i32;
                    }
                }
                7 => {
                    if !(PUT!(emitter, 'a')) {
                        return 0_i32;
                    }
                }
                8 => {
                    if !(PUT!(emitter, 'b')) {
                        return 0_i32;
                    }
                }
                9 => {
                    if !(PUT!(emitter, 't')) {
                        return 0_i32;
                    }
                }
                10 => {
                    if !(PUT!(emitter, 'n')) {
                        return 0_i32;
                    }
                }
                11 => {
                    if !(PUT!(emitter, 'v')) {
                        return 0_i32;
                    }
                }
                12 => {
                    if !(PUT!(emitter, 'f')) {
                        return 0_i32;
                    }
                }
                13 => {
                    if !(PUT!(emitter, 'r')) {
                        return 0_i32;
                    }
                }
                27 => {
                    if !(PUT!(emitter, 'e')) {
                        return 0_i32;
                    }
                }
                34 => {
                    if !(PUT!(emitter, '"')) {
                        return 0_i32;
                    }
                }
                92 => {
                    if !(PUT!(emitter, '\\')) {
                        return 0_i32;
                    }
                }
                133 => {
                    if !(PUT!(emitter, 'N')) {
                        return 0_i32;
                    }
                }
                160 => {
                    if !(PUT!(emitter, '_')) {
                        return 0_i32;
                    }
                }
                8232 => {
                    if !(PUT!(emitter, 'L')) {
                        return 0_i32;
                    }
                }
                8233 => {
                    if !(PUT!(emitter, 'P')) {
                        return 0_i32;
                    }
                }
                _ => {
                    if value_0 <= 0xff_i32 as libc::c_uint {
                        if !(PUT!(emitter, 'x')) {
                            return 0_i32;
                        }
                        width = 2_u32;
                    } else if value_0 <= 0xffff_i32 as libc::c_uint {
                        if !(PUT!(emitter, 'u')) {
                            return 0_i32;
                        }
                        width = 4_u32;
                    } else {
                        if !(PUT!(emitter, 'U')) {
                            return 0_i32;
                        }
                        width = 8_u32;
                    }
                    k = width.wrapping_sub(1_u32).wrapping_mul(4_u32) as libc::c_int;
                    while k >= 0_i32 {
                        let digit: libc::c_int =
                            (value_0 >> k & 0xf_i32 as libc::c_uint) as libc::c_int;
                        if !(PUT!(
                            emitter,
                            digit
                                + if digit < 10_i32 {
                                    '0' as i32
                                } else {
                                    'A' as i32 - 10_i32
                                }
                        )) {
                            return 0_i32;
                        }
                        k -= 4_i32;
                    }
                }
            }
            spaces = 0_i32;
        } else if IS_SPACE!(string) {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.wrapping_offset(-(1_isize))
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                if IS_SPACE_AT!(string, 1) {
                    if !(PUT!(emitter, '\\')) {
                        return 0_i32;
                    }
                }
                MOVE!(string);
            } else if !(WRITE!(emitter, string)) {
                return 0_i32;
            }
            spaces = 1_i32;
        } else {
            if !(WRITE!(emitter, string)) {
                return 0_i32;
            }
            spaces = 0_i32;
        }
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"\"\0" as *const u8 as *const libc::c_char,
        0_i32,
        0_i32,
        0_i32,
    ) == 0
    {
        return 0_i32;
    }
    (*emitter).whitespace = 0_i32;
    (*emitter).indention = 0_i32;
    1_i32
}

unsafe fn yaml_emitter_write_block_scalar_hints(
    mut emitter: *mut yaml_emitter_t,
    mut string: yaml_string_t,
) -> libc::c_int {
    let mut indent_hint: [libc::c_char; 2] = [0; 2];
    let mut chomp_hint: *const libc::c_char = ptr::null::<libc::c_char>();
    if IS_SPACE!(string) || IS_BREAK!(string) {
        indent_hint[0_usize] =
            ('0' as i32 + (*emitter).best_indent as libc::c_char as libc::c_int) as libc::c_char;
        indent_hint[1_usize] = '\0' as libc::c_char;
        if yaml_emitter_write_indicator(emitter, indent_hint.as_mut_ptr(), 0_i32, 0_i32, 0_i32) == 0
        {
            return 0_i32;
        }
    }
    (*emitter).open_ended = 0_i32;
    string.pointer = string.end;
    if string.start == string.pointer {
        chomp_hint = b"-\0" as *const u8 as *const libc::c_char;
    } else {
        loop {
            string.pointer = string.pointer.wrapping_offset(-1);
            if !(*string.pointer as libc::c_int & 0xc0_i32 == 0x80_i32) {
                break;
            }
        }
        if !IS_BREAK!(string) {
            chomp_hint = b"-\0" as *const u8 as *const libc::c_char;
        } else if string.start == string.pointer {
            chomp_hint = b"+\0" as *const u8 as *const libc::c_char;
            (*emitter).open_ended = 2_i32;
        } else {
            loop {
                string.pointer = string.pointer.wrapping_offset(-1);
                if !(*string.pointer as libc::c_int & 0xc0_i32 == 0x80_i32) {
                    break;
                }
            }
            if IS_BREAK!(string) {
                chomp_hint = b"+\0" as *const u8 as *const libc::c_char;
                (*emitter).open_ended = 2_i32;
            }
        }
    }
    if !chomp_hint.is_null() {
        if yaml_emitter_write_indicator(emitter, chomp_hint, 0_i32, 0_i32, 0_i32) == 0 {
            return 0_i32;
        }
    }
    1_i32
}

unsafe fn yaml_emitter_write_literal_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut breaks: libc::c_int = 1_i32;
    let mut string = STRING_ASSIGN!(value, length);
    if yaml_emitter_write_indicator(
        emitter,
        b"|\0" as *const u8 as *const libc::c_char,
        1_i32,
        0_i32,
        0_i32,
    ) == 0
    {
        return 0_i32;
    }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 {
        return 0_i32;
    }
    if !(PUT_BREAK!(emitter)) {
        return 0_i32;
    }
    (*emitter).indention = 1_i32;
    (*emitter).whitespace = 1_i32;
    while string.pointer != string.end {
        if IS_BREAK!(string) {
            if !(WRITE_BREAK!(emitter, string)) {
                return 0_i32;
            }
            (*emitter).indention = 1_i32;
            breaks = 1_i32;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
            }
            if !(WRITE!(emitter, string)) {
                return 0_i32;
            }
            (*emitter).indention = 0_i32;
            breaks = 0_i32;
        }
    }
    1_i32
}

unsafe fn yaml_emitter_write_folded_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut breaks: libc::c_int = 1_i32;
    let mut leading_spaces: libc::c_int = 1_i32;
    let mut string = STRING_ASSIGN!(value, length);
    if yaml_emitter_write_indicator(
        emitter,
        b">\0" as *const u8 as *const libc::c_char,
        1_i32,
        0_i32,
        0_i32,
    ) == 0
    {
        return 0_i32;
    }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 {
        return 0_i32;
    }
    if !(PUT_BREAK!(emitter)) {
        return 0_i32;
    }
    (*emitter).indention = 1_i32;
    (*emitter).whitespace = 1_i32;
    while string.pointer != string.end {
        if IS_BREAK!(string) {
            if breaks == 0
                && leading_spaces == 0
                && *string.pointer as libc::c_int == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let mut k: libc::c_int = 0_i32;
                while IS_BREAK_AT!(string, k as isize) {
                    k += if *string.pointer.wrapping_offset(k as isize) as libc::c_int & 0x80_i32
                        == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(k as isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(k as isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(k as isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    };
                }
                if !IS_BLANKZ_AT!(string, k) {
                    if !(PUT_BREAK!(emitter)) {
                        return 0_i32;
                    }
                }
            }
            if !(WRITE_BREAK!(emitter, string)) {
                return 0_i32;
            }
            (*emitter).indention = 1_i32;
            breaks = 1_i32;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                leading_spaces = IS_BLANK!(string) as libc::c_int;
            }
            if breaks == 0
                && IS_SPACE!(string)
                && !IS_SPACE_AT!(string, 1)
                && (*emitter).column > (*emitter).best_width
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                MOVE!(string);
            } else if !(WRITE!(emitter, string)) {
                return 0_i32;
            }
            (*emitter).indention = 0_i32;
            breaks = 0_i32;
        }
    }
    1_i32
}
