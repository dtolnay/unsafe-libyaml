use crate::api::{yaml_free, yaml_malloc, yaml_stack_extend, yaml_strdup};
use crate::externs::{memcpy, memset, strcmp, strlen};
use crate::scanner::yaml_parser_fetch_more_tokens;
use crate::success::{Success, FAIL, OK};
use crate::yaml::{size_t, yaml_char_t};
use crate::{
    libc, yaml_event_t, yaml_mark_t, yaml_parser_t, yaml_tag_directive_t, yaml_token_t,
    yaml_version_directive_t, YAML_ALIAS_EVENT, YAML_ALIAS_TOKEN, YAML_ANCHOR_TOKEN,
    YAML_BLOCK_END_TOKEN, YAML_BLOCK_ENTRY_TOKEN, YAML_BLOCK_MAPPING_START_TOKEN,
    YAML_BLOCK_MAPPING_STYLE, YAML_BLOCK_SEQUENCE_START_TOKEN, YAML_BLOCK_SEQUENCE_STYLE,
    YAML_DOCUMENT_END_EVENT, YAML_DOCUMENT_END_TOKEN, YAML_DOCUMENT_START_EVENT,
    YAML_DOCUMENT_START_TOKEN, YAML_FLOW_ENTRY_TOKEN, YAML_FLOW_MAPPING_END_TOKEN,
    YAML_FLOW_MAPPING_START_TOKEN, YAML_FLOW_MAPPING_STYLE, YAML_FLOW_SEQUENCE_END_TOKEN,
    YAML_FLOW_SEQUENCE_START_TOKEN, YAML_FLOW_SEQUENCE_STYLE, YAML_KEY_TOKEN,
    YAML_MAPPING_END_EVENT, YAML_MAPPING_START_EVENT, YAML_MEMORY_ERROR, YAML_PARSER_ERROR,
    YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE, YAML_PARSE_BLOCK_MAPPING_KEY_STATE,
    YAML_PARSE_BLOCK_MAPPING_VALUE_STATE, YAML_PARSE_BLOCK_NODE_STATE,
    YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE, YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE,
    YAML_PARSE_DOCUMENT_CONTENT_STATE, YAML_PARSE_DOCUMENT_END_STATE,
    YAML_PARSE_DOCUMENT_START_STATE, YAML_PARSE_END_STATE,
    YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE, YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE,
    YAML_PARSE_FLOW_MAPPING_KEY_STATE, YAML_PARSE_FLOW_MAPPING_VALUE_STATE,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE, YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE,
    YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE, YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE,
    YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE, YAML_PLAIN_SCALAR_STYLE, YAML_SCALAR_EVENT,
    YAML_SCALAR_TOKEN, YAML_SEQUENCE_END_EVENT, YAML_SEQUENCE_START_EVENT, YAML_STREAM_END_EVENT,
    YAML_STREAM_END_TOKEN, YAML_STREAM_START_EVENT, YAML_STREAM_START_TOKEN,
    YAML_TAG_DIRECTIVE_TOKEN, YAML_TAG_TOKEN, YAML_VALUE_TOKEN, YAML_VERSION_DIRECTIVE_TOKEN,
};
use core::mem::size_of;
use core::ptr::{self, addr_of_mut};

unsafe fn PEEK_TOKEN(parser: *mut yaml_parser_t) -> *mut yaml_token_t {
    if (*parser).token_available != 0 || yaml_parser_fetch_more_tokens(parser) != 0 {
        (*parser).tokens.head
    } else {
        ptr::null_mut::<yaml_token_t>()
    }
}

unsafe fn SKIP_TOKEN(parser: *mut yaml_parser_t) {
    (*parser).token_available = 0_i32;
    let fresh3 = addr_of_mut!((*parser).tokens_parsed);
    *fresh3 = (*fresh3).wrapping_add(1);
    (*parser).stream_end_produced = ((*(*parser).tokens.head).type_ as libc::c_uint
        == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint)
        as libc::c_int;
    let fresh4 = addr_of_mut!((*parser).tokens.head);
    *fresh4 = (*fresh4).wrapping_offset(1);
}

/// Parse the input stream and produce the next parsing event.
///
/// Call the function subsequently to produce a sequence of events corresponding
/// to the input stream. The initial event has the type YAML_STREAM_START_EVENT
/// while the ending event has the type YAML_STREAM_END_EVENT.
///
/// An application is responsible for freeing any buffers associated with the
/// produced event object using the yaml_event_delete() function.
///
/// An application must not alternate the calls of yaml_parser_parse() with the
/// calls of yaml_parser_scan() or yaml_parser_load(). Doing this will break the
/// parser.
pub unsafe fn yaml_parser_parse(parser: *mut yaml_parser_t, event: *mut yaml_event_t) -> Success {
    __assert!(!parser.is_null());
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    if (*parser).stream_end_produced != 0
        || (*parser).error as libc::c_uint != 0
        || (*parser).state as libc::c_uint == YAML_PARSE_END_STATE as libc::c_int as libc::c_uint
    {
        return OK;
    }
    yaml_parser_state_machine(parser, event)
}

unsafe fn yaml_parser_set_parser_error(
    mut parser: *mut yaml_parser_t,
    problem: *const libc::c_char,
    problem_mark: yaml_mark_t,
) -> Success {
    (*parser).error = YAML_PARSER_ERROR;
    let fresh0 = addr_of_mut!((*parser).problem);
    *fresh0 = problem;
    (*parser).problem_mark = problem_mark;
    FAIL
}

unsafe fn yaml_parser_set_parser_error_context(
    mut parser: *mut yaml_parser_t,
    context: *const libc::c_char,
    context_mark: yaml_mark_t,
    problem: *const libc::c_char,
    problem_mark: yaml_mark_t,
) -> Success {
    (*parser).error = YAML_PARSER_ERROR;
    let fresh1 = addr_of_mut!((*parser).context);
    *fresh1 = context;
    (*parser).context_mark = context_mark;
    let fresh2 = addr_of_mut!((*parser).problem);
    *fresh2 = problem;
    (*parser).problem_mark = problem_mark;
    FAIL
}

