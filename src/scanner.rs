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

macro_rules! CACHE {
    ($parser:expr, $length:expr) => {
        if (*$parser).unread >= $length {
            1_i32
        } else {
            yaml_parser_update_buffer($parser, $length)
        }
    };
}

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
            (if *((*$parser).buffer.pointer) as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if *((*$parser).buffer.pointer) as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                2_i32
            } else if *((*$parser).buffer.pointer) as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                3_i32
            } else if *((*$parser).buffer.pointer) as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                4_i32
            } else {
                0_i32
            }) as isize,
        );
    };
}

macro_rules! SKIP_LINE {
    ($parser:expr) => {
        if *((*$parser).buffer.pointer) as libc::c_int == '\r' as i32 as yaml_char_t as libc::c_int
            && *((*$parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
        {
            let index = addr_of_mut!((*$parser).mark.index);
            *index = (*index as libc::c_ulong).wrapping_add(2_u64) as size_t as size_t;
            (*$parser).mark.column = 0_u64;
            let line = addr_of_mut!((*$parser).mark.line);
            *line = (*line).wrapping_add(1);
            let unread = addr_of_mut!((*$parser).unread);
            *unread = (*unread as libc::c_ulong).wrapping_sub(2_u64) as size_t as size_t;
            let pointer = addr_of_mut!((*$parser).buffer.pointer);
            *pointer = (*pointer).wrapping_offset(2_isize);
        } else if *((*$parser).buffer.pointer) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*$parser).buffer.pointer) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*$parser).buffer.pointer) as libc::c_int == -62i32 as yaml_char_t as libc::c_int
                && *((*$parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*$parser).buffer.pointer) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                && *((*$parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*$parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*$parser).buffer.pointer) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                && *((*$parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*$parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int
        {
            let index = addr_of_mut!((*$parser).mark.index);
            *index = (*index).wrapping_add(1);
            (*$parser).mark.column = 0_u64;
            let line = addr_of_mut!((*$parser).mark.line);
            *line = (*line).wrapping_add(1);
            let unread = addr_of_mut!((*$parser).unread);
            *unread = (*unread).wrapping_sub(1);
            let pointer = addr_of_mut!((*$parser).buffer.pointer);
            *pointer = (*pointer).wrapping_offset(
                (if *((*$parser).buffer.pointer) as libc::c_int & 0x80_i32 == 0_i32 {
                    1_i32
                } else if *((*$parser).buffer.pointer) as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                    2_i32
                } else if *((*$parser).buffer.pointer) as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                    3_i32
                } else if *((*$parser).buffer.pointer) as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                    4_i32
                } else {
                    0_i32
                }) as isize,
            );
        };
    };
}

macro_rules! READ {
    ($parser:expr, $string:expr) => {
        if STRING_EXTEND!($parser, $string) != 0 {
            COPY!($string, (*$parser).buffer);
            let fresh160 = addr_of_mut!((*$parser).mark.index);
            *fresh160 = (*fresh160).wrapping_add(1);
            let fresh161 = addr_of_mut!((*$parser).mark.column);
            *fresh161 = (*fresh161).wrapping_add(1);
            let fresh162 = addr_of_mut!((*$parser).unread);
            *fresh162 = (*fresh162).wrapping_sub(1);
            1_i32
        } else {
            0_i32
        }
    };
}

