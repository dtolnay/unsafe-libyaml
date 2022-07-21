use crate::api::{
    yaml_free, yaml_malloc, yaml_queue_extend, yaml_stack_extend, yaml_string_extend,
    yaml_string_join,
};
use crate::externs::{memcpy, memmove, memset, strcmp, strlen};
use crate::reader::yaml_parser_update_buffer;
use crate::yaml::{ptrdiff_t, size_t, yaml_char_t, yaml_string_t};
use crate::{
    libc, yaml_mark_t, yaml_parser_t, yaml_simple_key_t, yaml_token_delete, yaml_token_t,
    yaml_token_type_t, PointerExt, YAML_ALIAS_TOKEN, YAML_ANCHOR_TOKEN, YAML_BLOCK_END_TOKEN,
    YAML_BLOCK_ENTRY_TOKEN, YAML_BLOCK_MAPPING_START_TOKEN, YAML_BLOCK_SEQUENCE_START_TOKEN,
    YAML_DOCUMENT_END_TOKEN, YAML_DOCUMENT_START_TOKEN, YAML_DOUBLE_QUOTED_SCALAR_STYLE,
    YAML_FLOW_ENTRY_TOKEN, YAML_FLOW_MAPPING_END_TOKEN, YAML_FLOW_MAPPING_START_TOKEN,
    YAML_FLOW_SEQUENCE_END_TOKEN, YAML_FLOW_SEQUENCE_START_TOKEN, YAML_FOLDED_SCALAR_STYLE,
    YAML_KEY_TOKEN, YAML_LITERAL_SCALAR_STYLE, YAML_MEMORY_ERROR, YAML_PLAIN_SCALAR_STYLE,
    YAML_SCALAR_TOKEN, YAML_SCANNER_ERROR, YAML_SINGLE_QUOTED_SCALAR_STYLE, YAML_STREAM_END_TOKEN,
    YAML_STREAM_START_TOKEN, YAML_TAG_DIRECTIVE_TOKEN, YAML_TAG_TOKEN, YAML_VALUE_TOKEN,
    YAML_VERSION_DIRECTIVE_TOKEN,
};
use core::mem::{size_of, MaybeUninit};
use core::ptr::{self, addr_of_mut};

macro_rules! SKIP {
    ($parser:expr) => {
        let index = addr_of_mut!((*$parser).mark.index);
        *index = (*index).wrapping_add(1);
        let column = addr_of_mut!((*$parser).mark.column);
        *column = (*column).wrapping_add(1);
        let unread = addr_of_mut!((*$parser).unread);
        *unread = (*unread).wrapping_sub(1);
        let pointer = addr_of_mut!((*$parser).buffer.pointer);
        *pointer = (*pointer).wrapping_offset(
            (if *((*$parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                == 0_i32
            {
                1_i32
            } else if *((*$parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                & 0xe0_i32
                == 0xc0_i32
            {
                2_i32
            } else if *((*$parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                & 0xf0_i32
                == 0xe0_i32
            {
                3_i32
            } else if *((*$parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                & 0xf8_i32
                == 0xf0_i32
            {
                4_i32
            } else {
                0_i32
            }) as isize,
        );
    };
}

/// Scan the input stream and produce the next token.
///
/// Call the function subsequently to produce a sequence of tokens corresponding
/// to the input stream. The initial token has the type YAML_STREAM_START_TOKEN
/// while the ending token has the type YAML_STREAM_END_TOKEN.
///
/// An application is responsible for freeing any buffers associated with the
/// produced token object using the yaml_token_delete function.
///
/// An application must not alternate the calls of yaml_parser_scan() with the
/// calls of yaml_parser_parse() or yaml_parser_load(). Doing this will break
/// the parser.
///
/// Returns 1 if the function succeeded, 0 on error.
#[must_use]
pub unsafe fn yaml_parser_scan(
    mut parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> libc::c_int {
    __assert!(!parser.is_null());
    __assert!(!token.is_null());
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    if (*parser).stream_end_produced != 0 || (*parser).error as libc::c_uint != 0 {
        return 1_i32;
    }
    if (*parser).token_available == 0 {
        if yaml_parser_fetch_more_tokens(parser) == 0 {
            return 0_i32;
        }
    }
    let fresh0 = addr_of_mut!((*parser).tokens.head);
    let fresh1 = *fresh0;
    *fresh0 = (*fresh0).wrapping_offset(1);
    *token = *fresh1;
    (*parser).token_available = 0_i32;
    let fresh2 = addr_of_mut!((*parser).tokens_parsed);
    *fresh2 = (*fresh2).wrapping_add(1);
    if (*token).type_ as libc::c_uint == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint {
        (*parser).stream_end_produced = 1_i32;
    }
    1_i32
}

unsafe fn yaml_parser_set_scanner_error(
    mut parser: *mut yaml_parser_t,
    context: *const libc::c_char,
    context_mark: yaml_mark_t,
    problem: *const libc::c_char,
) -> libc::c_int {
    (*parser).error = YAML_SCANNER_ERROR;
    let fresh3 = addr_of_mut!((*parser).context);
    *fresh3 = context;
    (*parser).context_mark = context_mark;
    let fresh4 = addr_of_mut!((*parser).problem);
    *fresh4 = problem;
    (*parser).problem_mark = (*parser).mark;
    0_i32
}

pub(crate) unsafe fn yaml_parser_fetch_more_tokens(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut need_more_tokens: libc::c_int;
    loop {
        need_more_tokens = 0_i32;
        if (*parser).tokens.head == (*parser).tokens.tail {
            need_more_tokens = 1_i32;
        } else {
            let mut simple_key: *mut yaml_simple_key_t;
            if yaml_parser_stale_simple_keys(parser) == 0 {
                return 0_i32;
            }
            simple_key = (*parser).simple_keys.start;
            while simple_key != (*parser).simple_keys.top {
                if (*simple_key).possible != 0
                    && (*simple_key).token_number == (*parser).tokens_parsed
                {
                    need_more_tokens = 1_i32;
                    break;
                } else {
                    simple_key = simple_key.wrapping_offset(1);
                }
            }
        }
        if need_more_tokens == 0 {
            break;
        }
        if yaml_parser_fetch_next_token(parser) == 0 {
            return 0_i32;
        }
    }
    (*parser).token_available = 1_i32;
    1_i32
}

unsafe fn yaml_parser_fetch_next_token(parser: *mut yaml_parser_t) -> libc::c_int {
    if if (*parser).unread >= 1_u64 {
        1_i32
    } else {
        yaml_parser_update_buffer(parser, 1_u64)
    } == 0
    {
        return 0_i32;
    }
    if (*parser).stream_start_produced == 0 {
        return yaml_parser_fetch_stream_start(parser);
    }
    if yaml_parser_scan_to_next_token(parser) == 0 {
        return 0_i32;
    }
    if yaml_parser_stale_simple_keys(parser) == 0 {
        return 0_i32;
    }
    if yaml_parser_unroll_indent(parser, (*parser).mark.column as ptrdiff_t) == 0 {
        return 0_i32;
    }
    if if (*parser).unread >= 4_u64 {
        1_i32
    } else {
        yaml_parser_update_buffer(parser, 4_u64)
    } == 0
    {
        return 0_i32;
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '\0' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_stream_end(parser);
    }
    if (*parser).mark.column == 0_u64
        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '%' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_directive(parser);
    }
    if (*parser).mark.column == 0_u64
        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && (*((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
            || (*((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 1_i32) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 1_i32) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 2_i32) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 1_i32) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 2_i32) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int))
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_START_TOKEN);
    }
    if (*parser).mark.column == 0_u64
        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
        && (*((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
            || (*((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 1_i32) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 1_i32) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 2_i32) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 1_i32) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((3_i32 + 2_i32) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int))
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_END_TOKEN);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '[' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_SEQUENCE_START_TOKEN);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '{' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_MAPPING_START_TOKEN);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == ']' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_SEQUENCE_END_TOKEN);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '}' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_MAPPING_END_TOKEN);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == ',' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_entry(parser);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '-' as i32 as yaml_char_t as libc::c_int
        && (*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
            || (*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 2_i32) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 2_i32) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int))
    {
        return yaml_parser_fetch_block_entry(parser);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '?' as i32 as yaml_char_t as libc::c_int
        && ((*parser).flow_level != 0
            || (*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
                || (*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                            as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 2_i32) as isize)
                            as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 2_i32) as isize)
                            as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == '\0' as i32 as yaml_char_t as libc::c_int)))
    {
        return yaml_parser_fetch_key(parser);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == ':' as i32 as yaml_char_t as libc::c_int
        && ((*parser).flow_level != 0
            || (*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
                || (*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                            as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 2_i32) as isize)
                            as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 2_i32) as isize)
                            as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == '\0' as i32 as yaml_char_t as libc::c_int)))
    {
        return yaml_parser_fetch_value(parser);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '*' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_anchor(parser, YAML_ALIAS_TOKEN);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '&' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_anchor(parser, YAML_ANCHOR_TOKEN);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '!' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_tag(parser);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '|' as i32 as yaml_char_t as libc::c_int
        && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 1_i32);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '>' as i32 as yaml_char_t as libc::c_int
        && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 0_i32);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '\'' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_scalar(parser, 1_i32);
    }
    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '"' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_scalar(parser, 0_i32);
    }
    if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == ' ' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '\t' as i32 as yaml_char_t as libc::c_int
        || (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '\0' as i32 as yaml_char_t as libc::c_int)
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '?' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == ':' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == ',' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '[' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == ']' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '{' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '}' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '#' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '&' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '*' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '!' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '|' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '>' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '\'' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '"' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '%' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '@' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '`' as i32 as yaml_char_t as libc::c_int)
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
            && !(*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int)
        || (*parser).flow_level == 0
            && (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '?' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == ':' as i32 as yaml_char_t as libc::c_int)
            && !(*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
                || (*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                            as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 2_i32) as isize)
                            as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 1_i32) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset((1_i32 + 2_i32) as isize)
                            as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == '\0' as i32 as yaml_char_t as libc::c_int))
    {
        return yaml_parser_fetch_plain_scalar(parser);
    }
    yaml_parser_set_scanner_error(
        parser,
        b"while scanning for the next token\0" as *const u8 as *const libc::c_char,
        (*parser).mark,
        b"found character that cannot start any token\0" as *const u8 as *const libc::c_char,
    )
}

unsafe fn yaml_parser_stale_simple_keys(parser: *mut yaml_parser_t) -> libc::c_int {
    let mut simple_key: *mut yaml_simple_key_t;
    simple_key = (*parser).simple_keys.start;
    while simple_key != (*parser).simple_keys.top {
        if (*simple_key).possible != 0
            && ((*simple_key).mark.line < (*parser).mark.line
                || ((*simple_key).mark.index).wrapping_add(1024_u64) < (*parser).mark.index)
        {
            if (*simple_key).required != 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a simple key\0" as *const u8 as *const libc::c_char,
                    (*simple_key).mark,
                    b"could not find expected ':'\0" as *const u8 as *const libc::c_char,
                );
            }
            (*simple_key).possible = 0_i32;
        }
        simple_key = simple_key.wrapping_offset(1);
    }
    1_i32
}

unsafe fn yaml_parser_save_simple_key(parser: *mut yaml_parser_t) -> libc::c_int {
    let required: libc::c_int = ((*parser).flow_level == 0
        && (*parser).indent as libc::c_long == (*parser).mark.column as ptrdiff_t)
        as libc::c_int;
    if (*parser).simple_key_allowed != 0 {
        let simple_key = yaml_simple_key_t {
            possible: 1_i32,
            required,
            token_number: ((*parser).tokens_parsed)
                .wrapping_add(((*parser).tokens.tail).c_offset_from((*parser).tokens.head)
                    as libc::c_long as libc::c_ulong),
            mark: (*parser).mark,
        };
        if yaml_parser_remove_simple_key(parser) == 0 {
            return 0_i32;
        }
        *((*parser).simple_keys.top).wrapping_offset(-(1_isize)) = simple_key;
    }
    1_i32
}

unsafe fn yaml_parser_remove_simple_key(parser: *mut yaml_parser_t) -> libc::c_int {
    let mut simple_key: *mut yaml_simple_key_t =
        ((*parser).simple_keys.top).wrapping_offset(-(1_isize));
    if (*simple_key).possible != 0 {
        if (*simple_key).required != 0 {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a simple key\0" as *const u8 as *const libc::c_char,
                (*simple_key).mark,
                b"could not find expected ':'\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*simple_key).possible = 0_i32;
    1_i32
}

unsafe fn yaml_parser_increase_flow_level(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let empty_simple_key = yaml_simple_key_t {
        possible: 0_i32,
        required: 0_i32,
        token_number: 0_u64,
        mark: yaml_mark_t {
            index: 0_u64,
            line: 0_u64,
            column: 0_u64,
        },
    };
    if if (*parser).simple_keys.top != (*parser).simple_keys.end
        || yaml_stack_extend(
            addr_of_mut!((*parser).simple_keys.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).simple_keys.top) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).simple_keys.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh5 = addr_of_mut!((*parser).simple_keys.top);
        let fresh6 = *fresh5;
        *fresh5 = (*fresh5).wrapping_offset(1);
        *fresh6 = empty_simple_key;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    if (*parser).flow_level == 2147483647_i32 {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0_i32;
    }
    let fresh7 = addr_of_mut!((*parser).flow_level);
    *fresh7 += 1;
    1_i32
}

unsafe fn yaml_parser_decrease_flow_level(parser: *mut yaml_parser_t) -> libc::c_int {
    if (*parser).flow_level != 0 {
        let fresh8 = addr_of_mut!((*parser).flow_level);
        *fresh8 -= 1;
        let fresh9 = addr_of_mut!((*parser).simple_keys.top);
        *fresh9 = (*fresh9).wrapping_offset(-1);
    }
    1_i32
}

unsafe fn yaml_parser_roll_indent(
    mut parser: *mut yaml_parser_t,
    column: ptrdiff_t,
    number: ptrdiff_t,
    type_: yaml_token_type_t,
    mark: yaml_mark_t,
) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if (*parser).flow_level != 0 {
        return 1_i32;
    }
    if ((*parser).indent as libc::c_long) < column {
        if if (*parser).indents.top != (*parser).indents.end
            || yaml_stack_extend(
                addr_of_mut!((*parser).indents.start) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).indents.top) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).indents.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh10 = addr_of_mut!((*parser).indents.top);
            let fresh11 = *fresh10;
            *fresh10 = (*fresh10).wrapping_offset(1);
            *fresh11 = (*parser).indent;
            1_i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0
        {
            return 0_i32;
        }
        if column > 2147483647_i64 {
            (*parser).error = YAML_MEMORY_ERROR;
            return 0_i32;
        }
        (*parser).indent = column as libc::c_int;
        memset(
            token as *mut libc::c_void,
            0_i32,
            size_of::<yaml_token_t>() as libc::c_ulong,
        );
        (*token).type_ = type_;
        (*token).start_mark = mark;
        (*token).end_mark = mark;
        if number == -1_i64 {
            if if (*parser).tokens.tail != (*parser).tokens.end
                || yaml_queue_extend(
                    addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
                    addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
                    addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
                    addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh12 = addr_of_mut!((*parser).tokens.tail);
                let fresh13 = *fresh12;
                *fresh12 = (*fresh12).wrapping_offset(1);
                *fresh13 = *token;
                1_i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0_i32
            } == 0
            {
                return 0_i32;
            }
        } else if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
            ) != 0
        {
            memmove(
                ((*parser).tokens.head)
                    .wrapping_offset(
                        (number as libc::c_ulong).wrapping_sub((*parser).tokens_parsed) as isize,
                    )
                    .wrapping_offset(1_isize) as *mut libc::c_void,
                ((*parser).tokens.head).wrapping_offset(
                    (number as libc::c_ulong).wrapping_sub((*parser).tokens_parsed) as isize,
                ) as *const libc::c_void,
                (((*parser).tokens.tail).c_offset_from((*parser).tokens.head) as libc::c_long
                    as libc::c_ulong)
                    .wrapping_sub((number as libc::c_ulong).wrapping_sub((*parser).tokens_parsed))
                    .wrapping_mul(size_of::<yaml_token_t>() as libc::c_ulong),
            );
            *((*parser).tokens.head).wrapping_offset(
                (number as libc::c_ulong).wrapping_sub((*parser).tokens_parsed) as isize,
            ) = *token;
            let fresh14 = addr_of_mut!((*parser).tokens.tail);
            *fresh14 = (*fresh14).wrapping_offset(1);
            1_i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0
        {
            return 0_i32;
        }
    }
    1_i32
}