unsafe fn yaml_parser_state_machine(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> Success {
    match (*parser).state as libc::c_uint {
        0 => return yaml_parser_parse_stream_start(parser, event),
        1 => return yaml_parser_parse_document_start(parser, event, 1_i32),
        2 => return yaml_parser_parse_document_start(parser, event, 0_i32),
        3 => return yaml_parser_parse_document_content(parser, event),
        4 => return yaml_parser_parse_document_end(parser, event),
        5 => {
            return yaml_parser_parse_node(parser, event, 1_i32, 0_i32);
        }
        6 => {
            return yaml_parser_parse_node(parser, event, 1_i32, 1_i32);
        }
        7 => {
            return yaml_parser_parse_node(parser, event, 0_i32, 0_i32);
        }
        8 => {
            return yaml_parser_parse_block_sequence_entry(parser, event, 1_i32);
        }
        9 => {
            return yaml_parser_parse_block_sequence_entry(parser, event, 0_i32);
        }
        10 => return yaml_parser_parse_indentless_sequence_entry(parser, event),
        11 => return yaml_parser_parse_block_mapping_key(parser, event, 1_i32),
        12 => return yaml_parser_parse_block_mapping_key(parser, event, 0_i32),
        13 => return yaml_parser_parse_block_mapping_value(parser, event),
        14 => {
            return yaml_parser_parse_flow_sequence_entry(parser, event, 1_i32);
        }
        15 => {
            return yaml_parser_parse_flow_sequence_entry(parser, event, 0_i32);
        }
        16 => return yaml_parser_parse_flow_sequence_entry_mapping_key(parser, event),
        17 => return yaml_parser_parse_flow_sequence_entry_mapping_value(parser, event),
        18 => return yaml_parser_parse_flow_sequence_entry_mapping_end(parser, event),
        19 => return yaml_parser_parse_flow_mapping_key(parser, event, 1_i32),
        20 => return yaml_parser_parse_flow_mapping_key(parser, event, 0_i32),
        21 => {
            return yaml_parser_parse_flow_mapping_value(parser, event, 0_i32);
        }
        22 => {
            return yaml_parser_parse_flow_mapping_value(parser, event, 1_i32);
        }
        _ => {}
    }
    FAIL
}

unsafe fn yaml_parser_parse_stream_start(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> Success {
    let token: *mut yaml_token_t = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint != YAML_STREAM_START_TOKEN as libc::c_int as libc::c_uint {
        return yaml_parser_set_parser_error(
            parser,
            b"did not find expected <stream-start>\0" as *const u8 as *const libc::c_char,
            (*token).start_mark,
        );
    }
    (*parser).state = YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE;
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_STREAM_START_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).start_mark;
    (*event).data.stream_start.encoding = (*token).data.stream_start.encoding;
    SKIP_TOKEN(parser);
    OK
}

unsafe fn yaml_parser_parse_document_start(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    implicit: libc::c_int,
) -> Success {
    let mut token: *mut yaml_token_t;
    let mut version_directive: *mut yaml_version_directive_t =
        ptr::null_mut::<yaml_version_directive_t>();
    struct TagDirectives {
        start: *mut yaml_tag_directive_t,
        end: *mut yaml_tag_directive_t,
    }
    let mut tag_directives = TagDirectives {
        start: ptr::null_mut::<yaml_tag_directive_t>(),
        end: ptr::null_mut::<yaml_tag_directive_t>(),
    };
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if implicit == 0 {
        while (*token).type_ as libc::c_uint
            == YAML_DOCUMENT_END_TOKEN as libc::c_int as libc::c_uint
        {
            SKIP_TOKEN(parser);
            token = PEEK_TOKEN(parser);
            if token.is_null() {
                return FAIL;
            }
        }
    }
    if implicit != 0
        && (*token).type_ as libc::c_uint
            != YAML_VERSION_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_ as libc::c_uint != YAML_TAG_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_ as libc::c_uint
            != YAML_DOCUMENT_START_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_ as libc::c_uint != YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint
    {
        if yaml_parser_process_directives(
            parser,
            ptr::null_mut::<*mut yaml_version_directive_t>(),
            ptr::null_mut::<*mut yaml_tag_directive_t>(),
            ptr::null_mut::<*mut yaml_tag_directive_t>(),
        ) == 0
        {
            return FAIL;
        }
        if PUSH!(parser, (*parser).states, YAML_PARSE_DOCUMENT_END_STATE) == 0 {
            return FAIL;
        }
        (*parser).state = YAML_PARSE_BLOCK_NODE_STATE;
        memset(
            event as *mut libc::c_void,
            0_i32,
            size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_ = YAML_DOCUMENT_START_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).start_mark;
        let fresh9 = addr_of_mut!((*event).data.document_start.version_directive);
        *fresh9 = ptr::null_mut::<yaml_version_directive_t>();
        let fresh10 = addr_of_mut!((*event).data.document_start.tag_directives.start);
        *fresh10 = ptr::null_mut::<yaml_tag_directive_t>();
        let fresh11 = addr_of_mut!((*event).data.document_start.tag_directives.end);
        *fresh11 = ptr::null_mut::<yaml_tag_directive_t>();
        (*event).data.document_start.implicit = 1_i32;
        OK
    } else if (*token).type_ as libc::c_uint != YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint
    {
        let end_mark: yaml_mark_t;
        let start_mark: yaml_mark_t = (*token).start_mark;
        if yaml_parser_process_directives(
            parser,
            addr_of_mut!(version_directive),
            addr_of_mut!(tag_directives.start),
            addr_of_mut!(tag_directives.end),
        ) == 0
        {
            return FAIL;
        }
        token = PEEK_TOKEN(parser);
        if !token.is_null() {
            if (*token).type_ as libc::c_uint
                != YAML_DOCUMENT_START_TOKEN as libc::c_int as libc::c_uint
            {
                let _ = yaml_parser_set_parser_error(
                    parser,
                    b"did not find expected <document start>\0" as *const u8 as *const libc::c_char,
                    (*token).start_mark,
                );
            } else if !(PUSH!(parser, (*parser).states, YAML_PARSE_DOCUMENT_END_STATE) == 0) {
                (*parser).state = YAML_PARSE_DOCUMENT_CONTENT_STATE;
                end_mark = (*token).end_mark;
                memset(
                    event as *mut libc::c_void,
                    0_i32,
                    size_of::<yaml_event_t>() as libc::c_ulong,
                );
                (*event).type_ = YAML_DOCUMENT_START_EVENT;
                (*event).start_mark = start_mark;
                (*event).end_mark = end_mark;
                let fresh14 = addr_of_mut!((*event).data.document_start.version_directive);
                *fresh14 = version_directive;
                let fresh15 = addr_of_mut!((*event).data.document_start.tag_directives.start);
                *fresh15 = tag_directives.start;
                let fresh16 = addr_of_mut!((*event).data.document_start.tag_directives.end);
                *fresh16 = tag_directives.end;
                (*event).data.document_start.implicit = 0_i32;
                SKIP_TOKEN(parser);
                tag_directives.end = ptr::null_mut::<yaml_tag_directive_t>();
                tag_directives.start = tag_directives.end;
                return OK;
            }
        }
        yaml_free(version_directive as *mut libc::c_void);
        while tag_directives.start != tag_directives.end {
            yaml_free((*tag_directives.end.wrapping_offset(-1_isize)).handle as *mut libc::c_void);
            yaml_free((*tag_directives.end.wrapping_offset(-1_isize)).prefix as *mut libc::c_void);
            tag_directives.end = tag_directives.end.wrapping_offset(-1);
        }
        yaml_free(tag_directives.start as *mut libc::c_void);
        FAIL
    } else {
        (*parser).state = YAML_PARSE_END_STATE;
        memset(
            event as *mut libc::c_void,
            0_i32,
            size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_ = YAML_STREAM_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        SKIP_TOKEN(parser);
        OK
    }
}

unsafe fn yaml_parser_parse_document_content(
    mut parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> Success {
    let token: *mut yaml_token_t = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint == YAML_VERSION_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
        || (*token).type_ as libc::c_uint == YAML_TAG_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
        || (*token).type_ as libc::c_uint
            == YAML_DOCUMENT_START_TOKEN as libc::c_int as libc::c_uint
        || (*token).type_ as libc::c_uint == YAML_DOCUMENT_END_TOKEN as libc::c_int as libc::c_uint
        || (*token).type_ as libc::c_uint == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint
    {
        (*parser).state = POP!((*parser).states);
        yaml_parser_process_empty_scalar(parser, event, (*token).start_mark)
    } else {
        yaml_parser_parse_node(parser, event, 1_i32, 0_i32)
    }
}

unsafe fn yaml_parser_parse_document_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> Success {
    let mut end_mark: yaml_mark_t;
    let mut implicit: libc::c_int = 1_i32;
    let token: *mut yaml_token_t = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    end_mark = (*token).start_mark;
    let start_mark: yaml_mark_t = end_mark;
    if (*token).type_ as libc::c_uint == YAML_DOCUMENT_END_TOKEN as libc::c_int as libc::c_uint {
        end_mark = (*token).end_mark;
        SKIP_TOKEN(parser);
        implicit = 0_i32;
    }
    while !STACK_EMPTY!((*parser).tag_directives) {
        let tag_directive = POP!((*parser).tag_directives);
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    (*parser).state = YAML_PARSE_DOCUMENT_START_STATE;
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_DOCUMENT_END_EVENT;
    (*event).start_mark = start_mark;
    (*event).end_mark = end_mark;
    (*event).data.document_end.implicit = implicit;
    OK
}

unsafe fn yaml_parser_parse_node(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    block: libc::c_int,
    indentless_sequence: libc::c_int,
) -> Success {
    let mut current_block: u64;
    let mut token: *mut yaml_token_t;
    let mut anchor: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut tag_handle: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut tag_suffix: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut tag: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut tag_mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let implicit: libc::c_int;
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint == YAML_ALIAS_TOKEN as libc::c_int as libc::c_uint {
        (*parser).state = POP!((*parser).states);
        memset(
            event as *mut libc::c_void,
            0_i32,
            size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_ = YAML_ALIAS_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        let fresh26 = addr_of_mut!((*event).data.alias.anchor);
        *fresh26 = (*token).data.alias.value;
        SKIP_TOKEN(parser);
        OK
    } else {
        end_mark = (*token).start_mark;
        start_mark = end_mark;
        if (*token).type_ as libc::c_uint == YAML_ANCHOR_TOKEN as libc::c_int as libc::c_uint {
            anchor = (*token).data.anchor.value;
            start_mark = (*token).start_mark;
            end_mark = (*token).end_mark;
            SKIP_TOKEN(parser);
            token = PEEK_TOKEN(parser);
            if token.is_null() {
                current_block = 17786380918591080555;
            } else if (*token).type_ as libc::c_uint
                == YAML_TAG_TOKEN as libc::c_int as libc::c_uint
            {
                tag_handle = (*token).data.tag.handle;
                tag_suffix = (*token).data.tag.suffix;
                tag_mark = (*token).start_mark;
                end_mark = (*token).end_mark;
                SKIP_TOKEN(parser);
                token = PEEK_TOKEN(parser);
                if token.is_null() {
                    current_block = 17786380918591080555;
                } else {
                    current_block = 11743904203796629665;
                }
            } else {
                current_block = 11743904203796629665;
            }
        } else if (*token).type_ as libc::c_uint == YAML_TAG_TOKEN as libc::c_int as libc::c_uint {
            tag_handle = (*token).data.tag.handle;
            tag_suffix = (*token).data.tag.suffix;
            tag_mark = (*token).start_mark;
            start_mark = tag_mark;
            end_mark = (*token).end_mark;
            SKIP_TOKEN(parser);
            token = PEEK_TOKEN(parser);
            if token.is_null() {
                current_block = 17786380918591080555;
            } else if (*token).type_ as libc::c_uint
                == YAML_ANCHOR_TOKEN as libc::c_int as libc::c_uint
            {
                anchor = (*token).data.anchor.value;
                end_mark = (*token).end_mark;
                SKIP_TOKEN(parser);
                token = PEEK_TOKEN(parser);
                if token.is_null() {
                    current_block = 17786380918591080555;
                } else {
                    current_block = 11743904203796629665;
                }
            } else {
                current_block = 11743904203796629665;
            }
        } else {
            current_block = 11743904203796629665;
        }
        match current_block {
            11743904203796629665 => {
                if !tag_handle.is_null() {
                    if *tag_handle == 0 {
                        tag = tag_suffix;
                        yaml_free(tag_handle as *mut libc::c_void);
                        tag_suffix = ptr::null_mut::<yaml_char_t>();
                        tag_handle = tag_suffix;
                        current_block = 9437013279121998969;
                    } else {
                        let mut tag_directive: *mut yaml_tag_directive_t;
                        tag_directive = (*parser).tag_directives.start;
                        loop {
                            if !(tag_directive != (*parser).tag_directives.top) {
                                current_block = 17728966195399430138;
                                break;
                            }
                            if strcmp(
                                (*tag_directive).handle as *mut libc::c_char,
                                tag_handle as *mut libc::c_char,
                            ) == 0_i32
                            {
                                let prefix_len: size_t =
                                    strlen((*tag_directive).prefix as *mut libc::c_char);
                                let suffix_len: size_t = strlen(tag_suffix as *mut libc::c_char);
                                tag = yaml_malloc(
                                    prefix_len.wrapping_add(suffix_len).wrapping_add(1_u64),
                                ) as *mut yaml_char_t;
                                if tag.is_null() {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    current_block = 17786380918591080555;
                                    break;
                                } else {
                                    memcpy(
                                        tag as *mut libc::c_void,
                                        (*tag_directive).prefix as *const libc::c_void,
                                        prefix_len,
                                    );
                                    memcpy(
                                        tag.wrapping_offset(prefix_len as isize)
                                            as *mut libc::c_void,
                                        tag_suffix as *const libc::c_void,
                                        suffix_len,
                                    );
                                    *tag.wrapping_offset(
                                        prefix_len.wrapping_add(suffix_len) as isize
                                    ) = b'\0';
                                    yaml_free(tag_handle as *mut libc::c_void);
                                    yaml_free(tag_suffix as *mut libc::c_void);
                                    tag_suffix = ptr::null_mut::<yaml_char_t>();
                                    tag_handle = tag_suffix;
                                    current_block = 17728966195399430138;
                                    break;
                                }
                            } else {
                                tag_directive = tag_directive.wrapping_offset(1);
                            }
                        }
                        match current_block {
                            17786380918591080555 => {}
                            _ => {
                                if tag.is_null() {
                                    let _ = yaml_parser_set_parser_error_context(
                                        parser,
                                        b"while parsing a node\0" as *const u8
                                            as *const libc::c_char,
                                        start_mark,
                                        b"found undefined tag handle\0" as *const u8
                                            as *const libc::c_char,
                                        tag_mark,
                                    );
                                    current_block = 17786380918591080555;
                                } else {
                                    current_block = 9437013279121998969;
                                }
                            }
                        }
                    }
                } else {
                    current_block = 9437013279121998969;
                }
                match current_block {
                    17786380918591080555 => {}
                    _ => {
                        implicit = (tag.is_null() || *tag == 0) as libc::c_int;
                        if indentless_sequence != 0
                            && (*token).type_ as libc::c_uint
                                == YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_ = YAML_SEQUENCE_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let fresh37 = addr_of_mut!((*event).data.sequence_start.anchor);
                            *fresh37 = anchor;
                            let fresh38 = addr_of_mut!((*event).data.sequence_start.tag);
                            *fresh38 = tag;
                            (*event).data.sequence_start.implicit = implicit;
                            (*event).data.sequence_start.style = YAML_BLOCK_SEQUENCE_STYLE;
                            return OK;
                        } else if (*token).type_ as libc::c_uint
                            == YAML_SCALAR_TOKEN as libc::c_int as libc::c_uint
                        {
                            let mut plain_implicit: libc::c_int = 0_i32;
                            let mut quoted_implicit: libc::c_int = 0_i32;
                            end_mark = (*token).end_mark;
                            if (*token).data.scalar.style as libc::c_uint
                                == YAML_PLAIN_SCALAR_STYLE as libc::c_int as libc::c_uint
                                && tag.is_null()
                                || !tag.is_null()
                                    && strcmp(
                                        tag as *mut libc::c_char,
                                        b"!\0" as *const u8 as *const libc::c_char,
                                    ) == 0_i32
                            {
                                plain_implicit = 1_i32;
                            } else if tag.is_null() {
                                quoted_implicit = 1_i32;
                            }
                            (*parser).state = POP!((*parser).states);
                            memset(
                                event as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_ = YAML_SCALAR_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let fresh40 = addr_of_mut!((*event).data.scalar.anchor);
                            *fresh40 = anchor;
                            let fresh41 = addr_of_mut!((*event).data.scalar.tag);
                            *fresh41 = tag;
                            let fresh42 = addr_of_mut!((*event).data.scalar.value);
                            *fresh42 = (*token).data.scalar.value;
                            (*event).data.scalar.length = (*token).data.scalar.length;
                            (*event).data.scalar.plain_implicit = plain_implicit;
                            (*event).data.scalar.quoted_implicit = quoted_implicit;
                            (*event).data.scalar.style = (*token).data.scalar.style;
                            SKIP_TOKEN(parser);
                            return OK;
                        } else if (*token).type_ as libc::c_uint
                            == YAML_FLOW_SEQUENCE_START_TOKEN as libc::c_int as libc::c_uint
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_ = YAML_SEQUENCE_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let fresh45 = addr_of_mut!((*event).data.sequence_start.anchor);
                            *fresh45 = anchor;
                            let fresh46 = addr_of_mut!((*event).data.sequence_start.tag);
                            *fresh46 = tag;
                            (*event).data.sequence_start.implicit = implicit;
                            (*event).data.sequence_start.style = YAML_FLOW_SEQUENCE_STYLE;
                            return OK;
                        } else if (*token).type_ as libc::c_uint
                            == YAML_FLOW_MAPPING_START_TOKEN as libc::c_int as libc::c_uint
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_ = YAML_MAPPING_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let fresh47 = addr_of_mut!((*event).data.mapping_start.anchor);
                            *fresh47 = anchor;
                            let fresh48 = addr_of_mut!((*event).data.mapping_start.tag);
                            *fresh48 = tag;
                            (*event).data.mapping_start.implicit = implicit;
                            (*event).data.mapping_start.style = YAML_FLOW_MAPPING_STYLE;
                            return OK;
                        } else if block != 0
                            && (*token).type_ as libc::c_uint
                                == YAML_BLOCK_SEQUENCE_START_TOKEN as libc::c_int as libc::c_uint
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_ = YAML_SEQUENCE_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let fresh49 = addr_of_mut!((*event).data.sequence_start.anchor);
                            *fresh49 = anchor;
                            let fresh50 = addr_of_mut!((*event).data.sequence_start.tag);
                            *fresh50 = tag;
                            (*event).data.sequence_start.implicit = implicit;
                            (*event).data.sequence_start.style = YAML_BLOCK_SEQUENCE_STYLE;
                            return OK;
                        } else if block != 0
                            && (*token).type_ as libc::c_uint
                                == YAML_BLOCK_MAPPING_START_TOKEN as libc::c_int as libc::c_uint
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_ = YAML_MAPPING_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            let fresh51 = addr_of_mut!((*event).data.mapping_start.anchor);
                            *fresh51 = anchor;
                            let fresh52 = addr_of_mut!((*event).data.mapping_start.tag);
                            *fresh52 = tag;
                            (*event).data.mapping_start.implicit = implicit;
                            (*event).data.mapping_start.style = YAML_BLOCK_MAPPING_STYLE;
                            return OK;
                        } else if !anchor.is_null() || !tag.is_null() {
                            let value: *mut yaml_char_t = yaml_malloc(1_u64) as *mut yaml_char_t;
                            if value.is_null() {
                                (*parser).error = YAML_MEMORY_ERROR;
                            } else {
                                *value = b'\0';
                                (*parser).state = POP!((*parser).states);
                                memset(
                                    event as *mut libc::c_void,
                                    0_i32,
                                    size_of::<yaml_event_t>() as libc::c_ulong,
                                );
                                (*event).type_ = YAML_SCALAR_EVENT;
                                (*event).start_mark = start_mark;
                                (*event).end_mark = end_mark;
                                let fresh54 = addr_of_mut!((*event).data.scalar.anchor);
                                *fresh54 = anchor;
                                let fresh55 = addr_of_mut!((*event).data.scalar.tag);
                                *fresh55 = tag;
                                let fresh56 = addr_of_mut!((*event).data.scalar.value);
                                *fresh56 = value;
                                (*event).data.scalar.length = 0_u64;
                                (*event).data.scalar.plain_implicit = implicit;
                                (*event).data.scalar.quoted_implicit = 0_i32;
                                (*event).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
                                return OK;
                            }
                        } else {
                            let _ = yaml_parser_set_parser_error_context(
                                parser,
                                if block != 0 {
                                    b"while parsing a block node\0" as *const u8
                                        as *const libc::c_char
                                } else {
                                    b"while parsing a flow node\0" as *const u8
                                        as *const libc::c_char
                                },
                                start_mark,
                                b"did not find expected node content\0" as *const u8
                                    as *const libc::c_char,
                                (*token).start_mark,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        yaml_free(anchor as *mut libc::c_void);
        yaml_free(tag_handle as *mut libc::c_void);
        yaml_free(tag_suffix as *mut libc::c_void);
        yaml_free(tag as *mut libc::c_void);
        FAIL
    }
}

unsafe fn yaml_parser_parse_block_sequence_entry(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    first: libc::c_int,
) -> Success {
    let mut token: *mut yaml_token_t;
    if first != 0 {
        token = PEEK_TOKEN(parser);
        if PUSH!(parser, (*parser).marks, (*token).start_mark) == 0 {
            return FAIL;
        }
        SKIP_TOKEN(parser);
    }
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint == YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return FAIL;
        }
        if (*token).type_ as libc::c_uint != YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint != YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE
            ) == 0
            {
                return FAIL;
            }
            yaml_parser_parse_node(parser, event, 1_i32, 0_i32)
        } else {
            (*parser).state = YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE;
            yaml_parser_process_empty_scalar(parser, event, mark)
        }
    } else if (*token).type_ as libc::c_uint == YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
    {
        (*parser).state = POP!((*parser).states);
        let _ = POP!((*parser).marks);
        memset(
            event as *mut libc::c_void,
            0_i32,
            size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_ = YAML_SEQUENCE_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        SKIP_TOKEN(parser);
        OK
    } else {
        yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block collection\0" as *const u8 as *const libc::c_char,
            POP!((*parser).marks),
            b"did not find expected '-' indicator\0" as *const u8 as *const libc::c_char,
            (*token).start_mark,
        )
    }
}

unsafe fn yaml_parser_parse_indentless_sequence_entry(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> Success {
    let mut token: *mut yaml_token_t;
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint == YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return FAIL;
        }
        if (*token).type_ as libc::c_uint != YAML_BLOCK_ENTRY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint != YAML_KEY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint != YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE
            ) == 0
            {
                return FAIL;
            }
            yaml_parser_parse_node(parser, event, 1_i32, 0_i32)
        } else {
            (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
            yaml_parser_process_empty_scalar(parser, event, mark)
        }
    } else {
        (*parser).state = POP!((*parser).states);
        memset(
            event as *mut libc::c_void,
            0_i32,
            size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_ = YAML_SEQUENCE_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).start_mark;
        OK
    }
}

unsafe fn yaml_parser_parse_block_mapping_key(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    first: libc::c_int,
) -> Success {
    let mut token: *mut yaml_token_t;
    if first != 0 {
        token = PEEK_TOKEN(parser);
        if PUSH!(parser, (*parser).marks, (*token).start_mark) == 0 {
            return FAIL;
        }
        SKIP_TOKEN(parser);
    }
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint == YAML_KEY_TOKEN as libc::c_int as libc::c_uint {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return FAIL;
        }
        if (*token).type_ as libc::c_uint != YAML_KEY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint != YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_BLOCK_MAPPING_VALUE_STATE
            ) == 0
            {
                return FAIL;
            }
            yaml_parser_parse_node(parser, event, 1_i32, 1_i32)
        } else {
            (*parser).state = YAML_PARSE_BLOCK_MAPPING_VALUE_STATE;
            yaml_parser_process_empty_scalar(parser, event, mark)
        }
    } else if (*token).type_ as libc::c_uint == YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
    {
        (*parser).state = POP!((*parser).states);
        let _ = POP!((*parser).marks);
        memset(
            event as *mut libc::c_void,
            0_i32,
            size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_ = YAML_MAPPING_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        SKIP_TOKEN(parser);
        OK
    } else {
        yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block mapping\0" as *const u8 as *const libc::c_char,
            POP!((*parser).marks),
            b"did not find expected key\0" as *const u8 as *const libc::c_char,
            (*token).start_mark,
        )
    }
}

