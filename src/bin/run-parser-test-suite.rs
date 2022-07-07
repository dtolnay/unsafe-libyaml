use std::cmp;
use std::env;
use std::ffi::CStr;
use std::fs;
use std::io::{self, Write as _};
use std::mem::MaybeUninit;
use std::process::{self, ExitCode};
use std::ptr;
use unsafe_libyaml::api::{
    yaml_event_delete, yaml_parser_delete, yaml_parser_initialize, yaml_parser_set_input,
};
use unsafe_libyaml::parser::yaml_parser_parse;
use unsafe_libyaml::*;
unsafe fn unsafe_main() -> ExitCode {
    let mut input = None;
    let mut parser = MaybeUninit::<yaml_parser_t>::uninit();
    let parser = parser.as_mut_ptr();
    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    let mut foundfile: libc::c_int = 0 as libc::c_int;
    for arg in env::args_os().skip(1) {
        if foundfile == 0 {
            input = fs::read(arg).ok();
            foundfile = 1 as libc::c_int;
        } else {
            return usage(ExitCode::FAILURE);
        }
    }
    let input = input.unwrap_or_else(|| __assert!(false));
    if yaml_parser_initialize(parser) == 0 {
        let _ = writeln!(io::stderr(), "Could not initialize the parser object");
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
        parser,
        Some(read_from_file),
        ptr::addr_of_mut!(remaining).cast(),
    );
    loop {
        if yaml_parser_parse(parser, event) == 0 {
            let _ = writeln!(
                io::stderr(),
                "Parse error: {}",
                CStr::from_ptr((*parser).problem).to_string_lossy(),
            );
            if (*parser).problem_mark.line != 0 || (*parser).problem_mark.column != 0 {
                let _ = writeln!(
                    io::stderr(),
                    "Line: {} Column: {}",
                    ((*parser).problem_mark.line).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ((*parser).problem_mark.column).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            }
            return ExitCode::FAILURE;
        }
        let type_0: yaml_event_type_t = (*event).type_0;
        if type_0 as libc::c_uint == YAML_NO_EVENT as libc::c_int as libc::c_uint {
            let _ = writeln!(io::stdout(), "???");
        } else if type_0 as libc::c_uint == YAML_STREAM_START_EVENT as libc::c_int as libc::c_uint {
            let _ = writeln!(io::stdout(), "+STR");
        } else if type_0 as libc::c_uint == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint {
            let _ = writeln!(io::stdout(), "-STR");
        } else if type_0 as libc::c_uint == YAML_DOCUMENT_START_EVENT as libc::c_int as libc::c_uint
        {
            let _ = write!(io::stdout(), "+DOC");
            if (*event).data.document_start.implicit == 0 {
                let _ = write!(io::stdout(), " ---");
            }
            let _ = writeln!(io::stdout());
        } else if type_0 as libc::c_uint == YAML_DOCUMENT_END_EVENT as libc::c_int as libc::c_uint {
            let _ = write!(io::stdout(), "-DOC");
            if (*event).data.document_end.implicit == 0 {
                let _ = write!(io::stdout(), " ...");
            }
            let _ = writeln!(io::stdout());
        } else if type_0 as libc::c_uint == YAML_MAPPING_START_EVENT as libc::c_int as libc::c_uint
        {
            let _ = write!(io::stdout(), "+MAP");
            if !((*event).data.mapping_start.anchor).is_null() {
                let _ = write!(
                    io::stdout(),
                    " &{}",
                    CStr::from_ptr((*event).data.mapping_start.anchor as *const libc::c_char)
                        .to_string_lossy(),
                );
            }
            if !((*event).data.mapping_start.tag).is_null() {
                let _ = write!(
                    io::stdout(),
                    " <{}>",
                    CStr::from_ptr((*event).data.mapping_start.tag as *const libc::c_char)
                        .to_string_lossy(),
                );
            }
            let _ = writeln!(io::stdout());
        } else if type_0 as libc::c_uint == YAML_MAPPING_END_EVENT as libc::c_int as libc::c_uint {
            let _ = writeln!(io::stdout(), "-MAP");
        } else if type_0 as libc::c_uint == YAML_SEQUENCE_START_EVENT as libc::c_int as libc::c_uint
        {
            let _ = write!(io::stdout(), "+SEQ");
            if !((*event).data.sequence_start.anchor).is_null() {
                let _ = write!(
                    io::stdout(),
                    " &{}",
                    CStr::from_ptr((*event).data.sequence_start.anchor as *const libc::c_char)
                        .to_string_lossy(),
                );
            }
            if !((*event).data.sequence_start.tag).is_null() {
                let _ = write!(
                    io::stdout(),
                    " <{}>",
                    CStr::from_ptr((*event).data.sequence_start.tag as *const libc::c_char)
                        .to_string_lossy(),
                );
            }
            let _ = writeln!(io::stdout());
        } else if type_0 as libc::c_uint == YAML_SEQUENCE_END_EVENT as libc::c_int as libc::c_uint {
            let _ = writeln!(io::stdout(), "-SEQ");
        } else if type_0 as libc::c_uint == YAML_SCALAR_EVENT as libc::c_int as libc::c_uint {
            let _ = write!(io::stdout(), "=VAL");
            if !((*event).data.scalar.anchor).is_null() {
                let _ = write!(
                    io::stdout(),
                    " &{}",
                    CStr::from_ptr((*event).data.scalar.anchor as *const libc::c_char)
                        .to_string_lossy(),
                );
            }
            if !((*event).data.scalar.tag).is_null() {
                let _ = write!(
                    io::stdout(),
                    " <{}>",
                    CStr::from_ptr((*event).data.scalar.tag as *const libc::c_char)
                        .to_string_lossy(),
                );
            }
            match (*event).data.scalar.style as libc::c_uint {
                1 => {
                    let _ = write!(io::stdout(), " :");
                }
                2 => {
                    let _ = write!(io::stdout(), " '");
                }
                3 => {
                    let _ = write!(io::stdout(), " \"");
                }
                4 => {
                    let _ = write!(io::stdout(), " |");
                }
                5 => {
                    let _ = write!(io::stdout(), " >");
                }
                0 => {
                    process::abort();
                }
                _ => {}
            }
            print_escaped((*event).data.scalar.value, (*event).data.scalar.length);
            let _ = writeln!(io::stdout());
        } else if type_0 as libc::c_uint == YAML_ALIAS_EVENT as libc::c_int as libc::c_uint {
            let _ = writeln!(
                io::stdout(),
                "=ALI *{}",
                CStr::from_ptr((*event).data.alias.anchor as *const libc::c_char).to_string_lossy(),
            );
        } else {
            process::abort();
        }
        yaml_event_delete(event);
        if type_0 as libc::c_uint == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint {
            break;
        }
    }
    yaml_parser_delete(parser);
    ExitCode::SUCCESS
}
#[no_mangle]
pub unsafe extern "C" fn print_escaped(str: *mut yaml_char_t, length: size_t) {
    let mut i: libc::c_int;
    let mut c: libc::c_char;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < length {
        c = *str.offset(i as isize) as libc::c_char;
        if c as libc::c_int == '\\' as i32 {
            let _ = write!(io::stdout(), "\\\\");
        } else if c as libc::c_int == '\0' as i32 {
            let _ = write!(io::stdout(), "\\0");
        } else if c as libc::c_int == '\u{8}' as i32 {
            let _ = write!(io::stdout(), "\\b");
        } else if c as libc::c_int == '\n' as i32 {
            let _ = write!(io::stdout(), "\\n");
        } else if c as libc::c_int == '\r' as i32 {
            let _ = write!(io::stdout(), "\\r");
        } else if c as libc::c_int == '\t' as i32 {
            let _ = write!(io::stdout(), "\\t");
        } else {
            let _ = io::stdout().write_all(&[c as u8]);
        }
        i += 1;
    }
}
unsafe fn usage(ret: ExitCode) -> ExitCode {
    let _ = writeln!(io::stderr(), "Usage: libyaml-parser [<input-file>]");
    ret
}
fn main() -> ExitCode {
    unsafe { unsafe_main() }
}
