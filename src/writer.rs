use crate::libc;
use crate::yaml::*;
use crate::PointerExt;
unsafe extern "C" fn yaml_emitter_set_writer_error(
    mut emitter: *mut yaml_emitter_t,
    mut problem: *const libc::c_char,
) -> libc::c_int {
    (*emitter).error = YAML_WRITER_ERROR;
    let ref mut fresh0 = (*emitter).problem;
    *fresh0 = problem;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_flush(mut emitter: *mut yaml_emitter_t) -> libc::c_int {
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    if !emitter.is_null() {
    } else {
        __assert_fail!(b"emitter\0" as *const u8 as *const libc::c_char);
    }
    if ((*emitter).write_handler).is_some() {
    } else {
        __assert_fail!(b"emitter->write_handler\0" as *const u8 as *const libc::c_char);
    }
    if (*emitter).encoding as u64 != 0 {
    } else {
        __assert_fail!(b"emitter->encoding\0" as *const u8 as *const libc::c_char);
    }
    let ref mut fresh1 = (*emitter).buffer.last;
    *fresh1 = (*emitter).buffer.pointer;
    let ref mut fresh2 = (*emitter).buffer.pointer;
    *fresh2 = (*emitter).buffer.start;
    if (*emitter).buffer.start == (*emitter).buffer.last {
        return 1 as libc::c_int;
    }
    if (*emitter).encoding as libc::c_uint == YAML_UTF8_ENCODING as libc::c_int as libc::c_uint {
        if ((*emitter).write_handler).expect("non-null function pointer")(
            (*emitter).write_handler_data,
            (*emitter).buffer.start,
            ((*emitter).buffer.last).c_offset_from((*emitter).buffer.start) as libc::c_long
                as size_t,
        ) != 0
        {
            let ref mut fresh3 = (*emitter).buffer.last;
            *fresh3 = (*emitter).buffer.start;
            let ref mut fresh4 = (*emitter).buffer.pointer;
            *fresh4 = (*emitter).buffer.start;
            return 1 as libc::c_int;
        } else {
            return yaml_emitter_set_writer_error(
                emitter,
                b"write error\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    low = if (*emitter).encoding as libc::c_uint
        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    high = if (*emitter).encoding as libc::c_uint
        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    while (*emitter).buffer.pointer != (*emitter).buffer.last {
        let mut octet: libc::c_uchar = 0;
        let mut width: libc::c_uint = 0;
        let mut value: libc::c_uint = 0;
        let mut k: size_t = 0;
        octet = *((*emitter).buffer.pointer).c_offset(0 as libc::c_int as isize);
        width = (if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
            1 as libc::c_int
        } else if octet as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
            2 as libc::c_int
        } else if octet as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
            3 as libc::c_int
        } else if octet as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        value = (if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
            octet as libc::c_int & 0x7f as libc::c_int
        } else if octet as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
            octet as libc::c_int & 0x1f as libc::c_int
        } else if octet as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
            octet as libc::c_int & 0xf as libc::c_int
        } else if octet as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
            octet as libc::c_int & 0x7 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        k = 1 as libc::c_int as size_t;
        while k < width as libc::c_ulong {
            octet = *((*emitter).buffer.pointer).c_offset(k as isize);
            value = (value << 6 as libc::c_int)
                .wrapping_add((octet as libc::c_int & 0x3f as libc::c_int) as libc::c_uint);
            k = k.wrapping_add(1);
        }
        let ref mut fresh5 = (*emitter).buffer.pointer;
        *fresh5 = (*fresh5).c_offset(width as isize);
        if value < 0x10000 as libc::c_int as libc::c_uint {
            *((*emitter).raw_buffer.last).c_offset(high as isize) =
                (value >> 8 as libc::c_int) as libc::c_uchar;
            *((*emitter).raw_buffer.last).c_offset(low as isize) =
                (value & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            let ref mut fresh6 = (*emitter).raw_buffer.last;
            *fresh6 = (*fresh6).c_offset(2 as libc::c_int as isize);
        } else {
            value = value.wrapping_sub(0x10000 as libc::c_int as libc::c_uint);
            *((*emitter).raw_buffer.last).c_offset(high as isize) =
                (0xd8 as libc::c_int as libc::c_uint).wrapping_add(value >> 18 as libc::c_int)
                    as libc::c_uchar;
            *((*emitter).raw_buffer.last).c_offset(low as isize) =
                (value >> 10 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            *((*emitter).raw_buffer.last).c_offset((high + 2 as libc::c_int) as isize) =
                (0xdc as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as libc::c_uchar;
            *((*emitter).raw_buffer.last).c_offset((low + 2 as libc::c_int) as isize) =
                (value & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            let ref mut fresh7 = (*emitter).raw_buffer.last;
            *fresh7 = (*fresh7).c_offset(4 as libc::c_int as isize);
        }
    }
    if ((*emitter).write_handler).expect("non-null function pointer")(
        (*emitter).write_handler_data,
        (*emitter).raw_buffer.start,
        ((*emitter).raw_buffer.last).c_offset_from((*emitter).raw_buffer.start) as libc::c_long
            as size_t,
    ) != 0
    {
        let ref mut fresh8 = (*emitter).buffer.last;
        *fresh8 = (*emitter).buffer.start;
        let ref mut fresh9 = (*emitter).buffer.pointer;
        *fresh9 = (*emitter).buffer.start;
        let ref mut fresh10 = (*emitter).raw_buffer.last;
        *fresh10 = (*emitter).raw_buffer.start;
        let ref mut fresh11 = (*emitter).raw_buffer.pointer;
        *fresh11 = (*emitter).raw_buffer.start;
        return 1 as libc::c_int;
    } else {
        return yaml_emitter_set_writer_error(
            emitter,
            b"write error\0" as *const u8 as *const libc::c_char,
        );
    };
}