unsafe fn yaml_parser_parse_block_mapping_value(
    mut parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> Success {
    let mut token: *mut yaml_token_t;
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint == YAML_VALUE_TOKEN as libc::c_int as libc::c_uint {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return FAIL;
        }
        if (*token).type_ as libc::c_uint != YAML_KEY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint != YAML_BLOCK_END_TOKEN as libc::c_int as libc::c_uint
        {
            if PUSH!(parser, (*parser).states, YAML_PARSE_BLOCK_MAPPING_KEY_STATE) == 0 {
                return FAIL;
            }
            yaml_parser_parse_node(parser, event, 1_i32, 1_i32)
        } else {
            (*parser).state = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
            yaml_parser_process_empty_scalar(parser, event, mark)
        }
    } else {
        (*parser).state = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
        yaml_parser_process_empty_scalar(parser, event, (*token).start_mark)
    }
}

unsafe fn yaml_parser_parse_flow_sequence_entry(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    first: libc::c_int,
) -> Success {
    let mut token: *mut yaml_token_t;
    if first != 0 {
        token = PEEK_TOKEN(parser);
        if PUSH!(parser, (*parser).marks, (*token).start_mark) == 0 {
            return FAIL;
        }
        SKIP_TOKEN(parser);
    }
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint != YAML_FLOW_SEQUENCE_END_TOKEN as libc::c_int as libc::c_uint
    {
        if first == 0 {
            if (*token).type_ as libc::c_uint
                == YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
            {
                SKIP_TOKEN(parser);
                token = PEEK_TOKEN(parser);
                if token.is_null() {
                    return FAIL;
                }
            } else {
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow sequence\0" as *const u8 as *const libc::c_char,
                    POP!((*parser).marks),
                    b"did not find expected ',' or ']'\0" as *const u8 as *const libc::c_char,
                    (*token).start_mark,
                );
            }
        }
        if (*token).type_ as libc::c_uint == YAML_KEY_TOKEN as libc::c_int as libc::c_uint {
            (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE;
            memset(
                event as *mut libc::c_void,
                0_i32,
                size_of::<yaml_event_t>() as libc::c_ulong,
            );
            (*event).type_ = YAML_MAPPING_START_EVENT;
            (*event).start_mark = (*token).start_mark;
            (*event).end_mark = (*token).end_mark;
            let fresh99 = addr_of_mut!((*event).data.mapping_start.anchor);
            *fresh99 = ptr::null_mut::<yaml_char_t>();
            let fresh100 = addr_of_mut!((*event).data.mapping_start.tag);
            *fresh100 = ptr::null_mut::<yaml_char_t>();
            (*event).data.mapping_start.implicit = 1_i32;
            (*event).data.mapping_start.style = YAML_FLOW_MAPPING_STYLE;
            SKIP_TOKEN(parser);
            return OK;
        } else if (*token).type_ as libc::c_uint
            != YAML_FLOW_SEQUENCE_END_TOKEN as libc::c_int as libc::c_uint
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE
            ) == 0
            {
                return FAIL;
            }
            return yaml_parser_parse_node(parser, event, 0_i32, 0_i32);
        }
    }
    (*parser).state = POP!((*parser).states);
    let _ = POP!((*parser).marks);
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_SEQUENCE_END_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).end_mark;
    SKIP_TOKEN(parser);
    OK
}

