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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn abort() -> !;
    fn yaml_emitter_emit(
        emitter: *mut yaml_emitter_t,
        event: *mut yaml_event_t,
    ) -> libc::c_int;
    fn yaml_emitter_set_unicode(emitter: *mut yaml_emitter_t, unicode: libc::c_int);
    fn yaml_emitter_set_canonical(emitter: *mut yaml_emitter_t, canonical: libc::c_int);
    fn yaml_emitter_set_output_file(emitter: *mut yaml_emitter_t, file: *mut FILE);
    fn yaml_emitter_delete(emitter: *mut yaml_emitter_t);
    fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn yaml_stream_start_event_initialize(
        event: *mut yaml_event_t,
        encoding: yaml_encoding_t,
    ) -> libc::c_int;
    fn yaml_stream_end_event_initialize(event: *mut yaml_event_t) -> libc::c_int;
    fn yaml_document_start_event_initialize(
        event: *mut yaml_event_t,
        version_directive: *mut yaml_version_directive_t,
        tag_directives_start: *mut yaml_tag_directive_t,
        tag_directives_end: *mut yaml_tag_directive_t,
        implicit: libc::c_int,
    ) -> libc::c_int;
    fn yaml_document_end_event_initialize(
        event: *mut yaml_event_t,
        implicit: libc::c_int,
    ) -> libc::c_int;
    fn yaml_alias_event_initialize(
        event: *mut yaml_event_t,
        anchor: *const yaml_char_t,
    ) -> libc::c_int;
    fn yaml_scalar_event_initialize(
        event: *mut yaml_event_t,
        anchor: *const yaml_char_t,
        tag: *const yaml_char_t,
        value: *const yaml_char_t,
        length: libc::c_int,
        plain_implicit: libc::c_int,
        quoted_implicit: libc::c_int,
        style: yaml_scalar_style_t,
    ) -> libc::c_int;
    fn yaml_sequence_start_event_initialize(
        event: *mut yaml_event_t,
        anchor: *const yaml_char_t,
        tag: *const yaml_char_t,
        implicit: libc::c_int,
        style: yaml_sequence_style_t,
    ) -> libc::c_int;
    fn yaml_sequence_end_event_initialize(event: *mut yaml_event_t) -> libc::c_int;
    fn yaml_mapping_start_event_initialize(
        event: *mut yaml_event_t,
        anchor: *const yaml_char_t,
        tag: *const yaml_char_t,
        implicit: libc::c_int,
        style: yaml_mapping_style_t,
    ) -> libc::c_int;
    fn yaml_mapping_end_event_initialize(event: *mut yaml_event_t) -> libc::c_int;
    fn yaml_malloc(size: size_t) -> *mut libc::c_void;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut input: *mut FILE = 0 as *mut FILE;
    let mut emitter: yaml_emitter_t = yaml_emitter_t {
        error: YAML_NO_ERROR,
        problem: 0 as *const libc::c_char,
        write_handler: None,
        write_handler_data: 0 as *mut libc::c_void,
        output: unnamed_yaml_emitter_s_output {
            string: unnamed_yaml_emitter_s_output_string {
                buffer: 0 as *mut libc::c_uchar,
                size: 0,
                size_written: 0 as *mut size_t,
            },
        },
        buffer: unnamed_yaml_emitter_s_buffer {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
            last: 0 as *mut yaml_char_t,
        },
        raw_buffer: unnamed_yaml_emitter_s_raw_buffer {
            start: 0 as *mut libc::c_uchar,
            end: 0 as *mut libc::c_uchar,
            pointer: 0 as *mut libc::c_uchar,
            last: 0 as *mut libc::c_uchar,
        },
        encoding: YAML_ANY_ENCODING,
        canonical: 0,
        best_indent: 0,
        best_width: 0,
        unicode: 0,
        line_break: YAML_ANY_BREAK,
        states: unnamed_yaml_emitter_s_states {
            start: 0 as *mut yaml_emitter_state_t,
            end: 0 as *mut yaml_emitter_state_t,
            top: 0 as *mut yaml_emitter_state_t,
        },
        state: YAML_EMIT_STREAM_START_STATE,
        events: unnamed_yaml_emitter_s_events {
            start: 0 as *mut yaml_event_t,
            end: 0 as *mut yaml_event_t,
            head: 0 as *mut yaml_event_t,
            tail: 0 as *mut yaml_event_t,
        },
        indents: unnamed_yaml_emitter_s_indents {
            start: 0 as *mut libc::c_int,
            end: 0 as *mut libc::c_int,
            top: 0 as *mut libc::c_int,
        },
        tag_directives: unnamed_yaml_emitter_s_tag_directives {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        },
        indent: 0,
        flow_level: 0,
        root_context: 0,
        sequence_context: 0,
        mapping_context: 0,
        simple_key_context: 0,
        line: 0,
        column: 0,
        whitespace: 0,
        indention: 0,
        open_ended: 0,
        anchor_data: unnamed_yaml_emitter_s_anchor_data {
            anchor: 0 as *mut yaml_char_t,
            anchor_length: 0,
            alias: 0,
        },
        tag_data: unnamed_yaml_emitter_s_tag_data {
            handle: 0 as *mut yaml_char_t,
            handle_length: 0,
            suffix: 0 as *mut yaml_char_t,
            suffix_length: 0,
        },
        scalar_data: unnamed_yaml_emitter_s_scalar_data {
            value: 0 as *mut yaml_char_t,
            length: 0,
            multiline: 0,
            flow_plain_allowed: 0,
            block_plain_allowed: 0,
            single_quoted_allowed: 0,
            block_allowed: 0,
            style: YAML_ANY_SCALAR_STYLE,
        },
        opened: 0,
        closed: 0,
        anchors: 0 as *mut yaml_anchors_t,
        last_anchor_id: 0,
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
    let mut version_directive: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut canonical: libc::c_int = 0 as libc::c_int;
    let mut unicode: libc::c_int = 0 as libc::c_int;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut foundfile: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut minor: libc::c_int = 0 as libc::c_int;
    let mut flow: libc::c_int = -(1 as libc::c_int);
    i = 1 as libc::c_int;
    while i < argc {
        if strncmp(
            *argv.offset(i as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return usage(0 as libc::c_int);
        }
        if strncmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return usage(0 as libc::c_int);
        }
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
                b"--directive\0" as *const u8 as *const libc::c_char,
                11 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            if i + 1 as libc::c_int == argc {
                return usage(1 as libc::c_int);
            }
            i += 1;
            if strncmp(
                *argv.offset(i as isize),
                b"1.1\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                minor = 1 as libc::c_int;
            } else if strncmp(
                    *argv.offset(i as isize),
                    b"1.2\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                minor = 2 as libc::c_int;
            } else {
                return usage(1 as libc::c_int)
            }
        } else if foundfile == 0 {
            input = fopen(
                *argv.offset(i as isize),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            foundfile = 1 as libc::c_int;
        }
        i += 1;
    }
    if minor != 0 {
        version_directive = yaml_malloc(
            ::std::mem::size_of::<yaml_version_directive_t>() as libc::c_ulong,
        ) as *mut yaml_version_directive_t;
        (*version_directive).major = 1 as libc::c_int;
        (*version_directive).minor = minor;
    }
    if foundfile == 0 {
        input = stdin;
    }
    if !input.is_null() {} else {
        __assert_fail(
            b"input\0" as *const u8 as *const libc::c_char,
            b"run-emitter-test-suite.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    if yaml_emitter_initialize(&mut emitter) == 0 {
        fprintf(
            stderr,
            b"Could not initalize the emitter object\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    yaml_emitter_set_output_file(&mut emitter, stdout);
    yaml_emitter_set_canonical(&mut emitter, canonical);
    yaml_emitter_set_unicode(&mut emitter, unicode);
    loop {
        if !(get_line(input, line.as_mut_ptr()) != 0) {
            current_block = 1934991416718554651;
            break;
        }
        let mut ok: libc::c_int = 0;
        let mut anchor: [libc::c_char; 256] = [0; 256];
        let mut tag: [libc::c_char; 256] = [0; 256];
        let mut implicit: libc::c_int = 0;
        let mut style: libc::c_int = 0;
        if strncmp(
            line.as_mut_ptr(),
            b"+STR\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ok = yaml_stream_start_event_initialize(&mut event, YAML_UTF8_ENCODING);
        } else if strncmp(
                line.as_mut_ptr(),
                b"-STR\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            ok = yaml_stream_end_event_initialize(&mut event);
        } else if strncmp(
                line.as_mut_ptr(),
                b"+DOC\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            implicit = (strncmp(
                line.as_mut_ptr().offset(4 as libc::c_int as isize),
                b" ---\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int) as libc::c_int;
            ok = yaml_document_start_event_initialize(
                &mut event,
                version_directive,
                0 as *mut yaml_tag_directive_t,
                0 as *mut yaml_tag_directive_t,
                implicit,
            );
        } else if strncmp(
                line.as_mut_ptr(),
                b"-DOC\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            implicit = (strncmp(
                line.as_mut_ptr().offset(4 as libc::c_int as isize),
                b" ...\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int) as libc::c_int;
            ok = yaml_document_end_event_initialize(&mut event, implicit);
        } else if strncmp(
                line.as_mut_ptr(),
                b"+MAP\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            style = YAML_BLOCK_MAPPING_STYLE as libc::c_int;
            if flow == 1 as libc::c_int {
                style = YAML_FLOW_MAPPING_STYLE as libc::c_int;
            } else if flow == 0 as libc::c_int
                    && strncmp(
                        line.as_mut_ptr().offset(5 as libc::c_int as isize),
                        b"{}\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                style = YAML_FLOW_MAPPING_STYLE as libc::c_int;
            }
            ok = yaml_mapping_start_event_initialize(
                &mut event,
                get_anchor(
                    '&' as i32 as libc::c_char,
                    line.as_mut_ptr(),
                    anchor.as_mut_ptr(),
                ) as *mut yaml_char_t,
                get_tag(line.as_mut_ptr(), tag.as_mut_ptr()) as *mut yaml_char_t,
                0 as libc::c_int,
                style as yaml_mapping_style_t,
            );
        } else if strncmp(
                line.as_mut_ptr(),
                b"-MAP\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            ok = yaml_mapping_end_event_initialize(&mut event);
        } else if strncmp(
                line.as_mut_ptr(),
                b"+SEQ\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            style = YAML_BLOCK_SEQUENCE_STYLE as libc::c_int;
            if flow == 1 as libc::c_int {
                style = YAML_FLOW_MAPPING_STYLE as libc::c_int;
            } else if flow == 0 as libc::c_int
                    && strncmp(
                        line.as_mut_ptr().offset(5 as libc::c_int as isize),
                        b"[]\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                style = YAML_FLOW_SEQUENCE_STYLE as libc::c_int;
            }
            ok = yaml_sequence_start_event_initialize(
                &mut event,
                get_anchor(
                    '&' as i32 as libc::c_char,
                    line.as_mut_ptr(),
                    anchor.as_mut_ptr(),
                ) as *mut yaml_char_t,
                get_tag(line.as_mut_ptr(), tag.as_mut_ptr()) as *mut yaml_char_t,
                0 as libc::c_int,
                style as yaml_sequence_style_t,
            );
        } else if strncmp(
                line.as_mut_ptr(),
                b"-SEQ\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            ok = yaml_sequence_end_event_initialize(&mut event);
        } else if strncmp(
                line.as_mut_ptr(),
                b"=VAL\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            let mut value: [libc::c_char; 1024] = [0; 1024];
            let mut style_0: libc::c_int = 0;
            get_value(line.as_mut_ptr(), value.as_mut_ptr(), &mut style_0);
            implicit = (get_tag(line.as_mut_ptr(), tag.as_mut_ptr())
                == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
            ok = yaml_scalar_event_initialize(
                &mut event,
                get_anchor(
                    '&' as i32 as libc::c_char,
                    line.as_mut_ptr(),
                    anchor.as_mut_ptr(),
                ) as *mut yaml_char_t,
                get_tag(line.as_mut_ptr(), tag.as_mut_ptr()) as *mut yaml_char_t,
                value.as_mut_ptr() as *mut yaml_char_t,
                -(1 as libc::c_int),
                implicit,
                implicit,
                style_0 as yaml_scalar_style_t,
            );
        } else if strncmp(
                line.as_mut_ptr(),
                b"=ALI\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
            ok = yaml_alias_event_initialize(
                &mut event,
                get_anchor(
                    '*' as i32 as libc::c_char,
                    line.as_mut_ptr(),
                    anchor.as_mut_ptr(),
                ) as *mut yaml_char_t,
            );
        } else {
            fprintf(
                stderr,
                b"Unknown event: '%s'\n\0" as *const u8 as *const libc::c_char,
                line.as_mut_ptr(),
            );
            fflush(stdout);
            return 1 as libc::c_int;
        }
        if ok == 0 {
            current_block = 13850764817919632987;
            break;
        }
        if yaml_emitter_emit(&mut emitter, &mut event) == 0 {
            current_block = 6684355725484023210;
            break;
        }
    }
    match current_block {
        13850764817919632987 => {
            fprintf(
                stderr,
                b"Memory error: Not enough memory for creating an event\n\0" as *const u8
                    as *const libc::c_char,
            );
            yaml_emitter_delete(&mut emitter);
            return 1 as libc::c_int;
        }
        6684355725484023210 => {
            match emitter.error as libc::c_uint {
                1 => {
                    fprintf(
                        stderr,
                        b"Memory error: Not enough memory for emitting\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                6 => {
                    fprintf(
                        stderr,
                        b"Writer error: %s\n\0" as *const u8 as *const libc::c_char,
                        emitter.problem,
                    );
                }
                7 => {
                    fprintf(
                        stderr,
                        b"Emitter error: %s\n\0" as *const u8 as *const libc::c_char,
                        emitter.problem,
                    );
                }
                _ => {
                    fprintf(
                        stderr,
                        b"Internal error\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            yaml_emitter_delete(&mut emitter);
            return 1 as libc::c_int;
        }
        _ => {
            if fclose(input) == 0 {} else {
                __assert_fail(
                    b"!fclose(input)\0" as *const u8 as *const libc::c_char,
                    b"run-emitter-test-suite.c\0" as *const u8 as *const libc::c_char,
                    157 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
            yaml_emitter_delete(&mut emitter);
            fflush(stdout);
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_line(
    mut input: *mut FILE,
    mut line: *mut libc::c_char,
) -> libc::c_int {
    let mut newline: *mut libc::c_char = 0 as *mut libc::c_char;
    if (fgets(line, 1024 as libc::c_int - 1 as libc::c_int, input)).is_null() {
        return 0 as libc::c_int;
    }
    newline = strchr(line, '\n' as i32);
    if newline.is_null() {
        fprintf(
            stderr,
            b"Line too long: '%s'\0" as *const u8 as *const libc::c_char,
            line,
        );
        abort();
    }
    *newline = '\0' as i32 as libc::c_char;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_anchor(
    mut sigil: libc::c_char,
    mut line: *mut libc::c_char,
    mut anchor: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    start = strchr(line, sigil as libc::c_int);
    if start.is_null() {
        return 0 as *mut libc::c_char;
    }
    start = start.offset(1);
    end = strchr(start, ' ' as i32);
    if end.is_null() {
        end = line.offset(strlen(line) as isize);
    }
    memcpy(
        anchor as *mut libc::c_void,
        start as *const libc::c_void,
        end.offset_from(start) as libc::c_long as libc::c_ulong,
    );
    *anchor
        .offset(
            end.offset_from(start) as libc::c_long as isize,
        ) = '\0' as i32 as libc::c_char;
    return anchor;
}
#[no_mangle]
pub unsafe extern "C" fn get_tag(
    mut line: *mut libc::c_char,
    mut tag: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    start = strchr(line, '<' as i32);
    if start.is_null() {
        return 0 as *mut libc::c_char;
    }
    end = strchr(line, '>' as i32);
    if end.is_null() {
        return 0 as *mut libc::c_char;
    }
    memcpy(
        tag as *mut libc::c_void,
        start.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (end.offset_from(start) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_ulong,
    );
    *tag
        .offset(
            (end.offset_from(start) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as isize,
        ) = '\0' as i32 as libc::c_char;
    return tag;
}
#[no_mangle]
pub unsafe extern "C" fn get_value(
    mut line: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut style: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = line.offset(strlen(line) as isize);
    let mut current_block_8: u64;
    c = line.offset(4 as libc::c_int as isize);
    while c < end {
        if *c as libc::c_int == ' ' as i32 {
            start = c.offset(1 as libc::c_int as isize);
            if *start as libc::c_int == ':' as i32 {
                *style = YAML_PLAIN_SCALAR_STYLE as libc::c_int;
                current_block_8 = 17407779659766490442;
            } else if *start as libc::c_int == '\'' as i32 {
                *style = YAML_SINGLE_QUOTED_SCALAR_STYLE as libc::c_int;
                current_block_8 = 17407779659766490442;
            } else if *start as libc::c_int == '"' as i32 {
                *style = YAML_DOUBLE_QUOTED_SCALAR_STYLE as libc::c_int;
                current_block_8 = 17407779659766490442;
            } else if *start as libc::c_int == '|' as i32 {
                *style = YAML_LITERAL_SCALAR_STYLE as libc::c_int;
                current_block_8 = 17407779659766490442;
            } else if *start as libc::c_int == '>' as i32 {
                *style = YAML_FOLDED_SCALAR_STYLE as libc::c_int;
                current_block_8 = 17407779659766490442;
            } else {
                start = 0 as *mut libc::c_char;
                current_block_8 = 12675440807659640239;
            }
            match current_block_8 {
                12675440807659640239 => {}
                _ => {
                    start = start.offset(1);
                    break;
                }
            }
        }
        c = c.offset(1);
    }
    if start.is_null() {
        abort();
    }
    c = start;
    while c < end {
        if *c as libc::c_int == '\\' as i32 {
            c = c.offset(1);
            if *c as libc::c_int == '\\' as i32 {
                let fresh0 = i;
                i = i + 1;
                *value.offset(fresh0 as isize) = '\\' as i32 as libc::c_char;
            } else if *c as libc::c_int == '0' as i32 {
                let fresh1 = i;
                i = i + 1;
                *value.offset(fresh1 as isize) = '\0' as i32 as libc::c_char;
            } else if *c as libc::c_int == 'b' as i32 {
                let fresh2 = i;
                i = i + 1;
                *value.offset(fresh2 as isize) = '\u{8}' as i32 as libc::c_char;
            } else if *c as libc::c_int == 'n' as i32 {
                let fresh3 = i;
                i = i + 1;
                *value.offset(fresh3 as isize) = '\n' as i32 as libc::c_char;
            } else if *c as libc::c_int == 'r' as i32 {
                let fresh4 = i;
                i = i + 1;
                *value.offset(fresh4 as isize) = '\r' as i32 as libc::c_char;
            } else if *c as libc::c_int == 't' as i32 {
                let fresh5 = i;
                i = i + 1;
                *value.offset(fresh5 as isize) = '\t' as i32 as libc::c_char;
            } else {
                abort();
            }
        } else {
            let fresh6 = i;
            i = i + 1;
            *value.offset(fresh6 as isize) = *c;
        }
        c = c.offset(1);
    }
    *value.offset(i as isize) = '\0' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut ret: libc::c_int) -> libc::c_int {
    fprintf(
        stderr,
        b"Usage: run-emitter-test-suite [--directive (1.1|1.2)] [--flow (on|off|keep)] [<input-file>]\n\0"
            as *const u8 as *const libc::c_char,
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