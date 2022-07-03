#![feature(extern_types)]
#![allow(
    non_camel_case_types,
    non_snake_case,
    unused_assignments,
    unused_mut,
)]

use unsafe_libyaml::externs::__assert_fail;
use unsafe_libyaml::*;
extern "C" {
    fn yaml_parser_parse(
        parser: *mut yaml_parser_t,
        event: *mut yaml_event_t,
    ) -> libc::c_int;
    fn yaml_parser_set_input_file(parser: *mut yaml_parser_t, file: *mut FILE);
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> libc::c_int;
    fn yaml_event_delete(event: *mut yaml_event_t);
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn abort() -> !;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut input: *mut FILE = 0 as *mut FILE;
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
    let mut flow: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut foundfile: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if strncmp(
            *argv.offset(i as isize),
            b"--flow\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            if i + 1 as libc::c_int == argc {
                return usage(1 as libc::c_int);
            }
            i += 1;
            if strncmp(
                *argv.offset(i as isize),
                b"keep\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                flow = 0 as libc::c_int;
            } else if strncmp(
                    *argv.offset(i as isize),
                    b"on\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                flow = 1 as libc::c_int;
            } else if strncmp(
                    *argv.offset(i as isize),
                    b"off\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                flow = -(1 as libc::c_int);
            } else {
                return usage(1 as libc::c_int)
            }
        } else if strncmp(
                *argv.offset(i as isize),
                b"--help\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            return usage(0 as libc::c_int)
        } else {
            if strncmp(
                *argv.offset(i as isize),
                b"-h\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                return usage(0 as libc::c_int)
            } else {
                if foundfile == 0 {
                    input = fopen(
                        *argv.offset(i as isize),
                        b"rb\0" as *const u8 as *const libc::c_char,
                    );
                    foundfile = 1 as libc::c_int;
                } else {
                    return usage(1 as libc::c_int)
                }
            }
        }
        i += 1;
    }
    if foundfile == 0 {
        input = stdin;
    }
    if !input.is_null() {} else {
        __assert_fail(
            b"input\0" as *const u8 as *const libc::c_char,
            b"run-parser-test-suite.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    if yaml_parser_initialize(&mut parser) == 0 {
        fprintf(
            stderr,
            b"Could not initialize the parser object\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    yaml_parser_set_input_file(&mut parser, input);
    loop {
        let mut type_0: yaml_event_type_t = YAML_NO_EVENT;
        if yaml_parser_parse(&mut parser, &mut event) == 0 {
            if parser.problem_mark.line != 0 || parser.problem_mark.column != 0 {
                fprintf(
                    stderr,
                    b"Parse error: %s\nLine: %lu Column: %lu\n\0" as *const u8
                        as *const libc::c_char,
                    parser.problem,
                    (parser.problem_mark.line)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    (parser.problem_mark.column)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            } else {
                fprintf(
                    stderr,
                    b"Parse error: %s\n\0" as *const u8 as *const libc::c_char,
                    parser.problem,
                );
            }
            return 1 as libc::c_int;
        }
        type_0 = event.type_0;
        if type_0 as libc::c_uint == YAML_NO_EVENT as libc::c_int as libc::c_uint {
            printf(b"???\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_STREAM_START_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"+STR\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"-STR\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_DOCUMENT_START_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"+DOC\0" as *const u8 as *const libc::c_char);
            if event.data.document_start.implicit == 0 {
                printf(b" ---\0" as *const u8 as *const libc::c_char);
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_DOCUMENT_END_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"-DOC\0" as *const u8 as *const libc::c_char);
            if event.data.document_end.implicit == 0 {
                printf(b" ...\0" as *const u8 as *const libc::c_char);
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_MAPPING_START_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"+MAP\0" as *const u8 as *const libc::c_char);
            if flow == 0 as libc::c_int
                && event.data.mapping_start.style as libc::c_uint
                    == YAML_FLOW_MAPPING_STYLE as libc::c_int as libc::c_uint
            {
                printf(b" {}\0" as *const u8 as *const libc::c_char);
            } else if flow == 1 as libc::c_int {
                printf(b" {}\0" as *const u8 as *const libc::c_char);
            }
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
        } else if type_0 as libc::c_uint
                == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"-MAP\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_SEQUENCE_START_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"+SEQ\0" as *const u8 as *const libc::c_char);
            if flow == 0 as libc::c_int
                && event.data.sequence_start.style as libc::c_uint
                    == YAML_FLOW_SEQUENCE_STYLE as libc::c_int as libc::c_uint
            {
                printf(b" []\0" as *const u8 as *const libc::c_char);
            } else if flow == 1 as libc::c_int {
                printf(b" []\0" as *const u8 as *const libc::c_char);
            }
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
        } else if type_0 as libc::c_uint
                == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint
            {
            printf(b"-SEQ\n\0" as *const u8 as *const libc::c_char);
        } else if type_0 as libc::c_uint
                == YAML_SCALAR_EVENT as libc::c_int as libc::c_uint
            {
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
        } else if type_0 as libc::c_uint
                == YAML_ALIAS_EVENT as libc::c_int as libc::c_uint
            {
            printf(
                b"=ALI *%s\n\0" as *const u8 as *const libc::c_char,
                event.data.alias.anchor,
            );
        } else {
            abort();
        }
        yaml_event_delete(&mut event);
        if type_0 as libc::c_uint == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint
        {
            break;
        }
    }
    if fclose(input) == 0 {} else {
        __assert_fail(
            b"!fclose(input)\0" as *const u8 as *const libc::c_char,
            b"run-parser-test-suite.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    yaml_parser_delete(&mut parser);
    fflush(stdout);
    return 0 as libc::c_int;
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
            printf(b"%c\0" as *const u8 as *const libc::c_char, c as libc::c_int);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut ret: libc::c_int) -> libc::c_int {
    fprintf(
        stderr,
        b"Usage: libyaml-parser [--flow (on|off|keep)] [<input-file>]\n\0" as *const u8
            as *const libc::c_char,
    );
    return ret;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}