#![allow(non_camel_case_types, non_snake_case, unused_assignments, unused_mut)]

use std::env;
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Read as _, Write as _};
use std::mem::MaybeUninit;
use std::process::{self, ExitCode};
use std::ptr;
use std::slice;
use unsafe_libyaml::api::{
    yaml_alias_event_initialize, yaml_document_end_event_initialize,
    yaml_document_start_event_initialize, yaml_emitter_delete, yaml_emitter_initialize,
    yaml_emitter_set_canonical, yaml_emitter_set_output, yaml_emitter_set_unicode,
    yaml_mapping_end_event_initialize, yaml_mapping_start_event_initialize,
    yaml_scalar_event_initialize, yaml_sequence_end_event_initialize,
    yaml_sequence_start_event_initialize, yaml_stream_end_event_initialize,
    yaml_stream_start_event_initialize,
};
use unsafe_libyaml::emitter::yaml_emitter_emit;
use unsafe_libyaml::externs::{memcpy, strlen, strncmp};
use unsafe_libyaml::*;
unsafe fn unsafe_main() -> ExitCode {
    let mut current_block: u64;
    let mut input = None;
    let mut emitter = MaybeUninit::<yaml_emitter_t>::uninit();
    let emitter = emitter.as_mut_ptr();
    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    let mut version_directive: *mut yaml_version_directive_t =
        ptr::null_mut::<yaml_version_directive_t>();
    let mut canonical: libc::c_int = 0 as libc::c_int;
    let mut unicode: libc::c_int = 0 as libc::c_int;
    let mut buf = ReadBuf::new();
    let mut foundfile: libc::c_int = 0 as libc::c_int;
    for arg in env::args().skip(1) {
        if foundfile == 0 {
            input = File::open(arg).ok();
            foundfile = 1 as libc::c_int;
        }
    }
    let input = input.unwrap_or_else(|| {
        __assert_fail!(
            b"input\0" as *const u8 as *const libc::c_char,
            b"run-emitter-test-suite.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    });
    if yaml_emitter_initialize(emitter) == 0 {
        let _ = writeln!(io::stderr(), "Could not initalize the emitter object");
        return ExitCode::FAILURE;
    }
    unsafe extern "C" fn write_to_stdout(
        _data: *mut libc::c_void,
        buffer: *mut libc::c_uchar,
        size: size_t,
    ) -> libc::c_int {
        let bytes = slice::from_raw_parts(buffer.cast(), size as usize);
        let _ = io::stdout().write_all(bytes);
        size as libc::c_int
    }
    yaml_emitter_set_output(
        emitter,
        Some(write_to_stdout),
        ptr::null_mut::<libc::c_void>(),
    );
    yaml_emitter_set_canonical(emitter, canonical);
    yaml_emitter_set_unicode(emitter, unicode);
    loop {
        let line = match buf.get_line(&input) {
            Some(line) => line,
            None => {
                current_block = 1934991416718554651;
                break;
            }
        };
        let line = line as *mut [u8] as *mut libc::c_char;
        let mut ok: libc::c_int = 0;
        let mut anchor: [libc::c_char; 256] = [0; 256];
        let mut tag: [libc::c_char; 256] = [0; 256];
        let mut implicit: libc::c_int = 0;
        let mut style: libc::c_int = 0;
        if strncmp(
            line,
            b"+STR\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ok = yaml_stream_start_event_initialize(event, YAML_UTF8_ENCODING);
        } else if strncmp(
            line,
            b"-STR\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ok = yaml_stream_end_event_initialize(event);
        } else if strncmp(
            line,
            b"+DOC\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            implicit = (strncmp(
                line.offset(4 as libc::c_int as isize),
                b" ---\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int) as libc::c_int;
            ok = yaml_document_start_event_initialize(
                event,
                version_directive,
                ptr::null_mut::<yaml_tag_directive_t>(),
                ptr::null_mut::<yaml_tag_directive_t>(),
                implicit,
            );
        } else if strncmp(
            line,
            b"-DOC\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            implicit = (strncmp(
                line.offset(4 as libc::c_int as isize),
                b" ...\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int) as libc::c_int;
            ok = yaml_document_end_event_initialize(event, implicit);
        } else if strncmp(
            line,
            b"+MAP\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            style = YAML_BLOCK_MAPPING_STYLE as libc::c_int;
            ok = yaml_mapping_start_event_initialize(
                event,
                get_anchor('&' as i32 as libc::c_char, line, anchor.as_mut_ptr())
                    as *mut yaml_char_t,
                get_tag(line, tag.as_mut_ptr()) as *mut yaml_char_t,
                0 as libc::c_int,
                style as yaml_mapping_style_t,
            );
        } else if strncmp(
            line,
            b"-MAP\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ok = yaml_mapping_end_event_initialize(event);
        } else if strncmp(
            line,
            b"+SEQ\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            style = YAML_BLOCK_SEQUENCE_STYLE as libc::c_int;
            ok = yaml_sequence_start_event_initialize(
                event,
                get_anchor('&' as i32 as libc::c_char, line, anchor.as_mut_ptr())
                    as *mut yaml_char_t,
                get_tag(line, tag.as_mut_ptr()) as *mut yaml_char_t,
                0 as libc::c_int,
                style as yaml_sequence_style_t,
            );
        } else if strncmp(
            line,
            b"-SEQ\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ok = yaml_sequence_end_event_initialize(event);
        } else if strncmp(
            line,
            b"=VAL\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut value: [libc::c_char; 1024] = [0; 1024];
            let mut style_0: libc::c_int = 0;
            get_value(line, value.as_mut_ptr(), &mut style_0);
            implicit = (get_tag(line, tag.as_mut_ptr())
                == ptr::null_mut::<libc::c_void>() as *mut libc::c_char)
                as libc::c_int;
            ok = yaml_scalar_event_initialize(
                event,
                get_anchor('&' as i32 as libc::c_char, line, anchor.as_mut_ptr())
                    as *mut yaml_char_t,
                get_tag(line, tag.as_mut_ptr()) as *mut yaml_char_t,
                value.as_mut_ptr() as *mut yaml_char_t,
                -(1 as libc::c_int),
                implicit,
                implicit,
                style_0 as yaml_scalar_style_t,
            );
        } else if strncmp(
            line,
            b"=ALI\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ok = yaml_alias_event_initialize(
                event,
                get_anchor('*' as i32 as libc::c_char, line, anchor.as_mut_ptr())
                    as *mut yaml_char_t,
            );
        } else {
            let _ = writeln!(
                io::stderr(),
                "Unknown event: '{}'",
                CStr::from_ptr(line).to_string_lossy(),
            );
            return ExitCode::FAILURE;
        }
        if ok == 0 {
            current_block = 13850764817919632987;
            break;
        }
        if yaml_emitter_emit(emitter, event) == 0 {
            current_block = 6684355725484023210;
            break;
        }
    }
    match current_block {
        13850764817919632987 => {
            let _ = writeln!(
                io::stderr(),
                "Memory error: Not enough memory for creating an event",
            );
            yaml_emitter_delete(emitter);
            return ExitCode::FAILURE;
        }
        6684355725484023210 => {
            match (*emitter).error as libc::c_uint {
                1 => {
                    let _ = writeln!(io::stderr(), "Memory error: Not enough memory for emitting");
                }
                6 => {
                    let _ = writeln!(
                        io::stderr(),
                        "Writer error: {}",
                        CStr::from_ptr((*emitter).problem).to_string_lossy(),
                    );
                }
                7 => {
                    let _ = writeln!(
                        io::stderr(),
                        "Emitter error: {}",
                        CStr::from_ptr((*emitter).problem).to_string_lossy(),
                    );
                }
                _ => {
                    let _ = writeln!(io::stderr(), "Internal error");
                }
            }
            yaml_emitter_delete(emitter);
            return ExitCode::FAILURE;
        }
        _ => {
            yaml_emitter_delete(emitter);
            return ExitCode::SUCCESS;
        }
    };
}
struct ReadBuf {
    buf: [u8; 1024],
    offset: usize,
    filled: usize,
}
impl ReadBuf {
    fn new() -> Self {
        ReadBuf {
            buf: [0; 1024],
            offset: 0,
            filled: 0,
        }
    }
    fn get_line(&mut self, mut input: &File) -> Option<&mut [u8]> {
        loop {
            for i in self.offset..self.offset + self.filled {
                if self.buf[i] == b'\n' {
                    self.buf[i] = b'\0';
                    let line = &mut self.buf[self.offset..=i];
                    self.offset = i + 1;
                    self.filled -= line.len();
                    return Some(line);
                }
            }
            let mut remainder = &mut self.buf[self.offset + self.filled..];
            if remainder.is_empty() {
                if self.offset == 0 {
                    let _ = writeln!(
                        io::stderr(),
                        "Line too long: '{}'",
                        String::from_utf8_lossy(&self.buf),
                    );
                    process::abort();
                }
                self.buf.copy_within(self.offset.., 0);
                self.offset = 0;
                remainder = &mut self.buf;
            }
            let n = input.read(remainder).ok()?;
            self.filled += n;
            if n == 0 {
                return None;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_anchor(
    mut sigil: libc::c_char,
    mut line: *mut libc::c_char,
    mut anchor: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut start: *mut libc::c_char = ptr::null_mut::<libc::c_char>();
    let mut end: *mut libc::c_char = ptr::null_mut::<libc::c_char>();
    start = strchr(line, sigil as libc::c_int);
    if start.is_null() {
        return ptr::null_mut::<libc::c_char>();
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
    *anchor.offset(end.offset_from(start) as libc::c_long as isize) = '\0' as i32 as libc::c_char;
    return anchor;
}
#[no_mangle]
pub unsafe extern "C" fn get_tag(
    mut line: *mut libc::c_char,
    mut tag: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut start: *mut libc::c_char = ptr::null_mut::<libc::c_char>();
    let mut end: *mut libc::c_char = ptr::null_mut::<libc::c_char>();
    start = strchr(line, '<' as i32);
    if start.is_null() {
        return ptr::null_mut::<libc::c_char>();
    }
    end = strchr(line, '>' as i32);
    if end.is_null() {
        return ptr::null_mut::<libc::c_char>();
    }
    memcpy(
        tag as *mut libc::c_void,
        start.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (end.offset_from(start) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_ulong,
    );
    *tag.offset(
        (end.offset_from(start) as libc::c_long - 1 as libc::c_int as libc::c_long) as isize,
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
    let mut c: *mut libc::c_char = ptr::null_mut::<libc::c_char>();
    let mut start: *mut libc::c_char = ptr::null_mut::<libc::c_char>();
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
                start = ptr::null_mut::<libc::c_char>();
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
        process::abort();
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
                process::abort();
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
unsafe fn strchr(mut str: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char {
    loop {
        match *str {
            0 => return ptr::null_mut(),
            curr if curr == c as libc::c_char => return str as *mut libc::c_char,
            _ => str = str.offset(1),
        }
    }
}
fn main() -> ExitCode {
    unsafe { unsafe_main() }
}
