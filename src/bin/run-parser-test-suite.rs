#![feature(extern_types)]
#![allow(non_camel_case_types, non_snake_case, unused_assignments, unused_mut)]

use std::cmp;
use std::env;
use std::fs;
use std::process::ExitCode;
use std::ptr;
use unsafe_libyaml::api::{
    yaml_event_delete, yaml_parser_delete, yaml_parser_initialize, yaml_parser_set_input,
};
use unsafe_libyaml::externs::__assert_fail;
use unsafe_libyaml::parser::yaml_parser_parse;
use unsafe_libyaml::*;
extern "C" {
    pub type FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn abort() -> !;
}
unsafe fn unsafe_main() -> ExitCode {
    let mut input = None;
    let mut parser: yaml_parser_t = yaml_parser_t {
        error: YAML_NO_ERROR,
        problem: 0 as *const libc::c_char,
        problem_offset: 0,
        problem_value: 0,
        problem_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        context: 0 as *const libc::c_char,
        context_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        read_handler: None,
        read_handler_data: 0 as *mut libc::c_void,
        input: unnamed_yaml_parser_s_input {
            string: unnamed_yaml_parser_s_input_string {
                start: 0 as *const libc::c_uchar,
                end: 0 as *const libc::c_uchar,
                current: 0 as *const libc::c_uchar,
            },
        },
        eof: 0,
        buffer: unnamed_yaml_parser_s_buffer {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
            last: 0 as *mut yaml_char_t,
        },
        unread: 0,
        raw_buffer: unnamed_yaml_parser_s_raw_buffer {
            start: 0 as *mut libc::c_uchar,
            end: 0 as *mut libc::c_uchar,
            pointer: 0 as *mut libc::c_uchar,
            last: 0 as *mut libc::c_uchar,
        },
        encoding: YAML_ANY_ENCODING,
        offset: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        stream_start_produced: 0,
        stream_end_produced: 0,
        flow_level: 0,
        tokens: unnamed_yaml_parser_s_tokens {
            start: 0 as *mut yaml_token_t,
            end: 0 as *mut yaml_token_t,
            head: 0 as *mut yaml_token_t,
            tail: 0 as *mut yaml_token_t,
        },
        tokens_parsed: 0,
        token_available: 0,
        indents: unnamed_yaml_parser_s_indents {
            start: 0 as *mut libc::c_int,
            end: 0 as *mut libc::c_int,
            top: 0 as *mut libc::c_int,
        },
        indent: 0,
        simple_key_allowed: 0,
        simple_keys: unnamed_yaml_parser_s_simple_keys {
            start: 0 as *mut yaml_simple_key_t,
            end: 0 as *mut yaml_simple_key_t,
            top: 0 as *mut yaml_simple_key_t,
        },
        states: unnamed_yaml_parser_s_states {
            start: 0 as *mut yaml_parser_state_t,
            end: 0 as *mut yaml_parser_state_t,
            top: 0 as *mut yaml_parser_state_t,
        },
        state: YAML_PARSE_STREAM_START_STATE,
        marks: unnamed_yaml_parser_s_marks {
            start: 0 as *mut yaml_mark_t,
            end: 0 as *mut yaml_mark_t,
            top: 0 as *mut yaml_mark_t,
        },
        tag_directives: unnamed_yaml_parser_s_tag_directives {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        },
        aliases: unnamed_yaml_parser_s_aliases {
            start: 0 as *mut yaml_alias_data_t,
            end: 0 as *mut yaml_alias_data_t,
            top: 0 as *mut yaml_alias_data_t,
        },
        document: 0 as *mut yaml_document_t,
    };
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: unnamed_yaml_event_s_data {
            stream_start: unnamed_yaml_event_s_data_stream_start {
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
    let mut foundfile: libc::c_int = 0 as libc::c_int;
    for arg in env::args_os().skip(1) {
        if foundfile == 0 {
            input = fs::read(arg).ok();
            foundfile = 1 as libc::c_int;
        } else {
            return usage(ExitCode::FAILURE);
        }
    }
    let input = input.unwrap_or_else(|| {
        __assert_fail(
            b"input\0" as *const u8 as *const libc::c_char,
            b"run-parser-test-suite.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"int main(int, char **)\0"))
                .as_ptr(),
        )
    });
    if yaml_parser_initialize(&mut parser) == 0 {
        fprintf(
            stderr,
            b"Could not initialize the parser object\n\0" as *const u8 as *const libc::c_char,
        );
        return ExitCode::FAILURE;
    }
    unsafe extern "C" fn read_from_file(
        data: *mut libc::c_void,
        buffer: *mut libc::c_uchar,
        size: size_t,
        size_read: *mut size_t,
    ) -> libc::c_int {
        let remaining: *mut &[u8] = data.cast();
        let n = cmp::min(size as usize, (*remaining).len());
        ptr::copy_nonoverlapping((*remaining).as_ptr().cast(), buffer, n);
        *remaining = &(*remaining)[n..];
        *size_read = n as size_t;
        1 as libc::c_int
    }
    let mut remaining = input.as_slice();
    yaml_parser_set_input(
        &mut parser,
        Some(read_from_file),
        ptr::addr_of_mut!(remaining).cast(),
    );
    loop {
        let mut type_0: yaml_event_type_t = YAML_NO_EVENT;
        if yaml_parser_parse(&mut parser, &mut event) == 0 {
            if parser.problem_mark.line != 0 || parser.problem_mark.column != 0 {
                fprintf(
                    stderr,
                    b"Parse error: %s\nLine: %lu Column: %lu\n\0" as *const u8
                        as *const libc::c_char,
                    parser.problem,
                    (parser.problem_mark.line).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    (parser.problem_mark.column).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            } else {
                fprintf(
                    stderr,
                    b"Parse error: %s\n\0" as *const u8 as *const libc::c_char,
                    parser.problem,
                );
            }
            return ExitCode::FAILURE;
        }
        type_0 = event.type_0;
        if type_0 as libc::c_uint == YAML_NO_EVENT as libc::c_int as libc::c_uint {
            printf(b"???\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_STREAM_START_EVENT as libc::c_int as libc::c_uint {
            printf(b"+STR\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint {
            printf(b"-STR\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_DOCUMENT_START_EVENT as libc::c_int as libc::c_uint
        {
            printf(b"+DOC\0" as *const u8 as *const libc::c_char);
            if event.data.document_start.implicit == 0 {
                printf(b" ---\0" as *const u8 as *const libc::c_char);
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_DOCUMENT_END_EVENT as libc::c_int as libc::c_uint {
            printf(b"-DOC\0" as *const u8 as *const libc::c_char);
            if event.data.document_end.implicit == 0 {
                printf(b" ...\0" as *const u8 as *const libc::c_char);
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_MAPPING_START_EVENT as libc::c_int as libc::c_uint
        {
            printf(b"+MAP\0" as *const u8 as *const libc::c_char);
            if !(event.data.mapping_start.anchor).is_null() {
                printf(
                    b" &%s\0" as *const u8 as *const libc::c_char,
                    event.data.mapping_start.anchor,
                );
            }
            if !(event.data.mapping_start.tag).is_null() {
                printf(
                    b" <%s>\0" as *const u8 as *const libc::c_char,
                    event.data.mapping_start.tag,
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint {
            printf(b"-MAP\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_SEQUENCE_START_EVENT as libc::c_int as libc::c_uint
        {
            printf(b"+SEQ\0" as *const u8 as *const libc::c_char);
            if !(event.data.sequence_start.anchor).is_null() {
                printf(
                    b" &%s\0" as *const u8 as *const libc::c_char,
                    event.data.sequence_start.anchor,
                );
            }
            if !(event.data.sequence_start.tag).is_null() {
                printf(
                    b" <%s>\0" as *const u8 as *const libc::c_char,
                    event.data.sequence_start.tag,
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint {
            printf(b"-SEQ\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_SCALAR_EVENT as libc::c_int as libc::c_uint {
            printf(b"=VAL\0" as *const u8 as *const libc::c_char);
            if !(event.data.scalar.anchor).is_null() {
                printf(
                    b" &%s\0" as *const u8 as *const libc::c_char,
                    event.data.scalar.anchor,
                );
            }
            if !(event.data.scalar.tag).is_null() {
                printf(
                    b" <%s>\0" as *const u8 as *const libc::c_char,
                    event.data.scalar.tag,
                );
            }
            match event.data.scalar.style as libc::c_uint {
                1 => {
                    printf(b" :\0" as *const u8 as *const libc::c_char);
                }
                2 => {
                    printf(b" '\0" as *const u8 as *const libc::c_char);
                }
                3 => {
                    printf(b" \"\0" as *const u8 as *const libc::c_char);
                }
                4 => {
                    printf(b" |\0" as *const u8 as *const libc::c_char);
                }
                5 => {
                    printf(b" >\0" as *const u8 as *const libc::c_char);
                }
                0 => {
                    abort();
                }
                _ => {}
            }
            print_escaped(event.data.scalar.value, event.data.scalar.length);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint == YAML_ALIAS_EVENT as libc::c_int as libc::c_uint {
            printf(
                b"=ALI *%s\n\0" as *const u8 as *const libc::c_char,
                event.data.alias.anchor,
            );
        } else {
            abort();
        }
        yaml_event_delete(&mut event);
        if type_0 as libc::c_uint == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint {
            break;
        }
    }
    yaml_parser_delete(&mut parser);
    return ExitCode::SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn print_escaped(mut str: *mut yaml_char_t, mut length: size_t) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < length {
        c = *str.offset(i as isize) as libc::c_char;
        if c as libc::c_int == '\\' as i32 {
            printf(b"\\\\\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\0' as i32 {
            printf(b"\\0\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\u{8}' as i32 {
            printf(b"\\b\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\n' as i32 {
            printf(b"\\n\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\r' as i32 {
            printf(b"\\r\0" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\t' as i32 {
            printf(b"\\t\0" as *const u8 as *const libc::c_char);
        } else {
            printf(
                b"%c\0" as *const u8 as *const libc::c_char,
                c as libc::c_int,
            );
        }
        i += 1;
    }
}
unsafe fn usage(ret: ExitCode) -> ExitCode {
    fprintf(
        stderr,
        b"Usage: libyaml-parser [<input-file>]\n\0" as *const u8 as *const libc::c_char,
    );
    return ret;
}
fn main() -> ExitCode {
    unsafe { unsafe_main() }
}