unsafe fn yaml_parser_parse_flow_sequence_entry_mapping_key(
    mut parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> Success {
    let token: *mut yaml_token_t = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_ as libc::c_uint != YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
        && (*token).type_ as libc::c_uint
            != YAML_FLOW_SEQUENCE_END_TOKEN as libc::c_int as libc::c_uint
    {
        if PUSH!(
            parser,
            (*parser).states,
            YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE
        ) == 0
        {
            return FAIL;
        }
        yaml_parser_parse_node(parser, event, 0_i32, 0_i32)
    } else {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE;
        yaml_parser_process_empty_scalar(parser, event, mark)
    }
}

unsafe fn yaml_parser_parse_flow_sequence_entry_mapping_value(
    mut parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> Success {
    let mut token: *mut yaml_token_t;
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint == YAML_VALUE_TOKEN as libc::c_int as libc::c_uint {
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return FAIL;
        }
        if (*token).type_ as libc::c_uint != YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint
                != YAML_FLOW_SEQUENCE_END_TOKEN as libc::c_int as libc::c_uint
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE
            ) == 0
            {
                return FAIL;
            }
            return yaml_parser_parse_node(parser, event, 0_i32, 0_i32);
        }
    }
    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE;
    yaml_parser_process_empty_scalar(parser, event, (*token).start_mark)
}

