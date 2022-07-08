#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::items_after_statements,
    clippy::let_underscore_drop,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::ptr_as_ptr,
    clippy::single_match_else,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

use std::env;
use std::error::Error;
use std::ffi::{c_void, CStr};
use std::fs::File;
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::process::{self, ExitCode};
use std::ptr::{self, addr_of_mut};
use std::slice;
use unsafe_libyaml::{
    yaml_alias_event_initialize, yaml_document_end_event_initialize,
    yaml_document_start_event_initialize, yaml_emitter_delete, yaml_emitter_emit,
    yaml_emitter_initialize, yaml_emitter_set_canonical, yaml_emitter_set_output,
    yaml_emitter_set_unicode, yaml_emitter_t, yaml_event_t, yaml_mapping_end_event_initialize,
    yaml_mapping_start_event_initialize, yaml_scalar_event_initialize, yaml_scalar_style_t,
    yaml_sequence_end_event_initialize, yaml_sequence_start_event_initialize,
    yaml_stream_end_event_initialize, yaml_stream_start_event_initialize, yaml_tag_directive_t,
    yaml_version_directive_t, YAML_ANY_SCALAR_STYLE, YAML_BLOCK_MAPPING_STYLE,
    YAML_BLOCK_SEQUENCE_STYLE, YAML_DOUBLE_QUOTED_SCALAR_STYLE, YAML_FOLDED_SCALAR_STYLE,
    YAML_LITERAL_SCALAR_STYLE, YAML_PLAIN_SCALAR_STYLE, YAML_SINGLE_QUOTED_SCALAR_STYLE,
    YAML_UTF8_ENCODING,
};
pub unsafe fn unsafe_main(
    stdin: &mut dyn Read,
    mut stdout: &mut dyn Write,
) -> Result<(), Box<dyn Error>> {
    let current_block: u64;
    let mut emitter = MaybeUninit::<yaml_emitter_t>::uninit();
    let emitter = emitter.as_mut_ptr();
    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    let version_directive: *mut yaml_version_directive_t =
        ptr::null_mut::<yaml_version_directive_t>();
    let canonical = 0_i32;
    let unicode = 0_i32;
    let mut buf = ReadBuf::new();
    if yaml_emitter_initialize(emitter) == 0 {
        return Err("Could not initalize the emitter object".into());
    }
    unsafe fn write_to_stdio(data: *mut c_void, buffer: *mut u8, size: u64) -> i32 {
        let stdout: *mut &mut dyn Write = data.cast();
        let bytes = slice::from_raw_parts(buffer.cast(), size as usize);
        match (*stdout).write(bytes) {
            Ok(n) => n as i32,
            Err(_) => 0,
        }
    }
    yaml_emitter_set_output(emitter, Some(write_to_stdio), addr_of_mut!(stdout).cast());
    yaml_emitter_set_canonical(emitter, canonical);
    yaml_emitter_set_unicode(emitter, unicode);
    loop {
        let line = match buf.get_line(stdin) {
            Some(line) => line,
            None => {
                current_block = 1934991416718554651;
                break;
            }
        };
        let line = line as *mut [u8] as *mut i8;
        let ok: i32;
        let mut anchor: [i8; 256] = [0; 256];
        let mut tag: [i8; 256] = [0; 256];
        let implicit: i32;
        if strncmp(line, b"+STR\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            ok = yaml_stream_start_event_initialize(event, YAML_UTF8_ENCODING);
        } else if strncmp(line, b"-STR\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            ok = yaml_stream_end_event_initialize(event);
        } else if strncmp(line, b"+DOC\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            implicit = (strncmp(
                line.offset(4_isize),
                b" ---\0" as *const u8 as *const i8,
                4_u64,
            ) != 0_i32) as i32;
            ok = yaml_document_start_event_initialize(
                event,
                version_directive,
                ptr::null_mut::<yaml_tag_directive_t>(),
                ptr::null_mut::<yaml_tag_directive_t>(),
                implicit,
            );
        } else if strncmp(line, b"-DOC\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            implicit = (strncmp(
                line.offset(4_isize),
                b" ...\0" as *const u8 as *const i8,
                4_u64,
            ) != 0_i32) as i32;
            ok = yaml_document_end_event_initialize(event, implicit);
        } else if strncmp(line, b"+MAP\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            ok = yaml_mapping_start_event_initialize(
                event,
                get_anchor('&' as i32 as i8, line, anchor.as_mut_ptr()) as *mut u8,
                get_tag(line, tag.as_mut_ptr()) as *mut u8,
                0_i32,
                YAML_BLOCK_MAPPING_STYLE,
            );
        } else if strncmp(line, b"-MAP\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            ok = yaml_mapping_end_event_initialize(event);
        } else if strncmp(line, b"+SEQ\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            ok = yaml_sequence_start_event_initialize(
                event,
                get_anchor('&' as i32 as i8, line, anchor.as_mut_ptr()) as *mut u8,
                get_tag(line, tag.as_mut_ptr()) as *mut u8,
                0_i32,
                YAML_BLOCK_SEQUENCE_STYLE,
            );
        } else if strncmp(line, b"-SEQ\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            ok = yaml_sequence_end_event_initialize(event);
        } else if strncmp(line, b"=VAL\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            let mut value: [i8; 1024] = [0; 1024];
            let mut style = YAML_ANY_SCALAR_STYLE;
            get_value(line, value.as_mut_ptr(), &mut style);
            implicit =
                (get_tag(line, tag.as_mut_ptr()) == ptr::null_mut::<c_void>() as *mut i8) as i32;
            ok = yaml_scalar_event_initialize(
                event,
                get_anchor('&' as i32 as i8, line, anchor.as_mut_ptr()) as *mut u8,
                get_tag(line, tag.as_mut_ptr()) as *mut u8,
                value.as_mut_ptr() as *mut u8,
                -1_i32,
                implicit,
                implicit,
                style,
            );
        } else if strncmp(line, b"=ALI\0" as *const u8 as *const i8, 4_u64) == 0_i32 {
            ok = yaml_alias_event_initialize(
                event,
                get_anchor('*' as i32 as i8, line, anchor.as_mut_ptr()) as *mut u8,
            );
        } else {
            yaml_emitter_delete(emitter);
            return Err(format!(
                "Unknown event: '{}'",
                CStr::from_ptr(line).to_string_lossy(),
            )
            .into());
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
    let result = match current_block {
        13850764817919632987 => Err("Memory error: Not enough memory for creating an event".into()),
        6684355725484023210 => Err(match (*emitter).error as u32 {
            1 => "Memory error: Not enough memory for emitting".into(),
            6 => format!(
                "Writer error: {}",
                CStr::from_ptr((*emitter).problem).to_string_lossy(),
            )
            .into(),
            7 => format!(
                "Emitter error: {}",
                CStr::from_ptr((*emitter).problem).to_string_lossy(),
            )
            .into(),
            _ => "Internal error".into(),
        }),
        _ => Ok(()),
    };
    yaml_emitter_delete(emitter);
    result
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
    fn get_line(&mut self, input: &mut dyn Read) -> Option<&mut [u8]> {
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
pub unsafe fn get_anchor(sigil: i8, line: *mut i8, anchor: *mut i8) -> *mut i8 {
    let mut start: *mut i8;
    let mut end: *mut i8;
    start = strchr(line, sigil as i32);
    if start.is_null() {
        return ptr::null_mut::<i8>();
    }
    start = start.offset(1);
    end = strchr(start, ' ' as i32);
    if end.is_null() {
        end = line.offset(strlen(line) as isize);
    }
    memcpy(
        anchor as *mut c_void,
        start as *const c_void,
        end.offset_from(start) as i64 as u64,
    );
    *anchor.offset(end.offset_from(start) as i64 as isize) = '\0' as i32 as i8;
    anchor
}
pub unsafe fn get_tag(line: *mut i8, tag: *mut i8) -> *mut i8 {
    let start: *mut i8 = strchr(line, '<' as i32);
    if start.is_null() {
        return ptr::null_mut::<i8>();
    }
    let end: *mut i8 = strchr(line, '>' as i32);
    if end.is_null() {
        return ptr::null_mut::<i8>();
    }
    memcpy(
        tag as *mut c_void,
        start.offset(1_isize) as *const c_void,
        (end.offset_from(start) as i64 - 1_i64) as u64,
    );
    *tag.offset((end.offset_from(start) as i64 - 1_i64) as isize) = '\0' as i32 as i8;
    tag
}
pub unsafe fn get_value(line: *mut i8, value: *mut i8, style: *mut yaml_scalar_style_t) {
    let mut i: i32 = 0_i32;
    let mut c: *mut i8;
    let mut start: *mut i8 = ptr::null_mut::<i8>();
    let end: *mut i8 = line.offset(strlen(line) as isize);
    let mut current_block_8: u64;
    c = line.offset(4_isize);
    while c < end {
        if *c as i32 == ' ' as i32 {
            start = c.offset(1_isize);
            if *start as i32 == ':' as i32 {
                *style = YAML_PLAIN_SCALAR_STYLE;
                current_block_8 = 17407779659766490442;
            } else if *start as i32 == '\'' as i32 {
                *style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
                current_block_8 = 17407779659766490442;
            } else if *start as i32 == '"' as i32 {
                *style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
                current_block_8 = 17407779659766490442;
            } else if *start as i32 == '|' as i32 {
                *style = YAML_LITERAL_SCALAR_STYLE;
                current_block_8 = 17407779659766490442;
            } else if *start as i32 == '>' as i32 {
                *style = YAML_FOLDED_SCALAR_STYLE;
                current_block_8 = 17407779659766490442;
            } else {
                start = ptr::null_mut::<i8>();
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
        if *c as i32 == '\\' as i32 {
            c = c.offset(1);
            if *c as i32 == '\\' as i32 {
                let fresh0 = i;
                i += 1;
                *value.offset(fresh0 as isize) = '\\' as i32 as i8;
            } else if *c as i32 == '0' as i32 {
                let fresh1 = i;
                i += 1;
                *value.offset(fresh1 as isize) = '\0' as i32 as i8;
            } else if *c as i32 == 'b' as i32 {
                let fresh2 = i;
                i += 1;
                *value.offset(fresh2 as isize) = '\u{8}' as i32 as i8;
            } else if *c as i32 == 'n' as i32 {
                let fresh3 = i;
                i += 1;
                *value.offset(fresh3 as isize) = '\n' as i32 as i8;
            } else if *c as i32 == 'r' as i32 {
                let fresh4 = i;
                i += 1;
                *value.offset(fresh4 as isize) = '\r' as i32 as i8;
            } else if *c as i32 == 't' as i32 {
                let fresh5 = i;
                i += 1;
                *value.offset(fresh5 as isize) = '\t' as i32 as i8;
            } else {
                process::abort();
            }
        } else {
            let fresh6 = i;
            i += 1;
            *value.offset(fresh6 as isize) = *c;
        }
        c = c.offset(1);
    }
    *value.offset(i as isize) = '\0' as i32 as i8;
}
unsafe fn memcpy(dest: *mut c_void, src: *const c_void, count: u64) -> *mut c_void {
    ptr::copy_nonoverlapping(
        src.cast::<MaybeUninit<u8>>(),
        dest.cast::<MaybeUninit<u8>>(),
        count as usize,
    );
    dest
}
unsafe fn strchr(mut str: *const i8, c: i32) -> *mut i8 {
    loop {
        match *str {
            0 => return ptr::null_mut(),
            curr if curr == c as i8 => return str as *mut i8,
            _ => str = str.offset(1),
        }
    }
}
unsafe fn strlen(str: *const i8) -> u64 {
    let mut end = str;
    while *end != 0 {
        end = end.add(1);
    }
    end.offset_from(str) as u64
}
unsafe fn strncmp(lhs: *const i8, rhs: *const i8, mut count: u64) -> i32 {
    let mut lhs = lhs.cast::<u8>();
    let mut rhs = rhs.cast::<u8>();
    while count > 0 && *lhs != 0 && *lhs == *rhs {
        lhs = lhs.add(1);
        rhs = rhs.add(1);
        count -= 1;
    }
    if count == 0 {
        0
    } else {
        (*lhs).cmp(&*rhs) as i32
    }
}
fn main() -> ExitCode {
    let args = env::args_os().skip(1);
    if args.len() == 0 {
        let _ = writeln!(
            io::stderr(),
            "Usage: run-emitter-test-suite <test.event>...",
        );
        return ExitCode::FAILURE;
    }
    for arg in args {
        let mut stdin = File::open(arg).unwrap();
        let mut stdout = io::stdout();
        let result = unsafe { unsafe_main(&mut stdin, &mut stdout) };
        if let Err(err) = result {
            let _ = writeln!(io::stderr(), "{}", err);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