macro_rules! READ_LINE {
    ($parser:expr, $string:expr) => {
        if STRING_EXTEND!($parser, $string) != 0 {
            if *((*$parser).buffer.pointer) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                && *((*$parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let fresh484 = addr_of_mut!($string.pointer);
                let fresh485 = *fresh484;
                *fresh484 = (*fresh484).wrapping_offset(1);
                *fresh485 = '\n' as i32 as yaml_char_t;
                let fresh486 = addr_of_mut!((*$parser).buffer.pointer);
                *fresh486 = (*fresh486).wrapping_offset(2_isize);
                let fresh487 = addr_of_mut!((*$parser).mark.index);
                *fresh487 = (*fresh487 as libc::c_ulong).wrapping_add(2_u64) as size_t as size_t;
                (*$parser).mark.column = 0_u64;
                let fresh488 = addr_of_mut!((*$parser).mark.line);
                *fresh488 = (*fresh488).wrapping_add(1);
                let fresh489 = addr_of_mut!((*$parser).unread);
                *fresh489 = (*fresh489 as libc::c_ulong).wrapping_sub(2_u64) as size_t as size_t;
            } else if *((*$parser).buffer.pointer) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *((*$parser).buffer.pointer) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
            {
                let fresh490 = addr_of_mut!($string.pointer);
                let fresh491 = *fresh490;
                *fresh490 = (*fresh490).wrapping_offset(1);
                *fresh491 = '\n' as i32 as yaml_char_t;
                let fresh492 = addr_of_mut!((*$parser).buffer.pointer);
                *fresh492 = (*fresh492).wrapping_offset(1);
                let fresh493 = addr_of_mut!((*$parser).mark.index);
                *fresh493 = (*fresh493).wrapping_add(1);
                (*$parser).mark.column = 0_u64;
                let fresh494 = addr_of_mut!((*$parser).mark.line);
                *fresh494 = (*fresh494).wrapping_add(1);
                let fresh495 = addr_of_mut!((*$parser).unread);
                *fresh495 = (*fresh495).wrapping_sub(1);
            } else if *((*$parser).buffer.pointer) as libc::c_int
                == -62i32 as yaml_char_t as libc::c_int
                && *((*$parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            {
                let fresh496 = addr_of_mut!($string.pointer);
                let fresh497 = *fresh496;
                *fresh496 = (*fresh496).wrapping_offset(1);
                *fresh497 = '\n' as i32 as yaml_char_t;
                let fresh498 = addr_of_mut!((*$parser).buffer.pointer);
                *fresh498 = (*fresh498).wrapping_offset(2_isize);
                let fresh499 = addr_of_mut!((*$parser).mark.index);
                *fresh499 = (*fresh499).wrapping_add(1);
                (*$parser).mark.column = 0_u64;
                let fresh500 = addr_of_mut!((*$parser).mark.line);
                *fresh500 = (*fresh500).wrapping_add(1);
                let fresh501 = addr_of_mut!((*$parser).unread);
                *fresh501 = (*fresh501).wrapping_sub(1);
            } else if *((*$parser).buffer.pointer) as libc::c_int
                == -30i32 as yaml_char_t as libc::c_int
                && *((*$parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && (*((*$parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
                    || *((*$parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int)
            {
                let fresh502 = addr_of_mut!((*$parser).buffer.pointer);
                let fresh503 = *fresh502;
                *fresh502 = (*fresh502).wrapping_offset(1);
                let fresh504 = addr_of_mut!($string.pointer);
                let fresh505 = *fresh504;
                *fresh504 = (*fresh504).wrapping_offset(1);
                *fresh505 = *fresh503;
                let fresh506 = addr_of_mut!((*$parser).buffer.pointer);
                let fresh507 = *fresh506;
                *fresh506 = (*fresh506).wrapping_offset(1);
                let fresh508 = addr_of_mut!($string.pointer);
                let fresh509 = *fresh508;
                *fresh508 = (*fresh508).wrapping_offset(1);
                *fresh509 = *fresh507;
                let fresh510 = addr_of_mut!((*$parser).buffer.pointer);
                let fresh511 = *fresh510;
                *fresh510 = (*fresh510).wrapping_offset(1);
                let fresh512 = addr_of_mut!($string.pointer);
                let fresh513 = *fresh512;
                *fresh512 = (*fresh512).wrapping_offset(1);
                *fresh513 = *fresh511;
                let fresh514 = addr_of_mut!((*$parser).mark.index);
                *fresh514 = (*fresh514).wrapping_add(1);
                (*$parser).mark.column = 0_u64;
                let fresh515 = addr_of_mut!((*$parser).mark.line);
                *fresh515 = (*fresh515).wrapping_add(1);
                let fresh516 = addr_of_mut!((*$parser).unread);
                *fresh516 = (*fresh516).wrapping_sub(1);
            };
            1_i32
        } else {
            0_i32
        }
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
    *token = DEQUEUE!((*parser).tokens);
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
    if CACHE!(parser, 1_u64) == 0 {
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
    if CACHE!(parser, 4_u64) == 0 {
        return 0_i32;
    }
    if *((*parser).buffer.pointer) as libc::c_int == '\0' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_stream_end(parser);
    }
    if (*parser).mark.column == 0_u64
        && *((*parser).buffer.pointer) as libc::c_int == '%' as i32 as yaml_char_t as libc::c_int
    {
        return yaml_parser_fetch_directive(parser);
    }
    if (*parser).mark.column == 0_u64
        && *((*parser).buffer.pointer) as libc::c_int == '-' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
            == '-' as i32 as yaml_char_t as libc::c_int
        && IS_BLANKZ_AT!((*parser).buffer, 3)
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_START_TOKEN);
    }
    if (*parser).mark.column == 0_u64
        && *((*parser).buffer.pointer) as libc::c_int == '.' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
        && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
            == '.' as i32 as yaml_char_t as libc::c_int
        && IS_BLANKZ_AT!((*parser).buffer, 3)
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_END_TOKEN);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '[' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_SEQUENCE_START_TOKEN);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '{' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_MAPPING_START_TOKEN);
    }
    if *((*parser).buffer.pointer) as libc::c_int == ']' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_SEQUENCE_END_TOKEN);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '}' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_MAPPING_END_TOKEN);
    }
    if *((*parser).buffer.pointer) as libc::c_int == ',' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_flow_entry(parser);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '-' as i32 as yaml_char_t as libc::c_int
        && IS_BLANKZ_AT!((*parser).buffer, 1)
    {
        return yaml_parser_fetch_block_entry(parser);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '?' as i32 as yaml_char_t as libc::c_int
        && ((*parser).flow_level != 0 || IS_BLANKZ_AT!((*parser).buffer, 1))
    {
        return yaml_parser_fetch_key(parser);
    }
    if *((*parser).buffer.pointer) as libc::c_int == ':' as i32 as yaml_char_t as libc::c_int
        && ((*parser).flow_level != 0 || IS_BLANKZ_AT!((*parser).buffer, 1))
    {
        return yaml_parser_fetch_value(parser);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '*' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_anchor(parser, YAML_ALIAS_TOKEN);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '&' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_anchor(parser, YAML_ANCHOR_TOKEN);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '!' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_tag(parser);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '|' as i32 as yaml_char_t as libc::c_int
        && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 1_i32);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '>' as i32 as yaml_char_t as libc::c_int
        && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 0_i32);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '\'' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_flow_scalar(parser, 1_i32);
    }
    if *((*parser).buffer.pointer) as libc::c_int == '"' as i32 as yaml_char_t as libc::c_int {
        return yaml_parser_fetch_flow_scalar(parser, 0_i32);
    }
    if !(IS_BLANKZ!((*parser).buffer)
        || *((*parser).buffer.pointer) as libc::c_int == '-' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '?' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == ':' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == ',' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '[' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == ']' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '{' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '}' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '#' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '&' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '*' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '!' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '|' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '>' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '\'' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '"' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '%' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '@' as i32 as yaml_char_t as libc::c_int
        || *((*parser).buffer.pointer) as libc::c_int == '`' as i32 as yaml_char_t as libc::c_int)
        || *((*parser).buffer.pointer) as libc::c_int == '-' as i32 as yaml_char_t as libc::c_int
            && !IS_BLANK_AT!((*parser).buffer, 1)
        || (*parser).flow_level == 0
            && (*((*parser).buffer.pointer) as libc::c_int
                == '?' as i32 as yaml_char_t as libc::c_int
                || *((*parser).buffer.pointer) as libc::c_int
                    == ':' as i32 as yaml_char_t as libc::c_int)
            && !IS_BLANKZ_AT!((*parser).buffer, 1)
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
    if PUSH!(parser, (*parser).simple_keys, empty_simple_key) == 0 {
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
        let _ = POP!((*parser).simple_keys);
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
        if PUSH!(parser, (*parser).indents, (*parser).indent) == 0 {
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
            if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
                return 0_i32;
            }
        } else if QUEUE_INSERT!(
            parser,
            (*parser).tokens,
            (number as libc::c_ulong).wrapping_sub((*parser).tokens_parsed),
            *token
        ) == 0
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
        if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
            return 0_i32;
        }
        (*parser).indent = POP!((*parser).indents);
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
    if PUSH!(parser, (*parser).simple_keys, simple_key) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
        if QUEUE_INSERT!(
            parser,
            (*parser).tokens,
            ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed),
            *token
        ) == 0
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
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
    if ENQUEUE!(parser, (*parser).tokens, *token) == 0 {
        yaml_token_delete(token);
        return 0_i32;
    }
    1_i32
}