unsafe fn yaml_parser_unroll_indent(
    mut parser: *mut yaml_parser_t,
    column: ptrdiff_t,
) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if (*parser).flow_level != 0 {
        return 1_i32;
    }
    while (*parser).indent as libc::c_long > column {
        memset(
            token as *mut libc::c_void,
            0_i32,
            size_of::<yaml_token_t>() as libc::c_ulong,
        );
        (*token).type_ = YAML_BLOCK_END_TOKEN;
        (*token).start_mark = (*parser).mark;
        (*token).end_mark = (*parser).mark;
        if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh15 = addr_of_mut!((*parser).tokens.tail);
            let fresh16 = *fresh15;
            *fresh15 = (*fresh15).wrapping_offset(1);
            *fresh16 = *token;
            1_i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0
        {
            return 0_i32;
        }
        let fresh17 = addr_of_mut!((*parser).indents.top);
        *fresh17 = (*fresh17).wrapping_offset(-1);
        (*parser).indent = **fresh17;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_stream_start(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let simple_key = yaml_simple_key_t {
        possible: 0_i32,
        required: 0_i32,
        token_number: 0_u64,
        mark: yaml_mark_t {
            index: 0_u64,
            line: 0_u64,
            column: 0_u64,
        },
    };
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    (*parser).indent = -1_i32;
    if if (*parser).simple_keys.top != (*parser).simple_keys.end
        || yaml_stack_extend(
            addr_of_mut!((*parser).simple_keys.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).simple_keys.top) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).simple_keys.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh18 = addr_of_mut!((*parser).simple_keys.top);
        let fresh19 = *fresh18;
        *fresh18 = (*fresh18).wrapping_offset(1);
        *fresh19 = simple_key;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 1_i32;
    (*parser).stream_start_produced = 1_i32;
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    (*token).type_ = YAML_STREAM_START_TOKEN;
    (*token).start_mark = (*parser).mark;
    (*token).end_mark = (*parser).mark;
    (*token).data.stream_start.encoding = (*parser).encoding;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh20 = addr_of_mut!((*parser).tokens.tail);
        let fresh21 = *fresh20;
        *fresh20 = (*fresh20).wrapping_offset(1);
        *fresh21 = *token;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_stream_end(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if (*parser).mark.column != 0_u64 {
        (*parser).mark.column = 0_u64;
        let fresh22 = addr_of_mut!((*parser).mark.line);
        *fresh22 = (*fresh22).wrapping_add(1);
    }
    if yaml_parser_unroll_indent(parser, -1_i64) == 0 {
        return 0_i32;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 0_i32;
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    (*token).type_ = YAML_STREAM_END_TOKEN;
    (*token).start_mark = (*parser).mark;
    (*token).end_mark = (*parser).mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh23 = addr_of_mut!((*parser).tokens.tail);
        let fresh24 = *fresh23;
        *fresh23 = (*fresh23).wrapping_offset(1);
        *fresh24 = *token;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_directive(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_unroll_indent(parser, -1_i64) == 0 {
        return 0_i32;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 0_i32;
    if yaml_parser_scan_directive(parser, token) == 0 {
        return 0_i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh25 = addr_of_mut!((*parser).tokens.tail);
        let fresh26 = *fresh25;
        *fresh25 = (*fresh25).wrapping_offset(1);
        ptr::copy_nonoverlapping(token, fresh26, 1);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        yaml_token_delete(token);
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_document_indicator(
    mut parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_unroll_indent(parser, -1_i64) == 0 {
        return 0_i32;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 0_i32;
    let start_mark: yaml_mark_t = (*parser).mark;
    SKIP!(parser);
    SKIP!(parser);
    SKIP!(parser);
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    (*token).type_ = type_;
    (*token).start_mark = start_mark;
    (*token).end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh39 = addr_of_mut!((*parser).tokens.tail);
        let fresh40 = *fresh39;
        *fresh39 = (*fresh39).wrapping_offset(1);
        *fresh40 = *token;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_flow_collection_start(
    mut parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0_i32;
    }
    if yaml_parser_increase_flow_level(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 1_i32;
    let start_mark: yaml_mark_t = (*parser).mark;
    SKIP!(parser);
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    (*token).type_ = type_;
    (*token).start_mark = start_mark;
    (*token).end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh45 = addr_of_mut!((*parser).tokens.tail);
        let fresh46 = *fresh45;
        *fresh45 = (*fresh45).wrapping_offset(1);
        *fresh46 = *token;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_flow_collection_end(
    mut parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0_i32;
    }
    if yaml_parser_decrease_flow_level(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 0_i32;
    let start_mark: yaml_mark_t = (*parser).mark;
    SKIP!(parser);
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    (*token).type_ = type_;
    (*token).start_mark = start_mark;
    (*token).end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh51 = addr_of_mut!((*parser).tokens.tail);
        let fresh52 = *fresh51;
        *fresh51 = (*fresh51).wrapping_offset(1);
        *fresh52 = *token;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_flow_entry(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 1_i32;
    let start_mark: yaml_mark_t = (*parser).mark;
    SKIP!(parser);
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    (*token).type_ = YAML_FLOW_ENTRY_TOKEN;
    (*token).start_mark = start_mark;
    (*token).end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh57 = addr_of_mut!((*parser).tokens.tail);
        let fresh58 = *fresh57;
        *fresh57 = (*fresh57).wrapping_offset(1);
        *fresh58 = *token;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_block_entry(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if (*parser).flow_level == 0 {
        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                ptr::null::<libc::c_char>(),
                (*parser).mark,
                b"block sequence entries are not allowed in this context\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as ptrdiff_t,
            -1_i64,
            YAML_BLOCK_SEQUENCE_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0_i32;
        }
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 1_i32;
    let start_mark: yaml_mark_t = (*parser).mark;
    SKIP!(parser);
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    (*token).type_ = YAML_BLOCK_ENTRY_TOKEN;
    (*token).start_mark = start_mark;
    (*token).end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh63 = addr_of_mut!((*parser).tokens.tail);
        let fresh64 = *fresh63;
        *fresh63 = (*fresh63).wrapping_offset(1);
        *fresh64 = *token;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_key(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if (*parser).flow_level == 0 {
        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                ptr::null::<libc::c_char>(),
                (*parser).mark,
                b"mapping keys are not allowed in this context\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as ptrdiff_t,
            -1_i64,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0_i32;
        }
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = ((*parser).flow_level == 0) as libc::c_int;
    let start_mark: yaml_mark_t = (*parser).mark;
    SKIP!(parser);
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    (*token).type_ = YAML_KEY_TOKEN;
    (*token).start_mark = start_mark;
    (*token).end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh69 = addr_of_mut!((*parser).tokens.tail);
        let fresh70 = *fresh69;
        *fresh69 = (*fresh69).wrapping_offset(1);
        *fresh70 = *token;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_value(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    let mut simple_key: *mut yaml_simple_key_t =
        ((*parser).simple_keys.top).wrapping_offset(-(1_isize));
    if (*simple_key).possible != 0 {
        memset(
            token as *mut libc::c_void,
            0_i32,
            size_of::<yaml_token_t>() as libc::c_ulong,
        );
        (*token).type_ = YAML_KEY_TOKEN;
        (*token).start_mark = (*simple_key).mark;
        (*token).end_mark = (*simple_key).mark;
        if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
                addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
            ) != 0
        {
            memmove(
                ((*parser).tokens.head)
                    .wrapping_offset(
                        ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed) as isize,
                    )
                    .wrapping_offset(1_isize) as *mut libc::c_void,
                ((*parser).tokens.head).wrapping_offset(
                    ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed) as isize,
                ) as *const libc::c_void,
                (((*parser).tokens.tail).c_offset_from((*parser).tokens.head) as libc::c_long
                    as libc::c_ulong)
                    .wrapping_sub(
                        ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed),
                    )
                    .wrapping_mul(size_of::<yaml_token_t>() as libc::c_ulong),
            );
            *((*parser).tokens.head).wrapping_offset(
                ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed) as isize,
            ) = *token;
            let fresh71 = addr_of_mut!((*parser).tokens.tail);
            *fresh71 = (*fresh71).wrapping_offset(1);
            1_i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0
        {
            return 0_i32;
        }
        if yaml_parser_roll_indent(
            parser,
            (*simple_key).mark.column as ptrdiff_t,
            (*simple_key).token_number as ptrdiff_t,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*simple_key).mark,
        ) == 0
        {
            return 0_i32;
        }
        (*simple_key).possible = 0_i32;
        (*parser).simple_key_allowed = 0_i32;
    } else {
        if (*parser).flow_level == 0 {
            if (*parser).simple_key_allowed == 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    ptr::null::<libc::c_char>(),
                    (*parser).mark,
                    b"mapping values are not allowed in this context\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if yaml_parser_roll_indent(
                parser,
                (*parser).mark.column as ptrdiff_t,
                -1_i64,
                YAML_BLOCK_MAPPING_START_TOKEN,
                (*parser).mark,
            ) == 0
            {
                return 0_i32;
            }
        }
        (*parser).simple_key_allowed = ((*parser).flow_level == 0) as libc::c_int;
    }
    let start_mark: yaml_mark_t = (*parser).mark;
    SKIP!(parser);
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        token as *mut libc::c_void,
        0_i32,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
    (*token).type_ = YAML_VALUE_TOKEN;
    (*token).start_mark = start_mark;
    (*token).end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh76 = addr_of_mut!((*parser).tokens.tail);
        let fresh77 = *fresh76;
        *fresh76 = (*fresh76).wrapping_offset(1);
        *fresh77 = *token;
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_anchor(
    mut parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 0_i32;
    if yaml_parser_scan_anchor(parser, token, type_) == 0 {
        return 0_i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh78 = addr_of_mut!((*parser).tokens.tail);
        let fresh79 = *fresh78;
        *fresh78 = (*fresh78).wrapping_offset(1);
        ptr::copy_nonoverlapping(token, fresh79, 1);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        yaml_token_delete(token);
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_tag(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 0_i32;
    if yaml_parser_scan_tag(parser, token) == 0 {
        return 0_i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh80 = addr_of_mut!((*parser).tokens.tail);
        let fresh81 = *fresh80;
        *fresh80 = (*fresh80).wrapping_offset(1);
        ptr::copy_nonoverlapping(token, fresh81, 1);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        yaml_token_delete(token);
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_block_scalar(
    mut parser: *mut yaml_parser_t,
    literal: libc::c_int,
) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 1_i32;
    if yaml_parser_scan_block_scalar(parser, token, literal) == 0 {
        return 0_i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh82 = addr_of_mut!((*parser).tokens.tail);
        let fresh83 = *fresh82;
        *fresh82 = (*fresh82).wrapping_offset(1);
        ptr::copy_nonoverlapping(token, fresh83, 1);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        yaml_token_delete(token);
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_flow_scalar(
    mut parser: *mut yaml_parser_t,
    single: libc::c_int,
) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 0_i32;
    if yaml_parser_scan_flow_scalar(parser, token, single) == 0 {
        return 0_i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh84 = addr_of_mut!((*parser).tokens.tail);
        let fresh85 = *fresh84;
        *fresh84 = (*fresh84).wrapping_offset(1);
        ptr::copy_nonoverlapping(token, fresh85, 1);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        yaml_token_delete(token);
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_fetch_plain_scalar(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token = MaybeUninit::<yaml_token_t>::uninit();
    let token = token.as_mut_ptr();
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0_i32;
    }
    (*parser).simple_key_allowed = 0_i32;
    if yaml_parser_scan_plain_scalar(parser, token) == 0 {
        return 0_i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh86 = addr_of_mut!((*parser).tokens.tail);
        let fresh87 = *fresh86;
        *fresh86 = (*fresh86).wrapping_offset(1);
        ptr::copy_nonoverlapping(token, fresh87, 1);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        yaml_token_delete(token);
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_scan_to_next_token(mut parser: *mut yaml_parser_t) -> libc::c_int {
    loop {
        if if (*parser).unread >= 1_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 1_u64)
        } == 0
        {
            return 0_i32;
        }
        if (*parser).mark.column == 0_u64
            && (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -17i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -69i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -65i32 as yaml_char_t as libc::c_int)
        {
            SKIP!(parser);
        }
        if if (*parser).unread >= 1_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 1_u64)
        } == 0
        {
            return 0_i32;
        }
        while *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || ((*parser).flow_level != 0 || (*parser).simple_key_allowed == 0)
                && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
        {
            SKIP!(parser);
            if if (*parser).unread >= 1_u64 {
                1_i32
            } else {
                yaml_parser_update_buffer(parser, 1_u64)
            } == 0
            {
                return 0_i32;
            }
        }
        if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '#' as i32 as yaml_char_t as libc::c_int
        {
            while !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int)
            {
                SKIP!(parser);
                if if (*parser).unread >= 1_u64 {
                    1_i32
                } else {
                    yaml_parser_update_buffer(parser, 1_u64)
                } == 0
                {
                    return 0_i32;
                }
            }
        }
        if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
        {
            break;
        }
        if if (*parser).unread >= 2_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 2_u64)
        } == 0
        {
            return 0_i32;
        }
        if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
        {
            let fresh100 = addr_of_mut!((*parser).mark.index);
            *fresh100 = (*fresh100 as libc::c_ulong).wrapping_add(2_u64) as size_t as size_t;
            (*parser).mark.column = 0_u64;
            let fresh101 = addr_of_mut!((*parser).mark.line);
            *fresh101 = (*fresh101).wrapping_add(1);
            let fresh102 = addr_of_mut!((*parser).unread);
            *fresh102 = (*fresh102 as libc::c_ulong).wrapping_sub(2_u64) as size_t as size_t;
            let fresh103 = addr_of_mut!((*parser).buffer.pointer);
            *fresh103 = (*fresh103).wrapping_offset(2_isize);
        } else if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            let fresh104 = addr_of_mut!((*parser).mark.index);
            *fresh104 = (*fresh104).wrapping_add(1);
            (*parser).mark.column = 0_u64;
            let fresh105 = addr_of_mut!((*parser).mark.line);
            *fresh105 = (*fresh105).wrapping_add(1);
            let fresh106 = addr_of_mut!((*parser).unread);
            *fresh106 = (*fresh106).wrapping_sub(1);
            let fresh107 = addr_of_mut!((*parser).buffer.pointer);
            *fresh107 = (*fresh107).wrapping_offset(
                (if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int & 0x80_i32
                    == 0_i32
                {
                    1_i32
                } else if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    & 0xe0_i32
                    == 0xc0_i32
                {
                    2_i32
                } else if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    & 0xf0_i32
                    == 0xe0_i32
                {
                    3_i32
                } else if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    & 0xf8_i32
                    == 0xf0_i32
                {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            );
        };
        if (*parser).flow_level == 0 {
            (*parser).simple_key_allowed = 1_i32;
        }
    }
    1_i32
}

unsafe fn yaml_parser_scan_directive(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> libc::c_int {
    let mut current_block: u64;
    let end_mark: yaml_mark_t;
    let mut name: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut major: libc::c_int = 0;
    let mut minor: libc::c_int = 0;
    let mut handle: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut prefix: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let start_mark: yaml_mark_t = (*parser).mark;
    SKIP!(parser);
    if !(yaml_parser_scan_directive_name(parser, start_mark, addr_of_mut!(name)) == 0) {
        if strcmp(
            name as *mut libc::c_char,
            b"YAML\0" as *const u8 as *const libc::c_char,
        ) == 0_i32
        {
            if yaml_parser_scan_version_directive_value(
                parser,
                start_mark,
                addr_of_mut!(major),
                addr_of_mut!(minor),
            ) == 0
            {
                current_block = 11397968426844348457;
            } else {
                end_mark = (*parser).mark;
                memset(
                    token as *mut libc::c_void,
                    0_i32,
                    size_of::<yaml_token_t>() as libc::c_ulong,
                );
                (*token).type_ = YAML_VERSION_DIRECTIVE_TOKEN;
                (*token).start_mark = start_mark;
                (*token).end_mark = end_mark;
                (*token).data.version_directive.major = major;
                (*token).data.version_directive.minor = minor;
                current_block = 17407779659766490442;
            }
        } else if strcmp(
            name as *mut libc::c_char,
            b"TAG\0" as *const u8 as *const libc::c_char,
        ) == 0_i32
        {
            if yaml_parser_scan_tag_directive_value(
                parser,
                start_mark,
                addr_of_mut!(handle),
                addr_of_mut!(prefix),
            ) == 0
            {
                current_block = 11397968426844348457;
            } else {
                end_mark = (*parser).mark;
                memset(
                    token as *mut libc::c_void,
                    0_i32,
                    size_of::<yaml_token_t>() as libc::c_ulong,
                );
                (*token).type_ = YAML_TAG_DIRECTIVE_TOKEN;
                (*token).start_mark = start_mark;
                (*token).end_mark = end_mark;
                let fresh112 = addr_of_mut!((*token).data.tag_directive.handle);
                *fresh112 = handle;
                let fresh113 = addr_of_mut!((*token).data.tag_directive.prefix);
                *fresh113 = prefix;
                current_block = 17407779659766490442;
            }
        } else {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a directive\0" as *const u8 as *const libc::c_char,
                start_mark,
                b"found unknown directive name\0" as *const u8 as *const libc::c_char,
            );
            current_block = 11397968426844348457;
        }
        match current_block {
            11397968426844348457 => {}
            _ => {
                if !(if (*parser).unread >= 1_u64 {
                    1_i32
                } else {
                    yaml_parser_update_buffer(parser, 1_u64)
                } == 0)
                {
                    loop {
                        if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int)
                        {
                            current_block = 11584701595673473500;
                            break;
                        }
                        SKIP!(parser);
                        if if (*parser).unread >= 1_u64 {
                            1_i32
                        } else {
                            yaml_parser_update_buffer(parser, 1_u64)
                        } == 0
                        {
                            current_block = 11397968426844348457;
                            break;
                        }
                    }
                    match current_block {
                        11397968426844348457 => {}
                        _ => {
                            if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '#' as i32 as yaml_char_t as libc::c_int
                            {
                                loop {
                                    if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\n' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -62i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -123i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                                as libc::c_int
                                                == -88i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                                as libc::c_int
                                                == -87i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\0' as i32 as yaml_char_t as libc::c_int
                                    {
                                        current_block = 6669252993407410313;
                                        break;
                                    }
                                    SKIP!(parser);
                                    if if (*parser).unread >= 1_u64 {
                                        1_i32
                                    } else {
                                        yaml_parser_update_buffer(parser, 1_u64)
                                    } == 0
                                    {
                                        current_block = 11397968426844348457;
                                        break;
                                    }
                                }
                            } else {
                                current_block = 6669252993407410313;
                            }
                            match current_block {
                                11397968426844348457 => {}
                                _ => {
                                    if !(*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\n' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -62i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -123i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                                as libc::c_int
                                                == -88i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                                as libc::c_int
                                                == -87i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\0' as i32 as yaml_char_t as libc::c_int)
                                    {
                                        yaml_parser_set_scanner_error(
                                            parser,
                                            b"while scanning a directive\0" as *const u8
                                                as *const libc::c_char,
                                            start_mark,
                                            b"did not find expected comment or line break\0"
                                                as *const u8
                                                as *const libc::c_char,
                                        );
                                    } else {
                                        if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(2_isize)
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(2_isize)
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int
                                        {
                                            if if (*parser).unread >= 2_u64 {
                                                1_i32
                                            } else {
                                                yaml_parser_update_buffer(parser, 2_u64)
                                            } == 0
                                            {
                                                current_block = 11397968426844348457;
                                            } else {
                                                if *((*parser).buffer.pointer)
                                                    .wrapping_offset(0_isize)
                                                    as libc::c_int
                                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                                    && *((*parser).buffer.pointer)
                                                        .wrapping_offset(1_isize)
                                                        as libc::c_int
                                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                                {
                                                    let fresh122 =
                                                        addr_of_mut!((*parser).mark.index);
                                                    *fresh122 = (*fresh122 as libc::c_ulong)
                                                        .wrapping_add(2_u64)
                                                        as size_t
                                                        as size_t;
                                                    (*parser).mark.column = 0_u64;
                                                    let fresh123 =
                                                        addr_of_mut!((*parser).mark.line);
                                                    *fresh123 = (*fresh123).wrapping_add(1);
                                                    let fresh124 = addr_of_mut!((*parser).unread);
                                                    *fresh124 = (*fresh124 as libc::c_ulong)
                                                        .wrapping_sub(2_u64)
                                                        as size_t
                                                        as size_t;
                                                    let fresh125 =
                                                        addr_of_mut!((*parser).buffer.pointer);
                                                    *fresh125 =
                                                        (*fresh125).wrapping_offset(2_isize);
                                                } else if *((*parser).buffer.pointer)
                                                    .wrapping_offset(0_isize)
                                                    as libc::c_int
                                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == -62i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(1_isize)
                                                            as libc::c_int
                                                            == -123i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(1_isize)
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(2_isize)
                                                            as libc::c_int
                                                            == -88i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(1_isize)
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(2_isize)
                                                            as libc::c_int
                                                            == -87i32 as yaml_char_t as libc::c_int
                                                {
                                                    let fresh126 =
                                                        addr_of_mut!((*parser).mark.index);
                                                    *fresh126 = (*fresh126).wrapping_add(1);
                                                    (*parser).mark.column = 0_u64;
                                                    let fresh127 =
                                                        addr_of_mut!((*parser).mark.line);
                                                    *fresh127 = (*fresh127).wrapping_add(1);
                                                    let fresh128 = addr_of_mut!((*parser).unread);
                                                    *fresh128 = (*fresh128).wrapping_sub(1);
                                                    let fresh129 =
                                                        addr_of_mut!((*parser).buffer.pointer);
                                                    *fresh129 = (*fresh129).wrapping_offset(
                                                        (if *((*parser).buffer.pointer)
                                                            .wrapping_offset(0_isize)
                                                            as libc::c_int
                                                            & 0x80_i32
                                                            == 0_i32
                                                        {
                                                            1_i32
                                                        } else if *((*parser).buffer.pointer)
                                                            .wrapping_offset(0_isize)
                                                            as libc::c_int
                                                            & 0xe0_i32
                                                            == 0xc0_i32
                                                        {
                                                            2_i32
                                                        } else if *((*parser).buffer.pointer)
                                                            .wrapping_offset(0_isize)
                                                            as libc::c_int
                                                            & 0xf0_i32
                                                            == 0xe0_i32
                                                        {
                                                            3_i32
                                                        } else if *((*parser).buffer.pointer)
                                                            .wrapping_offset(0_isize)
                                                            as libc::c_int
                                                            & 0xf8_i32
                                                            == 0xf0_i32
                                                        {
                                                            4_i32
                                                        } else {
                                                            0_i32
                                                        })
                                                            as isize,
                                                    );
                                                };
                                                current_block = 652864300344834934;
                                            }
                                        } else {
                                            current_block = 652864300344834934;
                                        }
                                        match current_block {
                                            11397968426844348457 => {}
                                            _ => {
                                                yaml_free(name as *mut libc::c_void);
                                                return 1_i32;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free(prefix as *mut libc::c_void);
    yaml_free(handle as *mut libc::c_void);
    yaml_free(name as *mut libc::c_void);
    0_i32
}

unsafe fn yaml_parser_scan_directive_name(
    mut parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    name: *mut *mut yaml_char_t,
) -> libc::c_int {
    let current_block: u64;
    let mut string = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    string.start = yaml_malloc(16_u64) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.wrapping_offset(16_isize);
        memset(string.start as *mut libc::c_void, 0_i32, 16_u64);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
        if !(if (*parser).unread >= 1_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 1_u64)
        } == 0)
        {
            loop {
                if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    >= '0' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        <= '9' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        >= 'A' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            <= 'Z' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        >= 'a' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            <= 'z' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        == '_' as i32
                    || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        == '-' as i32)
                {
                    current_block = 10879442775620481940;
                    break;
                }
                if if if string.pointer.wrapping_offset(5_isize) < string.end
                    || yaml_string_extend(
                        addr_of_mut!(string.start),
                        addr_of_mut!(string.pointer),
                        addr_of_mut!(string.end),
                    ) != 0
                {
                    1_i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0_i32
                } != 0
                {
                    if *(*parser).buffer.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh130 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh131 = *fresh130;
                        *fresh130 = (*fresh130).wrapping_offset(1);
                        let fresh132 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh132 = *fresh131;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh133 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh134 = *fresh133;
                        *fresh133 = (*fresh133).wrapping_offset(1);
                        let fresh135 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh135 = *fresh134;
                        let fresh136 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh137 = *fresh136;
                        *fresh136 = (*fresh136).wrapping_offset(1);
                        let fresh138 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh138 = *fresh137;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh139 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh140 = *fresh139;
                        *fresh139 = (*fresh139).wrapping_offset(1);
                        let fresh141 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh141 = *fresh140;
                        let fresh142 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh143 = *fresh142;
                        *fresh142 = (*fresh142).wrapping_offset(1);
                        let fresh144 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh144 = *fresh143;
                        let fresh145 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh146 = *fresh145;
                        *fresh145 = (*fresh145).wrapping_offset(1);
                        let fresh147 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh147 = *fresh146;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh148 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh149 = *fresh148;
                        *fresh148 = (*fresh148).wrapping_offset(1);
                        let fresh150 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh150 = *fresh149;
                        let fresh151 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh152 = *fresh151;
                        *fresh151 = (*fresh151).wrapping_offset(1);
                        let fresh153 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh153 = *fresh152;
                        let fresh154 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh155 = *fresh154;
                        *fresh154 = (*fresh154).wrapping_offset(1);
                        let fresh156 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh156 = *fresh155;
                        let fresh157 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh158 = *fresh157;
                        *fresh157 = (*fresh157).wrapping_offset(1);
                        let fresh159 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh159 = *fresh158;
                    };
                    let fresh160 = addr_of_mut!((*parser).mark.index);
                    *fresh160 = (*fresh160).wrapping_add(1);
                    let fresh161 = addr_of_mut!((*parser).mark.column);
                    *fresh161 = (*fresh161).wrapping_add(1);
                    let fresh162 = addr_of_mut!((*parser).unread);
                    *fresh162 = (*fresh162).wrapping_sub(1);
                    1_i32
                } else {
                    0_i32
                } == 0
                {
                    current_block = 8318012024179131575;
                    break;
                }
                if if (*parser).unread >= 1_u64 {
                    1_i32
                } else {
                    yaml_parser_update_buffer(parser, 1_u64)
                } == 0
                {
                    current_block = 8318012024179131575;
                    break;
                }
            }
            match current_block {
                8318012024179131575 => {}
                _ => {
                    if string.start == string.pointer {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a directive\0" as *const u8 as *const libc::c_char,
                            start_mark,
                            b"could not find expected directive name\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        == ' ' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '\t' as i32 as yaml_char_t as libc::c_int
                        || (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '\r' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\n' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == -62i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == -123i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == -30i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == -128i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                    as libc::c_int
                                    == -88i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == -30i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == -128i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                    as libc::c_int
                                    == -87i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\0' as i32 as yaml_char_t as libc::c_int))
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a directive\0" as *const u8 as *const libc::c_char,
                            start_mark,
                            b"found unexpected non-alphabetical character\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        *name = string.start;
                        return 1_i32;
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    0_i32
}

unsafe fn yaml_parser_scan_version_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    major: *mut libc::c_int,
    minor: *mut libc::c_int,
) -> libc::c_int {
    if if (*parser).unread >= 1_u64 {
        1_i32
    } else {
        yaml_parser_update_buffer(parser, 1_u64)
    } == 0
    {
        return 0_i32;
    }
    while *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == ' ' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '\t' as i32 as yaml_char_t as libc::c_int
    {
        SKIP!(parser);
        if if (*parser).unread >= 1_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 1_u64)
        } == 0
        {
            return 0_i32;
        }
    }
    if yaml_parser_scan_version_directive_number(parser, start_mark, major) == 0 {
        return 0_i32;
    }
    if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        == '.' as i32 as yaml_char_t as libc::c_int)
    {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0" as *const u8 as *const libc::c_char,
            start_mark,
            b"did not find expected digit or '.' character\0" as *const u8 as *const libc::c_char,
        );
    }
    SKIP!(parser);
    if yaml_parser_scan_version_directive_number(parser, start_mark, minor) == 0 {
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_scan_version_directive_number(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    number: *mut libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_int = 0_i32;
    let mut length: size_t = 0_u64;
    if if (*parser).unread >= 1_u64 {
        1_i32
    } else {
        yaml_parser_update_buffer(parser, 1_u64)
    } == 0
    {
        return 0_i32;
    }
    while *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
        >= '0' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            <= '9' as i32 as yaml_char_t as libc::c_int
    {
        length = length.wrapping_add(1);
        if length > 9_u64 {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a %YAML directive\0" as *const u8 as *const libc::c_char,
                start_mark,
                b"found extremely long version number\0" as *const u8 as *const libc::c_char,
            );
        }
        value = value * 10_i32
            + (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                - '0' as i32 as yaml_char_t as libc::c_int);
        SKIP!(parser);
        if if (*parser).unread >= 1_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 1_u64)
        } == 0
        {
            return 0_i32;
        }
    }
    if length == 0 {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0" as *const u8 as *const libc::c_char,
            start_mark,
            b"did not find expected version number\0" as *const u8 as *const libc::c_char,
        );
    }
    *number = value;
    1_i32
}

unsafe fn yaml_parser_scan_tag_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
    prefix: *mut *mut yaml_char_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut handle_value: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut prefix_value: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    if if (*parser).unread >= 1_u64 {
        1_i32
    } else {
        yaml_parser_update_buffer(parser, 1_u64)
    } == 0
    {
        current_block = 5231181710497607163;
    } else {
        current_block = 14916268686031723178;
    }
    'c_34337: loop {
        match current_block {
            5231181710497607163 => {
                yaml_free(handle_value as *mut libc::c_void);
                yaml_free(prefix_value as *mut libc::c_void);
                return 0_i32;
            }
            _ => {
                if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        == '\t' as i32 as yaml_char_t as libc::c_int
                {
                    SKIP!(parser);
                    if if (*parser).unread >= 1_u64 {
                        1_i32
                    } else {
                        yaml_parser_update_buffer(parser, 1_u64)
                    } == 0
                    {
                        current_block = 5231181710497607163;
                    } else {
                        current_block = 14916268686031723178;
                    }
                } else {
                    if yaml_parser_scan_tag_handle(
                        parser,
                        1_i32,
                        start_mark,
                        addr_of_mut!(handle_value),
                    ) == 0
                    {
                        current_block = 5231181710497607163;
                        continue;
                    }
                    if if (*parser).unread >= 1_u64 {
                        1_i32
                    } else {
                        yaml_parser_update_buffer(parser, 1_u64)
                    } == 0
                    {
                        current_block = 5231181710497607163;
                        continue;
                    }
                    if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        == ' ' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '\t' as i32 as yaml_char_t as libc::c_int)
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a %TAG directive\0" as *const u8
                                as *const libc::c_char,
                            start_mark,
                            b"did not find expected whitespace\0" as *const u8
                                as *const libc::c_char,
                        );
                        current_block = 5231181710497607163;
                    } else {
                        while *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                        {
                            SKIP!(parser);
                            if if (*parser).unread >= 1_u64 {
                                1_i32
                            } else {
                                yaml_parser_update_buffer(parser, 1_u64)
                            } == 0
                            {
                                current_block = 5231181710497607163;
                                continue 'c_34337;
                            }
                        }
                        if yaml_parser_scan_tag_uri(
                            parser,
                            1_i32,
                            1_i32,
                            ptr::null_mut::<yaml_char_t>(),
                            start_mark,
                            addr_of_mut!(prefix_value),
                        ) == 0
                        {
                            current_block = 5231181710497607163;
                            continue;
                        }
                        if if (*parser).unread >= 1_u64 {
                            1_i32
                        } else {
                            yaml_parser_update_buffer(parser, 1_u64)
                        } == 0
                        {
                            current_block = 5231181710497607163;
                            continue;
                        }
                        if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\0' as i32 as yaml_char_t as libc::c_int))
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a %TAG directive\0" as *const u8
                                    as *const libc::c_char,
                                start_mark,
                                b"did not find expected whitespace or line break\0" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block = 5231181710497607163;
                        } else {
                            *handle = handle_value;
                            *prefix = prefix_value;
                            return 1_i32;
                        }
                    }
                }
            }
        }
    }
}

unsafe fn yaml_parser_scan_anchor(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    type_: yaml_token_type_t,
) -> libc::c_int {
    let current_block: u64;
    let mut length: libc::c_int = 0_i32;
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut string = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    string.start = yaml_malloc(16_u64) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.wrapping_offset(16_isize);
        memset(string.start as *mut libc::c_void, 0_i32, 16_u64);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
        start_mark = (*parser).mark;
        SKIP!(parser);
        if !(if (*parser).unread >= 1_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 1_u64)
        } == 0)
        {
            loop {
                if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    >= '0' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        <= '9' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        >= 'A' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            <= 'Z' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        >= 'a' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            <= 'z' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        == '_' as i32
                    || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        == '-' as i32)
                {
                    current_block = 2868539653012386629;
                    break;
                }
                if if if string.pointer.wrapping_offset(5_isize) < string.end
                    || yaml_string_extend(
                        addr_of_mut!(string.start),
                        addr_of_mut!(string.pointer),
                        addr_of_mut!(string.end),
                    ) != 0
                {
                    1_i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0_i32
                } != 0
                {
                    if *(*parser).buffer.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                        let fresh187 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh188 = *fresh187;
                        *fresh187 = (*fresh187).wrapping_offset(1);
                        let fresh189 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh189 = *fresh188;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                        let fresh190 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh191 = *fresh190;
                        *fresh190 = (*fresh190).wrapping_offset(1);
                        let fresh192 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh192 = *fresh191;
                        let fresh193 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh194 = *fresh193;
                        *fresh193 = (*fresh193).wrapping_offset(1);
                        let fresh195 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh195 = *fresh194;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                        let fresh196 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh197 = *fresh196;
                        *fresh196 = (*fresh196).wrapping_offset(1);
                        let fresh198 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh198 = *fresh197;
                        let fresh199 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh200 = *fresh199;
                        *fresh199 = (*fresh199).wrapping_offset(1);
                        let fresh201 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh201 = *fresh200;
                        let fresh202 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh203 = *fresh202;
                        *fresh202 = (*fresh202).wrapping_offset(1);
                        let fresh204 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh204 = *fresh203;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                        let fresh205 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh206 = *fresh205;
                        *fresh205 = (*fresh205).wrapping_offset(1);
                        let fresh207 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh207 = *fresh206;
                        let fresh208 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh209 = *fresh208;
                        *fresh208 = (*fresh208).wrapping_offset(1);
                        let fresh210 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh210 = *fresh209;
                        let fresh211 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh212 = *fresh211;
                        *fresh211 = (*fresh211).wrapping_offset(1);
                        let fresh213 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh213 = *fresh212;
                        let fresh214 = addr_of_mut!((*parser).buffer.pointer);
                        let fresh215 = *fresh214;
                        *fresh214 = (*fresh214).wrapping_offset(1);
                        let fresh216 = string.pointer;
                        string.pointer = string.pointer.wrapping_offset(1);
                        *fresh216 = *fresh215;
                    };
                    let fresh217 = addr_of_mut!((*parser).mark.index);
                    *fresh217 = (*fresh217).wrapping_add(1);
                    let fresh218 = addr_of_mut!((*parser).mark.column);
                    *fresh218 = (*fresh218).wrapping_add(1);
                    let fresh219 = addr_of_mut!((*parser).unread);
                    *fresh219 = (*fresh219).wrapping_sub(1);
                    1_i32
                } else {
                    0_i32
                } == 0
                {
                    current_block = 5883759901342942623;
                    break;
                }
                if if (*parser).unread >= 1_u64 {
                    1_i32
                } else {
                    yaml_parser_update_buffer(parser, 1_u64)
                } == 0
                {
                    current_block = 5883759901342942623;
                    break;
                }
                length += 1;
            }
            match current_block {
                5883759901342942623 => {}
                _ => {
                    end_mark = (*parser).mark;
                    if length == 0
                        || !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\0' as i32 as yaml_char_t as libc::c_int)
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '?' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == ':' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == ',' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == ']' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '}' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '%' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '@' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '`' as i32 as yaml_char_t as libc::c_int)
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            if type_ as libc::c_uint
                                == YAML_ANCHOR_TOKEN as libc::c_int as libc::c_uint
                            {
                                b"while scanning an anchor\0" as *const u8 as *const libc::c_char
                            } else {
                                b"while scanning an alias\0" as *const u8 as *const libc::c_char
                            },
                            start_mark,
                            b"did not find expected alphabetic or numeric character\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        if type_ as libc::c_uint == YAML_ANCHOR_TOKEN as libc::c_int as libc::c_uint
                        {
                            memset(
                                token as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_ = YAML_ANCHOR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh220 = addr_of_mut!((*token).data.anchor.value);
                            *fresh220 = string.start;
                        } else {
                            memset(
                                token as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_ = YAML_ALIAS_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh221 = addr_of_mut!((*token).data.alias.value);
                            *fresh221 = string.start;
                        }
                        return 1_i32;
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    0_i32
}

unsafe fn yaml_parser_scan_tag(
    parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut handle: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut suffix: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let end_mark: yaml_mark_t;
    let start_mark: yaml_mark_t = (*parser).mark;
    if !(if (*parser).unread >= 2_u64 {
        1_i32
    } else {
        yaml_parser_update_buffer(parser, 2_u64)
    } == 0)
    {
        if *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
            == '<' as i32 as yaml_char_t as libc::c_int
        {
            handle = yaml_malloc(1_u64) as *mut yaml_char_t;
            if handle.is_null() {
                current_block = 17708497480799081542;
            } else {
                *handle.wrapping_offset(0_isize) = '\0' as i32 as yaml_char_t;
                SKIP!(parser);
                SKIP!(parser);
                if yaml_parser_scan_tag_uri(
                    parser,
                    1_i32,
                    0_i32,
                    ptr::null_mut::<yaml_char_t>(),
                    start_mark,
                    addr_of_mut!(suffix),
                ) == 0
                {
                    current_block = 17708497480799081542;
                } else if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == '>' as i32 as yaml_char_t as libc::c_int)
                {
                    yaml_parser_set_scanner_error(
                        parser,
                        b"while scanning a tag\0" as *const u8 as *const libc::c_char,
                        start_mark,
                        b"did not find the expected '>'\0" as *const u8 as *const libc::c_char,
                    );
                    current_block = 17708497480799081542;
                } else {
                    SKIP!(parser);
                    current_block = 4488286894823169796;
                }
            }
        } else if yaml_parser_scan_tag_handle(parser, 0_i32, start_mark, addr_of_mut!(handle)) == 0
        {
            current_block = 17708497480799081542;
        } else if *handle.wrapping_offset(0_isize) as libc::c_int == '!' as i32
            && *handle.wrapping_offset(1_isize) as libc::c_int != '\0' as i32
            && *handle
                .wrapping_offset((strlen(handle as *mut libc::c_char)).wrapping_sub(1_u64) as isize)
                as libc::c_int
                == '!' as i32
        {
            if yaml_parser_scan_tag_uri(
                parser,
                0_i32,
                0_i32,
                ptr::null_mut::<yaml_char_t>(),
                start_mark,
                addr_of_mut!(suffix),
            ) == 0
            {
                current_block = 17708497480799081542;
            } else {
                current_block = 4488286894823169796;
            }
        } else if yaml_parser_scan_tag_uri(
            parser,
            0_i32,
            0_i32,
            handle,
            start_mark,
            addr_of_mut!(suffix),
        ) == 0
        {
            current_block = 17708497480799081542;
        } else {
            yaml_free(handle as *mut libc::c_void);
            handle = yaml_malloc(2_u64) as *mut yaml_char_t;
            if handle.is_null() {
                current_block = 17708497480799081542;
            } else {
                *handle.wrapping_offset(0_isize) = '!' as i32 as yaml_char_t;
                *handle.wrapping_offset(1_isize) = '\0' as i32 as yaml_char_t;
                if *suffix.wrapping_offset(0_isize) as libc::c_int == '\0' as i32 {
                    let tmp: *mut yaml_char_t = handle;
                    handle = suffix;
                    suffix = tmp;
                }
                current_block = 4488286894823169796;
            }
        }
        match current_block {
            17708497480799081542 => {}
            _ => {
                if !(if (*parser).unread >= 1_u64 {
                    1_i32
                } else {
                    yaml_parser_update_buffer(parser, 1_u64)
                } == 0)
                {
                    if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        == ' ' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '\t' as i32 as yaml_char_t as libc::c_int
                        || (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '\r' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\n' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == -62i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == -123i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == -30i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == -128i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                    as libc::c_int
                                    == -88i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == -30i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == -128i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                    as libc::c_int
                                    == -87i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\0' as i32 as yaml_char_t as libc::c_int))
                    {
                        if (*parser).flow_level == 0
                            || !(*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                as libc::c_int
                                == ',' as i32 as yaml_char_t as libc::c_int)
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a tag\0" as *const u8 as *const libc::c_char,
                                start_mark,
                                b"did not find expected whitespace or line break\0" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block = 17708497480799081542;
                        } else {
                            current_block = 7333393191927787629;
                        }
                    } else {
                        current_block = 7333393191927787629;
                    }
                    match current_block {
                        17708497480799081542 => {}
                        _ => {
                            end_mark = (*parser).mark;
                            memset(
                                token as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_ = YAML_TAG_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh234 = addr_of_mut!((*token).data.tag.handle);
                            *fresh234 = handle;
                            let fresh235 = addr_of_mut!((*token).data.tag.suffix);
                            *fresh235 = suffix;
                            return 1_i32;
                        }
                    }
                }
            }
        }
    }
    yaml_free(handle as *mut libc::c_void);
    yaml_free(suffix as *mut libc::c_void);
    0_i32
}

unsafe fn yaml_parser_scan_tag_handle(
    mut parser: *mut yaml_parser_t,
    directive: libc::c_int,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut string = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    string.start = yaml_malloc(16_u64) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.wrapping_offset(16_isize);
        memset(string.start as *mut libc::c_void, 0_i32, 16_u64);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
        if !(if (*parser).unread >= 1_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 1_u64)
        } == 0)
        {
            if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '!' as i32 as yaml_char_t as libc::c_int)
            {
                yaml_parser_set_scanner_error(
                    parser,
                    if directive != 0 {
                        b"while scanning a tag directive\0" as *const u8 as *const libc::c_char
                    } else {
                        b"while scanning a tag\0" as *const u8 as *const libc::c_char
                    },
                    start_mark,
                    b"did not find expected '!'\0" as *const u8 as *const libc::c_char,
                );
            } else if !(if if string.pointer.wrapping_offset(5_isize) < string.end
                || yaml_string_extend(
                    addr_of_mut!(string.start),
                    addr_of_mut!(string.pointer),
                    addr_of_mut!(string.end),
                ) != 0
            {
                1_i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0_i32
            } != 0
            {
                if *(*parser).buffer.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                    let fresh236 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh237 = *fresh236;
                    *fresh236 = (*fresh236).wrapping_offset(1);
                    let fresh238 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh238 = *fresh237;
                } else if *(*parser).buffer.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                    let fresh239 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh240 = *fresh239;
                    *fresh239 = (*fresh239).wrapping_offset(1);
                    let fresh241 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh241 = *fresh240;
                    let fresh242 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh243 = *fresh242;
                    *fresh242 = (*fresh242).wrapping_offset(1);
                    let fresh244 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh244 = *fresh243;
                } else if *(*parser).buffer.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                    let fresh245 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh246 = *fresh245;
                    *fresh245 = (*fresh245).wrapping_offset(1);
                    let fresh247 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh247 = *fresh246;
                    let fresh248 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh249 = *fresh248;
                    *fresh248 = (*fresh248).wrapping_offset(1);
                    let fresh250 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh250 = *fresh249;
                    let fresh251 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh252 = *fresh251;
                    *fresh251 = (*fresh251).wrapping_offset(1);
                    let fresh253 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh253 = *fresh252;
                } else if *(*parser).buffer.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                    let fresh254 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh255 = *fresh254;
                    *fresh254 = (*fresh254).wrapping_offset(1);
                    let fresh256 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh256 = *fresh255;
                    let fresh257 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh258 = *fresh257;
                    *fresh257 = (*fresh257).wrapping_offset(1);
                    let fresh259 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh259 = *fresh258;
                    let fresh260 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh261 = *fresh260;
                    *fresh260 = (*fresh260).wrapping_offset(1);
                    let fresh262 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh262 = *fresh261;
                    let fresh263 = addr_of_mut!((*parser).buffer.pointer);
                    let fresh264 = *fresh263;
                    *fresh263 = (*fresh263).wrapping_offset(1);
                    let fresh265 = string.pointer;
                    string.pointer = string.pointer.wrapping_offset(1);
                    *fresh265 = *fresh264;
                };
                let fresh266 = addr_of_mut!((*parser).mark.index);
                *fresh266 = (*fresh266).wrapping_add(1);
                let fresh267 = addr_of_mut!((*parser).mark.column);
                *fresh267 = (*fresh267).wrapping_add(1);
                let fresh268 = addr_of_mut!((*parser).unread);
                *fresh268 = (*fresh268).wrapping_sub(1);
                1_i32
            } else {
                0_i32
            } == 0)
            {
                if !(if (*parser).unread >= 1_u64 {
                    1_i32
                } else {
                    yaml_parser_update_buffer(parser, 1_u64)
                } == 0)
                {
                    loop {
                        if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            >= '0' as i32 as yaml_char_t as libc::c_int
                            && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                <= '9' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                >= 'A' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    <= 'Z' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                >= 'a' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    <= 'z' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '_' as i32
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '-' as i32)
                        {
                            current_block = 7651349459974463963;
                            break;
                        }
                        if if if string.pointer.wrapping_offset(5_isize) < string.end
                            || yaml_string_extend(
                                addr_of_mut!(string.start),
                                addr_of_mut!(string.pointer),
                                addr_of_mut!(string.end),
                            ) != 0
                        {
                            1_i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0_i32
                        } != 0
                        {
                            if *(*parser).buffer.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                                let fresh269 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh270 = *fresh269;
                                *fresh269 = (*fresh269).wrapping_offset(1);
                                let fresh271 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh271 = *fresh270;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xe0_i32
                                == 0xc0_i32
                            {
                                let fresh272 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh273 = *fresh272;
                                *fresh272 = (*fresh272).wrapping_offset(1);
                                let fresh274 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh274 = *fresh273;
                                let fresh275 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh276 = *fresh275;
                                *fresh275 = (*fresh275).wrapping_offset(1);
                                let fresh277 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh277 = *fresh276;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xf0_i32
                                == 0xe0_i32
                            {
                                let fresh278 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh279 = *fresh278;
                                *fresh278 = (*fresh278).wrapping_offset(1);
                                let fresh280 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh280 = *fresh279;
                                let fresh281 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh282 = *fresh281;
                                *fresh281 = (*fresh281).wrapping_offset(1);
                                let fresh283 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh283 = *fresh282;
                                let fresh284 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh285 = *fresh284;
                                *fresh284 = (*fresh284).wrapping_offset(1);
                                let fresh286 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh286 = *fresh285;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xf8_i32
                                == 0xf0_i32
                            {
                                let fresh287 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh288 = *fresh287;
                                *fresh287 = (*fresh287).wrapping_offset(1);
                                let fresh289 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh289 = *fresh288;
                                let fresh290 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh291 = *fresh290;
                                *fresh290 = (*fresh290).wrapping_offset(1);
                                let fresh292 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh292 = *fresh291;
                                let fresh293 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh294 = *fresh293;
                                *fresh293 = (*fresh293).wrapping_offset(1);
                                let fresh295 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh295 = *fresh294;
                                let fresh296 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh297 = *fresh296;
                                *fresh296 = (*fresh296).wrapping_offset(1);
                                let fresh298 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh298 = *fresh297;
                            };
                            let fresh299 = addr_of_mut!((*parser).mark.index);
                            *fresh299 = (*fresh299).wrapping_add(1);
                            let fresh300 = addr_of_mut!((*parser).mark.column);
                            *fresh300 = (*fresh300).wrapping_add(1);
                            let fresh301 = addr_of_mut!((*parser).unread);
                            *fresh301 = (*fresh301).wrapping_sub(1);
                            1_i32
                        } else {
                            0_i32
                        } == 0
                        {
                            current_block = 1771849829115608806;
                            break;
                        }
                        if if (*parser).unread >= 1_u64 {
                            1_i32
                        } else {
                            yaml_parser_update_buffer(parser, 1_u64)
                        } == 0
                        {
                            current_block = 1771849829115608806;
                            break;
                        }
                    }
                    match current_block {
                        1771849829115608806 => {}
                        _ => {
                            if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '!' as i32 as yaml_char_t as libc::c_int
                            {
                                if if if string.pointer.wrapping_offset(5_isize) < string.end
                                    || yaml_string_extend(
                                        addr_of_mut!(string.start),
                                        addr_of_mut!(string.pointer),
                                        addr_of_mut!(string.end),
                                    ) != 0
                                {
                                    1_i32
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0_i32
                                } != 0
                                {
                                    if *(*parser).buffer.pointer as libc::c_int & 0x80_i32 == 0_i32
                                    {
                                        let fresh302 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh303 = *fresh302;
                                        *fresh302 = (*fresh302).wrapping_offset(1);
                                        let fresh304 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh304 = *fresh303;
                                    } else if *(*parser).buffer.pointer as libc::c_int & 0xe0_i32
                                        == 0xc0_i32
                                    {
                                        let fresh305 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh306 = *fresh305;
                                        *fresh305 = (*fresh305).wrapping_offset(1);
                                        let fresh307 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh307 = *fresh306;
                                        let fresh308 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh309 = *fresh308;
                                        *fresh308 = (*fresh308).wrapping_offset(1);
                                        let fresh310 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh310 = *fresh309;
                                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf0_i32
                                        == 0xe0_i32
                                    {
                                        let fresh311 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh312 = *fresh311;
                                        *fresh311 = (*fresh311).wrapping_offset(1);
                                        let fresh313 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh313 = *fresh312;
                                        let fresh314 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh315 = *fresh314;
                                        *fresh314 = (*fresh314).wrapping_offset(1);
                                        let fresh316 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh316 = *fresh315;
                                        let fresh317 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh318 = *fresh317;
                                        *fresh317 = (*fresh317).wrapping_offset(1);
                                        let fresh319 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh319 = *fresh318;
                                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf8_i32
                                        == 0xf0_i32
                                    {
                                        let fresh320 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh321 = *fresh320;
                                        *fresh320 = (*fresh320).wrapping_offset(1);
                                        let fresh322 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh322 = *fresh321;
                                        let fresh323 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh324 = *fresh323;
                                        *fresh323 = (*fresh323).wrapping_offset(1);
                                        let fresh325 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh325 = *fresh324;
                                        let fresh326 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh327 = *fresh326;
                                        *fresh326 = (*fresh326).wrapping_offset(1);
                                        let fresh328 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh328 = *fresh327;
                                        let fresh329 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh330 = *fresh329;
                                        *fresh329 = (*fresh329).wrapping_offset(1);
                                        let fresh331 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh331 = *fresh330;
                                    };
                                    let fresh332 = addr_of_mut!((*parser).mark.index);
                                    *fresh332 = (*fresh332).wrapping_add(1);
                                    let fresh333 = addr_of_mut!((*parser).mark.column);
                                    *fresh333 = (*fresh333).wrapping_add(1);
                                    let fresh334 = addr_of_mut!((*parser).unread);
                                    *fresh334 = (*fresh334).wrapping_sub(1);
                                    1_i32
                                } else {
                                    0_i32
                                } == 0
                                {
                                    current_block = 1771849829115608806;
                                } else {
                                    current_block = 5689001924483802034;
                                }
                            } else if directive != 0
                                && !(*string.start.wrapping_offset(0_isize) as libc::c_int
                                    == '!' as i32
                                    && *string.start.wrapping_offset(1_isize) as libc::c_int
                                        == '\0' as i32)
                            {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while parsing a tag directive\0" as *const u8
                                        as *const libc::c_char,
                                    start_mark,
                                    b"did not find expected '!'\0" as *const u8
                                        as *const libc::c_char,
                                );
                                current_block = 1771849829115608806;
                            } else {
                                current_block = 5689001924483802034;
                            }
                            match current_block {
                                1771849829115608806 => {}
                                _ => {
                                    *handle = string.start;
                                    return 1_i32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    0_i32
}

unsafe fn yaml_parser_scan_tag_uri(
    mut parser: *mut yaml_parser_t,
    uri_char: libc::c_int,
    directive: libc::c_int,
    head: *mut yaml_char_t,
    start_mark: yaml_mark_t,
    uri: *mut *mut yaml_char_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut length: size_t = if !head.is_null() {
        strlen(head as *mut libc::c_char)
    } else {
        0_u64
    };
    let mut string = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    string.start = yaml_malloc(16_u64) as *mut yaml_char_t;
    if if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.wrapping_offset(16_isize);
        memset(string.start as *mut libc::c_void, 0_i32, 16_u64);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0
    {
        current_block = 15265153392498847348;
    } else {
        current_block = 14916268686031723178;
    }
    'c_21953: loop {
        match current_block {
            15265153392498847348 => {
                yaml_free(string.start as *mut libc::c_void);
                string.end = ptr::null_mut::<yaml_char_t>();
                string.pointer = string.end;
                string.start = string.pointer;
                return 0_i32;
            }
            _ => {
                if string.end.c_offset_from(string.start) as libc::c_long as size_t <= length {
                    if !(yaml_string_extend(
                        addr_of_mut!(string.start),
                        addr_of_mut!(string.pointer),
                        addr_of_mut!(string.end),
                    ) == 0)
                    {
                        current_block = 14916268686031723178;
                        continue;
                    }
                    (*parser).error = YAML_MEMORY_ERROR;
                    current_block = 15265153392498847348;
                } else {
                    if length > 1_u64 {
                        memcpy(
                            string.start as *mut libc::c_void,
                            head.wrapping_offset(1_isize) as *const libc::c_void,
                            length.wrapping_sub(1_u64),
                        );
                        string.pointer = string
                            .pointer
                            .wrapping_offset(length.wrapping_sub(1_u64) as isize);
                    }
                    if if (*parser).unread >= 1_u64 {
                        1_i32
                    } else {
                        yaml_parser_update_buffer(parser, 1_u64)
                    } == 0
                    {
                        current_block = 15265153392498847348;
                        continue;
                    }
                    while *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        >= '0' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            <= '9' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            >= 'A' as i32 as yaml_char_t as libc::c_int
                            && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                <= 'Z' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            >= 'a' as i32 as yaml_char_t as libc::c_int
                            && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                <= 'z' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '_' as i32
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '-' as i32
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ';' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '/' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '?' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ':' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '@' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '&' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '=' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '+' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '$' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '.' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '%' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '!' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '~' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '*' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '\'' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '(' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ')' as i32 as yaml_char_t as libc::c_int
                        || uri_char != 0
                            && (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == ',' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '[' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == ']' as i32 as yaml_char_t as libc::c_int)
                    {
                        if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '%' as i32 as yaml_char_t as libc::c_int
                        {
                            if if string.pointer.wrapping_offset(5_isize) < string.end
                                || yaml_string_extend(
                                    addr_of_mut!(string.start),
                                    addr_of_mut!(string.pointer),
                                    addr_of_mut!(string.end),
                                ) != 0
                            {
                                1_i32
                            } else {
                                (*parser).error = YAML_MEMORY_ERROR;
                                0_i32
                            } == 0
                            {
                                current_block = 15265153392498847348;
                                continue 'c_21953;
                            }
                            if yaml_parser_scan_uri_escapes(
                                parser,
                                directive,
                                start_mark,
                                addr_of_mut!(string),
                            ) == 0
                            {
                                current_block = 15265153392498847348;
                                continue 'c_21953;
                            }
                        } else if if if string.pointer.wrapping_offset(5_isize) < string.end
                            || yaml_string_extend(
                                addr_of_mut!(string.start),
                                addr_of_mut!(string.pointer),
                                addr_of_mut!(string.end),
                            ) != 0
                        {
                            1_i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0_i32
                        } != 0
                        {
                            if *(*parser).buffer.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                                let fresh335 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh336 = *fresh335;
                                *fresh335 = (*fresh335).wrapping_offset(1);
                                let fresh337 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh337 = *fresh336;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xe0_i32
                                == 0xc0_i32
                            {
                                let fresh338 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh339 = *fresh338;
                                *fresh338 = (*fresh338).wrapping_offset(1);
                                let fresh340 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh340 = *fresh339;
                                let fresh341 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh342 = *fresh341;
                                *fresh341 = (*fresh341).wrapping_offset(1);
                                let fresh343 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh343 = *fresh342;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xf0_i32
                                == 0xe0_i32
                            {
                                let fresh344 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh345 = *fresh344;
                                *fresh344 = (*fresh344).wrapping_offset(1);
                                let fresh346 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh346 = *fresh345;
                                let fresh347 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh348 = *fresh347;
                                *fresh347 = (*fresh347).wrapping_offset(1);
                                let fresh349 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh349 = *fresh348;
                                let fresh350 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh351 = *fresh350;
                                *fresh350 = (*fresh350).wrapping_offset(1);
                                let fresh352 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh352 = *fresh351;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xf8_i32
                                == 0xf0_i32
                            {
                                let fresh353 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh354 = *fresh353;
                                *fresh353 = (*fresh353).wrapping_offset(1);
                                let fresh355 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh355 = *fresh354;
                                let fresh356 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh357 = *fresh356;
                                *fresh356 = (*fresh356).wrapping_offset(1);
                                let fresh358 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh358 = *fresh357;
                                let fresh359 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh360 = *fresh359;
                                *fresh359 = (*fresh359).wrapping_offset(1);
                                let fresh361 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh361 = *fresh360;
                                let fresh362 = addr_of_mut!((*parser).buffer.pointer);
                                let fresh363 = *fresh362;
                                *fresh362 = (*fresh362).wrapping_offset(1);
                                let fresh364 = string.pointer;
                                string.pointer = string.pointer.wrapping_offset(1);
                                *fresh364 = *fresh363;
                            };
                            let fresh365 = addr_of_mut!((*parser).mark.index);
                            *fresh365 = (*fresh365).wrapping_add(1);
                            let fresh366 = addr_of_mut!((*parser).mark.column);
                            *fresh366 = (*fresh366).wrapping_add(1);
                            let fresh367 = addr_of_mut!((*parser).unread);
                            *fresh367 = (*fresh367).wrapping_sub(1);
                            1_i32
                        } else {
                            0_i32
                        } == 0
                        {
                            current_block = 15265153392498847348;
                            continue 'c_21953;
                        }
                        length = length.wrapping_add(1);
                        if if (*parser).unread >= 1_u64 {
                            1_i32
                        } else {
                            yaml_parser_update_buffer(parser, 1_u64)
                        } == 0
                        {
                            current_block = 15265153392498847348;
                            continue 'c_21953;
                        }
                    }
                    if length == 0 {
                        if if string.pointer.wrapping_offset(5_isize) < string.end
                            || yaml_string_extend(
                                addr_of_mut!(string.start),
                                addr_of_mut!(string.pointer),
                                addr_of_mut!(string.end),
                            ) != 0
                        {
                            1_i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0_i32
                        } == 0
                        {
                            current_block = 15265153392498847348;
                            continue;
                        }
                        yaml_parser_set_scanner_error(
                            parser,
                            if directive != 0 {
                                b"while parsing a %TAG directive\0" as *const u8
                                    as *const libc::c_char
                            } else {
                                b"while parsing a tag\0" as *const u8 as *const libc::c_char
                            },
                            start_mark,
                            b"did not find expected tag URI\0" as *const u8 as *const libc::c_char,
                        );
                        current_block = 15265153392498847348;
                    } else {
                        *uri = string.start;
                        return 1_i32;
                    }
                }
            }
        }
    }
}

unsafe fn yaml_parser_scan_uri_escapes(
    parser: *mut yaml_parser_t,
    directive: libc::c_int,
    start_mark: yaml_mark_t,
    string: *mut yaml_string_t,
) -> libc::c_int {
    let mut width: libc::c_int = 0_i32;
    loop {
        if if (*parser).unread >= 3_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 3_u64)
        } == 0
        {
            return 0_i32;
        }
        if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '%' as i32 as yaml_char_t as libc::c_int
            && (*((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                >= '0' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    <= '9' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    >= 'A' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        <= 'F' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    >= 'a' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                        <= 'f' as i32 as yaml_char_t as libc::c_int)
            && (*((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                >= '0' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    <= '9' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    >= 'A' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                        <= 'F' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    >= 'a' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                        <= 'f' as i32 as yaml_char_t as libc::c_int))
        {
            return yaml_parser_set_scanner_error(
                parser,
                if directive != 0 {
                    b"while parsing a %TAG directive\0" as *const u8 as *const libc::c_char
                } else {
                    b"while parsing a tag\0" as *const u8 as *const libc::c_char
                },
                start_mark,
                b"did not find URI escaped octet\0" as *const u8 as *const libc::c_char,
            );
        }
        let octet: libc::c_uchar = (((if *((*parser).buffer.pointer).wrapping_offset(1_isize)
            as libc::c_int
            >= 'A' as i32 as yaml_char_t as libc::c_int
            && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                <= 'F' as i32 as yaml_char_t as libc::c_int
        {
            *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                - 'A' as i32 as yaml_char_t as libc::c_int
                + 10_i32
        } else if *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
            >= 'a' as i32 as yaml_char_t as libc::c_int
            && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                <= 'f' as i32 as yaml_char_t as libc::c_int
        {
            *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                - 'a' as i32 as yaml_char_t as libc::c_int
                + 10_i32
        } else {
            *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                - '0' as i32 as yaml_char_t as libc::c_int
        }) << 4_i32)
            + (if *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    <= 'F' as i32 as yaml_char_t as libc::c_int
            {
                *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    - 'A' as i32 as yaml_char_t as libc::c_int
                    + 10_i32
            } else if *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    <= 'f' as i32 as yaml_char_t as libc::c_int
            {
                *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    - 'a' as i32 as yaml_char_t as libc::c_int
                    + 10_i32
            } else {
                *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    - '0' as i32 as yaml_char_t as libc::c_int
            })) as libc::c_uchar;
        if width == 0 {
            width = if octet as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if octet as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                2_i32
            } else if octet as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                3_i32
            } else if octet as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                4_i32
            } else {
                0_i32
            };
            if width == 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    if directive != 0 {
                        b"while parsing a %TAG directive\0" as *const u8 as *const libc::c_char
                    } else {
                        b"while parsing a tag\0" as *const u8 as *const libc::c_char
                    },
                    start_mark,
                    b"found an incorrect leading UTF-8 octet\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if octet as libc::c_int & 0xc0_i32 != 0x80_i32 {
            return yaml_parser_set_scanner_error(
                parser,
                if directive != 0 {
                    b"while parsing a %TAG directive\0" as *const u8 as *const libc::c_char
                } else {
                    b"while parsing a tag\0" as *const u8 as *const libc::c_char
                },
                start_mark,
                b"found an incorrect trailing UTF-8 octet\0" as *const u8 as *const libc::c_char,
            );
        }
        let fresh368 = addr_of_mut!((*string).pointer);
        let fresh369 = *fresh368;
        *fresh368 = (*fresh368).wrapping_offset(1);
        *fresh369 = octet;
        SKIP!(parser);
        SKIP!(parser);
        SKIP!(parser);
        width -= 1;
        if !(width != 0) {
            break;
        }
    }
    1_i32
}

unsafe fn yaml_parser_scan_block_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    literal: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut string = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_break = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut trailing_breaks = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut chomping: libc::c_int = 0_i32;
    let mut increment: libc::c_int = 0_i32;
    let mut indent: libc::c_int = 0_i32;
    let mut leading_blank: libc::c_int = 0_i32;
    let mut trailing_blank: libc::c_int;
    string.start = yaml_malloc(16_u64) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.wrapping_offset(16_isize);
        memset(string.start as *mut libc::c_void, 0_i32, 16_u64);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
        leading_break.start = yaml_malloc(16_u64) as *mut yaml_char_t;
        if !(if !leading_break.start.is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = leading_break.start.wrapping_offset(16_isize);
            memset(leading_break.start as *mut libc::c_void, 0_i32, 16_u64);
            1_i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16_u64) as *mut yaml_char_t;
            if !(if !trailing_breaks.start.is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = trailing_breaks.start.wrapping_offset(16_isize);
                memset(trailing_breaks.start as *mut libc::c_void, 0_i32, 16_u64);
                1_i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0_i32
            } == 0)
            {
                start_mark = (*parser).mark;
                SKIP!(parser);
                if !(if (*parser).unread >= 1_u64 {
                    1_i32
                } else {
                    yaml_parser_update_buffer(parser, 1_u64)
                } == 0)
                {
                    if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        == '+' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '-' as i32 as yaml_char_t as libc::c_int
                    {
                        chomping = if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                            as libc::c_int
                            == '+' as i32 as yaml_char_t as libc::c_int
                        {
                            1_i32
                        } else {
                            -1_i32
                        };
                        SKIP!(parser);
                        if if (*parser).unread >= 1_u64 {
                            1_i32
                        } else {
                            yaml_parser_update_buffer(parser, 1_u64)
                        } == 0
                        {
                            current_block = 14984465786483313892;
                        } else if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                            as libc::c_int
                            >= '0' as i32 as yaml_char_t as libc::c_int
                            && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                <= '9' as i32 as yaml_char_t as libc::c_int
                        {
                            if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '0' as i32 as yaml_char_t as libc::c_int
                            {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while scanning a block scalar\0" as *const u8
                                        as *const libc::c_char,
                                    start_mark,
                                    b"found an indentation indicator equal to 0\0" as *const u8
                                        as *const libc::c_char,
                                );
                                current_block = 14984465786483313892;
                            } else {
                                increment = *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    - '0' as i32 as yaml_char_t as libc::c_int;
                                SKIP!(parser);
                                current_block = 11913429853522160501;
                            }
                        } else {
                            current_block = 11913429853522160501;
                        }
                    } else if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                        >= '0' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            <= '9' as i32 as yaml_char_t as libc::c_int
                    {
                        if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '0' as i32 as yaml_char_t as libc::c_int
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a block scalar\0" as *const u8
                                    as *const libc::c_char,
                                start_mark,
                                b"found an indentation indicator equal to 0\0" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block = 14984465786483313892;
                        } else {
                            increment = *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                as libc::c_int
                                - '0' as i32 as yaml_char_t as libc::c_int;
                            SKIP!(parser);
                            if if (*parser).unread >= 1_u64 {
                                1_i32
                            } else {
                                yaml_parser_update_buffer(parser, 1_u64)
                            } == 0
                            {
                                current_block = 14984465786483313892;
                            } else {
                                if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '+' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '-' as i32 as yaml_char_t as libc::c_int
                                {
                                    chomping = if *((*parser).buffer.pointer)
                                        .wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '+' as i32 as yaml_char_t as libc::c_int
                                    {
                                        1_i32
                                    } else {
                                        -1_i32
                                    };
                                    SKIP!(parser);
                                }
                                current_block = 11913429853522160501;
                            }
                        }
                    } else {
                        current_block = 11913429853522160501;
                    }
                    match current_block {
                        14984465786483313892 => {}
                        _ => {
                            if !(if (*parser).unread >= 1_u64 {
                                1_i32
                            } else {
                                yaml_parser_update_buffer(parser, 1_u64)
                            } == 0)
                            {
                                loop {
                                    if !(*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == ' ' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\t' as i32 as yaml_char_t as libc::c_int)
                                    {
                                        current_block = 4090602189656566074;
                                        break;
                                    }
                                    SKIP!(parser);
                                    if if (*parser).unread >= 1_u64 {
                                        1_i32
                                    } else {
                                        yaml_parser_update_buffer(parser, 1_u64)
                                    } == 0
                                    {
                                        current_block = 14984465786483313892;
                                        break;
                                    }
                                }
                                match current_block {
                                    14984465786483313892 => {}
                                    _ => {
                                        if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '#' as i32 as yaml_char_t as libc::c_int
                                        {
                                            loop {
                                                if *((*parser).buffer.pointer)
                                                    .wrapping_offset(0_isize)
                                                    as libc::c_int
                                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == -62i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(1_isize)
                                                            as libc::c_int
                                                            == -123i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(1_isize)
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(2_isize)
                                                            as libc::c_int
                                                            == -88i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(1_isize)
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(2_isize)
                                                            as libc::c_int
                                                            == -87i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == '\0' as i32 as yaml_char_t as libc::c_int
                                                {
                                                    current_block = 12997042908615822766;
                                                    break;
                                                }
                                                SKIP!(parser);
                                                if if (*parser).unread >= 1_u64 {
                                                    1_i32
                                                } else {
                                                    yaml_parser_update_buffer(parser, 1_u64)
                                                } == 0
                                                {
                                                    current_block = 14984465786483313892;
                                                    break;
                                                }
                                            }
                                        } else {
                                            current_block = 12997042908615822766;
                                        }
                                        match current_block {
                                            14984465786483313892 => {}
                                            _ => {
                                                if !(*((*parser).buffer.pointer)
                                                    .wrapping_offset(0_isize)
                                                    as libc::c_int
                                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == '\n' as i32 as yaml_char_t
                                                            as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == -62i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(1_isize)
                                                            as libc::c_int
                                                            == -123i32 as yaml_char_t
                                                                as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(1_isize)
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t
                                                                as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(2_isize)
                                                            as libc::c_int
                                                            == -88i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(1_isize)
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t
                                                                as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(2_isize)
                                                            as libc::c_int
                                                            == -87i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == '\0' as i32 as yaml_char_t
                                                            as libc::c_int)
                                                {
                                                    yaml_parser_set_scanner_error(
                                                        parser,
                                                        b"while scanning a block scalar\0" as *const u8
                                                            as *const libc::c_char,
                                                        start_mark,
                                                        b"did not find expected comment or line break\0"
                                                            as *const u8 as *const libc::c_char,
                                                    );
                                                } else {
                                                    if *((*parser).buffer.pointer)
                                                        .wrapping_offset(0_isize)
                                                        as libc::c_int
                                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            .wrapping_offset(0_isize)
                                                            as libc::c_int
                                                            == '\n' as i32 as yaml_char_t
                                                                as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            .wrapping_offset(0_isize)
                                                            as libc::c_int
                                                            == -62i32 as yaml_char_t as libc::c_int
                                                            && *((*parser).buffer.pointer)
                                                                .wrapping_offset(1_isize)
                                                                as libc::c_int
                                                                == -123i32 as yaml_char_t
                                                                    as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            .wrapping_offset(0_isize)
                                                            as libc::c_int
                                                            == -30i32 as yaml_char_t as libc::c_int
                                                            && *((*parser).buffer.pointer)
                                                                .wrapping_offset(1_isize)
                                                                as libc::c_int
                                                                == -128i32 as yaml_char_t
                                                                    as libc::c_int
                                                            && *((*parser).buffer.pointer)
                                                                .wrapping_offset(2_isize)
                                                                as libc::c_int
                                                                == -88i32 as yaml_char_t
                                                                    as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            .wrapping_offset(0_isize)
                                                            as libc::c_int
                                                            == -30i32 as yaml_char_t as libc::c_int
                                                            && *((*parser).buffer.pointer)
                                                                .wrapping_offset(1_isize)
                                                                as libc::c_int
                                                                == -128i32 as yaml_char_t
                                                                    as libc::c_int
                                                            && *((*parser).buffer.pointer)
                                                                .wrapping_offset(2_isize)
                                                                as libc::c_int
                                                                == -87i32 as yaml_char_t
                                                                    as libc::c_int
                                                    {
                                                        if if (*parser).unread >= 2_u64 {
                                                            1_i32
                                                        } else {
                                                            yaml_parser_update_buffer(parser, 2_u64)
                                                        } == 0
                                                        {
                                                            current_block = 14984465786483313892;
                                                        } else {
                                                            if *((*parser).buffer.pointer)
                                                                .wrapping_offset(0_isize)
                                                                as libc::c_int
                                                                == '\r' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                                && *((*parser).buffer.pointer)
                                                                    .wrapping_offset(1_isize)
                                                                    as libc::c_int
                                                                    == '\n' as i32 as yaml_char_t
                                                                        as libc::c_int
                                                            {
                                                                let fresh410 = addr_of_mut!(
                                                                    (*parser).mark.index
                                                                );
                                                                *fresh410 = (*fresh410
                                                                    as libc::c_ulong)
                                                                    .wrapping_add(2_u64)
                                                                    as size_t
                                                                    as size_t;
                                                                (*parser).mark.column = 0_u64;
                                                                let fresh411 = addr_of_mut!(
                                                                    (*parser).mark.line
                                                                );
                                                                *fresh411 =
                                                                    (*fresh411).wrapping_add(1);
                                                                let fresh412 =
                                                                    addr_of_mut!((*parser).unread);
                                                                *fresh412 = (*fresh412
                                                                    as libc::c_ulong)
                                                                    .wrapping_sub(2_u64)
                                                                    as size_t
                                                                    as size_t;
                                                                let fresh413 = addr_of_mut!(
                                                                    (*parser).buffer.pointer
                                                                );
                                                                *fresh413 = (*fresh413)
                                                                    .wrapping_offset(2_isize);
                                                            } else if *((*parser).buffer.pointer)
                                                                .wrapping_offset(0_isize)
                                                                as libc::c_int
                                                                == '\r' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                                || *((*parser).buffer.pointer)
                                                                    .wrapping_offset(0_isize)
                                                                    as libc::c_int
                                                                    == '\n' as i32 as yaml_char_t
                                                                        as libc::c_int
                                                                || *((*parser).buffer.pointer)
                                                                    .wrapping_offset(0_isize)
                                                                    as libc::c_int
                                                                    == -62i32 as yaml_char_t
                                                                        as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .wrapping_offset(1_isize)
                                                                        as libc::c_int
                                                                        == -123i32 as yaml_char_t
                                                                            as libc::c_int
                                                                || *((*parser).buffer.pointer)
                                                                    .wrapping_offset(0_isize)
                                                                    as libc::c_int
                                                                    == -30i32 as yaml_char_t
                                                                        as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .wrapping_offset(1_isize)
                                                                        as libc::c_int
                                                                        == -128i32 as yaml_char_t
                                                                            as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .wrapping_offset(2_isize)
                                                                        as libc::c_int
                                                                        == -88i32 as yaml_char_t
                                                                            as libc::c_int
                                                                || *((*parser).buffer.pointer)
                                                                    .wrapping_offset(0_isize)
                                                                    as libc::c_int
                                                                    == -30i32 as yaml_char_t
                                                                        as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .wrapping_offset(1_isize)
                                                                        as libc::c_int
                                                                        == -128i32 as yaml_char_t
                                                                            as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .wrapping_offset(2_isize)
                                                                        as libc::c_int
                                                                        == -87i32 as yaml_char_t
                                                                            as libc::c_int
                                                            {
                                                                let fresh414 = addr_of_mut!(
                                                                    (*parser).mark.index
                                                                );
                                                                *fresh414 =
                                                                    (*fresh414).wrapping_add(1);
                                                                (*parser).mark.column = 0_u64;
                                                                let fresh415 = addr_of_mut!(
                                                                    (*parser).mark.line
                                                                );
                                                                *fresh415 =
                                                                    (*fresh415).wrapping_add(1);
                                                                let fresh416 =
                                                                    addr_of_mut!((*parser).unread);
                                                                *fresh416 =
                                                                    (*fresh416).wrapping_sub(1);
                                                                let fresh417 = addr_of_mut!(
                                                                    (*parser).buffer.pointer
                                                                );
                                                                *fresh417 = (*fresh417)
                                                                    .wrapping_offset(
                                                                        (if *((*parser)
                                                                            .buffer
                                                                            .pointer)
                                                                            .wrapping_offset(
                                                                                0_isize,
                                                                            )
                                                                            as libc::c_int
                                                                            & 0x80_i32
                                                                            == 0_i32
                                                                        {
                                                                            1_i32
                                                                        } else if *((*parser)
                                                                            .buffer
                                                                            .pointer)
                                                                            .wrapping_offset(
                                                                                0_isize,
                                                                            )
                                                                            as libc::c_int
                                                                            & 0xe0_i32
                                                                            == 0xc0_i32
                                                                        {
                                                                            2_i32
                                                                        } else if *((*parser)
                                                                            .buffer
                                                                            .pointer)
                                                                            .wrapping_offset(
                                                                                0_isize,
                                                                            )
                                                                            as libc::c_int
                                                                            & 0xf0_i32
                                                                            == 0xe0_i32
                                                                        {
                                                                            3_i32
                                                                        } else if *((*parser)
                                                                            .buffer
                                                                            .pointer)
                                                                            .wrapping_offset(
                                                                                0_isize,
                                                                            )
                                                                            as libc::c_int
                                                                            & 0xf8_i32
                                                                            == 0xf0_i32
                                                                        {
                                                                            4_i32
                                                                        } else {
                                                                            0_i32
                                                                        })
                                                                            as isize,
                                                                    );
                                                            };
                                                            current_block = 13619784596304402172;
                                                        }
                                                    } else {
                                                        current_block = 13619784596304402172;
                                                    }
                                                    match current_block {
                                                        14984465786483313892 => {}
                                                        _ => {
                                                            end_mark = (*parser).mark;
                                                            if increment != 0 {
                                                                indent =
                                                                    if (*parser).indent >= 0_i32 {
                                                                        (*parser).indent + increment
                                                                    } else {
                                                                        increment
                                                                    };
                                                            }
                                                            if !(yaml_parser_scan_block_scalar_breaks(
                                                                parser,
                                                                addr_of_mut!(indent),
                                                                addr_of_mut!(trailing_breaks),
                                                                start_mark,
                                                                addr_of_mut!(end_mark),
                                                            ) == 0)
                                                            {
                                                                if !(if (*parser).unread
                                                                    >= 1_u64
                                                                {
                                                                    1_i32
                                                                } else {
                                                                    yaml_parser_update_buffer(
                                                                        parser,
                                                                        1_u64,
                                                                    )
                                                                } == 0)
                                                                {
                                                                    's_281: loop {
                                                                        if !((*parser).mark.column as libc::c_int == indent
                                                                            && !(*((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == '\0' as i32 as yaml_char_t as libc::c_int))
                                                                        {
                                                                            current_block = 5793491756164225964;
                                                                            break;
                                                                        }
                                                                        trailing_blank = (*((*parser).buffer.pointer)
                                                                            .wrapping_offset(0_isize) as libc::c_int
                                                                            == ' ' as i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == '\t' as i32 as yaml_char_t as libc::c_int)
                                                                            as libc::c_int;
                                                                        if literal == 0
                                                                            && *leading_break.start as libc::c_int == '\n' as i32
                                                                            && leading_blank == 0 && trailing_blank == 0
                                                                        {
                                                                            if *trailing_breaks.start as libc::c_int == '\0' as i32 {
                                                                                if if string.pointer.wrapping_offset(5_isize)
                                                                                    < string.end
                                                                                    || yaml_string_extend(
                                                                                        addr_of_mut!(string.start),
                                                                                        addr_of_mut!(string.pointer),
                                                                                        addr_of_mut!(string.end),
                                                                                    ) != 0
                                                                                {
                                                                                    1_i32
                                                                                } else {
                                                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                                                    0_i32
                                                                                } == 0
                                                                                {
                                                                                    current_block = 14984465786483313892;
                                                                                    break;
                                                                                }
                                                                                let fresh418 = string.pointer;
                                                                                string.pointer = string.pointer.wrapping_offset(1);
                                                                                *fresh418 = ' ' as i32 as yaml_char_t;
                                                                            }
                                                                            leading_break.pointer = leading_break.start;
                                                                            memset(
                                                                                leading_break.start as *mut libc::c_void,
                                                                                0_i32,
                                                                                leading_break.end.c_offset_from(leading_break.start)
                                                                                    as libc::c_long as libc::c_ulong,
                                                                            );
                                                                        } else {
                                                                            if if yaml_string_join(
                                                                                addr_of_mut!(string.start),
                                                                                addr_of_mut!(string.pointer),
                                                                                addr_of_mut!(string.end),
                                                                                addr_of_mut!(leading_break.start),
                                                                                addr_of_mut!(leading_break.pointer),
                                                                                addr_of_mut!(leading_break.end),
                                                                            ) != 0
                                                                            {
                                                                                leading_break.pointer = leading_break.start;
                                                                                1_i32
                                                                            } else {
                                                                                (*parser).error = YAML_MEMORY_ERROR;
                                                                                0_i32
                                                                            } == 0
                                                                            {
                                                                                current_block = 14984465786483313892;
                                                                                break;
                                                                            }
                                                                            leading_break.pointer = leading_break.start;
                                                                            memset(
                                                                                leading_break.start as *mut libc::c_void,
                                                                                0_i32,
                                                                                leading_break.end.c_offset_from(leading_break.start)
                                                                                    as libc::c_long as libc::c_ulong,
                                                                            );
                                                                        }
                                                                        if if yaml_string_join(
                                                                            addr_of_mut!(string.start),
                                                                            addr_of_mut!(string.pointer),
                                                                            addr_of_mut!(string.end),
                                                                            addr_of_mut!(trailing_breaks.start),
                                                                            addr_of_mut!(trailing_breaks.pointer),
                                                                            addr_of_mut!(trailing_breaks.end),
                                                                        ) != 0
                                                                        {
                                                                            trailing_breaks.pointer = trailing_breaks.start;
                                                                            1_i32
                                                                        } else {
                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                            0_i32
                                                                        } == 0
                                                                        {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                        trailing_breaks.pointer = trailing_breaks.start;
                                                                        memset(
                                                                            trailing_breaks.start as *mut libc::c_void,
                                                                            0_i32,
                                                                            trailing_breaks.end.c_offset_from(trailing_breaks.start)
                                                                                as libc::c_long as libc::c_ulong,
                                                                        );
                                                                        leading_blank = (*((*parser).buffer.pointer)
                                                                            .wrapping_offset(0_isize) as libc::c_int
                                                                            == ' ' as i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == '\t' as i32 as yaml_char_t as libc::c_int)
                                                                            as libc::c_int;
                                                                        while !(*((*parser).buffer.pointer)
                                                                            .wrapping_offset(0_isize) as libc::c_int
                                                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == -62i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .wrapping_offset(1_isize)
                                                                                    as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == -30i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .wrapping_offset(1_isize)
                                                                                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .wrapping_offset(2_isize)
                                                                                    as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == -30i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .wrapping_offset(1_isize)
                                                                                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .wrapping_offset(2_isize)
                                                                                    as libc::c_int == -87i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == '\0' as i32 as yaml_char_t as libc::c_int)
                                                                        {
                                                                            if if if string.pointer.wrapping_offset(5_isize)
                                                                                < string.end
                                                                                || yaml_string_extend(
                                                                                    addr_of_mut!(string.start),
                                                                                    addr_of_mut!(string.pointer),
                                                                                    addr_of_mut!(string.end),
                                                                                ) != 0
                                                                            {
                                                                                1_i32
                                                                            } else {
                                                                                (*parser).error = YAML_MEMORY_ERROR;
                                                                                0_i32
                                                                            } != 0
                                                                            {
                                                                                if *(*parser).buffer.pointer as libc::c_int
                                                                                    & 0x80_i32 == 0_i32
                                                                                {
                                                                                    let fresh419 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh420 = *fresh419;
                                                                                    *fresh419 = (*fresh419).wrapping_offset(1);
                                                                                    let fresh421 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh421 = *fresh420;
                                                                                } else if *(*parser).buffer.pointer as libc::c_int
                                                                                    & 0xe0_i32 == 0xc0_i32
                                                                                {
                                                                                    let fresh422 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh423 = *fresh422;
                                                                                    *fresh422 = (*fresh422).wrapping_offset(1);
                                                                                    let fresh424 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh424 = *fresh423;
                                                                                    let fresh425 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh426 = *fresh425;
                                                                                    *fresh425 = (*fresh425).wrapping_offset(1);
                                                                                    let fresh427 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh427 = *fresh426;
                                                                                } else if *(*parser).buffer.pointer as libc::c_int
                                                                                    & 0xf0_i32 == 0xe0_i32
                                                                                {
                                                                                    let fresh428 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh429 = *fresh428;
                                                                                    *fresh428 = (*fresh428).wrapping_offset(1);
                                                                                    let fresh430 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh430 = *fresh429;
                                                                                    let fresh431 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh432 = *fresh431;
                                                                                    *fresh431 = (*fresh431).wrapping_offset(1);
                                                                                    let fresh433 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh433 = *fresh432;
                                                                                    let fresh434 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh435 = *fresh434;
                                                                                    *fresh434 = (*fresh434).wrapping_offset(1);
                                                                                    let fresh436 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh436 = *fresh435;
                                                                                } else if *(*parser).buffer.pointer as libc::c_int
                                                                                    & 0xf8_i32 == 0xf0_i32
                                                                                {
                                                                                    let fresh437 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh438 = *fresh437;
                                                                                    *fresh437 = (*fresh437).wrapping_offset(1);
                                                                                    let fresh439 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh439 = *fresh438;
                                                                                    let fresh440 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh441 = *fresh440;
                                                                                    *fresh440 = (*fresh440).wrapping_offset(1);
                                                                                    let fresh442 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh442 = *fresh441;
                                                                                    let fresh443 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh444 = *fresh443;
                                                                                    *fresh443 = (*fresh443).wrapping_offset(1);
                                                                                    let fresh445 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh445 = *fresh444;
                                                                                    let fresh446 = addr_of_mut!((*parser).buffer.pointer);
                                                                                    let fresh447 = *fresh446;
                                                                                    *fresh446 = (*fresh446).wrapping_offset(1);
                                                                                    let fresh448 = string.pointer;
                                                                                    string.pointer = string.pointer.wrapping_offset(1);
                                                                                    *fresh448 = *fresh447;
                                                                                } else {};
                                                                                let fresh449 = addr_of_mut!((*parser).mark.index);
                                                                                *fresh449 = (*fresh449).wrapping_add(1);
                                                                                let fresh450 = addr_of_mut!((*parser).mark.column);
                                                                                *fresh450 = (*fresh450).wrapping_add(1);
                                                                                let fresh451 = addr_of_mut!((*parser).unread);
                                                                                *fresh451 = (*fresh451).wrapping_sub(1);
                                                                                1_i32
                                                                            } else {
                                                                                0_i32
                                                                            } == 0
                                                                            {
                                                                                current_block = 14984465786483313892;
                                                                                break 's_281;
                                                                            }
                                                                            if if (*parser).unread >= 1_u64
                                                                            {
                                                                                1_i32
                                                                            } else {
                                                                                yaml_parser_update_buffer(
                                                                                    parser,
                                                                                    1_u64,
                                                                                )
                                                                            } == 0
                                                                            {
                                                                                current_block = 14984465786483313892;
                                                                                break 's_281;
                                                                            }
                                                                        }
                                                                        if if (*parser).unread >= 2_u64
                                                                        {
                                                                            1_i32
                                                                        } else {
                                                                            yaml_parser_update_buffer(
                                                                                parser,
                                                                                2_u64,
                                                                            )
                                                                        } == 0
                                                                        {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                        if if if (leading_break.pointer)
                                                                            .wrapping_offset(5_isize) < leading_break.end
                                                                            || yaml_string_extend(
                                                                                addr_of_mut!(leading_break.start),
                                                                                addr_of_mut!(leading_break.pointer),
                                                                                addr_of_mut!(leading_break.end),
                                                                            ) != 0
                                                                        {
                                                                            1_i32
                                                                        } else {
                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                            0_i32
                                                                        } != 0
                                                                        {
                                                                            if *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == '\r' as i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .wrapping_offset(1_isize) as libc::c_int
                                                                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                                                            {
                                                                                let fresh452 = leading_break.pointer;
                                                                                leading_break.pointer = leading_break.pointer.wrapping_offset(1);
                                                                                *fresh452 = '\n' as i32 as yaml_char_t;
                                                                                let fresh453 = addr_of_mut!((*parser).buffer.pointer);
                                                                                *fresh453 = (*fresh453).wrapping_offset(2_isize);
                                                                                let fresh454 = addr_of_mut!((*parser).mark.index);
                                                                                *fresh454 = (*fresh454 as libc::c_ulong)
                                                                                    .wrapping_add(2_u64) as size_t
                                                                                    as size_t;
                                                                                (*parser).mark.column = 0_u64;
                                                                                let fresh455 = addr_of_mut!((*parser).mark.line);
                                                                                *fresh455 = (*fresh455).wrapping_add(1);
                                                                                let fresh456 = addr_of_mut!((*parser).unread);
                                                                                *fresh456 = (*fresh456 as libc::c_ulong)
                                                                                    .wrapping_sub(2_u64) as size_t
                                                                                    as size_t;
                                                                            } else if *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == '\r' as i32 as yaml_char_t as libc::c_int
                                                                                || *((*parser).buffer.pointer)
                                                                                    .wrapping_offset(0_isize) as libc::c_int
                                                                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                                                            {
                                                                                let fresh457 = leading_break.pointer;
                                                                                leading_break.pointer = leading_break.pointer.wrapping_offset(1);
                                                                                *fresh457 = '\n' as i32 as yaml_char_t;
                                                                                let fresh458 = addr_of_mut!((*parser).buffer.pointer);
                                                                                *fresh458 = (*fresh458).wrapping_offset(1);
                                                                                let fresh459 = addr_of_mut!((*parser).mark.index);
                                                                                *fresh459 = (*fresh459).wrapping_add(1);
                                                                                (*parser).mark.column = 0_u64;
                                                                                let fresh460 = addr_of_mut!((*parser).mark.line);
                                                                                *fresh460 = (*fresh460).wrapping_add(1);
                                                                                let fresh461 = addr_of_mut!((*parser).unread);
                                                                                *fresh461 = (*fresh461).wrapping_sub(1);
                                                                            } else if *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == -62i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .wrapping_offset(1_isize) as libc::c_int
                                                                                    == -123i32 as yaml_char_t as libc::c_int
                                                                            {
                                                                                let fresh462 = leading_break.pointer;
                                                                                leading_break.pointer = leading_break.pointer.wrapping_offset(1);
                                                                                *fresh462 = '\n' as i32 as yaml_char_t;
                                                                                let fresh463 = addr_of_mut!((*parser).buffer.pointer);
                                                                                *fresh463 = (*fresh463).wrapping_offset(2_isize);
                                                                                let fresh464 = addr_of_mut!((*parser).mark.index);
                                                                                *fresh464 = (*fresh464).wrapping_add(1);
                                                                                (*parser).mark.column = 0_u64;
                                                                                let fresh465 = addr_of_mut!((*parser).mark.line);
                                                                                *fresh465 = (*fresh465).wrapping_add(1);
                                                                                let fresh466 = addr_of_mut!((*parser).unread);
                                                                                *fresh466 = (*fresh466).wrapping_sub(1);
                                                                            } else if *((*parser).buffer.pointer)
                                                                                .wrapping_offset(0_isize) as libc::c_int
                                                                                == -30i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .wrapping_offset(1_isize) as libc::c_int
                                                                                    == -128i32 as yaml_char_t as libc::c_int
                                                                                && (*((*parser).buffer.pointer)
                                                                                    .wrapping_offset(2_isize) as libc::c_int
                                                                                    == -88i32 as yaml_char_t as libc::c_int
                                                                                    || *((*parser).buffer.pointer)
                                                                                        .wrapping_offset(2_isize) as libc::c_int
                                                                                        == -87i32 as yaml_char_t as libc::c_int)
                                                                            {
                                                                                let fresh467 = addr_of_mut!((*parser).buffer.pointer);
                                                                                let fresh468 = *fresh467;
                                                                                *fresh467 = (*fresh467).wrapping_offset(1);
                                                                                let fresh469 = leading_break.pointer;
                                                                                leading_break.pointer = leading_break.pointer.wrapping_offset(1);
                                                                                *fresh469 = *fresh468;
                                                                                let fresh470 = addr_of_mut!((*parser).buffer.pointer);
                                                                                let fresh471 = *fresh470;
                                                                                *fresh470 = (*fresh470).wrapping_offset(1);
                                                                                let fresh472 = leading_break.pointer;
                                                                                leading_break.pointer = leading_break.pointer.wrapping_offset(1);
                                                                                *fresh472 = *fresh471;
                                                                                let fresh473 = addr_of_mut!((*parser).buffer.pointer);
                                                                                let fresh474 = *fresh473;
                                                                                *fresh473 = (*fresh473).wrapping_offset(1);
                                                                                let fresh475 = leading_break.pointer;
                                                                                leading_break.pointer = leading_break.pointer.wrapping_offset(1);
                                                                                *fresh475 = *fresh474;
                                                                                let fresh476 = addr_of_mut!((*parser).mark.index);
                                                                                *fresh476 = (*fresh476).wrapping_add(1);
                                                                                (*parser).mark.column = 0_u64;
                                                                                let fresh477 = addr_of_mut!((*parser).mark.line);
                                                                                *fresh477 = (*fresh477).wrapping_add(1);
                                                                                let fresh478 = addr_of_mut!((*parser).unread);
                                                                                *fresh478 = (*fresh478).wrapping_sub(1);
                                                                            } else {};
                                                                            1_i32
                                                                        } else {
                                                                            0_i32
                                                                        } == 0
                                                                        {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                        if yaml_parser_scan_block_scalar_breaks(
                                                                            parser,
                                                                            addr_of_mut!(indent),
                                                                            addr_of_mut!(trailing_breaks),
                                                                            start_mark,
                                                                            addr_of_mut!(end_mark),
                                                                        ) == 0
                                                                        {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                    }
                                                                    match current_block {
                                                                        14984465786483313892 => {}
                                                                        _ => {
                                                                            if chomping != -1_i32 {
                                                                                if if yaml_string_join(
                                                                                    addr_of_mut!(string.start),
                                                                                    addr_of_mut!(string.pointer),
                                                                                    addr_of_mut!(string.end),
                                                                                    addr_of_mut!(leading_break.start),
                                                                                    addr_of_mut!(leading_break.pointer),
                                                                                    addr_of_mut!(leading_break.end),
                                                                                ) != 0
                                                                                {
                                                                                    leading_break.pointer = leading_break.start;
                                                                                    1_i32
                                                                                } else {
                                                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                                                    0_i32
                                                                                } == 0
                                                                                {
                                                                                    current_block = 14984465786483313892;
                                                                                } else {
                                                                                    current_block = 17787701279558130514;
                                                                                }
                                                                            } else {
                                                                                current_block = 17787701279558130514;
                                                                            }
                                                                            match current_block {
                                                                                14984465786483313892 => {}
                                                                                _ => {
                                                                                    if chomping == 1_i32 {
                                                                                        if if yaml_string_join(
                                                                                            addr_of_mut!(string.start),
                                                                                            addr_of_mut!(string.pointer),
                                                                                            addr_of_mut!(string.end),
                                                                                            addr_of_mut!(trailing_breaks.start),
                                                                                            addr_of_mut!(trailing_breaks.pointer),
                                                                                            addr_of_mut!(trailing_breaks.end),
                                                                                        ) != 0
                                                                                        {
                                                                                            trailing_breaks.pointer = trailing_breaks.start;
                                                                                            1_i32
                                                                                        } else {
                                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                                            0_i32
                                                                                        } == 0
                                                                                        {
                                                                                            current_block = 14984465786483313892;
                                                                                        } else {
                                                                                            current_block = 14648606000749551097;
                                                                                        }
                                                                                    } else {
                                                                                        current_block = 14648606000749551097;
                                                                                    }
                                                                                    match current_block {
                                                                                        14984465786483313892 => {}
                                                                                        _ => {
                                                                                            memset(
                                                                                                token as *mut libc::c_void,
                                                                                                0_i32,
                                                                                                size_of::<yaml_token_t>() as libc::c_ulong,
                                                                                            );
                                                                                            (*token).type_ = YAML_SCALAR_TOKEN;
                                                                                            (*token).start_mark = start_mark;
                                                                                            (*token).end_mark = end_mark;
                                                                                            let fresh479 = addr_of_mut!((*token).data.scalar.value);
                                                                                            *fresh479 = string.start;
                                                                                            (*token)
                                                                                                .data
                                                                                                .scalar
                                                                                                .length = string.pointer.c_offset_from(string.start)
                                                                                                as libc::c_long as size_t;
                                                                                            (*token)
                                                                                                .data
                                                                                                .scalar
                                                                                                .style = if literal != 0 {
                                                                                                YAML_LITERAL_SCALAR_STYLE
                                                                                            } else {
                                                                                                YAML_FOLDED_SCALAR_STYLE
                                                                                            };
                                                                                            yaml_free(leading_break.start as *mut libc::c_void);
                                                                                            leading_break.end = ptr::null_mut::<yaml_char_t>();
                                                                                            leading_break.pointer = leading_break.end;
                                                                                            leading_break.start = leading_break.pointer;
                                                                                            yaml_free(trailing_breaks.start as *mut libc::c_void);
                                                                                            trailing_breaks.end = ptr::null_mut::<yaml_char_t>();
                                                                                            trailing_breaks.pointer = trailing_breaks.end;
                                                                                            trailing_breaks.start = trailing_breaks.pointer;
                                                                                            return 1_i32;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    yaml_free(leading_break.start as *mut libc::c_void);
    leading_break.end = ptr::null_mut::<yaml_char_t>();
    leading_break.pointer = leading_break.end;
    leading_break.start = leading_break.pointer;
    yaml_free(trailing_breaks.start as *mut libc::c_void);
    trailing_breaks.end = ptr::null_mut::<yaml_char_t>();
    trailing_breaks.pointer = trailing_breaks.end;
    trailing_breaks.start = trailing_breaks.pointer;
    0_i32
}

unsafe fn yaml_parser_scan_block_scalar_breaks(
    mut parser: *mut yaml_parser_t,
    indent: *mut libc::c_int,
    breaks: *mut yaml_string_t,
    start_mark: yaml_mark_t,
    end_mark: *mut yaml_mark_t,
) -> libc::c_int {
    let mut max_indent: libc::c_int = 0_i32;
    *end_mark = (*parser).mark;
    loop {
        if if (*parser).unread >= 1_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 1_u64)
        } == 0
        {
            return 0_i32;
        }
        while (*indent == 0 || ((*parser).mark.column as libc::c_int) < *indent)
            && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
        {
            SKIP!(parser);
            if if (*parser).unread >= 1_u64 {
                1_i32
            } else {
                yaml_parser_update_buffer(parser, 1_u64)
            } == 0
            {
                return 0_i32;
            }
        }
        if (*parser).mark.column as libc::c_int > max_indent {
            max_indent = (*parser).mark.column as libc::c_int;
        }
        if (*indent == 0 || ((*parser).mark.column as libc::c_int) < *indent)
            && *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
        {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a block scalar\0" as *const u8 as *const libc::c_char,
                start_mark,
                b"found a tab character where an indentation space is expected\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
        {
            break;
        }
        if if (*parser).unread >= 2_u64 {
            1_i32
        } else {
            yaml_parser_update_buffer(parser, 2_u64)
        } == 0
        {
            return 0_i32;
        }
        if if if ((*breaks).pointer).wrapping_offset(5_isize) < (*breaks).end
            || yaml_string_extend(
                addr_of_mut!((*breaks).start),
                addr_of_mut!((*breaks).pointer),
                addr_of_mut!((*breaks).end),
            ) != 0
        {
            1_i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0_i32
        } != 0
        {
            if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let fresh484 = addr_of_mut!((*breaks).pointer);
                let fresh485 = *fresh484;
                *fresh484 = (*fresh484).wrapping_offset(1);
                *fresh485 = '\n' as i32 as yaml_char_t;
                let fresh486 = addr_of_mut!((*parser).buffer.pointer);
                *fresh486 = (*fresh486).wrapping_offset(2_isize);
                let fresh487 = addr_of_mut!((*parser).mark.index);
                *fresh487 = (*fresh487 as libc::c_ulong).wrapping_add(2_u64) as size_t as size_t;
                (*parser).mark.column = 0_u64;
                let fresh488 = addr_of_mut!((*parser).mark.line);
                *fresh488 = (*fresh488).wrapping_add(1);
                let fresh489 = addr_of_mut!((*parser).unread);
                *fresh489 = (*fresh489 as libc::c_ulong).wrapping_sub(2_u64) as size_t as size_t;
            } else if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let fresh490 = addr_of_mut!((*breaks).pointer);
                let fresh491 = *fresh490;
                *fresh490 = (*fresh490).wrapping_offset(1);
                *fresh491 = '\n' as i32 as yaml_char_t;
                let fresh492 = addr_of_mut!((*parser).buffer.pointer);
                *fresh492 = (*fresh492).wrapping_offset(1);
                let fresh493 = addr_of_mut!((*parser).mark.index);
                *fresh493 = (*fresh493).wrapping_add(1);
                (*parser).mark.column = 0_u64;
                let fresh494 = addr_of_mut!((*parser).mark.line);
                *fresh494 = (*fresh494).wrapping_add(1);
                let fresh495 = addr_of_mut!((*parser).unread);
                *fresh495 = (*fresh495).wrapping_sub(1);
            } else if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            {
                let fresh496 = addr_of_mut!((*breaks).pointer);
                let fresh497 = *fresh496;
                *fresh496 = (*fresh496).wrapping_offset(1);
                *fresh497 = '\n' as i32 as yaml_char_t;
                let fresh498 = addr_of_mut!((*parser).buffer.pointer);
                *fresh498 = (*fresh498).wrapping_offset(2_isize);
                let fresh499 = addr_of_mut!((*parser).mark.index);
                *fresh499 = (*fresh499).wrapping_add(1);
                (*parser).mark.column = 0_u64;
                let fresh500 = addr_of_mut!((*parser).mark.line);
                *fresh500 = (*fresh500).wrapping_add(1);
                let fresh501 = addr_of_mut!((*parser).unread);
                *fresh501 = (*fresh501).wrapping_sub(1);
            } else if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && (*((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int)
            {
                let fresh502 = addr_of_mut!((*parser).buffer.pointer);
                let fresh503 = *fresh502;
                *fresh502 = (*fresh502).wrapping_offset(1);
                let fresh504 = addr_of_mut!((*breaks).pointer);
                let fresh505 = *fresh504;
                *fresh504 = (*fresh504).wrapping_offset(1);
                *fresh505 = *fresh503;
                let fresh506 = addr_of_mut!((*parser).buffer.pointer);
                let fresh507 = *fresh506;
                *fresh506 = (*fresh506).wrapping_offset(1);
                let fresh508 = addr_of_mut!((*breaks).pointer);
                let fresh509 = *fresh508;
                *fresh508 = (*fresh508).wrapping_offset(1);
                *fresh509 = *fresh507;
                let fresh510 = addr_of_mut!((*parser).buffer.pointer);
                let fresh511 = *fresh510;
                *fresh510 = (*fresh510).wrapping_offset(1);
                let fresh512 = addr_of_mut!((*breaks).pointer);
                let fresh513 = *fresh512;
                *fresh512 = (*fresh512).wrapping_offset(1);
                *fresh513 = *fresh511;
                let fresh514 = addr_of_mut!((*parser).mark.index);
                *fresh514 = (*fresh514).wrapping_add(1);
                (*parser).mark.column = 0_u64;
                let fresh515 = addr_of_mut!((*parser).mark.line);
                *fresh515 = (*fresh515).wrapping_add(1);
                let fresh516 = addr_of_mut!((*parser).unread);
                *fresh516 = (*fresh516).wrapping_sub(1);
            };
            1_i32
        } else {
            0_i32
        } == 0
        {
            return 0_i32;
        }
        *end_mark = (*parser).mark;
    }
    if *indent == 0 {
        *indent = max_indent;
        if *indent < (*parser).indent + 1_i32 {
            *indent = (*parser).indent + 1_i32;
        }
        if *indent < 1_i32 {
            *indent = 1_i32;
        }
    }
    1_i32
}

unsafe fn yaml_parser_scan_flow_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    single: libc::c_int,
) -> libc::c_int {
    let current_block: u64;
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut string = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_break = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut trailing_breaks = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut whitespaces = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_blanks: libc::c_int;
    string.start = yaml_malloc(16_u64) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.wrapping_offset(16_isize);
        memset(string.start as *mut libc::c_void, 0_i32, 16_u64);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
        leading_break.start = yaml_malloc(16_u64) as *mut yaml_char_t;
        if !(if !leading_break.start.is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = leading_break.start.wrapping_offset(16_isize);
            memset(leading_break.start as *mut libc::c_void, 0_i32, 16_u64);
            1_i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16_u64) as *mut yaml_char_t;
            if !(if !trailing_breaks.start.is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = trailing_breaks.start.wrapping_offset(16_isize);
                memset(trailing_breaks.start as *mut libc::c_void, 0_i32, 16_u64);
                1_i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0_i32
            } == 0)
            {
                whitespaces.start = yaml_malloc(16_u64) as *mut yaml_char_t;
                if !(if !whitespaces.start.is_null() {
                    whitespaces.pointer = whitespaces.start;
                    whitespaces.end = whitespaces.start.wrapping_offset(16_isize);
                    memset(whitespaces.start as *mut libc::c_void, 0_i32, 16_u64);
                    1_i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0_i32
                } == 0)
                {
                    start_mark = (*parser).mark;
                    SKIP!(parser);
                    's_58: loop {
                        if if (*parser).unread >= 4_u64 {
                            1_i32
                        } else {
                            yaml_parser_update_buffer(parser, 4_u64)
                        } == 0
                        {
                            current_block = 8114179180390253173;
                            break;
                        }
                        if (*parser).mark.column == 0_u64
                            && (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int)
                            && (*((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                                || (*((*parser).buffer.pointer).wrapping_offset(3_isize)
                                    as libc::c_int
                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 1_i32) as isize)
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 1_i32) as isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 2_i32) as isize)
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 1_i32) as isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 2_i32) as isize)
                                            as libc::c_int
                                            == -87i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == '\0' as i32 as yaml_char_t as libc::c_int))
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a quoted scalar\0" as *const u8
                                    as *const libc::c_char,
                                start_mark,
                                b"found unexpected document indicator\0" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block = 8114179180390253173;
                            break;
                        } else if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                            as libc::c_int
                            == '\0' as i32 as yaml_char_t as libc::c_int
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a quoted scalar\0" as *const u8
                                    as *const libc::c_char,
                                start_mark,
                                b"found unexpected end of stream\0" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block = 8114179180390253173;
                            break;
                        } else {
                            if if (*parser).unread >= 2_u64 {
                                1_i32
                            } else {
                                yaml_parser_update_buffer(parser, 2_u64)
                            } == 0
                            {
                                current_block = 8114179180390253173;
                                break;
                            }
                            leading_blanks = 0_i32;
                            while !(*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                                || (*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                            as libc::c_int
                                            == -87i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\0' as i32 as yaml_char_t as libc::c_int))
                            {
                                if single != 0
                                    && *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\'' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '\'' as i32 as yaml_char_t as libc::c_int
                                {
                                    if if string.pointer.wrapping_offset(5_isize) < string.end
                                        || yaml_string_extend(
                                            addr_of_mut!(string.start),
                                            addr_of_mut!(string.pointer),
                                            addr_of_mut!(string.end),
                                        ) != 0
                                    {
                                        1_i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0_i32
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                    let fresh521 = string.pointer;
                                    string.pointer = string.pointer.wrapping_offset(1);
                                    *fresh521 = '\'' as i32 as yaml_char_t;
                                    SKIP!(parser);
                                    SKIP!(parser);
                                } else {
                                    if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                            as yaml_char_t
                                            as libc::c_int
                                    {
                                        break;
                                    }
                                    if single == 0
                                        && *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\\' as i32 as yaml_char_t as libc::c_int
                                        && (*((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 1_i32) as isize)
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 1_i32) as isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 2_i32) as isize)
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 1_i32) as isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 2_i32) as isize)
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int)
                                    {
                                        if if (*parser).unread >= 3_u64 {
                                            1_i32
                                        } else {
                                            yaml_parser_update_buffer(parser, 3_u64)
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                        SKIP!(parser);
                                        if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh534 = addr_of_mut!((*parser).mark.index);
                                            *fresh534 = (*fresh534 as libc::c_ulong)
                                                .wrapping_add(2_u64)
                                                as size_t
                                                as size_t;
                                            (*parser).mark.column = 0_u64;
                                            let fresh535 = addr_of_mut!((*parser).mark.line);
                                            *fresh535 = (*fresh535).wrapping_add(1);
                                            let fresh536 = addr_of_mut!((*parser).unread);
                                            *fresh536 = (*fresh536 as libc::c_ulong)
                                                .wrapping_sub(2_u64)
                                                as size_t
                                                as size_t;
                                            let fresh537 = addr_of_mut!((*parser).buffer.pointer);
                                            *fresh537 = (*fresh537).wrapping_offset(2_isize);
                                        } else if *((*parser).buffer.pointer)
                                            .wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(2_isize)
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(2_isize)
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh538 = addr_of_mut!((*parser).mark.index);
                                            *fresh538 = (*fresh538).wrapping_add(1);
                                            (*parser).mark.column = 0_u64;
                                            let fresh539 = addr_of_mut!((*parser).mark.line);
                                            *fresh539 = (*fresh539).wrapping_add(1);
                                            let fresh540 = addr_of_mut!((*parser).unread);
                                            *fresh540 = (*fresh540).wrapping_sub(1);
                                            let fresh541 = addr_of_mut!((*parser).buffer.pointer);
                                            *fresh541 = (*fresh541).wrapping_offset(
                                                (if *((*parser).buffer.pointer)
                                                    .wrapping_offset(0_isize)
                                                    as libc::c_int
                                                    & 0x80_i32
                                                    == 0_i32
                                                {
                                                    1_i32
                                                } else if *((*parser).buffer.pointer)
                                                    .wrapping_offset(0_isize)
                                                    as libc::c_int
                                                    & 0xe0_i32
                                                    == 0xc0_i32
                                                {
                                                    2_i32
                                                } else if *((*parser).buffer.pointer)
                                                    .wrapping_offset(0_isize)
                                                    as libc::c_int
                                                    & 0xf0_i32
                                                    == 0xe0_i32
                                                {
                                                    3_i32
                                                } else if *((*parser).buffer.pointer)
                                                    .wrapping_offset(0_isize)
                                                    as libc::c_int
                                                    & 0xf8_i32
                                                    == 0xf0_i32
                                                {
                                                    4_i32
                                                } else {
                                                    0_i32
                                                })
                                                    as isize,
                                            );
                                        };
                                        leading_blanks = 1_i32;
                                        break;
                                    } else if single == 0
                                        && *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\\' as i32 as yaml_char_t as libc::c_int
                                    {
                                        let mut code_length: size_t = 0_u64;
                                        if if string.pointer.wrapping_offset(5_isize) < string.end
                                            || yaml_string_extend(
                                                addr_of_mut!(string.start),
                                                addr_of_mut!(string.pointer),
                                                addr_of_mut!(string.end),
                                            ) != 0
                                        {
                                            1_i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0_i32
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                        match *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                        {
                                            48 => {
                                                let fresh542 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh542 = '\0' as i32 as yaml_char_t;
                                            }
                                            97 => {
                                                let fresh543 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh543 = '\u{7}' as i32 as yaml_char_t;
                                            }
                                            98 => {
                                                let fresh544 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh544 = '\u{8}' as i32 as yaml_char_t;
                                            }
                                            116 | 9 => {
                                                let fresh545 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh545 = '\t' as i32 as yaml_char_t;
                                            }
                                            110 => {
                                                let fresh546 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh546 = '\n' as i32 as yaml_char_t;
                                            }
                                            118 => {
                                                let fresh547 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh547 = '\u{b}' as i32 as yaml_char_t;
                                            }
                                            102 => {
                                                let fresh548 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh548 = '\u{c}' as i32 as yaml_char_t;
                                            }
                                            114 => {
                                                let fresh549 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh549 = '\r' as i32 as yaml_char_t;
                                            }
                                            101 => {
                                                let fresh550 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh550 = '\u{1b}' as i32 as yaml_char_t;
                                            }
                                            32 => {
                                                let fresh551 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh551 = ' ' as i32 as yaml_char_t;
                                            }
                                            34 => {
                                                let fresh552 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh552 = '"' as i32 as yaml_char_t;
                                            }
                                            47 => {
                                                let fresh553 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh553 = '/' as i32 as yaml_char_t;
                                            }
                                            92 => {
                                                let fresh554 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh554 = '\\' as i32 as yaml_char_t;
                                            }
                                            78 => {
                                                let fresh555 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh555 = -62i32 as yaml_char_t;
                                                let fresh556 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh556 = -123i32 as yaml_char_t;
                                            }
                                            95 => {
                                                let fresh557 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh557 = -62i32 as yaml_char_t;
                                                let fresh558 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh558 = -96i32 as yaml_char_t;
                                            }
                                            76 => {
                                                let fresh559 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh559 = -30i32 as yaml_char_t;
                                                let fresh560 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh560 = -128i32 as yaml_char_t;
                                                let fresh561 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh561 = -88i32 as yaml_char_t;
                                            }
                                            80 => {
                                                let fresh562 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh562 = -30i32 as yaml_char_t;
                                                let fresh563 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh563 = -128i32 as yaml_char_t;
                                                let fresh564 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh564 = -87i32 as yaml_char_t;
                                            }
                                            120 => {
                                                code_length = 2_u64;
                                            }
                                            117 => {
                                                code_length = 4_u64;
                                            }
                                            85 => {
                                                code_length = 8_u64;
                                            }
                                            _ => {
                                                yaml_parser_set_scanner_error(
                                                    parser,
                                                    b"while parsing a quoted scalar\0" as *const u8
                                                        as *const libc::c_char,
                                                    start_mark,
                                                    b"found unknown escape character\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                current_block = 8114179180390253173;
                                                break 's_58;
                                            }
                                        }
                                        SKIP!(parser);
                                        SKIP!(parser);
                                        if code_length != 0 {
                                            let mut value: libc::c_uint = 0_u32;
                                            let mut k: size_t;
                                            if if (*parser).unread >= code_length {
                                                1_i32
                                            } else {
                                                yaml_parser_update_buffer(parser, code_length)
                                            } == 0
                                            {
                                                current_block = 8114179180390253173;
                                                break 's_58;
                                            }
                                            k = 0_u64;
                                            while k < code_length {
                                                if !(*((*parser).buffer.pointer)
                                                    .wrapping_offset(k as isize)
                                                    as libc::c_int
                                                    >= '0' as i32 as yaml_char_t as libc::c_int
                                                    && *((*parser).buffer.pointer)
                                                        .wrapping_offset(k as isize)
                                                        as libc::c_int
                                                        <= '9' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(k as isize)
                                                        as libc::c_int
                                                        >= 'A' as i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(k as isize)
                                                            as libc::c_int
                                                            <= 'F' as i32 as yaml_char_t
                                                                as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(k as isize)
                                                        as libc::c_int
                                                        >= 'a' as i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .wrapping_offset(k as isize)
                                                            as libc::c_int
                                                            <= 'f' as i32 as yaml_char_t
                                                                as libc::c_int)
                                                {
                                                    yaml_parser_set_scanner_error(
                                                        parser,
                                                        b"while parsing a quoted scalar\0"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        start_mark,
                                                        b"did not find expected hexdecimal number\0"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    current_block = 8114179180390253173;
                                                    break 's_58;
                                                } else {
                                                    value = (value << 4_i32).wrapping_add(
                                                        (if *((*parser).buffer.pointer)
                                                            .wrapping_offset(k as isize)
                                                            as libc::c_int
                                                            >= 'A' as i32 as yaml_char_t
                                                                as libc::c_int
                                                            && *((*parser).buffer.pointer)
                                                                .wrapping_offset(k as isize)
                                                                as libc::c_int
                                                                <= 'F' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                        {
                                                            *((*parser).buffer.pointer)
                                                                .wrapping_offset(k as isize)
                                                                as libc::c_int
                                                                - 'A' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                                + 10_i32
                                                        } else if *((*parser).buffer.pointer)
                                                            .wrapping_offset(k as isize)
                                                            as libc::c_int
                                                            >= 'a' as i32 as yaml_char_t
                                                                as libc::c_int
                                                            && *((*parser).buffer.pointer)
                                                                .wrapping_offset(k as isize)
                                                                as libc::c_int
                                                                <= 'f' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                        {
                                                            *((*parser).buffer.pointer)
                                                                .wrapping_offset(k as isize)
                                                                as libc::c_int
                                                                - 'a' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                                + 10_i32
                                                        } else {
                                                            *((*parser).buffer.pointer)
                                                                .wrapping_offset(k as isize)
                                                                as libc::c_int
                                                                - '0' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                        })
                                                            as libc::c_uint,
                                                    );
                                                    k = k.wrapping_add(1);
                                                }
                                            }
                                            if value >= 0xd800_i32 as libc::c_uint
                                                && value <= 0xdfff_i32 as libc::c_uint
                                                || value > 0x10ffff_i32 as libc::c_uint
                                            {
                                                yaml_parser_set_scanner_error(
                                                    parser,
                                                    b"while parsing a quoted scalar\0" as *const u8
                                                        as *const libc::c_char,
                                                    start_mark,
                                                    b"found invalid Unicode character escape code\0"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                );
                                                current_block = 8114179180390253173;
                                                break 's_58;
                                            } else {
                                                if value <= 0x7f_i32 as libc::c_uint {
                                                    let fresh573 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh573 = value as yaml_char_t;
                                                } else if value <= 0x7ff_i32 as libc::c_uint {
                                                    let fresh574 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh574 = (0xc0_i32 as libc::c_uint)
                                                        .wrapping_add(value >> 6_i32)
                                                        as yaml_char_t;
                                                    let fresh575 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh575 = (0x80_i32 as libc::c_uint)
                                                        .wrapping_add(
                                                            value & 0x3f_i32 as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                } else if value <= 0xffff_i32 as libc::c_uint {
                                                    let fresh576 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh576 = (0xe0_i32 as libc::c_uint)
                                                        .wrapping_add(value >> 12_i32)
                                                        as yaml_char_t;
                                                    let fresh577 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh577 = (0x80_i32 as libc::c_uint)
                                                        .wrapping_add(
                                                            value >> 6_i32
                                                                & 0x3f_i32 as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh578 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh578 = (0x80_i32 as libc::c_uint)
                                                        .wrapping_add(
                                                            value & 0x3f_i32 as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                } else {
                                                    let fresh579 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh579 = (0xf0_i32 as libc::c_uint)
                                                        .wrapping_add(value >> 18_i32)
                                                        as yaml_char_t;
                                                    let fresh580 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh580 = (0x80_i32 as libc::c_uint)
                                                        .wrapping_add(
                                                            value >> 12_i32
                                                                & 0x3f_i32 as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh581 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh581 = (0x80_i32 as libc::c_uint)
                                                        .wrapping_add(
                                                            value >> 6_i32
                                                                & 0x3f_i32 as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh582 = string.pointer;
                                                    string.pointer =
                                                        string.pointer.wrapping_offset(1);
                                                    *fresh582 = (0x80_i32 as libc::c_uint)
                                                        .wrapping_add(
                                                            value & 0x3f_i32 as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                }
                                                k = 0_u64;
                                                while k < code_length {
                                                    SKIP!(parser);
                                                    k = k.wrapping_add(1);
                                                }
                                            }
                                        }
                                    } else if if if string.pointer.wrapping_offset(5_isize)
                                        < string.end
                                        || yaml_string_extend(
                                            addr_of_mut!(string.start),
                                            addr_of_mut!(string.pointer),
                                            addr_of_mut!(string.end),
                                        ) != 0
                                    {
                                        1_i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0_i32
                                    } != 0
                                    {
                                        if *(*parser).buffer.pointer as libc::c_int & 0x80_i32
                                            == 0_i32
                                        {
                                            let fresh587 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh588 = *fresh587;
                                            *fresh587 = (*fresh587).wrapping_offset(1);
                                            let fresh589 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh589 = *fresh588;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xe0_i32
                                            == 0xc0_i32
                                        {
                                            let fresh590 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh591 = *fresh590;
                                            *fresh590 = (*fresh590).wrapping_offset(1);
                                            let fresh592 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh592 = *fresh591;
                                            let fresh593 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh594 = *fresh593;
                                            *fresh593 = (*fresh593).wrapping_offset(1);
                                            let fresh595 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh595 = *fresh594;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xf0_i32
                                            == 0xe0_i32
                                        {
                                            let fresh596 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh597 = *fresh596;
                                            *fresh596 = (*fresh596).wrapping_offset(1);
                                            let fresh598 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh598 = *fresh597;
                                            let fresh599 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh600 = *fresh599;
                                            *fresh599 = (*fresh599).wrapping_offset(1);
                                            let fresh601 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh601 = *fresh600;
                                            let fresh602 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh603 = *fresh602;
                                            *fresh602 = (*fresh602).wrapping_offset(1);
                                            let fresh604 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh604 = *fresh603;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xf8_i32
                                            == 0xf0_i32
                                        {
                                            let fresh605 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh606 = *fresh605;
                                            *fresh605 = (*fresh605).wrapping_offset(1);
                                            let fresh607 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh607 = *fresh606;
                                            let fresh608 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh609 = *fresh608;
                                            *fresh608 = (*fresh608).wrapping_offset(1);
                                            let fresh610 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh610 = *fresh609;
                                            let fresh611 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh612 = *fresh611;
                                            *fresh611 = (*fresh611).wrapping_offset(1);
                                            let fresh613 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh613 = *fresh612;
                                            let fresh614 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh615 = *fresh614;
                                            *fresh614 = (*fresh614).wrapping_offset(1);
                                            let fresh616 = string.pointer;
                                            string.pointer = string.pointer.wrapping_offset(1);
                                            *fresh616 = *fresh615;
                                        };
                                        let fresh617 = addr_of_mut!((*parser).mark.index);
                                        *fresh617 = (*fresh617).wrapping_add(1);
                                        let fresh618 = addr_of_mut!((*parser).mark.column);
                                        *fresh618 = (*fresh618).wrapping_add(1);
                                        let fresh619 = addr_of_mut!((*parser).unread);
                                        *fresh619 = (*fresh619).wrapping_sub(1);
                                        1_i32
                                    } else {
                                        0_i32
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                }
                                if if (*parser).unread >= 2_u64 {
                                    1_i32
                                } else {
                                    yaml_parser_update_buffer(parser, 2_u64)
                                } == 0
                                {
                                    current_block = 8114179180390253173;
                                    break 's_58;
                                }
                            }
                            if if (*parser).unread >= 1_u64 {
                                1_i32
                            } else {
                                yaml_parser_update_buffer(parser, 1_u64)
                            } == 0
                            {
                                current_block = 8114179180390253173;
                                break;
                            }
                            if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                    as yaml_char_t as libc::c_int
                            {
                                current_block = 7468767852762055642;
                                break;
                            }
                            if if (*parser).unread >= 1_u64 {
                                1_i32
                            } else {
                                yaml_parser_update_buffer(parser, 1_u64)
                            } == 0
                            {
                                current_block = 8114179180390253173;
                                break;
                            }
                            while *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                                || (*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                            as libc::c_int
                                            == -87i32 as yaml_char_t as libc::c_int)
                            {
                                if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == ' ' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\t' as i32 as yaml_char_t as libc::c_int
                                {
                                    if leading_blanks == 0 {
                                        if if if whitespaces.pointer.wrapping_offset(5_isize)
                                            < whitespaces.end
                                            || yaml_string_extend(
                                                addr_of_mut!(whitespaces.start),
                                                addr_of_mut!(whitespaces.pointer),
                                                addr_of_mut!(whitespaces.end),
                                            ) != 0
                                        {
                                            1_i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0_i32
                                        } != 0
                                        {
                                            if *(*parser).buffer.pointer as libc::c_int & 0x80_i32
                                                == 0_i32
                                            {
                                                let fresh620 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh621 = *fresh620;
                                                *fresh620 = (*fresh620).wrapping_offset(1);
                                                let fresh622 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh622 = *fresh621;
                                            } else if *(*parser).buffer.pointer as libc::c_int
                                                & 0xe0_i32
                                                == 0xc0_i32
                                            {
                                                let fresh623 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh624 = *fresh623;
                                                *fresh623 = (*fresh623).wrapping_offset(1);
                                                let fresh625 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh625 = *fresh624;
                                                let fresh626 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh627 = *fresh626;
                                                *fresh626 = (*fresh626).wrapping_offset(1);
                                                let fresh628 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh628 = *fresh627;
                                            } else if *(*parser).buffer.pointer as libc::c_int
                                                & 0xf0_i32
                                                == 0xe0_i32
                                            {
                                                let fresh629 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh630 = *fresh629;
                                                *fresh629 = (*fresh629).wrapping_offset(1);
                                                let fresh631 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh631 = *fresh630;
                                                let fresh632 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh633 = *fresh632;
                                                *fresh632 = (*fresh632).wrapping_offset(1);
                                                let fresh634 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh634 = *fresh633;
                                                let fresh635 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh636 = *fresh635;
                                                *fresh635 = (*fresh635).wrapping_offset(1);
                                                let fresh637 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh637 = *fresh636;
                                            } else if *(*parser).buffer.pointer as libc::c_int
                                                & 0xf8_i32
                                                == 0xf0_i32
                                            {
                                                let fresh638 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh639 = *fresh638;
                                                *fresh638 = (*fresh638).wrapping_offset(1);
                                                let fresh640 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh640 = *fresh639;
                                                let fresh641 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh642 = *fresh641;
                                                *fresh641 = (*fresh641).wrapping_offset(1);
                                                let fresh643 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh643 = *fresh642;
                                                let fresh644 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh645 = *fresh644;
                                                *fresh644 = (*fresh644).wrapping_offset(1);
                                                let fresh646 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh646 = *fresh645;
                                                let fresh647 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh648 = *fresh647;
                                                *fresh647 = (*fresh647).wrapping_offset(1);
                                                let fresh649 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    whitespaces.pointer.wrapping_offset(1);
                                                *fresh649 = *fresh648;
                                            };
                                            let fresh650 = addr_of_mut!((*parser).mark.index);
                                            *fresh650 = (*fresh650).wrapping_add(1);
                                            let fresh651 = addr_of_mut!((*parser).mark.column);
                                            *fresh651 = (*fresh651).wrapping_add(1);
                                            let fresh652 = addr_of_mut!((*parser).unread);
                                            *fresh652 = (*fresh652).wrapping_sub(1);
                                            1_i32
                                        } else {
                                            0_i32
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                    } else {
                                        SKIP!(parser);
                                    }
                                } else {
                                    if if (*parser).unread >= 2_u64 {
                                        1_i32
                                    } else {
                                        yaml_parser_update_buffer(parser, 2_u64)
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                    if leading_blanks == 0 {
                                        whitespaces.pointer = whitespaces.start;
                                        memset(
                                            whitespaces.start as *mut libc::c_void,
                                            0_i32,
                                            whitespaces.end.c_offset_from(whitespaces.start)
                                                as libc::c_long
                                                as libc::c_ulong,
                                        );
                                        if if if leading_break.pointer.wrapping_offset(5_isize)
                                            < leading_break.end
                                            || yaml_string_extend(
                                                addr_of_mut!(leading_break.start),
                                                addr_of_mut!(leading_break.pointer),
                                                addr_of_mut!(leading_break.end),
                                            ) != 0
                                        {
                                            1_i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0_i32
                                        } != 0
                                        {
                                            if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '\r' as i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                            {
                                                let fresh657 = leading_break.pointer;
                                                leading_break.pointer =
                                                    leading_break.pointer.wrapping_offset(1);
                                                *fresh657 = '\n' as i32 as yaml_char_t;
                                                let fresh658 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                *fresh658 = (*fresh658).wrapping_offset(2_isize);
                                                let fresh659 = addr_of_mut!((*parser).mark.index);
                                                *fresh659 = (*fresh659 as libc::c_ulong)
                                                    .wrapping_add(2_u64)
                                                    as size_t
                                                    as size_t;
                                                (*parser).mark.column = 0_u64;
                                                let fresh660 = addr_of_mut!((*parser).mark.line);
                                                *fresh660 = (*fresh660).wrapping_add(1);
                                                let fresh661 = addr_of_mut!((*parser).unread);
                                                *fresh661 = (*fresh661 as libc::c_ulong)
                                                    .wrapping_sub(2_u64)
                                                    as size_t
                                                    as size_t;
                                            } else if *((*parser).buffer.pointer)
                                                .wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '\r' as i32 as yaml_char_t as libc::c_int
                                                || *((*parser).buffer.pointer)
                                                    .wrapping_offset(0_isize)
                                                    as libc::c_int
                                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                            {
                                                let fresh662 = leading_break.pointer;
                                                leading_break.pointer =
                                                    leading_break.pointer.wrapping_offset(1);
                                                *fresh662 = '\n' as i32 as yaml_char_t;
                                                let fresh663 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                *fresh663 = (*fresh663).wrapping_offset(1);
                                                let fresh664 = addr_of_mut!((*parser).mark.index);
                                                *fresh664 = (*fresh664).wrapping_add(1);
                                                (*parser).mark.column = 0_u64;
                                                let fresh665 = addr_of_mut!((*parser).mark.line);
                                                *fresh665 = (*fresh665).wrapping_add(1);
                                                let fresh666 = addr_of_mut!((*parser).unread);
                                                *fresh666 = (*fresh666).wrapping_sub(1);
                                            } else if *((*parser).buffer.pointer)
                                                .wrapping_offset(0_isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            {
                                                let fresh667 = leading_break.pointer;
                                                leading_break.pointer =
                                                    leading_break.pointer.wrapping_offset(1);
                                                *fresh667 = '\n' as i32 as yaml_char_t;
                                                let fresh668 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                *fresh668 = (*fresh668).wrapping_offset(2_isize);
                                                let fresh669 = addr_of_mut!((*parser).mark.index);
                                                *fresh669 = (*fresh669).wrapping_add(1);
                                                (*parser).mark.column = 0_u64;
                                                let fresh670 = addr_of_mut!((*parser).mark.line);
                                                *fresh670 = (*fresh670).wrapping_add(1);
                                                let fresh671 = addr_of_mut!((*parser).unread);
                                                *fresh671 = (*fresh671).wrapping_sub(1);
                                            } else if *((*parser).buffer.pointer)
                                                .wrapping_offset(0_isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && (*((*parser).buffer.pointer)
                                                    .wrapping_offset(2_isize)
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .wrapping_offset(2_isize)
                                                        as libc::c_int
                                                        == -87i32 as yaml_char_t as libc::c_int)
                                            {
                                                let fresh672 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh673 = *fresh672;
                                                *fresh672 = (*fresh672).wrapping_offset(1);
                                                let fresh674 = leading_break.pointer;
                                                leading_break.pointer =
                                                    leading_break.pointer.wrapping_offset(1);
                                                *fresh674 = *fresh673;
                                                let fresh675 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh676 = *fresh675;
                                                *fresh675 = (*fresh675).wrapping_offset(1);
                                                let fresh677 = leading_break.pointer;
                                                leading_break.pointer =
                                                    leading_break.pointer.wrapping_offset(1);
                                                *fresh677 = *fresh676;
                                                let fresh678 =
                                                    addr_of_mut!((*parser).buffer.pointer);
                                                let fresh679 = *fresh678;
                                                *fresh678 = (*fresh678).wrapping_offset(1);
                                                let fresh680 = leading_break.pointer;
                                                leading_break.pointer =
                                                    leading_break.pointer.wrapping_offset(1);
                                                *fresh680 = *fresh679;
                                                let fresh681 = addr_of_mut!((*parser).mark.index);
                                                *fresh681 = (*fresh681).wrapping_add(1);
                                                (*parser).mark.column = 0_u64;
                                                let fresh682 = addr_of_mut!((*parser).mark.line);
                                                *fresh682 = (*fresh682).wrapping_add(1);
                                                let fresh683 = addr_of_mut!((*parser).unread);
                                                *fresh683 = (*fresh683).wrapping_sub(1);
                                            };
                                            1_i32
                                        } else {
                                            0_i32
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                        leading_blanks = 1_i32;
                                    } else if if if (trailing_breaks.pointer)
                                        .wrapping_offset(5_isize)
                                        < trailing_breaks.end
                                        || yaml_string_extend(
                                            addr_of_mut!(trailing_breaks.start),
                                            addr_of_mut!(trailing_breaks.pointer),
                                            addr_of_mut!(trailing_breaks.end),
                                        ) != 0
                                    {
                                        1_i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0_i32
                                    } != 0
                                    {
                                        if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh684 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                trailing_breaks.pointer.wrapping_offset(1);
                                            *fresh684 = '\n' as i32 as yaml_char_t;
                                            let fresh685 = addr_of_mut!((*parser).buffer.pointer);
                                            *fresh685 = (*fresh685).wrapping_offset(2_isize);
                                            let fresh686 = addr_of_mut!((*parser).mark.index);
                                            *fresh686 = (*fresh686 as libc::c_ulong)
                                                .wrapping_add(2_u64)
                                                as size_t
                                                as size_t;
                                            (*parser).mark.column = 0_u64;
                                            let fresh687 = addr_of_mut!((*parser).mark.line);
                                            *fresh687 = (*fresh687).wrapping_add(1);
                                            let fresh688 = addr_of_mut!((*parser).unread);
                                            *fresh688 = (*fresh688 as libc::c_ulong)
                                                .wrapping_sub(2_u64)
                                                as size_t
                                                as size_t;
                                        } else if *((*parser).buffer.pointer)
                                            .wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh689 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                trailing_breaks.pointer.wrapping_offset(1);
                                            *fresh689 = '\n' as i32 as yaml_char_t;
                                            let fresh690 = addr_of_mut!((*parser).buffer.pointer);
                                            *fresh690 = (*fresh690).wrapping_offset(1);
                                            let fresh691 = addr_of_mut!((*parser).mark.index);
                                            *fresh691 = (*fresh691).wrapping_add(1);
                                            (*parser).mark.column = 0_u64;
                                            let fresh692 = addr_of_mut!((*parser).mark.line);
                                            *fresh692 = (*fresh692).wrapping_add(1);
                                            let fresh693 = addr_of_mut!((*parser).unread);
                                            *fresh693 = (*fresh693).wrapping_sub(1);
                                        } else if *((*parser).buffer.pointer)
                                            .wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -62i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -123i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh694 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                trailing_breaks.pointer.wrapping_offset(1);
                                            *fresh694 = '\n' as i32 as yaml_char_t;
                                            let fresh695 = addr_of_mut!((*parser).buffer.pointer);
                                            *fresh695 = (*fresh695).wrapping_offset(2_isize);
                                            let fresh696 = addr_of_mut!((*parser).mark.index);
                                            *fresh696 = (*fresh696).wrapping_add(1);
                                            (*parser).mark.column = 0_u64;
                                            let fresh697 = addr_of_mut!((*parser).mark.line);
                                            *fresh697 = (*fresh697).wrapping_add(1);
                                            let fresh698 = addr_of_mut!((*parser).unread);
                                            *fresh698 = (*fresh698).wrapping_sub(1);
                                        } else if *((*parser).buffer.pointer)
                                            .wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && (*((*parser).buffer.pointer).wrapping_offset(2_isize)
                                                as libc::c_int
                                                == -88i32 as yaml_char_t as libc::c_int
                                                || *((*parser).buffer.pointer)
                                                    .wrapping_offset(2_isize)
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int)
                                        {
                                            let fresh699 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh700 = *fresh699;
                                            *fresh699 = (*fresh699).wrapping_offset(1);
                                            let fresh701 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                trailing_breaks.pointer.wrapping_offset(1);
                                            *fresh701 = *fresh700;
                                            let fresh702 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh703 = *fresh702;
                                            *fresh702 = (*fresh702).wrapping_offset(1);
                                            let fresh704 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                trailing_breaks.pointer.wrapping_offset(1);
                                            *fresh704 = *fresh703;
                                            let fresh705 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh706 = *fresh705;
                                            *fresh705 = (*fresh705).wrapping_offset(1);
                                            let fresh707 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                trailing_breaks.pointer.wrapping_offset(1);
                                            *fresh707 = *fresh706;
                                            let fresh708 = addr_of_mut!((*parser).mark.index);
                                            *fresh708 = (*fresh708).wrapping_add(1);
                                            (*parser).mark.column = 0_u64;
                                            let fresh709 = addr_of_mut!((*parser).mark.line);
                                            *fresh709 = (*fresh709).wrapping_add(1);
                                            let fresh710 = addr_of_mut!((*parser).unread);
                                            *fresh710 = (*fresh710).wrapping_sub(1);
                                        };
                                        1_i32
                                    } else {
                                        0_i32
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                }
                                if if (*parser).unread >= 1_u64 {
                                    1_i32
                                } else {
                                    yaml_parser_update_buffer(parser, 1_u64)
                                } == 0
                                {
                                    current_block = 8114179180390253173;
                                    break 's_58;
                                }
                            }
                            if leading_blanks != 0 {
                                if *leading_break.start.wrapping_offset(0_isize) as libc::c_int
                                    == '\n' as i32
                                {
                                    if *trailing_breaks.start.wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\0' as i32
                                    {
                                        if if string.pointer.wrapping_offset(5_isize) < string.end
                                            || yaml_string_extend(
                                                addr_of_mut!(string.start),
                                                addr_of_mut!(string.pointer),
                                                addr_of_mut!(string.end),
                                            ) != 0
                                        {
                                            1_i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0_i32
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break;
                                        }
                                        let fresh711 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh711 = ' ' as i32 as yaml_char_t;
                                    } else {
                                        if if yaml_string_join(
                                            addr_of_mut!(string.start),
                                            addr_of_mut!(string.pointer),
                                            addr_of_mut!(string.end),
                                            addr_of_mut!(trailing_breaks.start),
                                            addr_of_mut!(trailing_breaks.pointer),
                                            addr_of_mut!(trailing_breaks.end),
                                        ) != 0
                                        {
                                            trailing_breaks.pointer = trailing_breaks.start;
                                            1_i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0_i32
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break;
                                        }
                                        trailing_breaks.pointer = trailing_breaks.start;
                                        memset(
                                            trailing_breaks.start as *mut libc::c_void,
                                            0_i32,
                                            (trailing_breaks.end)
                                                .c_offset_from(trailing_breaks.start)
                                                as libc::c_long
                                                as libc::c_ulong,
                                        );
                                    }
                                    leading_break.pointer = leading_break.start;
                                    memset(
                                        leading_break.start as *mut libc::c_void,
                                        0_i32,
                                        leading_break.end.c_offset_from(leading_break.start)
                                            as libc::c_long
                                            as libc::c_ulong,
                                    );
                                } else {
                                    if if yaml_string_join(
                                        addr_of_mut!(string.start),
                                        addr_of_mut!(string.pointer),
                                        addr_of_mut!(string.end),
                                        addr_of_mut!(leading_break.start),
                                        addr_of_mut!(leading_break.pointer),
                                        addr_of_mut!(leading_break.end),
                                    ) != 0
                                    {
                                        leading_break.pointer = leading_break.start;
                                        1_i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0_i32
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break;
                                    }
                                    if if yaml_string_join(
                                        addr_of_mut!(string.start),
                                        addr_of_mut!(string.pointer),
                                        addr_of_mut!(string.end),
                                        addr_of_mut!(trailing_breaks.start),
                                        addr_of_mut!(trailing_breaks.pointer),
                                        addr_of_mut!(trailing_breaks.end),
                                    ) != 0
                                    {
                                        trailing_breaks.pointer = trailing_breaks.start;
                                        1_i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0_i32
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break;
                                    }
                                    leading_break.pointer = leading_break.start;
                                    memset(
                                        leading_break.start as *mut libc::c_void,
                                        0_i32,
                                        leading_break.end.c_offset_from(leading_break.start)
                                            as libc::c_long
                                            as libc::c_ulong,
                                    );
                                    trailing_breaks.pointer = trailing_breaks.start;
                                    memset(
                                        trailing_breaks.start as *mut libc::c_void,
                                        0_i32,
                                        trailing_breaks.end.c_offset_from(trailing_breaks.start)
                                            as libc::c_long
                                            as libc::c_ulong,
                                    );
                                }
                            } else {
                                if if yaml_string_join(
                                    addr_of_mut!(string.start),
                                    addr_of_mut!(string.pointer),
                                    addr_of_mut!(string.end),
                                    addr_of_mut!(whitespaces.start),
                                    addr_of_mut!(whitespaces.pointer),
                                    addr_of_mut!(whitespaces.end),
                                ) != 0
                                {
                                    whitespaces.pointer = whitespaces.start;
                                    1_i32
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0_i32
                                } == 0
                                {
                                    current_block = 8114179180390253173;
                                    break;
                                }
                                whitespaces.pointer = whitespaces.start;
                                memset(
                                    whitespaces.start as *mut libc::c_void,
                                    0_i32,
                                    whitespaces.end.c_offset_from(whitespaces.start) as libc::c_long
                                        as libc::c_ulong,
                                );
                            }
                        }
                    }
                    match current_block {
                        8114179180390253173 => {}
                        _ => {
                            SKIP!(parser);
                            end_mark = (*parser).mark;
                            memset(
                                token as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_ = YAML_SCALAR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh716 = addr_of_mut!((*token).data.scalar.value);
                            *fresh716 = string.start;
                            (*token).data.scalar.length = string.pointer.c_offset_from(string.start)
                                as libc::c_long
                                as size_t;
                            (*token).data.scalar.style = if single != 0 {
                                YAML_SINGLE_QUOTED_SCALAR_STYLE
                            } else {
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE
                            };
                            yaml_free(leading_break.start as *mut libc::c_void);
                            leading_break.end = ptr::null_mut::<yaml_char_t>();
                            leading_break.pointer = leading_break.end;
                            leading_break.start = leading_break.pointer;
                            yaml_free(trailing_breaks.start as *mut libc::c_void);
                            trailing_breaks.end = ptr::null_mut::<yaml_char_t>();
                            trailing_breaks.pointer = trailing_breaks.end;
                            trailing_breaks.start = trailing_breaks.pointer;
                            yaml_free(whitespaces.start as *mut libc::c_void);
                            whitespaces.end = ptr::null_mut::<yaml_char_t>();
                            whitespaces.pointer = whitespaces.end;
                            whitespaces.start = whitespaces.pointer;
                            return 1_i32;
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    yaml_free(leading_break.start as *mut libc::c_void);
    leading_break.end = ptr::null_mut::<yaml_char_t>();
    leading_break.pointer = leading_break.end;
    leading_break.start = leading_break.pointer;
    yaml_free(trailing_breaks.start as *mut libc::c_void);
    trailing_breaks.end = ptr::null_mut::<yaml_char_t>();
    trailing_breaks.pointer = trailing_breaks.end;
    trailing_breaks.start = trailing_breaks.pointer;
    yaml_free(whitespaces.start as *mut libc::c_void);
    whitespaces.end = ptr::null_mut::<yaml_char_t>();
    whitespaces.pointer = whitespaces.end;
    whitespaces.start = whitespaces.pointer;
    0_i32
}

unsafe fn yaml_parser_scan_plain_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> libc::c_int {
    let current_block: u64;
    let start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut string = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_break = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut trailing_breaks = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut whitespaces = yaml_string_t {
        start: ptr::null_mut::<yaml_char_t>(),
        end: ptr::null_mut::<yaml_char_t>(),
        pointer: ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_blanks: libc::c_int = 0_i32;
    let indent: libc::c_int = (*parser).indent + 1_i32;
    string.start = yaml_malloc(16_u64) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.wrapping_offset(16_isize);
        memset(string.start as *mut libc::c_void, 0_i32, 16_u64);
        1_i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0_i32
    } == 0)
    {
        leading_break.start = yaml_malloc(16_u64) as *mut yaml_char_t;
        if !(if !leading_break.start.is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = leading_break.start.wrapping_offset(16_isize);
            memset(leading_break.start as *mut libc::c_void, 0_i32, 16_u64);
            1_i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0_i32
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16_u64) as *mut yaml_char_t;
            if !(if !trailing_breaks.start.is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = trailing_breaks.start.wrapping_offset(16_isize);
                memset(trailing_breaks.start as *mut libc::c_void, 0_i32, 16_u64);
                1_i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0_i32
            } == 0)
            {
                whitespaces.start = yaml_malloc(16_u64) as *mut yaml_char_t;
                if !(if !whitespaces.start.is_null() {
                    whitespaces.pointer = whitespaces.start;
                    whitespaces.end = whitespaces.start.wrapping_offset(16_isize);
                    memset(whitespaces.start as *mut libc::c_void, 0_i32, 16_u64);
                    1_i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0_i32
                } == 0)
                {
                    end_mark = (*parser).mark;
                    start_mark = end_mark;
                    's_57: loop {
                        if if (*parser).unread >= 4_u64 {
                            1_i32
                        } else {
                            yaml_parser_update_buffer(parser, 4_u64)
                        } == 0
                        {
                            current_block = 16642808987012640029;
                            break;
                        }
                        if (*parser).mark.column == 0_u64
                            && (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int)
                            && (*((*parser).buffer.pointer).wrapping_offset(3_isize) as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                                || (*((*parser).buffer.pointer).wrapping_offset(3_isize)
                                    as libc::c_int
                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 1_i32) as isize)
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 1_i32) as isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 2_i32) as isize)
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 1_i32) as isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .wrapping_offset((3_i32 + 2_i32) as isize)
                                            as libc::c_int
                                            == -87i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(3_isize)
                                        as libc::c_int
                                        == '\0' as i32 as yaml_char_t as libc::c_int))
                        {
                            current_block = 6281126495347172768;
                            break;
                        }
                        if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == '#' as i32 as yaml_char_t as libc::c_int
                        {
                            current_block = 6281126495347172768;
                            break;
                        }
                        while !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\0' as i32 as yaml_char_t as libc::c_int))
                        {
                            if (*parser).flow_level != 0
                                && *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == ':' as i32 as yaml_char_t as libc::c_int
                                && (*((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == ',' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '?' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '[' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == ']' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '{' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '}' as i32 as yaml_char_t as libc::c_int)
                            {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while scanning a plain scalar\0" as *const u8
                                        as *const libc::c_char,
                                    start_mark,
                                    b"found unexpected ':'\0" as *const u8 as *const libc::c_char,
                                );
                                current_block = 16642808987012640029;
                                break 's_57;
                            } else {
                                if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == ':' as i32 as yaml_char_t as libc::c_int
                                    && (*((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == ' ' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == '\t' as i32 as yaml_char_t as libc::c_int
                                        || (*((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 1_i32) as isize)
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 1_i32) as isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 2_i32) as isize)
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 1_i32) as isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset((1_i32 + 2_i32) as isize)
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == '\0' as i32 as yaml_char_t as libc::c_int))
                                    || (*parser).flow_level != 0
                                        && (*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == ',' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '[' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == ']' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '{' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '}' as i32 as yaml_char_t as libc::c_int)
                                {
                                    break;
                                }
                                if leading_blanks != 0 || whitespaces.start != whitespaces.pointer {
                                    if leading_blanks != 0 {
                                        if *leading_break.start.wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\n' as i32
                                        {
                                            if *trailing_breaks.start.wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '\0' as i32
                                            {
                                                if if string.pointer.wrapping_offset(5_isize)
                                                    < string.end
                                                    || yaml_string_extend(
                                                        addr_of_mut!(string.start),
                                                        addr_of_mut!(string.pointer),
                                                        addr_of_mut!(string.end),
                                                    ) != 0
                                                {
                                                    1_i32
                                                } else {
                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                    0_i32
                                                } == 0
                                                {
                                                    current_block = 16642808987012640029;
                                                    break 's_57;
                                                }
                                                let fresh717 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh717 = ' ' as i32 as yaml_char_t;
                                            } else {
                                                if if yaml_string_join(
                                                    addr_of_mut!(string.start),
                                                    addr_of_mut!(string.pointer),
                                                    addr_of_mut!(string.end),
                                                    addr_of_mut!(trailing_breaks.start),
                                                    addr_of_mut!(trailing_breaks.pointer),
                                                    addr_of_mut!(trailing_breaks.end),
                                                ) != 0
                                                {
                                                    trailing_breaks.pointer = trailing_breaks.start;
                                                    1_i32
                                                } else {
                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                    0_i32
                                                } == 0
                                                {
                                                    current_block = 16642808987012640029;
                                                    break 's_57;
                                                }
                                                trailing_breaks.pointer = trailing_breaks.start;
                                                memset(
                                                    trailing_breaks.start as *mut libc::c_void,
                                                    0_i32,
                                                    (trailing_breaks.end)
                                                        .c_offset_from(trailing_breaks.start)
                                                        as libc::c_long
                                                        as libc::c_ulong,
                                                );
                                            }
                                            leading_break.pointer = leading_break.start;
                                            memset(
                                                leading_break.start as *mut libc::c_void,
                                                0_i32,
                                                (leading_break.end)
                                                    .c_offset_from(leading_break.start)
                                                    as libc::c_long
                                                    as libc::c_ulong,
                                            );
                                        } else {
                                            if if yaml_string_join(
                                                addr_of_mut!(string.start),
                                                addr_of_mut!(string.pointer),
                                                addr_of_mut!(string.end),
                                                addr_of_mut!(leading_break.start),
                                                addr_of_mut!(leading_break.pointer),
                                                addr_of_mut!(leading_break.end),
                                            ) != 0
                                            {
                                                leading_break.pointer = leading_break.start;
                                                1_i32
                                            } else {
                                                (*parser).error = YAML_MEMORY_ERROR;
                                                0_i32
                                            } == 0
                                            {
                                                current_block = 16642808987012640029;
                                                break 's_57;
                                            }
                                            if if yaml_string_join(
                                                addr_of_mut!(string.start),
                                                addr_of_mut!(string.pointer),
                                                addr_of_mut!(string.end),
                                                addr_of_mut!(trailing_breaks.start),
                                                addr_of_mut!(trailing_breaks.pointer),
                                                addr_of_mut!(trailing_breaks.end),
                                            ) != 0
                                            {
                                                trailing_breaks.pointer = trailing_breaks.start;
                                                1_i32
                                            } else {
                                                (*parser).error = YAML_MEMORY_ERROR;
                                                0_i32
                                            } == 0
                                            {
                                                current_block = 16642808987012640029;
                                                break 's_57;
                                            }
                                            leading_break.pointer = leading_break.start;
                                            memset(
                                                leading_break.start as *mut libc::c_void,
                                                0_i32,
                                                (leading_break.end)
                                                    .c_offset_from(leading_break.start)
                                                    as libc::c_long
                                                    as libc::c_ulong,
                                            );
                                            trailing_breaks.pointer = trailing_breaks.start;
                                            memset(
                                                trailing_breaks.start as *mut libc::c_void,
                                                0_i32,
                                                (trailing_breaks.end)
                                                    .c_offset_from(trailing_breaks.start)
                                                    as libc::c_long
                                                    as libc::c_ulong,
                                            );
                                        }
                                        leading_blanks = 0_i32;
                                    } else {
                                        if if yaml_string_join(
                                            addr_of_mut!(string.start),
                                            addr_of_mut!(string.pointer),
                                            addr_of_mut!(string.end),
                                            addr_of_mut!(whitespaces.start),
                                            addr_of_mut!(whitespaces.pointer),
                                            addr_of_mut!(whitespaces.end),
                                        ) != 0
                                        {
                                            whitespaces.pointer = whitespaces.start;
                                            1_i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0_i32
                                        } == 0
                                        {
                                            current_block = 16642808987012640029;
                                            break 's_57;
                                        }
                                        whitespaces.pointer = whitespaces.start;
                                        memset(
                                            whitespaces.start as *mut libc::c_void,
                                            0_i32,
                                            whitespaces.end.c_offset_from(whitespaces.start)
                                                as libc::c_long
                                                as libc::c_ulong,
                                        );
                                    }
                                }
                                if if if string.pointer.wrapping_offset(5_isize) < string.end
                                    || yaml_string_extend(
                                        addr_of_mut!(string.start),
                                        addr_of_mut!(string.pointer),
                                        addr_of_mut!(string.end),
                                    ) != 0
                                {
                                    1_i32
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0_i32
                                } != 0
                                {
                                    if *(*parser).buffer.pointer as libc::c_int & 0x80_i32 == 0_i32
                                    {
                                        let fresh718 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh719 = *fresh718;
                                        *fresh718 = (*fresh718).wrapping_offset(1);
                                        let fresh720 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh720 = *fresh719;
                                    } else if *(*parser).buffer.pointer as libc::c_int & 0xe0_i32
                                        == 0xc0_i32
                                    {
                                        let fresh721 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh722 = *fresh721;
                                        *fresh721 = (*fresh721).wrapping_offset(1);
                                        let fresh723 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh723 = *fresh722;
                                        let fresh724 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh725 = *fresh724;
                                        *fresh724 = (*fresh724).wrapping_offset(1);
                                        let fresh726 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh726 = *fresh725;
                                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf0_i32
                                        == 0xe0_i32
                                    {
                                        let fresh727 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh728 = *fresh727;
                                        *fresh727 = (*fresh727).wrapping_offset(1);
                                        let fresh729 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh729 = *fresh728;
                                        let fresh730 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh731 = *fresh730;
                                        *fresh730 = (*fresh730).wrapping_offset(1);
                                        let fresh732 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh732 = *fresh731;
                                        let fresh733 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh734 = *fresh733;
                                        *fresh733 = (*fresh733).wrapping_offset(1);
                                        let fresh735 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh735 = *fresh734;
                                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf8_i32
                                        == 0xf0_i32
                                    {
                                        let fresh736 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh737 = *fresh736;
                                        *fresh736 = (*fresh736).wrapping_offset(1);
                                        let fresh738 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh738 = *fresh737;
                                        let fresh739 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh740 = *fresh739;
                                        *fresh739 = (*fresh739).wrapping_offset(1);
                                        let fresh741 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh741 = *fresh740;
                                        let fresh742 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh743 = *fresh742;
                                        *fresh742 = (*fresh742).wrapping_offset(1);
                                        let fresh744 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh744 = *fresh743;
                                        let fresh745 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh746 = *fresh745;
                                        *fresh745 = (*fresh745).wrapping_offset(1);
                                        let fresh747 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh747 = *fresh746;
                                    };
                                    let fresh748 = addr_of_mut!((*parser).mark.index);
                                    *fresh748 = (*fresh748).wrapping_add(1);
                                    let fresh749 = addr_of_mut!((*parser).mark.column);
                                    *fresh749 = (*fresh749).wrapping_add(1);
                                    let fresh750 = addr_of_mut!((*parser).unread);
                                    *fresh750 = (*fresh750).wrapping_sub(1);
                                    1_i32
                                } else {
                                    0_i32
                                } == 0
                                {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                                end_mark = (*parser).mark;
                                if if (*parser).unread >= 2_u64 {
                                    1_i32
                                } else {
                                    yaml_parser_update_buffer(parser, 2_u64)
                                } == 0
                                {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                            }
                        }
                        if !(*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).wrapping_offset(0_isize)
                                as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int))
                        {
                            current_block = 6281126495347172768;
                            break;
                        }
                        if if (*parser).unread >= 1_u64 {
                            1_i32
                        } else {
                            yaml_parser_update_buffer(parser, 1_u64)
                        } == 0
                        {
                            current_block = 16642808987012640029;
                            break;
                        }
                        while *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int)
                        {
                            if *((*parser).buffer.pointer).wrapping_offset(0_isize) as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                            {
                                if leading_blanks != 0
                                    && ((*parser).mark.column as libc::c_int) < indent
                                    && *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\t' as i32 as yaml_char_t as libc::c_int
                                {
                                    yaml_parser_set_scanner_error(
                                        parser,
                                        b"while scanning a plain scalar\0" as *const u8
                                            as *const libc::c_char,
                                        start_mark,
                                        b"found a tab character that violates indentation\0"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                } else if leading_blanks == 0 {
                                    if if if whitespaces.pointer.wrapping_offset(5_isize)
                                        < whitespaces.end
                                        || yaml_string_extend(
                                            addr_of_mut!(whitespaces.start),
                                            addr_of_mut!(whitespaces.pointer),
                                            addr_of_mut!(whitespaces.end),
                                        ) != 0
                                    {
                                        1_i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0_i32
                                    } != 0
                                    {
                                        if *(*parser).buffer.pointer as libc::c_int & 0x80_i32
                                            == 0_i32
                                        {
                                            let fresh751 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh752 = *fresh751;
                                            *fresh751 = (*fresh751).wrapping_offset(1);
                                            let fresh753 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh753 = *fresh752;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xe0_i32
                                            == 0xc0_i32
                                        {
                                            let fresh754 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh755 = *fresh754;
                                            *fresh754 = (*fresh754).wrapping_offset(1);
                                            let fresh756 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh756 = *fresh755;
                                            let fresh757 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh758 = *fresh757;
                                            *fresh757 = (*fresh757).wrapping_offset(1);
                                            let fresh759 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh759 = *fresh758;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xf0_i32
                                            == 0xe0_i32
                                        {
                                            let fresh760 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh761 = *fresh760;
                                            *fresh760 = (*fresh760).wrapping_offset(1);
                                            let fresh762 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh762 = *fresh761;
                                            let fresh763 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh764 = *fresh763;
                                            *fresh763 = (*fresh763).wrapping_offset(1);
                                            let fresh765 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh765 = *fresh764;
                                            let fresh766 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh767 = *fresh766;
                                            *fresh766 = (*fresh766).wrapping_offset(1);
                                            let fresh768 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh768 = *fresh767;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xf8_i32
                                            == 0xf0_i32
                                        {
                                            let fresh769 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh770 = *fresh769;
                                            *fresh769 = (*fresh769).wrapping_offset(1);
                                            let fresh771 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh771 = *fresh770;
                                            let fresh772 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh773 = *fresh772;
                                            *fresh772 = (*fresh772).wrapping_offset(1);
                                            let fresh774 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh774 = *fresh773;
                                            let fresh775 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh776 = *fresh775;
                                            *fresh775 = (*fresh775).wrapping_offset(1);
                                            let fresh777 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh777 = *fresh776;
                                            let fresh778 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh779 = *fresh778;
                                            *fresh778 = (*fresh778).wrapping_offset(1);
                                            let fresh780 = whitespaces.pointer;
                                            whitespaces.pointer =
                                                whitespaces.pointer.wrapping_offset(1);
                                            *fresh780 = *fresh779;
                                        };
                                        let fresh781 = addr_of_mut!((*parser).mark.index);
                                        *fresh781 = (*fresh781).wrapping_add(1);
                                        let fresh782 = addr_of_mut!((*parser).mark.column);
                                        *fresh782 = (*fresh782).wrapping_add(1);
                                        let fresh783 = addr_of_mut!((*parser).unread);
                                        *fresh783 = (*fresh783).wrapping_sub(1);
                                        1_i32
                                    } else {
                                        0_i32
                                    } == 0
                                    {
                                        current_block = 16642808987012640029;
                                        break 's_57;
                                    }
                                } else {
                                    SKIP!(parser);
                                }
                            } else {
                                if if (*parser).unread >= 2_u64 {
                                    1_i32
                                } else {
                                    yaml_parser_update_buffer(parser, 2_u64)
                                } == 0
                                {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                                if leading_blanks == 0 {
                                    whitespaces.pointer = whitespaces.start;
                                    memset(
                                        whitespaces.start as *mut libc::c_void,
                                        0_i32,
                                        whitespaces.end.c_offset_from(whitespaces.start)
                                            as libc::c_long
                                            as libc::c_ulong,
                                    );
                                    if if if leading_break.pointer.wrapping_offset(5_isize)
                                        < leading_break.end
                                        || yaml_string_extend(
                                            addr_of_mut!(leading_break.start),
                                            addr_of_mut!(leading_break.pointer),
                                            addr_of_mut!(leading_break.end),
                                        ) != 0
                                    {
                                        1_i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0_i32
                                    } != 0
                                    {
                                        if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh788 = leading_break.pointer;
                                            leading_break.pointer =
                                                leading_break.pointer.wrapping_offset(1);
                                            *fresh788 = '\n' as i32 as yaml_char_t;
                                            let fresh789 = addr_of_mut!((*parser).buffer.pointer);
                                            *fresh789 = (*fresh789).wrapping_offset(2_isize);
                                            let fresh790 = addr_of_mut!((*parser).mark.index);
                                            *fresh790 = (*fresh790 as libc::c_ulong)
                                                .wrapping_add(2_u64)
                                                as size_t
                                                as size_t;
                                            (*parser).mark.column = 0_u64;
                                            let fresh791 = addr_of_mut!((*parser).mark.line);
                                            *fresh791 = (*fresh791).wrapping_add(1);
                                            let fresh792 = addr_of_mut!((*parser).unread);
                                            *fresh792 = (*fresh792 as libc::c_ulong)
                                                .wrapping_sub(2_u64)
                                                as size_t
                                                as size_t;
                                        } else if *((*parser).buffer.pointer)
                                            .wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh793 = leading_break.pointer;
                                            leading_break.pointer =
                                                leading_break.pointer.wrapping_offset(1);
                                            *fresh793 = '\n' as i32 as yaml_char_t;
                                            let fresh794 = addr_of_mut!((*parser).buffer.pointer);
                                            *fresh794 = (*fresh794).wrapping_offset(1);
                                            let fresh795 = addr_of_mut!((*parser).mark.index);
                                            *fresh795 = (*fresh795).wrapping_add(1);
                                            (*parser).mark.column = 0_u64;
                                            let fresh796 = addr_of_mut!((*parser).mark.line);
                                            *fresh796 = (*fresh796).wrapping_add(1);
                                            let fresh797 = addr_of_mut!((*parser).unread);
                                            *fresh797 = (*fresh797).wrapping_sub(1);
                                        } else if *((*parser).buffer.pointer)
                                            .wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -62i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -123i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh798 = leading_break.pointer;
                                            leading_break.pointer =
                                                leading_break.pointer.wrapping_offset(1);
                                            *fresh798 = '\n' as i32 as yaml_char_t;
                                            let fresh799 = addr_of_mut!((*parser).buffer.pointer);
                                            *fresh799 = (*fresh799).wrapping_offset(2_isize);
                                            let fresh800 = addr_of_mut!((*parser).mark.index);
                                            *fresh800 = (*fresh800).wrapping_add(1);
                                            (*parser).mark.column = 0_u64;
                                            let fresh801 = addr_of_mut!((*parser).mark.line);
                                            *fresh801 = (*fresh801).wrapping_add(1);
                                            let fresh802 = addr_of_mut!((*parser).unread);
                                            *fresh802 = (*fresh802).wrapping_sub(1);
                                        } else if *((*parser).buffer.pointer)
                                            .wrapping_offset(0_isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && (*((*parser).buffer.pointer).wrapping_offset(2_isize)
                                                as libc::c_int
                                                == -88i32 as yaml_char_t as libc::c_int
                                                || *((*parser).buffer.pointer)
                                                    .wrapping_offset(2_isize)
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int)
                                        {
                                            let fresh803 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh804 = *fresh803;
                                            *fresh803 = (*fresh803).wrapping_offset(1);
                                            let fresh805 = leading_break.pointer;
                                            leading_break.pointer =
                                                leading_break.pointer.wrapping_offset(1);
                                            *fresh805 = *fresh804;
                                            let fresh806 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh807 = *fresh806;
                                            *fresh806 = (*fresh806).wrapping_offset(1);
                                            let fresh808 = leading_break.pointer;
                                            leading_break.pointer =
                                                leading_break.pointer.wrapping_offset(1);
                                            *fresh808 = *fresh807;
                                            let fresh809 = addr_of_mut!((*parser).buffer.pointer);
                                            let fresh810 = *fresh809;
                                            *fresh809 = (*fresh809).wrapping_offset(1);
                                            let fresh811 = leading_break.pointer;
                                            leading_break.pointer =
                                                leading_break.pointer.wrapping_offset(1);
                                            *fresh811 = *fresh810;
                                            let fresh812 = addr_of_mut!((*parser).mark.index);
                                            *fresh812 = (*fresh812).wrapping_add(1);
                                            (*parser).mark.column = 0_u64;
                                            let fresh813 = addr_of_mut!((*parser).mark.line);
                                            *fresh813 = (*fresh813).wrapping_add(1);
                                            let fresh814 = addr_of_mut!((*parser).unread);
                                            *fresh814 = (*fresh814).wrapping_sub(1);
                                        };
                                        1_i32
                                    } else {
                                        0_i32
                                    } == 0
                                    {
                                        current_block = 16642808987012640029;
                                        break 's_57;
                                    }
                                    leading_blanks = 1_i32;
                                } else if if if trailing_breaks.pointer.wrapping_offset(5_isize)
                                    < trailing_breaks.end
                                    || yaml_string_extend(
                                        addr_of_mut!(trailing_breaks.start),
                                        addr_of_mut!(trailing_breaks.pointer),
                                        addr_of_mut!(trailing_breaks.end),
                                    ) != 0
                                {
                                    1_i32
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0_i32
                                } != 0
                                {
                                    if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == '\n' as i32 as yaml_char_t as libc::c_int
                                    {
                                        let fresh815 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            trailing_breaks.pointer.wrapping_offset(1);
                                        *fresh815 = '\n' as i32 as yaml_char_t;
                                        let fresh816 = addr_of_mut!((*parser).buffer.pointer);
                                        *fresh816 = (*fresh816).wrapping_offset(2_isize);
                                        let fresh817 = addr_of_mut!((*parser).mark.index);
                                        *fresh817 = (*fresh817 as libc::c_ulong).wrapping_add(2_u64)
                                            as size_t
                                            as size_t;
                                        (*parser).mark.column = 0_u64;
                                        let fresh818 = addr_of_mut!((*parser).mark.line);
                                        *fresh818 = (*fresh818).wrapping_add(1);
                                        let fresh819 = addr_of_mut!((*parser).unread);
                                        *fresh819 = (*fresh819 as libc::c_ulong).wrapping_sub(2_u64)
                                            as size_t
                                            as size_t;
                                    } else if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                            as libc::c_int
                                            == '\n' as i32 as yaml_char_t as libc::c_int
                                    {
                                        let fresh820 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            trailing_breaks.pointer.wrapping_offset(1);
                                        *fresh820 = '\n' as i32 as yaml_char_t;
                                        let fresh821 = addr_of_mut!((*parser).buffer.pointer);
                                        *fresh821 = (*fresh821).wrapping_offset(1);
                                        let fresh822 = addr_of_mut!((*parser).mark.index);
                                        *fresh822 = (*fresh822).wrapping_add(1);
                                        (*parser).mark.column = 0_u64;
                                        let fresh823 = addr_of_mut!((*parser).mark.line);
                                        *fresh823 = (*fresh823).wrapping_add(1);
                                        let fresh824 = addr_of_mut!((*parser).unread);
                                        *fresh824 = (*fresh824).wrapping_sub(1);
                                    } else if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    {
                                        let fresh825 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            trailing_breaks.pointer.wrapping_offset(1);
                                        *fresh825 = '\n' as i32 as yaml_char_t;
                                        let fresh826 = addr_of_mut!((*parser).buffer.pointer);
                                        *fresh826 = (*fresh826).wrapping_offset(2_isize);
                                        let fresh827 = addr_of_mut!((*parser).mark.index);
                                        *fresh827 = (*fresh827).wrapping_add(1);
                                        (*parser).mark.column = 0_u64;
                                        let fresh828 = addr_of_mut!((*parser).mark.line);
                                        *fresh828 = (*fresh828).wrapping_add(1);
                                        let fresh829 = addr_of_mut!((*parser).unread);
                                        *fresh829 = (*fresh829).wrapping_sub(1);
                                    } else if *((*parser).buffer.pointer).wrapping_offset(0_isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && (*((*parser).buffer.pointer).wrapping_offset(2_isize)
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                                as libc::c_int
                                                == -87i32 as yaml_char_t as libc::c_int)
                                    {
                                        let fresh830 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh831 = *fresh830;
                                        *fresh830 = (*fresh830).wrapping_offset(1);
                                        let fresh832 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            trailing_breaks.pointer.wrapping_offset(1);
                                        *fresh832 = *fresh831;
                                        let fresh833 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh834 = *fresh833;
                                        *fresh833 = (*fresh833).wrapping_offset(1);
                                        let fresh835 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            trailing_breaks.pointer.wrapping_offset(1);
                                        *fresh835 = *fresh834;
                                        let fresh836 = addr_of_mut!((*parser).buffer.pointer);
                                        let fresh837 = *fresh836;
                                        *fresh836 = (*fresh836).wrapping_offset(1);
                                        let fresh838 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            trailing_breaks.pointer.wrapping_offset(1);
                                        *fresh838 = *fresh837;
                                        let fresh839 = addr_of_mut!((*parser).mark.index);
                                        *fresh839 = (*fresh839).wrapping_add(1);
                                        (*parser).mark.column = 0_u64;
                                        let fresh840 = addr_of_mut!((*parser).mark.line);
                                        *fresh840 = (*fresh840).wrapping_add(1);
                                        let fresh841 = addr_of_mut!((*parser).unread);
                                        *fresh841 = (*fresh841).wrapping_sub(1);
                                    };
                                    1_i32
                                } else {
                                    0_i32
                                } == 0
                                {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                            }
                            if if (*parser).unread >= 1_u64 {
                                1_i32
                            } else {
                                yaml_parser_update_buffer(parser, 1_u64)
                            } == 0
                            {
                                current_block = 16642808987012640029;
                                break 's_57;
                            }
                        }
                        if (*parser).flow_level == 0
                            && ((*parser).mark.column as libc::c_int) < indent
                        {
                            current_block = 6281126495347172768;
                            break;
                        }
                    }
                    match current_block {
                        16642808987012640029 => {}
                        _ => {
                            memset(
                                token as *mut libc::c_void,
                                0_i32,
                                size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_ = YAML_SCALAR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh842 = addr_of_mut!((*token).data.scalar.value);
                            *fresh842 = string.start;
                            (*token).data.scalar.length = string.pointer.c_offset_from(string.start)
                                as libc::c_long
                                as size_t;
                            (*token).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
                            if leading_blanks != 0 {
                                (*parser).simple_key_allowed = 1_i32;
                            }
                            yaml_free(leading_break.start as *mut libc::c_void);
                            leading_break.end = ptr::null_mut::<yaml_char_t>();
                            leading_break.pointer = leading_break.end;
                            leading_break.start = leading_break.pointer;
                            yaml_free(trailing_breaks.start as *mut libc::c_void);
                            trailing_breaks.end = ptr::null_mut::<yaml_char_t>();
                            trailing_breaks.pointer = trailing_breaks.end;
                            trailing_breaks.start = trailing_breaks.pointer;
                            yaml_free(whitespaces.start as *mut libc::c_void);
                            whitespaces.end = ptr::null_mut::<yaml_char_t>();
                            whitespaces.pointer = whitespaces.end;
                            whitespaces.start = whitespaces.pointer;
                            return 1_i32;
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    yaml_free(leading_break.start as *mut libc::c_void);
    leading_break.end = ptr::null_mut::<yaml_char_t>();
    leading_break.pointer = leading_break.end;
    leading_break.start = leading_break.pointer;
    yaml_free(trailing_breaks.start as *mut libc::c_void);
    trailing_breaks.end = ptr::null_mut::<yaml_char_t>();
    trailing_breaks.pointer = trailing_breaks.end;
    trailing_breaks.start = trailing_breaks.pointer;
    yaml_free(whitespaces.start as *mut libc::c_void);
    whitespaces.end = ptr::null_mut::<yaml_char_t>();
    whitespaces.pointer = whitespaces.end;
    whitespaces.start = whitespaces.pointer;
    0_i32
}
