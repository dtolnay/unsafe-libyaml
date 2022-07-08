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
pub unsafe fn yaml_emitter_emit(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if if (*emitter).events.tail != (*emitter).events.end
        || yaml_queue_extend(
            addr_of_mut!((*emitter).events.start) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).events.head) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).events.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).events.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh1 = addr_of_mut!((*emitter).events.tail);
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_offset(1);
        *fresh2 = *event;
        1_i32
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
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
        let fresh3 = addr_of_mut!((*emitter).events.head);
        let fresh4 = *fresh3;
        *fresh3 = (*fresh3).wrapping_offset(1);
        yaml_event_delete(fresh4);
    }
    1_i32
}
unsafe fn yaml_emitter_need_more_events(emitter: *mut yaml_emitter_t) -> libc::c_int {
    let mut level: libc::c_int = 0_i32;
    let mut event: *mut yaml_event_t;
    if (*emitter).events.head == (*emitter).events.tail {
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
    } else if !(if (*emitter).tag_directives.top != (*emitter).tag_directives.end
        || yaml_stack_extend(
            addr_of_mut!((*emitter).tag_directives.start) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).tag_directives.top) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).tag_directives.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh5 = addr_of_mut!((*emitter).tag_directives.top);
        let fresh6 = *fresh5;
        *fresh5 = (*fresh5).wrapping_offset(1);
        *fresh6 = copy;
        1_i32
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
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
    if if (*emitter).indents.top != (*emitter).indents.end
        || yaml_stack_extend(
            addr_of_mut!((*emitter).indents.start) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).indents.top) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).indents.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh7 = addr_of_mut!((*emitter).indents.top);
        let fresh8 = *fresh7;
        *fresh7 = (*fresh7).wrapping_offset(1);
        *fresh8 = (*emitter).indent;
        1_i32
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
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
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh9 = addr_of_mut!((*emitter).states.top);
        let fresh10 = *fresh9;
        *fresh9 = (*fresh9).wrapping_offset(1);
        *fresh10 = YAML_EMIT_DOCUMENT_END_STATE;
        1_i32
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
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
        while !((*emitter).tag_directives.start == (*emitter).tag_directives.top) {
            let fresh11 = addr_of_mut!((*emitter).tag_directives.top);
            *fresh11 = (*fresh11).wrapping_offset(-1);
            let tag_directive: yaml_tag_directive_t = **fresh11;
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
        let fresh14 = addr_of_mut!((*emitter).indents.top);
        *fresh14 = (*fresh14).wrapping_offset(-1);
        (*emitter).indent = **fresh14;
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
        let fresh15 = addr_of_mut!((*emitter).states.top);
        *fresh15 = (*fresh15).wrapping_offset(-1);
        (*emitter).state = **fresh15;
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
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh16 = addr_of_mut!((*emitter).states.top);
        let fresh17 = *fresh16;
        *fresh16 = (*fresh16).wrapping_offset(1);
        *fresh17 = YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE;
        1_i32
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
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
        let fresh20 = addr_of_mut!((*emitter).indents.top);
        *fresh20 = (*fresh20).wrapping_offset(-1);
        (*emitter).indent = **fresh20;
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
        let fresh21 = addr_of_mut!((*emitter).states.top);
        *fresh21 = (*fresh21).wrapping_offset(-1);
        (*emitter).state = **fresh21;
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
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
                addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
                addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh22 = addr_of_mut!((*emitter).states.top);
            let fresh23 = *fresh22;
            *fresh22 = (*fresh22).wrapping_offset(1);
            *fresh23 = YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE;
            1_i32
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0
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
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
                addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
                addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh24 = addr_of_mut!((*emitter).states.top);
            let fresh25 = *fresh24;
            *fresh24 = (*fresh24).wrapping_offset(1);
            *fresh25 = YAML_EMIT_FLOW_MAPPING_VALUE_STATE;
            1_i32
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0
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
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh26 = addr_of_mut!((*emitter).states.top);
        let fresh27 = *fresh26;
        *fresh26 = (*fresh26).wrapping_offset(1);
        *fresh27 = YAML_EMIT_FLOW_MAPPING_KEY_STATE;
        1_i32
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
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
        let fresh28 = addr_of_mut!((*emitter).indents.top);
        *fresh28 = (*fresh28).wrapping_offset(-1);
        (*emitter).indent = **fresh28;
        let fresh29 = addr_of_mut!((*emitter).states.top);
        *fresh29 = (*fresh29).wrapping_offset(-1);
        (*emitter).state = **fresh29;
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
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh30 = addr_of_mut!((*emitter).states.top);
        let fresh31 = *fresh30;
        *fresh30 = (*fresh30).wrapping_offset(1);
        *fresh31 = YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE;
        1_i32
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
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
        let fresh32 = addr_of_mut!((*emitter).indents.top);
        *fresh32 = (*fresh32).wrapping_offset(-1);
        (*emitter).indent = **fresh32;
        let fresh33 = addr_of_mut!((*emitter).states.top);
        *fresh33 = (*fresh33).wrapping_offset(-1);
        (*emitter).state = **fresh33;
        return 1_i32;
    }
    if yaml_emitter_write_indent(emitter) == 0 {
        return 0_i32;
    }
    if yaml_emitter_check_simple_key(emitter) != 0 {
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
                addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
                addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh34 = addr_of_mut!((*emitter).states.top);
            let fresh35 = *fresh34;
            *fresh34 = (*fresh34).wrapping_offset(1);
            *fresh35 = YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE;
            1_i32
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0
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
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
                addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
                addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh36 = addr_of_mut!((*emitter).states.top);
            let fresh37 = *fresh36;
            *fresh36 = (*fresh36).wrapping_offset(1);
            *fresh37 = YAML_EMIT_BLOCK_MAPPING_VALUE_STATE;
            1_i32
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0
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
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh38 = addr_of_mut!((*emitter).states.top);
        let fresh39 = *fresh38;
        *fresh38 = (*fresh38).wrapping_offset(1);
        *fresh39 = YAML_EMIT_BLOCK_MAPPING_KEY_STATE;
        1_i32
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
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
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh40 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh41 = *fresh40;
                *fresh40 = (*fresh40).wrapping_offset(1);
                *fresh41 = ' ' as i32 as yaml_char_t;
                let fresh42 = addr_of_mut!((*emitter).column);
                *fresh42 += 1;
                1_i32 != 0
            })
        {
            return 0_i32;
        }
    }
    let fresh43 = addr_of_mut!((*emitter).states.top);
    *fresh43 = (*fresh43).wrapping_offset(-1);
    (*emitter).state = **fresh43;
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
    let fresh44 = addr_of_mut!((*emitter).indents.top);
    *fresh44 = (*fresh44).wrapping_offset(-1);
    (*emitter).indent = **fresh44;
    let fresh45 = addr_of_mut!((*emitter).states.top);
    *fresh45 = (*fresh45).wrapping_offset(-1);
    (*emitter).state = **fresh45;
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
    ((*((*emitter).events.head).wrapping_offset(0_isize)).type_ as libc::c_uint
        == YAML_SEQUENCE_START_EVENT as libc::c_int as libc::c_uint
        && (*((*emitter).events.head).wrapping_offset(1_isize)).type_ as libc::c_uint
            == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint) as libc::c_int
}
unsafe fn yaml_emitter_check_empty_mapping(emitter: *mut yaml_emitter_t) -> libc::c_int {
    if (((*emitter).events.tail).c_offset_from((*emitter).events.head) as libc::c_long) < 2_i64 {
        return 0_i32;
    }
    ((*((*emitter).events.head).wrapping_offset(0_isize)).type_ as libc::c_uint
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
    let mut handle = yaml_string_t {
        start: tag_directive.handle,
        end: tag_directive.handle.wrapping_offset(handle_length as isize),
        pointer: tag_directive.handle,
    };
    let prefix = yaml_string_t {
        start: tag_directive.prefix,
        end: tag_directive.prefix.wrapping_offset(prefix_length as isize),
        pointer: tag_directive.prefix,
    };
    if handle.start == handle.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must not be empty\0" as *const u8 as *const libc::c_char,
        );
    }
    if *handle.start.wrapping_offset(0_isize) as libc::c_int != '!' as i32 {
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
        if !(*handle.pointer.wrapping_offset(0_isize) as libc::c_int
            >= '0' as i32 as yaml_char_t as libc::c_int
            && *handle.pointer.wrapping_offset(0_isize) as libc::c_int
                <= '9' as i32 as yaml_char_t as libc::c_int
            || *handle.pointer.wrapping_offset(0_isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *handle.pointer.wrapping_offset(0_isize) as libc::c_int
                    <= 'Z' as i32 as yaml_char_t as libc::c_int
            || *handle.pointer.wrapping_offset(0_isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *handle.pointer.wrapping_offset(0_isize) as libc::c_int
                    <= 'z' as i32 as yaml_char_t as libc::c_int
            || *handle.pointer.wrapping_offset(0_isize) as libc::c_int == '_' as i32
            || *handle.pointer.wrapping_offset(0_isize) as libc::c_int == '-' as i32)
        {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"tag handle must contain alphanumerical characters only\0" as *const u8
                    as *const libc::c_char,
            );
        }
        handle.pointer = handle.pointer.wrapping_offset(
            (if *handle.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if *handle.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32 == 0xc0_i32
            {
                2_i32
            } else if *handle.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32 == 0xe0_i32
            {
                3_i32
            } else if *handle.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32 == 0xf0_i32
            {
                4_i32
            } else {
                0_i32
            }) as isize,
        );
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
    let mut string = yaml_string_t {
        start: anchor,
        end: anchor.wrapping_offset(anchor_length as isize),
        pointer: anchor,
    };
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
        if !(*string.pointer.wrapping_offset(0_isize) as libc::c_int
            >= '0' as i32 as yaml_char_t as libc::c_int
            && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                <= '9' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    <= 'Z' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    <= 'z' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == '_' as i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == '-' as i32)
        {
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
        string.pointer = string.pointer.wrapping_offset(
            (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32 == 0xc0_i32
            {
                2_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32 == 0xe0_i32
            {
                3_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32 == 0xf0_i32
            {
                4_i32
            } else {
                0_i32
            }) as isize,
        );
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
    let string = yaml_string_t {
        start: tag,
        end: tag.wrapping_offset(tag_length as isize),
        pointer: tag,
    };
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
    let mut string = yaml_string_t {
        start: value,
        end: value.wrapping_offset(length as isize),
        pointer: value,
    };
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
    if *string.pointer.wrapping_offset(0_isize) as libc::c_int
        == '-' as i32 as yaml_char_t as libc::c_int
        && *string.pointer.wrapping_offset(1_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && *string.pointer.wrapping_offset(2_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        || *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
            && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
            && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
    {
        block_indicators = 1_i32;
        flow_indicators = 1_i32;
    }
    preceded_by_whitespace = 1_i32;
    followed_by_whitespace = (*string.pointer.wrapping_offset(
        (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
            1_i32
        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32 == 0xc0_i32 {
            2_i32
        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32 == 0xe0_i32 {
            3_i32
        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32 == 0xf0_i32 {
            4_i32
        } else {
            0_i32
        }) as isize,
    ) as libc::c_int
        == ' ' as i32 as yaml_char_t as libc::c_int
        || *string.pointer.wrapping_offset(
            (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32 == 0xc0_i32
            {
                2_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32 == 0xe0_i32
            {
                3_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32 == 0xf0_i32
            {
                4_i32
            } else {
                0_i32
            }) as isize,
        ) as libc::c_int
            == '\t' as i32 as yaml_char_t as libc::c_int
        || (*string.pointer.wrapping_offset(
            (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32 == 0xc0_i32
            {
                2_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32 == 0xe0_i32
            {
                3_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32 == 0xf0_i32
            {
                4_i32
            } else {
                0_i32
            }) as isize,
        ) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(
                (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                    == 0xc0_i32
                {
                    2_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                    == 0xe0_i32
                {
                    3_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                    == 0xf0_i32
                {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            ) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(
                (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                    == 0xc0_i32
                {
                    2_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                    == 0xe0_i32
                {
                    3_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                    == 0xf0_i32
                {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            ) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(
                    ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) + 1_i32) as isize,
                ) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(
                (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                    == 0xc0_i32
                {
                    2_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                    == 0xe0_i32
                {
                    3_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                    == 0xf0_i32
                {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            ) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(
                    ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) + 1_i32) as isize,
                ) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(
                    ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) + 2_i32) as isize,
                ) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(
                (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                    == 0xc0_i32
                {
                    2_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                    == 0xe0_i32
                {
                    3_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                    == 0xf0_i32
                {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            ) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(
                    ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) + 1_i32) as isize,
                ) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(
                    ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) + 2_i32) as isize,
                ) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(
                (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                    == 0xc0_i32
                {
                    2_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                    == 0xe0_i32
                {
                    3_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                    == 0xf0_i32
                {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            ) as libc::c_int
                == '\0' as i32 as yaml_char_t as libc::c_int))
        as libc::c_int;
    while string.pointer != string.end {
        if string.start == string.pointer {
            if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '#' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == ',' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '[' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == ']' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '{' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '}' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '&' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '*' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '!' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '|' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '>' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\'' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '"' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '%' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '@' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '`' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1_i32;
                block_indicators = 1_i32;
            }
            if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '?' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == ':' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1_i32;
                if followed_by_whitespace != 0 {
                    block_indicators = 1_i32;
                }
            }
            if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '-' as i32 as yaml_char_t as libc::c_int
                && followed_by_whitespace != 0
            {
                flow_indicators = 1_i32;
                block_indicators = 1_i32;
            }
        } else {
            if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == ',' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '?' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '[' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == ']' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '{' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '}' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1_i32;
            }
            if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == ':' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1_i32;
                if followed_by_whitespace != 0 {
                    block_indicators = 1_i32;
                }
            }
            if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '#' as i32 as yaml_char_t as libc::c_int
                && preceded_by_whitespace != 0
            {
                flow_indicators = 1_i32;
                block_indicators = 1_i32;
            }
        }
        if !(*string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xa_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int >= 0x20_i32
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int <= 0x7e_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xc2_i32
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int >= 0xa0_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int > 0xc2_i32
                && (*string.pointer.wrapping_offset(0_isize) as libc::c_int) < 0xed_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xed_i32
                && (*string.pointer.wrapping_offset(1_isize) as libc::c_int) < 0xa0_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xee_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xef_i32
                && !(*string.pointer.wrapping_offset(1_isize) as libc::c_int == 0xbb_i32
                    && *string.pointer.wrapping_offset(2_isize) as libc::c_int == 0xbf_i32)
                && !(*string.pointer.wrapping_offset(1_isize) as libc::c_int == 0xbf_i32
                    && (*string.pointer.wrapping_offset(2_isize) as libc::c_int == 0xbe_i32
                        || *string.pointer.wrapping_offset(2_isize) as libc::c_int == 0xbf_i32)))
            || !(*string.pointer.wrapping_offset(0_isize) as libc::c_int
                <= '\u{7f}' as i32 as yaml_char_t as libc::c_int)
                && (*emitter).unicode == 0
        {
            special_characters = 1_i32;
        }
        if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            line_breaks = 1_i32;
        }
        if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if string.start == string.pointer {
                leading_space = 1_i32;
            }
            if string.pointer.wrapping_offset(
                (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                    == 0xc0_i32
                {
                    2_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                    == 0xe0_i32
                {
                    3_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                    == 0xf0_i32
                {
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
        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if string.start == string.pointer {
                leading_break = 1_i32;
            }
            if string.pointer.wrapping_offset(
                (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                    == 0xc0_i32
                {
                    2_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                    == 0xe0_i32
                {
                    3_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                    == 0xf0_i32
                {
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
        preceded_by_whitespace = (*string.pointer.wrapping_offset(0_isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
            || (*string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int))
            as libc::c_int;
        string.pointer = string.pointer.wrapping_offset(
            (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32 == 0xc0_i32
            {
                2_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32 == 0xe0_i32
            {
                3_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32 == 0xf0_i32
            {
                4_i32
            } else {
                0_i32
            }) as isize,
        );
        if string.pointer != string.end {
            followed_by_whitespace = (*string.pointer.wrapping_offset(
                (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                    == 0xc0_i32
                {
                    2_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                    == 0xe0_i32
                {
                    3_i32
                } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                    == 0xf0_i32
                {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            ) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(
                    (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) as isize,
                ) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
                || (*string.pointer.wrapping_offset(
                    (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) as isize,
                ) as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(
                        (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                            == 0_i32
                        {
                            1_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                            == 0xc0_i32
                        {
                            2_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                            == 0xe0_i32
                        {
                            3_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                            == 0xf0_i32
                        {
                            4_i32
                        } else {
                            0_i32
                        }) as isize,
                    ) as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(
                        (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                            == 0_i32
                        {
                            1_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                            == 0xc0_i32
                        {
                            2_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                            == 0xe0_i32
                        {
                            3_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                            == 0xf0_i32
                        {
                            4_i32
                        } else {
                            0_i32
                        }) as isize,
                    ) as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset(
                            ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                                == 0_i32
                            {
                                1_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xe0_i32
                                == 0xc0_i32
                            {
                                2_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf0_i32
                                == 0xe0_i32
                            {
                                3_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf8_i32
                                == 0xf0_i32
                            {
                                4_i32
                            } else {
                                0_i32
                            }) + 1_i32) as isize,
                        ) as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(
                        (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                            == 0_i32
                        {
                            1_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                            == 0xc0_i32
                        {
                            2_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                            == 0xe0_i32
                        {
                            3_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                            == 0xf0_i32
                        {
                            4_i32
                        } else {
                            0_i32
                        }) as isize,
                    ) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset(
                            ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                                == 0_i32
                            {
                                1_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xe0_i32
                                == 0xc0_i32
                            {
                                2_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf0_i32
                                == 0xe0_i32
                            {
                                3_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf8_i32
                                == 0xf0_i32
                            {
                                4_i32
                            } else {
                                0_i32
                            }) + 1_i32) as isize,
                        ) as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset(
                            ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                                == 0_i32
                            {
                                1_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xe0_i32
                                == 0xc0_i32
                            {
                                2_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf0_i32
                                == 0xe0_i32
                            {
                                3_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf8_i32
                                == 0xf0_i32
                            {
                                4_i32
                            } else {
                                0_i32
                            }) + 2_i32) as isize,
                        ) as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(
                        (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                            == 0_i32
                        {
                            1_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                            == 0xc0_i32
                        {
                            2_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                            == 0xe0_i32
                        {
                            3_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                            == 0xf0_i32
                        {
                            4_i32
                        } else {
                            0_i32
                        }) as isize,
                    ) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset(
                            ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                                == 0_i32
                            {
                                1_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xe0_i32
                                == 0xc0_i32
                            {
                                2_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf0_i32
                                == 0xe0_i32
                            {
                                3_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf8_i32
                                == 0xf0_i32
                            {
                                4_i32
                            } else {
                                0_i32
                            }) + 1_i32) as isize,
                        ) as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset(
                            ((if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                                == 0_i32
                            {
                                1_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xe0_i32
                                == 0xc0_i32
                            {
                                2_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf0_i32
                                == 0xe0_i32
                            {
                                3_i32
                            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                                & 0xf8_i32
                                == 0xf0_i32
                            {
                                4_i32
                            } else {
                                0_i32
                            }) + 2_i32) as isize,
                        ) as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(
                        (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                            == 0_i32
                        {
                            1_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                            == 0xc0_i32
                        {
                            2_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                            == 0xe0_i32
                        {
                            3_i32
                        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                            == 0xf0_i32
                        {
                            4_i32
                        } else {
                            0_i32
                        }) as isize,
                    ) as libc::c_int
                        == '\0' as i32 as yaml_char_t as libc::c_int))
                as libc::c_int;
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
    if !(((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
        || yaml_emitter_flush(emitter) != 0)
    {
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
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if (*emitter).line_break as libc::c_uint
                    == YAML_CR_BREAK as libc::c_int as libc::c_uint
                {
                    let fresh62 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh63 = *fresh62;
                    *fresh62 = (*fresh62).wrapping_offset(1);
                    *fresh63 = '\r' as i32 as yaml_char_t;
                } else if (*emitter).line_break as libc::c_uint
                    == YAML_LN_BREAK as libc::c_int as libc::c_uint
                {
                    let fresh64 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh65 = *fresh64;
                    *fresh64 = (*fresh64).wrapping_offset(1);
                    *fresh65 = '\n' as i32 as yaml_char_t;
                } else if (*emitter).line_break as libc::c_uint
                    == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                {
                    let fresh66 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh67 = *fresh66;
                    *fresh66 = (*fresh66).wrapping_offset(1);
                    *fresh67 = '\r' as i32 as yaml_char_t;
                    let fresh68 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh69 = *fresh68;
                    *fresh68 = (*fresh68).wrapping_offset(1);
                    *fresh69 = '\n' as i32 as yaml_char_t;
                };
                (*emitter).column = 0_i32;
                let fresh70 = addr_of_mut!((*emitter).line);
                *fresh70 += 1;
                1_i32 != 0
            })
        {
            return 0_i32;
        }
    }
    while (*emitter).column < indent {
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh71 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh72 = *fresh71;
                *fresh71 = (*fresh71).wrapping_offset(1);
                *fresh72 = ' ' as i32 as yaml_char_t;
                let fresh73 = addr_of_mut!((*emitter).column);
                *fresh73 += 1;
                1_i32 != 0
            })
        {
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
    let mut string = yaml_string_t {
        start: indicator as *mut yaml_char_t,
        end: (indicator as *mut yaml_char_t).wrapping_offset(indicator_length as isize),
        pointer: indicator as *mut yaml_char_t,
    };
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh74 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh75 = *fresh74;
                *fresh74 = (*fresh74).wrapping_offset(1);
                *fresh75 = ' ' as i32 as yaml_char_t;
                let fresh76 = addr_of_mut!((*emitter).column);
                *fresh76 += 1;
                1_i32 != 0
            })
        {
            return 0_i32;
        }
    }
    while string.pointer != string.end {
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                    let fresh77 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh78 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh79 = *fresh78;
                    *fresh78 = (*fresh78).wrapping_offset(1);
                    *fresh79 = *fresh77;
                } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                    let fresh80 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh81 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh82 = *fresh81;
                    *fresh81 = (*fresh81).wrapping_offset(1);
                    *fresh82 = *fresh80;
                    let fresh83 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh84 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh85 = *fresh84;
                    *fresh84 = (*fresh84).wrapping_offset(1);
                    *fresh85 = *fresh83;
                } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                    let fresh86 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh87 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh88 = *fresh87;
                    *fresh87 = (*fresh87).wrapping_offset(1);
                    *fresh88 = *fresh86;
                    let fresh89 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh90 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh91 = *fresh90;
                    *fresh90 = (*fresh90).wrapping_offset(1);
                    *fresh91 = *fresh89;
                    let fresh92 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh93 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh94 = *fresh93;
                    *fresh93 = (*fresh93).wrapping_offset(1);
                    *fresh94 = *fresh92;
                } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                    let fresh95 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh96 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh97 = *fresh96;
                    *fresh96 = (*fresh96).wrapping_offset(1);
                    *fresh97 = *fresh95;
                    let fresh98 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh99 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh100 = *fresh99;
                    *fresh99 = (*fresh99).wrapping_offset(1);
                    *fresh100 = *fresh98;
                    let fresh101 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh102 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh103 = *fresh102;
                    *fresh102 = (*fresh102).wrapping_offset(1);
                    *fresh103 = *fresh101;
                    let fresh104 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh105 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh106 = *fresh105;
                    *fresh105 = (*fresh105).wrapping_offset(1);
                    *fresh106 = *fresh104;
                };
                let fresh107 = addr_of_mut!((*emitter).column);
                *fresh107 += 1;
                1_i32 != 0
            })
        {
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
    let mut string = yaml_string_t {
        start: value,
        end: value.wrapping_offset(length as isize),
        pointer: value,
    };
    while string.pointer != string.end {
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                    let fresh108 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh109 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh110 = *fresh109;
                    *fresh109 = (*fresh109).wrapping_offset(1);
                    *fresh110 = *fresh108;
                } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                    let fresh111 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh112 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh113 = *fresh112;
                    *fresh112 = (*fresh112).wrapping_offset(1);
                    *fresh113 = *fresh111;
                    let fresh114 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh115 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh116 = *fresh115;
                    *fresh115 = (*fresh115).wrapping_offset(1);
                    *fresh116 = *fresh114;
                } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                    let fresh117 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh118 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh119 = *fresh118;
                    *fresh118 = (*fresh118).wrapping_offset(1);
                    *fresh119 = *fresh117;
                    let fresh120 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh121 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh122 = *fresh121;
                    *fresh121 = (*fresh121).wrapping_offset(1);
                    *fresh122 = *fresh120;
                    let fresh123 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh124 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh125 = *fresh124;
                    *fresh124 = (*fresh124).wrapping_offset(1);
                    *fresh125 = *fresh123;
                } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                    let fresh126 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh127 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh128 = *fresh127;
                    *fresh127 = (*fresh127).wrapping_offset(1);
                    *fresh128 = *fresh126;
                    let fresh129 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh130 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh131 = *fresh130;
                    *fresh130 = (*fresh130).wrapping_offset(1);
                    *fresh131 = *fresh129;
                    let fresh132 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh133 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh134 = *fresh133;
                    *fresh133 = (*fresh133).wrapping_offset(1);
                    *fresh134 = *fresh132;
                    let fresh135 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh136 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh137 = *fresh136;
                    *fresh136 = (*fresh136).wrapping_offset(1);
                    *fresh137 = *fresh135;
                };
                let fresh138 = addr_of_mut!((*emitter).column);
                *fresh138 += 1;
                1_i32 != 0
            })
        {
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
    let mut string = yaml_string_t {
        start: value,
        end: value.wrapping_offset(length as isize),
        pointer: value,
    };
    if (*emitter).whitespace == 0 {
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh139 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh140 = *fresh139;
                *fresh139 = (*fresh139).wrapping_offset(1);
                *fresh140 = ' ' as i32 as yaml_char_t;
                let fresh141 = addr_of_mut!((*emitter).column);
                *fresh141 += 1;
                1_i32 != 0
            })
        {
            return 0_i32;
        }
    }
    while string.pointer != string.end {
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                    let fresh142 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh143 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh144 = *fresh143;
                    *fresh143 = (*fresh143).wrapping_offset(1);
                    *fresh144 = *fresh142;
                } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                    let fresh145 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh146 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh147 = *fresh146;
                    *fresh146 = (*fresh146).wrapping_offset(1);
                    *fresh147 = *fresh145;
                    let fresh148 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh149 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh150 = *fresh149;
                    *fresh149 = (*fresh149).wrapping_offset(1);
                    *fresh150 = *fresh148;
                } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                    let fresh151 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh152 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh153 = *fresh152;
                    *fresh152 = (*fresh152).wrapping_offset(1);
                    *fresh153 = *fresh151;
                    let fresh154 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh155 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh156 = *fresh155;
                    *fresh155 = (*fresh155).wrapping_offset(1);
                    *fresh156 = *fresh154;
                    let fresh157 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh158 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh159 = *fresh158;
                    *fresh158 = (*fresh158).wrapping_offset(1);
                    *fresh159 = *fresh157;
                } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                    let fresh160 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh161 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh162 = *fresh161;
                    *fresh161 = (*fresh161).wrapping_offset(1);
                    *fresh162 = *fresh160;
                    let fresh163 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh164 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh165 = *fresh164;
                    *fresh164 = (*fresh164).wrapping_offset(1);
                    *fresh165 = *fresh163;
                    let fresh166 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh167 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh168 = *fresh167;
                    *fresh167 = (*fresh167).wrapping_offset(1);
                    *fresh168 = *fresh166;
                    let fresh169 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    let fresh170 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh171 = *fresh170;
                    *fresh170 = (*fresh170).wrapping_offset(1);
                    *fresh171 = *fresh169;
                };
                let fresh172 = addr_of_mut!((*emitter).column);
                *fresh172 += 1;
                1_i32 != 0
            })
        {
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
    let mut string = yaml_string_t {
        start: value,
        end: value.wrapping_offset(length as isize),
        pointer: value,
    };
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh173 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh174 = *fresh173;
                *fresh173 = (*fresh173).wrapping_offset(1);
                *fresh174 = ' ' as i32 as yaml_char_t;
                let fresh175 = addr_of_mut!((*emitter).column);
                *fresh175 += 1;
                1_i32 != 0
            })
        {
            return 0_i32;
        }
    }
    while string.pointer != string.end {
        if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            >= '0' as i32 as yaml_char_t as libc::c_int
            && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                <= '9' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    <= 'Z' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    <= 'z' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == '_' as i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == '-' as i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == ';' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '/' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '?' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == ':' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '@' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '&' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '=' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '+' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '$' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == ',' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '_' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '~' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '*' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\'' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '(' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == ')' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '[' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == ']' as i32 as yaml_char_t as libc::c_int
        {
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh176 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh177 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh178 = *fresh177;
                        *fresh177 = (*fresh177).wrapping_offset(1);
                        *fresh178 = *fresh176;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh179 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh180 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh181 = *fresh180;
                        *fresh180 = (*fresh180).wrapping_offset(1);
                        *fresh181 = *fresh179;
                        let fresh182 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh183 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh184 = *fresh183;
                        *fresh183 = (*fresh183).wrapping_offset(1);
                        *fresh184 = *fresh182;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh185 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh186 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh187 = *fresh186;
                        *fresh186 = (*fresh186).wrapping_offset(1);
                        *fresh187 = *fresh185;
                        let fresh188 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh189 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh190 = *fresh189;
                        *fresh189 = (*fresh189).wrapping_offset(1);
                        *fresh190 = *fresh188;
                        let fresh191 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh192 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh193 = *fresh192;
                        *fresh192 = (*fresh192).wrapping_offset(1);
                        *fresh193 = *fresh191;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh194 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh195 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh196 = *fresh195;
                        *fresh195 = (*fresh195).wrapping_offset(1);
                        *fresh196 = *fresh194;
                        let fresh197 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh198 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh199 = *fresh198;
                        *fresh198 = (*fresh198).wrapping_offset(1);
                        *fresh199 = *fresh197;
                        let fresh200 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh201 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh202 = *fresh201;
                        *fresh201 = (*fresh201).wrapping_offset(1);
                        *fresh202 = *fresh200;
                        let fresh203 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh204 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh205 = *fresh204;
                        *fresh204 = (*fresh204).wrapping_offset(1);
                        *fresh205 = *fresh203;
                    };
                    let fresh206 = addr_of_mut!((*emitter).column);
                    *fresh206 += 1;
                    1_i32 != 0
                })
            {
                return 0_i32;
            }
        } else {
            let mut width: libc::c_int = if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                & 0x80_i32
                == 0_i32
            {
                1_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32 == 0xc0_i32
            {
                2_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32 == 0xe0_i32
            {
                3_i32
            } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32 == 0xf0_i32
            {
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
                if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh209 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh210 = *fresh209;
                        *fresh209 = (*fresh209).wrapping_offset(1);
                        *fresh210 = '%' as i32 as yaml_char_t;
                        let fresh211 = addr_of_mut!((*emitter).column);
                        *fresh211 += 1;
                        1_i32 != 0
                    })
                {
                    return 0_i32;
                }
                if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh212 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh213 = *fresh212;
                        *fresh212 = (*fresh212).wrapping_offset(1);
                        *fresh213 = (value_0 >> 4_i32).wrapping_add(
                            (if (value_0 >> 4_i32) < 10_u32 {
                                '0' as i32
                            } else {
                                'A' as i32 - 10_i32
                            }) as libc::c_uint,
                        ) as yaml_char_t;
                        let fresh214 = addr_of_mut!((*emitter).column);
                        *fresh214 += 1;
                        1_i32 != 0
                    })
                {
                    return 0_i32;
                }
                if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh215 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh216 = *fresh215;
                        *fresh215 = (*fresh215).wrapping_offset(1);
                        *fresh216 = (value_0 & 0xf_i32 as libc::c_uint).wrapping_add(
                            (if (value_0 & 0xf_i32 as libc::c_uint) < 10_u32 {
                                '0' as i32
                            } else {
                                'A' as i32 - 10_i32
                            }) as libc::c_uint,
                        ) as yaml_char_t;
                        let fresh217 = addr_of_mut!((*emitter).column);
                        *fresh217 += 1;
                        1_i32 != 0
                    })
                {
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
    let mut string = yaml_string_t {
        start: value,
        end: value.wrapping_offset(length as isize),
        pointer: value,
    };
    if (*emitter).whitespace == 0 && (length != 0 || (*emitter).flow_level != 0) {
        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh218 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh219 = *fresh218;
                *fresh218 = (*fresh218).wrapping_offset(1);
                *fresh219 = ' ' as i32 as yaml_char_t;
                let fresh220 = addr_of_mut!((*emitter).column);
                *fresh220 += 1;
                1_i32 != 0
            })
        {
            return 0_i32;
        }
    }
    while string.pointer != string.end {
        if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && !(*string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                string.pointer = string.pointer.wrapping_offset(
                    (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) as isize,
                );
            } else if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh221 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh222 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh223 = *fresh222;
                        *fresh222 = (*fresh222).wrapping_offset(1);
                        *fresh223 = *fresh221;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh224 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh225 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh226 = *fresh225;
                        *fresh225 = (*fresh225).wrapping_offset(1);
                        *fresh226 = *fresh224;
                        let fresh227 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh228 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh229 = *fresh228;
                        *fresh228 = (*fresh228).wrapping_offset(1);
                        *fresh229 = *fresh227;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh230 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh231 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh232 = *fresh231;
                        *fresh231 = (*fresh231).wrapping_offset(1);
                        *fresh232 = *fresh230;
                        let fresh233 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh234 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh235 = *fresh234;
                        *fresh234 = (*fresh234).wrapping_offset(1);
                        *fresh235 = *fresh233;
                        let fresh236 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh237 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh238 = *fresh237;
                        *fresh237 = (*fresh237).wrapping_offset(1);
                        *fresh238 = *fresh236;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh239 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh240 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh241 = *fresh240;
                        *fresh240 = (*fresh240).wrapping_offset(1);
                        *fresh241 = *fresh239;
                        let fresh242 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh243 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh244 = *fresh243;
                        *fresh243 = (*fresh243).wrapping_offset(1);
                        *fresh244 = *fresh242;
                        let fresh245 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh246 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh247 = *fresh246;
                        *fresh246 = (*fresh246).wrapping_offset(1);
                        *fresh247 = *fresh245;
                        let fresh248 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh249 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh250 = *fresh249;
                        *fresh249 = (*fresh249).wrapping_offset(1);
                        *fresh250 = *fresh248;
                    };
                    let fresh251 = addr_of_mut!((*emitter).column);
                    *fresh251 += 1;
                    1_i32 != 0
                })
            {
                return 0_i32;
            }
            spaces = 1_i32;
        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if breaks == 0
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        if (*emitter).line_break as libc::c_uint
                            == YAML_CR_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh252 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh253 = *fresh252;
                            *fresh252 = (*fresh252).wrapping_offset(1);
                            *fresh253 = '\r' as i32 as yaml_char_t;
                        } else if (*emitter).line_break as libc::c_uint
                            == YAML_LN_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh254 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh255 = *fresh254;
                            *fresh254 = (*fresh254).wrapping_offset(1);
                            *fresh255 = '\n' as i32 as yaml_char_t;
                        } else if (*emitter).line_break as libc::c_uint
                            == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh256 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh257 = *fresh256;
                            *fresh256 = (*fresh256).wrapping_offset(1);
                            *fresh257 = '\r' as i32 as yaml_char_t;
                            let fresh258 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh259 = *fresh258;
                            *fresh258 = (*fresh258).wrapping_offset(1);
                            *fresh259 = '\n' as i32 as yaml_char_t;
                        };
                        (*emitter).column = 0_i32;
                        let fresh260 = addr_of_mut!((*emitter).line);
                        *fresh260 += 1;
                        1_i32 != 0
                    })
                {
                    return 0_i32;
                }
            }
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    if ((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0
                    {
                        {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh261 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh262 = *fresh261;
                                *fresh261 = (*fresh261).wrapping_offset(1);
                                *fresh262 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh263 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh264 = *fresh263;
                                *fresh263 = (*fresh263).wrapping_offset(1);
                                *fresh264 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh265 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh266 = *fresh265;
                                *fresh265 = (*fresh265).wrapping_offset(1);
                                *fresh266 = '\r' as i32 as yaml_char_t;
                                let fresh267 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh268 = *fresh267;
                                *fresh267 = (*fresh267).wrapping_offset(1);
                                *fresh268 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0_i32;
                            let fresh269 = addr_of_mut!((*emitter).line);
                            *fresh269 += 1;
                            1_i32 != 0
                        };
                    }
                    string.pointer = string.pointer.wrapping_offset(1);
                    1_i32
                } else {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh270 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh271 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh272 = *fresh271;
                        *fresh271 = (*fresh271).wrapping_offset(1);
                        *fresh272 = *fresh270;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh273 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh274 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh275 = *fresh274;
                        *fresh274 = (*fresh274).wrapping_offset(1);
                        *fresh275 = *fresh273;
                        let fresh276 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh277 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh278 = *fresh277;
                        *fresh277 = (*fresh277).wrapping_offset(1);
                        *fresh278 = *fresh276;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh279 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh280 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh281 = *fresh280;
                        *fresh280 = (*fresh280).wrapping_offset(1);
                        *fresh281 = *fresh279;
                        let fresh282 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh283 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh284 = *fresh283;
                        *fresh283 = (*fresh283).wrapping_offset(1);
                        *fresh284 = *fresh282;
                        let fresh285 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh286 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh287 = *fresh286;
                        *fresh286 = (*fresh286).wrapping_offset(1);
                        *fresh287 = *fresh285;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh288 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh289 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh290 = *fresh289;
                        *fresh289 = (*fresh289).wrapping_offset(1);
                        *fresh290 = *fresh288;
                        let fresh291 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh292 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh293 = *fresh292;
                        *fresh292 = (*fresh292).wrapping_offset(1);
                        *fresh293 = *fresh291;
                        let fresh294 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh295 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh296 = *fresh295;
                        *fresh295 = (*fresh295).wrapping_offset(1);
                        *fresh296 = *fresh294;
                        let fresh297 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh298 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh299 = *fresh298;
                        *fresh298 = (*fresh298).wrapping_offset(1);
                        *fresh299 = *fresh297;
                    };
                    (*emitter).column = 0_i32;
                    let fresh300 = addr_of_mut!((*emitter).line);
                    *fresh300 += 1;
                    1_i32
                }) != 0)
            {
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
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh301 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh302 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh303 = *fresh302;
                        *fresh302 = (*fresh302).wrapping_offset(1);
                        *fresh303 = *fresh301;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh304 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh305 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh306 = *fresh305;
                        *fresh305 = (*fresh305).wrapping_offset(1);
                        *fresh306 = *fresh304;
                        let fresh307 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh308 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh309 = *fresh308;
                        *fresh308 = (*fresh308).wrapping_offset(1);
                        *fresh309 = *fresh307;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh310 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh311 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh312 = *fresh311;
                        *fresh311 = (*fresh311).wrapping_offset(1);
                        *fresh312 = *fresh310;
                        let fresh313 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh314 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh315 = *fresh314;
                        *fresh314 = (*fresh314).wrapping_offset(1);
                        *fresh315 = *fresh313;
                        let fresh316 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh317 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh318 = *fresh317;
                        *fresh317 = (*fresh317).wrapping_offset(1);
                        *fresh318 = *fresh316;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh319 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh320 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh321 = *fresh320;
                        *fresh320 = (*fresh320).wrapping_offset(1);
                        *fresh321 = *fresh319;
                        let fresh322 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh323 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh324 = *fresh323;
                        *fresh323 = (*fresh323).wrapping_offset(1);
                        *fresh324 = *fresh322;
                        let fresh325 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh326 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh327 = *fresh326;
                        *fresh326 = (*fresh326).wrapping_offset(1);
                        *fresh327 = *fresh325;
                        let fresh328 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh329 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh330 = *fresh329;
                        *fresh329 = (*fresh329).wrapping_offset(1);
                        *fresh330 = *fresh328;
                    };
                    let fresh331 = addr_of_mut!((*emitter).column);
                    *fresh331 += 1;
                    1_i32 != 0
                })
            {
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
    let mut string = yaml_string_t {
        start: value,
        end: value.wrapping_offset(length as isize),
        pointer: value,
    };
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
        if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.wrapping_offset(-(1_isize))
                && !(*string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                string.pointer = string.pointer.wrapping_offset(
                    (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) as isize,
                );
            } else if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh332 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh333 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh334 = *fresh333;
                        *fresh333 = (*fresh333).wrapping_offset(1);
                        *fresh334 = *fresh332;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh335 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh336 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh337 = *fresh336;
                        *fresh336 = (*fresh336).wrapping_offset(1);
                        *fresh337 = *fresh335;
                        let fresh338 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh339 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh340 = *fresh339;
                        *fresh339 = (*fresh339).wrapping_offset(1);
                        *fresh340 = *fresh338;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh341 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh342 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh343 = *fresh342;
                        *fresh342 = (*fresh342).wrapping_offset(1);
                        *fresh343 = *fresh341;
                        let fresh344 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh345 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh346 = *fresh345;
                        *fresh345 = (*fresh345).wrapping_offset(1);
                        *fresh346 = *fresh344;
                        let fresh347 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh348 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh349 = *fresh348;
                        *fresh348 = (*fresh348).wrapping_offset(1);
                        *fresh349 = *fresh347;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh350 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh351 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh352 = *fresh351;
                        *fresh351 = (*fresh351).wrapping_offset(1);
                        *fresh352 = *fresh350;
                        let fresh353 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh354 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh355 = *fresh354;
                        *fresh354 = (*fresh354).wrapping_offset(1);
                        *fresh355 = *fresh353;
                        let fresh356 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh357 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh358 = *fresh357;
                        *fresh357 = (*fresh357).wrapping_offset(1);
                        *fresh358 = *fresh356;
                        let fresh359 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh360 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh361 = *fresh360;
                        *fresh360 = (*fresh360).wrapping_offset(1);
                        *fresh361 = *fresh359;
                    };
                    let fresh362 = addr_of_mut!((*emitter).column);
                    *fresh362 += 1;
                    1_i32 != 0
                })
            {
                return 0_i32;
            }
            spaces = 1_i32;
        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if breaks == 0
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        if (*emitter).line_break as libc::c_uint
                            == YAML_CR_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh363 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh364 = *fresh363;
                            *fresh363 = (*fresh363).wrapping_offset(1);
                            *fresh364 = '\r' as i32 as yaml_char_t;
                        } else if (*emitter).line_break as libc::c_uint
                            == YAML_LN_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh365 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh366 = *fresh365;
                            *fresh365 = (*fresh365).wrapping_offset(1);
                            *fresh366 = '\n' as i32 as yaml_char_t;
                        } else if (*emitter).line_break as libc::c_uint
                            == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh367 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh368 = *fresh367;
                            *fresh367 = (*fresh367).wrapping_offset(1);
                            *fresh368 = '\r' as i32 as yaml_char_t;
                            let fresh369 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh370 = *fresh369;
                            *fresh369 = (*fresh369).wrapping_offset(1);
                            *fresh370 = '\n' as i32 as yaml_char_t;
                        };
                        (*emitter).column = 0_i32;
                        let fresh371 = addr_of_mut!((*emitter).line);
                        *fresh371 += 1;
                        1_i32 != 0
                    })
                {
                    return 0_i32;
                }
            }
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    if ((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0
                    {
                        {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh372 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh373 = *fresh372;
                                *fresh372 = (*fresh372).wrapping_offset(1);
                                *fresh373 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh374 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh375 = *fresh374;
                                *fresh374 = (*fresh374).wrapping_offset(1);
                                *fresh375 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh376 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh377 = *fresh376;
                                *fresh376 = (*fresh376).wrapping_offset(1);
                                *fresh377 = '\r' as i32 as yaml_char_t;
                                let fresh378 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh379 = *fresh378;
                                *fresh378 = (*fresh378).wrapping_offset(1);
                                *fresh379 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0_i32;
                            let fresh380 = addr_of_mut!((*emitter).line);
                            *fresh380 += 1;
                            1_i32 != 0
                        };
                    }
                    string.pointer = string.pointer.wrapping_offset(1);
                    1_i32
                } else {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh381 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh382 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh383 = *fresh382;
                        *fresh382 = (*fresh382).wrapping_offset(1);
                        *fresh383 = *fresh381;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh384 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh385 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh386 = *fresh385;
                        *fresh385 = (*fresh385).wrapping_offset(1);
                        *fresh386 = *fresh384;
                        let fresh387 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh388 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh389 = *fresh388;
                        *fresh388 = (*fresh388).wrapping_offset(1);
                        *fresh389 = *fresh387;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh390 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh391 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh392 = *fresh391;
                        *fresh391 = (*fresh391).wrapping_offset(1);
                        *fresh392 = *fresh390;
                        let fresh393 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh394 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh395 = *fresh394;
                        *fresh394 = (*fresh394).wrapping_offset(1);
                        *fresh395 = *fresh393;
                        let fresh396 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh397 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh398 = *fresh397;
                        *fresh397 = (*fresh397).wrapping_offset(1);
                        *fresh398 = *fresh396;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh399 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh400 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh401 = *fresh400;
                        *fresh400 = (*fresh400).wrapping_offset(1);
                        *fresh401 = *fresh399;
                        let fresh402 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh403 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh404 = *fresh403;
                        *fresh403 = (*fresh403).wrapping_offset(1);
                        *fresh404 = *fresh402;
                        let fresh405 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh406 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh407 = *fresh406;
                        *fresh406 = (*fresh406).wrapping_offset(1);
                        *fresh407 = *fresh405;
                        let fresh408 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh409 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh410 = *fresh409;
                        *fresh409 = (*fresh409).wrapping_offset(1);
                        *fresh410 = *fresh408;
                    };
                    (*emitter).column = 0_i32;
                    let fresh411 = addr_of_mut!((*emitter).line);
                    *fresh411 += 1;
                    1_i32
                }) != 0)
            {
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
            if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\'' as i32 as yaml_char_t as libc::c_int
            {
                if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh412 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh413 = *fresh412;
                        *fresh412 = (*fresh412).wrapping_offset(1);
                        *fresh413 = '\'' as i32 as yaml_char_t;
                        let fresh414 = addr_of_mut!((*emitter).column);
                        *fresh414 += 1;
                        1_i32 != 0
                    })
                {
                    return 0_i32;
                }
            }
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh415 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh416 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh417 = *fresh416;
                        *fresh416 = (*fresh416).wrapping_offset(1);
                        *fresh417 = *fresh415;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh418 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh419 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh420 = *fresh419;
                        *fresh419 = (*fresh419).wrapping_offset(1);
                        *fresh420 = *fresh418;
                        let fresh421 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh422 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh423 = *fresh422;
                        *fresh422 = (*fresh422).wrapping_offset(1);
                        *fresh423 = *fresh421;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh424 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh425 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh426 = *fresh425;
                        *fresh425 = (*fresh425).wrapping_offset(1);
                        *fresh426 = *fresh424;
                        let fresh427 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh428 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh429 = *fresh428;
                        *fresh428 = (*fresh428).wrapping_offset(1);
                        *fresh429 = *fresh427;
                        let fresh430 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh431 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh432 = *fresh431;
                        *fresh431 = (*fresh431).wrapping_offset(1);
                        *fresh432 = *fresh430;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh433 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh434 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh435 = *fresh434;
                        *fresh434 = (*fresh434).wrapping_offset(1);
                        *fresh435 = *fresh433;
                        let fresh436 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh437 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh438 = *fresh437;
                        *fresh437 = (*fresh437).wrapping_offset(1);
                        *fresh438 = *fresh436;
                        let fresh439 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh440 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh441 = *fresh440;
                        *fresh440 = (*fresh440).wrapping_offset(1);
                        *fresh441 = *fresh439;
                        let fresh442 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh443 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh444 = *fresh443;
                        *fresh443 = (*fresh443).wrapping_offset(1);
                        *fresh444 = *fresh442;
                    };
                    let fresh445 = addr_of_mut!((*emitter).column);
                    *fresh445 += 1;
                    1_i32 != 0
                })
            {
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
    let mut string = yaml_string_t {
        start: value,
        end: value.wrapping_offset(length as isize),
        pointer: value,
    };
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
        if !(*string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xa_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int >= 0x20_i32
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int <= 0x7e_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xc2_i32
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int >= 0xa0_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int > 0xc2_i32
                && (*string.pointer.wrapping_offset(0_isize) as libc::c_int) < 0xed_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xed_i32
                && (*string.pointer.wrapping_offset(1_isize) as libc::c_int) < 0xa0_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xee_i32
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int == 0xef_i32
                && !(*string.pointer.wrapping_offset(1_isize) as libc::c_int == 0xbb_i32
                    && *string.pointer.wrapping_offset(2_isize) as libc::c_int == 0xbf_i32)
                && !(*string.pointer.wrapping_offset(1_isize) as libc::c_int == 0xbf_i32
                    && (*string.pointer.wrapping_offset(2_isize) as libc::c_int == 0xbe_i32
                        || *string.pointer.wrapping_offset(2_isize) as libc::c_int == 0xbf_i32)))
            || (*emitter).unicode == 0
                && !(*string.pointer.wrapping_offset(0_isize) as libc::c_int
                    <= '\u{7f}' as i32 as yaml_char_t as libc::c_int)
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -17i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -69i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -65i32 as yaml_char_t as libc::c_int
            || (*string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int)
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '"' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\\' as i32 as yaml_char_t as libc::c_int
        {
            let mut octet: libc::c_uchar;
            let mut width: libc::c_uint;
            let mut value_0: libc::c_uint;
            let mut k: libc::c_int;
            octet = *string.pointer.wrapping_offset(0_isize);
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
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    let fresh446 = addr_of_mut!((*emitter).buffer.pointer);
                    let fresh447 = *fresh446;
                    *fresh446 = (*fresh446).wrapping_offset(1);
                    *fresh447 = '\\' as i32 as yaml_char_t;
                    let fresh448 = addr_of_mut!((*emitter).column);
                    *fresh448 += 1;
                    1_i32 != 0
                })
            {
                return 0_i32;
            }
            match value_0 {
                0 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh449 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh450 = *fresh449;
                            *fresh449 = (*fresh449).wrapping_offset(1);
                            *fresh450 = '0' as i32 as yaml_char_t;
                            let fresh451 = addr_of_mut!((*emitter).column);
                            *fresh451 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                7 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh452 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh453 = *fresh452;
                            *fresh452 = (*fresh452).wrapping_offset(1);
                            *fresh453 = 'a' as i32 as yaml_char_t;
                            let fresh454 = addr_of_mut!((*emitter).column);
                            *fresh454 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                8 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh455 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh456 = *fresh455;
                            *fresh455 = (*fresh455).wrapping_offset(1);
                            *fresh456 = 'b' as i32 as yaml_char_t;
                            let fresh457 = addr_of_mut!((*emitter).column);
                            *fresh457 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                9 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh458 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh459 = *fresh458;
                            *fresh458 = (*fresh458).wrapping_offset(1);
                            *fresh459 = 't' as i32 as yaml_char_t;
                            let fresh460 = addr_of_mut!((*emitter).column);
                            *fresh460 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                10 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh461 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh462 = *fresh461;
                            *fresh461 = (*fresh461).wrapping_offset(1);
                            *fresh462 = 'n' as i32 as yaml_char_t;
                            let fresh463 = addr_of_mut!((*emitter).column);
                            *fresh463 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                11 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh464 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh465 = *fresh464;
                            *fresh464 = (*fresh464).wrapping_offset(1);
                            *fresh465 = 'v' as i32 as yaml_char_t;
                            let fresh466 = addr_of_mut!((*emitter).column);
                            *fresh466 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                12 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh467 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh468 = *fresh467;
                            *fresh467 = (*fresh467).wrapping_offset(1);
                            *fresh468 = 'f' as i32 as yaml_char_t;
                            let fresh469 = addr_of_mut!((*emitter).column);
                            *fresh469 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                13 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh470 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh471 = *fresh470;
                            *fresh470 = (*fresh470).wrapping_offset(1);
                            *fresh471 = 'r' as i32 as yaml_char_t;
                            let fresh472 = addr_of_mut!((*emitter).column);
                            *fresh472 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                27 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh473 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh474 = *fresh473;
                            *fresh473 = (*fresh473).wrapping_offset(1);
                            *fresh474 = 'e' as i32 as yaml_char_t;
                            let fresh475 = addr_of_mut!((*emitter).column);
                            *fresh475 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                34 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh476 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh477 = *fresh476;
                            *fresh476 = (*fresh476).wrapping_offset(1);
                            *fresh477 = '"' as i32 as yaml_char_t;
                            let fresh478 = addr_of_mut!((*emitter).column);
                            *fresh478 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                92 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh479 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh480 = *fresh479;
                            *fresh479 = (*fresh479).wrapping_offset(1);
                            *fresh480 = '\\' as i32 as yaml_char_t;
                            let fresh481 = addr_of_mut!((*emitter).column);
                            *fresh481 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                133 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh482 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh483 = *fresh482;
                            *fresh482 = (*fresh482).wrapping_offset(1);
                            *fresh483 = 'N' as i32 as yaml_char_t;
                            let fresh484 = addr_of_mut!((*emitter).column);
                            *fresh484 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                160 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh485 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh486 = *fresh485;
                            *fresh485 = (*fresh485).wrapping_offset(1);
                            *fresh486 = '_' as i32 as yaml_char_t;
                            let fresh487 = addr_of_mut!((*emitter).column);
                            *fresh487 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                8232 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh488 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh489 = *fresh488;
                            *fresh488 = (*fresh488).wrapping_offset(1);
                            *fresh489 = 'L' as i32 as yaml_char_t;
                            let fresh490 = addr_of_mut!((*emitter).column);
                            *fresh490 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                8233 => {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh491 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh492 = *fresh491;
                            *fresh491 = (*fresh491).wrapping_offset(1);
                            *fresh492 = 'P' as i32 as yaml_char_t;
                            let fresh493 = addr_of_mut!((*emitter).column);
                            *fresh493 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                _ => {
                    if value_0 <= 0xff_i32 as libc::c_uint {
                        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh494 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh495 = *fresh494;
                                *fresh494 = (*fresh494).wrapping_offset(1);
                                *fresh495 = 'x' as i32 as yaml_char_t;
                                let fresh496 = addr_of_mut!((*emitter).column);
                                *fresh496 += 1;
                                1_i32 != 0
                            })
                        {
                            return 0_i32;
                        }
                        width = 2_u32;
                    } else if value_0 <= 0xffff_i32 as libc::c_uint {
                        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh497 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh498 = *fresh497;
                                *fresh497 = (*fresh497).wrapping_offset(1);
                                *fresh498 = 'u' as i32 as yaml_char_t;
                                let fresh499 = addr_of_mut!((*emitter).column);
                                *fresh499 += 1;
                                1_i32 != 0
                            })
                        {
                            return 0_i32;
                        }
                        width = 4_u32;
                    } else {
                        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh500 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh501 = *fresh500;
                                *fresh500 = (*fresh500).wrapping_offset(1);
                                *fresh501 = 'U' as i32 as yaml_char_t;
                                let fresh502 = addr_of_mut!((*emitter).column);
                                *fresh502 += 1;
                                1_i32 != 0
                            })
                        {
                            return 0_i32;
                        }
                        width = 8_u32;
                    }
                    k = width.wrapping_sub(1_u32).wrapping_mul(4_u32) as libc::c_int;
                    while k >= 0_i32 {
                        let digit: libc::c_int =
                            (value_0 >> k & 0xf_i32 as libc::c_uint) as libc::c_int;
                        if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh503 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh504 = *fresh503;
                                *fresh503 = (*fresh503).wrapping_offset(1);
                                *fresh504 = (digit
                                    + (if digit < 10_i32 {
                                        '0' as i32
                                    } else {
                                        'A' as i32 - 10_i32
                                    })) as yaml_char_t;
                                let fresh505 = addr_of_mut!((*emitter).column);
                                *fresh505 += 1;
                                1_i32 != 0
                            })
                        {
                            return 0_i32;
                        }
                        k -= 4_i32;
                    }
                }
            }
            spaces = 0_i32;
        } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.wrapping_offset(-(1_isize))
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                if *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh506 = addr_of_mut!((*emitter).buffer.pointer);
                            let fresh507 = *fresh506;
                            *fresh506 = (*fresh506).wrapping_offset(1);
                            *fresh507 = '\\' as i32 as yaml_char_t;
                            let fresh508 = addr_of_mut!((*emitter).column);
                            *fresh508 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
                string.pointer = string.pointer.wrapping_offset(
                    (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) as isize,
                );
            } else if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh509 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh510 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh511 = *fresh510;
                        *fresh510 = (*fresh510).wrapping_offset(1);
                        *fresh511 = *fresh509;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh512 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh513 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh514 = *fresh513;
                        *fresh513 = (*fresh513).wrapping_offset(1);
                        *fresh514 = *fresh512;
                        let fresh515 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh516 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh517 = *fresh516;
                        *fresh516 = (*fresh516).wrapping_offset(1);
                        *fresh517 = *fresh515;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh518 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh519 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh520 = *fresh519;
                        *fresh519 = (*fresh519).wrapping_offset(1);
                        *fresh520 = *fresh518;
                        let fresh521 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh522 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh523 = *fresh522;
                        *fresh522 = (*fresh522).wrapping_offset(1);
                        *fresh523 = *fresh521;
                        let fresh524 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh525 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh526 = *fresh525;
                        *fresh525 = (*fresh525).wrapping_offset(1);
                        *fresh526 = *fresh524;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh527 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh528 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh529 = *fresh528;
                        *fresh528 = (*fresh528).wrapping_offset(1);
                        *fresh529 = *fresh527;
                        let fresh530 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh531 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh532 = *fresh531;
                        *fresh531 = (*fresh531).wrapping_offset(1);
                        *fresh532 = *fresh530;
                        let fresh533 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh534 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh535 = *fresh534;
                        *fresh534 = (*fresh534).wrapping_offset(1);
                        *fresh535 = *fresh533;
                        let fresh536 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh537 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh538 = *fresh537;
                        *fresh537 = (*fresh537).wrapping_offset(1);
                        *fresh538 = *fresh536;
                    };
                    let fresh539 = addr_of_mut!((*emitter).column);
                    *fresh539 += 1;
                    1_i32 != 0
                })
            {
                return 0_i32;
            }
            spaces = 1_i32;
        } else {
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh540 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh541 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh542 = *fresh541;
                        *fresh541 = (*fresh541).wrapping_offset(1);
                        *fresh542 = *fresh540;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh543 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh544 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh545 = *fresh544;
                        *fresh544 = (*fresh544).wrapping_offset(1);
                        *fresh545 = *fresh543;
                        let fresh546 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh547 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh548 = *fresh547;
                        *fresh547 = (*fresh547).wrapping_offset(1);
                        *fresh548 = *fresh546;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh549 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh550 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh551 = *fresh550;
                        *fresh550 = (*fresh550).wrapping_offset(1);
                        *fresh551 = *fresh549;
                        let fresh552 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh553 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh554 = *fresh553;
                        *fresh553 = (*fresh553).wrapping_offset(1);
                        *fresh554 = *fresh552;
                        let fresh555 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh556 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh557 = *fresh556;
                        *fresh556 = (*fresh556).wrapping_offset(1);
                        *fresh557 = *fresh555;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh558 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh559 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh560 = *fresh559;
                        *fresh559 = (*fresh559).wrapping_offset(1);
                        *fresh560 = *fresh558;
                        let fresh561 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh562 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh563 = *fresh562;
                        *fresh562 = (*fresh562).wrapping_offset(1);
                        *fresh563 = *fresh561;
                        let fresh564 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh565 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh566 = *fresh565;
                        *fresh565 = (*fresh565).wrapping_offset(1);
                        *fresh566 = *fresh564;
                        let fresh567 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh568 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh569 = *fresh568;
                        *fresh568 = (*fresh568).wrapping_offset(1);
                        *fresh569 = *fresh567;
                    };
                    let fresh570 = addr_of_mut!((*emitter).column);
                    *fresh570 += 1;
                    1_i32 != 0
                })
            {
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
    if *string.pointer.wrapping_offset(0_isize) as libc::c_int
        == ' ' as i32 as yaml_char_t as libc::c_int
        || (*string.pointer.wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
    {
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
        if !(*string.pointer.wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
        {
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
            if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
            {
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
    let mut string = yaml_string_t {
        start: value,
        end: value.wrapping_offset(length as isize),
        pointer: value,
    };
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
    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
        || yaml_emitter_flush(emitter) != 0)
        && {
            if (*emitter).line_break as libc::c_uint == YAML_CR_BREAK as libc::c_int as libc::c_uint
            {
                let fresh571 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh572 = *fresh571;
                *fresh571 = (*fresh571).wrapping_offset(1);
                *fresh572 = '\r' as i32 as yaml_char_t;
            } else if (*emitter).line_break as libc::c_uint
                == YAML_LN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh573 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh574 = *fresh573;
                *fresh573 = (*fresh573).wrapping_offset(1);
                *fresh574 = '\n' as i32 as yaml_char_t;
            } else if (*emitter).line_break as libc::c_uint
                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh575 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh576 = *fresh575;
                *fresh575 = (*fresh575).wrapping_offset(1);
                *fresh576 = '\r' as i32 as yaml_char_t;
                let fresh577 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh578 = *fresh577;
                *fresh577 = (*fresh577).wrapping_offset(1);
                *fresh578 = '\n' as i32 as yaml_char_t;
            };
            (*emitter).column = 0_i32;
            let fresh579 = addr_of_mut!((*emitter).line);
            *fresh579 += 1;
            1_i32 != 0
        })
    {
        return 0_i32;
    }
    (*emitter).indention = 1_i32;
    (*emitter).whitespace = 1_i32;
    while string.pointer != string.end {
        if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    if ((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0
                    {
                        {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh580 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh581 = *fresh580;
                                *fresh580 = (*fresh580).wrapping_offset(1);
                                *fresh581 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh582 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh583 = *fresh582;
                                *fresh582 = (*fresh582).wrapping_offset(1);
                                *fresh583 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh584 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh585 = *fresh584;
                                *fresh584 = (*fresh584).wrapping_offset(1);
                                *fresh585 = '\r' as i32 as yaml_char_t;
                                let fresh586 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh587 = *fresh586;
                                *fresh586 = (*fresh586).wrapping_offset(1);
                                *fresh587 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0_i32;
                            let fresh588 = addr_of_mut!((*emitter).line);
                            *fresh588 += 1;
                            1_i32 != 0
                        };
                    }
                    string.pointer = string.pointer.wrapping_offset(1);
                    1_i32
                } else {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh589 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh590 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh591 = *fresh590;
                        *fresh590 = (*fresh590).wrapping_offset(1);
                        *fresh591 = *fresh589;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh592 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh593 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh594 = *fresh593;
                        *fresh593 = (*fresh593).wrapping_offset(1);
                        *fresh594 = *fresh592;
                        let fresh595 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh596 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh597 = *fresh596;
                        *fresh596 = (*fresh596).wrapping_offset(1);
                        *fresh597 = *fresh595;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh598 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh599 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh600 = *fresh599;
                        *fresh599 = (*fresh599).wrapping_offset(1);
                        *fresh600 = *fresh598;
                        let fresh601 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh602 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh603 = *fresh602;
                        *fresh602 = (*fresh602).wrapping_offset(1);
                        *fresh603 = *fresh601;
                        let fresh604 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh605 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh606 = *fresh605;
                        *fresh605 = (*fresh605).wrapping_offset(1);
                        *fresh606 = *fresh604;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh607 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh608 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh609 = *fresh608;
                        *fresh608 = (*fresh608).wrapping_offset(1);
                        *fresh609 = *fresh607;
                        let fresh610 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh611 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh612 = *fresh611;
                        *fresh611 = (*fresh611).wrapping_offset(1);
                        *fresh612 = *fresh610;
                        let fresh613 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh614 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh615 = *fresh614;
                        *fresh614 = (*fresh614).wrapping_offset(1);
                        *fresh615 = *fresh613;
                        let fresh616 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh617 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh618 = *fresh617;
                        *fresh617 = (*fresh617).wrapping_offset(1);
                        *fresh618 = *fresh616;
                    };
                    (*emitter).column = 0_i32;
                    let fresh619 = addr_of_mut!((*emitter).line);
                    *fresh619 += 1;
                    1_i32
                }) != 0)
            {
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
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh620 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh621 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh622 = *fresh621;
                        *fresh621 = (*fresh621).wrapping_offset(1);
                        *fresh622 = *fresh620;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh623 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh624 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh625 = *fresh624;
                        *fresh624 = (*fresh624).wrapping_offset(1);
                        *fresh625 = *fresh623;
                        let fresh626 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh627 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh628 = *fresh627;
                        *fresh627 = (*fresh627).wrapping_offset(1);
                        *fresh628 = *fresh626;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh629 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh630 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh631 = *fresh630;
                        *fresh630 = (*fresh630).wrapping_offset(1);
                        *fresh631 = *fresh629;
                        let fresh632 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh633 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh634 = *fresh633;
                        *fresh633 = (*fresh633).wrapping_offset(1);
                        *fresh634 = *fresh632;
                        let fresh635 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh636 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh637 = *fresh636;
                        *fresh636 = (*fresh636).wrapping_offset(1);
                        *fresh637 = *fresh635;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh638 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh639 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh640 = *fresh639;
                        *fresh639 = (*fresh639).wrapping_offset(1);
                        *fresh640 = *fresh638;
                        let fresh641 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh642 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh643 = *fresh642;
                        *fresh642 = (*fresh642).wrapping_offset(1);
                        *fresh643 = *fresh641;
                        let fresh644 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh645 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh646 = *fresh645;
                        *fresh645 = (*fresh645).wrapping_offset(1);
                        *fresh646 = *fresh644;
                        let fresh647 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh648 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh649 = *fresh648;
                        *fresh648 = (*fresh648).wrapping_offset(1);
                        *fresh649 = *fresh647;
                    };
                    let fresh650 = addr_of_mut!((*emitter).column);
                    *fresh650 += 1;
                    1_i32 != 0
                })
            {
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
    let mut string = yaml_string_t {
        start: value,
        end: value.wrapping_offset(length as isize),
        pointer: value,
    };
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
    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
        || yaml_emitter_flush(emitter) != 0)
        && {
            if (*emitter).line_break as libc::c_uint == YAML_CR_BREAK as libc::c_int as libc::c_uint
            {
                let fresh651 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh652 = *fresh651;
                *fresh651 = (*fresh651).wrapping_offset(1);
                *fresh652 = '\r' as i32 as yaml_char_t;
            } else if (*emitter).line_break as libc::c_uint
                == YAML_LN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh653 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh654 = *fresh653;
                *fresh653 = (*fresh653).wrapping_offset(1);
                *fresh654 = '\n' as i32 as yaml_char_t;
            } else if (*emitter).line_break as libc::c_uint
                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh655 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh656 = *fresh655;
                *fresh655 = (*fresh655).wrapping_offset(1);
                *fresh656 = '\r' as i32 as yaml_char_t;
                let fresh657 = addr_of_mut!((*emitter).buffer.pointer);
                let fresh658 = *fresh657;
                *fresh657 = (*fresh657).wrapping_offset(1);
                *fresh658 = '\n' as i32 as yaml_char_t;
            };
            (*emitter).column = 0_i32;
            let fresh659 = addr_of_mut!((*emitter).line);
            *fresh659 += 1;
            1_i32 != 0
        })
    {
        return 0_i32;
    }
    (*emitter).indention = 1_i32;
    (*emitter).whitespace = 1_i32;
    while string.pointer != string.end {
        if *string.pointer.wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *string.pointer.wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if breaks == 0
                && leading_spaces == 0
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let mut k: libc::c_int = 0_i32;
                while *string.pointer.wrapping_offset(k as isize) as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset((k + 1_i32) as isize) as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset((k + 1_i32) as isize) as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset((k + 2_i32) as isize) as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset((k + 1_i32) as isize) as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *string.pointer.wrapping_offset((k + 2_i32) as isize) as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                {
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
                if !(*string.pointer.wrapping_offset(k as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                        == '\t' as i32 as yaml_char_t as libc::c_int
                    || (*string.pointer.wrapping_offset(k as isize) as libc::c_int
                        == '\r' as i32 as yaml_char_t as libc::c_int
                        || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                            == '\n' as i32 as yaml_char_t as libc::c_int
                        || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                            == -62i32 as yaml_char_t as libc::c_int
                            && *string.pointer.wrapping_offset((k + 1_i32) as isize)
                                as libc::c_int
                                == -123i32 as yaml_char_t as libc::c_int
                        || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                            == -30i32 as yaml_char_t as libc::c_int
                            && *string.pointer.wrapping_offset((k + 1_i32) as isize)
                                as libc::c_int
                                == -128i32 as yaml_char_t as libc::c_int
                            && *string.pointer.wrapping_offset((k + 2_i32) as isize)
                                as libc::c_int
                                == -88i32 as yaml_char_t as libc::c_int
                        || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                            == -30i32 as yaml_char_t as libc::c_int
                            && *string.pointer.wrapping_offset((k + 1_i32) as isize)
                                as libc::c_int
                                == -128i32 as yaml_char_t as libc::c_int
                            && *string.pointer.wrapping_offset((k + 2_i32) as isize)
                                as libc::c_int
                                == -87i32 as yaml_char_t as libc::c_int
                        || *string.pointer.wrapping_offset(k as isize) as libc::c_int
                            == '\0' as i32 as yaml_char_t as libc::c_int))
                {
                    if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh660 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh661 = *fresh660;
                                *fresh660 = (*fresh660).wrapping_offset(1);
                                *fresh661 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh662 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh663 = *fresh662;
                                *fresh662 = (*fresh662).wrapping_offset(1);
                                *fresh663 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh664 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh665 = *fresh664;
                                *fresh664 = (*fresh664).wrapping_offset(1);
                                *fresh665 = '\r' as i32 as yaml_char_t;
                                let fresh666 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh667 = *fresh666;
                                *fresh666 = (*fresh666).wrapping_offset(1);
                                *fresh667 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0_i32;
                            let fresh668 = addr_of_mut!((*emitter).line);
                            *fresh668 += 1;
                            1_i32 != 0
                        })
                    {
                        return 0_i32;
                    }
                }
            }
            if !((((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    if ((*emitter).buffer.pointer).wrapping_offset(5_isize) < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0
                    {
                        {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh669 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh670 = *fresh669;
                                *fresh669 = (*fresh669).wrapping_offset(1);
                                *fresh670 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh671 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh672 = *fresh671;
                                *fresh671 = (*fresh671).wrapping_offset(1);
                                *fresh672 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh673 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh674 = *fresh673;
                                *fresh673 = (*fresh673).wrapping_offset(1);
                                *fresh674 = '\r' as i32 as yaml_char_t;
                                let fresh675 = addr_of_mut!((*emitter).buffer.pointer);
                                let fresh676 = *fresh675;
                                *fresh675 = (*fresh675).wrapping_offset(1);
                                *fresh676 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0_i32;
                            let fresh677 = addr_of_mut!((*emitter).line);
                            *fresh677 += 1;
                            1_i32 != 0
                        };
                    }
                    string.pointer = string.pointer.wrapping_offset(1);
                    1_i32
                } else {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh678 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh679 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh680 = *fresh679;
                        *fresh679 = (*fresh679).wrapping_offset(1);
                        *fresh680 = *fresh678;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh681 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh682 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh683 = *fresh682;
                        *fresh682 = (*fresh682).wrapping_offset(1);
                        *fresh683 = *fresh681;
                        let fresh684 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh685 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh686 = *fresh685;
                        *fresh685 = (*fresh685).wrapping_offset(1);
                        *fresh686 = *fresh684;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh687 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh688 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh689 = *fresh688;
                        *fresh688 = (*fresh688).wrapping_offset(1);
                        *fresh689 = *fresh687;
                        let fresh690 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh691 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh692 = *fresh691;
                        *fresh691 = (*fresh691).wrapping_offset(1);
                        *fresh692 = *fresh690;
                        let fresh693 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh694 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh695 = *fresh694;
                        *fresh694 = (*fresh694).wrapping_offset(1);
                        *fresh695 = *fresh693;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh696 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh697 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh698 = *fresh697;
                        *fresh697 = (*fresh697).wrapping_offset(1);
                        *fresh698 = *fresh696;
                        let fresh699 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh700 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh701 = *fresh700;
                        *fresh700 = (*fresh700).wrapping_offset(1);
                        *fresh701 = *fresh699;
                        let fresh702 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh703 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh704 = *fresh703;
                        *fresh703 = (*fresh703).wrapping_offset(1);
                        *fresh704 = *fresh702;
                        let fresh705 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh706 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh707 = *fresh706;
                        *fresh706 = (*fresh706).wrapping_offset(1);
                        *fresh707 = *fresh705;
                    };
                    (*emitter).column = 0_i32;
                    let fresh708 = addr_of_mut!((*emitter).line);
                    *fresh708 += 1;
                    1_i32
                }) != 0)
            {
                return 0_i32;
            }
            (*emitter).indention = 1_i32;
            breaks = 1_i32;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                leading_spaces = (*string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                    || *string.pointer.wrapping_offset(0_isize) as libc::c_int
                        == '\t' as i32 as yaml_char_t as libc::c_int)
                    as libc::c_int;
            }
            if breaks == 0
                && *string.pointer.wrapping_offset(0_isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                && !(*string.pointer.wrapping_offset(1_isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int)
                && (*emitter).column > (*emitter).best_width
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0_i32;
                }
                string.pointer = string.pointer.wrapping_offset(
                    (if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32
                    {
                        1_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                        == 0xc0_i32
                    {
                        2_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                        == 0xe0_i32
                    {
                        3_i32
                    } else if *string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                        == 0xf0_i32
                    {
                        4_i32
                    } else {
                        0_i32
                    }) as isize,
                );
            } else if !((((*emitter).buffer.pointer).wrapping_offset(5_isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh709 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh710 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh711 = *fresh710;
                        *fresh710 = (*fresh710).wrapping_offset(1);
                        *fresh711 = *fresh709;
                    } else if *string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh712 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh713 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh714 = *fresh713;
                        *fresh713 = (*fresh713).wrapping_offset(1);
                        *fresh714 = *fresh712;
                        let fresh715 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh716 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh717 = *fresh716;
                        *fresh716 = (*fresh716).wrapping_offset(1);
                        *fresh717 = *fresh715;
                    } else if *string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh718 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh719 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh720 = *fresh719;
                        *fresh719 = (*fresh719).wrapping_offset(1);
                        *fresh720 = *fresh718;
                        let fresh721 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh722 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh723 = *fresh722;
                        *fresh722 = (*fresh722).wrapping_offset(1);
                        *fresh723 = *fresh721;
                        let fresh724 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh725 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh726 = *fresh725;
                        *fresh725 = (*fresh725).wrapping_offset(1);
                        *fresh726 = *fresh724;
                    } else if *string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh727 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh728 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh729 = *fresh728;
                        *fresh728 = (*fresh728).wrapping_offset(1);
                        *fresh729 = *fresh727;
                        let fresh730 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh731 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh732 = *fresh731;
                        *fresh731 = (*fresh731).wrapping_offset(1);
                        *fresh732 = *fresh730;
                        let fresh733 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh734 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh735 = *fresh734;
                        *fresh734 = (*fresh734).wrapping_offset(1);
                        *fresh735 = *fresh733;
                        let fresh736 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        let fresh737 = addr_of_mut!((*emitter).buffer.pointer);
                        let fresh738 = *fresh737;
                        *fresh737 = (*fresh737).wrapping_offset(1);
                        *fresh738 = *fresh736;
                    };
                    let fresh739 = addr_of_mut!((*emitter).column);
                    *fresh739 += 1;
                    1_i32 != 0
                })
            {
                return 0_i32;
            }
            (*emitter).indention = 0_i32;
            breaks = 0_i32;
        }
    }
    1_i32
}