unsafe fn yaml_parser_scan_to_next_token(mut parser: *mut yaml_parser_t) -> libc::c_int {
    loop {
        if CACHE!(parser, 1_u64) == 0 {
            return 0_i32;
        }
        if (*parser).mark.column == 0_u64
            && (*((*parser).buffer.pointer) as libc::c_int == -17i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -69i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -65i32 as yaml_char_t as libc::c_int)
        {
            SKIP!(parser);
        }
        if CACHE!(parser, 1_u64) == 0 {
            return 0_i32;
        }
        while *((*parser).buffer.pointer) as libc::c_int == ' ' as i32 as yaml_char_t as libc::c_int
            || ((*parser).flow_level != 0 || (*parser).simple_key_allowed == 0)
                && *((*parser).buffer.pointer) as libc::c_int
                    == '\t' as i32 as yaml_char_t as libc::c_int
        {
            SKIP!(parser);
            if CACHE!(parser, 1_u64) == 0 {
                return 0_i32;
            }
        }
        if *((*parser).buffer.pointer) as libc::c_int == '#' as i32 as yaml_char_t as libc::c_int {
            while !IS_BREAKZ!((*parser).buffer) {
                SKIP!(parser);
                if CACHE!(parser, 1_u64) == 0 {
                    return 0_i32;
                }
            }
        }
        if !(*((*parser).buffer.pointer) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer) as libc::c_int == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
        {
            break;
        }
        if CACHE!(parser, 2_u64) == 0 {
            return 0_i32;
        }
        SKIP_LINE!(parser);
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
                if !(CACHE!(parser, 1_u64) == 0) {
                    loop {
                        if !IS_BLANK!((*parser).buffer) {
                            current_block = 11584701595673473500;
                            break;
                        }
                        SKIP!(parser);
                        if CACHE!(parser, 1_u64) == 0 {
                            current_block = 11397968426844348457;
                            break;
                        }
                    }
                    match current_block {
                        11397968426844348457 => {}
                        _ => {
                            if *((*parser).buffer.pointer) as libc::c_int
                                == '#' as i32 as yaml_char_t as libc::c_int
                            {
                                loop {
                                    if IS_BREAKZ!((*parser).buffer) {
                                        current_block = 6669252993407410313;
                                        break;
                                    }
                                    SKIP!(parser);
                                    if CACHE!(parser, 1_u64) == 0 {
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
                                    if !IS_BREAKZ!((*parser).buffer) {
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
                                        if *((*parser).buffer.pointer) as libc::c_int
                                            == '\r' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer) as libc::c_int
                                                == '\n' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer) as libc::c_int
                                                == -62i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -123i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer) as libc::c_int
                                                == -30i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(1_isize)
                                                    as libc::c_int
                                                    == -128i32 as yaml_char_t as libc::c_int
                                                && *((*parser).buffer.pointer)
                                                    .wrapping_offset(2_isize)
                                                    as libc::c_int
                                                    == -88i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer) as libc::c_int
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
                                            if CACHE!(parser, 2_u64) == 0 {
                                                current_block = 11397968426844348457;
                                            } else {
                                                SKIP_LINE!(parser);
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
    let mut string = NULL_STRING!();
    if !(STRING_INIT!(parser, string) == 0) {
        if !(CACHE!(parser, 1_u64) == 0) {
            loop {
                if !IS_ALPHA!((*parser).buffer) {
                    current_block = 10879442775620481940;
                    break;
                }
                if READ!(parser, string) == 0 {
                    current_block = 8318012024179131575;
                    break;
                }
                if CACHE!(parser, 1_u64) == 0 {
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
                    } else if !IS_BLANKZ!((*parser).buffer) {
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
    STRING_DEL!(string);
    0_i32
}

unsafe fn yaml_parser_scan_version_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    major: *mut libc::c_int,
    minor: *mut libc::c_int,
) -> libc::c_int {
    if CACHE!(parser, 1_u64) == 0 {
        return 0_i32;
    }
    while IS_BLANK!((*parser).buffer) {
        SKIP!(parser);
        if CACHE!(parser, 1_u64) == 0 {
            return 0_i32;
        }
    }
    if yaml_parser_scan_version_directive_number(parser, start_mark, major) == 0 {
        return 0_i32;
    }
    if !(*((*parser).buffer.pointer) as libc::c_int == '.' as i32 as yaml_char_t as libc::c_int) {
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

const MAX_NUMBER_LENGTH: u64 = 9_u64;

unsafe fn yaml_parser_scan_version_directive_number(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    number: *mut libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_int = 0_i32;
    let mut length: size_t = 0_u64;
    if CACHE!(parser, 1_u64) == 0 {
        return 0_i32;
    }
    while IS_DIGIT!((*parser).buffer) {
        length = length.wrapping_add(1);
        if length > MAX_NUMBER_LENGTH {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a %YAML directive\0" as *const u8 as *const libc::c_char,
                start_mark,
                b"found extremely long version number\0" as *const u8 as *const libc::c_char,
            );
        }
        value = value * 10_i32 + AS_DIGIT!((*parser).buffer);
        SKIP!(parser);
        if CACHE!(parser, 1_u64) == 0 {
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
    if CACHE!(parser, 1_u64) == 0 {
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
                if IS_BLANK!((*parser).buffer) {
                    SKIP!(parser);
                    if CACHE!(parser, 1_u64) == 0 {
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
                    if CACHE!(parser, 1_u64) == 0 {
                        current_block = 5231181710497607163;
                        continue;
                    }
                    if !IS_BLANK!((*parser).buffer) {
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
                        while IS_BLANK!((*parser).buffer) {
                            SKIP!(parser);
                            if CACHE!(parser, 1_u64) == 0 {
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
                        if CACHE!(parser, 1_u64) == 0 {
                            current_block = 5231181710497607163;
                            continue;
                        }
                        if !IS_BLANKZ!((*parser).buffer) {
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
    let mut string = NULL_STRING!();
    if !(STRING_INIT!(parser, string) == 0) {
        start_mark = (*parser).mark;
        SKIP!(parser);
        if !(CACHE!(parser, 1_u64) == 0) {
            loop {
                if !IS_ALPHA!((*parser).buffer) {
                    current_block = 2868539653012386629;
                    break;
                }
                if READ!(parser, string) == 0 {
                    current_block = 5883759901342942623;
                    break;
                }
                if CACHE!(parser, 1_u64) == 0 {
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
                        || !(IS_BLANKZ!((*parser).buffer)
                            || *((*parser).buffer.pointer) as libc::c_int
                                == '?' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer) as libc::c_int
                                == ':' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer) as libc::c_int
                                == ',' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer) as libc::c_int
                                == ']' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer) as libc::c_int
                                == '}' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer) as libc::c_int
                                == '%' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer) as libc::c_int
                                == '@' as i32 as yaml_char_t as libc::c_int
                            || *((*parser).buffer.pointer) as libc::c_int
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
    STRING_DEL!(string);
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
    if !(CACHE!(parser, 2_u64) == 0) {
        if *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
            == '<' as i32 as yaml_char_t as libc::c_int
        {
            handle = yaml_malloc(1_u64) as *mut yaml_char_t;
            if handle.is_null() {
                current_block = 17708497480799081542;
            } else {
                *handle = '\0' as i32 as yaml_char_t;
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
                } else if !(*((*parser).buffer.pointer) as libc::c_int
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
        } else if *handle as libc::c_int == '!' as i32
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
                *handle = '!' as i32 as yaml_char_t;
                *handle.wrapping_offset(1_isize) = '\0' as i32 as yaml_char_t;
                if *suffix as libc::c_int == '\0' as i32 {
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
                if !(CACHE!(parser, 1_u64) == 0) {
                    if !IS_BLANKZ!((*parser).buffer) {
                        if (*parser).flow_level == 0
                            || !(*((*parser).buffer.pointer) as libc::c_int
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
    let mut string = NULL_STRING!();
    if !(STRING_INIT!(parser, string) == 0) {
        if !(CACHE!(parser, 1_u64) == 0) {
            if !(*((*parser).buffer.pointer) as libc::c_int
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
            } else if !(READ!(parser, string) == 0) {
                if !(CACHE!(parser, 1_u64) == 0) {
                    loop {
                        if !IS_ALPHA!((*parser).buffer) {
                            current_block = 7651349459974463963;
                            break;
                        }
                        if READ!(parser, string) == 0 {
                            current_block = 1771849829115608806;
                            break;
                        }
                        if CACHE!(parser, 1_u64) == 0 {
                            current_block = 1771849829115608806;
                            break;
                        }
                    }
                    match current_block {
                        1771849829115608806 => {}
                        _ => {
                            if *((*parser).buffer.pointer) as libc::c_int
                                == '!' as i32 as yaml_char_t as libc::c_int
                            {
                                if READ!(parser, string) == 0 {
                                    current_block = 1771849829115608806;
                                } else {
                                    current_block = 5689001924483802034;
                                }
                            } else if directive != 0
                                && !(*string.start as libc::c_int == '!' as i32
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
    STRING_DEL!(string);
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
    let mut string = NULL_STRING!();
    if STRING_INIT!(parser, string) == 0 {
        current_block = 15265153392498847348;
    } else {
        current_block = 14916268686031723178;
    }
    'c_21953: loop {
        match current_block {
            15265153392498847348 => {
                STRING_DEL!(string);
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
                    if CACHE!(parser, 1_u64) == 0 {
                        current_block = 15265153392498847348;
                        continue;
                    }
                    while IS_ALPHA!((*parser).buffer)
                        || *((*parser).buffer.pointer) as libc::c_int
                            == ';' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '/' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '?' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == ':' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '@' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '&' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '=' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '+' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '$' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '.' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '%' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '!' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '~' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '*' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '\'' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '(' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == ')' as i32 as yaml_char_t as libc::c_int
                        || uri_char != 0
                            && (*((*parser).buffer.pointer) as libc::c_int
                                == ',' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == '[' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == ']' as i32 as yaml_char_t as libc::c_int)
                    {
                        if *((*parser).buffer.pointer) as libc::c_int
                            == '%' as i32 as yaml_char_t as libc::c_int
                        {
                            if STRING_EXTEND!(parser, string) == 0 {
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
                        } else if READ!(parser, string) == 0 {
                            current_block = 15265153392498847348;
                            continue 'c_21953;
                        }
                        length = length.wrapping_add(1);
                        if CACHE!(parser, 1_u64) == 0 {
                            current_block = 15265153392498847348;
                            continue 'c_21953;
                        }
                    }
                    if length == 0 {
                        if STRING_EXTEND!(parser, string) == 0 {
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
        if CACHE!(parser, 3_u64) == 0 {
            return 0_i32;
        }
        if !(*((*parser).buffer.pointer) as libc::c_int == '%' as i32 as yaml_char_t as libc::c_int
            && IS_HEX_AT!((*parser).buffer, 1)
            && IS_HEX_AT!((*parser).buffer, 2))
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
        let octet: libc::c_uchar = ((AS_HEX_AT!((*parser).buffer, 1) << 4_i32)
            + AS_HEX_AT!((*parser).buffer, 2)) as libc::c_uchar;
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
    let mut string = NULL_STRING!();
    let mut leading_break = NULL_STRING!();
    let mut trailing_breaks = NULL_STRING!();
    let mut chomping: libc::c_int = 0_i32;
    let mut increment: libc::c_int = 0_i32;
    let mut indent: libc::c_int = 0_i32;
    let mut leading_blank: libc::c_int = 0_i32;
    let mut trailing_blank: libc::c_int;
    if !(STRING_INIT!(parser, string) == 0) {
        if !(STRING_INIT!(parser, leading_break) == 0) {
            if !(STRING_INIT!(parser, trailing_breaks) == 0) {
                start_mark = (*parser).mark;
                SKIP!(parser);
                if !(CACHE!(parser, 1_u64) == 0) {
                    if *((*parser).buffer.pointer) as libc::c_int
                        == '+' as i32 as yaml_char_t as libc::c_int
                        || *((*parser).buffer.pointer) as libc::c_int
                            == '-' as i32 as yaml_char_t as libc::c_int
                    {
                        chomping = if *((*parser).buffer.pointer) as libc::c_int
                            == '+' as i32 as yaml_char_t as libc::c_int
                        {
                            1_i32
                        } else {
                            -1_i32
                        };
                        SKIP!(parser);
                        if CACHE!(parser, 1_u64) == 0 {
                            current_block = 14984465786483313892;
                        } else if IS_DIGIT!((*parser).buffer) {
                            if *((*parser).buffer.pointer) as libc::c_int
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
                                increment = AS_DIGIT!((*parser).buffer);
                                SKIP!(parser);
                                current_block = 11913429853522160501;
                            }
                        } else {
                            current_block = 11913429853522160501;
                        }
                    } else if IS_DIGIT!((*parser).buffer) {
                        if *((*parser).buffer.pointer) as libc::c_int
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
                            increment = AS_DIGIT!((*parser).buffer);
                            SKIP!(parser);
                            if CACHE!(parser, 1_u64) == 0 {
                                current_block = 14984465786483313892;
                            } else {
                                if *((*parser).buffer.pointer) as libc::c_int
                                    == '+' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer) as libc::c_int
                                        == '-' as i32 as yaml_char_t as libc::c_int
                                {
                                    chomping = if *((*parser).buffer.pointer) as libc::c_int
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
                            if !(CACHE!(parser, 1_u64) == 0) {
                                loop {
                                    if !IS_BLANK!((*parser).buffer) {
                                        current_block = 4090602189656566074;
                                        break;
                                    }
                                    SKIP!(parser);
                                    if CACHE!(parser, 1_u64) == 0 {
                                        current_block = 14984465786483313892;
                                        break;
                                    }
                                }
                                match current_block {
                                    14984465786483313892 => {}
                                    _ => {
                                        if *((*parser).buffer.pointer) as libc::c_int
                                            == '#' as i32 as yaml_char_t as libc::c_int
                                        {
                                            loop {
                                                if IS_BREAKZ!((*parser).buffer) {
                                                    current_block = 12997042908615822766;
                                                    break;
                                                }
                                                SKIP!(parser);
                                                if CACHE!(parser, 1_u64) == 0 {
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
                                                if !IS_BREAKZ!((*parser).buffer) {
                                                    yaml_parser_set_scanner_error(
                                                        parser,
                                                        b"while scanning a block scalar\0" as *const u8
                                                            as *const libc::c_char,
                                                        start_mark,
                                                        b"did not find expected comment or line break\0"
                                                            as *const u8 as *const libc::c_char,
                                                    );
                                                } else {
                                                    if *((*parser).buffer.pointer) as libc::c_int
                                                        == '\r' as i32 as yaml_char_t as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            as libc::c_int
                                                            == '\n' as i32 as yaml_char_t
                                                                as libc::c_int
                                                        || *((*parser).buffer.pointer)
                                                            as libc::c_int
                                                            == -62i32 as yaml_char_t as libc::c_int
                                                            && *((*parser).buffer.pointer)
                                                                .wrapping_offset(1_isize)
                                                                as libc::c_int
                                                                == -123i32 as yaml_char_t
                                                                    as libc::c_int
                                                        || *((*parser).buffer.pointer)
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
                                                        if CACHE!(parser, 2_u64) == 0 {
                                                            current_block = 14984465786483313892;
                                                        } else {
                                                            SKIP_LINE!(parser);
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
                                                                if !(CACHE!(parser, 1_u64) == 0) {
                                                                    's_281: loop {
                                                                        if !((*parser).mark.column as libc::c_int == indent
                                                                            && !(*((*parser).buffer.pointer)
                                                                                 as libc::c_int
                                                                                == '\0' as i32 as yaml_char_t as libc::c_int))
                                                                        {
                                                                            current_block = 5793491756164225964;
                                                                            break;
                                                                        }
                                                                        trailing_blank = IS_BLANK!((*parser).buffer) as libc::c_int;
                                                                        if literal == 0
                                                                            && *leading_break.start as libc::c_int == '\n' as i32
                                                                            && leading_blank == 0 && trailing_blank == 0
                                                                        {
                                                                            if *trailing_breaks.start as libc::c_int == '\0' as i32 {
                                                                                if STRING_EXTEND!(parser, string) == 0 {
                                                                                    current_block = 14984465786483313892;
                                                                                    break;
                                                                                }
                                                                                let fresh418 = string.pointer;
                                                                                string.pointer = string.pointer.wrapping_offset(1);
                                                                                *fresh418 = ' ' as i32 as yaml_char_t;
                                                                            }
                                                                            CLEAR!(leading_break);
                                                                        } else {
                                                                            if JOIN!(parser, string, leading_break) == 0 {
                                                                                current_block = 14984465786483313892;
                                                                                break;
                                                                            }
                                                                            CLEAR!(leading_break);
                                                                        }
                                                                        if JOIN!(parser, string, trailing_breaks) == 0 {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                        CLEAR!(trailing_breaks);
                                                                        leading_blank = IS_BLANK!((*parser).buffer) as libc::c_int;
                                                                        while !IS_BREAKZ!((*parser).buffer) {
                                                                            if READ!(parser, string) == 0 {
                                                                                current_block = 14984465786483313892;
                                                                                break 's_281;
                                                                            }
                                                                            if CACHE!(parser, 1_u64) == 0 {
                                                                                current_block = 14984465786483313892;
                                                                                break 's_281;
                                                                            }
                                                                        }
                                                                        if CACHE!(parser, 2_u64) == 0 {
                                                                            current_block = 14984465786483313892;
                                                                            break;
                                                                        }
                                                                        if READ_LINE!(parser, leading_break) == 0 {
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
                                                                                if JOIN!(parser, string, leading_break) == 0 {
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
                                                                                        if JOIN!(parser, string, trailing_breaks) == 0 {
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
                                                                                            STRING_DEL!(leading_break);
                                                                                            STRING_DEL!(trailing_breaks);
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
    STRING_DEL!(string);
    STRING_DEL!(leading_break);
    STRING_DEL!(trailing_breaks);
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
        if CACHE!(parser, 1_u64) == 0 {
            return 0_i32;
        }
        while (*indent == 0 || ((*parser).mark.column as libc::c_int) < *indent)
            && *((*parser).buffer.pointer) as libc::c_int
                == ' ' as i32 as yaml_char_t as libc::c_int
        {
            SKIP!(parser);
            if CACHE!(parser, 1_u64) == 0 {
                return 0_i32;
            }
        }
        if (*parser).mark.column as libc::c_int > max_indent {
            max_indent = (*parser).mark.column as libc::c_int;
        }
        if (*indent == 0 || ((*parser).mark.column as libc::c_int) < *indent)
            && *((*parser).buffer.pointer) as libc::c_int
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
        if !(*((*parser).buffer.pointer) as libc::c_int
            == '\r' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer) as libc::c_int
                == '\n' as i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer) as libc::c_int == -62i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -123i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -88i32 as yaml_char_t as libc::c_int
            || *((*parser).buffer.pointer) as libc::c_int == -30i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(1_isize) as libc::c_int
                    == -128i32 as yaml_char_t as libc::c_int
                && *((*parser).buffer.pointer).wrapping_offset(2_isize) as libc::c_int
                    == -87i32 as yaml_char_t as libc::c_int)
        {
            break;
        }
        if CACHE!(parser, 2_u64) == 0 {
            return 0_i32;
        }
        if READ_LINE!(parser, *breaks) == 0 {
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
    let mut string = NULL_STRING!();
    let mut leading_break = NULL_STRING!();
    let mut trailing_breaks = NULL_STRING!();
    let mut whitespaces = NULL_STRING!();
    let mut leading_blanks: libc::c_int;
    if !(STRING_INIT!(parser, string) == 0) {
        if !(STRING_INIT!(parser, leading_break) == 0) {
            if !(STRING_INIT!(parser, trailing_breaks) == 0) {
                if !(STRING_INIT!(parser, whitespaces) == 0) {
                    start_mark = (*parser).mark;
                    SKIP!(parser);
                    's_58: loop {
                        if CACHE!(parser, 4_u64) == 0 {
                            current_block = 8114179180390253173;
                            break;
                        }
                        if (*parser).mark.column == 0_u64
                            && (*((*parser).buffer.pointer) as libc::c_int
                                == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int)
                            && IS_BLANKZ_AT!((*parser).buffer, 3)
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
                        } else if *((*parser).buffer.pointer) as libc::c_int
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
                            if CACHE!(parser, 2_u64) == 0 {
                                current_block = 8114179180390253173;
                                break;
                            }
                            leading_blanks = 0_i32;
                            while !IS_BLANKZ!((*parser).buffer) {
                                if single != 0
                                    && *((*parser).buffer.pointer) as libc::c_int
                                        == '\'' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '\'' as i32 as yaml_char_t as libc::c_int
                                {
                                    if STRING_EXTEND!(parser, string) == 0 {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                    let fresh521 = string.pointer;
                                    string.pointer = string.pointer.wrapping_offset(1);
                                    *fresh521 = '\'' as i32 as yaml_char_t;
                                    SKIP!(parser);
                                    SKIP!(parser);
                                } else {
                                    if *((*parser).buffer.pointer) as libc::c_int
                                        == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                            as yaml_char_t
                                            as libc::c_int
                                    {
                                        break;
                                    }
                                    if single == 0
                                        && *((*parser).buffer.pointer) as libc::c_int
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
                                        if CACHE!(parser, 3_u64) == 0 {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                        SKIP!(parser);
                                        SKIP_LINE!(parser);
                                        leading_blanks = 1_i32;
                                        break;
                                    } else if single == 0
                                        && *((*parser).buffer.pointer) as libc::c_int
                                            == '\\' as i32 as yaml_char_t as libc::c_int
                                    {
                                        let mut code_length: size_t = 0_u64;
                                        if STRING_EXTEND!(parser, string) == 0 {
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
                                            if CACHE!(parser, code_length) == 0 {
                                                current_block = 8114179180390253173;
                                                break 's_58;
                                            }
                                            k = 0_u64;
                                            while k < code_length {
                                                if !IS_HEX_AT!((*parser).buffer, k as isize) {
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
                                                        AS_HEX_AT!((*parser).buffer, k as isize)
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
                                    } else if READ!(parser, string) == 0 {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                }
                                if CACHE!(parser, 2_u64) == 0 {
                                    current_block = 8114179180390253173;
                                    break 's_58;
                                }
                            }
                            if CACHE!(parser, 1_u64) == 0 {
                                current_block = 8114179180390253173;
                                break;
                            }
                            if *((*parser).buffer.pointer) as libc::c_int
                                == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                    as yaml_char_t as libc::c_int
                            {
                                current_block = 7468767852762055642;
                                break;
                            }
                            if CACHE!(parser, 1_u64) == 0 {
                                current_block = 8114179180390253173;
                                break;
                            }
                            while IS_BLANK!((*parser).buffer)
                                || (*((*parser).buffer.pointer) as libc::c_int
                                    == '\r' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer) as libc::c_int
                                        == '\n' as i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer) as libc::c_int
                                        == -62i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -123i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer) as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                            as libc::c_int
                                            == -88i32 as yaml_char_t as libc::c_int
                                    || *((*parser).buffer.pointer) as libc::c_int
                                        == -30i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                            as libc::c_int
                                            == -128i32 as yaml_char_t as libc::c_int
                                        && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                            as libc::c_int
                                            == -87i32 as yaml_char_t as libc::c_int)
                            {
                                if IS_BLANK!((*parser).buffer) {
                                    if leading_blanks == 0 {
                                        if READ!(parser, whitespaces) == 0 {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                    } else {
                                        SKIP!(parser);
                                    }
                                } else {
                                    if CACHE!(parser, 2_u64) == 0 {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                    if leading_blanks == 0 {
                                        CLEAR!(whitespaces);
                                        if READ_LINE!(parser, leading_break) == 0 {
                                            current_block = 8114179180390253173;
                                            break 's_58;
                                        }
                                        leading_blanks = 1_i32;
                                    } else if READ_LINE!(parser, trailing_breaks) == 0 {
                                        current_block = 8114179180390253173;
                                        break 's_58;
                                    }
                                }
                                if CACHE!(parser, 1_u64) == 0 {
                                    current_block = 8114179180390253173;
                                    break 's_58;
                                }
                            }
                            if leading_blanks != 0 {
                                if *leading_break.start as libc::c_int == '\n' as i32 {
                                    if *trailing_breaks.start as libc::c_int == '\0' as i32 {
                                        if STRING_EXTEND!(parser, string) == 0 {
                                            current_block = 8114179180390253173;
                                            break;
                                        }
                                        let fresh711 = string.pointer;
                                        string.pointer = string.pointer.wrapping_offset(1);
                                        *fresh711 = ' ' as i32 as yaml_char_t;
                                    } else {
                                        if JOIN!(parser, string, trailing_breaks) == 0 {
                                            current_block = 8114179180390253173;
                                            break;
                                        }
                                        CLEAR!(trailing_breaks);
                                    }
                                    CLEAR!(leading_break);
                                } else {
                                    if JOIN!(parser, string, leading_break) == 0 {
                                        current_block = 8114179180390253173;
                                        break;
                                    }
                                    if JOIN!(parser, string, trailing_breaks) == 0 {
                                        current_block = 8114179180390253173;
                                        break;
                                    }
                                    CLEAR!(leading_break);
                                    CLEAR!(trailing_breaks);
                                }
                            } else {
                                if JOIN!(parser, string, whitespaces) == 0 {
                                    current_block = 8114179180390253173;
                                    break;
                                }
                                CLEAR!(whitespaces);
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
                            STRING_DEL!(leading_break);
                            STRING_DEL!(trailing_breaks);
                            STRING_DEL!(whitespaces);
                            return 1_i32;
                        }
                    }
                }
            }
        }
    }
    STRING_DEL!(string);
    STRING_DEL!(leading_break);
    STRING_DEL!(trailing_breaks);
    STRING_DEL!(whitespaces);
    0_i32
}

unsafe fn yaml_parser_scan_plain_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> libc::c_int {
    let current_block: u64;
    let start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut string = NULL_STRING!();
    let mut leading_break = NULL_STRING!();
    let mut trailing_breaks = NULL_STRING!();
    let mut whitespaces = NULL_STRING!();
    let mut leading_blanks: libc::c_int = 0_i32;
    let indent: libc::c_int = (*parser).indent + 1_i32;
    if !(STRING_INIT!(parser, string) == 0) {
        if !(STRING_INIT!(parser, leading_break) == 0) {
            if !(STRING_INIT!(parser, trailing_breaks) == 0) {
                if !(STRING_INIT!(parser, whitespaces) == 0) {
                    end_mark = (*parser).mark;
                    start_mark = end_mark;
                    's_57: loop {
                        if CACHE!(parser, 4_u64) == 0 {
                            current_block = 16642808987012640029;
                            break;
                        }
                        if (*parser).mark.column == 0_u64
                            && (*((*parser).buffer.pointer) as libc::c_int
                                == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                    as libc::c_int
                                    == '-' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == '.' as i32 as yaml_char_t as libc::c_int)
                            && IS_BLANKZ_AT!((*parser).buffer, 3)
                        {
                            current_block = 6281126495347172768;
                            break;
                        }
                        if *((*parser).buffer.pointer) as libc::c_int
                            == '#' as i32 as yaml_char_t as libc::c_int
                        {
                            current_block = 6281126495347172768;
                            break;
                        }
                        while !IS_BLANKZ!((*parser).buffer) {
                            if (*parser).flow_level != 0
                                && *((*parser).buffer.pointer) as libc::c_int
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
                                if *((*parser).buffer.pointer) as libc::c_int
                                    == ':' as i32 as yaml_char_t as libc::c_int
                                    && IS_BLANKZ_AT!((*parser).buffer, 1)
                                    || (*parser).flow_level != 0
                                        && (*((*parser).buffer.pointer) as libc::c_int
                                            == ',' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer) as libc::c_int
                                                == '[' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer) as libc::c_int
                                                == ']' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer) as libc::c_int
                                                == '{' as i32 as yaml_char_t as libc::c_int
                                            || *((*parser).buffer.pointer) as libc::c_int
                                                == '}' as i32 as yaml_char_t as libc::c_int)
                                {
                                    break;
                                }
                                if leading_blanks != 0 || whitespaces.start != whitespaces.pointer {
                                    if leading_blanks != 0 {
                                        if *leading_break.start as libc::c_int == '\n' as i32 {
                                            if *trailing_breaks.start as libc::c_int == '\0' as i32
                                            {
                                                if STRING_EXTEND!(parser, string) == 0 {
                                                    current_block = 16642808987012640029;
                                                    break 's_57;
                                                }
                                                let fresh717 = string.pointer;
                                                string.pointer = string.pointer.wrapping_offset(1);
                                                *fresh717 = ' ' as i32 as yaml_char_t;
                                            } else {
                                                if JOIN!(parser, string, trailing_breaks) == 0 {
                                                    current_block = 16642808987012640029;
                                                    break 's_57;
                                                }
                                                CLEAR!(trailing_breaks);
                                            }
                                            CLEAR!(leading_break);
                                        } else {
                                            if JOIN!(parser, string, leading_break) == 0 {
                                                current_block = 16642808987012640029;
                                                break 's_57;
                                            }
                                            if JOIN!(parser, string, trailing_breaks) == 0 {
                                                current_block = 16642808987012640029;
                                                break 's_57;
                                            }
                                            CLEAR!(leading_break);
                                            CLEAR!(trailing_breaks);
                                        }
                                        leading_blanks = 0_i32;
                                    } else {
                                        if JOIN!(parser, string, whitespaces) == 0 {
                                            current_block = 16642808987012640029;
                                            break 's_57;
                                        }
                                        CLEAR!(whitespaces);
                                    }
                                }
                                if READ!(parser, string) == 0 {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                                end_mark = (*parser).mark;
                                if CACHE!(parser, 2_u64) == 0 {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                            }
                        }
                        if !(IS_BLANK!((*parser).buffer)
                            || (*((*parser).buffer.pointer) as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
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
                        if CACHE!(parser, 1_u64) == 0 {
                            current_block = 16642808987012640029;
                            break;
                        }
                        while IS_BLANK!((*parser).buffer)
                            || (*((*parser).buffer.pointer) as libc::c_int
                                == '\r' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == '\n' as i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == -62i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -123i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -88i32 as yaml_char_t as libc::c_int
                                || *((*parser).buffer.pointer) as libc::c_int
                                    == -30i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(1_isize)
                                        as libc::c_int
                                        == -128i32 as yaml_char_t as libc::c_int
                                    && *((*parser).buffer.pointer).wrapping_offset(2_isize)
                                        as libc::c_int
                                        == -87i32 as yaml_char_t as libc::c_int)
                        {
                            if IS_BLANK!((*parser).buffer) {
                                if leading_blanks != 0
                                    && ((*parser).mark.column as libc::c_int) < indent
                                    && *((*parser).buffer.pointer) as libc::c_int
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
                                    if READ!(parser, whitespaces) == 0 {
                                        current_block = 16642808987012640029;
                                        break 's_57;
                                    }
                                } else {
                                    SKIP!(parser);
                                }
                            } else {
                                if CACHE!(parser, 2_u64) == 0 {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                                if leading_blanks == 0 {
                                    CLEAR!(whitespaces);
                                    if READ_LINE!(parser, leading_break) == 0 {
                                        current_block = 16642808987012640029;
                                        break 's_57;
                                    }
                                    leading_blanks = 1_i32;
                                } else if READ_LINE!(parser, trailing_breaks) == 0 {
                                    current_block = 16642808987012640029;
                                    break 's_57;
                                }
                            }
                            if CACHE!(parser, 1_u64) == 0 {
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
                            STRING_DEL!(leading_break);
                            STRING_DEL!(trailing_breaks);
                            STRING_DEL!(whitespaces);
                            return 1_i32;
                        }
                    }
                }
            }
        }
    }
    STRING_DEL!(string);
    STRING_DEL!(leading_break);
    STRING_DEL!(trailing_breaks);
    STRING_DEL!(whitespaces);
    0_i32
}
