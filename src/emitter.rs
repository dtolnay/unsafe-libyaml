use crate::externs::{strcmp, strlen, strncmp};
use crate::{
    libc, size_t, yaml_char_t, yaml_emitter_flush, yaml_emitter_t, yaml_event_delete, yaml_event_t,
    yaml_free, yaml_queue_extend, yaml_scalar_style_t, yaml_stack_extend, yaml_strdup,
    yaml_string_t, yaml_tag_directive_t, yaml_version_directive_t, PointerExt,
    YAML_ANY_SCALAR_STYLE, YAML_CRLN_BREAK, YAML_CR_BREAK, YAML_DOCUMENT_END_EVENT,
    YAML_DOCUMENT_START_EVENT, YAML_DOUBLE_QUOTED_SCALAR_STYLE, YAML_EMITTER_ERROR,
    YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE, YAML_EMIT_BLOCK_MAPPING_KEY_STATE,
    YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE, YAML_EMIT_BLOCK_MAPPING_VALUE_STATE,
    YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE, YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE,
    YAML_EMIT_DOCUMENT_CONTENT_STATE, YAML_EMIT_DOCUMENT_END_STATE, YAML_EMIT_DOCUMENT_START_STATE,
    YAML_EMIT_END_STATE, YAML_EMIT_FIRST_DOCUMENT_START_STATE,
    YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE, YAML_EMIT_FLOW_MAPPING_KEY_STATE,
    YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE, YAML_EMIT_FLOW_MAPPING_VALUE_STATE,
    YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE, YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE,
    YAML_FLOW_MAPPING_STYLE, YAML_FLOW_SEQUENCE_STYLE, YAML_FOLDED_SCALAR_STYLE,
    YAML_LITERAL_SCALAR_STYLE, YAML_LN_BREAK, YAML_MAPPING_END_EVENT, YAML_MAPPING_START_EVENT,
    YAML_MEMORY_ERROR, YAML_PLAIN_SCALAR_STYLE, YAML_SEQUENCE_END_EVENT, YAML_SEQUENCE_START_EVENT,
    YAML_SINGLE_QUOTED_SCALAR_STYLE, YAML_STREAM_END_EVENT, YAML_STREAM_START_EVENT,
    YAML_UTF8_ENCODING,
};
use std::ptr;
unsafe fn yaml_emitter_set_emitter_error(
    mut emitter: *mut yaml_emitter_t,
    problem: *const libc::c_char,
) -> libc::c_int {
    (*emitter).error = YAML_EMITTER_ERROR;
    let fresh0 = &mut (*emitter).problem;
    *fresh0 = problem;
    0 as libc::c_int
}
pub unsafe fn yaml_emitter_emit(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if if (*emitter).events.tail != (*emitter).events.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*emitter).events.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).events.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).events.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).events.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh1 = &mut (*emitter).events.tail;
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).c_offset(1);
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
        let fresh3 = &mut (*emitter).events.head;
        let fresh4 = *fresh3;
        *fresh3 = (*fresh3).c_offset(1);
        yaml_event_delete(fresh4);
    }
    1 as libc::c_int
}
unsafe fn yaml_emitter_need_more_events(emitter: *mut yaml_emitter_t) -> libc::c_int {
    let mut level: libc::c_int = 0 as libc::c_int;
    let mut event: *mut yaml_event_t;
    if (*emitter).events.head == (*emitter).events.tail {
        return 1 as libc::c_int;
    }
    let accumulate = match (*(*emitter).events.head).type_0 as libc::c_uint {
        3 => 1 as libc::c_int,
        7 => 2 as libc::c_int,
        9 => 3 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
    if ((*emitter).events.tail).c_offset_from((*emitter).events.head) as libc::c_long
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
        event = event.c_offset(1);
    }
    1 as libc::c_int
}
unsafe fn yaml_emitter_append_tag_directive(
    mut emitter: *mut yaml_emitter_t,
    value: yaml_tag_directive_t,
    allow_duplicates: libc::c_int,
) -> libc::c_int {
    let mut tag_directive: *mut yaml_tag_directive_t;
    let mut copy: yaml_tag_directive_t = yaml_tag_directive_t {
        handle: ptr::null_mut::<yaml_char_t>(),
        prefix: ptr::null_mut::<yaml_char_t>(),
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
        tag_directive = tag_directive.c_offset(1);
    }
    copy.handle = yaml_strdup(value.handle);
    copy.prefix = yaml_strdup(value.prefix);
    if (copy.handle).is_null() || (copy.prefix).is_null() {
        (*emitter).error = YAML_MEMORY_ERROR;
    } else if !(if (*emitter).tag_directives.top != (*emitter).tag_directives.end
        || yaml_stack_extend(
            ptr::addr_of_mut!((*emitter).tag_directives.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).tag_directives.top) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).tag_directives.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh5 = &mut (*emitter).tag_directives.top;
        let fresh6 = *fresh5;
        *fresh5 = (*fresh5).c_offset(1);
        *fresh6 = copy;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        return 1 as libc::c_int;
    }
    yaml_free(copy.handle as *mut libc::c_void);
    yaml_free(copy.prefix as *mut libc::c_void);
    0 as libc::c_int
}
unsafe fn yaml_emitter_increase_indent(
    mut emitter: *mut yaml_emitter_t,
    flow: libc::c_int,
    indentless: libc::c_int,
) -> libc::c_int {
    if if (*emitter).indents.top != (*emitter).indents.end
        || yaml_stack_extend(
            ptr::addr_of_mut!((*emitter).indents.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).indents.top) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).indents.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh7 = &mut (*emitter).indents.top;
        let fresh8 = *fresh7;
        *fresh7 = (*fresh7).c_offset(1);
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
        (*emitter).indent = if flow != 0 {
            (*emitter).best_indent
        } else {
            0 as libc::c_int
        };
    } else if indentless == 0 {
        (*emitter).indent += (*emitter).best_indent;
    }
    1 as libc::c_int
}
unsafe fn yaml_emitter_state_machine(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
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
            return yaml_emitter_emit_block_sequence_item(emitter, event, 1 as libc::c_int);
        }
        12 => {
            return yaml_emitter_emit_block_sequence_item(emitter, event, 0 as libc::c_int);
        }
        13 => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 1 as libc::c_int);
        }
        14 => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 0 as libc::c_int);
        }
        15 => {
            return yaml_emitter_emit_block_mapping_value(emitter, event, 1 as libc::c_int);
        }
        16 => {
            return yaml_emitter_emit_block_mapping_value(emitter, event, 0 as libc::c_int);
        }
        17 => {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"expected nothing after STREAM-END\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    0 as libc::c_int
}
unsafe fn yaml_emitter_emit_stream_start(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    (*emitter).open_ended = 0 as libc::c_int;
    if (*event).type_0 as libc::c_uint == YAML_STREAM_START_EVENT as libc::c_int as libc::c_uint {
        if (*emitter).encoding as u64 == 0 {
            (*emitter).encoding = (*event).data.stream_start.encoding;
        }
        if (*emitter).encoding as u64 == 0 {
            (*emitter).encoding = YAML_UTF8_ENCODING;
        }
        if (*emitter).best_indent < 2 as libc::c_int || (*emitter).best_indent > 9 as libc::c_int {
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
        if (*emitter).encoding as libc::c_uint != YAML_UTF8_ENCODING as libc::c_int as libc::c_uint
        {
            if yaml_emitter_write_bom(emitter) == 0 {
                return 0 as libc::c_int;
            }
        }
        (*emitter).state = YAML_EMIT_FIRST_DOCUMENT_START_STATE;
        return 1 as libc::c_int;
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
    if (*event).type_0 as libc::c_uint == YAML_DOCUMENT_START_EVENT as libc::c_int as libc::c_uint {
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
                return 0 as libc::c_int;
            }
        }
        tag_directive = (*event).data.document_start.tag_directives.start;
        while tag_directive != (*event).data.document_start.tag_directives.end {
            if yaml_emitter_analyze_tag_directive(emitter, *tag_directive) == 0 {
                return 0 as libc::c_int;
            }
            if yaml_emitter_append_tag_directive(emitter, *tag_directive, 0 as libc::c_int) == 0 {
                return 0 as libc::c_int;
            }
            tag_directive = tag_directive.c_offset(1);
        }
        tag_directive = default_tag_directives.as_mut_ptr();
        while !((*tag_directive).handle).is_null() {
            if yaml_emitter_append_tag_directive(emitter, *tag_directive, 1 as libc::c_int) == 0 {
                return 0 as libc::c_int;
            }
            tag_directive = tag_directive.c_offset(1);
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
            if (*(*event).data.document_start.version_directive).minor == 1 as libc::c_int {
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
                return 0 as libc::c_int;
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
                tag_directive = tag_directive.c_offset(1);
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
    } else if (*event).type_0 as libc::c_uint
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
            ptr::addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh9 = &mut (*emitter).states.top;
        let fresh10 = *fresh9;
        *fresh9 = (*fresh9).c_offset(1);
        *fresh10 = YAML_EMIT_DOCUMENT_END_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    yaml_emitter_emit_node(
        emitter,
        event,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    )
}
unsafe fn yaml_emitter_emit_document_end(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if (*event).type_0 as libc::c_uint == YAML_DOCUMENT_END_EVENT as libc::c_int as libc::c_uint {
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
            let fresh11 = &mut (*emitter).tag_directives.top;
            *fresh11 = (*fresh11).c_offset(-1);
            let tag_directive: yaml_tag_directive_t = **fresh11;
            yaml_free(tag_directive.handle as *mut libc::c_void);
            yaml_free(tag_directive.prefix as *mut libc::c_void);
        }
        return 1 as libc::c_int;
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
            1 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if yaml_emitter_increase_indent(emitter, 1 as libc::c_int, 0 as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
        let fresh12 = &mut (*emitter).flow_level;
        *fresh12 += 1;
    }
    if (*event).type_0 as libc::c_uint == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint {
        let fresh13 = &mut (*emitter).flow_level;
        *fresh13 -= 1;
        let fresh14 = &mut (*emitter).indents.top;
        *fresh14 = (*fresh14).c_offset(-1);
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
        let fresh15 = &mut (*emitter).states.top;
        *fresh15 = (*fresh15).c_offset(-1);
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
            ptr::addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh16 = &mut (*emitter).states.top;
        let fresh17 = *fresh16;
        *fresh16 = (*fresh16).c_offset(1);
        *fresh17 = YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    yaml_emitter_emit_node(
        emitter,
        event,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    )
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
            1 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if yaml_emitter_increase_indent(emitter, 1 as libc::c_int, 0 as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
        let fresh18 = &mut (*emitter).flow_level;
        *fresh18 += 1;
    }
    if (*event).type_0 as libc::c_uint == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint {
        let fresh19 = &mut (*emitter).flow_level;
        *fresh19 -= 1;
        let fresh20 = &mut (*emitter).indents.top;
        *fresh20 = (*fresh20).c_offset(-1);
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
        let fresh21 = &mut (*emitter).states.top;
        *fresh21 = (*fresh21).c_offset(-1);
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
                ptr::addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh22 = &mut (*emitter).states.top;
            let fresh23 = *fresh22;
            *fresh22 = (*fresh22).c_offset(1);
            *fresh23 = YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE;
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        yaml_emitter_emit_node(
            emitter,
            event,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
        )
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
                ptr::addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh24 = &mut (*emitter).states.top;
            let fresh25 = *fresh24;
            *fresh24 = (*fresh24).c_offset(1);
            *fresh25 = YAML_EMIT_FLOW_MAPPING_VALUE_STATE;
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        yaml_emitter_emit_node(
            emitter,
            event,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        )
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
            ptr::addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh26 = &mut (*emitter).states.top;
        let fresh27 = *fresh26;
        *fresh26 = (*fresh26).c_offset(1);
        *fresh27 = YAML_EMIT_FLOW_MAPPING_KEY_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    yaml_emitter_emit_node(
        emitter,
        event,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    )
}
unsafe fn yaml_emitter_emit_block_sequence_item(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: libc::c_int,
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
    if (*event).type_0 as libc::c_uint == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint {
        let fresh28 = &mut (*emitter).indents.top;
        *fresh28 = (*fresh28).c_offset(-1);
        (*emitter).indent = **fresh28;
        let fresh29 = &mut (*emitter).states.top;
        *fresh29 = (*fresh29).c_offset(-1);
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
            ptr::addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh30 = &mut (*emitter).states.top;
        let fresh31 = *fresh30;
        *fresh30 = (*fresh30).c_offset(1);
        *fresh31 = YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    yaml_emitter_emit_node(
        emitter,
        event,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    )
}
unsafe fn yaml_emitter_emit_block_mapping_key(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: libc::c_int,
) -> libc::c_int {
    if first != 0 {
        if yaml_emitter_increase_indent(emitter, 0 as libc::c_int, 0 as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
    }
    if (*event).type_0 as libc::c_uint == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint {
        let fresh32 = &mut (*emitter).indents.top;
        *fresh32 = (*fresh32).c_offset(-1);
        (*emitter).indent = **fresh32;
        let fresh33 = &mut (*emitter).states.top;
        *fresh33 = (*fresh33).c_offset(-1);
        (*emitter).state = **fresh33;
        return 1 as libc::c_int;
    }
    if yaml_emitter_write_indent(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_check_simple_key(emitter) != 0 {
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                ptr::addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh34 = &mut (*emitter).states.top;
            let fresh35 = *fresh34;
            *fresh34 = (*fresh34).c_offset(1);
            *fresh35 = YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE;
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        yaml_emitter_emit_node(
            emitter,
            event,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
        )
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
                ptr::addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh36 = &mut (*emitter).states.top;
            let fresh37 = *fresh36;
            *fresh36 = (*fresh36).c_offset(1);
            *fresh37 = YAML_EMIT_BLOCK_MAPPING_VALUE_STATE;
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        yaml_emitter_emit_node(
            emitter,
            event,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        )
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
            ptr::addr_of_mut!((*emitter).states.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.top) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*emitter).states.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh38 = &mut (*emitter).states.top;
        let fresh39 = *fresh38;
        *fresh38 = (*fresh38).c_offset(1);
        *fresh39 = YAML_EMIT_BLOCK_MAPPING_KEY_STATE;
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    yaml_emitter_emit_node(
        emitter,
        event,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    )
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
    match (*event).type_0 as libc::c_uint {
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
        return 0 as libc::c_int;
    }
    if (*emitter).simple_key_context != 0 {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh40 = &mut (*emitter).buffer.pointer;
                let fresh41 = *fresh40;
                *fresh40 = (*fresh40).c_offset(1);
                *fresh41 = ' ' as i32 as yaml_char_t;
                let fresh42 = &mut (*emitter).column;
                *fresh42 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    let fresh43 = &mut (*emitter).states.top;
    *fresh43 = (*fresh43).c_offset(-1);
    (*emitter).state = **fresh43;
    1 as libc::c_int
}
unsafe fn yaml_emitter_emit_scalar(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
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
    let fresh44 = &mut (*emitter).indents.top;
    *fresh44 = (*fresh44).c_offset(-1);
    (*emitter).indent = **fresh44;
    let fresh45 = &mut (*emitter).states.top;
    *fresh45 = (*fresh45).c_offset(-1);
    (*emitter).state = **fresh45;
    1 as libc::c_int
}
unsafe fn yaml_emitter_emit_sequence_start(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0 as libc::c_int;
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_emit_mapping_start(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0 as libc::c_int;
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_check_empty_document(_emitter: *mut yaml_emitter_t) -> libc::c_int {
    0 as libc::c_int
}
unsafe fn yaml_emitter_check_empty_sequence(emitter: *mut yaml_emitter_t) -> libc::c_int {
    if (((*emitter).events.tail).c_offset_from((*emitter).events.head) as libc::c_long)
        < 2 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    ((*((*emitter).events.head).c_offset(0 as libc::c_int as isize)).type_0 as libc::c_uint
        == YAML_SEQUENCE_START_EVENT as libc::c_int as libc::c_uint
        && (*((*emitter).events.head).c_offset(1 as libc::c_int as isize)).type_0 as libc::c_uint
            == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint) as libc::c_int
}
unsafe fn yaml_emitter_check_empty_mapping(emitter: *mut yaml_emitter_t) -> libc::c_int {
    if (((*emitter).events.tail).c_offset_from((*emitter).events.head) as libc::c_long)
        < 2 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    ((*((*emitter).events.head).c_offset(0 as libc::c_int as isize)).type_0 as libc::c_uint
        == YAML_MAPPING_START_EVENT as libc::c_int as libc::c_uint
        && (*((*emitter).events.head).c_offset(1 as libc::c_int as isize)).type_0 as libc::c_uint
            == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint) as libc::c_int
}
unsafe fn yaml_emitter_check_simple_key(emitter: *mut yaml_emitter_t) -> libc::c_int {
    let event: *mut yaml_event_t = (*emitter).events.head;
    let mut length: size_t = 0 as libc::c_int as size_t;
    match (*event).type_0 as libc::c_uint {
        5 => {
            length = (length as libc::c_ulong).wrapping_add((*emitter).anchor_data.anchor_length)
                as size_t as size_t;
        }
        6 => {
            if (*emitter).scalar_data.multiline != 0 {
                return 0 as libc::c_int;
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
                return 0 as libc::c_int;
            }
            length = (length as libc::c_ulong).wrapping_add(
                ((*emitter).anchor_data.anchor_length)
                    .wrapping_add((*emitter).tag_data.handle_length)
                    .wrapping_add((*emitter).tag_data.suffix_length),
            ) as size_t as size_t;
        }
        9 => {
            if yaml_emitter_check_empty_mapping(emitter) == 0 {
                return 0 as libc::c_int;
            }
            length = (length as libc::c_ulong).wrapping_add(
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
    1 as libc::c_int
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
        let fresh46 = &mut (*emitter).tag_data.handle;
        *fresh46 = b"!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t;
        (*emitter).tag_data.handle_length = 1 as libc::c_int as size_t;
    }
    (*emitter).scalar_data.style = style;
    1 as libc::c_int
}
unsafe fn yaml_emitter_process_anchor(emitter: *mut yaml_emitter_t) -> libc::c_int {
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
    yaml_emitter_write_anchor(
        emitter,
        (*emitter).anchor_data.anchor,
        (*emitter).anchor_data.anchor_length,
    )
}
unsafe fn yaml_emitter_process_tag(emitter: *mut yaml_emitter_t) -> libc::c_int {
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
    1 as libc::c_int
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
    0 as libc::c_int
}
unsafe fn yaml_emitter_analyze_version_directive(
    emitter: *mut yaml_emitter_t,
    version_directive: yaml_version_directive_t,
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_analyze_tag_directive(
    emitter: *mut yaml_emitter_t,
    tag_directive: yaml_tag_directive_t,
) -> libc::c_int {
    let mut handle: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut prefix: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let handle_length: size_t = strlen(tag_directive.handle as *mut libc::c_char);
    let prefix_length: size_t = strlen(tag_directive.prefix as *mut libc::c_char);
    handle.start = tag_directive.handle;
    handle.end = (tag_directive.handle).c_offset(handle_length as isize);
    handle.pointer = tag_directive.handle;
    prefix.start = tag_directive.prefix;
    prefix.end = (tag_directive.prefix).c_offset(prefix_length as isize);
    prefix.pointer = tag_directive.prefix;
    if handle.start == handle.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must not be empty\0" as *const u8 as *const libc::c_char,
        );
    }
    if *(handle.start).c_offset(0 as libc::c_int as isize) as libc::c_int != '!' as i32 {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must start with '!'\0" as *const u8 as *const libc::c_char,
        );
    }
    if *(handle.end).c_offset(-(1 as libc::c_int) as isize) as libc::c_int != '!' as i32 {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must end with '!'\0" as *const u8 as *const libc::c_char,
        );
    }
    handle.pointer = (handle.pointer).c_offset(1);
    while handle.pointer < (handle.end).c_offset(-(1 as libc::c_int as isize)) {
        if !(*(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            >= '0' as i32 as yaml_char_t as libc::c_int
            && *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                <= '9' as i32 as yaml_char_t as libc::c_int
            || *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'Z' as i32 as yaml_char_t as libc::c_int
            || *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'z' as i32 as yaml_char_t as libc::c_int
            || *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
            || *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
        {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"tag handle must contain alphanumerical characters only\0" as *const u8
                    as *const libc::c_char,
            );
        }
        handle.pointer = (handle.pointer).c_offset(
            (if *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *(handle.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_analyze_anchor(
    mut emitter: *mut yaml_emitter_t,
    anchor: *mut yaml_char_t,
    alias: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let anchor_length: size_t = strlen(anchor as *mut libc::c_char);
    string.start = anchor;
    string.end = anchor.c_offset(anchor_length as isize);
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
        if !(*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            >= '0' as i32 as yaml_char_t as libc::c_int
            && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                <= '9' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'Z' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'z' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
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
        string.pointer = (string.pointer).c_offset(
            (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        );
    }
    let fresh47 = &mut (*emitter).anchor_data.anchor;
    *fresh47 = string.start;
    (*emitter).anchor_data.anchor_length =
        (string.end).c_offset_from(string.start) as libc::c_long as size_t;
    (*emitter).anchor_data.alias = alias;
    1 as libc::c_int
}
unsafe fn yaml_emitter_analyze_tag(
    mut emitter: *mut yaml_emitter_t,
    tag: *mut yaml_char_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut tag_directive: *mut yaml_tag_directive_t;
    let tag_length: size_t = strlen(tag as *mut libc::c_char);
    string.start = tag;
    string.end = tag.c_offset(tag_length as isize);
    string.pointer = tag;
    if string.start == string.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag value must not be empty\0" as *const u8 as *const libc::c_char,
        );
    }
    tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        let prefix_length: size_t = strlen((*tag_directive).prefix as *mut libc::c_char);
        if prefix_length < (string.end).c_offset_from(string.start) as libc::c_long as size_t
            && strncmp(
                (*tag_directive).prefix as *mut libc::c_char,
                string.start as *mut libc::c_char,
                prefix_length,
            ) == 0 as libc::c_int
        {
            let fresh48 = &mut (*emitter).tag_data.handle;
            *fresh48 = (*tag_directive).handle;
            (*emitter).tag_data.handle_length =
                strlen((*tag_directive).handle as *mut libc::c_char);
            let fresh49 = &mut (*emitter).tag_data.suffix;
            *fresh49 = (string.start).c_offset(prefix_length as isize);
            (*emitter).tag_data.suffix_length = ((string.end).c_offset_from(string.start)
                as libc::c_long as libc::c_ulong)
                .wrapping_sub(prefix_length);
            return 1 as libc::c_int;
        }
        tag_directive = tag_directive.c_offset(1);
    }
    let fresh50 = &mut (*emitter).tag_data.suffix;
    *fresh50 = string.start;
    (*emitter).tag_data.suffix_length =
        (string.end).c_offset_from(string.start) as libc::c_long as size_t;
    1 as libc::c_int
}
unsafe fn yaml_emitter_analyze_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
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
    let mut preceded_by_whitespace: libc::c_int;
    let mut followed_by_whitespace: libc::c_int;
    let mut previous_space: libc::c_int = 0 as libc::c_int;
    let mut previous_break: libc::c_int = 0 as libc::c_int;
    string.start = value;
    string.end = value.c_offset(length as isize);
    string.pointer = value;
    let fresh51 = &mut (*emitter).scalar_data.value;
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
    if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '-' as i32 as yaml_char_t as libc::c_int
        && *(string.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && *(string.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
            && *(string.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
            && *(string.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
    {
        block_indicators = 1 as libc::c_int;
        flow_indicators = 1 as libc::c_int;
    }
    preceded_by_whitespace = 1 as libc::c_int;
    followed_by_whitespace = (*(string.pointer).c_offset(
        (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    ) as libc::c_int
        == ' ' as i32 as yaml_char_t as libc::c_int
        || *(string.pointer).c_offset(
            (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        ) as libc::c_int
            == '\t' as i32 as yaml_char_t as libc::c_int
        || (*(string.pointer).c_offset(
            (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        ) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(
                (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            ) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(
                (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            ) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset(
                    ((if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) + 1 as libc::c_int) as isize,
                ) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(
                (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            ) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset(
                    ((if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) + 1 as libc::c_int) as isize,
                ) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset(
                    ((if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) + 2 as libc::c_int) as isize,
                ) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(
                (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            ) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset(
                    ((if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) + 1 as libc::c_int) as isize,
                ) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset(
                    ((if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) + 2 as libc::c_int) as isize,
                ) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(
                (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            ) as libc::c_int
                == '\0' as i32 as yaml_char_t as libc::c_int))
        as libc::c_int;
    while string.pointer != string.end {
        if string.start == string.pointer {
            if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '#' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == ',' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '[' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == ']' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '{' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '}' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '&' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '*' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '!' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '|' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '>' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\'' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '"' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '%' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '@' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '`' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1 as libc::c_int;
                block_indicators = 1 as libc::c_int;
            }
            if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '?' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == ':' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1 as libc::c_int;
                if followed_by_whitespace != 0 {
                    block_indicators = 1 as libc::c_int;
                }
            }
            if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32 as yaml_char_t as libc::c_int
                && followed_by_whitespace != 0
            {
                flow_indicators = 1 as libc::c_int;
                block_indicators = 1 as libc::c_int;
            }
        } else {
            if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '?' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '[' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == ']' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '{' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '}' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1 as libc::c_int;
            }
            if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == ':' as i32 as yaml_char_t as libc::c_int
            {
                flow_indicators = 1 as libc::c_int;
                if followed_by_whitespace != 0 {
                    block_indicators = 1 as libc::c_int;
                }
            }
            if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '#' as i32 as yaml_char_t as libc::c_int
                && preceded_by_whitespace != 0
            {
                flow_indicators = 1 as libc::c_int;
                block_indicators = 1 as libc::c_int;
            }
        }
        if !(*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == 0xa as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                >= 0x20 as libc::c_int
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    <= 0x7e as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == 0xc2 as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    >= 0xa0 as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                > 0xc2 as libc::c_int
                && (*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int)
                    < 0xed as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == 0xed as libc::c_int
                && (*(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int)
                    < 0xa0 as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == 0xee as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == 0xef as libc::c_int
                && !(*(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == 0xbb as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == 0xbf as libc::c_int)
                && !(*(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == 0xbf as libc::c_int
                    && (*(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == 0xbe as libc::c_int
                        || *(string.pointer)
                            .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == 0xbf as libc::c_int)))
            || !(*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                <= '\u{7f}' as i32 as yaml_char_t as libc::c_int)
                && (*emitter).unicode == 0
        {
            special_characters = 1 as libc::c_int;
        }
        if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            line_breaks = 1 as libc::c_int;
        }
        if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if string.start == string.pointer {
                leading_space = 1 as libc::c_int;
            }
            if (string.pointer).c_offset(
                (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
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
        } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if string.start == string.pointer {
                leading_break = 1 as libc::c_int;
            }
            if (string.pointer).c_offset(
                (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
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
        preceded_by_whitespace = (*(string.pointer).c_offset(0 as libc::c_int as isize)
            as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
            || (*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int))
            as libc::c_int;
        string.pointer = (string.pointer).c_offset(
            (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        );
        if string.pointer != string.end {
            followed_by_whitespace = (*(string.pointer).c_offset(
                (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            ) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(
                    (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                ) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
                || (*(string.pointer).c_offset(
                    (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                ) as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(
                        (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf8 as libc::c_int
                            == 0xf0 as libc::c_int
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    ) as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(
                        (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf8 as libc::c_int
                            == 0xf0 as libc::c_int
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    ) as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset(
                            ((if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0x80 as libc::c_int
                                == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) + 1 as libc::c_int) as isize,
                        ) as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(
                        (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf8 as libc::c_int
                            == 0xf0 as libc::c_int
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    ) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset(
                            ((if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0x80 as libc::c_int
                                == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) + 1 as libc::c_int) as isize,
                        ) as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset(
                            ((if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0x80 as libc::c_int
                                == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) + 2 as libc::c_int) as isize,
                        ) as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(
                        (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf8 as libc::c_int
                            == 0xf0 as libc::c_int
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    ) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset(
                            ((if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0x80 as libc::c_int
                                == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) + 1 as libc::c_int) as isize,
                        ) as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset(
                            ((if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0x80 as libc::c_int
                                == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) + 2 as libc::c_int) as isize,
                        ) as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(
                        (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else if *(string.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf8 as libc::c_int
                            == 0xf0 as libc::c_int
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    ) as libc::c_int
                        == '\0' as i32 as yaml_char_t as libc::c_int))
                as libc::c_int;
        }
    }
    (*emitter).scalar_data.multiline = line_breaks;
    (*emitter).scalar_data.flow_plain_allowed = 1 as libc::c_int;
    (*emitter).scalar_data.block_plain_allowed = 1 as libc::c_int;
    (*emitter).scalar_data.single_quoted_allowed = 1 as libc::c_int;
    (*emitter).scalar_data.block_allowed = 1 as libc::c_int;
    if leading_space != 0 || leading_break != 0 || trailing_space != 0 || trailing_break != 0 {
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_analyze_event(
    mut emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    let fresh52 = &mut (*emitter).anchor_data.anchor;
    *fresh52 = ptr::null_mut::<yaml_char_t>();
    (*emitter).anchor_data.anchor_length = 0 as libc::c_int as size_t;
    let fresh53 = &mut (*emitter).tag_data.handle;
    *fresh53 = ptr::null_mut::<yaml_char_t>();
    (*emitter).tag_data.handle_length = 0 as libc::c_int as size_t;
    let fresh54 = &mut (*emitter).tag_data.suffix;
    *fresh54 = ptr::null_mut::<yaml_char_t>();
    (*emitter).tag_data.suffix_length = 0 as libc::c_int as size_t;
    let fresh55 = &mut (*emitter).scalar_data.value;
    *fresh55 = ptr::null_mut::<yaml_char_t>();
    (*emitter).scalar_data.length = 0 as libc::c_int as size_t;
    match (*event).type_0 as libc::c_uint {
        5 => {
            if yaml_emitter_analyze_anchor(emitter, (*event).data.alias.anchor, 1 as libc::c_int)
                == 0
            {
                return 0 as libc::c_int;
            }
            1 as libc::c_int
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
            1 as libc::c_int
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
                && ((*emitter).canonical != 0 || (*event).data.sequence_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.sequence_start.tag) == 0 {
                    return 0 as libc::c_int;
                }
            }
            1 as libc::c_int
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
                && ((*emitter).canonical != 0 || (*event).data.mapping_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.mapping_start.tag) == 0 {
                    return 0 as libc::c_int;
                }
            }
            1 as libc::c_int
        }
        _ => 1 as libc::c_int,
    }
}
unsafe fn yaml_emitter_write_bom(emitter: *mut yaml_emitter_t) -> libc::c_int {
    if !(((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize) < (*emitter).buffer.end
        || yaml_emitter_flush(emitter) != 0)
    {
        return 0 as libc::c_int;
    }
    let fresh56 = &mut (*emitter).buffer.pointer;
    let fresh57 = *fresh56;
    *fresh56 = (*fresh56).c_offset(1);
    *fresh57 = -17i32 as yaml_char_t;
    let fresh58 = &mut (*emitter).buffer.pointer;
    let fresh59 = *fresh58;
    *fresh58 = (*fresh58).c_offset(1);
    *fresh59 = -69i32 as yaml_char_t;
    let fresh60 = &mut (*emitter).buffer.pointer;
    let fresh61 = *fresh60;
    *fresh60 = (*fresh60).c_offset(1);
    *fresh61 = -65i32 as yaml_char_t;
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_indent(mut emitter: *mut yaml_emitter_t) -> libc::c_int {
    let indent: libc::c_int = if (*emitter).indent >= 0 as libc::c_int {
        (*emitter).indent
    } else {
        0 as libc::c_int
    };
    if (*emitter).indention == 0
        || (*emitter).column > indent
        || (*emitter).column == indent && (*emitter).whitespace == 0
    {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if (*emitter).line_break as libc::c_uint
                    == YAML_CR_BREAK as libc::c_int as libc::c_uint
                {
                    let fresh62 = &mut (*emitter).buffer.pointer;
                    let fresh63 = *fresh62;
                    *fresh62 = (*fresh62).c_offset(1);
                    *fresh63 = '\r' as i32 as yaml_char_t;
                } else if (*emitter).line_break as libc::c_uint
                    == YAML_LN_BREAK as libc::c_int as libc::c_uint
                {
                    let fresh64 = &mut (*emitter).buffer.pointer;
                    let fresh65 = *fresh64;
                    *fresh64 = (*fresh64).c_offset(1);
                    *fresh65 = '\n' as i32 as yaml_char_t;
                } else if (*emitter).line_break as libc::c_uint
                    == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                {
                    let fresh66 = &mut (*emitter).buffer.pointer;
                    let fresh67 = *fresh66;
                    *fresh66 = (*fresh66).c_offset(1);
                    *fresh67 = '\r' as i32 as yaml_char_t;
                    let fresh68 = &mut (*emitter).buffer.pointer;
                    let fresh69 = *fresh68;
                    *fresh68 = (*fresh68).c_offset(1);
                    *fresh69 = '\n' as i32 as yaml_char_t;
                };
                (*emitter).column = 0 as libc::c_int;
                let fresh70 = &mut (*emitter).line;
                *fresh70 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while (*emitter).column < indent {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh71 = &mut (*emitter).buffer.pointer;
                let fresh72 = *fresh71;
                *fresh71 = (*fresh71).c_offset(1);
                *fresh72 = ' ' as i32 as yaml_char_t;
                let fresh73 = &mut (*emitter).column;
                *fresh73 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    (*emitter).whitespace = 1 as libc::c_int;
    (*emitter).indention = 1 as libc::c_int;
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_indicator(
    mut emitter: *mut yaml_emitter_t,
    indicator: *const libc::c_char,
    need_whitespace: libc::c_int,
    is_whitespace: libc::c_int,
    is_indention: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let indicator_length: size_t = strlen(indicator);
    string.start = indicator as *mut yaml_char_t;
    string.end = (indicator as *mut yaml_char_t).c_offset(indicator_length as isize);
    string.pointer = indicator as *mut yaml_char_t;
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh74 = &mut (*emitter).buffer.pointer;
                let fresh75 = *fresh74;
                *fresh74 = (*fresh74).c_offset(1);
                *fresh75 = ' ' as i32 as yaml_char_t;
                let fresh76 = &mut (*emitter).column;
                *fresh76 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while string.pointer != string.end {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                    let fresh77 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh78 = &mut (*emitter).buffer.pointer;
                    let fresh79 = *fresh78;
                    *fresh78 = (*fresh78).c_offset(1);
                    *fresh79 = *fresh77;
                } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    let fresh80 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh81 = &mut (*emitter).buffer.pointer;
                    let fresh82 = *fresh81;
                    *fresh81 = (*fresh81).c_offset(1);
                    *fresh82 = *fresh80;
                    let fresh83 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh84 = &mut (*emitter).buffer.pointer;
                    let fresh85 = *fresh84;
                    *fresh84 = (*fresh84).c_offset(1);
                    *fresh85 = *fresh83;
                } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    let fresh86 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh87 = &mut (*emitter).buffer.pointer;
                    let fresh88 = *fresh87;
                    *fresh87 = (*fresh87).c_offset(1);
                    *fresh88 = *fresh86;
                    let fresh89 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh90 = &mut (*emitter).buffer.pointer;
                    let fresh91 = *fresh90;
                    *fresh90 = (*fresh90).c_offset(1);
                    *fresh91 = *fresh89;
                    let fresh92 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh93 = &mut (*emitter).buffer.pointer;
                    let fresh94 = *fresh93;
                    *fresh93 = (*fresh93).c_offset(1);
                    *fresh94 = *fresh92;
                } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    let fresh95 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh96 = &mut (*emitter).buffer.pointer;
                    let fresh97 = *fresh96;
                    *fresh96 = (*fresh96).c_offset(1);
                    *fresh97 = *fresh95;
                    let fresh98 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh99 = &mut (*emitter).buffer.pointer;
                    let fresh100 = *fresh99;
                    *fresh99 = (*fresh99).c_offset(1);
                    *fresh100 = *fresh98;
                    let fresh101 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh102 = &mut (*emitter).buffer.pointer;
                    let fresh103 = *fresh102;
                    *fresh102 = (*fresh102).c_offset(1);
                    *fresh103 = *fresh101;
                    let fresh104 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh105 = &mut (*emitter).buffer.pointer;
                    let fresh106 = *fresh105;
                    *fresh105 = (*fresh105).c_offset(1);
                    *fresh106 = *fresh104;
                };
                let fresh107 = &mut (*emitter).column;
                *fresh107 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    (*emitter).whitespace = is_whitespace;
    (*emitter).indention = ((*emitter).indention != 0 && is_indention != 0) as libc::c_int;
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_anchor(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    string.start = value;
    string.end = value.c_offset(length as isize);
    string.pointer = value;
    while string.pointer != string.end {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                    let fresh108 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh109 = &mut (*emitter).buffer.pointer;
                    let fresh110 = *fresh109;
                    *fresh109 = (*fresh109).c_offset(1);
                    *fresh110 = *fresh108;
                } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    let fresh111 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh112 = &mut (*emitter).buffer.pointer;
                    let fresh113 = *fresh112;
                    *fresh112 = (*fresh112).c_offset(1);
                    *fresh113 = *fresh111;
                    let fresh114 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh115 = &mut (*emitter).buffer.pointer;
                    let fresh116 = *fresh115;
                    *fresh115 = (*fresh115).c_offset(1);
                    *fresh116 = *fresh114;
                } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    let fresh117 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh118 = &mut (*emitter).buffer.pointer;
                    let fresh119 = *fresh118;
                    *fresh118 = (*fresh118).c_offset(1);
                    *fresh119 = *fresh117;
                    let fresh120 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh121 = &mut (*emitter).buffer.pointer;
                    let fresh122 = *fresh121;
                    *fresh121 = (*fresh121).c_offset(1);
                    *fresh122 = *fresh120;
                    let fresh123 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh124 = &mut (*emitter).buffer.pointer;
                    let fresh125 = *fresh124;
                    *fresh124 = (*fresh124).c_offset(1);
                    *fresh125 = *fresh123;
                } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    let fresh126 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh127 = &mut (*emitter).buffer.pointer;
                    let fresh128 = *fresh127;
                    *fresh127 = (*fresh127).c_offset(1);
                    *fresh128 = *fresh126;
                    let fresh129 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh130 = &mut (*emitter).buffer.pointer;
                    let fresh131 = *fresh130;
                    *fresh130 = (*fresh130).c_offset(1);
                    *fresh131 = *fresh129;
                    let fresh132 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh133 = &mut (*emitter).buffer.pointer;
                    let fresh134 = *fresh133;
                    *fresh133 = (*fresh133).c_offset(1);
                    *fresh134 = *fresh132;
                    let fresh135 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh136 = &mut (*emitter).buffer.pointer;
                    let fresh137 = *fresh136;
                    *fresh136 = (*fresh136).c_offset(1);
                    *fresh137 = *fresh135;
                };
                let fresh138 = &mut (*emitter).column;
                *fresh138 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    (*emitter).whitespace = 0 as libc::c_int;
    (*emitter).indention = 0 as libc::c_int;
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_tag_handle(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    string.start = value;
    string.end = value.c_offset(length as isize);
    string.pointer = value;
    if (*emitter).whitespace == 0 {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh139 = &mut (*emitter).buffer.pointer;
                let fresh140 = *fresh139;
                *fresh139 = (*fresh139).c_offset(1);
                *fresh140 = ' ' as i32 as yaml_char_t;
                let fresh141 = &mut (*emitter).column;
                *fresh141 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while string.pointer != string.end {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                    let fresh142 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh143 = &mut (*emitter).buffer.pointer;
                    let fresh144 = *fresh143;
                    *fresh143 = (*fresh143).c_offset(1);
                    *fresh144 = *fresh142;
                } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    let fresh145 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh146 = &mut (*emitter).buffer.pointer;
                    let fresh147 = *fresh146;
                    *fresh146 = (*fresh146).c_offset(1);
                    *fresh147 = *fresh145;
                    let fresh148 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh149 = &mut (*emitter).buffer.pointer;
                    let fresh150 = *fresh149;
                    *fresh149 = (*fresh149).c_offset(1);
                    *fresh150 = *fresh148;
                } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    let fresh151 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh152 = &mut (*emitter).buffer.pointer;
                    let fresh153 = *fresh152;
                    *fresh152 = (*fresh152).c_offset(1);
                    *fresh153 = *fresh151;
                    let fresh154 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh155 = &mut (*emitter).buffer.pointer;
                    let fresh156 = *fresh155;
                    *fresh155 = (*fresh155).c_offset(1);
                    *fresh156 = *fresh154;
                    let fresh157 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh158 = &mut (*emitter).buffer.pointer;
                    let fresh159 = *fresh158;
                    *fresh158 = (*fresh158).c_offset(1);
                    *fresh159 = *fresh157;
                } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    let fresh160 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh161 = &mut (*emitter).buffer.pointer;
                    let fresh162 = *fresh161;
                    *fresh161 = (*fresh161).c_offset(1);
                    *fresh162 = *fresh160;
                    let fresh163 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh164 = &mut (*emitter).buffer.pointer;
                    let fresh165 = *fresh164;
                    *fresh164 = (*fresh164).c_offset(1);
                    *fresh165 = *fresh163;
                    let fresh166 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh167 = &mut (*emitter).buffer.pointer;
                    let fresh168 = *fresh167;
                    *fresh167 = (*fresh167).c_offset(1);
                    *fresh168 = *fresh166;
                    let fresh169 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    let fresh170 = &mut (*emitter).buffer.pointer;
                    let fresh171 = *fresh170;
                    *fresh170 = (*fresh170).c_offset(1);
                    *fresh171 = *fresh169;
                };
                let fresh172 = &mut (*emitter).column;
                *fresh172 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    (*emitter).whitespace = 0 as libc::c_int;
    (*emitter).indention = 0 as libc::c_int;
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_tag_content(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    need_whitespace: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    string.start = value;
    string.end = value.c_offset(length as isize);
    string.pointer = value;
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh173 = &mut (*emitter).buffer.pointer;
                let fresh174 = *fresh173;
                *fresh173 = (*fresh173).c_offset(1);
                *fresh174 = ' ' as i32 as yaml_char_t;
                let fresh175 = &mut (*emitter).column;
                *fresh175 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while string.pointer != string.end {
        if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            >= '0' as i32 as yaml_char_t as libc::c_int
            && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                <= '9' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'Z' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'z' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == ';' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '/' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '?' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == ':' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '@' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '&' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '=' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '+' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '$' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == ',' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '_' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '.' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '~' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '*' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\'' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '(' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == ')' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '[' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == ']' as i32 as yaml_char_t as libc::c_int
        {
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh176 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh177 = &mut (*emitter).buffer.pointer;
                        let fresh178 = *fresh177;
                        *fresh177 = (*fresh177).c_offset(1);
                        *fresh178 = *fresh176;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh179 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh180 = &mut (*emitter).buffer.pointer;
                        let fresh181 = *fresh180;
                        *fresh180 = (*fresh180).c_offset(1);
                        *fresh181 = *fresh179;
                        let fresh182 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh183 = &mut (*emitter).buffer.pointer;
                        let fresh184 = *fresh183;
                        *fresh183 = (*fresh183).c_offset(1);
                        *fresh184 = *fresh182;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh185 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh186 = &mut (*emitter).buffer.pointer;
                        let fresh187 = *fresh186;
                        *fresh186 = (*fresh186).c_offset(1);
                        *fresh187 = *fresh185;
                        let fresh188 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh189 = &mut (*emitter).buffer.pointer;
                        let fresh190 = *fresh189;
                        *fresh189 = (*fresh189).c_offset(1);
                        *fresh190 = *fresh188;
                        let fresh191 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh192 = &mut (*emitter).buffer.pointer;
                        let fresh193 = *fresh192;
                        *fresh192 = (*fresh192).c_offset(1);
                        *fresh193 = *fresh191;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh194 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh195 = &mut (*emitter).buffer.pointer;
                        let fresh196 = *fresh195;
                        *fresh195 = (*fresh195).c_offset(1);
                        *fresh196 = *fresh194;
                        let fresh197 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh198 = &mut (*emitter).buffer.pointer;
                        let fresh199 = *fresh198;
                        *fresh198 = (*fresh198).c_offset(1);
                        *fresh199 = *fresh197;
                        let fresh200 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh201 = &mut (*emitter).buffer.pointer;
                        let fresh202 = *fresh201;
                        *fresh201 = (*fresh201).c_offset(1);
                        *fresh202 = *fresh200;
                        let fresh203 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh204 = &mut (*emitter).buffer.pointer;
                        let fresh205 = *fresh204;
                        *fresh204 = (*fresh204).c_offset(1);
                        *fresh205 = *fresh203;
                    };
                    let fresh206 = &mut (*emitter).column;
                    *fresh206 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
        } else {
            let mut width: libc::c_int = if *(string.pointer).c_offset(0 as libc::c_int as isize)
                as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut value_0: libc::c_uint;
            loop {
                let fresh207 = width;
                width -= 1;
                if !(fresh207 != 0) {
                    break;
                }
                let fresh208 = string.pointer;
                string.pointer = (string.pointer).c_offset(1);
                value_0 = *fresh208 as libc::c_uint;
                if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh209 = &mut (*emitter).buffer.pointer;
                        let fresh210 = *fresh209;
                        *fresh209 = (*fresh209).c_offset(1);
                        *fresh210 = '%' as i32 as yaml_char_t;
                        let fresh211 = &mut (*emitter).column;
                        *fresh211 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
                if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh212 = &mut (*emitter).buffer.pointer;
                        let fresh213 = *fresh212;
                        *fresh212 = (*fresh212).c_offset(1);
                        *fresh213 = (value_0 >> 4 as libc::c_int).wrapping_add(
                            (if (value_0 >> 4 as libc::c_int) < 10 as libc::c_int as libc::c_uint {
                                '0' as i32
                            } else {
                                'A' as i32 - 10 as libc::c_int
                            }) as libc::c_uint,
                        ) as yaml_char_t;
                        let fresh214 = &mut (*emitter).column;
                        *fresh214 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
                if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh215 = &mut (*emitter).buffer.pointer;
                        let fresh216 = *fresh215;
                        *fresh215 = (*fresh215).c_offset(1);
                        *fresh216 = (value_0 & 0xf as libc::c_int as libc::c_uint).wrapping_add(
                            (if (value_0 & 0xf as libc::c_int as libc::c_uint)
                                < 10 as libc::c_int as libc::c_uint
                            {
                                '0' as i32
                            } else {
                                'A' as i32 - 10 as libc::c_int
                            }) as libc::c_uint,
                        ) as yaml_char_t;
                        let fresh217 = &mut (*emitter).column;
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_plain_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut spaces: libc::c_int = 0 as libc::c_int;
    let mut breaks: libc::c_int = 0 as libc::c_int;
    string.start = value;
    string.end = value.c_offset(length as isize);
    string.pointer = value;
    if (*emitter).whitespace == 0 && (length != 0 || (*emitter).flow_level != 0) {
        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh218 = &mut (*emitter).buffer.pointer;
                let fresh219 = *fresh218;
                *fresh218 = (*fresh218).c_offset(1);
                *fresh219 = ' ' as i32 as yaml_char_t;
                let fresh220 = &mut (*emitter).column;
                *fresh220 += 1;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    while string.pointer != string.end {
        if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && !(*(string.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                string.pointer = (string.pointer).c_offset(
                    (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                );
            } else if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh221 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh222 = &mut (*emitter).buffer.pointer;
                        let fresh223 = *fresh222;
                        *fresh222 = (*fresh222).c_offset(1);
                        *fresh223 = *fresh221;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh224 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh225 = &mut (*emitter).buffer.pointer;
                        let fresh226 = *fresh225;
                        *fresh225 = (*fresh225).c_offset(1);
                        *fresh226 = *fresh224;
                        let fresh227 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh228 = &mut (*emitter).buffer.pointer;
                        let fresh229 = *fresh228;
                        *fresh228 = (*fresh228).c_offset(1);
                        *fresh229 = *fresh227;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh230 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh231 = &mut (*emitter).buffer.pointer;
                        let fresh232 = *fresh231;
                        *fresh231 = (*fresh231).c_offset(1);
                        *fresh232 = *fresh230;
                        let fresh233 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh234 = &mut (*emitter).buffer.pointer;
                        let fresh235 = *fresh234;
                        *fresh234 = (*fresh234).c_offset(1);
                        *fresh235 = *fresh233;
                        let fresh236 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh237 = &mut (*emitter).buffer.pointer;
                        let fresh238 = *fresh237;
                        *fresh237 = (*fresh237).c_offset(1);
                        *fresh238 = *fresh236;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh239 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh240 = &mut (*emitter).buffer.pointer;
                        let fresh241 = *fresh240;
                        *fresh240 = (*fresh240).c_offset(1);
                        *fresh241 = *fresh239;
                        let fresh242 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh243 = &mut (*emitter).buffer.pointer;
                        let fresh244 = *fresh243;
                        *fresh243 = (*fresh243).c_offset(1);
                        *fresh244 = *fresh242;
                        let fresh245 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh246 = &mut (*emitter).buffer.pointer;
                        let fresh247 = *fresh246;
                        *fresh246 = (*fresh246).c_offset(1);
                        *fresh247 = *fresh245;
                        let fresh248 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh249 = &mut (*emitter).buffer.pointer;
                        let fresh250 = *fresh249;
                        *fresh249 = (*fresh249).c_offset(1);
                        *fresh250 = *fresh248;
                    };
                    let fresh251 = &mut (*emitter).column;
                    *fresh251 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            spaces = 1 as libc::c_int;
        } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if breaks == 0
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        if (*emitter).line_break as libc::c_uint
                            == YAML_CR_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh252 = &mut (*emitter).buffer.pointer;
                            let fresh253 = *fresh252;
                            *fresh252 = (*fresh252).c_offset(1);
                            *fresh253 = '\r' as i32 as yaml_char_t;
                        } else if (*emitter).line_break as libc::c_uint
                            == YAML_LN_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh254 = &mut (*emitter).buffer.pointer;
                            let fresh255 = *fresh254;
                            *fresh254 = (*fresh254).c_offset(1);
                            *fresh255 = '\n' as i32 as yaml_char_t;
                        } else if (*emitter).line_break as libc::c_uint
                            == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh256 = &mut (*emitter).buffer.pointer;
                            let fresh257 = *fresh256;
                            *fresh256 = (*fresh256).c_offset(1);
                            *fresh257 = '\r' as i32 as yaml_char_t;
                            let fresh258 = &mut (*emitter).buffer.pointer;
                            let fresh259 = *fresh258;
                            *fresh258 = (*fresh258).c_offset(1);
                            *fresh259 = '\n' as i32 as yaml_char_t;
                        };
                        (*emitter).column = 0 as libc::c_int;
                        let fresh260 = &mut (*emitter).line;
                        *fresh260 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
            }
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    if ((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0
                    {
                        {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh261 = &mut (*emitter).buffer.pointer;
                                let fresh262 = *fresh261;
                                *fresh261 = (*fresh261).c_offset(1);
                                *fresh262 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh263 = &mut (*emitter).buffer.pointer;
                                let fresh264 = *fresh263;
                                *fresh263 = (*fresh263).c_offset(1);
                                *fresh264 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh265 = &mut (*emitter).buffer.pointer;
                                let fresh266 = *fresh265;
                                *fresh265 = (*fresh265).c_offset(1);
                                *fresh266 = '\r' as i32 as yaml_char_t;
                                let fresh267 = &mut (*emitter).buffer.pointer;
                                let fresh268 = *fresh267;
                                *fresh267 = (*fresh267).c_offset(1);
                                *fresh268 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let fresh269 = &mut (*emitter).line;
                            *fresh269 += 1;
                            1 as libc::c_int != 0
                        };
                    }
                    string.pointer = (string.pointer).c_offset(1);
                    1 as libc::c_int
                } else {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh270 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh271 = &mut (*emitter).buffer.pointer;
                        let fresh272 = *fresh271;
                        *fresh271 = (*fresh271).c_offset(1);
                        *fresh272 = *fresh270;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh273 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh274 = &mut (*emitter).buffer.pointer;
                        let fresh275 = *fresh274;
                        *fresh274 = (*fresh274).c_offset(1);
                        *fresh275 = *fresh273;
                        let fresh276 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh277 = &mut (*emitter).buffer.pointer;
                        let fresh278 = *fresh277;
                        *fresh277 = (*fresh277).c_offset(1);
                        *fresh278 = *fresh276;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh279 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh280 = &mut (*emitter).buffer.pointer;
                        let fresh281 = *fresh280;
                        *fresh280 = (*fresh280).c_offset(1);
                        *fresh281 = *fresh279;
                        let fresh282 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh283 = &mut (*emitter).buffer.pointer;
                        let fresh284 = *fresh283;
                        *fresh283 = (*fresh283).c_offset(1);
                        *fresh284 = *fresh282;
                        let fresh285 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh286 = &mut (*emitter).buffer.pointer;
                        let fresh287 = *fresh286;
                        *fresh286 = (*fresh286).c_offset(1);
                        *fresh287 = *fresh285;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh288 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh289 = &mut (*emitter).buffer.pointer;
                        let fresh290 = *fresh289;
                        *fresh289 = (*fresh289).c_offset(1);
                        *fresh290 = *fresh288;
                        let fresh291 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh292 = &mut (*emitter).buffer.pointer;
                        let fresh293 = *fresh292;
                        *fresh292 = (*fresh292).c_offset(1);
                        *fresh293 = *fresh291;
                        let fresh294 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh295 = &mut (*emitter).buffer.pointer;
                        let fresh296 = *fresh295;
                        *fresh295 = (*fresh295).c_offset(1);
                        *fresh296 = *fresh294;
                        let fresh297 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh298 = &mut (*emitter).buffer.pointer;
                        let fresh299 = *fresh298;
                        *fresh298 = (*fresh298).c_offset(1);
                        *fresh299 = *fresh297;
                    };
                    (*emitter).column = 0 as libc::c_int;
                    let fresh300 = &mut (*emitter).line;
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
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh301 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh302 = &mut (*emitter).buffer.pointer;
                        let fresh303 = *fresh302;
                        *fresh302 = (*fresh302).c_offset(1);
                        *fresh303 = *fresh301;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh304 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh305 = &mut (*emitter).buffer.pointer;
                        let fresh306 = *fresh305;
                        *fresh305 = (*fresh305).c_offset(1);
                        *fresh306 = *fresh304;
                        let fresh307 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh308 = &mut (*emitter).buffer.pointer;
                        let fresh309 = *fresh308;
                        *fresh308 = (*fresh308).c_offset(1);
                        *fresh309 = *fresh307;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh310 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh311 = &mut (*emitter).buffer.pointer;
                        let fresh312 = *fresh311;
                        *fresh311 = (*fresh311).c_offset(1);
                        *fresh312 = *fresh310;
                        let fresh313 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh314 = &mut (*emitter).buffer.pointer;
                        let fresh315 = *fresh314;
                        *fresh314 = (*fresh314).c_offset(1);
                        *fresh315 = *fresh313;
                        let fresh316 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh317 = &mut (*emitter).buffer.pointer;
                        let fresh318 = *fresh317;
                        *fresh317 = (*fresh317).c_offset(1);
                        *fresh318 = *fresh316;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh319 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh320 = &mut (*emitter).buffer.pointer;
                        let fresh321 = *fresh320;
                        *fresh320 = (*fresh320).c_offset(1);
                        *fresh321 = *fresh319;
                        let fresh322 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh323 = &mut (*emitter).buffer.pointer;
                        let fresh324 = *fresh323;
                        *fresh323 = (*fresh323).c_offset(1);
                        *fresh324 = *fresh322;
                        let fresh325 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh326 = &mut (*emitter).buffer.pointer;
                        let fresh327 = *fresh326;
                        *fresh326 = (*fresh326).c_offset(1);
                        *fresh327 = *fresh325;
                        let fresh328 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh329 = &mut (*emitter).buffer.pointer;
                        let fresh330 = *fresh329;
                        *fresh329 = (*fresh329).c_offset(1);
                        *fresh330 = *fresh328;
                    };
                    let fresh331 = &mut (*emitter).column;
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_single_quoted_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut spaces: libc::c_int = 0 as libc::c_int;
    let mut breaks: libc::c_int = 0 as libc::c_int;
    string.start = value;
    string.end = value.c_offset(length as isize);
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
        if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != (string.end).c_offset(-(1 as libc::c_int as isize))
                && !(*(string.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                string.pointer = (string.pointer).c_offset(
                    (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                );
            } else if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh332 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh333 = &mut (*emitter).buffer.pointer;
                        let fresh334 = *fresh333;
                        *fresh333 = (*fresh333).c_offset(1);
                        *fresh334 = *fresh332;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh335 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh336 = &mut (*emitter).buffer.pointer;
                        let fresh337 = *fresh336;
                        *fresh336 = (*fresh336).c_offset(1);
                        *fresh337 = *fresh335;
                        let fresh338 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh339 = &mut (*emitter).buffer.pointer;
                        let fresh340 = *fresh339;
                        *fresh339 = (*fresh339).c_offset(1);
                        *fresh340 = *fresh338;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh341 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh342 = &mut (*emitter).buffer.pointer;
                        let fresh343 = *fresh342;
                        *fresh342 = (*fresh342).c_offset(1);
                        *fresh343 = *fresh341;
                        let fresh344 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh345 = &mut (*emitter).buffer.pointer;
                        let fresh346 = *fresh345;
                        *fresh345 = (*fresh345).c_offset(1);
                        *fresh346 = *fresh344;
                        let fresh347 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh348 = &mut (*emitter).buffer.pointer;
                        let fresh349 = *fresh348;
                        *fresh348 = (*fresh348).c_offset(1);
                        *fresh349 = *fresh347;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh350 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh351 = &mut (*emitter).buffer.pointer;
                        let fresh352 = *fresh351;
                        *fresh351 = (*fresh351).c_offset(1);
                        *fresh352 = *fresh350;
                        let fresh353 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh354 = &mut (*emitter).buffer.pointer;
                        let fresh355 = *fresh354;
                        *fresh354 = (*fresh354).c_offset(1);
                        *fresh355 = *fresh353;
                        let fresh356 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh357 = &mut (*emitter).buffer.pointer;
                        let fresh358 = *fresh357;
                        *fresh357 = (*fresh357).c_offset(1);
                        *fresh358 = *fresh356;
                        let fresh359 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh360 = &mut (*emitter).buffer.pointer;
                        let fresh361 = *fresh360;
                        *fresh360 = (*fresh360).c_offset(1);
                        *fresh361 = *fresh359;
                    };
                    let fresh362 = &mut (*emitter).column;
                    *fresh362 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            spaces = 1 as libc::c_int;
        } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if breaks == 0
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        if (*emitter).line_break as libc::c_uint
                            == YAML_CR_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh363 = &mut (*emitter).buffer.pointer;
                            let fresh364 = *fresh363;
                            *fresh363 = (*fresh363).c_offset(1);
                            *fresh364 = '\r' as i32 as yaml_char_t;
                        } else if (*emitter).line_break as libc::c_uint
                            == YAML_LN_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh365 = &mut (*emitter).buffer.pointer;
                            let fresh366 = *fresh365;
                            *fresh365 = (*fresh365).c_offset(1);
                            *fresh366 = '\n' as i32 as yaml_char_t;
                        } else if (*emitter).line_break as libc::c_uint
                            == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                        {
                            let fresh367 = &mut (*emitter).buffer.pointer;
                            let fresh368 = *fresh367;
                            *fresh367 = (*fresh367).c_offset(1);
                            *fresh368 = '\r' as i32 as yaml_char_t;
                            let fresh369 = &mut (*emitter).buffer.pointer;
                            let fresh370 = *fresh369;
                            *fresh369 = (*fresh369).c_offset(1);
                            *fresh370 = '\n' as i32 as yaml_char_t;
                        };
                        (*emitter).column = 0 as libc::c_int;
                        let fresh371 = &mut (*emitter).line;
                        *fresh371 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
            }
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    if ((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0
                    {
                        {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh372 = &mut (*emitter).buffer.pointer;
                                let fresh373 = *fresh372;
                                *fresh372 = (*fresh372).c_offset(1);
                                *fresh373 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh374 = &mut (*emitter).buffer.pointer;
                                let fresh375 = *fresh374;
                                *fresh374 = (*fresh374).c_offset(1);
                                *fresh375 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh376 = &mut (*emitter).buffer.pointer;
                                let fresh377 = *fresh376;
                                *fresh376 = (*fresh376).c_offset(1);
                                *fresh377 = '\r' as i32 as yaml_char_t;
                                let fresh378 = &mut (*emitter).buffer.pointer;
                                let fresh379 = *fresh378;
                                *fresh378 = (*fresh378).c_offset(1);
                                *fresh379 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let fresh380 = &mut (*emitter).line;
                            *fresh380 += 1;
                            1 as libc::c_int != 0
                        };
                    }
                    string.pointer = (string.pointer).c_offset(1);
                    1 as libc::c_int
                } else {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh381 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh382 = &mut (*emitter).buffer.pointer;
                        let fresh383 = *fresh382;
                        *fresh382 = (*fresh382).c_offset(1);
                        *fresh383 = *fresh381;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh384 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh385 = &mut (*emitter).buffer.pointer;
                        let fresh386 = *fresh385;
                        *fresh385 = (*fresh385).c_offset(1);
                        *fresh386 = *fresh384;
                        let fresh387 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh388 = &mut (*emitter).buffer.pointer;
                        let fresh389 = *fresh388;
                        *fresh388 = (*fresh388).c_offset(1);
                        *fresh389 = *fresh387;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh390 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh391 = &mut (*emitter).buffer.pointer;
                        let fresh392 = *fresh391;
                        *fresh391 = (*fresh391).c_offset(1);
                        *fresh392 = *fresh390;
                        let fresh393 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh394 = &mut (*emitter).buffer.pointer;
                        let fresh395 = *fresh394;
                        *fresh394 = (*fresh394).c_offset(1);
                        *fresh395 = *fresh393;
                        let fresh396 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh397 = &mut (*emitter).buffer.pointer;
                        let fresh398 = *fresh397;
                        *fresh397 = (*fresh397).c_offset(1);
                        *fresh398 = *fresh396;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh399 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh400 = &mut (*emitter).buffer.pointer;
                        let fresh401 = *fresh400;
                        *fresh400 = (*fresh400).c_offset(1);
                        *fresh401 = *fresh399;
                        let fresh402 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh403 = &mut (*emitter).buffer.pointer;
                        let fresh404 = *fresh403;
                        *fresh403 = (*fresh403).c_offset(1);
                        *fresh404 = *fresh402;
                        let fresh405 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh406 = &mut (*emitter).buffer.pointer;
                        let fresh407 = *fresh406;
                        *fresh406 = (*fresh406).c_offset(1);
                        *fresh407 = *fresh405;
                        let fresh408 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh409 = &mut (*emitter).buffer.pointer;
                        let fresh410 = *fresh409;
                        *fresh409 = (*fresh409).c_offset(1);
                        *fresh410 = *fresh408;
                    };
                    (*emitter).column = 0 as libc::c_int;
                    let fresh411 = &mut (*emitter).line;
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
            if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\'' as i32 as yaml_char_t as libc::c_int
            {
                if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh412 = &mut (*emitter).buffer.pointer;
                        let fresh413 = *fresh412;
                        *fresh412 = (*fresh412).c_offset(1);
                        *fresh413 = '\'' as i32 as yaml_char_t;
                        let fresh414 = &mut (*emitter).column;
                        *fresh414 += 1;
                        1 as libc::c_int != 0
                    })
                {
                    return 0 as libc::c_int;
                }
            }
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh415 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh416 = &mut (*emitter).buffer.pointer;
                        let fresh417 = *fresh416;
                        *fresh416 = (*fresh416).c_offset(1);
                        *fresh417 = *fresh415;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh418 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh419 = &mut (*emitter).buffer.pointer;
                        let fresh420 = *fresh419;
                        *fresh419 = (*fresh419).c_offset(1);
                        *fresh420 = *fresh418;
                        let fresh421 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh422 = &mut (*emitter).buffer.pointer;
                        let fresh423 = *fresh422;
                        *fresh422 = (*fresh422).c_offset(1);
                        *fresh423 = *fresh421;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh424 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh425 = &mut (*emitter).buffer.pointer;
                        let fresh426 = *fresh425;
                        *fresh425 = (*fresh425).c_offset(1);
                        *fresh426 = *fresh424;
                        let fresh427 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh428 = &mut (*emitter).buffer.pointer;
                        let fresh429 = *fresh428;
                        *fresh428 = (*fresh428).c_offset(1);
                        *fresh429 = *fresh427;
                        let fresh430 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh431 = &mut (*emitter).buffer.pointer;
                        let fresh432 = *fresh431;
                        *fresh431 = (*fresh431).c_offset(1);
                        *fresh432 = *fresh430;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh433 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh434 = &mut (*emitter).buffer.pointer;
                        let fresh435 = *fresh434;
                        *fresh434 = (*fresh434).c_offset(1);
                        *fresh435 = *fresh433;
                        let fresh436 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh437 = &mut (*emitter).buffer.pointer;
                        let fresh438 = *fresh437;
                        *fresh437 = (*fresh437).c_offset(1);
                        *fresh438 = *fresh436;
                        let fresh439 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh440 = &mut (*emitter).buffer.pointer;
                        let fresh441 = *fresh440;
                        *fresh440 = (*fresh440).c_offset(1);
                        *fresh441 = *fresh439;
                        let fresh442 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh443 = &mut (*emitter).buffer.pointer;
                        let fresh444 = *fresh443;
                        *fresh443 = (*fresh443).c_offset(1);
                        *fresh444 = *fresh442;
                    };
                    let fresh445 = &mut (*emitter).column;
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_double_quoted_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: libc::c_int,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut spaces: libc::c_int = 0 as libc::c_int;
    string.start = value;
    string.end = value.c_offset(length as isize);
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
        if !(*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == 0xa as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                >= 0x20 as libc::c_int
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    <= 0x7e as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == 0xc2 as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    >= 0xa0 as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                > 0xc2 as libc::c_int
                && (*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int)
                    < 0xed as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == 0xed as libc::c_int
                && (*(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int)
                    < 0xa0 as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == 0xee as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == 0xef as libc::c_int
                && !(*(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == 0xbb as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == 0xbf as libc::c_int)
                && !(*(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == 0xbf as libc::c_int
                    && (*(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == 0xbe as libc::c_int
                        || *(string.pointer)
                            .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == 0xbf as libc::c_int)))
            || (*emitter).unicode == 0
                && !(*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    <= '\u{7f}' as i32 as yaml_char_t as libc::c_int)
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -17i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -69i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -65i32 as yaml_char_t as libc::c_int
            || (*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int)
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '"' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\\' as i32 as yaml_char_t as libc::c_int
        {
            let mut octet: libc::c_uchar;
            let mut width: libc::c_uint;
            let mut value_0: libc::c_uint;
            let mut k: libc::c_int;
            octet = *(string.pointer).c_offset(0 as libc::c_int as isize);
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
            value_0 = (if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
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
                octet = *(string.pointer).c_offset(k as isize);
                value_0 = (value_0 << 6 as libc::c_int)
                    .wrapping_add((octet as libc::c_int & 0x3f as libc::c_int) as libc::c_uint);
                k += 1;
            }
            string.pointer = (string.pointer).c_offset(width as isize);
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    let fresh446 = &mut (*emitter).buffer.pointer;
                    let fresh447 = *fresh446;
                    *fresh446 = (*fresh446).c_offset(1);
                    *fresh447 = '\\' as i32 as yaml_char_t;
                    let fresh448 = &mut (*emitter).column;
                    *fresh448 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            match value_0 {
                0 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh449 = &mut (*emitter).buffer.pointer;
                            let fresh450 = *fresh449;
                            *fresh449 = (*fresh449).c_offset(1);
                            *fresh450 = '0' as i32 as yaml_char_t;
                            let fresh451 = &mut (*emitter).column;
                            *fresh451 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                7 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh452 = &mut (*emitter).buffer.pointer;
                            let fresh453 = *fresh452;
                            *fresh452 = (*fresh452).c_offset(1);
                            *fresh453 = 'a' as i32 as yaml_char_t;
                            let fresh454 = &mut (*emitter).column;
                            *fresh454 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                8 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh455 = &mut (*emitter).buffer.pointer;
                            let fresh456 = *fresh455;
                            *fresh455 = (*fresh455).c_offset(1);
                            *fresh456 = 'b' as i32 as yaml_char_t;
                            let fresh457 = &mut (*emitter).column;
                            *fresh457 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                9 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh458 = &mut (*emitter).buffer.pointer;
                            let fresh459 = *fresh458;
                            *fresh458 = (*fresh458).c_offset(1);
                            *fresh459 = 't' as i32 as yaml_char_t;
                            let fresh460 = &mut (*emitter).column;
                            *fresh460 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                10 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh461 = &mut (*emitter).buffer.pointer;
                            let fresh462 = *fresh461;
                            *fresh461 = (*fresh461).c_offset(1);
                            *fresh462 = 'n' as i32 as yaml_char_t;
                            let fresh463 = &mut (*emitter).column;
                            *fresh463 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                11 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh464 = &mut (*emitter).buffer.pointer;
                            let fresh465 = *fresh464;
                            *fresh464 = (*fresh464).c_offset(1);
                            *fresh465 = 'v' as i32 as yaml_char_t;
                            let fresh466 = &mut (*emitter).column;
                            *fresh466 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                12 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh467 = &mut (*emitter).buffer.pointer;
                            let fresh468 = *fresh467;
                            *fresh467 = (*fresh467).c_offset(1);
                            *fresh468 = 'f' as i32 as yaml_char_t;
                            let fresh469 = &mut (*emitter).column;
                            *fresh469 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                13 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh470 = &mut (*emitter).buffer.pointer;
                            let fresh471 = *fresh470;
                            *fresh470 = (*fresh470).c_offset(1);
                            *fresh471 = 'r' as i32 as yaml_char_t;
                            let fresh472 = &mut (*emitter).column;
                            *fresh472 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                27 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh473 = &mut (*emitter).buffer.pointer;
                            let fresh474 = *fresh473;
                            *fresh473 = (*fresh473).c_offset(1);
                            *fresh474 = 'e' as i32 as yaml_char_t;
                            let fresh475 = &mut (*emitter).column;
                            *fresh475 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                34 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh476 = &mut (*emitter).buffer.pointer;
                            let fresh477 = *fresh476;
                            *fresh476 = (*fresh476).c_offset(1);
                            *fresh477 = '"' as i32 as yaml_char_t;
                            let fresh478 = &mut (*emitter).column;
                            *fresh478 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                92 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh479 = &mut (*emitter).buffer.pointer;
                            let fresh480 = *fresh479;
                            *fresh479 = (*fresh479).c_offset(1);
                            *fresh480 = '\\' as i32 as yaml_char_t;
                            let fresh481 = &mut (*emitter).column;
                            *fresh481 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                133 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh482 = &mut (*emitter).buffer.pointer;
                            let fresh483 = *fresh482;
                            *fresh482 = (*fresh482).c_offset(1);
                            *fresh483 = 'N' as i32 as yaml_char_t;
                            let fresh484 = &mut (*emitter).column;
                            *fresh484 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                160 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh485 = &mut (*emitter).buffer.pointer;
                            let fresh486 = *fresh485;
                            *fresh485 = (*fresh485).c_offset(1);
                            *fresh486 = '_' as i32 as yaml_char_t;
                            let fresh487 = &mut (*emitter).column;
                            *fresh487 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                8232 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh488 = &mut (*emitter).buffer.pointer;
                            let fresh489 = *fresh488;
                            *fresh488 = (*fresh488).c_offset(1);
                            *fresh489 = 'L' as i32 as yaml_char_t;
                            let fresh490 = &mut (*emitter).column;
                            *fresh490 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                8233 => {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh491 = &mut (*emitter).buffer.pointer;
                            let fresh492 = *fresh491;
                            *fresh491 = (*fresh491).c_offset(1);
                            *fresh492 = 'P' as i32 as yaml_char_t;
                            let fresh493 = &mut (*emitter).column;
                            *fresh493 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                _ => {
                    if value_0 <= 0xff as libc::c_int as libc::c_uint {
                        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh494 = &mut (*emitter).buffer.pointer;
                                let fresh495 = *fresh494;
                                *fresh494 = (*fresh494).c_offset(1);
                                *fresh495 = 'x' as i32 as yaml_char_t;
                                let fresh496 = &mut (*emitter).column;
                                *fresh496 += 1;
                                1 as libc::c_int != 0
                            })
                        {
                            return 0 as libc::c_int;
                        }
                        width = 2 as libc::c_int as libc::c_uint;
                    } else if value_0 <= 0xffff as libc::c_int as libc::c_uint {
                        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh497 = &mut (*emitter).buffer.pointer;
                                let fresh498 = *fresh497;
                                *fresh497 = (*fresh497).c_offset(1);
                                *fresh498 = 'u' as i32 as yaml_char_t;
                                let fresh499 = &mut (*emitter).column;
                                *fresh499 += 1;
                                1 as libc::c_int != 0
                            })
                        {
                            return 0 as libc::c_int;
                        }
                        width = 4 as libc::c_int as libc::c_uint;
                    } else {
                        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh500 = &mut (*emitter).buffer.pointer;
                                let fresh501 = *fresh500;
                                *fresh500 = (*fresh500).c_offset(1);
                                *fresh501 = 'U' as i32 as yaml_char_t;
                                let fresh502 = &mut (*emitter).column;
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
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    while k >= 0 as libc::c_int {
                        let digit: libc::c_int =
                            (value_0 >> k & 0xf as libc::c_int as libc::c_uint) as libc::c_int;
                        if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh503 = &mut (*emitter).buffer.pointer;
                                let fresh504 = *fresh503;
                                *fresh503 = (*fresh503).c_offset(1);
                                *fresh504 = (digit
                                    + (if digit < 10 as libc::c_int {
                                        '0' as i32
                                    } else {
                                        'A' as i32 - 10 as libc::c_int
                                    })) as yaml_char_t;
                                let fresh505 = &mut (*emitter).column;
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
        } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
        {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != (string.end).c_offset(-(1 as libc::c_int as isize))
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                if *(string.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh506 = &mut (*emitter).buffer.pointer;
                            let fresh507 = *fresh506;
                            *fresh506 = (*fresh506).c_offset(1);
                            *fresh507 = '\\' as i32 as yaml_char_t;
                            let fresh508 = &mut (*emitter).column;
                            *fresh508 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
                string.pointer = (string.pointer).c_offset(
                    (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                );
            } else if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh509 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh510 = &mut (*emitter).buffer.pointer;
                        let fresh511 = *fresh510;
                        *fresh510 = (*fresh510).c_offset(1);
                        *fresh511 = *fresh509;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh512 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh513 = &mut (*emitter).buffer.pointer;
                        let fresh514 = *fresh513;
                        *fresh513 = (*fresh513).c_offset(1);
                        *fresh514 = *fresh512;
                        let fresh515 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh516 = &mut (*emitter).buffer.pointer;
                        let fresh517 = *fresh516;
                        *fresh516 = (*fresh516).c_offset(1);
                        *fresh517 = *fresh515;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh518 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh519 = &mut (*emitter).buffer.pointer;
                        let fresh520 = *fresh519;
                        *fresh519 = (*fresh519).c_offset(1);
                        *fresh520 = *fresh518;
                        let fresh521 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh522 = &mut (*emitter).buffer.pointer;
                        let fresh523 = *fresh522;
                        *fresh522 = (*fresh522).c_offset(1);
                        *fresh523 = *fresh521;
                        let fresh524 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh525 = &mut (*emitter).buffer.pointer;
                        let fresh526 = *fresh525;
                        *fresh525 = (*fresh525).c_offset(1);
                        *fresh526 = *fresh524;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh527 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh528 = &mut (*emitter).buffer.pointer;
                        let fresh529 = *fresh528;
                        *fresh528 = (*fresh528).c_offset(1);
                        *fresh529 = *fresh527;
                        let fresh530 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh531 = &mut (*emitter).buffer.pointer;
                        let fresh532 = *fresh531;
                        *fresh531 = (*fresh531).c_offset(1);
                        *fresh532 = *fresh530;
                        let fresh533 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh534 = &mut (*emitter).buffer.pointer;
                        let fresh535 = *fresh534;
                        *fresh534 = (*fresh534).c_offset(1);
                        *fresh535 = *fresh533;
                        let fresh536 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh537 = &mut (*emitter).buffer.pointer;
                        let fresh538 = *fresh537;
                        *fresh537 = (*fresh537).c_offset(1);
                        *fresh538 = *fresh536;
                    };
                    let fresh539 = &mut (*emitter).column;
                    *fresh539 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            spaces = 1 as libc::c_int;
        } else {
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh540 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh541 = &mut (*emitter).buffer.pointer;
                        let fresh542 = *fresh541;
                        *fresh541 = (*fresh541).c_offset(1);
                        *fresh542 = *fresh540;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh543 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh544 = &mut (*emitter).buffer.pointer;
                        let fresh545 = *fresh544;
                        *fresh544 = (*fresh544).c_offset(1);
                        *fresh545 = *fresh543;
                        let fresh546 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh547 = &mut (*emitter).buffer.pointer;
                        let fresh548 = *fresh547;
                        *fresh547 = (*fresh547).c_offset(1);
                        *fresh548 = *fresh546;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh549 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh550 = &mut (*emitter).buffer.pointer;
                        let fresh551 = *fresh550;
                        *fresh550 = (*fresh550).c_offset(1);
                        *fresh551 = *fresh549;
                        let fresh552 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh553 = &mut (*emitter).buffer.pointer;
                        let fresh554 = *fresh553;
                        *fresh553 = (*fresh553).c_offset(1);
                        *fresh554 = *fresh552;
                        let fresh555 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh556 = &mut (*emitter).buffer.pointer;
                        let fresh557 = *fresh556;
                        *fresh556 = (*fresh556).c_offset(1);
                        *fresh557 = *fresh555;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh558 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh559 = &mut (*emitter).buffer.pointer;
                        let fresh560 = *fresh559;
                        *fresh559 = (*fresh559).c_offset(1);
                        *fresh560 = *fresh558;
                        let fresh561 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh562 = &mut (*emitter).buffer.pointer;
                        let fresh563 = *fresh562;
                        *fresh562 = (*fresh562).c_offset(1);
                        *fresh563 = *fresh561;
                        let fresh564 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh565 = &mut (*emitter).buffer.pointer;
                        let fresh566 = *fresh565;
                        *fresh565 = (*fresh565).c_offset(1);
                        *fresh566 = *fresh564;
                        let fresh567 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh568 = &mut (*emitter).buffer.pointer;
                        let fresh569 = *fresh568;
                        *fresh568 = (*fresh568).c_offset(1);
                        *fresh569 = *fresh567;
                    };
                    let fresh570 = &mut (*emitter).column;
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_block_scalar_hints(
    mut emitter: *mut yaml_emitter_t,
    mut string: yaml_string_t,
) -> libc::c_int {
    let mut indent_hint: [libc::c_char; 2] = [0; 2];
    let mut chomp_hint: *const libc::c_char = ptr::null::<libc::c_char>();
    if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == ' ' as i32 as yaml_char_t as libc::c_int
        || (*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
    {
        indent_hint[0 as libc::c_int as usize] =
            ('0' as i32 + (*emitter).best_indent as libc::c_char as libc::c_int) as libc::c_char;
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
            string.pointer = (string.pointer).c_offset(-1);
            if !(*string.pointer as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int) {
                break;
            }
        }
        if !(*(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
        {
            chomp_hint = b"-\0" as *const u8 as *const libc::c_char;
        } else if string.start == string.pointer {
            chomp_hint = b"+\0" as *const u8 as *const libc::c_char;
            (*emitter).open_ended = 2 as libc::c_int;
        } else {
            loop {
                string.pointer = (string.pointer).c_offset(-1);
                if !(*string.pointer as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int) {
                    break;
                }
            }
            if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_literal_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut breaks: libc::c_int = 1 as libc::c_int;
    string.start = value;
    string.end = value.c_offset(length as isize);
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
    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize) < (*emitter).buffer.end
        || yaml_emitter_flush(emitter) != 0)
        && {
            if (*emitter).line_break as libc::c_uint == YAML_CR_BREAK as libc::c_int as libc::c_uint
            {
                let fresh571 = &mut (*emitter).buffer.pointer;
                let fresh572 = *fresh571;
                *fresh571 = (*fresh571).c_offset(1);
                *fresh572 = '\r' as i32 as yaml_char_t;
            } else if (*emitter).line_break as libc::c_uint
                == YAML_LN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh573 = &mut (*emitter).buffer.pointer;
                let fresh574 = *fresh573;
                *fresh573 = (*fresh573).c_offset(1);
                *fresh574 = '\n' as i32 as yaml_char_t;
            } else if (*emitter).line_break as libc::c_uint
                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh575 = &mut (*emitter).buffer.pointer;
                let fresh576 = *fresh575;
                *fresh575 = (*fresh575).c_offset(1);
                *fresh576 = '\r' as i32 as yaml_char_t;
                let fresh577 = &mut (*emitter).buffer.pointer;
                let fresh578 = *fresh577;
                *fresh577 = (*fresh577).c_offset(1);
                *fresh578 = '\n' as i32 as yaml_char_t;
            };
            (*emitter).column = 0 as libc::c_int;
            let fresh579 = &mut (*emitter).line;
            *fresh579 += 1;
            1 as libc::c_int != 0
        })
    {
        return 0 as libc::c_int;
    }
    (*emitter).indention = 1 as libc::c_int;
    (*emitter).whitespace = 1 as libc::c_int;
    while string.pointer != string.end {
        if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    if ((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0
                    {
                        {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh580 = &mut (*emitter).buffer.pointer;
                                let fresh581 = *fresh580;
                                *fresh580 = (*fresh580).c_offset(1);
                                *fresh581 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh582 = &mut (*emitter).buffer.pointer;
                                let fresh583 = *fresh582;
                                *fresh582 = (*fresh582).c_offset(1);
                                *fresh583 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh584 = &mut (*emitter).buffer.pointer;
                                let fresh585 = *fresh584;
                                *fresh584 = (*fresh584).c_offset(1);
                                *fresh585 = '\r' as i32 as yaml_char_t;
                                let fresh586 = &mut (*emitter).buffer.pointer;
                                let fresh587 = *fresh586;
                                *fresh586 = (*fresh586).c_offset(1);
                                *fresh587 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let fresh588 = &mut (*emitter).line;
                            *fresh588 += 1;
                            1 as libc::c_int != 0
                        };
                    }
                    string.pointer = (string.pointer).c_offset(1);
                    1 as libc::c_int
                } else {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh589 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh590 = &mut (*emitter).buffer.pointer;
                        let fresh591 = *fresh590;
                        *fresh590 = (*fresh590).c_offset(1);
                        *fresh591 = *fresh589;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh592 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh593 = &mut (*emitter).buffer.pointer;
                        let fresh594 = *fresh593;
                        *fresh593 = (*fresh593).c_offset(1);
                        *fresh594 = *fresh592;
                        let fresh595 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh596 = &mut (*emitter).buffer.pointer;
                        let fresh597 = *fresh596;
                        *fresh596 = (*fresh596).c_offset(1);
                        *fresh597 = *fresh595;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh598 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh599 = &mut (*emitter).buffer.pointer;
                        let fresh600 = *fresh599;
                        *fresh599 = (*fresh599).c_offset(1);
                        *fresh600 = *fresh598;
                        let fresh601 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh602 = &mut (*emitter).buffer.pointer;
                        let fresh603 = *fresh602;
                        *fresh602 = (*fresh602).c_offset(1);
                        *fresh603 = *fresh601;
                        let fresh604 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh605 = &mut (*emitter).buffer.pointer;
                        let fresh606 = *fresh605;
                        *fresh605 = (*fresh605).c_offset(1);
                        *fresh606 = *fresh604;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh607 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh608 = &mut (*emitter).buffer.pointer;
                        let fresh609 = *fresh608;
                        *fresh608 = (*fresh608).c_offset(1);
                        *fresh609 = *fresh607;
                        let fresh610 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh611 = &mut (*emitter).buffer.pointer;
                        let fresh612 = *fresh611;
                        *fresh611 = (*fresh611).c_offset(1);
                        *fresh612 = *fresh610;
                        let fresh613 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh614 = &mut (*emitter).buffer.pointer;
                        let fresh615 = *fresh614;
                        *fresh614 = (*fresh614).c_offset(1);
                        *fresh615 = *fresh613;
                        let fresh616 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh617 = &mut (*emitter).buffer.pointer;
                        let fresh618 = *fresh617;
                        *fresh617 = (*fresh617).c_offset(1);
                        *fresh618 = *fresh616;
                    };
                    (*emitter).column = 0 as libc::c_int;
                    let fresh619 = &mut (*emitter).line;
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
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh620 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh621 = &mut (*emitter).buffer.pointer;
                        let fresh622 = *fresh621;
                        *fresh621 = (*fresh621).c_offset(1);
                        *fresh622 = *fresh620;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh623 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh624 = &mut (*emitter).buffer.pointer;
                        let fresh625 = *fresh624;
                        *fresh624 = (*fresh624).c_offset(1);
                        *fresh625 = *fresh623;
                        let fresh626 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh627 = &mut (*emitter).buffer.pointer;
                        let fresh628 = *fresh627;
                        *fresh627 = (*fresh627).c_offset(1);
                        *fresh628 = *fresh626;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh629 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh630 = &mut (*emitter).buffer.pointer;
                        let fresh631 = *fresh630;
                        *fresh630 = (*fresh630).c_offset(1);
                        *fresh631 = *fresh629;
                        let fresh632 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh633 = &mut (*emitter).buffer.pointer;
                        let fresh634 = *fresh633;
                        *fresh633 = (*fresh633).c_offset(1);
                        *fresh634 = *fresh632;
                        let fresh635 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh636 = &mut (*emitter).buffer.pointer;
                        let fresh637 = *fresh636;
                        *fresh636 = (*fresh636).c_offset(1);
                        *fresh637 = *fresh635;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh638 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh639 = &mut (*emitter).buffer.pointer;
                        let fresh640 = *fresh639;
                        *fresh639 = (*fresh639).c_offset(1);
                        *fresh640 = *fresh638;
                        let fresh641 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh642 = &mut (*emitter).buffer.pointer;
                        let fresh643 = *fresh642;
                        *fresh642 = (*fresh642).c_offset(1);
                        *fresh643 = *fresh641;
                        let fresh644 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh645 = &mut (*emitter).buffer.pointer;
                        let fresh646 = *fresh645;
                        *fresh645 = (*fresh645).c_offset(1);
                        *fresh646 = *fresh644;
                        let fresh647 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh648 = &mut (*emitter).buffer.pointer;
                        let fresh649 = *fresh648;
                        *fresh648 = (*fresh648).c_offset(1);
                        *fresh649 = *fresh647;
                    };
                    let fresh650 = &mut (*emitter).column;
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
    1 as libc::c_int
}
unsafe fn yaml_emitter_write_folded_scalar(
    mut emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> libc::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut breaks: libc::c_int = 1 as libc::c_int;
    let mut leading_spaces: libc::c_int = 1 as libc::c_int;
    string.start = value;
    string.end = value.c_offset(length as isize);
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
    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize) < (*emitter).buffer.end
        || yaml_emitter_flush(emitter) != 0)
        && {
            if (*emitter).line_break as libc::c_uint == YAML_CR_BREAK as libc::c_int as libc::c_uint
            {
                let fresh651 = &mut (*emitter).buffer.pointer;
                let fresh652 = *fresh651;
                *fresh651 = (*fresh651).c_offset(1);
                *fresh652 = '\r' as i32 as yaml_char_t;
            } else if (*emitter).line_break as libc::c_uint
                == YAML_LN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh653 = &mut (*emitter).buffer.pointer;
                let fresh654 = *fresh653;
                *fresh653 = (*fresh653).c_offset(1);
                *fresh654 = '\n' as i32 as yaml_char_t;
            } else if (*emitter).line_break as libc::c_uint
                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
            {
                let fresh655 = &mut (*emitter).buffer.pointer;
                let fresh656 = *fresh655;
                *fresh655 = (*fresh655).c_offset(1);
                *fresh656 = '\r' as i32 as yaml_char_t;
                let fresh657 = &mut (*emitter).buffer.pointer;
                let fresh658 = *fresh657;
                *fresh657 = (*fresh657).c_offset(1);
                *fresh658 = '\n' as i32 as yaml_char_t;
            };
            (*emitter).column = 0 as libc::c_int;
            let fresh659 = &mut (*emitter).line;
            *fresh659 += 1;
            1 as libc::c_int != 0
        })
    {
        return 0 as libc::c_int;
    }
    (*emitter).indention = 1 as libc::c_int;
    (*emitter).whitespace = 1 as libc::c_int;
    while string.pointer != string.end {
        if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *(string.pointer).c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            if breaks == 0
                && leading_spaces == 0
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let mut k: libc::c_int = 0 as libc::c_int;
                while *(string.pointer).c_offset(k as isize) as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(k as isize) as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(k as isize) as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset((k + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(k as isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset((k + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset((k + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(k as isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset((k + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *(string.pointer).c_offset((k + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                {
                    k += if *(string.pointer).c_offset(k as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(k as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(k as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(k as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                }
                if !(*(string.pointer).c_offset(k as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(k as isize) as libc::c_int
                        == '\t' as i32 as yaml_char_t as libc::c_int
                    || (*(string.pointer).c_offset(k as isize) as libc::c_int
                        == '\r' as i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).c_offset(k as isize) as libc::c_int
                            == '\n' as i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).c_offset(k as isize) as libc::c_int
                            == -62i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).c_offset((k + 1 as libc::c_int) as isize)
                                as libc::c_int
                                == -123i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).c_offset(k as isize) as libc::c_int
                            == -30i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).c_offset((k + 1 as libc::c_int) as isize)
                                as libc::c_int
                                == -128i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).c_offset((k + 2 as libc::c_int) as isize)
                                as libc::c_int
                                == -88i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).c_offset(k as isize) as libc::c_int
                            == -30i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).c_offset((k + 1 as libc::c_int) as isize)
                                as libc::c_int
                                == -128i32 as yaml_char_t as libc::c_int
                            && *(string.pointer).c_offset((k + 2 as libc::c_int) as isize)
                                as libc::c_int
                                == -87i32 as yaml_char_t as libc::c_int
                        || *(string.pointer).c_offset(k as isize) as libc::c_int
                            == '\0' as i32 as yaml_char_t as libc::c_int))
                {
                    if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh660 = &mut (*emitter).buffer.pointer;
                                let fresh661 = *fresh660;
                                *fresh660 = (*fresh660).c_offset(1);
                                *fresh661 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh662 = &mut (*emitter).buffer.pointer;
                                let fresh663 = *fresh662;
                                *fresh662 = (*fresh662).c_offset(1);
                                *fresh663 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh664 = &mut (*emitter).buffer.pointer;
                                let fresh665 = *fresh664;
                                *fresh664 = (*fresh664).c_offset(1);
                                *fresh665 = '\r' as i32 as yaml_char_t;
                                let fresh666 = &mut (*emitter).buffer.pointer;
                                let fresh667 = *fresh666;
                                *fresh666 = (*fresh666).c_offset(1);
                                *fresh667 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let fresh668 = &mut (*emitter).line;
                            *fresh668 += 1;
                            1 as libc::c_int != 0
                        })
                    {
                        return 0 as libc::c_int;
                    }
                }
            }
            if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                {
                    if ((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0
                    {
                        {
                            if (*emitter).line_break as libc::c_uint
                                == YAML_CR_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh669 = &mut (*emitter).buffer.pointer;
                                let fresh670 = *fresh669;
                                *fresh669 = (*fresh669).c_offset(1);
                                *fresh670 = '\r' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_LN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh671 = &mut (*emitter).buffer.pointer;
                                let fresh672 = *fresh671;
                                *fresh671 = (*fresh671).c_offset(1);
                                *fresh672 = '\n' as i32 as yaml_char_t;
                            } else if (*emitter).line_break as libc::c_uint
                                == YAML_CRLN_BREAK as libc::c_int as libc::c_uint
                            {
                                let fresh673 = &mut (*emitter).buffer.pointer;
                                let fresh674 = *fresh673;
                                *fresh673 = (*fresh673).c_offset(1);
                                *fresh674 = '\r' as i32 as yaml_char_t;
                                let fresh675 = &mut (*emitter).buffer.pointer;
                                let fresh676 = *fresh675;
                                *fresh675 = (*fresh675).c_offset(1);
                                *fresh676 = '\n' as i32 as yaml_char_t;
                            };
                            (*emitter).column = 0 as libc::c_int;
                            let fresh677 = &mut (*emitter).line;
                            *fresh677 += 1;
                            1 as libc::c_int != 0
                        };
                    }
                    string.pointer = (string.pointer).c_offset(1);
                    1 as libc::c_int
                } else {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh678 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh679 = &mut (*emitter).buffer.pointer;
                        let fresh680 = *fresh679;
                        *fresh679 = (*fresh679).c_offset(1);
                        *fresh680 = *fresh678;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh681 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh682 = &mut (*emitter).buffer.pointer;
                        let fresh683 = *fresh682;
                        *fresh682 = (*fresh682).c_offset(1);
                        *fresh683 = *fresh681;
                        let fresh684 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh685 = &mut (*emitter).buffer.pointer;
                        let fresh686 = *fresh685;
                        *fresh685 = (*fresh685).c_offset(1);
                        *fresh686 = *fresh684;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh687 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh688 = &mut (*emitter).buffer.pointer;
                        let fresh689 = *fresh688;
                        *fresh688 = (*fresh688).c_offset(1);
                        *fresh689 = *fresh687;
                        let fresh690 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh691 = &mut (*emitter).buffer.pointer;
                        let fresh692 = *fresh691;
                        *fresh691 = (*fresh691).c_offset(1);
                        *fresh692 = *fresh690;
                        let fresh693 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh694 = &mut (*emitter).buffer.pointer;
                        let fresh695 = *fresh694;
                        *fresh694 = (*fresh694).c_offset(1);
                        *fresh695 = *fresh693;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh696 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh697 = &mut (*emitter).buffer.pointer;
                        let fresh698 = *fresh697;
                        *fresh697 = (*fresh697).c_offset(1);
                        *fresh698 = *fresh696;
                        let fresh699 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh700 = &mut (*emitter).buffer.pointer;
                        let fresh701 = *fresh700;
                        *fresh700 = (*fresh700).c_offset(1);
                        *fresh701 = *fresh699;
                        let fresh702 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh703 = &mut (*emitter).buffer.pointer;
                        let fresh704 = *fresh703;
                        *fresh703 = (*fresh703).c_offset(1);
                        *fresh704 = *fresh702;
                        let fresh705 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh706 = &mut (*emitter).buffer.pointer;
                        let fresh707 = *fresh706;
                        *fresh706 = (*fresh706).c_offset(1);
                        *fresh707 = *fresh705;
                    };
                    (*emitter).column = 0 as libc::c_int;
                    let fresh708 = &mut (*emitter).line;
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
                leading_spaces = (*(string.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                    || *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        == '\t' as i32 as yaml_char_t as libc::c_int)
                    as libc::c_int;
            }
            if breaks == 0
                && *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                && !(*(string.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int)
                && (*emitter).column > (*emitter).best_width
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as libc::c_int;
                }
                string.pointer = (string.pointer).c_offset(
                    (if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *(string.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                );
            } else if !((((*emitter).buffer.pointer).c_offset(5 as libc::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                        let fresh709 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh710 = &mut (*emitter).buffer.pointer;
                        let fresh711 = *fresh710;
                        *fresh710 = (*fresh710).c_offset(1);
                        *fresh711 = *fresh709;
                    } else if *string.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh712 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh713 = &mut (*emitter).buffer.pointer;
                        let fresh714 = *fresh713;
                        *fresh713 = (*fresh713).c_offset(1);
                        *fresh714 = *fresh712;
                        let fresh715 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh716 = &mut (*emitter).buffer.pointer;
                        let fresh717 = *fresh716;
                        *fresh716 = (*fresh716).c_offset(1);
                        *fresh717 = *fresh715;
                    } else if *string.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh718 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh719 = &mut (*emitter).buffer.pointer;
                        let fresh720 = *fresh719;
                        *fresh719 = (*fresh719).c_offset(1);
                        *fresh720 = *fresh718;
                        let fresh721 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh722 = &mut (*emitter).buffer.pointer;
                        let fresh723 = *fresh722;
                        *fresh722 = (*fresh722).c_offset(1);
                        *fresh723 = *fresh721;
                        let fresh724 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh725 = &mut (*emitter).buffer.pointer;
                        let fresh726 = *fresh725;
                        *fresh725 = (*fresh725).c_offset(1);
                        *fresh726 = *fresh724;
                    } else if *string.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh727 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh728 = &mut (*emitter).buffer.pointer;
                        let fresh729 = *fresh728;
                        *fresh728 = (*fresh728).c_offset(1);
                        *fresh729 = *fresh727;
                        let fresh730 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh731 = &mut (*emitter).buffer.pointer;
                        let fresh732 = *fresh731;
                        *fresh731 = (*fresh731).c_offset(1);
                        *fresh732 = *fresh730;
                        let fresh733 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh734 = &mut (*emitter).buffer.pointer;
                        let fresh735 = *fresh734;
                        *fresh734 = (*fresh734).c_offset(1);
                        *fresh735 = *fresh733;
                        let fresh736 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        let fresh737 = &mut (*emitter).buffer.pointer;
                        let fresh738 = *fresh737;
                        *fresh737 = (*fresh737).c_offset(1);
                        *fresh738 = *fresh736;
                    };
                    let fresh739 = &mut (*emitter).column;
                    *fresh739 += 1;
                    1 as libc::c_int != 0
                })
            {
                return 0 as libc::c_int;
            }
            (*emitter).indention = 0 as libc::c_int;
            breaks = 0 as libc::c_int;
        }
    }
    1 as libc::c_int
}
