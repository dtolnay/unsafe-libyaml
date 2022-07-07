use crate::api::{
    yaml_free, yaml_malloc, yaml_queue_extend, yaml_stack_extend, yaml_string_extend,
    yaml_string_join, yaml_token_delete,
};
use crate::externs::*;
use crate::libc;
use crate::reader::yaml_parser_update_buffer;
use crate::yaml::*;
use crate::PointerExt;
use std::mem;
use std::ptr;
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_scan(
    mut parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> libc::c_int {
    __assert!(!parser.is_null());
    __assert!(!token.is_null());
    memset(
        token as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    if (*parser).stream_end_produced != 0 || (*parser).error as libc::c_uint != 0 {
        return 1 as libc::c_int;
    }
    if (*parser).token_available == 0 {
        if yaml_parser_fetch_more_tokens(parser) == 0 {
            return 0 as libc::c_int;
        }
    }
    let fresh0 = &mut (*parser).tokens.head;
    let fresh1 = *fresh0;
    *fresh0 = (*fresh0).c_offset(1);
    *token = *fresh1;
    (*parser).token_available = 0 as libc::c_int;
    let fresh2 = &mut (*parser).tokens_parsed;
    *fresh2 = (*fresh2).wrapping_add(1);
    if (*token).type_0 as libc::c_uint == YAML_STREAM_END_TOKEN as libc::c_int as libc::c_uint {
        (*parser).stream_end_produced = 1 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_set_scanner_error(
    mut parser: *mut yaml_parser_t,
    context: *const libc::c_char,
    context_mark: yaml_mark_t,
    problem: *const libc::c_char,
) -> libc::c_int {
    (*parser).error = YAML_SCANNER_ERROR;
    let fresh3 = &mut (*parser).context;
    *fresh3 = context;
    (*parser).context_mark = context_mark;
    let fresh4 = &mut (*parser).problem;
    *fresh4 = problem;
    (*parser).problem_mark = (*parser).mark;
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_fetch_more_tokens(
    mut parser: *mut yaml_parser_t,
) -> libc::c_int {
    let mut need_more_tokens: libc::c_int;
    loop {
        need_more_tokens = 0 as libc::c_int;
        if (*parser).tokens.head == (*parser).tokens.tail {
            need_more_tokens = 1 as libc::c_int;
        } else {
            let mut simple_key: *mut yaml_simple_key_t;
            if yaml_parser_stale_simple_keys(parser) == 0 {
                return 0 as libc::c_int;
            }
            simple_key = (*parser).simple_keys.start;
            while simple_key != (*parser).simple_keys.top {
                if (*simple_key).possible != 0
                    && (*simple_key).token_number == (*parser).tokens_parsed
                {
                    need_more_tokens = 1 as libc::c_int;
                    break;
                } else {
                    simple_key = simple_key.c_offset(1);
                }
            }
        }
        if need_more_tokens == 0 {
            break;
        }
        if yaml_parser_fetch_next_token(parser) == 0 {
            return 0 as libc::c_int;
        }
    }
    (*parser).token_available = 1 as libc::c_int;
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_next_token(parser: *mut yaml_parser_t) -> libc::c_int {
    if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
        1 as libc::c_int
    } else {
        yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
    } == 0
    {
        return 0 as libc::c_int;
    }
    if (*parser).stream_start_produced == 0 {
        return yaml_parser_fetch_stream_start(parser);
    }
    if yaml_parser_scan_to_next_token(parser) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_parser_stale_simple_keys(parser) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_parser_unroll_indent(parser, (*parser).mark.column as ptrdiff_t) == 0 {
        return 0 as libc::c_int;
    }
    if if (*parser).unread >= 4 as libc::c_int as libc::c_ulong {
        1 as libc::c_int
    } else {
        yaml_parser_update_buffer(parser, 4 as libc::c_int as size_t)
    } == 0
    {
        return 0 as libc::c_int;
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '\0' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_stream_end(parser);
    }
    if (*parser).mark.column == 0 as libc::c_int as libc::c_ulong
        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '%' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_directive(parser);
    }
    if (*parser).mark.column == 0 as libc::c_int as libc::c_ulong
        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && (*((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
            || (*((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int))
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_START_TOKEN);
    }
    if (*parser).mark.column == 0 as libc::c_int as libc::c_ulong
        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
        && (*((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
            || (*((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((3 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int))
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_END_TOKEN);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '[' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_SEQUENCE_START_TOKEN);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '{' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_MAPPING_START_TOKEN);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == ']' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_SEQUENCE_END_TOKEN);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '}' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_MAPPING_END_TOKEN);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == ',' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_entry(parser);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '-' as i32 as yaml_char_t as libc::c_int
        && (*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
            || (*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((1 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((1 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int))
    {
        return yaml_parser_fetch_block_entry(parser);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '?' as i32 as yaml_char_t as libc::c_int
        && ((*parser).flow_level != 0
            || (*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
                || (*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                    as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == '\0' as i32 as yaml_char_t as libc::c_int)))
    {
        return yaml_parser_fetch_key(parser);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == ':' as i32 as yaml_char_t as libc::c_int
        && ((*parser).flow_level != 0
            || (*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
                || (*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                    as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == '\0' as i32 as yaml_char_t as libc::c_int)))
    {
        return yaml_parser_fetch_value(parser);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '*' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_anchor(parser, YAML_ALIAS_TOKEN);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '&' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_anchor(parser, YAML_ANCHOR_TOKEN);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '!' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_tag(parser);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '|' as i32 as yaml_char_t as libc::c_int
        && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 1 as libc::c_int);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '>' as i32 as yaml_char_t as libc::c_int
        && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 0 as libc::c_int);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '\'' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_scalar(parser, 1 as libc::c_int);
    }
    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '"' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_flow_scalar(parser, 0 as libc::c_int);
    }
    if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == ' ' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\t' as i32 as yaml_char_t as libc::c_int
        || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32 as yaml_char_t as libc::c_int)
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '?' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == ':' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == ',' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '[' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == ']' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '{' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '}' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '#' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '&' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '*' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '!' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '|' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '>' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\'' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '"' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '%' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '@' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '`' as i32 as yaml_char_t as libc::c_int)
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
            && !(*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int)
        || (*parser).flow_level == 0
            && (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '?' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == ':' as i32 as yaml_char_t as libc::c_int)
            && !(*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
                || (*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == '\r' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == '\n' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == -62i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -123i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == -88i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        == -30i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int
                            == -128i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer)
                            .c_offset((1 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int
                            == -87i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
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
unsafe extern "C" fn yaml_parser_stale_simple_keys(parser: *mut yaml_parser_t) -> libc::c_int {
    let mut simple_key: *mut yaml_simple_key_t;
    simple_key = (*parser).simple_keys.start;
    while simple_key != (*parser).simple_keys.top {
        if (*simple_key).possible != 0
            && ((*simple_key).mark.line < (*parser).mark.line
                || ((*simple_key).mark.index).wrapping_add(1024 as libc::c_int as libc::c_ulong)
                    < (*parser).mark.index)
        {
            if (*simple_key).required != 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a simple key\0" as *const u8 as *const libc::c_char,
                    (*simple_key).mark,
                    b"could not find expected ':'\0" as *const u8 as *const libc::c_char,
                );
            }
            (*simple_key).possible = 0 as libc::c_int;
        }
        simple_key = simple_key.c_offset(1);
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_save_simple_key(parser: *mut yaml_parser_t) -> libc::c_int {
    let required: libc::c_int = ((*parser).flow_level == 0
        && (*parser).indent as libc::c_long == (*parser).mark.column as ptrdiff_t)
        as libc::c_int;
    if (*parser).simple_key_allowed != 0 {
        let mut simple_key: yaml_simple_key_t = yaml_simple_key_t {
            possible: 0,
            required: 0,
            token_number: 0,
            mark: yaml_mark_t {
                index: 0,
                line: 0,
                column: 0,
            },
        };
        simple_key.possible = 1 as libc::c_int;
        simple_key.required = required;
        simple_key.token_number = ((*parser).tokens_parsed)
            .wrapping_add(((*parser).tokens.tail).c_offset_from((*parser).tokens.head)
                as libc::c_long as libc::c_ulong);
        simple_key.mark = (*parser).mark;
        if yaml_parser_remove_simple_key(parser) == 0 {
            return 0 as libc::c_int;
        }
        *((*parser).simple_keys.top).c_offset(-(1 as libc::c_int as isize)) = simple_key;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_remove_simple_key(parser: *mut yaml_parser_t) -> libc::c_int {
    let mut simple_key: *mut yaml_simple_key_t =
        ((*parser).simple_keys.top).c_offset(-(1 as libc::c_int as isize));
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
    (*simple_key).possible = 0 as libc::c_int;
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_increase_flow_level(
    mut parser: *mut yaml_parser_t,
) -> libc::c_int {
    let empty_simple_key: yaml_simple_key_t = {
        yaml_simple_key_s {
            possible: 0 as libc::c_int,
            required: 0 as libc::c_int,
            token_number: 0 as libc::c_int as size_t,
            mark: {
                yaml_mark_s {
                    index: 0 as libc::c_int as size_t,
                    line: 0 as libc::c_int as size_t,
                    column: 0 as libc::c_int as size_t,
                }
            },
        }
    };
    if if (*parser).simple_keys.top != (*parser).simple_keys.end
        || yaml_stack_extend(
            ptr::addr_of_mut!((*parser).simple_keys.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).simple_keys.top) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).simple_keys.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh5 = &mut (*parser).simple_keys.top;
        let fresh6 = *fresh5;
        *fresh5 = (*fresh5).c_offset(1);
        *fresh6 = empty_simple_key;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    if (*parser).flow_level == 2147483647 as libc::c_int {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0 as libc::c_int;
    }
    let fresh7 = &mut (*parser).flow_level;
    *fresh7 += 1;
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_decrease_flow_level(parser: *mut yaml_parser_t) -> libc::c_int {
    if (*parser).flow_level != 0 {
        let fresh8 = &mut (*parser).flow_level;
        *fresh8 -= 1;
        let fresh9 = &mut (*parser).simple_keys.top;
        *fresh9 = (*fresh9).c_offset(-1);
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_roll_indent(
    mut parser: *mut yaml_parser_t,
    column: ptrdiff_t,
    number: ptrdiff_t,
    type_0: yaml_token_type_t,
    mark: yaml_mark_t,
) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if (*parser).flow_level != 0 {
        return 1 as libc::c_int;
    }
    if ((*parser).indent as libc::c_long) < column {
        if if (*parser).indents.top != (*parser).indents.end
            || yaml_stack_extend(
                ptr::addr_of_mut!((*parser).indents.start) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).indents.top) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).indents.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh10 = &mut (*parser).indents.top;
            let fresh11 = *fresh10;
            *fresh10 = (*fresh10).c_offset(1);
            *fresh11 = (*parser).indent;
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        if column > 2147483647 as libc::c_int as libc::c_long {
            (*parser).error = YAML_MEMORY_ERROR;
            return 0 as libc::c_int;
        }
        (*parser).indent = column as libc::c_int;
        memset(
            ptr::addr_of_mut!(token) as *mut libc::c_void,
            0 as libc::c_int,
            mem::size_of::<yaml_token_t>() as libc::c_ulong,
        );
        token.type_0 = type_0;
        token.start_mark = mark;
        token.end_mark = mark;
        if number == -(1 as libc::c_int) as libc::c_long {
            if if (*parser).tokens.tail != (*parser).tokens.end
                || yaml_queue_extend(
                    ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
                    ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
                    ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
                    ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh12 = &mut (*parser).tokens.tail;
                let fresh13 = *fresh12;
                *fresh12 = (*fresh12).c_offset(1);
                *fresh13 = token;
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
        } else if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
            ) != 0
        {
            memmove(
                ((*parser).tokens.head)
                    .c_offset(
                        (number as libc::c_ulong).wrapping_sub((*parser).tokens_parsed) as isize,
                    )
                    .c_offset(1 as libc::c_int as isize) as *mut libc::c_void,
                ((*parser).tokens.head).c_offset(
                    (number as libc::c_ulong).wrapping_sub((*parser).tokens_parsed) as isize,
                ) as *const libc::c_void,
                (((*parser).tokens.tail).c_offset_from((*parser).tokens.head) as libc::c_long
                    as libc::c_ulong)
                    .wrapping_sub((number as libc::c_ulong).wrapping_sub((*parser).tokens_parsed))
                    .wrapping_mul(mem::size_of::<yaml_token_t>() as libc::c_ulong),
            );
            *((*parser).tokens.head).c_offset(
                (number as libc::c_ulong).wrapping_sub((*parser).tokens_parsed) as isize,
            ) = token;
            let fresh14 = &mut (*parser).tokens.tail;
            *fresh14 = (*fresh14).c_offset(1);
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_unroll_indent(
    mut parser: *mut yaml_parser_t,
    column: ptrdiff_t,
) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if (*parser).flow_level != 0 {
        return 1 as libc::c_int;
    }
    while (*parser).indent as libc::c_long > column {
        memset(
            ptr::addr_of_mut!(token) as *mut libc::c_void,
            0 as libc::c_int,
            mem::size_of::<yaml_token_t>() as libc::c_ulong,
        );
        token.type_0 = YAML_BLOCK_END_TOKEN;
        token.start_mark = (*parser).mark;
        token.end_mark = (*parser).mark;
        if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh15 = &mut (*parser).tokens.tail;
            let fresh16 = *fresh15;
            *fresh15 = (*fresh15).c_offset(1);
            *fresh16 = token;
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        let fresh17 = &mut (*parser).indents.top;
        *fresh17 = (*fresh17).c_offset(-1);
        (*parser).indent = **fresh17;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_stream_start(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let simple_key: yaml_simple_key_t = {
        yaml_simple_key_s {
            possible: 0 as libc::c_int,
            required: 0 as libc::c_int,
            token_number: 0 as libc::c_int as size_t,
            mark: {
                yaml_mark_s {
                    index: 0 as libc::c_int as size_t,
                    line: 0 as libc::c_int as size_t,
                    column: 0 as libc::c_int as size_t,
                }
            },
        }
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    (*parser).indent = -(1 as libc::c_int);
    if if (*parser).simple_keys.top != (*parser).simple_keys.end
        || yaml_stack_extend(
            ptr::addr_of_mut!((*parser).simple_keys.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).simple_keys.top) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).simple_keys.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh18 = &mut (*parser).simple_keys.top;
        let fresh19 = *fresh18;
        *fresh18 = (*fresh18).c_offset(1);
        *fresh19 = simple_key;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 1 as libc::c_int;
    (*parser).stream_start_produced = 1 as libc::c_int;
    memset(
        ptr::addr_of_mut!(token) as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    token.type_0 = YAML_STREAM_START_TOKEN;
    token.start_mark = (*parser).mark;
    token.end_mark = (*parser).mark;
    token.data.stream_start.encoding = (*parser).encoding;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh20 = &mut (*parser).tokens.tail;
        let fresh21 = *fresh20;
        *fresh20 = (*fresh20).c_offset(1);
        *fresh21 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_stream_end(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if (*parser).mark.column != 0 as libc::c_int as libc::c_ulong {
        (*parser).mark.column = 0 as libc::c_int as size_t;
        let fresh22 = &mut (*parser).mark.line;
        *fresh22 = (*fresh22).wrapping_add(1);
    }
    if yaml_parser_unroll_indent(parser, -(1 as libc::c_int) as ptrdiff_t) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 0 as libc::c_int;
    memset(
        ptr::addr_of_mut!(token) as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    token.type_0 = YAML_STREAM_END_TOKEN;
    token.start_mark = (*parser).mark;
    token.end_mark = (*parser).mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh23 = &mut (*parser).tokens.tail;
        let fresh24 = *fresh23;
        *fresh23 = (*fresh23).c_offset(1);
        *fresh24 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_directive(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_unroll_indent(parser, -(1 as libc::c_int) as ptrdiff_t) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 0 as libc::c_int;
    if yaml_parser_scan_directive(parser, &mut token) == 0 {
        return 0 as libc::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh25 = &mut (*parser).tokens.tail;
        let fresh26 = *fresh25;
        *fresh25 = (*fresh25).c_offset(1);
        *fresh26 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_document_indicator(
    mut parser: *mut yaml_parser_t,
    type_0: yaml_token_type_t,
) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_unroll_indent(parser, -(1 as libc::c_int) as ptrdiff_t) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 0 as libc::c_int;
    let start_mark: yaml_mark_t = (*parser).mark;
    let fresh27 = &mut (*parser).mark.index;
    *fresh27 = (*fresh27).wrapping_add(1);
    let fresh28 = &mut (*parser).mark.column;
    *fresh28 = (*fresh28).wrapping_add(1);
    let fresh29 = &mut (*parser).unread;
    *fresh29 = (*fresh29).wrapping_sub(1);
    let fresh30 = &mut (*parser).buffer.pointer;
    *fresh30 = (*fresh30).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    let fresh31 = &mut (*parser).mark.index;
    *fresh31 = (*fresh31).wrapping_add(1);
    let fresh32 = &mut (*parser).mark.column;
    *fresh32 = (*fresh32).wrapping_add(1);
    let fresh33 = &mut (*parser).unread;
    *fresh33 = (*fresh33).wrapping_sub(1);
    let fresh34 = &mut (*parser).buffer.pointer;
    *fresh34 = (*fresh34).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    let fresh35 = &mut (*parser).mark.index;
    *fresh35 = (*fresh35).wrapping_add(1);
    let fresh36 = &mut (*parser).mark.column;
    *fresh36 = (*fresh36).wrapping_add(1);
    let fresh37 = &mut (*parser).unread;
    *fresh37 = (*fresh37).wrapping_sub(1);
    let fresh38 = &mut (*parser).buffer.pointer;
    *fresh38 = (*fresh38).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        ptr::addr_of_mut!(token) as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    token.type_0 = type_0;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh39 = &mut (*parser).tokens.tail;
        let fresh40 = *fresh39;
        *fresh39 = (*fresh39).c_offset(1);
        *fresh40 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_flow_collection_start(
    mut parser: *mut yaml_parser_t,
    type_0: yaml_token_type_t,
) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_parser_increase_flow_level(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 1 as libc::c_int;
    let start_mark: yaml_mark_t = (*parser).mark;
    let fresh41 = &mut (*parser).mark.index;
    *fresh41 = (*fresh41).wrapping_add(1);
    let fresh42 = &mut (*parser).mark.column;
    *fresh42 = (*fresh42).wrapping_add(1);
    let fresh43 = &mut (*parser).unread;
    *fresh43 = (*fresh43).wrapping_sub(1);
    let fresh44 = &mut (*parser).buffer.pointer;
    *fresh44 = (*fresh44).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        ptr::addr_of_mut!(token) as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    token.type_0 = type_0;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh45 = &mut (*parser).tokens.tail;
        let fresh46 = *fresh45;
        *fresh45 = (*fresh45).c_offset(1);
        *fresh46 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_flow_collection_end(
    mut parser: *mut yaml_parser_t,
    type_0: yaml_token_type_t,
) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    if yaml_parser_decrease_flow_level(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 0 as libc::c_int;
    let start_mark: yaml_mark_t = (*parser).mark;
    let fresh47 = &mut (*parser).mark.index;
    *fresh47 = (*fresh47).wrapping_add(1);
    let fresh48 = &mut (*parser).mark.column;
    *fresh48 = (*fresh48).wrapping_add(1);
    let fresh49 = &mut (*parser).unread;
    *fresh49 = (*fresh49).wrapping_sub(1);
    let fresh50 = &mut (*parser).buffer.pointer;
    *fresh50 = (*fresh50).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        ptr::addr_of_mut!(token) as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    token.type_0 = type_0;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh51 = &mut (*parser).tokens.tail;
        let fresh52 = *fresh51;
        *fresh51 = (*fresh51).c_offset(1);
        *fresh52 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_flow_entry(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 1 as libc::c_int;
    let start_mark: yaml_mark_t = (*parser).mark;
    let fresh53 = &mut (*parser).mark.index;
    *fresh53 = (*fresh53).wrapping_add(1);
    let fresh54 = &mut (*parser).mark.column;
    *fresh54 = (*fresh54).wrapping_add(1);
    let fresh55 = &mut (*parser).unread;
    *fresh55 = (*fresh55).wrapping_sub(1);
    let fresh56 = &mut (*parser).buffer.pointer;
    *fresh56 = (*fresh56).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        ptr::addr_of_mut!(token) as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    token.type_0 = YAML_FLOW_ENTRY_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh57 = &mut (*parser).tokens.tail;
        let fresh58 = *fresh57;
        *fresh57 = (*fresh57).c_offset(1);
        *fresh58 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_block_entry(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
            -(1 as libc::c_int) as ptrdiff_t,
            YAML_BLOCK_SEQUENCE_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 1 as libc::c_int;
    let start_mark: yaml_mark_t = (*parser).mark;
    let fresh59 = &mut (*parser).mark.index;
    *fresh59 = (*fresh59).wrapping_add(1);
    let fresh60 = &mut (*parser).mark.column;
    *fresh60 = (*fresh60).wrapping_add(1);
    let fresh61 = &mut (*parser).unread;
    *fresh61 = (*fresh61).wrapping_sub(1);
    let fresh62 = &mut (*parser).buffer.pointer;
    *fresh62 = (*fresh62).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        ptr::addr_of_mut!(token) as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    token.type_0 = YAML_BLOCK_ENTRY_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh63 = &mut (*parser).tokens.tail;
        let fresh64 = *fresh63;
        *fresh63 = (*fresh63).c_offset(1);
        *fresh64 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_key(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
            -(1 as libc::c_int) as ptrdiff_t,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = ((*parser).flow_level == 0) as libc::c_int;
    let start_mark: yaml_mark_t = (*parser).mark;
    let fresh65 = &mut (*parser).mark.index;
    *fresh65 = (*fresh65).wrapping_add(1);
    let fresh66 = &mut (*parser).mark.column;
    *fresh66 = (*fresh66).wrapping_add(1);
    let fresh67 = &mut (*parser).unread;
    *fresh67 = (*fresh67).wrapping_sub(1);
    let fresh68 = &mut (*parser).buffer.pointer;
    *fresh68 = (*fresh68).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        ptr::addr_of_mut!(token) as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    token.type_0 = YAML_KEY_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh69 = &mut (*parser).tokens.tail;
        let fresh70 = *fresh69;
        *fresh69 = (*fresh69).c_offset(1);
        *fresh70 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_value(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    let mut simple_key: *mut yaml_simple_key_t =
        ((*parser).simple_keys.top).c_offset(-(1 as libc::c_int as isize));
    if (*simple_key).possible != 0 {
        memset(
            ptr::addr_of_mut!(token) as *mut libc::c_void,
            0 as libc::c_int,
            mem::size_of::<yaml_token_t>() as libc::c_ulong,
        );
        token.type_0 = YAML_KEY_TOKEN;
        token.start_mark = (*simple_key).mark;
        token.end_mark = (*simple_key).mark;
        if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
                ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
            ) != 0
        {
            memmove(
                ((*parser).tokens.head)
                    .c_offset(
                        ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed) as isize,
                    )
                    .c_offset(1 as libc::c_int as isize) as *mut libc::c_void,
                ((*parser).tokens.head).c_offset(
                    ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed) as isize,
                ) as *const libc::c_void,
                (((*parser).tokens.tail).c_offset_from((*parser).tokens.head) as libc::c_long
                    as libc::c_ulong)
                    .wrapping_sub(
                        ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed),
                    )
                    .wrapping_mul(mem::size_of::<yaml_token_t>() as libc::c_ulong),
            );
            *((*parser).tokens.head).c_offset(
                ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed) as isize,
            ) = token;
            let fresh71 = &mut (*parser).tokens.tail;
            *fresh71 = (*fresh71).c_offset(1);
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        if yaml_parser_roll_indent(
            parser,
            (*simple_key).mark.column as ptrdiff_t,
            (*simple_key).token_number as ptrdiff_t,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*simple_key).mark,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        (*simple_key).possible = 0 as libc::c_int;
        (*parser).simple_key_allowed = 0 as libc::c_int;
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
                -(1 as libc::c_int) as ptrdiff_t,
                YAML_BLOCK_MAPPING_START_TOKEN,
                (*parser).mark,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        (*parser).simple_key_allowed = ((*parser).flow_level == 0) as libc::c_int;
    }
    let start_mark: yaml_mark_t = (*parser).mark;
    let fresh72 = &mut (*parser).mark.index;
    *fresh72 = (*fresh72).wrapping_add(1);
    let fresh73 = &mut (*parser).mark.column;
    *fresh73 = (*fresh73).wrapping_add(1);
    let fresh74 = &mut (*parser).unread;
    *fresh74 = (*fresh74).wrapping_sub(1);
    let fresh75 = &mut (*parser).buffer.pointer;
    *fresh75 = (*fresh75).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    let end_mark: yaml_mark_t = (*parser).mark;
    memset(
        ptr::addr_of_mut!(token) as *mut libc::c_void,
        0 as libc::c_int,
        mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
    token.type_0 = YAML_VALUE_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh76 = &mut (*parser).tokens.tail;
        let fresh77 = *fresh76;
        *fresh76 = (*fresh76).c_offset(1);
        *fresh77 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_anchor(
    mut parser: *mut yaml_parser_t,
    type_0: yaml_token_type_t,
) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 0 as libc::c_int;
    if yaml_parser_scan_anchor(parser, &mut token, type_0) == 0 {
        return 0 as libc::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh78 = &mut (*parser).tokens.tail;
        let fresh79 = *fresh78;
        *fresh78 = (*fresh78).c_offset(1);
        *fresh79 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_tag(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 0 as libc::c_int;
    if yaml_parser_scan_tag(parser, &mut token) == 0 {
        return 0 as libc::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh80 = &mut (*parser).tokens.tail;
        let fresh81 = *fresh80;
        *fresh80 = (*fresh80).c_offset(1);
        *fresh81 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_block_scalar(
    mut parser: *mut yaml_parser_t,
    literal: libc::c_int,
) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 1 as libc::c_int;
    if yaml_parser_scan_block_scalar(parser, &mut token, literal) == 0 {
        return 0 as libc::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh82 = &mut (*parser).tokens.tail;
        let fresh83 = *fresh82;
        *fresh82 = (*fresh82).c_offset(1);
        *fresh83 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_flow_scalar(
    mut parser: *mut yaml_parser_t,
    single: libc::c_int,
) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 0 as libc::c_int;
    if yaml_parser_scan_flow_scalar(parser, &mut token, single) == 0 {
        return 0 as libc::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh84 = &mut (*parser).tokens.tail;
        let fresh85 = *fresh84;
        *fresh84 = (*fresh84).c_offset(1);
        *fresh85 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_fetch_plain_scalar(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: unnamed_yaml_token_s_data {
            stream_start: unnamed_yaml_token_s_data_stream_start {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as libc::c_int;
    }
    (*parser).simple_key_allowed = 0 as libc::c_int;
    if yaml_parser_scan_plain_scalar(parser, &mut token) == 0 {
        return 0 as libc::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            ptr::addr_of_mut!((*parser).tokens.start) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.head) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.tail) as *mut *mut libc::c_void,
            ptr::addr_of_mut!((*parser).tokens.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh86 = &mut (*parser).tokens.tail;
        let fresh87 = *fresh86;
        *fresh86 = (*fresh86).c_offset(1);
        *fresh87 = token;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_to_next_token(mut parser: *mut yaml_parser_t) -> libc::c_int {
    loop {
        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
        } == 0
        {
            return 0 as libc::c_int;
        }
        if (*parser).mark.column == 0 as libc::c_int as libc::c_ulong
            && (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -17i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -69i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -65i32 as yaml_char_t as libc::c_int)
        {
            let fresh88 = &mut (*parser).mark.index;
            *fresh88 = (*fresh88).wrapping_add(1);
            let fresh89 = &mut (*parser).mark.column;
            *fresh89 = (*fresh89).wrapping_add(1);
            let fresh90 = &mut (*parser).unread;
            *fresh90 = (*fresh90).wrapping_sub(1);
            let fresh91 = &mut (*parser).buffer.pointer;
            *fresh91 = (*fresh91).c_offset(
                (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            );
        }
        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
        } == 0
        {
            return 0 as libc::c_int;
        }
        while *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || ((*parser).flow_level != 0 || (*parser).simple_key_allowed == 0)
                && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
        {
            let fresh92 = &mut (*parser).mark.index;
            *fresh92 = (*fresh92).wrapping_add(1);
            let fresh93 = &mut (*parser).mark.column;
            *fresh93 = (*fresh93).wrapping_add(1);
            let fresh94 = &mut (*parser).unread;
            *fresh94 = (*fresh94).wrapping_sub(1);
            let fresh95 = &mut (*parser).buffer.pointer;
            *fresh95 = (*fresh95).c_offset(
                (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            );
            if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                1 as libc::c_int
            } else {
                yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
            } == 0
            {
                return 0 as libc::c_int;
            }
        }
        if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '#' as i32 as yaml_char_t as libc::c_int
        {
            while !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer)
                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int)
            {
                let fresh96 = &mut (*parser).mark.index;
                *fresh96 = (*fresh96).wrapping_add(1);
                let fresh97 = &mut (*parser).mark.column;
                *fresh97 = (*fresh97).wrapping_add(1);
                let fresh98 = &mut (*parser).unread;
                *fresh98 = (*fresh98).wrapping_sub(1);
                let fresh99 = &mut (*parser).buffer.pointer;
                *fresh99 = (*fresh99).c_offset(
                    (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                );
                if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                    1 as libc::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                } == 0
                {
                    return 0 as libc::c_int;
                }
            }
        }
        if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
        {
            break;
        }
        if if (*parser).unread >= 2 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 2 as libc::c_int as size_t)
        } == 0
        {
            return 0 as libc::c_int;
        }
        if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            && *((*parser).buffer.pointer).c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
        {
            let fresh100 = &mut (*parser).mark.index;
            *fresh100 = (*fresh100 as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            (*parser).mark.column = 0 as libc::c_int as size_t;
            let fresh101 = &mut (*parser).mark.line;
            *fresh101 = (*fresh101).wrapping_add(1);
            let fresh102 = &mut (*parser).unread;
            *fresh102 = (*fresh102 as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            let fresh103 = &mut (*parser).buffer.pointer;
            *fresh103 = (*fresh103).c_offset(2 as libc::c_int as isize);
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            let fresh104 = &mut (*parser).mark.index;
            *fresh104 = (*fresh104).wrapping_add(1);
            (*parser).mark.column = 0 as libc::c_int as size_t;
            let fresh105 = &mut (*parser).mark.line;
            *fresh105 = (*fresh105).wrapping_add(1);
            let fresh106 = &mut (*parser).unread;
            *fresh106 = (*fresh106).wrapping_sub(1);
            let fresh107 = &mut (*parser).buffer.pointer;
            *fresh107 = (*fresh107).c_offset(
                (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            );
        };
        if (*parser).flow_level == 0 {
            (*parser).simple_key_allowed = 1 as libc::c_int;
        }
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_directive(
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
    let fresh108 = &mut (*parser).mark.index;
    *fresh108 = (*fresh108).wrapping_add(1);
    let fresh109 = &mut (*parser).mark.column;
    *fresh109 = (*fresh109).wrapping_add(1);
    let fresh110 = &mut (*parser).unread;
    *fresh110 = (*fresh110).wrapping_sub(1);
    let fresh111 = &mut (*parser).buffer.pointer;
    *fresh111 = (*fresh111).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    if !(yaml_parser_scan_directive_name(parser, start_mark, &mut name) == 0) {
        if strcmp(
            name as *mut libc::c_char,
            b"YAML\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if yaml_parser_scan_version_directive_value(parser, start_mark, &mut major, &mut minor)
                == 0
            {
                current_block = 11397968426844348457;
            } else {
                end_mark = (*parser).mark;
                memset(
                    token as *mut libc::c_void,
                    0 as libc::c_int,
                    mem::size_of::<yaml_token_t>() as libc::c_ulong,
                );
                (*token).type_0 = YAML_VERSION_DIRECTIVE_TOKEN;
                (*token).start_mark = start_mark;
                (*token).end_mark = end_mark;
                (*token).data.version_directive.major = major;
                (*token).data.version_directive.minor = minor;
                current_block = 17407779659766490442;
            }
        } else if strcmp(
            name as *mut libc::c_char,
            b"TAG\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if yaml_parser_scan_tag_directive_value(parser, start_mark, &mut handle, &mut prefix)
                == 0
            {
                current_block = 11397968426844348457;
            } else {
                end_mark = (*parser).mark;
                memset(
                    token as *mut libc::c_void,
                    0 as libc::c_int,
                    mem::size_of::<yaml_token_t>() as libc::c_ulong,
                );
                (*token).type_0 = YAML_TAG_DIRECTIVE_TOKEN;
                (*token).start_mark = start_mark;
                (*token).end_mark = end_mark;
                let fresh112 = &mut (*token).data.tag_directive.handle;
                *fresh112 = handle;
                let fresh113 = &mut (*token).data.tag_directive.prefix;
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
                if !(if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                    1 as libc::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                } == 0)
                {
                    loop {
                        if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int)
                        {
                            current_block = 11584701595673473500;
                            break;
                        }
                        let fresh114 = &mut (*parser).mark.index;
                        *fresh114 = (*fresh114).wrapping_add(1);
                        let fresh115 = &mut (*parser).mark.column;
                        *fresh115 = (*fresh115).wrapping_add(1);
                        let fresh116 = &mut (*parser).unread;
                        *fresh116 = (*fresh116).wrapping_sub(1);
                        let fresh117 = &mut (*parser).buffer.pointer;
                        *fresh117 = (*fresh117).c_offset(
                            (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0x80 as libc::c_int
                                == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else if *((*parser).buffer.pointer)
                                .c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else if *((*parser).buffer.pointer)
                                .c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else if *((*parser).buffer.pointer)
                                .c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) as isize,
                        );
                        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                            1 as libc::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                        } == 0
                        {
                            current_block = 11397968426844348457;
                            break;
                        }
                    }
                    match current_block {
                        11397968426844348457 => {}
                        _ => {
                            if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '#' as i32 as yaml_char_t as libc::c_int
                            {
                                loop {
                                    if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\n' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -62i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 1 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -123i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 1 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 2 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -88i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 1 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 2 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -87i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\0' as i32 as yaml_char_t as libc::c_int
                                    {
                                        current_block = 6669252993407410313;
                                        break;
                                    }
                                    let fresh118 = &mut (*parser).mark.index;
                                    *fresh118 = (*fresh118).wrapping_add(1);
                                    let fresh119 = &mut (*parser).mark.column;
                                    *fresh119 = (*fresh119).wrapping_add(1);
                                    let fresh120 = &mut (*parser).unread;
                                    *fresh120 = (*fresh120).wrapping_sub(1);
                                    let fresh121 = &mut (*parser).buffer.pointer;
                                    *fresh121 = (*fresh121).c_offset(
                                        (if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0x80 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            1 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xe0 as libc::c_int
                                            == 0xc0 as libc::c_int
                                        {
                                            2 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf0 as libc::c_int
                                            == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf8 as libc::c_int
                                            == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }) as isize,
                                    );
                                    if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                                        1 as libc::c_int
                                    } else {
                                        yaml_parser_update_buffer(
                                            parser,
                                            1 as libc::c_int as size_t,
                                        )
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
                                    if !(*((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\n' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -62i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 1 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -123i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 1 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 2 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -88i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 1 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 2 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == -87i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
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
                                        if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 2 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 2 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int
                                        {
                                            if if (*parser).unread
                                                >= 2 as libc::c_int as libc::c_ulong
                                            {
                                                1 as libc::c_int
                                            } else {
                                                yaml_parser_update_buffer(
                                                    parser,
                                                    2 as libc::c_int as size_t,
                                                )
                                            } == 0
                                            {
                                                current_block = 11397968426844348457;
                                            } else {
                                                if *((*parser).buffer.pointer)
                                                    .c_offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                                    && *((*parser).buffer.pointer).c_offset(
                                                        (0 as libc::c_int + 1 as libc::c_int)
                                                            as isize,
                                                    )
                                                        as libc::c_int
                                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                                {
                                                    let fresh122 = &mut (*parser).mark.index;
                                                    *fresh122 = (*fresh122 as libc::c_ulong)
                                                        .wrapping_add(
                                                            2 as libc::c_int as libc::c_ulong,
                                                        )
                                                        as size_t
                                                        as size_t;
                                                    (*parser).mark.column =
                                                        0 as libc::c_int as size_t;
                                                    let fresh123 = &mut (*parser).mark.line;
                                                    *fresh123 = (*fresh123).wrapping_add(1);
                                                    let fresh124 = &mut (*parser).unread;
                                                    *fresh124 = (*fresh124 as libc::c_ulong)
                                                        .wrapping_sub(
                                                            2 as libc::c_int as libc::c_ulong,
                                                        )
                                                        as size_t
                                                        as size_t;
                                                    let fresh125 = &mut (*parser).buffer.pointer;
                                                    *fresh125 = (*fresh125)
                                                        .c_offset(2 as libc::c_int as isize);
                                                } else if *((*parser).buffer.pointer)
                                                    .c_offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -62i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 1 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -123i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 1 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 2 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -88i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 1 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 2 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -87i32 as yaml_char_t as libc::c_int
                                                {
                                                    let fresh126 = &mut (*parser).mark.index;
                                                    *fresh126 = (*fresh126).wrapping_add(1);
                                                    (*parser).mark.column =
                                                        0 as libc::c_int as size_t;
                                                    let fresh127 = &mut (*parser).mark.line;
                                                    *fresh127 = (*fresh127).wrapping_add(1);
                                                    let fresh128 = &mut (*parser).unread;
                                                    *fresh128 = (*fresh128).wrapping_sub(1);
                                                    let fresh129 = &mut (*parser).buffer.pointer;
                                                    *fresh129 = (*fresh129).c_offset(
                                                        (if *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            & 0x80 as libc::c_int
                                                            == 0 as libc::c_int
                                                        {
                                                            1 as libc::c_int
                                                        } else if *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            & 0xe0 as libc::c_int
                                                            == 0xc0 as libc::c_int
                                                        {
                                                            2 as libc::c_int
                                                        } else if *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            & 0xf0 as libc::c_int
                                                            == 0xe0 as libc::c_int
                                                        {
                                                            3 as libc::c_int
                                                        } else if *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            & 0xf8 as libc::c_int
                                                            == 0xf0 as libc::c_int
                                                        {
                                                            4 as libc::c_int
                                                        } else {
                                                            0 as libc::c_int
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
                                                return 1 as libc::c_int;
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
    0 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_directive_name(
    mut parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    name: *mut *mut yaml_char_t,
) -> libc::c_int {
    let current_block: u64;
    let mut string: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    string.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).c_offset(16 as libc::c_int as isize);
        memset(
            string.start as *mut libc::c_void,
            0 as libc::c_int,
            16 as libc::c_int as libc::c_ulong,
        );
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        if !(if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
        } == 0)
        {
            loop {
                if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    >= '0' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        <= '9' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        >= 'A' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            <= 'Z' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        >= 'a' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            <= 'z' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == '_' as i32
                    || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == '-' as i32)
                {
                    current_block = 10879442775620481940;
                    break;
                }
                if if if (string.pointer).c_offset(5 as libc::c_int as isize) < string.end
                    || yaml_string_extend(&mut string.start, &mut string.pointer, &mut string.end)
                        != 0
                {
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } != 0
                {
                    if *(*parser).buffer.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh130 = &mut (*parser).buffer.pointer;
                        let fresh131 = *fresh130;
                        *fresh130 = (*fresh130).c_offset(1);
                        let fresh132 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh132 = *fresh131;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh133 = &mut (*parser).buffer.pointer;
                        let fresh134 = *fresh133;
                        *fresh133 = (*fresh133).c_offset(1);
                        let fresh135 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh135 = *fresh134;
                        let fresh136 = &mut (*parser).buffer.pointer;
                        let fresh137 = *fresh136;
                        *fresh136 = (*fresh136).c_offset(1);
                        let fresh138 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh138 = *fresh137;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh139 = &mut (*parser).buffer.pointer;
                        let fresh140 = *fresh139;
                        *fresh139 = (*fresh139).c_offset(1);
                        let fresh141 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh141 = *fresh140;
                        let fresh142 = &mut (*parser).buffer.pointer;
                        let fresh143 = *fresh142;
                        *fresh142 = (*fresh142).c_offset(1);
                        let fresh144 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh144 = *fresh143;
                        let fresh145 = &mut (*parser).buffer.pointer;
                        let fresh146 = *fresh145;
                        *fresh145 = (*fresh145).c_offset(1);
                        let fresh147 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh147 = *fresh146;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh148 = &mut (*parser).buffer.pointer;
                        let fresh149 = *fresh148;
                        *fresh148 = (*fresh148).c_offset(1);
                        let fresh150 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh150 = *fresh149;
                        let fresh151 = &mut (*parser).buffer.pointer;
                        let fresh152 = *fresh151;
                        *fresh151 = (*fresh151).c_offset(1);
                        let fresh153 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh153 = *fresh152;
                        let fresh154 = &mut (*parser).buffer.pointer;
                        let fresh155 = *fresh154;
                        *fresh154 = (*fresh154).c_offset(1);
                        let fresh156 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh156 = *fresh155;
                        let fresh157 = &mut (*parser).buffer.pointer;
                        let fresh158 = *fresh157;
                        *fresh157 = (*fresh157).c_offset(1);
                        let fresh159 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh159 = *fresh158;
                    };
                    let fresh160 = &mut (*parser).mark.index;
                    *fresh160 = (*fresh160).wrapping_add(1);
                    let fresh161 = &mut (*parser).mark.column;
                    *fresh161 = (*fresh161).wrapping_add(1);
                    let fresh162 = &mut (*parser).unread;
                    *fresh162 = (*fresh162).wrapping_sub(1);
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                } == 0
                {
                    current_block = 8318012024179131575;
                    break;
                }
                if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                    1 as libc::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
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
                    } else if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == ' ' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '\t' as i32 as yaml_char_t as libc::c_int
                        || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '\r' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\n' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == -62i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -123i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == -30i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -128i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -88i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == -30i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -128i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -87i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
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
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    0 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_version_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    major: *mut libc::c_int,
    minor: *mut libc::c_int,
) -> libc::c_int {
    if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
        1 as libc::c_int
    } else {
        yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
    } == 0
    {
        return 0 as libc::c_int;
    }
    while *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == ' ' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\t' as i32 as yaml_char_t as libc::c_int
    {
        let fresh163 = &mut (*parser).mark.index;
        *fresh163 = (*fresh163).wrapping_add(1);
        let fresh164 = &mut (*parser).mark.column;
        *fresh164 = (*fresh164).wrapping_add(1);
        let fresh165 = &mut (*parser).unread;
        *fresh165 = (*fresh165).wrapping_sub(1);
        let fresh166 = &mut (*parser).buffer.pointer;
        *fresh166 = (*fresh166).c_offset(
            (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        );
        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
        } == 0
        {
            return 0 as libc::c_int;
        }
    }
    if yaml_parser_scan_version_directive_number(parser, start_mark, major) == 0 {
        return 0 as libc::c_int;
    }
    if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        == '.' as i32 as yaml_char_t as libc::c_int)
    {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0" as *const u8 as *const libc::c_char,
            start_mark,
            b"did not find expected digit or '.' character\0" as *const u8 as *const libc::c_char,
        );
    }
    let fresh167 = &mut (*parser).mark.index;
    *fresh167 = (*fresh167).wrapping_add(1);
    let fresh168 = &mut (*parser).mark.column;
    *fresh168 = (*fresh168).wrapping_add(1);
    let fresh169 = &mut (*parser).unread;
    *fresh169 = (*fresh169).wrapping_sub(1);
    let fresh170 = &mut (*parser).buffer.pointer;
    *fresh170 = (*fresh170).c_offset(
        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0x80 as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            2 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        {
            3 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    if yaml_parser_scan_version_directive_number(parser, start_mark, minor) == 0 {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_version_directive_number(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    number: *mut libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_int = 0 as libc::c_int;
    let mut length: size_t = 0 as libc::c_int as size_t;
    if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
        1 as libc::c_int
    } else {
        yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
    } == 0
    {
        return 0 as libc::c_int;
    }
    while *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
        >= '0' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            <= '9' as i32 as yaml_char_t as libc::c_int
    {
        length = length.wrapping_add(1);
        if length > 9 as libc::c_int as libc::c_ulong {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a %YAML directive\0" as *const u8 as *const libc::c_char,
                start_mark,
                b"found extremely long version number\0" as *const u8 as *const libc::c_char,
            );
        }
        value = value * 10 as libc::c_int
            + (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                - '0' as i32 as yaml_char_t as libc::c_int);
        let fresh171 = &mut (*parser).mark.index;
        *fresh171 = (*fresh171).wrapping_add(1);
        let fresh172 = &mut (*parser).mark.column;
        *fresh172 = (*fresh172).wrapping_add(1);
        let fresh173 = &mut (*parser).unread;
        *fresh173 = (*fresh173).wrapping_sub(1);
        let fresh174 = &mut (*parser).buffer.pointer;
        *fresh174 = (*fresh174).c_offset(
            (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        );
        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
        } == 0
        {
            return 0 as libc::c_int;
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
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_tag_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
    prefix: *mut *mut yaml_char_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut handle_value: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut prefix_value: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
        1 as libc::c_int
    } else {
        yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
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
                return 0 as libc::c_int;
            }
            _ => {
                if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == '\t' as i32 as yaml_char_t as libc::c_int
                {
                    let fresh175 = &mut (*parser).mark.index;
                    *fresh175 = (*fresh175).wrapping_add(1);
                    let fresh176 = &mut (*parser).mark.column;
                    *fresh176 = (*fresh176).wrapping_add(1);
                    let fresh177 = &mut (*parser).unread;
                    *fresh177 = (*fresh177).wrapping_sub(1);
                    let fresh178 = &mut (*parser).buffer.pointer;
                    *fresh178 = (*fresh178).c_offset(
                        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf8 as libc::c_int
                            == 0xf0 as libc::c_int
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    );
                    if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                        1 as libc::c_int
                    } else {
                        yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                    } == 0
                    {
                        current_block = 5231181710497607163;
                    } else {
                        current_block = 14916268686031723178;
                    }
                } else {
                    if yaml_parser_scan_tag_handle(
                        parser,
                        1 as libc::c_int,
                        start_mark,
                        &mut handle_value,
                    ) == 0
                    {
                        current_block = 5231181710497607163;
                        continue;
                    }
                    if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                        1 as libc::c_int
                    } else {
                        yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                    } == 0
                    {
                        current_block = 5231181710497607163;
                        continue;
                    }
                    if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == ' ' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
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
                        while *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                        {
                            let fresh179 = &mut (*parser).mark.index;
                            *fresh179 = (*fresh179).wrapping_add(1);
                            let fresh180 = &mut (*parser).mark.column;
                            *fresh180 = (*fresh180).wrapping_add(1);
                            let fresh181 = &mut (*parser).unread;
                            *fresh181 = (*fresh181).wrapping_sub(1);
                            let fresh182 = &mut (*parser).buffer.pointer;
                            *fresh182 = (*fresh182).c_offset(
                                (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0x80 as libc::c_int
                                    == 0 as libc::c_int
                                {
                                    1 as libc::c_int
                                } else if *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0xe0 as libc::c_int
                                    == 0xc0 as libc::c_int
                                {
                                    2 as libc::c_int
                                } else if *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0xf0 as libc::c_int
                                    == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else if *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as isize,
                            );
                            if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                                1 as libc::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                            } == 0
                            {
                                current_block = 5231181710497607163;
                                continue 'c_34337;
                            }
                        }
                        if yaml_parser_scan_tag_uri(
                            parser,
                            1 as libc::c_int,
                            1 as libc::c_int,
                            ptr::null_mut::<yaml_char_t>(),
                            start_mark,
                            &mut prefix_value,
                        ) == 0
                        {
                            current_block = 5231181710497607163;
                            continue;
                        }
                        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                            1 as libc::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                        } == 0
                        {
                            current_block = 5231181710497607163;
                            continue;
                        }
                        if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
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
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn yaml_parser_scan_anchor(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    type_0: yaml_token_type_t,
) -> libc::c_int {
    let current_block: u64;
    let mut length: libc::c_int = 0 as libc::c_int;
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut string: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    string.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).c_offset(16 as libc::c_int as isize);
        memset(
            string.start as *mut libc::c_void,
            0 as libc::c_int,
            16 as libc::c_int as libc::c_ulong,
        );
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        start_mark = (*parser).mark;
        let fresh183 = &mut (*parser).mark.index;
        *fresh183 = (*fresh183).wrapping_add(1);
        let fresh184 = &mut (*parser).mark.column;
        *fresh184 = (*fresh184).wrapping_add(1);
        let fresh185 = &mut (*parser).unread;
        *fresh185 = (*fresh185).wrapping_sub(1);
        let fresh186 = &mut (*parser).buffer.pointer;
        *fresh186 = (*fresh186).c_offset(
            (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        );
        if !(if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
        } == 0)
        {
            loop {
                if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    >= '0' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        <= '9' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        >= 'A' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            <= 'Z' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        >= 'a' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            <= 'z' as i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == '_' as i32
                    || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == '-' as i32)
                {
                    current_block = 2868539653012386629;
                    break;
                }
                if if if (string.pointer).c_offset(5 as libc::c_int as isize) < string.end
                    || yaml_string_extend(&mut string.start, &mut string.pointer, &mut string.end)
                        != 0
                {
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } != 0
                {
                    if *(*parser).buffer.pointer as libc::c_int & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        let fresh187 = &mut (*parser).buffer.pointer;
                        let fresh188 = *fresh187;
                        *fresh187 = (*fresh187).c_offset(1);
                        let fresh189 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh189 = *fresh188;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        let fresh190 = &mut (*parser).buffer.pointer;
                        let fresh191 = *fresh190;
                        *fresh190 = (*fresh190).c_offset(1);
                        let fresh192 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh192 = *fresh191;
                        let fresh193 = &mut (*parser).buffer.pointer;
                        let fresh194 = *fresh193;
                        *fresh193 = (*fresh193).c_offset(1);
                        let fresh195 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh195 = *fresh194;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        let fresh196 = &mut (*parser).buffer.pointer;
                        let fresh197 = *fresh196;
                        *fresh196 = (*fresh196).c_offset(1);
                        let fresh198 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh198 = *fresh197;
                        let fresh199 = &mut (*parser).buffer.pointer;
                        let fresh200 = *fresh199;
                        *fresh199 = (*fresh199).c_offset(1);
                        let fresh201 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh201 = *fresh200;
                        let fresh202 = &mut (*parser).buffer.pointer;
                        let fresh203 = *fresh202;
                        *fresh202 = (*fresh202).c_offset(1);
                        let fresh204 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh204 = *fresh203;
                    } else if *(*parser).buffer.pointer as libc::c_int & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        let fresh205 = &mut (*parser).buffer.pointer;
                        let fresh206 = *fresh205;
                        *fresh205 = (*fresh205).c_offset(1);
                        let fresh207 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh207 = *fresh206;
                        let fresh208 = &mut (*parser).buffer.pointer;
                        let fresh209 = *fresh208;
                        *fresh208 = (*fresh208).c_offset(1);
                        let fresh210 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh210 = *fresh209;
                        let fresh211 = &mut (*parser).buffer.pointer;
                        let fresh212 = *fresh211;
                        *fresh211 = (*fresh211).c_offset(1);
                        let fresh213 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh213 = *fresh212;
                        let fresh214 = &mut (*parser).buffer.pointer;
                        let fresh215 = *fresh214;
                        *fresh214 = (*fresh214).c_offset(1);
                        let fresh216 = string.pointer;
                        string.pointer = (string.pointer).c_offset(1);
                        *fresh216 = *fresh215;
                    };
                    let fresh217 = &mut (*parser).mark.index;
                    *fresh217 = (*fresh217).wrapping_add(1);
                    let fresh218 = &mut (*parser).mark.column;
                    *fresh218 = (*fresh218).wrapping_add(1);
                    let fresh219 = &mut (*parser).unread;
                    *fresh219 = (*fresh219).wrapping_sub(1);
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                } == 0
                {
                    current_block = 5883759901342942623;
                    break;
                }
                if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                    1 as libc::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
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
                        || !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\0' as i32 as yaml_char_t as libc::c_int)
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '?' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == ':' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == ',' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == ']' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '}' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '%' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '@' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '`' as i32 as yaml_char_t as libc::c_int)
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            if type_0 as libc::c_uint
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
                        if type_0 as libc::c_uint
                            == YAML_ANCHOR_TOKEN as libc::c_int as libc::c_uint
                        {
                            memset(
                                token as *mut libc::c_void,
                                0 as libc::c_int,
                                mem::size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_0 = YAML_ANCHOR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh220 = &mut (*token).data.anchor.value;
                            *fresh220 = string.start;
                        } else {
                            memset(
                                token as *mut libc::c_void,
                                0 as libc::c_int,
                                mem::size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_0 = YAML_ALIAS_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh221 = &mut (*token).data.alias.value;
                            *fresh221 = string.start;
                        }
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    0 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_tag(
    parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut handle: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut suffix: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let end_mark: yaml_mark_t;
    let start_mark: yaml_mark_t = (*parser).mark;
    if !(if (*parser).unread >= 2 as libc::c_int as libc::c_ulong {
        1 as libc::c_int
    } else {
        yaml_parser_update_buffer(parser, 2 as libc::c_int as size_t)
    } == 0)
    {
        if *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
            == '<' as i32 as yaml_char_t as libc::c_int
        {
            handle = yaml_malloc(1 as libc::c_int as size_t) as *mut yaml_char_t;
            if handle.is_null() {
                current_block = 17708497480799081542;
            } else {
                *handle.c_offset(0 as libc::c_int as isize) = '\0' as i32 as yaml_char_t;
                let fresh222 = &mut (*parser).mark.index;
                *fresh222 = (*fresh222).wrapping_add(1);
                let fresh223 = &mut (*parser).mark.column;
                *fresh223 = (*fresh223).wrapping_add(1);
                let fresh224 = &mut (*parser).unread;
                *fresh224 = (*fresh224).wrapping_sub(1);
                let fresh225 = &mut (*parser).buffer.pointer;
                *fresh225 = (*fresh225).c_offset(
                    (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                );
                let fresh226 = &mut (*parser).mark.index;
                *fresh226 = (*fresh226).wrapping_add(1);
                let fresh227 = &mut (*parser).mark.column;
                *fresh227 = (*fresh227).wrapping_add(1);
                let fresh228 = &mut (*parser).unread;
                *fresh228 = (*fresh228).wrapping_sub(1);
                let fresh229 = &mut (*parser).buffer.pointer;
                *fresh229 = (*fresh229).c_offset(
                    (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                );
                if yaml_parser_scan_tag_uri(
                    parser,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    ptr::null_mut::<yaml_char_t>(),
                    start_mark,
                    &mut suffix,
                ) == 0
                {
                    current_block = 17708497480799081542;
                } else if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
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
                    let fresh230 = &mut (*parser).mark.index;
                    *fresh230 = (*fresh230).wrapping_add(1);
                    let fresh231 = &mut (*parser).mark.column;
                    *fresh231 = (*fresh231).wrapping_add(1);
                    let fresh232 = &mut (*parser).unread;
                    *fresh232 = (*fresh232).wrapping_sub(1);
                    let fresh233 = &mut (*parser).buffer.pointer;
                    *fresh233 = (*fresh233).c_offset(
                        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf8 as libc::c_int
                            == 0xf0 as libc::c_int
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    );
                    current_block = 4488286894823169796;
                }
            }
        } else if yaml_parser_scan_tag_handle(parser, 0 as libc::c_int, start_mark, &mut handle)
            == 0
        {
            current_block = 17708497480799081542;
        } else if *handle.c_offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32
            && *handle.c_offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
            && *handle.c_offset(
                (strlen(handle as *mut libc::c_char))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int
                == '!' as i32
        {
            if yaml_parser_scan_tag_uri(
                parser,
                0 as libc::c_int,
                0 as libc::c_int,
                ptr::null_mut::<yaml_char_t>(),
                start_mark,
                &mut suffix,
            ) == 0
            {
                current_block = 17708497480799081542;
            } else {
                current_block = 4488286894823169796;
            }
        } else if yaml_parser_scan_tag_uri(
            parser,
            0 as libc::c_int,
            0 as libc::c_int,
            handle,
            start_mark,
            &mut suffix,
        ) == 0
        {
            current_block = 17708497480799081542;
        } else {
            yaml_free(handle as *mut libc::c_void);
            handle = yaml_malloc(2 as libc::c_int as size_t) as *mut yaml_char_t;
            if handle.is_null() {
                current_block = 17708497480799081542;
            } else {
                *handle.c_offset(0 as libc::c_int as isize) = '!' as i32 as yaml_char_t;
                *handle.c_offset(1 as libc::c_int as isize) = '\0' as i32 as yaml_char_t;
                if *suffix.c_offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
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
                if !(if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                    1 as libc::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                } == 0)
                {
                    if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == ' ' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '\t' as i32 as yaml_char_t as libc::c_int
                        || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '\r' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\n' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == -62i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -123i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == -30i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -128i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -88i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == -30i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -128i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer)
                                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                    as libc::c_int
                                    == -87i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\0' as i32 as yaml_char_t as libc::c_int))
                    {
                        if (*parser).flow_level == 0
                            || !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
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
                                0 as libc::c_int,
                                mem::size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_0 = YAML_TAG_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh234 = &mut (*token).data.tag.handle;
                            *fresh234 = handle;
                            let fresh235 = &mut (*token).data.tag.suffix;
                            *fresh235 = suffix;
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_free(handle as *mut libc::c_void);
    yaml_free(suffix as *mut libc::c_void);
    0 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_tag_handle(
    mut parser: *mut yaml_parser_t,
    directive: libc::c_int,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut string: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    string.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).c_offset(16 as libc::c_int as isize);
        memset(
            string.start as *mut libc::c_void,
            0 as libc::c_int,
            16 as libc::c_int as libc::c_ulong,
        );
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        if !(if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
        } == 0)
        {
            if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
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
            } else if !(if if (string.pointer).c_offset(5 as libc::c_int as isize) < string.end
                || yaml_string_extend(&mut string.start, &mut string.pointer, &mut string.end) != 0
            {
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } != 0
            {
                if *(*parser).buffer.pointer as libc::c_int & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    let fresh236 = &mut (*parser).buffer.pointer;
                    let fresh237 = *fresh236;
                    *fresh236 = (*fresh236).c_offset(1);
                    let fresh238 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh238 = *fresh237;
                } else if *(*parser).buffer.pointer as libc::c_int & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    let fresh239 = &mut (*parser).buffer.pointer;
                    let fresh240 = *fresh239;
                    *fresh239 = (*fresh239).c_offset(1);
                    let fresh241 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh241 = *fresh240;
                    let fresh242 = &mut (*parser).buffer.pointer;
                    let fresh243 = *fresh242;
                    *fresh242 = (*fresh242).c_offset(1);
                    let fresh244 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh244 = *fresh243;
                } else if *(*parser).buffer.pointer as libc::c_int & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    let fresh245 = &mut (*parser).buffer.pointer;
                    let fresh246 = *fresh245;
                    *fresh245 = (*fresh245).c_offset(1);
                    let fresh247 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh247 = *fresh246;
                    let fresh248 = &mut (*parser).buffer.pointer;
                    let fresh249 = *fresh248;
                    *fresh248 = (*fresh248).c_offset(1);
                    let fresh250 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh250 = *fresh249;
                    let fresh251 = &mut (*parser).buffer.pointer;
                    let fresh252 = *fresh251;
                    *fresh251 = (*fresh251).c_offset(1);
                    let fresh253 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh253 = *fresh252;
                } else if *(*parser).buffer.pointer as libc::c_int & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    let fresh254 = &mut (*parser).buffer.pointer;
                    let fresh255 = *fresh254;
                    *fresh254 = (*fresh254).c_offset(1);
                    let fresh256 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh256 = *fresh255;
                    let fresh257 = &mut (*parser).buffer.pointer;
                    let fresh258 = *fresh257;
                    *fresh257 = (*fresh257).c_offset(1);
                    let fresh259 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh259 = *fresh258;
                    let fresh260 = &mut (*parser).buffer.pointer;
                    let fresh261 = *fresh260;
                    *fresh260 = (*fresh260).c_offset(1);
                    let fresh262 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh262 = *fresh261;
                    let fresh263 = &mut (*parser).buffer.pointer;
                    let fresh264 = *fresh263;
                    *fresh263 = (*fresh263).c_offset(1);
                    let fresh265 = string.pointer;
                    string.pointer = (string.pointer).c_offset(1);
                    *fresh265 = *fresh264;
                };
                let fresh266 = &mut (*parser).mark.index;
                *fresh266 = (*fresh266).wrapping_add(1);
                let fresh267 = &mut (*parser).mark.column;
                *fresh267 = (*fresh267).wrapping_add(1);
                let fresh268 = &mut (*parser).unread;
                *fresh268 = (*fresh268).wrapping_sub(1);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0)
            {
                if !(if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                    1 as libc::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                } == 0)
                {
                    loop {
                        if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            >= '0' as i32 as yaml_char_t as libc::c_int
                            && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                <= '9' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                >= 'A' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    <= 'Z' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                >= 'a' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    <= 'z' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '_' as i32
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '-' as i32)
                        {
                            current_block = 7651349459974463963;
                            break;
                        }
                        if if if (string.pointer).c_offset(5 as libc::c_int as isize) < string.end
                            || yaml_string_extend(
                                &mut string.start,
                                &mut string.pointer,
                                &mut string.end,
                            ) != 0
                        {
                            1 as libc::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } != 0
                        {
                            if *(*parser).buffer.pointer as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                            {
                                let fresh269 = &mut (*parser).buffer.pointer;
                                let fresh270 = *fresh269;
                                *fresh269 = (*fresh269).c_offset(1);
                                let fresh271 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh271 = *fresh270;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                let fresh272 = &mut (*parser).buffer.pointer;
                                let fresh273 = *fresh272;
                                *fresh272 = (*fresh272).c_offset(1);
                                let fresh274 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh274 = *fresh273;
                                let fresh275 = &mut (*parser).buffer.pointer;
                                let fresh276 = *fresh275;
                                *fresh275 = (*fresh275).c_offset(1);
                                let fresh277 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh277 = *fresh276;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh278 = &mut (*parser).buffer.pointer;
                                let fresh279 = *fresh278;
                                *fresh278 = (*fresh278).c_offset(1);
                                let fresh280 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh280 = *fresh279;
                                let fresh281 = &mut (*parser).buffer.pointer;
                                let fresh282 = *fresh281;
                                *fresh281 = (*fresh281).c_offset(1);
                                let fresh283 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh283 = *fresh282;
                                let fresh284 = &mut (*parser).buffer.pointer;
                                let fresh285 = *fresh284;
                                *fresh284 = (*fresh284).c_offset(1);
                                let fresh286 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh286 = *fresh285;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                let fresh287 = &mut (*parser).buffer.pointer;
                                let fresh288 = *fresh287;
                                *fresh287 = (*fresh287).c_offset(1);
                                let fresh289 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh289 = *fresh288;
                                let fresh290 = &mut (*parser).buffer.pointer;
                                let fresh291 = *fresh290;
                                *fresh290 = (*fresh290).c_offset(1);
                                let fresh292 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh292 = *fresh291;
                                let fresh293 = &mut (*parser).buffer.pointer;
                                let fresh294 = *fresh293;
                                *fresh293 = (*fresh293).c_offset(1);
                                let fresh295 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh295 = *fresh294;
                                let fresh296 = &mut (*parser).buffer.pointer;
                                let fresh297 = *fresh296;
                                *fresh296 = (*fresh296).c_offset(1);
                                let fresh298 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh298 = *fresh297;
                            };
                            let fresh299 = &mut (*parser).mark.index;
                            *fresh299 = (*fresh299).wrapping_add(1);
                            let fresh300 = &mut (*parser).mark.column;
                            *fresh300 = (*fresh300).wrapping_add(1);
                            let fresh301 = &mut (*parser).unread;
                            *fresh301 = (*fresh301).wrapping_sub(1);
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        } == 0
                        {
                            current_block = 1771849829115608806;
                            break;
                        }
                        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                            1 as libc::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                        } == 0
                        {
                            current_block = 1771849829115608806;
                            break;
                        }
                    }
                    match current_block {
                        1771849829115608806 => {}
                        _ => {
                            if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '!' as i32 as yaml_char_t as libc::c_int
                            {
                                if if if (string.pointer).c_offset(5 as libc::c_int as isize)
                                    < string.end
                                    || yaml_string_extend(
                                        &mut string.start,
                                        &mut string.pointer,
                                        &mut string.end,
                                    ) != 0
                                {
                                    1 as libc::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as libc::c_int
                                } != 0
                                {
                                    if *(*parser).buffer.pointer as libc::c_int
                                        & 0x80 as libc::c_int
                                        == 0 as libc::c_int
                                    {
                                        let fresh302 = &mut (*parser).buffer.pointer;
                                        let fresh303 = *fresh302;
                                        *fresh302 = (*fresh302).c_offset(1);
                                        let fresh304 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh304 = *fresh303;
                                    } else if *(*parser).buffer.pointer as libc::c_int
                                        & 0xe0 as libc::c_int
                                        == 0xc0 as libc::c_int
                                    {
                                        let fresh305 = &mut (*parser).buffer.pointer;
                                        let fresh306 = *fresh305;
                                        *fresh305 = (*fresh305).c_offset(1);
                                        let fresh307 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh307 = *fresh306;
                                        let fresh308 = &mut (*parser).buffer.pointer;
                                        let fresh309 = *fresh308;
                                        *fresh308 = (*fresh308).c_offset(1);
                                        let fresh310 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh310 = *fresh309;
                                    } else if *(*parser).buffer.pointer as libc::c_int
                                        & 0xf0 as libc::c_int
                                        == 0xe0 as libc::c_int
                                    {
                                        let fresh311 = &mut (*parser).buffer.pointer;
                                        let fresh312 = *fresh311;
                                        *fresh311 = (*fresh311).c_offset(1);
                                        let fresh313 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh313 = *fresh312;
                                        let fresh314 = &mut (*parser).buffer.pointer;
                                        let fresh315 = *fresh314;
                                        *fresh314 = (*fresh314).c_offset(1);
                                        let fresh316 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh316 = *fresh315;
                                        let fresh317 = &mut (*parser).buffer.pointer;
                                        let fresh318 = *fresh317;
                                        *fresh317 = (*fresh317).c_offset(1);
                                        let fresh319 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh319 = *fresh318;
                                    } else if *(*parser).buffer.pointer as libc::c_int
                                        & 0xf8 as libc::c_int
                                        == 0xf0 as libc::c_int
                                    {
                                        let fresh320 = &mut (*parser).buffer.pointer;
                                        let fresh321 = *fresh320;
                                        *fresh320 = (*fresh320).c_offset(1);
                                        let fresh322 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh322 = *fresh321;
                                        let fresh323 = &mut (*parser).buffer.pointer;
                                        let fresh324 = *fresh323;
                                        *fresh323 = (*fresh323).c_offset(1);
                                        let fresh325 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh325 = *fresh324;
                                        let fresh326 = &mut (*parser).buffer.pointer;
                                        let fresh327 = *fresh326;
                                        *fresh326 = (*fresh326).c_offset(1);
                                        let fresh328 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh328 = *fresh327;
                                        let fresh329 = &mut (*parser).buffer.pointer;
                                        let fresh330 = *fresh329;
                                        *fresh329 = (*fresh329).c_offset(1);
                                        let fresh331 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh331 = *fresh330;
                                    };
                                    let fresh332 = &mut (*parser).mark.index;
                                    *fresh332 = (*fresh332).wrapping_add(1);
                                    let fresh333 = &mut (*parser).mark.column;
                                    *fresh333 = (*fresh333).wrapping_add(1);
                                    let fresh334 = &mut (*parser).unread;
                                    *fresh334 = (*fresh334).wrapping_sub(1);
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                } == 0
                                {
                                    current_block = 1771849829115608806;
                                } else {
                                    current_block = 5689001924483802034;
                                }
                            } else if directive != 0
                                && !(*(string.start).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '!' as i32
                                    && *(string.start).c_offset(1 as libc::c_int as isize)
                                        as libc::c_int
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
                                    return 1 as libc::c_int;
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
    0 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_tag_uri(
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
        0 as libc::c_int as libc::c_ulong
    };
    let mut string: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    string.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
    if if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).c_offset(16 as libc::c_int as isize);
        memset(
            string.start as *mut libc::c_void,
            0 as libc::c_int,
            16 as libc::c_int as libc::c_ulong,
        );
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
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
                return 0 as libc::c_int;
            }
            _ => {
                if (string.end).c_offset_from(string.start) as libc::c_long as size_t <= length {
                    if !(yaml_string_extend(
                        &mut string.start,
                        &mut string.pointer,
                        &mut string.end,
                    ) == 0)
                    {
                        current_block = 14916268686031723178;
                        continue;
                    }
                    (*parser).error = YAML_MEMORY_ERROR;
                    current_block = 15265153392498847348;
                } else {
                    if length > 1 as libc::c_int as libc::c_ulong {
                        memcpy(
                            string.start as *mut libc::c_void,
                            head.c_offset(1 as libc::c_int as isize) as *const libc::c_void,
                            length.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        string.pointer = (string.pointer).c_offset(
                            length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    }
                    if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                        1 as libc::c_int
                    } else {
                        yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                    } == 0
                    {
                        current_block = 15265153392498847348;
                        continue;
                    }
                    while *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        >= '0' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            <= '9' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            >= 'A' as i32 as yaml_char_t as libc::c_int
                            && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                <= 'Z' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            >= 'a' as i32 as yaml_char_t as libc::c_int
                            && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                <= 'z' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '_' as i32
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '-' as i32
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ';' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '/' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '?' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ':' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '@' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '&' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '=' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '+' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '$' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '.' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '%' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '!' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '~' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '*' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '\'' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '(' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ')' as i32 as yaml_char_t as libc::c_int
                        || uri_char != 0
                            && (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == ',' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '[' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == ']' as i32 as yaml_char_t as libc::c_int)
                    {
                        if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '%' as i32 as yaml_char_t as libc::c_int
                        {
                            if if (string.pointer).c_offset(5 as libc::c_int as isize) < string.end
                                || yaml_string_extend(
                                    &mut string.start,
                                    &mut string.pointer,
                                    &mut string.end,
                                ) != 0
                            {
                                1 as libc::c_int
                            } else {
                                (*parser).error = YAML_MEMORY_ERROR;
                                0 as libc::c_int
                            } == 0
                            {
                                current_block = 15265153392498847348;
                                continue 'c_21953;
                            }
                            if yaml_parser_scan_uri_escapes(
                                parser,
                                directive,
                                start_mark,
                                &mut string,
                            ) == 0
                            {
                                current_block = 15265153392498847348;
                                continue 'c_21953;
                            }
                        } else if if if (string.pointer).c_offset(5 as libc::c_int as isize)
                            < string.end
                            || yaml_string_extend(
                                &mut string.start,
                                &mut string.pointer,
                                &mut string.end,
                            ) != 0
                        {
                            1 as libc::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } != 0
                        {
                            if *(*parser).buffer.pointer as libc::c_int & 0x80 as libc::c_int
                                == 0 as libc::c_int
                            {
                                let fresh335 = &mut (*parser).buffer.pointer;
                                let fresh336 = *fresh335;
                                *fresh335 = (*fresh335).c_offset(1);
                                let fresh337 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh337 = *fresh336;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                let fresh338 = &mut (*parser).buffer.pointer;
                                let fresh339 = *fresh338;
                                *fresh338 = (*fresh338).c_offset(1);
                                let fresh340 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh340 = *fresh339;
                                let fresh341 = &mut (*parser).buffer.pointer;
                                let fresh342 = *fresh341;
                                *fresh341 = (*fresh341).c_offset(1);
                                let fresh343 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh343 = *fresh342;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                let fresh344 = &mut (*parser).buffer.pointer;
                                let fresh345 = *fresh344;
                                *fresh344 = (*fresh344).c_offset(1);
                                let fresh346 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh346 = *fresh345;
                                let fresh347 = &mut (*parser).buffer.pointer;
                                let fresh348 = *fresh347;
                                *fresh347 = (*fresh347).c_offset(1);
                                let fresh349 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh349 = *fresh348;
                                let fresh350 = &mut (*parser).buffer.pointer;
                                let fresh351 = *fresh350;
                                *fresh350 = (*fresh350).c_offset(1);
                                let fresh352 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh352 = *fresh351;
                            } else if *(*parser).buffer.pointer as libc::c_int & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                let fresh353 = &mut (*parser).buffer.pointer;
                                let fresh354 = *fresh353;
                                *fresh353 = (*fresh353).c_offset(1);
                                let fresh355 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh355 = *fresh354;
                                let fresh356 = &mut (*parser).buffer.pointer;
                                let fresh357 = *fresh356;
                                *fresh356 = (*fresh356).c_offset(1);
                                let fresh358 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh358 = *fresh357;
                                let fresh359 = &mut (*parser).buffer.pointer;
                                let fresh360 = *fresh359;
                                *fresh359 = (*fresh359).c_offset(1);
                                let fresh361 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh361 = *fresh360;
                                let fresh362 = &mut (*parser).buffer.pointer;
                                let fresh363 = *fresh362;
                                *fresh362 = (*fresh362).c_offset(1);
                                let fresh364 = string.pointer;
                                string.pointer = (string.pointer).c_offset(1);
                                *fresh364 = *fresh363;
                            };
                            let fresh365 = &mut (*parser).mark.index;
                            *fresh365 = (*fresh365).wrapping_add(1);
                            let fresh366 = &mut (*parser).mark.column;
                            *fresh366 = (*fresh366).wrapping_add(1);
                            let fresh367 = &mut (*parser).unread;
                            *fresh367 = (*fresh367).wrapping_sub(1);
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        } == 0
                        {
                            current_block = 15265153392498847348;
                            continue 'c_21953;
                        }
                        length = length.wrapping_add(1);
                        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                            1 as libc::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                        } == 0
                        {
                            current_block = 15265153392498847348;
                            continue 'c_21953;
                        }
                    }
                    if length == 0 {
                        if if (string.pointer).c_offset(5 as libc::c_int as isize) < string.end
                            || yaml_string_extend(
                                &mut string.start,
                                &mut string.pointer,
                                &mut string.end,
                            ) != 0
                        {
                            1 as libc::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
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
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn yaml_parser_scan_uri_escapes(
    parser: *mut yaml_parser_t,
    directive: libc::c_int,
    start_mark: yaml_mark_t,
    string: *mut yaml_string_t,
) -> libc::c_int {
    let mut width: libc::c_int = 0 as libc::c_int;
    loop {
        if if (*parser).unread >= 3 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 3 as libc::c_int as size_t)
        } == 0
        {
            return 0 as libc::c_int;
        }
        if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '%' as i32 as yaml_char_t as libc::c_int
            && (*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                >= '0' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    <= '9' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    >= 'A' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        <= 'F' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    >= 'a' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                        as libc::c_int
                        <= 'f' as i32 as yaml_char_t as libc::c_int)
            && (*((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                >= '0' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                    <= '9' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                    >= 'A' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize)
                        as libc::c_int
                        <= 'F' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                    >= 'a' as i32 as yaml_char_t as libc::c_int
                    && *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize)
                        as libc::c_int
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
        let octet: libc::c_uchar = (((if *((*parser).buffer.pointer)
            .c_offset(1 as libc::c_int as isize)
            as libc::c_int
            >= 'A' as i32 as yaml_char_t as libc::c_int
            && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                <= 'F' as i32 as yaml_char_t as libc::c_int
        {
            *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                - 'A' as i32 as yaml_char_t as libc::c_int
                + 10 as libc::c_int
        } else if *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
            >= 'a' as i32 as yaml_char_t as libc::c_int
            && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                <= 'f' as i32 as yaml_char_t as libc::c_int
        {
            *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                - 'a' as i32 as yaml_char_t as libc::c_int
                + 10 as libc::c_int
        } else {
            *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                - '0' as i32 as yaml_char_t as libc::c_int
        }) << 4 as libc::c_int)
            + (if *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                >= 'A' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                    <= 'F' as i32 as yaml_char_t as libc::c_int
            {
                *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                    - 'A' as i32 as yaml_char_t as libc::c_int
                    + 10 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                >= 'a' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                    <= 'f' as i32 as yaml_char_t as libc::c_int
            {
                *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                    - 'a' as i32 as yaml_char_t as libc::c_int
                    + 10 as libc::c_int
            } else {
                *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                    - '0' as i32 as yaml_char_t as libc::c_int
            })) as libc::c_uchar;
        if width == 0 {
            width = if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                1 as libc::c_int
            } else if octet as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
                2 as libc::c_int
            } else if octet as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
                3 as libc::c_int
            } else if octet as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
                4 as libc::c_int
            } else {
                0 as libc::c_int
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
        } else if octet as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int {
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
        let fresh368 = &mut (*string).pointer;
        let fresh369 = *fresh368;
        *fresh368 = (*fresh368).c_offset(1);
        *fresh369 = octet;
        let fresh370 = &mut (*parser).mark.index;
        *fresh370 = (*fresh370).wrapping_add(1);
        let fresh371 = &mut (*parser).mark.column;
        *fresh371 = (*fresh371).wrapping_add(1);
        let fresh372 = &mut (*parser).unread;
        *fresh372 = (*fresh372).wrapping_sub(1);
        let fresh373 = &mut (*parser).buffer.pointer;
        *fresh373 = (*fresh373).c_offset(
            (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        );
        let fresh374 = &mut (*parser).mark.index;
        *fresh374 = (*fresh374).wrapping_add(1);
        let fresh375 = &mut (*parser).mark.column;
        *fresh375 = (*fresh375).wrapping_add(1);
        let fresh376 = &mut (*parser).unread;
        *fresh376 = (*fresh376).wrapping_sub(1);
        let fresh377 = &mut (*parser).buffer.pointer;
        *fresh377 = (*fresh377).c_offset(
            (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        );
        let fresh378 = &mut (*parser).mark.index;
        *fresh378 = (*fresh378).wrapping_add(1);
        let fresh379 = &mut (*parser).mark.column;
        *fresh379 = (*fresh379).wrapping_add(1);
        let fresh380 = &mut (*parser).unread;
        *fresh380 = (*fresh380).wrapping_sub(1);
        let fresh381 = &mut (*parser).buffer.pointer;
        *fresh381 = (*fresh381).c_offset(
            (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int
                == 0xc0 as libc::c_int
            {
                2 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int
                == 0xe0 as libc::c_int
            {
                3 as libc::c_int
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf8 as libc::c_int
                == 0xf0 as libc::c_int
            {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as isize,
        );
        width -= 1;
        if !(width != 0) {
            break;
        }
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_block_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    literal: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut string: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut leading_break: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut trailing_breaks: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut chomping: libc::c_int = 0 as libc::c_int;
    let mut increment: libc::c_int = 0 as libc::c_int;
    let mut indent: libc::c_int = 0 as libc::c_int;
    let mut leading_blank: libc::c_int = 0 as libc::c_int;
    let mut trailing_blank: libc::c_int;
    string.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).c_offset(16 as libc::c_int as isize);
        memset(
            string.start as *mut libc::c_void,
            0 as libc::c_int,
            16 as libc::c_int as libc::c_ulong,
        );
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        leading_break.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
        if !(if !(leading_break.start).is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = (leading_break.start).c_offset(16 as libc::c_int as isize);
            memset(
                leading_break.start as *mut libc::c_void,
                0 as libc::c_int,
                16 as libc::c_int as libc::c_ulong,
            );
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
            if !(if !(trailing_breaks.start).is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = (trailing_breaks.start).c_offset(16 as libc::c_int as isize);
                memset(
                    trailing_breaks.start as *mut libc::c_void,
                    0 as libc::c_int,
                    16 as libc::c_int as libc::c_ulong,
                );
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                start_mark = (*parser).mark;
                let fresh382 = &mut (*parser).mark.index;
                *fresh382 = (*fresh382).wrapping_add(1);
                let fresh383 = &mut (*parser).mark.column;
                *fresh383 = (*fresh383).wrapping_add(1);
                let fresh384 = &mut (*parser).unread;
                *fresh384 = (*fresh384).wrapping_sub(1);
                let fresh385 = &mut (*parser).buffer.pointer;
                *fresh385 = (*fresh385).c_offset(
                    (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0x80 as libc::c_int
                        == 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xe0 as libc::c_int
                        == 0xc0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xf0 as libc::c_int
                        == 0xe0 as libc::c_int
                    {
                        3 as libc::c_int
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        & 0xf8 as libc::c_int
                        == 0xf0 as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                );
                if !(if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                    1 as libc::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                } == 0)
                {
                    if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        == '+' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '-' as i32 as yaml_char_t as libc::c_int
                    {
                        chomping = if *((*parser).buffer.pointer)
                            .c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '+' as i32 as yaml_char_t as libc::c_int
                        {
                            1 as libc::c_int
                        } else {
                            -(1 as libc::c_int)
                        };
                        let fresh386 = &mut (*parser).mark.index;
                        *fresh386 = (*fresh386).wrapping_add(1);
                        let fresh387 = &mut (*parser).mark.column;
                        *fresh387 = (*fresh387).wrapping_add(1);
                        let fresh388 = &mut (*parser).unread;
                        *fresh388 = (*fresh388).wrapping_sub(1);
                        let fresh389 = &mut (*parser).buffer.pointer;
                        *fresh389 = (*fresh389).c_offset(
                            (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0x80 as libc::c_int
                                == 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else if *((*parser).buffer.pointer)
                                .c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xe0 as libc::c_int
                                == 0xc0 as libc::c_int
                            {
                                2 as libc::c_int
                            } else if *((*parser).buffer.pointer)
                                .c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf0 as libc::c_int
                                == 0xe0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else if *((*parser).buffer.pointer)
                                .c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & 0xf8 as libc::c_int
                                == 0xf0 as libc::c_int
                            {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) as isize,
                        );
                        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                            1 as libc::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                        } == 0
                        {
                            current_block = 14984465786483313892;
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            >= '0' as i32 as yaml_char_t as libc::c_int
                            && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                <= '9' as i32 as yaml_char_t as libc::c_int
                        {
                            if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
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
                                increment = *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    - '0' as i32 as yaml_char_t as libc::c_int;
                                let fresh390 = &mut (*parser).mark.index;
                                *fresh390 = (*fresh390).wrapping_add(1);
                                let fresh391 = &mut (*parser).mark.column;
                                *fresh391 = (*fresh391).wrapping_add(1);
                                let fresh392 = &mut (*parser).unread;
                                *fresh392 = (*fresh392).wrapping_sub(1);
                                let fresh393 = &mut (*parser).buffer.pointer;
                                *fresh393 = (*fresh393).c_offset(
                                    (if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        & 0x80 as libc::c_int
                                        == 0 as libc::c_int
                                    {
                                        1 as libc::c_int
                                    } else if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        & 0xe0 as libc::c_int
                                        == 0xc0 as libc::c_int
                                    {
                                        2 as libc::c_int
                                    } else if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        & 0xf0 as libc::c_int
                                        == 0xe0 as libc::c_int
                                    {
                                        3 as libc::c_int
                                    } else if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        & 0xf8 as libc::c_int
                                        == 0xf0 as libc::c_int
                                    {
                                        4 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    }) as isize,
                                );
                                current_block = 11913429853522160501;
                            }
                        } else {
                            current_block = 11913429853522160501;
                        }
                    } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                        as libc::c_int
                        >= '0' as i32 as yaml_char_t as libc::c_int
                        && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            <= '9' as i32 as yaml_char_t as libc::c_int
                    {
                        if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
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
                            increment = *((*parser).buffer.pointer)
                                .c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                - '0' as i32 as yaml_char_t as libc::c_int;
                            let fresh394 = &mut (*parser).mark.index;
                            *fresh394 = (*fresh394).wrapping_add(1);
                            let fresh395 = &mut (*parser).mark.column;
                            *fresh395 = (*fresh395).wrapping_add(1);
                            let fresh396 = &mut (*parser).unread;
                            *fresh396 = (*fresh396).wrapping_sub(1);
                            let fresh397 = &mut (*parser).buffer.pointer;
                            *fresh397 = (*fresh397).c_offset(
                                (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0x80 as libc::c_int
                                    == 0 as libc::c_int
                                {
                                    1 as libc::c_int
                                } else if *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0xe0 as libc::c_int
                                    == 0xc0 as libc::c_int
                                {
                                    2 as libc::c_int
                                } else if *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0xf0 as libc::c_int
                                    == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else if *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as isize,
                            );
                            if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                                1 as libc::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                            } == 0
                            {
                                current_block = 14984465786483313892;
                            } else {
                                if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '+' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '-' as i32 as yaml_char_t as libc::c_int
                                {
                                    chomping = if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '+' as i32 as yaml_char_t as libc::c_int
                                    {
                                        1 as libc::c_int
                                    } else {
                                        -(1 as libc::c_int)
                                    };
                                    let fresh398 = &mut (*parser).mark.index;
                                    *fresh398 = (*fresh398).wrapping_add(1);
                                    let fresh399 = &mut (*parser).mark.column;
                                    *fresh399 = (*fresh399).wrapping_add(1);
                                    let fresh400 = &mut (*parser).unread;
                                    *fresh400 = (*fresh400).wrapping_sub(1);
                                    let fresh401 = &mut (*parser).buffer.pointer;
                                    *fresh401 = (*fresh401).c_offset(
                                        (if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0x80 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            1 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xe0 as libc::c_int
                                            == 0xc0 as libc::c_int
                                        {
                                            2 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf0 as libc::c_int
                                            == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf8 as libc::c_int
                                            == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }) as isize,
                                    );
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
                            if !(if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                                1 as libc::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                            } == 0)
                            {
                                loop {
                                    if !(*((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == ' ' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\t' as i32 as yaml_char_t as libc::c_int)
                                    {
                                        current_block = 4090602189656566074;
                                        break;
                                    }
                                    let fresh402 = &mut (*parser).mark.index;
                                    *fresh402 = (*fresh402).wrapping_add(1);
                                    let fresh403 = &mut (*parser).mark.column;
                                    *fresh403 = (*fresh403).wrapping_add(1);
                                    let fresh404 = &mut (*parser).unread;
                                    *fresh404 = (*fresh404).wrapping_sub(1);
                                    let fresh405 = &mut (*parser).buffer.pointer;
                                    *fresh405 = (*fresh405).c_offset(
                                        (if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0x80 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            1 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xe0 as libc::c_int
                                            == 0xc0 as libc::c_int
                                        {
                                            2 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf0 as libc::c_int
                                            == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf8 as libc::c_int
                                            == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }) as isize,
                                    );
                                    if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                                        1 as libc::c_int
                                    } else {
                                        yaml_parser_update_buffer(
                                            parser,
                                            1 as libc::c_int as size_t,
                                        )
                                    } == 0
                                    {
                                        current_block = 14984465786483313892;
                                        break;
                                    }
                                }
                                match current_block {
                                    14984465786483313892 => {}
                                    _ => {
                                        if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '#' as i32 as yaml_char_t as libc::c_int
                                        {
                                            loop {
                                                if *((*parser).buffer.pointer)
                                                    .c_offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -62i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 1 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -123i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 1 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 2 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -88i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 1 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 2 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -87i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == '\0' as i32 as yaml_char_t as libc::c_int
                                                {
                                                    current_block = 12997042908615822766;
                                                    break;
                                                }
                                                let fresh406 = &mut (*parser).mark.index;
                                                *fresh406 = (*fresh406).wrapping_add(1);
                                                let fresh407 = &mut (*parser).mark.column;
                                                *fresh407 = (*fresh407).wrapping_add(1);
                                                let fresh408 = &mut (*parser).unread;
                                                *fresh408 = (*fresh408).wrapping_sub(1);
                                                let fresh409 = &mut (*parser).buffer.pointer;
                                                *fresh409 = (*fresh409).c_offset(
                                                    (if *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        & 0x80 as libc::c_int
                                                        == 0 as libc::c_int
                                                    {
                                                        1 as libc::c_int
                                                    } else if *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        & 0xe0 as libc::c_int
                                                        == 0xc0 as libc::c_int
                                                    {
                                                        2 as libc::c_int
                                                    } else if *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        & 0xf0 as libc::c_int
                                                        == 0xe0 as libc::c_int
                                                    {
                                                        3 as libc::c_int
                                                    } else if *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        & 0xf8 as libc::c_int
                                                        == 0xf0 as libc::c_int
                                                    {
                                                        4 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    })
                                                        as isize,
                                                );
                                                if if (*parser).unread
                                                    >= 1 as libc::c_int as libc::c_ulong
                                                {
                                                    1 as libc::c_int
                                                } else {
                                                    yaml_parser_update_buffer(
                                                        parser,
                                                        1 as libc::c_int as size_t,
                                                    )
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
                                                    .c_offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == '\n' as i32 as yaml_char_t
                                                            as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -62i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 1 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -123i32 as yaml_char_t
                                                                as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 1 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t
                                                                as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 2 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -88i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -30i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 1 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -128i32 as yaml_char_t
                                                                as libc::c_int
                                                        && *((*parser).buffer.pointer).c_offset(
                                                            (0 as libc::c_int + 2 as libc::c_int)
                                                                as isize,
                                                        )
                                                            as libc::c_int
                                                            == -87i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(0 as libc::c_int as isize)
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
                                                        .c_offset(0 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == '\n' as i32 as yaml_char_t
                                                                as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == -62i32 as yaml_char_t as libc::c_int
                                                            && *((*parser).buffer.pointer).c_offset(
                                                                (0 as libc::c_int
                                                                    + 1 as libc::c_int)
                                                                    as isize,
                                                            )
                                                                as libc::c_int
                                                                == -123i32 as yaml_char_t
                                                                    as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == -30i32 as yaml_char_t as libc::c_int
                                                            && *((*parser).buffer.pointer).c_offset(
                                                                (0 as libc::c_int
                                                                    + 1 as libc::c_int)
                                                                    as isize,
                                                            )
                                                                as libc::c_int
                                                                == -128i32 as yaml_char_t
                                                                    as libc::c_int
                                                            && *((*parser).buffer.pointer).c_offset(
                                                                (0 as libc::c_int
                                                                    + 2 as libc::c_int)
                                                                    as isize,
                                                            )
                                                                as libc::c_int
                                                                == -88i32 as yaml_char_t
                                                                    as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            == -30i32 as yaml_char_t as libc::c_int
                                                            && *((*parser).buffer.pointer).c_offset(
                                                                (0 as libc::c_int
                                                                    + 1 as libc::c_int)
                                                                    as isize,
                                                            )
                                                                as libc::c_int
                                                                == -128i32 as yaml_char_t
                                                                    as libc::c_int
                                                            && *((*parser).buffer.pointer).c_offset(
                                                                (0 as libc::c_int
                                                                    + 2 as libc::c_int)
                                                                    as isize,
                                                            )
                                                                as libc::c_int
                                                                == -87i32 as yaml_char_t
                                                                    as libc::c_int
                                                    {
                                                        if if (*parser).unread
                                                            >= 2 as libc::c_int as libc::c_ulong
                                                        {
                                                            1 as libc::c_int
                                                        } else {
                                                            yaml_parser_update_buffer(
                                                                parser,
                                                                2 as libc::c_int as size_t,
                                                            )
                                                        } == 0
                                                        {
                                                            current_block = 14984465786483313892;
                                                        } else {
                                                            if *((*parser).buffer.pointer)
                                                                .c_offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\r' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                                && *((*parser).buffer.pointer)
                                                                    .c_offset(
                                                                        (0 as libc::c_int
                                                                            + 1 as libc::c_int)
                                                                            as isize,
                                                                    )
                                                                    as libc::c_int
                                                                    == '\n' as i32 as yaml_char_t
                                                                        as libc::c_int
                                                            {
                                                                let fresh410 =
                                                                    &mut (*parser).mark.index;
                                                                *fresh410 = (*fresh410
                                                                    as libc::c_ulong)
                                                                    .wrapping_add(
                                                                        2 as libc::c_int
                                                                            as libc::c_ulong,
                                                                    )
                                                                    as size_t
                                                                    as size_t;
                                                                (*parser).mark.column =
                                                                    0 as libc::c_int as size_t;
                                                                let fresh411 =
                                                                    &mut (*parser).mark.line;
                                                                *fresh411 =
                                                                    (*fresh411).wrapping_add(1);
                                                                let fresh412 =
                                                                    &mut (*parser).unread;
                                                                *fresh412 = (*fresh412
                                                                    as libc::c_ulong)
                                                                    .wrapping_sub(
                                                                        2 as libc::c_int
                                                                            as libc::c_ulong,
                                                                    )
                                                                    as size_t
                                                                    as size_t;
                                                                let fresh413 =
                                                                    &mut (*parser).buffer.pointer;
                                                                *fresh413 = (*fresh413).c_offset(
                                                                    2 as libc::c_int as isize,
                                                                );
                                                            } else if *((*parser).buffer.pointer)
                                                                .c_offset(0 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\r' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                                || *((*parser).buffer.pointer)
                                                                    .c_offset(
                                                                        0 as libc::c_int as isize,
                                                                    )
                                                                    as libc::c_int
                                                                    == '\n' as i32 as yaml_char_t
                                                                        as libc::c_int
                                                                || *((*parser).buffer.pointer)
                                                                    .c_offset(
                                                                        0 as libc::c_int as isize,
                                                                    )
                                                                    as libc::c_int
                                                                    == -62i32 as yaml_char_t
                                                                        as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .c_offset(
                                                                            (0 as libc::c_int
                                                                                + 1 as libc::c_int)
                                                                                as isize,
                                                                        )
                                                                        as libc::c_int
                                                                        == -123i32 as yaml_char_t
                                                                            as libc::c_int
                                                                || *((*parser).buffer.pointer)
                                                                    .c_offset(
                                                                        0 as libc::c_int as isize,
                                                                    )
                                                                    as libc::c_int
                                                                    == -30i32 as yaml_char_t
                                                                        as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .c_offset(
                                                                            (0 as libc::c_int
                                                                                + 1 as libc::c_int)
                                                                                as isize,
                                                                        )
                                                                        as libc::c_int
                                                                        == -128i32 as yaml_char_t
                                                                            as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .c_offset(
                                                                            (0 as libc::c_int
                                                                                + 2 as libc::c_int)
                                                                                as isize,
                                                                        )
                                                                        as libc::c_int
                                                                        == -88i32 as yaml_char_t
                                                                            as libc::c_int
                                                                || *((*parser).buffer.pointer)
                                                                    .c_offset(
                                                                        0 as libc::c_int as isize,
                                                                    )
                                                                    as libc::c_int
                                                                    == -30i32 as yaml_char_t
                                                                        as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .c_offset(
                                                                            (0 as libc::c_int
                                                                                + 1 as libc::c_int)
                                                                                as isize,
                                                                        )
                                                                        as libc::c_int
                                                                        == -128i32 as yaml_char_t
                                                                            as libc::c_int
                                                                    && *((*parser).buffer.pointer)
                                                                        .c_offset(
                                                                            (0 as libc::c_int
                                                                                + 2 as libc::c_int)
                                                                                as isize,
                                                                        )
                                                                        as libc::c_int
                                                                        == -87i32 as yaml_char_t
                                                                            as libc::c_int
                                                            {
                                                                let fresh414 =
                                                                    &mut (*parser).mark.index;
                                                                *fresh414 =
                                                                    (*fresh414).wrapping_add(1);
                                                                (*parser).mark.column =
                                                                    0 as libc::c_int as size_t;
                                                                let fresh415 =
                                                                    &mut (*parser).mark.line;
                                                                *fresh415 =
                                                                    (*fresh415).wrapping_add(1);
                                                                let fresh416 =
                                                                    &mut (*parser).unread;
                                                                *fresh416 =
                                                                    (*fresh416).wrapping_sub(1);
                                                                let fresh417 =
                                                                    &mut (*parser).buffer.pointer;
                                                                *fresh417 = (*fresh417).c_offset(
                                                                    (if *((*parser).buffer.pointer)
                                                                        .c_offset(
                                                                            0 as libc::c_int
                                                                                as isize,
                                                                        )
                                                                        as libc::c_int
                                                                        & 0x80 as libc::c_int
                                                                        == 0 as libc::c_int
                                                                    {
                                                                        1 as libc::c_int
                                                                    } else if *((*parser)
                                                                        .buffer
                                                                        .pointer)
                                                                        .c_offset(
                                                                            0 as libc::c_int
                                                                                as isize,
                                                                        )
                                                                        as libc::c_int
                                                                        & 0xe0 as libc::c_int
                                                                        == 0xc0 as libc::c_int
                                                                    {
                                                                        2 as libc::c_int
                                                                    } else if *((*parser)
                                                                        .buffer
                                                                        .pointer)
                                                                        .c_offset(
                                                                            0 as libc::c_int
                                                                                as isize,
                                                                        )
                                                                        as libc::c_int
                                                                        & 0xf0 as libc::c_int
                                                                        == 0xe0 as libc::c_int
                                                                    {
                                                                        3 as libc::c_int
                                                                    } else if *((*parser)
                                                                        .buffer
                                                                        .pointer)
                                                                        .c_offset(
                                                                            0 as libc::c_int
                                                                                as isize,
                                                                        )
                                                                        as libc::c_int
                                                                        & 0xf8 as libc::c_int
                                                                        == 0xf0 as libc::c_int
                                                                    {
                                                                        4 as libc::c_int
                                                                    } else {
                                                                        0 as libc::c_int
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
                                                                indent = if (*parser).indent
                                                                    >= 0 as libc::c_int
                                                                {
                                                                    (*parser).indent + increment
                                                                } else {
                                                                    increment
                                                                };
                                                            }
                                                            if !(yaml_parser_scan_block_scalar_breaks(
                                                                parser,
                                                                &mut indent,
                                                                &mut trailing_breaks,
                                                                start_mark,
                                                                &mut end_mark,
                                                            ) == 0)
                                                            {
                                                                if !(if (*parser).unread
                                                                    >= 1 as libc::c_int as libc::c_ulong
                                                                {
                                                                    1 as libc::c_int
                                                                } else {
                                                                    yaml_parser_update_buffer(
                                                                        parser,
                                                                        1 as libc::c_int as size_t,
                                                                    )
                                                                } == 0)
                                                                {
                                                                    's_281: loop {
                                                                        if !((*parser).mark.column as libc::c_int == indent
                                                                            && !(*((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == '\0' as i32 as yaml_char_t as libc::c_int))
                                                                        {
                                                                            current_block = 5793491756164225964;
                                                                            break;
                                                                        }
                                                                        trailing_blank = (*((*parser).buffer.pointer)
                                                                            .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                            == ' ' as i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == '\t' as i32 as yaml_char_t as libc::c_int)
                                                                            as libc::c_int;
                                                                        if literal == 0
                                                                            && *leading_break.start as libc::c_int == '\n' as i32
                                                                            && leading_blank == 0 && trailing_blank == 0
                                                                        {
                                                                            if *trailing_breaks.start as libc::c_int == '\0' as i32 {
                                                                                if if (string.pointer).c_offset(5 as libc::c_int as isize)
                                                                                    < string.end
                                                                                    || yaml_string_extend(
                                                                                        &mut string.start,
                                                                                        &mut string.pointer,
                                                                                        &mut string.end,
                                                                                    ) != 0
                                                                                {
                                                                                    1 as libc::c_int
                                                                                } else {
                                                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                                                    0 as libc::c_int
                                                                                } == 0
                                                                                {
                                                                                    current_block = 14984465786483313892;
                                                                                    break;
                                                                                }
                                                                                let fresh418 = string.pointer;
                                                                                string.pointer = (string.pointer).c_offset(1);
                                                                                *fresh418 = ' ' as i32 as yaml_char_t;
                                                                            }
                                                                            leading_break.pointer = leading_break.start;
                                                                            memset(
                                                                                leading_break.start as *mut libc::c_void,
                                                                                0 as libc::c_int,
                                                                                (leading_break.end).c_offset_from(leading_break.start)
                                                                                    as libc::c_long as libc::c_ulong,
                                                                            );
                                                                        } else {
                                                                            if if yaml_string_join(
                                                                                &mut string.start,
                                                                                &mut string.pointer,
                                                                                &mut string.end,
                                                                                &mut leading_break.start,
                                                                                &mut leading_break.pointer,
                                                                                &mut leading_break.end,
                                                                            ) != 0
                                                                            {
                                                                                leading_break.pointer = leading_break.start;
                                                                                1 as libc::c_int
                                                                            } else {
                                                                                (*parser).error = YAML_MEMORY_ERROR;
                                                                                0 as libc::c_int
                                                                            } == 0
                                                                            {
                                                                                current_block = 14984465786483313892;
                                                                                break;
                                                                            }
                                                                            leading_break.pointer = leading_break.start;
                                                                            memset(
                                                                                leading_break.start as *mut libc::c_void,
                                                                                0 as libc::c_int,
                                                                                (leading_break.end).c_offset_from(leading_break.start)
                                                                                    as libc::c_long as libc::c_ulong,
                                                                            );
                                                                        }
                                                                        if if yaml_string_join(
                                                                            &mut string.start,
                                                                            &mut string.pointer,
                                                                            &mut string.end,
                                                                            &mut trailing_breaks.start,
                                                                            &mut trailing_breaks.pointer,
                                                                            &mut trailing_breaks.end,
                                                                        ) != 0
                                                                        {
                                                                            trailing_breaks.pointer = trailing_breaks.start;
                                                                            1 as libc::c_int
                                                                        } else {
                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                            0 as libc::c_int
                                                                        } == 0
                                                                        {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                        trailing_breaks.pointer = trailing_breaks.start;
                                                                        memset(
                                                                            trailing_breaks.start as *mut libc::c_void,
                                                                            0 as libc::c_int,
                                                                            (trailing_breaks.end).c_offset_from(trailing_breaks.start)
                                                                                as libc::c_long as libc::c_ulong,
                                                                        );
                                                                        leading_blank = (*((*parser).buffer.pointer)
                                                                            .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                            == ' ' as i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == '\t' as i32 as yaml_char_t as libc::c_int)
                                                                            as libc::c_int;
                                                                        while !(*((*parser).buffer.pointer)
                                                                            .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == -62i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                                                                    as libc::c_int == -123i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == -30i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                                                                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                                                                    as libc::c_int == -88i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == -30i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                                                                    as libc::c_int == -128i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                                                                    as libc::c_int == -87i32 as yaml_char_t as libc::c_int
                                                                            || *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == '\0' as i32 as yaml_char_t as libc::c_int)
                                                                        {
                                                                            if if if (string.pointer).c_offset(5 as libc::c_int as isize)
                                                                                < string.end
                                                                                || yaml_string_extend(
                                                                                    &mut string.start,
                                                                                    &mut string.pointer,
                                                                                    &mut string.end,
                                                                                ) != 0
                                                                            {
                                                                                1 as libc::c_int
                                                                            } else {
                                                                                (*parser).error = YAML_MEMORY_ERROR;
                                                                                0 as libc::c_int
                                                                            } != 0
                                                                            {
                                                                                if *(*parser).buffer.pointer as libc::c_int
                                                                                    & 0x80 as libc::c_int == 0 as libc::c_int
                                                                                {
                                                                                    let fresh419 = &mut (*parser).buffer.pointer;
                                                                                    let fresh420 = *fresh419;
                                                                                    *fresh419 = (*fresh419).c_offset(1);
                                                                                    let fresh421 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh421 = *fresh420;
                                                                                } else if *(*parser).buffer.pointer as libc::c_int
                                                                                    & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                                                                                {
                                                                                    let fresh422 = &mut (*parser).buffer.pointer;
                                                                                    let fresh423 = *fresh422;
                                                                                    *fresh422 = (*fresh422).c_offset(1);
                                                                                    let fresh424 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh424 = *fresh423;
                                                                                    let fresh425 = &mut (*parser).buffer.pointer;
                                                                                    let fresh426 = *fresh425;
                                                                                    *fresh425 = (*fresh425).c_offset(1);
                                                                                    let fresh427 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh427 = *fresh426;
                                                                                } else if *(*parser).buffer.pointer as libc::c_int
                                                                                    & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                                                                                {
                                                                                    let fresh428 = &mut (*parser).buffer.pointer;
                                                                                    let fresh429 = *fresh428;
                                                                                    *fresh428 = (*fresh428).c_offset(1);
                                                                                    let fresh430 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh430 = *fresh429;
                                                                                    let fresh431 = &mut (*parser).buffer.pointer;
                                                                                    let fresh432 = *fresh431;
                                                                                    *fresh431 = (*fresh431).c_offset(1);
                                                                                    let fresh433 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh433 = *fresh432;
                                                                                    let fresh434 = &mut (*parser).buffer.pointer;
                                                                                    let fresh435 = *fresh434;
                                                                                    *fresh434 = (*fresh434).c_offset(1);
                                                                                    let fresh436 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh436 = *fresh435;
                                                                                } else if *(*parser).buffer.pointer as libc::c_int
                                                                                    & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                                                                                {
                                                                                    let fresh437 = &mut (*parser).buffer.pointer;
                                                                                    let fresh438 = *fresh437;
                                                                                    *fresh437 = (*fresh437).c_offset(1);
                                                                                    let fresh439 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh439 = *fresh438;
                                                                                    let fresh440 = &mut (*parser).buffer.pointer;
                                                                                    let fresh441 = *fresh440;
                                                                                    *fresh440 = (*fresh440).c_offset(1);
                                                                                    let fresh442 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh442 = *fresh441;
                                                                                    let fresh443 = &mut (*parser).buffer.pointer;
                                                                                    let fresh444 = *fresh443;
                                                                                    *fresh443 = (*fresh443).c_offset(1);
                                                                                    let fresh445 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh445 = *fresh444;
                                                                                    let fresh446 = &mut (*parser).buffer.pointer;
                                                                                    let fresh447 = *fresh446;
                                                                                    *fresh446 = (*fresh446).c_offset(1);
                                                                                    let fresh448 = string.pointer;
                                                                                    string.pointer = (string.pointer).c_offset(1);
                                                                                    *fresh448 = *fresh447;
                                                                                } else {};
                                                                                let fresh449 = &mut (*parser).mark.index;
                                                                                *fresh449 = (*fresh449).wrapping_add(1);
                                                                                let fresh450 = &mut (*parser).mark.column;
                                                                                *fresh450 = (*fresh450).wrapping_add(1);
                                                                                let fresh451 = &mut (*parser).unread;
                                                                                *fresh451 = (*fresh451).wrapping_sub(1);
                                                                                1 as libc::c_int
                                                                            } else {
                                                                                0 as libc::c_int
                                                                            } == 0
                                                                            {
                                                                                current_block = 14984465786483313892;
                                                                                break 's_281;
                                                                            }
                                                                            if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong
                                                                            {
                                                                                1 as libc::c_int
                                                                            } else {
                                                                                yaml_parser_update_buffer(
                                                                                    parser,
                                                                                    1 as libc::c_int as size_t,
                                                                                )
                                                                            } == 0
                                                                            {
                                                                                current_block = 14984465786483313892;
                                                                                break 's_281;
                                                                            }
                                                                        }
                                                                        if if (*parser).unread >= 2 as libc::c_int as libc::c_ulong
                                                                        {
                                                                            1 as libc::c_int
                                                                        } else {
                                                                            yaml_parser_update_buffer(
                                                                                parser,
                                                                                2 as libc::c_int as size_t,
                                                                            )
                                                                        } == 0
                                                                        {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                        if if if (leading_break.pointer)
                                                                            .c_offset(5 as libc::c_int as isize) < leading_break.end
                                                                            || yaml_string_extend(
                                                                                &mut leading_break.start,
                                                                                &mut leading_break.pointer,
                                                                                &mut leading_break.end,
                                                                            ) != 0
                                                                        {
                                                                            1 as libc::c_int
                                                                        } else {
                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                            0 as libc::c_int
                                                                        } != 0
                                                                        {
                                                                            if *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == '\r' as i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .c_offset(1 as libc::c_int as isize) as libc::c_int
                                                                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                                                            {
                                                                                let fresh452 = leading_break.pointer;
                                                                                leading_break.pointer = (leading_break.pointer).c_offset(1);
                                                                                *fresh452 = '\n' as i32 as yaml_char_t;
                                                                                let fresh453 = &mut (*parser).buffer.pointer;
                                                                                *fresh453 = (*fresh453).c_offset(2 as libc::c_int as isize);
                                                                                let fresh454 = &mut (*parser).mark.index;
                                                                                *fresh454 = (*fresh454 as libc::c_ulong)
                                                                                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                                                                    as size_t;
                                                                                (*parser).mark.column = 0 as libc::c_int as size_t;
                                                                                let fresh455 = &mut (*parser).mark.line;
                                                                                *fresh455 = (*fresh455).wrapping_add(1);
                                                                                let fresh456 = &mut (*parser).unread;
                                                                                *fresh456 = (*fresh456 as libc::c_ulong)
                                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
                                                                                    as size_t;
                                                                            } else if *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == '\r' as i32 as yaml_char_t as libc::c_int
                                                                                || *((*parser).buffer.pointer)
                                                                                    .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                                                            {
                                                                                let fresh457 = leading_break.pointer;
                                                                                leading_break.pointer = (leading_break.pointer).c_offset(1);
                                                                                *fresh457 = '\n' as i32 as yaml_char_t;
                                                                                let fresh458 = &mut (*parser).buffer.pointer;
                                                                                *fresh458 = (*fresh458).c_offset(1);
                                                                                let fresh459 = &mut (*parser).mark.index;
                                                                                *fresh459 = (*fresh459).wrapping_add(1);
                                                                                (*parser).mark.column = 0 as libc::c_int as size_t;
                                                                                let fresh460 = &mut (*parser).mark.line;
                                                                                *fresh460 = (*fresh460).wrapping_add(1);
                                                                                let fresh461 = &mut (*parser).unread;
                                                                                *fresh461 = (*fresh461).wrapping_sub(1);
                                                                            } else if *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == -62i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .c_offset(1 as libc::c_int as isize) as libc::c_int
                                                                                    == -123i32 as yaml_char_t as libc::c_int
                                                                            {
                                                                                let fresh462 = leading_break.pointer;
                                                                                leading_break.pointer = (leading_break.pointer).c_offset(1);
                                                                                *fresh462 = '\n' as i32 as yaml_char_t;
                                                                                let fresh463 = &mut (*parser).buffer.pointer;
                                                                                *fresh463 = (*fresh463).c_offset(2 as libc::c_int as isize);
                                                                                let fresh464 = &mut (*parser).mark.index;
                                                                                *fresh464 = (*fresh464).wrapping_add(1);
                                                                                (*parser).mark.column = 0 as libc::c_int as size_t;
                                                                                let fresh465 = &mut (*parser).mark.line;
                                                                                *fresh465 = (*fresh465).wrapping_add(1);
                                                                                let fresh466 = &mut (*parser).unread;
                                                                                *fresh466 = (*fresh466).wrapping_sub(1);
                                                                            } else if *((*parser).buffer.pointer)
                                                                                .c_offset(0 as libc::c_int as isize) as libc::c_int
                                                                                == -30i32 as yaml_char_t as libc::c_int
                                                                                && *((*parser).buffer.pointer)
                                                                                    .c_offset(1 as libc::c_int as isize) as libc::c_int
                                                                                    == -128i32 as yaml_char_t as libc::c_int
                                                                                && (*((*parser).buffer.pointer)
                                                                                    .c_offset(2 as libc::c_int as isize) as libc::c_int
                                                                                    == -88i32 as yaml_char_t as libc::c_int
                                                                                    || *((*parser).buffer.pointer)
                                                                                        .c_offset(2 as libc::c_int as isize) as libc::c_int
                                                                                        == -87i32 as yaml_char_t as libc::c_int)
                                                                            {
                                                                                let fresh467 = &mut (*parser).buffer.pointer;
                                                                                let fresh468 = *fresh467;
                                                                                *fresh467 = (*fresh467).c_offset(1);
                                                                                let fresh469 = leading_break.pointer;
                                                                                leading_break.pointer = (leading_break.pointer).c_offset(1);
                                                                                *fresh469 = *fresh468;
                                                                                let fresh470 = &mut (*parser).buffer.pointer;
                                                                                let fresh471 = *fresh470;
                                                                                *fresh470 = (*fresh470).c_offset(1);
                                                                                let fresh472 = leading_break.pointer;
                                                                                leading_break.pointer = (leading_break.pointer).c_offset(1);
                                                                                *fresh472 = *fresh471;
                                                                                let fresh473 = &mut (*parser).buffer.pointer;
                                                                                let fresh474 = *fresh473;
                                                                                *fresh473 = (*fresh473).c_offset(1);
                                                                                let fresh475 = leading_break.pointer;
                                                                                leading_break.pointer = (leading_break.pointer).c_offset(1);
                                                                                *fresh475 = *fresh474;
                                                                                let fresh476 = &mut (*parser).mark.index;
                                                                                *fresh476 = (*fresh476).wrapping_add(1);
                                                                                (*parser).mark.column = 0 as libc::c_int as size_t;
                                                                                let fresh477 = &mut (*parser).mark.line;
                                                                                *fresh477 = (*fresh477).wrapping_add(1);
                                                                                let fresh478 = &mut (*parser).unread;
                                                                                *fresh478 = (*fresh478).wrapping_sub(1);
                                                                            } else {};
                                                                            1 as libc::c_int
                                                                        } else {
                                                                            0 as libc::c_int
                                                                        } == 0
                                                                        {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                        if yaml_parser_scan_block_scalar_breaks(
                                                                            parser,
                                                                            &mut indent,
                                                                            &mut trailing_breaks,
                                                                            start_mark,
                                                                            &mut end_mark,
                                                                        ) == 0
                                                                        {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                    }
                                                                    match current_block {
                                                                        14984465786483313892 => {}
                                                                        _ => {
                                                                            if chomping != -(1 as libc::c_int) {
                                                                                if if yaml_string_join(
                                                                                    &mut string.start,
                                                                                    &mut string.pointer,
                                                                                    &mut string.end,
                                                                                    &mut leading_break.start,
                                                                                    &mut leading_break.pointer,
                                                                                    &mut leading_break.end,
                                                                                ) != 0
                                                                                {
                                                                                    leading_break.pointer = leading_break.start;
                                                                                    1 as libc::c_int
                                                                                } else {
                                                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                                                    0 as libc::c_int
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
                                                                                    if chomping == 1 as libc::c_int {
                                                                                        if if yaml_string_join(
                                                                                            &mut string.start,
                                                                                            &mut string.pointer,
                                                                                            &mut string.end,
                                                                                            &mut trailing_breaks.start,
                                                                                            &mut trailing_breaks.pointer,
                                                                                            &mut trailing_breaks.end,
                                                                                        ) != 0
                                                                                        {
                                                                                            trailing_breaks.pointer = trailing_breaks.start;
                                                                                            1 as libc::c_int
                                                                                        } else {
                                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                                            0 as libc::c_int
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
                                                                                                0 as libc::c_int,
                                                                                                mem::size_of::<yaml_token_t>() as libc::c_ulong,
                                                                                            );
                                                                                            (*token).type_0 = YAML_SCALAR_TOKEN;
                                                                                            (*token).start_mark = start_mark;
                                                                                            (*token).end_mark = end_mark;
                                                                                            let fresh479 = &mut (*token).data.scalar.value;
                                                                                            *fresh479 = string.start;
                                                                                            (*token)
                                                                                                .data
                                                                                                .scalar
                                                                                                .length = (string.pointer).c_offset_from(string.start)
                                                                                                as libc::c_long as size_t;
                                                                                            (*token)
                                                                                                .data
                                                                                                .scalar
                                                                                                .style = (if literal != 0 {
                                                                                                YAML_LITERAL_SCALAR_STYLE as libc::c_int
                                                                                            } else {
                                                                                                YAML_FOLDED_SCALAR_STYLE as libc::c_int
                                                                                            }) as yaml_scalar_style_t;
                                                                                            yaml_free(leading_break.start as *mut libc::c_void);
                                                                                            leading_break.end = ptr::null_mut::<yaml_char_t>();
                                                                                            leading_break.pointer = leading_break.end;
                                                                                            leading_break.start = leading_break.pointer;
                                                                                            yaml_free(trailing_breaks.start as *mut libc::c_void);
                                                                                            trailing_breaks.end = ptr::null_mut::<yaml_char_t>();
                                                                                            trailing_breaks.pointer = trailing_breaks.end;
                                                                                            trailing_breaks.start = trailing_breaks.pointer;
                                                                                            return 1 as libc::c_int;
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
    0 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_block_scalar_breaks(
    mut parser: *mut yaml_parser_t,
    indent: *mut libc::c_int,
    breaks: *mut yaml_string_t,
    start_mark: yaml_mark_t,
    end_mark: *mut yaml_mark_t,
) -> libc::c_int {
    let mut max_indent: libc::c_int = 0 as libc::c_int;
    *end_mark = (*parser).mark;
    loop {
        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
        } == 0
        {
            return 0 as libc::c_int;
        }
        while (*indent == 0 || ((*parser).mark.column as libc::c_int) < *indent)
            && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
        {
            let fresh480 = &mut (*parser).mark.index;
            *fresh480 = (*fresh480).wrapping_add(1);
            let fresh481 = &mut (*parser).mark.column;
            *fresh481 = (*fresh481).wrapping_add(1);
            let fresh482 = &mut (*parser).unread;
            *fresh482 = (*fresh482).wrapping_sub(1);
            let fresh483 = &mut (*parser).buffer.pointer;
            *fresh483 = (*fresh483).c_offset(
                (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xe0 as libc::c_int
                    == 0xc0 as libc::c_int
                {
                    2 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xf0 as libc::c_int
                    == 0xe0 as libc::c_int
                {
                    3 as libc::c_int
                } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                    as libc::c_int
                    & 0xf8 as libc::c_int
                    == 0xf0 as libc::c_int
                {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            );
            if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                1 as libc::c_int
            } else {
                yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
            } == 0
            {
                return 0 as libc::c_int;
            }
        }
        if (*parser).mark.column as libc::c_int > max_indent {
            max_indent = (*parser).mark.column as libc::c_int;
        }
        if (*indent == 0 || ((*parser).mark.column as libc::c_int) < *indent)
            && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
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
        if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer)
                    .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
        {
            break;
        }
        if if (*parser).unread >= 2 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            yaml_parser_update_buffer(parser, 2 as libc::c_int as size_t)
        } == 0
        {
            return 0 as libc::c_int;
        }
        if if if ((*breaks).pointer).c_offset(5 as libc::c_int as isize) < (*breaks).end
            || yaml_string_extend(
                &mut (*breaks).start,
                &mut (*breaks).pointer,
                &mut (*breaks).end,
            ) != 0
        {
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } != 0
        {
            if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let fresh484 = &mut (*breaks).pointer;
                let fresh485 = *fresh484;
                *fresh484 = (*fresh484).c_offset(1);
                *fresh485 = '\n' as i32 as yaml_char_t;
                let fresh486 = &mut (*parser).buffer.pointer;
                *fresh486 = (*fresh486).c_offset(2 as libc::c_int as isize);
                let fresh487 = &mut (*parser).mark.index;
                *fresh487 = (*fresh487 as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                (*parser).mark.column = 0 as libc::c_int as size_t;
                let fresh488 = &mut (*parser).mark.line;
                *fresh488 = (*fresh488).wrapping_add(1);
                let fresh489 = &mut (*parser).unread;
                *fresh489 = (*fresh489 as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let fresh490 = &mut (*breaks).pointer;
                let fresh491 = *fresh490;
                *fresh490 = (*fresh490).c_offset(1);
                *fresh491 = '\n' as i32 as yaml_char_t;
                let fresh492 = &mut (*parser).buffer.pointer;
                *fresh492 = (*fresh492).c_offset(1);
                let fresh493 = &mut (*parser).mark.index;
                *fresh493 = (*fresh493).wrapping_add(1);
                (*parser).mark.column = 0 as libc::c_int as size_t;
                let fresh494 = &mut (*parser).mark.line;
                *fresh494 = (*fresh494).wrapping_add(1);
                let fresh495 = &mut (*parser).unread;
                *fresh495 = (*fresh495).wrapping_sub(1);
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            {
                let fresh496 = &mut (*breaks).pointer;
                let fresh497 = *fresh496;
                *fresh496 = (*fresh496).c_offset(1);
                *fresh497 = '\n' as i32 as yaml_char_t;
                let fresh498 = &mut (*parser).buffer.pointer;
                *fresh498 = (*fresh498).c_offset(2 as libc::c_int as isize);
                let fresh499 = &mut (*parser).mark.index;
                *fresh499 = (*fresh499).wrapping_add(1);
                (*parser).mark.column = 0 as libc::c_int as size_t;
                let fresh500 = &mut (*parser).mark.line;
                *fresh500 = (*fresh500).wrapping_add(1);
                let fresh501 = &mut (*parser).unread;
                *fresh501 = (*fresh501).wrapping_sub(1);
            } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && (*((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
                    || *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize)
                        as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int)
            {
                let fresh502 = &mut (*parser).buffer.pointer;
                let fresh503 = *fresh502;
                *fresh502 = (*fresh502).c_offset(1);
                let fresh504 = &mut (*breaks).pointer;
                let fresh505 = *fresh504;
                *fresh504 = (*fresh504).c_offset(1);
                *fresh505 = *fresh503;
                let fresh506 = &mut (*parser).buffer.pointer;
                let fresh507 = *fresh506;
                *fresh506 = (*fresh506).c_offset(1);
                let fresh508 = &mut (*breaks).pointer;
                let fresh509 = *fresh508;
                *fresh508 = (*fresh508).c_offset(1);
                *fresh509 = *fresh507;
                let fresh510 = &mut (*parser).buffer.pointer;
                let fresh511 = *fresh510;
                *fresh510 = (*fresh510).c_offset(1);
                let fresh512 = &mut (*breaks).pointer;
                let fresh513 = *fresh512;
                *fresh512 = (*fresh512).c_offset(1);
                *fresh513 = *fresh511;
                let fresh514 = &mut (*parser).mark.index;
                *fresh514 = (*fresh514).wrapping_add(1);
                (*parser).mark.column = 0 as libc::c_int as size_t;
                let fresh515 = &mut (*parser).mark.line;
                *fresh515 = (*fresh515).wrapping_add(1);
                let fresh516 = &mut (*parser).unread;
                *fresh516 = (*fresh516).wrapping_sub(1);
            };
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        *end_mark = (*parser).mark;
    }
    if *indent == 0 {
        *indent = max_indent;
        if *indent < (*parser).indent + 1 as libc::c_int {
            *indent = (*parser).indent + 1 as libc::c_int;
        }
        if *indent < 1 as libc::c_int {
            *indent = 1 as libc::c_int;
        }
    }
    1 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_flow_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    single: libc::c_int,
) -> libc::c_int {
    let current_block: u64;
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut string: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut leading_break: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut trailing_breaks: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut whitespaces: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut leading_blanks: libc::c_int;
    string.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).c_offset(16 as libc::c_int as isize);
        memset(
            string.start as *mut libc::c_void,
            0 as libc::c_int,
            16 as libc::c_int as libc::c_ulong,
        );
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        leading_break.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
        if !(if !(leading_break.start).is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = (leading_break.start).c_offset(16 as libc::c_int as isize);
            memset(
                leading_break.start as *mut libc::c_void,
                0 as libc::c_int,
                16 as libc::c_int as libc::c_ulong,
            );
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
            if !(if !(trailing_breaks.start).is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = (trailing_breaks.start).c_offset(16 as libc::c_int as isize);
                memset(
                    trailing_breaks.start as *mut libc::c_void,
                    0 as libc::c_int,
                    16 as libc::c_int as libc::c_ulong,
                );
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                whitespaces.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
                if !(if !(whitespaces.start).is_null() {
                    whitespaces.pointer = whitespaces.start;
                    whitespaces.end = (whitespaces.start).c_offset(16 as libc::c_int as isize);
                    memset(
                        whitespaces.start as *mut libc::c_void,
                        0 as libc::c_int,
                        16 as libc::c_int as libc::c_ulong,
                    );
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    start_mark = (*parser).mark;
                    let fresh517 = &mut (*parser).mark.index;
                    *fresh517 = (*fresh517).wrapping_add(1);
                    let fresh518 = &mut (*parser).mark.column;
                    *fresh518 = (*fresh518).wrapping_add(1);
                    let fresh519 = &mut (*parser).unread;
                    *fresh519 = (*fresh519).wrapping_sub(1);
                    let fresh520 = &mut (*parser).buffer.pointer;
                    *fresh520 = (*fresh520).c_offset(
                        (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0x80 as libc::c_int
                            == 0 as libc::c_int
                        {
                            1 as libc::c_int
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xe0 as libc::c_int
                            == 0xc0 as libc::c_int
                        {
                            2 as libc::c_int
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf0 as libc::c_int
                            == 0xe0 as libc::c_int
                        {
                            3 as libc::c_int
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            & 0xf8 as libc::c_int
                            == 0xf0 as libc::c_int
                        {
                            4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    );
                    's_58: loop {
                        if if (*parser).unread >= 4 as libc::c_int as libc::c_ulong {
                            1 as libc::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 4 as libc::c_int as size_t)
                        } == 0
                        {
                            current_block = 8114179180390253173;
                            break;
                        }
                        if (*parser).mark.column == 0 as libc::c_int as libc::c_ulong
                            && (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset(2 as libc::c_int as isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int)
                            && (*((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize)
                                as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                                || (*((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 1 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 1 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 2 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 1 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 2 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -87i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
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
                        } else if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
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
                            if if (*parser).unread >= 2 as libc::c_int as libc::c_ulong {
                                1 as libc::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 2 as libc::c_int as size_t)
                            } == 0
                            {
                                current_block = 8114179180390253173;
                                break;
                            }
                            leading_blanks = 0 as libc::c_int;
                            while !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                                || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 1 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 1 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 2 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 1 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 2 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -87i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\0' as i32 as yaml_char_t as libc::c_int))
                            {
                                if single != 0
                                    && *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\'' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\'' as i32 as yaml_char_t as libc::c_int
                                {
                                    if if (string.pointer).c_offset(5 as libc::c_int as isize)
                                        < string.end
                                        || yaml_string_extend(
                                            &mut string.start,
                                            &mut string.pointer,
                                            &mut string.end,
                                        ) != 0
                                    {
                                        1 as libc::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as libc::c_int
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                    let fresh521 = string.pointer;
                                    string.pointer = (string.pointer).c_offset(1);
                                    *fresh521 = '\'' as i32 as yaml_char_t;
                                    let fresh522 = &mut (*parser).mark.index;
                                    *fresh522 = (*fresh522).wrapping_add(1);
                                    let fresh523 = &mut (*parser).mark.column;
                                    *fresh523 = (*fresh523).wrapping_add(1);
                                    let fresh524 = &mut (*parser).unread;
                                    *fresh524 = (*fresh524).wrapping_sub(1);
                                    let fresh525 = &mut (*parser).buffer.pointer;
                                    *fresh525 = (*fresh525).c_offset(
                                        (if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0x80 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            1 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xe0 as libc::c_int
                                            == 0xc0 as libc::c_int
                                        {
                                            2 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf0 as libc::c_int
                                            == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf8 as libc::c_int
                                            == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }) as isize,
                                    );
                                    let fresh526 = &mut (*parser).mark.index;
                                    *fresh526 = (*fresh526).wrapping_add(1);
                                    let fresh527 = &mut (*parser).mark.column;
                                    *fresh527 = (*fresh527).wrapping_add(1);
                                    let fresh528 = &mut (*parser).unread;
                                    *fresh528 = (*fresh528).wrapping_sub(1);
                                    let fresh529 = &mut (*parser).buffer.pointer;
                                    *fresh529 = (*fresh529).c_offset(
                                        (if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0x80 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            1 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xe0 as libc::c_int
                                            == 0xc0 as libc::c_int
                                        {
                                            2 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf0 as libc::c_int
                                            == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf8 as libc::c_int
                                            == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }) as isize,
                                    );
                                } else {
                                    if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                            as yaml_char_t
                                            as libc::c_int
                                    {
                                        break;
                                    }
                                    if single == 0
                                        && *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\\' as i32 as yaml_char_t as libc::c_int
                                        && (*((*parser).buffer.pointer)
                                            .c_offset(1 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 2 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 2 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int)
                                    {
                                        if if (*parser).unread >= 3 as libc::c_int as libc::c_ulong
                                        {
                                            1 as libc::c_int
                                        } else {
                                            yaml_parser_update_buffer(
                                                parser,
                                                3 as libc::c_int as size_t,
                                            )
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                        let fresh530 = &mut (*parser).mark.index;
                                        *fresh530 = (*fresh530).wrapping_add(1);
                                        let fresh531 = &mut (*parser).mark.column;
                                        *fresh531 = (*fresh531).wrapping_add(1);
                                        let fresh532 = &mut (*parser).unread;
                                        *fresh532 = (*fresh532).wrapping_sub(1);
                                        let fresh533 = &mut (*parser).buffer.pointer;
                                        *fresh533 = (*fresh533).c_offset(
                                            (if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0x80 as libc::c_int
                                                == 0 as libc::c_int
                                            {
                                                1 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xe0 as libc::c_int
                                                == 0xc0 as libc::c_int
                                            {
                                                2 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xf0 as libc::c_int
                                                == 0xe0 as libc::c_int
                                            {
                                                3 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xf8 as libc::c_int
                                                == 0xf0 as libc::c_int
                                            {
                                                4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            }) as isize,
                                        );
                                        if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer).c_offset(
                                                (0 as libc::c_int + 1 as libc::c_int) as isize,
                                            )
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh534 = &mut (*parser).mark.index;
                                            *fresh534 = (*fresh534 as libc::c_ulong)
                                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                as size_t
                                                as size_t;
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh535 = &mut (*parser).mark.line;
                                            *fresh535 = (*fresh535).wrapping_add(1);
                                            let fresh536 = &mut (*parser).unread;
                                            *fresh536 = (*fresh536 as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as size_t
                                                as size_t;
                                            let fresh537 = &mut (*parser).buffer.pointer;
                                            *fresh537 =
                                                (*fresh537).c_offset(2 as libc::c_int as isize);
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 2 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (0 as libc::c_int + 2 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh538 = &mut (*parser).mark.index;
                                            *fresh538 = (*fresh538).wrapping_add(1);
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh539 = &mut (*parser).mark.line;
                                            *fresh539 = (*fresh539).wrapping_add(1);
                                            let fresh540 = &mut (*parser).unread;
                                            *fresh540 = (*fresh540).wrapping_sub(1);
                                            let fresh541 = &mut (*parser).buffer.pointer;
                                            *fresh541 = (*fresh541).c_offset(
                                                (if *((*parser).buffer.pointer)
                                                    .c_offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    & 0x80 as libc::c_int
                                                    == 0 as libc::c_int
                                                {
                                                    1 as libc::c_int
                                                } else if *((*parser).buffer.pointer)
                                                    .c_offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    & 0xe0 as libc::c_int
                                                    == 0xc0 as libc::c_int
                                                {
                                                    2 as libc::c_int
                                                } else if *((*parser).buffer.pointer)
                                                    .c_offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    & 0xf0 as libc::c_int
                                                    == 0xe0 as libc::c_int
                                                {
                                                    3 as libc::c_int
                                                } else if *((*parser).buffer.pointer)
                                                    .c_offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    & 0xf8 as libc::c_int
                                                    == 0xf0 as libc::c_int
                                                {
                                                    4 as libc::c_int
                                                } else {
                                                    0 as libc::c_int
                                                })
                                                    as isize,
                                            );
                                        };
                                        leading_blanks = 1 as libc::c_int;
                                        break;
                                    } else if single == 0
                                        && *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\\' as i32 as yaml_char_t as libc::c_int
                                    {
                                        let mut code_length: size_t = 0 as libc::c_int as size_t;
                                        if if (string.pointer).c_offset(5 as libc::c_int as isize)
                                            < string.end
                                            || yaml_string_extend(
                                                &mut string.start,
                                                &mut string.pointer,
                                                &mut string.end,
                                            ) != 0
                                        {
                                            1 as libc::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as libc::c_int
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                        match *((*parser).buffer.pointer)
                                            .c_offset(1 as libc::c_int as isize)
                                            as libc::c_int
                                        {
                                            48 => {
                                                let fresh542 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh542 = '\0' as i32 as yaml_char_t;
                                            }
                                            97 => {
                                                let fresh543 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh543 = '\u{7}' as i32 as yaml_char_t;
                                            }
                                            98 => {
                                                let fresh544 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh544 = '\u{8}' as i32 as yaml_char_t;
                                            }
                                            116 | 9 => {
                                                let fresh545 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh545 = '\t' as i32 as yaml_char_t;
                                            }
                                            110 => {
                                                let fresh546 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh546 = '\n' as i32 as yaml_char_t;
                                            }
                                            118 => {
                                                let fresh547 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh547 = '\u{b}' as i32 as yaml_char_t;
                                            }
                                            102 => {
                                                let fresh548 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh548 = '\u{c}' as i32 as yaml_char_t;
                                            }
                                            114 => {
                                                let fresh549 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh549 = '\r' as i32 as yaml_char_t;
                                            }
                                            101 => {
                                                let fresh550 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh550 = '\u{1b}' as i32 as yaml_char_t;
                                            }
                                            32 => {
                                                let fresh551 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh551 = ' ' as i32 as yaml_char_t;
                                            }
                                            34 => {
                                                let fresh552 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh552 = '"' as i32 as yaml_char_t;
                                            }
                                            47 => {
                                                let fresh553 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh553 = '/' as i32 as yaml_char_t;
                                            }
                                            92 => {
                                                let fresh554 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh554 = '\\' as i32 as yaml_char_t;
                                            }
                                            78 => {
                                                let fresh555 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh555 = -62i32 as yaml_char_t;
                                                let fresh556 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh556 = -123i32 as yaml_char_t;
                                            }
                                            95 => {
                                                let fresh557 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh557 = -62i32 as yaml_char_t;
                                                let fresh558 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh558 = -96i32 as yaml_char_t;
                                            }
                                            76 => {
                                                let fresh559 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh559 = -30i32 as yaml_char_t;
                                                let fresh560 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh560 = -128i32 as yaml_char_t;
                                                let fresh561 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh561 = -88i32 as yaml_char_t;
                                            }
                                            80 => {
                                                let fresh562 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh562 = -30i32 as yaml_char_t;
                                                let fresh563 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh563 = -128i32 as yaml_char_t;
                                                let fresh564 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh564 = -87i32 as yaml_char_t;
                                            }
                                            120 => {
                                                code_length = 2 as libc::c_int as size_t;
                                            }
                                            117 => {
                                                code_length = 4 as libc::c_int as size_t;
                                            }
                                            85 => {
                                                code_length = 8 as libc::c_int as size_t;
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
                                        let fresh565 = &mut (*parser).mark.index;
                                        *fresh565 = (*fresh565).wrapping_add(1);
                                        let fresh566 = &mut (*parser).mark.column;
                                        *fresh566 = (*fresh566).wrapping_add(1);
                                        let fresh567 = &mut (*parser).unread;
                                        *fresh567 = (*fresh567).wrapping_sub(1);
                                        let fresh568 = &mut (*parser).buffer.pointer;
                                        *fresh568 = (*fresh568).c_offset(
                                            (if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0x80 as libc::c_int
                                                == 0 as libc::c_int
                                            {
                                                1 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xe0 as libc::c_int
                                                == 0xc0 as libc::c_int
                                            {
                                                2 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xf0 as libc::c_int
                                                == 0xe0 as libc::c_int
                                            {
                                                3 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xf8 as libc::c_int
                                                == 0xf0 as libc::c_int
                                            {
                                                4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            }) as isize,
                                        );
                                        let fresh569 = &mut (*parser).mark.index;
                                        *fresh569 = (*fresh569).wrapping_add(1);
                                        let fresh570 = &mut (*parser).mark.column;
                                        *fresh570 = (*fresh570).wrapping_add(1);
                                        let fresh571 = &mut (*parser).unread;
                                        *fresh571 = (*fresh571).wrapping_sub(1);
                                        let fresh572 = &mut (*parser).buffer.pointer;
                                        *fresh572 = (*fresh572).c_offset(
                                            (if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0x80 as libc::c_int
                                                == 0 as libc::c_int
                                            {
                                                1 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xe0 as libc::c_int
                                                == 0xc0 as libc::c_int
                                            {
                                                2 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xf0 as libc::c_int
                                                == 0xe0 as libc::c_int
                                            {
                                                3 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xf8 as libc::c_int
                                                == 0xf0 as libc::c_int
                                            {
                                                4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            }) as isize,
                                        );
                                        if code_length != 0 {
                                            let mut value: libc::c_uint =
                                                0 as libc::c_int as libc::c_uint;
                                            let mut k: size_t;
                                            if if (*parser).unread >= code_length {
                                                1 as libc::c_int
                                            } else {
                                                yaml_parser_update_buffer(parser, code_length)
                                            } == 0
                                            {
                                                current_block = 8114179180390253173;
                                                break 's_58;
                                            }
                                            k = 0 as libc::c_int as size_t;
                                            while k < code_length {
                                                if !(*((*parser).buffer.pointer)
                                                    .c_offset(k as isize)
                                                    as libc::c_int
                                                    >= '0' as i32 as yaml_char_t as libc::c_int
                                                    && *((*parser).buffer.pointer)
                                                        .c_offset(k as isize)
                                                        as libc::c_int
                                                        <= '9' as i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(k as isize)
                                                        as libc::c_int
                                                        >= 'A' as i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .c_offset(k as isize)
                                                            as libc::c_int
                                                            <= 'F' as i32 as yaml_char_t
                                                                as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(k as isize)
                                                        as libc::c_int
                                                        >= 'a' as i32 as yaml_char_t as libc::c_int
                                                        && *((*parser).buffer.pointer)
                                                            .c_offset(k as isize)
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
                                                    value = (value << 4 as libc::c_int)
                                                        .wrapping_add(
                                                            (if *((*parser).buffer.pointer)
                                                                .c_offset(k as isize)
                                                                as libc::c_int
                                                                >= 'A' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                                && *((*parser).buffer.pointer)
                                                                    .c_offset(k as isize)
                                                                    as libc::c_int
                                                                    <= 'F' as i32 as yaml_char_t
                                                                        as libc::c_int
                                                            {
                                                                *((*parser).buffer.pointer)
                                                                    .c_offset(k as isize)
                                                                    as libc::c_int
                                                                    - 'A' as i32 as yaml_char_t
                                                                        as libc::c_int
                                                                    + 10 as libc::c_int
                                                            } else if *((*parser).buffer.pointer)
                                                                .c_offset(k as isize)
                                                                as libc::c_int
                                                                >= 'a' as i32 as yaml_char_t
                                                                    as libc::c_int
                                                                && *((*parser).buffer.pointer)
                                                                    .c_offset(k as isize)
                                                                    as libc::c_int
                                                                    <= 'f' as i32 as yaml_char_t
                                                                        as libc::c_int
                                                            {
                                                                *((*parser).buffer.pointer)
                                                                    .c_offset(k as isize)
                                                                    as libc::c_int
                                                                    - 'a' as i32 as yaml_char_t
                                                                        as libc::c_int
                                                                    + 10 as libc::c_int
                                                            } else {
                                                                *((*parser).buffer.pointer)
                                                                    .c_offset(k as isize)
                                                                    as libc::c_int
                                                                    - '0' as i32 as yaml_char_t
                                                                        as libc::c_int
                                                            })
                                                                as libc::c_uint,
                                                        );
                                                    k = k.wrapping_add(1);
                                                }
                                            }
                                            if value >= 0xd800 as libc::c_int as libc::c_uint
                                                && value <= 0xdfff as libc::c_int as libc::c_uint
                                                || value > 0x10ffff as libc::c_int as libc::c_uint
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
                                                if value <= 0x7f as libc::c_int as libc::c_uint {
                                                    let fresh573 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh573 = value as yaml_char_t;
                                                } else if value
                                                    <= 0x7ff as libc::c_int as libc::c_uint
                                                {
                                                    let fresh574 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh574 = (0xc0 as libc::c_int
                                                        as libc::c_uint)
                                                        .wrapping_add(value >> 6 as libc::c_int)
                                                        as yaml_char_t;
                                                    let fresh575 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh575 = (0x80 as libc::c_int
                                                        as libc::c_uint)
                                                        .wrapping_add(
                                                            value
                                                                & 0x3f as libc::c_int
                                                                    as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                } else if value
                                                    <= 0xffff as libc::c_int as libc::c_uint
                                                {
                                                    let fresh576 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh576 = (0xe0 as libc::c_int
                                                        as libc::c_uint)
                                                        .wrapping_add(value >> 12 as libc::c_int)
                                                        as yaml_char_t;
                                                    let fresh577 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh577 = (0x80 as libc::c_int
                                                        as libc::c_uint)
                                                        .wrapping_add(
                                                            value >> 6 as libc::c_int
                                                                & 0x3f as libc::c_int
                                                                    as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh578 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh578 = (0x80 as libc::c_int
                                                        as libc::c_uint)
                                                        .wrapping_add(
                                                            value
                                                                & 0x3f as libc::c_int
                                                                    as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                } else {
                                                    let fresh579 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh579 = (0xf0 as libc::c_int
                                                        as libc::c_uint)
                                                        .wrapping_add(value >> 18 as libc::c_int)
                                                        as yaml_char_t;
                                                    let fresh580 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh580 = (0x80 as libc::c_int
                                                        as libc::c_uint)
                                                        .wrapping_add(
                                                            value >> 12 as libc::c_int
                                                                & 0x3f as libc::c_int
                                                                    as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh581 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh581 = (0x80 as libc::c_int
                                                        as libc::c_uint)
                                                        .wrapping_add(
                                                            value >> 6 as libc::c_int
                                                                & 0x3f as libc::c_int
                                                                    as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh582 = string.pointer;
                                                    string.pointer = (string.pointer).c_offset(1);
                                                    *fresh582 = (0x80 as libc::c_int
                                                        as libc::c_uint)
                                                        .wrapping_add(
                                                            value
                                                                & 0x3f as libc::c_int
                                                                    as libc::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                }
                                                k = 0 as libc::c_int as size_t;
                                                while k < code_length {
                                                    let fresh583 = &mut (*parser).mark.index;
                                                    *fresh583 = (*fresh583).wrapping_add(1);
                                                    let fresh584 = &mut (*parser).mark.column;
                                                    *fresh584 = (*fresh584).wrapping_add(1);
                                                    let fresh585 = &mut (*parser).unread;
                                                    *fresh585 = (*fresh585).wrapping_sub(1);
                                                    let fresh586 = &mut (*parser).buffer.pointer;
                                                    *fresh586 = (*fresh586).c_offset(
                                                        (if *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            & 0x80 as libc::c_int
                                                            == 0 as libc::c_int
                                                        {
                                                            1 as libc::c_int
                                                        } else if *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            & 0xe0 as libc::c_int
                                                            == 0xc0 as libc::c_int
                                                        {
                                                            2 as libc::c_int
                                                        } else if *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            & 0xf0 as libc::c_int
                                                            == 0xe0 as libc::c_int
                                                        {
                                                            3 as libc::c_int
                                                        } else if *((*parser).buffer.pointer)
                                                            .c_offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                            & 0xf8 as libc::c_int
                                                            == 0xf0 as libc::c_int
                                                        {
                                                            4 as libc::c_int
                                                        } else {
                                                            0 as libc::c_int
                                                        })
                                                            as isize,
                                                    );
                                                    k = k.wrapping_add(1);
                                                }
                                            }
                                        }
                                    } else if if if (string.pointer)
                                        .c_offset(5 as libc::c_int as isize)
                                        < string.end
                                        || yaml_string_extend(
                                            &mut string.start,
                                            &mut string.pointer,
                                            &mut string.end,
                                        ) != 0
                                    {
                                        1 as libc::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as libc::c_int
                                    } != 0
                                    {
                                        if *(*parser).buffer.pointer as libc::c_int
                                            & 0x80 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            let fresh587 = &mut (*parser).buffer.pointer;
                                            let fresh588 = *fresh587;
                                            *fresh587 = (*fresh587).c_offset(1);
                                            let fresh589 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh589 = *fresh588;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xe0 as libc::c_int
                                            == 0xc0 as libc::c_int
                                        {
                                            let fresh590 = &mut (*parser).buffer.pointer;
                                            let fresh591 = *fresh590;
                                            *fresh590 = (*fresh590).c_offset(1);
                                            let fresh592 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh592 = *fresh591;
                                            let fresh593 = &mut (*parser).buffer.pointer;
                                            let fresh594 = *fresh593;
                                            *fresh593 = (*fresh593).c_offset(1);
                                            let fresh595 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh595 = *fresh594;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xf0 as libc::c_int
                                            == 0xe0 as libc::c_int
                                        {
                                            let fresh596 = &mut (*parser).buffer.pointer;
                                            let fresh597 = *fresh596;
                                            *fresh596 = (*fresh596).c_offset(1);
                                            let fresh598 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh598 = *fresh597;
                                            let fresh599 = &mut (*parser).buffer.pointer;
                                            let fresh600 = *fresh599;
                                            *fresh599 = (*fresh599).c_offset(1);
                                            let fresh601 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh601 = *fresh600;
                                            let fresh602 = &mut (*parser).buffer.pointer;
                                            let fresh603 = *fresh602;
                                            *fresh602 = (*fresh602).c_offset(1);
                                            let fresh604 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh604 = *fresh603;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xf8 as libc::c_int
                                            == 0xf0 as libc::c_int
                                        {
                                            let fresh605 = &mut (*parser).buffer.pointer;
                                            let fresh606 = *fresh605;
                                            *fresh605 = (*fresh605).c_offset(1);
                                            let fresh607 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh607 = *fresh606;
                                            let fresh608 = &mut (*parser).buffer.pointer;
                                            let fresh609 = *fresh608;
                                            *fresh608 = (*fresh608).c_offset(1);
                                            let fresh610 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh610 = *fresh609;
                                            let fresh611 = &mut (*parser).buffer.pointer;
                                            let fresh612 = *fresh611;
                                            *fresh611 = (*fresh611).c_offset(1);
                                            let fresh613 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh613 = *fresh612;
                                            let fresh614 = &mut (*parser).buffer.pointer;
                                            let fresh615 = *fresh614;
                                            *fresh614 = (*fresh614).c_offset(1);
                                            let fresh616 = string.pointer;
                                            string.pointer = (string.pointer).c_offset(1);
                                            *fresh616 = *fresh615;
                                        };
                                        let fresh617 = &mut (*parser).mark.index;
                                        *fresh617 = (*fresh617).wrapping_add(1);
                                        let fresh618 = &mut (*parser).mark.column;
                                        *fresh618 = (*fresh618).wrapping_add(1);
                                        let fresh619 = &mut (*parser).unread;
                                        *fresh619 = (*fresh619).wrapping_sub(1);
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                }
                                if if (*parser).unread >= 2 as libc::c_int as libc::c_ulong {
                                    1 as libc::c_int
                                } else {
                                    yaml_parser_update_buffer(parser, 2 as libc::c_int as size_t)
                                } == 0
                                {
                                    current_block = 8114179180390253173;
                                    break 's_58;
                                }
                            }
                            if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                                1 as libc::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                            } == 0
                            {
                                current_block = 8114179180390253173;
                                break;
                            }
                            if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                    as yaml_char_t as libc::c_int
                            {
                                current_block = 7468767852762055642;
                                break;
                            }
                            if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                                1 as libc::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                            } == 0
                            {
                                current_block = 8114179180390253173;
                                break;
                            }
                            while *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                                || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 1 as libc::c_int) as isize,
                                        ) as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 1 as libc::c_int) as isize,
                                        ) as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 2 as libc::c_int) as isize,
                                        ) as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 1 as libc::c_int) as isize,
                                        ) as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (0 as libc::c_int + 2 as libc::c_int) as isize,
                                        ) as libc::c_int
                                            == -87i32 as yaml_char_t as libc::c_int)
                            {
                                if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == ' ' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\t' as i32 as yaml_char_t as libc::c_int
                                {
                                    if leading_blanks == 0 {
                                        if if if (whitespaces.pointer)
                                            .c_offset(5 as libc::c_int as isize)
                                            < whitespaces.end
                                            || yaml_string_extend(
                                                &mut whitespaces.start,
                                                &mut whitespaces.pointer,
                                                &mut whitespaces.end,
                                            ) != 0
                                        {
                                            1 as libc::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as libc::c_int
                                        } != 0
                                        {
                                            if *(*parser).buffer.pointer as libc::c_int
                                                & 0x80 as libc::c_int
                                                == 0 as libc::c_int
                                            {
                                                let fresh620 = &mut (*parser).buffer.pointer;
                                                let fresh621 = *fresh620;
                                                *fresh620 = (*fresh620).c_offset(1);
                                                let fresh622 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh622 = *fresh621;
                                            } else if *(*parser).buffer.pointer as libc::c_int
                                                & 0xe0 as libc::c_int
                                                == 0xc0 as libc::c_int
                                            {
                                                let fresh623 = &mut (*parser).buffer.pointer;
                                                let fresh624 = *fresh623;
                                                *fresh623 = (*fresh623).c_offset(1);
                                                let fresh625 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh625 = *fresh624;
                                                let fresh626 = &mut (*parser).buffer.pointer;
                                                let fresh627 = *fresh626;
                                                *fresh626 = (*fresh626).c_offset(1);
                                                let fresh628 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh628 = *fresh627;
                                            } else if *(*parser).buffer.pointer as libc::c_int
                                                & 0xf0 as libc::c_int
                                                == 0xe0 as libc::c_int
                                            {
                                                let fresh629 = &mut (*parser).buffer.pointer;
                                                let fresh630 = *fresh629;
                                                *fresh629 = (*fresh629).c_offset(1);
                                                let fresh631 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh631 = *fresh630;
                                                let fresh632 = &mut (*parser).buffer.pointer;
                                                let fresh633 = *fresh632;
                                                *fresh632 = (*fresh632).c_offset(1);
                                                let fresh634 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh634 = *fresh633;
                                                let fresh635 = &mut (*parser).buffer.pointer;
                                                let fresh636 = *fresh635;
                                                *fresh635 = (*fresh635).c_offset(1);
                                                let fresh637 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh637 = *fresh636;
                                            } else if *(*parser).buffer.pointer as libc::c_int
                                                & 0xf8 as libc::c_int
                                                == 0xf0 as libc::c_int
                                            {
                                                let fresh638 = &mut (*parser).buffer.pointer;
                                                let fresh639 = *fresh638;
                                                *fresh638 = (*fresh638).c_offset(1);
                                                let fresh640 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh640 = *fresh639;
                                                let fresh641 = &mut (*parser).buffer.pointer;
                                                let fresh642 = *fresh641;
                                                *fresh641 = (*fresh641).c_offset(1);
                                                let fresh643 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh643 = *fresh642;
                                                let fresh644 = &mut (*parser).buffer.pointer;
                                                let fresh645 = *fresh644;
                                                *fresh644 = (*fresh644).c_offset(1);
                                                let fresh646 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh646 = *fresh645;
                                                let fresh647 = &mut (*parser).buffer.pointer;
                                                let fresh648 = *fresh647;
                                                *fresh647 = (*fresh647).c_offset(1);
                                                let fresh649 = whitespaces.pointer;
                                                whitespaces.pointer =
                                                    (whitespaces.pointer).c_offset(1);
                                                *fresh649 = *fresh648;
                                            };
                                            let fresh650 = &mut (*parser).mark.index;
                                            *fresh650 = (*fresh650).wrapping_add(1);
                                            let fresh651 = &mut (*parser).mark.column;
                                            *fresh651 = (*fresh651).wrapping_add(1);
                                            let fresh652 = &mut (*parser).unread;
                                            *fresh652 = (*fresh652).wrapping_sub(1);
                                            1 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                    } else {
                                        let fresh653 = &mut (*parser).mark.index;
                                        *fresh653 = (*fresh653).wrapping_add(1);
                                        let fresh654 = &mut (*parser).mark.column;
                                        *fresh654 = (*fresh654).wrapping_add(1);
                                        let fresh655 = &mut (*parser).unread;
                                        *fresh655 = (*fresh655).wrapping_sub(1);
                                        let fresh656 = &mut (*parser).buffer.pointer;
                                        *fresh656 = (*fresh656).c_offset(
                                            (if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0x80 as libc::c_int
                                                == 0 as libc::c_int
                                            {
                                                1 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xe0 as libc::c_int
                                                == 0xc0 as libc::c_int
                                            {
                                                2 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xf0 as libc::c_int
                                                == 0xe0 as libc::c_int
                                            {
                                                3 as libc::c_int
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                & 0xf8 as libc::c_int
                                                == 0xf0 as libc::c_int
                                            {
                                                4 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            }) as isize,
                                        );
                                    }
                                } else {
                                    if if (*parser).unread >= 2 as libc::c_int as libc::c_ulong {
                                        1 as libc::c_int
                                    } else {
                                        yaml_parser_update_buffer(
                                            parser,
                                            2 as libc::c_int as size_t,
                                        )
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                    if leading_blanks == 0 {
                                        whitespaces.pointer = whitespaces.start;
                                        memset(
                                            whitespaces.start as *mut libc::c_void,
                                            0 as libc::c_int,
                                            (whitespaces.end).c_offset_from(whitespaces.start)
                                                as libc::c_long
                                                as libc::c_ulong,
                                        );
                                        if if if (leading_break.pointer)
                                            .c_offset(5 as libc::c_int as isize)
                                            < leading_break.end
                                            || yaml_string_extend(
                                                &mut leading_break.start,
                                                &mut leading_break.pointer,
                                                &mut leading_break.end,
                                            ) != 0
                                        {
                                            1 as libc::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as libc::c_int
                                        } != 0
                                        {
                                            if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\r' as i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .c_offset(1 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                            {
                                                let fresh657 = leading_break.pointer;
                                                leading_break.pointer =
                                                    (leading_break.pointer).c_offset(1);
                                                *fresh657 = '\n' as i32 as yaml_char_t;
                                                let fresh658 = &mut (*parser).buffer.pointer;
                                                *fresh658 =
                                                    (*fresh658).c_offset(2 as libc::c_int as isize);
                                                let fresh659 = &mut (*parser).mark.index;
                                                *fresh659 = (*fresh659 as libc::c_ulong)
                                                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                    as size_t
                                                    as size_t;
                                                (*parser).mark.column = 0 as libc::c_int as size_t;
                                                let fresh660 = &mut (*parser).mark.line;
                                                *fresh660 = (*fresh660).wrapping_add(1);
                                                let fresh661 = &mut (*parser).unread;
                                                *fresh661 = (*fresh661 as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                    as size_t
                                                    as size_t;
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\r' as i32 as yaml_char_t as libc::c_int
                                                || *((*parser).buffer.pointer)
                                                    .c_offset(0 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                            {
                                                let fresh662 = leading_break.pointer;
                                                leading_break.pointer =
                                                    (leading_break.pointer).c_offset(1);
                                                *fresh662 = '\n' as i32 as yaml_char_t;
                                                let fresh663 = &mut (*parser).buffer.pointer;
                                                *fresh663 = (*fresh663).c_offset(1);
                                                let fresh664 = &mut (*parser).mark.index;
                                                *fresh664 = (*fresh664).wrapping_add(1);
                                                (*parser).mark.column = 0 as libc::c_int as size_t;
                                                let fresh665 = &mut (*parser).mark.line;
                                                *fresh665 = (*fresh665).wrapping_add(1);
                                                let fresh666 = &mut (*parser).unread;
                                                *fresh666 = (*fresh666).wrapping_sub(1);
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .c_offset(1 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            {
                                                let fresh667 = leading_break.pointer;
                                                leading_break.pointer =
                                                    (leading_break.pointer).c_offset(1);
                                                *fresh667 = '\n' as i32 as yaml_char_t;
                                                let fresh668 = &mut (*parser).buffer.pointer;
                                                *fresh668 =
                                                    (*fresh668).c_offset(2 as libc::c_int as isize);
                                                let fresh669 = &mut (*parser).mark.index;
                                                *fresh669 = (*fresh669).wrapping_add(1);
                                                (*parser).mark.column = 0 as libc::c_int as size_t;
                                                let fresh670 = &mut (*parser).mark.line;
                                                *fresh670 = (*fresh670).wrapping_add(1);
                                                let fresh671 = &mut (*parser).unread;
                                                *fresh671 = (*fresh671).wrapping_sub(1);
                                            } else if *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .c_offset(1 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && (*((*parser).buffer.pointer)
                                                    .c_offset(2 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                                    || *((*parser).buffer.pointer)
                                                        .c_offset(2 as libc::c_int as isize)
                                                        as libc::c_int
                                                        == -87i32 as yaml_char_t as libc::c_int)
                                            {
                                                let fresh672 = &mut (*parser).buffer.pointer;
                                                let fresh673 = *fresh672;
                                                *fresh672 = (*fresh672).c_offset(1);
                                                let fresh674 = leading_break.pointer;
                                                leading_break.pointer =
                                                    (leading_break.pointer).c_offset(1);
                                                *fresh674 = *fresh673;
                                                let fresh675 = &mut (*parser).buffer.pointer;
                                                let fresh676 = *fresh675;
                                                *fresh675 = (*fresh675).c_offset(1);
                                                let fresh677 = leading_break.pointer;
                                                leading_break.pointer =
                                                    (leading_break.pointer).c_offset(1);
                                                *fresh677 = *fresh676;
                                                let fresh678 = &mut (*parser).buffer.pointer;
                                                let fresh679 = *fresh678;
                                                *fresh678 = (*fresh678).c_offset(1);
                                                let fresh680 = leading_break.pointer;
                                                leading_break.pointer =
                                                    (leading_break.pointer).c_offset(1);
                                                *fresh680 = *fresh679;
                                                let fresh681 = &mut (*parser).mark.index;
                                                *fresh681 = (*fresh681).wrapping_add(1);
                                                (*parser).mark.column = 0 as libc::c_int as size_t;
                                                let fresh682 = &mut (*parser).mark.line;
                                                *fresh682 = (*fresh682).wrapping_add(1);
                                                let fresh683 = &mut (*parser).unread;
                                                *fresh683 = (*fresh683).wrapping_sub(1);
                                            };
                                            1 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                        leading_blanks = 1 as libc::c_int;
                                    } else if if if (trailing_breaks.pointer)
                                        .c_offset(5 as libc::c_int as isize)
                                        < trailing_breaks.end
                                        || yaml_string_extend(
                                            &mut trailing_breaks.start,
                                            &mut trailing_breaks.pointer,
                                            &mut trailing_breaks.end,
                                        ) != 0
                                    {
                                        1 as libc::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as libc::c_int
                                    } != 0
                                    {
                                        if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh684 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                (trailing_breaks.pointer).c_offset(1);
                                            *fresh684 = '\n' as i32 as yaml_char_t;
                                            let fresh685 = &mut (*parser).buffer.pointer;
                                            *fresh685 =
                                                (*fresh685).c_offset(2 as libc::c_int as isize);
                                            let fresh686 = &mut (*parser).mark.index;
                                            *fresh686 = (*fresh686 as libc::c_ulong)
                                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                as size_t
                                                as size_t;
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh687 = &mut (*parser).mark.line;
                                            *fresh687 = (*fresh687).wrapping_add(1);
                                            let fresh688 = &mut (*parser).unread;
                                            *fresh688 = (*fresh688 as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as size_t
                                                as size_t;
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh689 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                (trailing_breaks.pointer).c_offset(1);
                                            *fresh689 = '\n' as i32 as yaml_char_t;
                                            let fresh690 = &mut (*parser).buffer.pointer;
                                            *fresh690 = (*fresh690).c_offset(1);
                                            let fresh691 = &mut (*parser).mark.index;
                                            *fresh691 = (*fresh691).wrapping_add(1);
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh692 = &mut (*parser).mark.line;
                                            *fresh692 = (*fresh692).wrapping_add(1);
                                            let fresh693 = &mut (*parser).unread;
                                            *fresh693 = (*fresh693).wrapping_sub(1);
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -62i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -123i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh694 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                (trailing_breaks.pointer).c_offset(1);
                                            *fresh694 = '\n' as i32 as yaml_char_t;
                                            let fresh695 = &mut (*parser).buffer.pointer;
                                            *fresh695 =
                                                (*fresh695).c_offset(2 as libc::c_int as isize);
                                            let fresh696 = &mut (*parser).mark.index;
                                            *fresh696 = (*fresh696).wrapping_add(1);
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh697 = &mut (*parser).mark.line;
                                            *fresh697 = (*fresh697).wrapping_add(1);
                                            let fresh698 = &mut (*parser).unread;
                                            *fresh698 = (*fresh698).wrapping_sub(1);
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && (*((*parser).buffer.pointer)
                                                .c_offset(2 as libc::c_int as isize)
                                                as libc::c_int
                                                == -88i32 as yaml_char_t as libc::c_int
                                                || *((*parser).buffer.pointer)
                                                    .c_offset(2 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int)
                                        {
                                            let fresh699 = &mut (*parser).buffer.pointer;
                                            let fresh700 = *fresh699;
                                            *fresh699 = (*fresh699).c_offset(1);
                                            let fresh701 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                (trailing_breaks.pointer).c_offset(1);
                                            *fresh701 = *fresh700;
                                            let fresh702 = &mut (*parser).buffer.pointer;
                                            let fresh703 = *fresh702;
                                            *fresh702 = (*fresh702).c_offset(1);
                                            let fresh704 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                (trailing_breaks.pointer).c_offset(1);
                                            *fresh704 = *fresh703;
                                            let fresh705 = &mut (*parser).buffer.pointer;
                                            let fresh706 = *fresh705;
                                            *fresh705 = (*fresh705).c_offset(1);
                                            let fresh707 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                (trailing_breaks.pointer).c_offset(1);
                                            *fresh707 = *fresh706;
                                            let fresh708 = &mut (*parser).mark.index;
                                            *fresh708 = (*fresh708).wrapping_add(1);
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh709 = &mut (*parser).mark.line;
                                            *fresh709 = (*fresh709).wrapping_add(1);
                                            let fresh710 = &mut (*parser).unread;
                                            *fresh710 = (*fresh710).wrapping_sub(1);
                                        };
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                }
                                if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                                    1 as libc::c_int
                                } else {
                                    yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                                } == 0
                                {
                                    current_block = 8114179180390253173;
                                    break 's_58;
                                }
                            }
                            if leading_blanks != 0 {
                                if *(leading_break.start).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\n' as i32
                                {
                                    if *(trailing_breaks.start).c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\0' as i32
                                    {
                                        if if (string.pointer).c_offset(5 as libc::c_int as isize)
                                            < string.end
                                            || yaml_string_extend(
                                                &mut string.start,
                                                &mut string.pointer,
                                                &mut string.end,
                                            ) != 0
                                        {
                                            1 as libc::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as libc::c_int
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break;
                                        }
                                        let fresh711 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh711 = ' ' as i32 as yaml_char_t;
                                    } else {
                                        if if yaml_string_join(
                                            &mut string.start,
                                            &mut string.pointer,
                                            &mut string.end,
                                            &mut trailing_breaks.start,
                                            &mut trailing_breaks.pointer,
                                            &mut trailing_breaks.end,
                                        ) != 0
                                        {
                                            trailing_breaks.pointer = trailing_breaks.start;
                                            1 as libc::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as libc::c_int
                                        } == 0
                                        {
                                            current_block = 8114179180390253173;
                                            break;
                                        }
                                        trailing_breaks.pointer = trailing_breaks.start;
                                        memset(
                                            trailing_breaks.start as *mut libc::c_void,
                                            0 as libc::c_int,
                                            (trailing_breaks.end)
                                                .c_offset_from(trailing_breaks.start)
                                                as libc::c_long
                                                as libc::c_ulong,
                                        );
                                    }
                                    leading_break.pointer = leading_break.start;
                                    memset(
                                        leading_break.start as *mut libc::c_void,
                                        0 as libc::c_int,
                                        (leading_break.end).c_offset_from(leading_break.start)
                                            as libc::c_long
                                            as libc::c_ulong,
                                    );
                                } else {
                                    if if yaml_string_join(
                                        &mut string.start,
                                        &mut string.pointer,
                                        &mut string.end,
                                        &mut leading_break.start,
                                        &mut leading_break.pointer,
                                        &mut leading_break.end,
                                    ) != 0
                                    {
                                        leading_break.pointer = leading_break.start;
                                        1 as libc::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as libc::c_int
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break;
                                    }
                                    if if yaml_string_join(
                                        &mut string.start,
                                        &mut string.pointer,
                                        &mut string.end,
                                        &mut trailing_breaks.start,
                                        &mut trailing_breaks.pointer,
                                        &mut trailing_breaks.end,
                                    ) != 0
                                    {
                                        trailing_breaks.pointer = trailing_breaks.start;
                                        1 as libc::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as libc::c_int
                                    } == 0
                                    {
                                        current_block = 8114179180390253173;
                                        break;
                                    }
                                    leading_break.pointer = leading_break.start;
                                    memset(
                                        leading_break.start as *mut libc::c_void,
                                        0 as libc::c_int,
                                        (leading_break.end).c_offset_from(leading_break.start)
                                            as libc::c_long
                                            as libc::c_ulong,
                                    );
                                    trailing_breaks.pointer = trailing_breaks.start;
                                    memset(
                                        trailing_breaks.start as *mut libc::c_void,
                                        0 as libc::c_int,
                                        (trailing_breaks.end).c_offset_from(trailing_breaks.start)
                                            as libc::c_long
                                            as libc::c_ulong,
                                    );
                                }
                            } else {
                                if if yaml_string_join(
                                    &mut string.start,
                                    &mut string.pointer,
                                    &mut string.end,
                                    &mut whitespaces.start,
                                    &mut whitespaces.pointer,
                                    &mut whitespaces.end,
                                ) != 0
                                {
                                    whitespaces.pointer = whitespaces.start;
                                    1 as libc::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as libc::c_int
                                } == 0
                                {
                                    current_block = 8114179180390253173;
                                    break;
                                }
                                whitespaces.pointer = whitespaces.start;
                                memset(
                                    whitespaces.start as *mut libc::c_void,
                                    0 as libc::c_int,
                                    (whitespaces.end).c_offset_from(whitespaces.start)
                                        as libc::c_long
                                        as libc::c_ulong,
                                );
                            }
                        }
                    }
                    match current_block {
                        8114179180390253173 => {}
                        _ => {
                            let fresh712 = &mut (*parser).mark.index;
                            *fresh712 = (*fresh712).wrapping_add(1);
                            let fresh713 = &mut (*parser).mark.column;
                            *fresh713 = (*fresh713).wrapping_add(1);
                            let fresh714 = &mut (*parser).unread;
                            *fresh714 = (*fresh714).wrapping_sub(1);
                            let fresh715 = &mut (*parser).buffer.pointer;
                            *fresh715 = (*fresh715).c_offset(
                                (if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0x80 as libc::c_int
                                    == 0 as libc::c_int
                                {
                                    1 as libc::c_int
                                } else if *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0xe0 as libc::c_int
                                    == 0xc0 as libc::c_int
                                {
                                    2 as libc::c_int
                                } else if *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0xf0 as libc::c_int
                                    == 0xe0 as libc::c_int
                                {
                                    3 as libc::c_int
                                } else if *((*parser).buffer.pointer)
                                    .c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0xf8 as libc::c_int
                                    == 0xf0 as libc::c_int
                                {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as isize,
                            );
                            end_mark = (*parser).mark;
                            memset(
                                token as *mut libc::c_void,
                                0 as libc::c_int,
                                mem::size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_0 = YAML_SCALAR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh716 = &mut (*token).data.scalar.value;
                            *fresh716 = string.start;
                            (*token).data.scalar.length =
                                (string.pointer).c_offset_from(string.start) as libc::c_long
                                    as size_t;
                            (*token).data.scalar.style = (if single != 0 {
                                YAML_SINGLE_QUOTED_SCALAR_STYLE as libc::c_int
                            } else {
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE as libc::c_int
                            })
                                as yaml_scalar_style_t;
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
                            return 1 as libc::c_int;
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
    0 as libc::c_int
}
unsafe extern "C" fn yaml_parser_scan_plain_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> libc::c_int {
    let current_block: u64;
    let start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut string: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut leading_break: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut trailing_breaks: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut whitespaces: yaml_string_t = {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
    let mut leading_blanks: libc::c_int = 0 as libc::c_int;
    let indent: libc::c_int = (*parser).indent + 1 as libc::c_int;
    string.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).c_offset(16 as libc::c_int as isize);
        memset(
            string.start as *mut libc::c_void,
            0 as libc::c_int,
            16 as libc::c_int as libc::c_ulong,
        );
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        leading_break.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
        if !(if !(leading_break.start).is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = (leading_break.start).c_offset(16 as libc::c_int as isize);
            memset(
                leading_break.start as *mut libc::c_void,
                0 as libc::c_int,
                16 as libc::c_int as libc::c_ulong,
            );
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
            if !(if !(trailing_breaks.start).is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = (trailing_breaks.start).c_offset(16 as libc::c_int as isize);
                memset(
                    trailing_breaks.start as *mut libc::c_void,
                    0 as libc::c_int,
                    16 as libc::c_int as libc::c_ulong,
                );
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                whitespaces.start = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
                if !(if !(whitespaces.start).is_null() {
                    whitespaces.pointer = whitespaces.start;
                    whitespaces.end = (whitespaces.start).c_offset(16 as libc::c_int as isize);
                    memset(
                        whitespaces.start as *mut libc::c_void,
                        0 as libc::c_int,
                        16 as libc::c_int as libc::c_ulong,
                    );
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    end_mark = (*parser).mark;
                    start_mark = end_mark;
                    's_57: loop {
                        if if (*parser).unread >= 4 as libc::c_int as libc::c_ulong {
                            1 as libc::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 4 as libc::c_int as size_t)
                        } == 0
                        {
                            current_block = 16642808987012640029;
                            break;
                        }
                        if (*parser).mark.column == 0 as libc::c_int as libc::c_ulong
                            && (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).c_offset(2 as libc::c_int as isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset(2 as libc::c_int as isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int)
                            && (*((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize)
                                as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                                || (*((*parser).buffer.pointer).c_offset(3 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 1 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 1 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 2 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 1 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).c_offset(
                                            (3 as libc::c_int + 2 as libc::c_int) as isize,
                                        )
                                            as libc::c_int
                                            == -87i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(3 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\0' as i32 as yaml_char_t as libc::c_int))
                        {
                            current_block = 6281126495347172768;
                            break;
                        }
                        if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == '#' as i32 as yaml_char_t as libc::c_int
                        {
                            current_block = 6281126495347172768;
                            break;
                        }
                        while !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\0' as i32 as yaml_char_t as libc::c_int))
                        {
                            if (*parser).flow_level != 0
                                && *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == ':' as i32 as yaml_char_t as libc::c_int
                                && (*((*parser).buffer.pointer).c_offset(1 as libc::c_int as isize)
                                    as libc::c_int
                                    == ',' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                        == '?' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                        == '[' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                        == ']' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                        == '{' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer)
                                        .c_offset(1 as libc::c_int as isize)
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
                                if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == ':' as i32 as yaml_char_t as libc::c_int
                                    && (*((*parser).buffer.pointer)
                                        .c_offset(1 as libc::c_int as isize)
                                        as libc::c_int
                                        == ' ' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(1 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\t' as i32 as yaml_char_t as libc::c_int
                                        || (*((*parser).buffer.pointer)
                                            .c_offset(1 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 2 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 1 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer).c_offset(
                                                    (1 as libc::c_int + 2 as libc::c_int) as isize,
                                                )
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\0' as i32 as yaml_char_t as libc::c_int))
                                    || (*parser).flow_level != 0
                                        && (*((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == ',' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '[' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == ']' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '{' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '}' as i32 as yaml_char_t as libc::c_int)
                                {
                                    break;
                                }
                                if leading_blanks != 0 || whitespaces.start != whitespaces.pointer {
                                    if leading_blanks != 0 {
                                        if *(leading_break.start)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\n' as i32
                                        {
                                            if *(trailing_breaks.start)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\0' as i32
                                            {
                                                if if (string.pointer)
                                                    .c_offset(5 as libc::c_int as isize)
                                                    < string.end
                                                    || yaml_string_extend(
                                                        &mut string.start,
                                                        &mut string.pointer,
                                                        &mut string.end,
                                                    ) != 0
                                                {
                                                    1 as libc::c_int
                                                } else {
                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                    0 as libc::c_int
                                                } == 0
                                                {
                                                    current_block = 16642808987012640029;
                                                    break 's_57;
                                                }
                                                let fresh717 = string.pointer;
                                                string.pointer = (string.pointer).c_offset(1);
                                                *fresh717 = ' ' as i32 as yaml_char_t;
                                            } else {
                                                if if yaml_string_join(
                                                    &mut string.start,
                                                    &mut string.pointer,
                                                    &mut string.end,
                                                    &mut trailing_breaks.start,
                                                    &mut trailing_breaks.pointer,
                                                    &mut trailing_breaks.end,
                                                ) != 0
                                                {
                                                    trailing_breaks.pointer = trailing_breaks.start;
                                                    1 as libc::c_int
                                                } else {
                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                    0 as libc::c_int
                                                } == 0
                                                {
                                                    current_block = 16642808987012640029;
                                                    break 's_57;
                                                }
                                                trailing_breaks.pointer = trailing_breaks.start;
                                                memset(
                                                    trailing_breaks.start as *mut libc::c_void,
                                                    0 as libc::c_int,
                                                    (trailing_breaks.end)
                                                        .c_offset_from(trailing_breaks.start)
                                                        as libc::c_long
                                                        as libc::c_ulong,
                                                );
                                            }
                                            leading_break.pointer = leading_break.start;
                                            memset(
                                                leading_break.start as *mut libc::c_void,
                                                0 as libc::c_int,
                                                (leading_break.end)
                                                    .c_offset_from(leading_break.start)
                                                    as libc::c_long
                                                    as libc::c_ulong,
                                            );
                                        } else {
                                            if if yaml_string_join(
                                                &mut string.start,
                                                &mut string.pointer,
                                                &mut string.end,
                                                &mut leading_break.start,
                                                &mut leading_break.pointer,
                                                &mut leading_break.end,
                                            ) != 0
                                            {
                                                leading_break.pointer = leading_break.start;
                                                1 as libc::c_int
                                            } else {
                                                (*parser).error = YAML_MEMORY_ERROR;
                                                0 as libc::c_int
                                            } == 0
                                            {
                                                current_block = 16642808987012640029;
                                                break 's_57;
                                            }
                                            if if yaml_string_join(
                                                &mut string.start,
                                                &mut string.pointer,
                                                &mut string.end,
                                                &mut trailing_breaks.start,
                                                &mut trailing_breaks.pointer,
                                                &mut trailing_breaks.end,
                                            ) != 0
                                            {
                                                trailing_breaks.pointer = trailing_breaks.start;
                                                1 as libc::c_int
                                            } else {
                                                (*parser).error = YAML_MEMORY_ERROR;
                                                0 as libc::c_int
                                            } == 0
                                            {
                                                current_block = 16642808987012640029;
                                                break 's_57;
                                            }
                                            leading_break.pointer = leading_break.start;
                                            memset(
                                                leading_break.start as *mut libc::c_void,
                                                0 as libc::c_int,
                                                (leading_break.end)
                                                    .c_offset_from(leading_break.start)
                                                    as libc::c_long
                                                    as libc::c_ulong,
                                            );
                                            trailing_breaks.pointer = trailing_breaks.start;
                                            memset(
                                                trailing_breaks.start as *mut libc::c_void,
                                                0 as libc::c_int,
                                                (trailing_breaks.end)
                                                    .c_offset_from(trailing_breaks.start)
                                                    as libc::c_long
                                                    as libc::c_ulong,
                                            );
                                        }
                                        leading_blanks = 0 as libc::c_int;
                                    } else {
                                        if if yaml_string_join(
                                            &mut string.start,
                                            &mut string.pointer,
                                            &mut string.end,
                                            &mut whitespaces.start,
                                            &mut whitespaces.pointer,
                                            &mut whitespaces.end,
                                        ) != 0
                                        {
                                            whitespaces.pointer = whitespaces.start;
                                            1 as libc::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as libc::c_int
                                        } == 0
                                        {
                                            current_block = 16642808987012640029;
                                            break 's_57;
                                        }
                                        whitespaces.pointer = whitespaces.start;
                                        memset(
                                            whitespaces.start as *mut libc::c_void,
                                            0 as libc::c_int,
                                            (whitespaces.end).c_offset_from(whitespaces.start)
                                                as libc::c_long
                                                as libc::c_ulong,
                                        );
                                    }
                                }
                                if if if (string.pointer).c_offset(5 as libc::c_int as isize)
                                    < string.end
                                    || yaml_string_extend(
                                        &mut string.start,
                                        &mut string.pointer,
                                        &mut string.end,
                                    ) != 0
                                {
                                    1 as libc::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as libc::c_int
                                } != 0
                                {
                                    if *(*parser).buffer.pointer as libc::c_int
                                        & 0x80 as libc::c_int
                                        == 0 as libc::c_int
                                    {
                                        let fresh718 = &mut (*parser).buffer.pointer;
                                        let fresh719 = *fresh718;
                                        *fresh718 = (*fresh718).c_offset(1);
                                        let fresh720 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh720 = *fresh719;
                                    } else if *(*parser).buffer.pointer as libc::c_int
                                        & 0xe0 as libc::c_int
                                        == 0xc0 as libc::c_int
                                    {
                                        let fresh721 = &mut (*parser).buffer.pointer;
                                        let fresh722 = *fresh721;
                                        *fresh721 = (*fresh721).c_offset(1);
                                        let fresh723 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh723 = *fresh722;
                                        let fresh724 = &mut (*parser).buffer.pointer;
                                        let fresh725 = *fresh724;
                                        *fresh724 = (*fresh724).c_offset(1);
                                        let fresh726 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh726 = *fresh725;
                                    } else if *(*parser).buffer.pointer as libc::c_int
                                        & 0xf0 as libc::c_int
                                        == 0xe0 as libc::c_int
                                    {
                                        let fresh727 = &mut (*parser).buffer.pointer;
                                        let fresh728 = *fresh727;
                                        *fresh727 = (*fresh727).c_offset(1);
                                        let fresh729 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh729 = *fresh728;
                                        let fresh730 = &mut (*parser).buffer.pointer;
                                        let fresh731 = *fresh730;
                                        *fresh730 = (*fresh730).c_offset(1);
                                        let fresh732 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh732 = *fresh731;
                                        let fresh733 = &mut (*parser).buffer.pointer;
                                        let fresh734 = *fresh733;
                                        *fresh733 = (*fresh733).c_offset(1);
                                        let fresh735 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh735 = *fresh734;
                                    } else if *(*parser).buffer.pointer as libc::c_int
                                        & 0xf8 as libc::c_int
                                        == 0xf0 as libc::c_int
                                    {
                                        let fresh736 = &mut (*parser).buffer.pointer;
                                        let fresh737 = *fresh736;
                                        *fresh736 = (*fresh736).c_offset(1);
                                        let fresh738 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh738 = *fresh737;
                                        let fresh739 = &mut (*parser).buffer.pointer;
                                        let fresh740 = *fresh739;
                                        *fresh739 = (*fresh739).c_offset(1);
                                        let fresh741 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh741 = *fresh740;
                                        let fresh742 = &mut (*parser).buffer.pointer;
                                        let fresh743 = *fresh742;
                                        *fresh742 = (*fresh742).c_offset(1);
                                        let fresh744 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh744 = *fresh743;
                                        let fresh745 = &mut (*parser).buffer.pointer;
                                        let fresh746 = *fresh745;
                                        *fresh745 = (*fresh745).c_offset(1);
                                        let fresh747 = string.pointer;
                                        string.pointer = (string.pointer).c_offset(1);
                                        *fresh747 = *fresh746;
                                    };
                                    let fresh748 = &mut (*parser).mark.index;
                                    *fresh748 = (*fresh748).wrapping_add(1);
                                    let fresh749 = &mut (*parser).mark.column;
                                    *fresh749 = (*fresh749).wrapping_add(1);
                                    let fresh750 = &mut (*parser).unread;
                                    *fresh750 = (*fresh750).wrapping_sub(1);
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                } == 0
                                {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                                end_mark = (*parser).mark;
                                if if (*parser).unread >= 2 as libc::c_int as libc::c_ulong {
                                    1 as libc::c_int
                                } else {
                                    yaml_parser_update_buffer(parser, 2 as libc::c_int as size_t)
                                } == 0
                                {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                            }
                        }
                        if !(*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int))
                        {
                            current_block = 6281126495347172768;
                            break;
                        }
                        if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                            1 as libc::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
                        } == 0
                        {
                            current_block = 16642808987012640029;
                            break;
                        }
                        while *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ' ' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\t' as i32 as yaml_char_t as libc::c_int
                            || (*((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer)
                                        .c_offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int)
                        {
                            if *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == ' ' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer).c_offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == '\t' as i32 as yaml_char_t as libc::c_int
                            {
                                if leading_blanks != 0
                                    && ((*parser).mark.column as libc::c_int) < indent
                                    && *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
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
                                    if if if (whitespaces.pointer)
                                        .c_offset(5 as libc::c_int as isize)
                                        < whitespaces.end
                                        || yaml_string_extend(
                                            &mut whitespaces.start,
                                            &mut whitespaces.pointer,
                                            &mut whitespaces.end,
                                        ) != 0
                                    {
                                        1 as libc::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as libc::c_int
                                    } != 0
                                    {
                                        if *(*parser).buffer.pointer as libc::c_int
                                            & 0x80 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            let fresh751 = &mut (*parser).buffer.pointer;
                                            let fresh752 = *fresh751;
                                            *fresh751 = (*fresh751).c_offset(1);
                                            let fresh753 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh753 = *fresh752;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xe0 as libc::c_int
                                            == 0xc0 as libc::c_int
                                        {
                                            let fresh754 = &mut (*parser).buffer.pointer;
                                            let fresh755 = *fresh754;
                                            *fresh754 = (*fresh754).c_offset(1);
                                            let fresh756 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh756 = *fresh755;
                                            let fresh757 = &mut (*parser).buffer.pointer;
                                            let fresh758 = *fresh757;
                                            *fresh757 = (*fresh757).c_offset(1);
                                            let fresh759 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh759 = *fresh758;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xf0 as libc::c_int
                                            == 0xe0 as libc::c_int
                                        {
                                            let fresh760 = &mut (*parser).buffer.pointer;
                                            let fresh761 = *fresh760;
                                            *fresh760 = (*fresh760).c_offset(1);
                                            let fresh762 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh762 = *fresh761;
                                            let fresh763 = &mut (*parser).buffer.pointer;
                                            let fresh764 = *fresh763;
                                            *fresh763 = (*fresh763).c_offset(1);
                                            let fresh765 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh765 = *fresh764;
                                            let fresh766 = &mut (*parser).buffer.pointer;
                                            let fresh767 = *fresh766;
                                            *fresh766 = (*fresh766).c_offset(1);
                                            let fresh768 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh768 = *fresh767;
                                        } else if *(*parser).buffer.pointer as libc::c_int
                                            & 0xf8 as libc::c_int
                                            == 0xf0 as libc::c_int
                                        {
                                            let fresh769 = &mut (*parser).buffer.pointer;
                                            let fresh770 = *fresh769;
                                            *fresh769 = (*fresh769).c_offset(1);
                                            let fresh771 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh771 = *fresh770;
                                            let fresh772 = &mut (*parser).buffer.pointer;
                                            let fresh773 = *fresh772;
                                            *fresh772 = (*fresh772).c_offset(1);
                                            let fresh774 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh774 = *fresh773;
                                            let fresh775 = &mut (*parser).buffer.pointer;
                                            let fresh776 = *fresh775;
                                            *fresh775 = (*fresh775).c_offset(1);
                                            let fresh777 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh777 = *fresh776;
                                            let fresh778 = &mut (*parser).buffer.pointer;
                                            let fresh779 = *fresh778;
                                            *fresh778 = (*fresh778).c_offset(1);
                                            let fresh780 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).c_offset(1);
                                            *fresh780 = *fresh779;
                                        };
                                        let fresh781 = &mut (*parser).mark.index;
                                        *fresh781 = (*fresh781).wrapping_add(1);
                                        let fresh782 = &mut (*parser).mark.column;
                                        *fresh782 = (*fresh782).wrapping_add(1);
                                        let fresh783 = &mut (*parser).unread;
                                        *fresh783 = (*fresh783).wrapping_sub(1);
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    } == 0
                                    {
                                        current_block = 16642808987012640029;
                                        break 's_57;
                                    }
                                } else {
                                    let fresh784 = &mut (*parser).mark.index;
                                    *fresh784 = (*fresh784).wrapping_add(1);
                                    let fresh785 = &mut (*parser).mark.column;
                                    *fresh785 = (*fresh785).wrapping_add(1);
                                    let fresh786 = &mut (*parser).unread;
                                    *fresh786 = (*fresh786).wrapping_sub(1);
                                    let fresh787 = &mut (*parser).buffer.pointer;
                                    *fresh787 = (*fresh787).c_offset(
                                        (if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0x80 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            1 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xe0 as libc::c_int
                                            == 0xc0 as libc::c_int
                                        {
                                            2 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf0 as libc::c_int
                                            == 0xe0 as libc::c_int
                                        {
                                            3 as libc::c_int
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            & 0xf8 as libc::c_int
                                            == 0xf0 as libc::c_int
                                        {
                                            4 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }) as isize,
                                    );
                                }
                            } else {
                                if if (*parser).unread >= 2 as libc::c_int as libc::c_ulong {
                                    1 as libc::c_int
                                } else {
                                    yaml_parser_update_buffer(parser, 2 as libc::c_int as size_t)
                                } == 0
                                {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                                if leading_blanks == 0 {
                                    whitespaces.pointer = whitespaces.start;
                                    memset(
                                        whitespaces.start as *mut libc::c_void,
                                        0 as libc::c_int,
                                        (whitespaces.end).c_offset_from(whitespaces.start)
                                            as libc::c_long
                                            as libc::c_ulong,
                                    );
                                    if if if (leading_break.pointer)
                                        .c_offset(5 as libc::c_int as isize)
                                        < leading_break.end
                                        || yaml_string_extend(
                                            &mut leading_break.start,
                                            &mut leading_break.pointer,
                                            &mut leading_break.end,
                                        ) != 0
                                    {
                                        1 as libc::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as libc::c_int
                                    } != 0
                                    {
                                        if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh788 = leading_break.pointer;
                                            leading_break.pointer =
                                                (leading_break.pointer).c_offset(1);
                                            *fresh788 = '\n' as i32 as yaml_char_t;
                                            let fresh789 = &mut (*parser).buffer.pointer;
                                            *fresh789 =
                                                (*fresh789).c_offset(2 as libc::c_int as isize);
                                            let fresh790 = &mut (*parser).mark.index;
                                            *fresh790 = (*fresh790 as libc::c_ulong)
                                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                as size_t
                                                as size_t;
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh791 = &mut (*parser).mark.line;
                                            *fresh791 = (*fresh791).wrapping_add(1);
                                            let fresh792 = &mut (*parser).unread;
                                            *fresh792 = (*fresh792 as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as size_t
                                                as size_t;
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(0 as libc::c_int as isize)
                                                as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh793 = leading_break.pointer;
                                            leading_break.pointer =
                                                (leading_break.pointer).c_offset(1);
                                            *fresh793 = '\n' as i32 as yaml_char_t;
                                            let fresh794 = &mut (*parser).buffer.pointer;
                                            *fresh794 = (*fresh794).c_offset(1);
                                            let fresh795 = &mut (*parser).mark.index;
                                            *fresh795 = (*fresh795).wrapping_add(1);
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh796 = &mut (*parser).mark.line;
                                            *fresh796 = (*fresh796).wrapping_add(1);
                                            let fresh797 = &mut (*parser).unread;
                                            *fresh797 = (*fresh797).wrapping_sub(1);
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -62i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -123i32 as yaml_char_t as libc::c_int
                                        {
                                            let fresh798 = leading_break.pointer;
                                            leading_break.pointer =
                                                (leading_break.pointer).c_offset(1);
                                            *fresh798 = '\n' as i32 as yaml_char_t;
                                            let fresh799 = &mut (*parser).buffer.pointer;
                                            *fresh799 =
                                                (*fresh799).c_offset(2 as libc::c_int as isize);
                                            let fresh800 = &mut (*parser).mark.index;
                                            *fresh800 = (*fresh800).wrapping_add(1);
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh801 = &mut (*parser).mark.line;
                                            *fresh801 = (*fresh801).wrapping_add(1);
                                            let fresh802 = &mut (*parser).unread;
                                            *fresh802 = (*fresh802).wrapping_sub(1);
                                        } else if *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == -30i32 as yaml_char_t as libc::c_int
                                            && *((*parser).buffer.pointer)
                                                .c_offset(1 as libc::c_int as isize)
                                                as libc::c_int
                                                == -128i32 as yaml_char_t as libc::c_int
                                            && (*((*parser).buffer.pointer)
                                                .c_offset(2 as libc::c_int as isize)
                                                as libc::c_int
                                                == -88i32 as yaml_char_t as libc::c_int
                                                || *((*parser).buffer.pointer)
                                                    .c_offset(2 as libc::c_int as isize)
                                                    as libc::c_int
                                                    == -87i32 as yaml_char_t as libc::c_int)
                                        {
                                            let fresh803 = &mut (*parser).buffer.pointer;
                                            let fresh804 = *fresh803;
                                            *fresh803 = (*fresh803).c_offset(1);
                                            let fresh805 = leading_break.pointer;
                                            leading_break.pointer =
                                                (leading_break.pointer).c_offset(1);
                                            *fresh805 = *fresh804;
                                            let fresh806 = &mut (*parser).buffer.pointer;
                                            let fresh807 = *fresh806;
                                            *fresh806 = (*fresh806).c_offset(1);
                                            let fresh808 = leading_break.pointer;
                                            leading_break.pointer =
                                                (leading_break.pointer).c_offset(1);
                                            *fresh808 = *fresh807;
                                            let fresh809 = &mut (*parser).buffer.pointer;
                                            let fresh810 = *fresh809;
                                            *fresh809 = (*fresh809).c_offset(1);
                                            let fresh811 = leading_break.pointer;
                                            leading_break.pointer =
                                                (leading_break.pointer).c_offset(1);
                                            *fresh811 = *fresh810;
                                            let fresh812 = &mut (*parser).mark.index;
                                            *fresh812 = (*fresh812).wrapping_add(1);
                                            (*parser).mark.column = 0 as libc::c_int as size_t;
                                            let fresh813 = &mut (*parser).mark.line;
                                            *fresh813 = (*fresh813).wrapping_add(1);
                                            let fresh814 = &mut (*parser).unread;
                                            *fresh814 = (*fresh814).wrapping_sub(1);
                                        };
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    } == 0
                                    {
                                        current_block = 16642808987012640029;
                                        break 's_57;
                                    }
                                    leading_blanks = 1 as libc::c_int;
                                } else if if if (trailing_breaks.pointer)
                                    .c_offset(5 as libc::c_int as isize)
                                    < trailing_breaks.end
                                    || yaml_string_extend(
                                        &mut trailing_breaks.start,
                                        &mut trailing_breaks.pointer,
                                        &mut trailing_breaks.end,
                                    ) != 0
                                {
                                    1 as libc::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as libc::c_int
                                } != 0
                                {
                                    if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .c_offset(1 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\n' as i32 as yaml_char_t as libc::c_int
                                    {
                                        let fresh815 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            (trailing_breaks.pointer).c_offset(1);
                                        *fresh815 = '\n' as i32 as yaml_char_t;
                                        let fresh816 = &mut (*parser).buffer.pointer;
                                        *fresh816 = (*fresh816).c_offset(2 as libc::c_int as isize);
                                        let fresh817 = &mut (*parser).mark.index;
                                        *fresh817 = (*fresh817 as libc::c_ulong)
                                            .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                            as size_t
                                            as size_t;
                                        (*parser).mark.column = 0 as libc::c_int as size_t;
                                        let fresh818 = &mut (*parser).mark.line;
                                        *fresh818 = (*fresh818).wrapping_add(1);
                                        let fresh819 = &mut (*parser).unread;
                                        *fresh819 = (*fresh819 as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                            as size_t
                                            as size_t;
                                    } else if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                        || *((*parser).buffer.pointer)
                                            .c_offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == '\n' as i32 as yaml_char_t as libc::c_int
                                    {
                                        let fresh820 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            (trailing_breaks.pointer).c_offset(1);
                                        *fresh820 = '\n' as i32 as yaml_char_t;
                                        let fresh821 = &mut (*parser).buffer.pointer;
                                        *fresh821 = (*fresh821).c_offset(1);
                                        let fresh822 = &mut (*parser).mark.index;
                                        *fresh822 = (*fresh822).wrapping_add(1);
                                        (*parser).mark.column = 0 as libc::c_int as size_t;
                                        let fresh823 = &mut (*parser).mark.line;
                                        *fresh823 = (*fresh823).wrapping_add(1);
                                        let fresh824 = &mut (*parser).unread;
                                        *fresh824 = (*fresh824).wrapping_sub(1);
                                    } else if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .c_offset(1 as libc::c_int as isize)
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    {
                                        let fresh825 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            (trailing_breaks.pointer).c_offset(1);
                                        *fresh825 = '\n' as i32 as yaml_char_t;
                                        let fresh826 = &mut (*parser).buffer.pointer;
                                        *fresh826 = (*fresh826).c_offset(2 as libc::c_int as isize);
                                        let fresh827 = &mut (*parser).mark.index;
                                        *fresh827 = (*fresh827).wrapping_add(1);
                                        (*parser).mark.column = 0 as libc::c_int as size_t;
                                        let fresh828 = &mut (*parser).mark.line;
                                        *fresh828 = (*fresh828).wrapping_add(1);
                                        let fresh829 = &mut (*parser).unread;
                                        *fresh829 = (*fresh829).wrapping_sub(1);
                                    } else if *((*parser).buffer.pointer)
                                        .c_offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer)
                                            .c_offset(1 as libc::c_int as isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && (*((*parser).buffer.pointer)
                                            .c_offset(2 as libc::c_int as isize)
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer)
                                                .c_offset(2 as libc::c_int as isize)
                                                as libc::c_int
                                                == -87i32 as yaml_char_t as libc::c_int)
                                    {
                                        let fresh830 = &mut (*parser).buffer.pointer;
                                        let fresh831 = *fresh830;
                                        *fresh830 = (*fresh830).c_offset(1);
                                        let fresh832 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            (trailing_breaks.pointer).c_offset(1);
                                        *fresh832 = *fresh831;
                                        let fresh833 = &mut (*parser).buffer.pointer;
                                        let fresh834 = *fresh833;
                                        *fresh833 = (*fresh833).c_offset(1);
                                        let fresh835 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            (trailing_breaks.pointer).c_offset(1);
                                        *fresh835 = *fresh834;
                                        let fresh836 = &mut (*parser).buffer.pointer;
                                        let fresh837 = *fresh836;
                                        *fresh836 = (*fresh836).c_offset(1);
                                        let fresh838 = trailing_breaks.pointer;
                                        trailing_breaks.pointer =
                                            (trailing_breaks.pointer).c_offset(1);
                                        *fresh838 = *fresh837;
                                        let fresh839 = &mut (*parser).mark.index;
                                        *fresh839 = (*fresh839).wrapping_add(1);
                                        (*parser).mark.column = 0 as libc::c_int as size_t;
                                        let fresh840 = &mut (*parser).mark.line;
                                        *fresh840 = (*fresh840).wrapping_add(1);
                                        let fresh841 = &mut (*parser).unread;
                                        *fresh841 = (*fresh841).wrapping_sub(1);
                                    };
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                } == 0
                                {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                            }
                            if if (*parser).unread >= 1 as libc::c_int as libc::c_ulong {
                                1 as libc::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as libc::c_int as size_t)
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
                                0 as libc::c_int,
                                mem::size_of::<yaml_token_t>() as libc::c_ulong,
                            );
                            (*token).type_0 = YAML_SCALAR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            let fresh842 = &mut (*token).data.scalar.value;
                            *fresh842 = string.start;
                            (*token).data.scalar.length =
                                (string.pointer).c_offset_from(string.start) as libc::c_long
                                    as size_t;
                            (*token).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
                            if leading_blanks != 0 {
                                (*parser).simple_key_allowed = 1 as libc::c_int;
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
                            return 1 as libc::c_int;
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
    0 as libc::c_int
}