unsafe fn yaml_parser_parse_flow_sequence_entry_mapping_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> Success {
    let token: *mut yaml_token_t = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE;
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_MAPPING_END_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).start_mark;
    OK
}

unsafe fn yaml_parser_parse_flow_mapping_key(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    first: libc::c_int,
) -> Success {
    let mut token: *mut yaml_token_t;
    if first != 0 {
        token = PEEK_TOKEN(parser);
        if PUSH!(parser, (*parser).marks, (*token).start_mark) == 0 {
            return FAIL;
        }
        SKIP_TOKEN(parser);
    }
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if (*token).type_ as libc::c_uint != YAML_FLOW_MAPPING_END_TOKEN as libc::c_int as libc::c_uint
    {
        if first == 0 {
            if (*token).type_ as libc::c_uint
                == YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
            {
                SKIP_TOKEN(parser);
                token = PEEK_TOKEN(parser);
                if token.is_null() {
                    return FAIL;
                }
            } else {
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow mapping\0" as *const u8 as *const libc::c_char,
                    POP!((*parser).marks),
                    b"did not find expected ',' or '}'\0" as *const u8 as *const libc::c_char,
                    (*token).start_mark,
                );
            }
        }
        if (*token).type_ as libc::c_uint == YAML_KEY_TOKEN as libc::c_int as libc::c_uint {
            SKIP_TOKEN(parser);
            token = PEEK_TOKEN(parser);
            if token.is_null() {
                return FAIL;
            }
            if (*token).type_ as libc::c_uint != YAML_VALUE_TOKEN as libc::c_int as libc::c_uint
                && (*token).type_ as libc::c_uint
                    != YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
                && (*token).type_ as libc::c_uint
                    != YAML_FLOW_MAPPING_END_TOKEN as libc::c_int as libc::c_uint
            {
                if PUSH!(
                    parser,
                    (*parser).states,
                    YAML_PARSE_FLOW_MAPPING_VALUE_STATE
                ) == 0
                {
                    return FAIL;
                }
                return yaml_parser_parse_node(parser, event, 0_i32, 0_i32);
            } else {
                (*parser).state = YAML_PARSE_FLOW_MAPPING_VALUE_STATE;
                return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
            }
        } else if (*token).type_ as libc::c_uint
            != YAML_FLOW_MAPPING_END_TOKEN as libc::c_int as libc::c_uint
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE
            ) == 0
            {
                return FAIL;
            }
            return yaml_parser_parse_node(parser, event, 0_i32, 0_i32);
        }
    }
    (*parser).state = POP!((*parser).states);
    let _ = POP!((*parser).marks);
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_MAPPING_END_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).end_mark;
    SKIP_TOKEN(parser);
    OK
}

unsafe fn yaml_parser_parse_flow_mapping_value(
    mut parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    empty: libc::c_int,
) -> Success {
    let mut token: *mut yaml_token_t;
    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return FAIL;
    }
    if empty != 0 {
        (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    }
    if (*token).type_ as libc::c_uint == YAML_VALUE_TOKEN as libc::c_int as libc::c_uint {
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return FAIL;
        }
        if (*token).type_ as libc::c_uint != YAML_FLOW_ENTRY_TOKEN as libc::c_int as libc::c_uint
            && (*token).type_ as libc::c_uint
                != YAML_FLOW_MAPPING_END_TOKEN as libc::c_int as libc::c_uint
        {
            if PUSH!(parser, (*parser).states, YAML_PARSE_FLOW_MAPPING_KEY_STATE) == 0 {
                return FAIL;
            }
            return yaml_parser_parse_node(parser, event, 0_i32, 0_i32);
        }
    }
    (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
    yaml_parser_process_empty_scalar(parser, event, (*token).start_mark)
}

unsafe fn yaml_parser_process_empty_scalar(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mark: yaml_mark_t,
) -> Success {
    let value: *mut yaml_char_t = yaml_malloc(1_u64) as *mut yaml_char_t;
    if value.is_null() {
        (*parser).error = YAML_MEMORY_ERROR;
        return FAIL;
    }
    *value = b'\0';
    memset(
        event as *mut libc::c_void,
        0_i32,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_SCALAR_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    let fresh138 = addr_of_mut!((*event).data.scalar.anchor);
    *fresh138 = ptr::null_mut::<yaml_char_t>();
    let fresh139 = addr_of_mut!((*event).data.scalar.tag);
    *fresh139 = ptr::null_mut::<yaml_char_t>();
    let fresh140 = addr_of_mut!((*event).data.scalar.value);
    *fresh140 = value;
    (*event).data.scalar.length = 0_u64;
    (*event).data.scalar.plain_implicit = 1_i32;
    (*event).data.scalar.quoted_implicit = 0_i32;
    (*event).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
    OK
}

unsafe fn yaml_parser_process_directives(
    mut parser: *mut yaml_parser_t,
    version_directive_ref: *mut *mut yaml_version_directive_t,
    tag_directives_start_ref: *mut *mut yaml_tag_directive_t,
    tag_directives_end_ref: *mut *mut yaml_tag_directive_t,
) -> Success {
    let mut current_block: u64;
    let mut default_tag_directives: [yaml_tag_directive_t; 3] = [
        yaml_tag_directive_t {
            handle: b"!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
            prefix: b"!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
        },
        yaml_tag_directive_t {
            handle: b"!!\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
            prefix: b"tag:yaml.org,2002:\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
        },
        yaml_tag_directive_t {
            handle: ptr::null_mut::<yaml_char_t>(),
            prefix: ptr::null_mut::<yaml_char_t>(),
        },
    ];
    let mut default_tag_directive: *mut yaml_tag_directive_t;
    let mut version_directive: *mut yaml_version_directive_t =
        ptr::null_mut::<yaml_version_directive_t>();
    struct TagDirectives {
        start: *mut yaml_tag_directive_t,
        end: *mut yaml_tag_directive_t,
        top: *mut yaml_tag_directive_t,
    }
    let mut tag_directives = TagDirectives {
        start: ptr::null_mut::<yaml_tag_directive_t>(),
        end: ptr::null_mut::<yaml_tag_directive_t>(),
        top: ptr::null_mut::<yaml_tag_directive_t>(),
    };
    let mut token: *mut yaml_token_t;
    if !(STACK_INIT!(parser, tag_directives, yaml_tag_directive_t) == 0) {
        token = PEEK_TOKEN(parser);
        if !token.is_null() {
            loop {
                if !((*token).type_ as libc::c_uint
                    == YAML_VERSION_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
                    || (*token).type_ as libc::c_uint
                        == YAML_TAG_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint)
                {
                    current_block = 16924917904204750491;
                    break;
                }
                if (*token).type_ as libc::c_uint
                    == YAML_VERSION_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
                {
                    if !version_directive.is_null() {
                        let _ = yaml_parser_set_parser_error(
                            parser,
                            b"found duplicate %YAML directive\0" as *const u8
                                as *const libc::c_char,
                            (*token).start_mark,
                        );
                        current_block = 17143798186130252483;
                        break;
                    } else if (*token).data.version_directive.major != 1_i32
                        || (*token).data.version_directive.minor != 1_i32
                            && (*token).data.version_directive.minor != 2_i32
                    {
                        let _ = yaml_parser_set_parser_error(
                            parser,
                            b"found incompatible YAML document\0" as *const u8
                                as *const libc::c_char,
                            (*token).start_mark,
                        );
                        current_block = 17143798186130252483;
                        break;
                    } else {
                        version_directive =
                            yaml_malloc(size_of::<yaml_version_directive_t>() as libc::c_ulong)
                                as *mut yaml_version_directive_t;
                        if version_directive.is_null() {
                            (*parser).error = YAML_MEMORY_ERROR;
                            current_block = 17143798186130252483;
                            break;
                        } else {
                            (*version_directive).major = (*token).data.version_directive.major;
                            (*version_directive).minor = (*token).data.version_directive.minor;
                        }
                    }
                } else if (*token).type_ as libc::c_uint
                    == YAML_TAG_DIRECTIVE_TOKEN as libc::c_int as libc::c_uint
                {
                    let value = yaml_tag_directive_t {
                        handle: (*token).data.tag_directive.handle,
                        prefix: (*token).data.tag_directive.prefix,
                    };
                    if yaml_parser_append_tag_directive(parser, value, 0_i32, (*token).start_mark)
                        == 0
                    {
                        current_block = 17143798186130252483;
                        break;
                    }
                    if PUSH!(parser, tag_directives, value) == 0 {
                        current_block = 17143798186130252483;
                        break;
                    }
                }
                SKIP_TOKEN(parser);
                token = PEEK_TOKEN(parser);
                if token.is_null() {
                    current_block = 17143798186130252483;
                    break;
                }
            }
            match current_block {
                17143798186130252483 => {}
                _ => {
                    default_tag_directive = default_tag_directives.as_mut_ptr();
                    loop {
                        if ((*default_tag_directive).handle).is_null() {
                            current_block = 18377268871191777778;
                            break;
                        }
                        if yaml_parser_append_tag_directive(
                            parser,
                            *default_tag_directive,
                            1_i32,
                            (*token).start_mark,
                        ) == 0
                        {
                            current_block = 17143798186130252483;
                            break;
                        }
                        default_tag_directive = default_tag_directive.wrapping_offset(1);
                    }
                    match current_block {
                        17143798186130252483 => {}
                        _ => {
                            if !version_directive_ref.is_null() {
                                *version_directive_ref = version_directive;
                            }
                            if !tag_directives_start_ref.is_null() {
                                if STACK_EMPTY!(tag_directives) {
                                    *tag_directives_end_ref =
                                        ptr::null_mut::<yaml_tag_directive_t>();
                                    *tag_directives_start_ref = *tag_directives_end_ref;
                                    STACK_DEL!(tag_directives);
                                } else {
                                    *tag_directives_start_ref = tag_directives.start;
                                    *tag_directives_end_ref = tag_directives.top;
                                }
                            } else {
                                STACK_DEL!(tag_directives);
                            }
                            if version_directive_ref.is_null() {
                                yaml_free(version_directive as *mut libc::c_void);
                            }
                            return OK;
                        }
                    }
                }
            }
        }
    }
    yaml_free(version_directive as *mut libc::c_void);
    while !STACK_EMPTY!(tag_directives) {
        let tag_directive = POP!(tag_directives);
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    STACK_DEL!(tag_directives);
    FAIL
}

unsafe fn yaml_parser_append_tag_directive(
    mut parser: *mut yaml_parser_t,
    value: yaml_tag_directive_t,
    allow_duplicates: libc::c_int,
    mark: yaml_mark_t,
) -> Success {
    let mut tag_directive: *mut yaml_tag_directive_t;
    let mut copy = yaml_tag_directive_t {
        handle: ptr::null_mut::<yaml_char_t>(),
        prefix: ptr::null_mut::<yaml_char_t>(),
    };
    tag_directive = (*parser).tag_directives.start;
    while tag_directive != (*parser).tag_directives.top {
        if strcmp(
            value.handle as *mut libc::c_char,
            (*tag_directive).handle as *mut libc::c_char,
        ) == 0_i32
        {
            if allow_duplicates != 0 {
                return OK;
            }
            return yaml_parser_set_parser_error(
                parser,
                b"found duplicate %TAG directive\0" as *const u8 as *const libc::c_char,
                mark,
            );
        }
        tag_directive = tag_directive.wrapping_offset(1);
    }
    copy.handle = yaml_strdup(value.handle);
    copy.prefix = yaml_strdup(value.prefix);
    if copy.handle.is_null() || copy.prefix.is_null() {
        (*parser).error = YAML_MEMORY_ERROR;
    } else if !(PUSH!(parser, (*parser).tag_directives, copy) == 0) {
        return OK;
    }
    yaml_free(copy.handle as *mut libc::c_void);
    yaml_free(copy.prefix as *mut libc::c_void);
    FAIL
}
