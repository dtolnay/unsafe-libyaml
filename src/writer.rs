use crate::yaml::size_t;
use crate::{
    libc, yaml_emitter_t, PointerExt, YAML_UTF16LE_ENCODING, YAML_UTF8_ENCODING, YAML_WRITER_ERROR,
};
use core::ptr::addr_of_mut;

unsafe fn yaml_emitter_set_writer_error(
    mut emitter: *mut yaml_emitter_t,
    problem: *const libc::c_char,
) -> libc::c_int {
    (*emitter).error = YAML_WRITER_ERROR;
    let fresh0 = addr_of_mut!((*emitter).problem);
    *fresh0 = problem;
    0_i32
}

/// Flush the accumulated characters to the output.
///
/// Returns 1 if the function succeeded, 0 on error.
#[must_use]
pub unsafe fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> libc::c_int {
    __assert!(!emitter.is_null());
    __assert!(((*emitter).write_handler).is_some());
    __assert!((*emitter).encoding as u64 != 0);
    let fresh1 = addr_of_mut!((*emitter).buffer.last);
    *fresh1 = (*emitter).buffer.pointer;
    let fresh2 = addr_of_mut!((*emitter).buffer.pointer);
    *fresh2 = (*emitter).buffer.start;
    if (*emitter).buffer.start == (*emitter).buffer.last {
        return 1_i32;
    }
    if (*emitter).encoding as libc::c_uint == YAML_UTF8_ENCODING as libc::c_int as libc::c_uint {
        if ((*emitter).write_handler).expect("non-null function pointer")(
            (*emitter).write_handler_data,
            (*emitter).buffer.start,
            ((*emitter).buffer.last).c_offset_from((*emitter).buffer.start) as libc::c_long
                as size_t,
        ) != 0
        {
            let fresh3 = addr_of_mut!((*emitter).buffer.last);
            *fresh3 = (*emitter).buffer.start;
            let fresh4 = addr_of_mut!((*emitter).buffer.pointer);
            *fresh4 = (*emitter).buffer.start;
            return 1_i32;
        } else {
            return yaml_emitter_set_writer_error(
                emitter,
                b"write error\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let low: libc::c_int = if (*emitter).encoding as libc::c_uint
        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
    {
        0_i32
    } else {
        1_i32
    };
    let high: libc::c_int = if (*emitter).encoding as libc::c_uint
        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
    {
        1_i32
    } else {
        0_i32
    };
    while (*emitter).buffer.pointer != (*emitter).buffer.last {
        let mut octet: libc::c_uchar;
        let mut value: libc::c_uint;
        let mut k: size_t;
        octet = *((*emitter).buffer.pointer);
        let width: libc::c_uint = (if octet as libc::c_int & 0x80_i32 == 0_i32 {
            1_i32
        } else if octet as libc::c_int & 0xe0_i32 == 0xc0_i32 {
            2_i32
        } else if octet as libc::c_int & 0xf0_i32 == 0xe0_i32 {
            3_i32
        } else if octet as libc::c_int & 0xf8_i32 == 0xf0_i32 {
            4_i32
        } else {
            0_i32
        }) as libc::c_uint;
        value = (if octet as libc::c_int & 0x80_i32 == 0_i32 {
            octet as libc::c_int & 0x7f_i32
        } else if octet as libc::c_int & 0xe0_i32 == 0xc0_i32 {
            octet as libc::c_int & 0x1f_i32
        } else if octet as libc::c_int & 0xf0_i32 == 0xe0_i32 {
            octet as libc::c_int & 0xf_i32
        } else if octet as libc::c_int & 0xf8_i32 == 0xf0_i32 {
            octet as libc::c_int & 0x7_i32
        } else {
            0_i32
        }) as libc::c_uint;
        k = 1_u64;
        while k < width as libc::c_ulong {
            octet = *((*emitter).buffer.pointer).wrapping_offset(k as isize);
            value =
                (value << 6_i32).wrapping_add((octet as libc::c_int & 0x3f_i32) as libc::c_uint);
            k = k.wrapping_add(1);
        }
        let fresh5 = addr_of_mut!((*emitter).buffer.pointer);
        *fresh5 = (*fresh5).wrapping_offset(width as isize);
        if value < 0x10000_i32 as libc::c_uint {
            *((*emitter).raw_buffer.last).wrapping_offset(high as isize) =
                (value >> 8_i32) as libc::c_uchar;
            *((*emitter).raw_buffer.last).wrapping_offset(low as isize) =
                (value & 0xff_i32 as libc::c_uint) as libc::c_uchar;
            let fresh6 = addr_of_mut!((*emitter).raw_buffer.last);
            *fresh6 = (*fresh6).wrapping_offset(2_isize);
        } else {
            value = value.wrapping_sub(0x10000_i32 as libc::c_uint);
            *((*emitter).raw_buffer.last).wrapping_offset(high as isize) =
                (0xd8_i32 as libc::c_uint).wrapping_add(value >> 18_i32) as libc::c_uchar;
            *((*emitter).raw_buffer.last).wrapping_offset(low as isize) =
                (value >> 10_i32 & 0xff_i32 as libc::c_uint) as libc::c_uchar;
            *((*emitter).raw_buffer.last).wrapping_offset((high + 2_i32) as isize) =
                (0xdc_i32 as libc::c_uint).wrapping_add(value >> 8_i32 & 0xff_i32 as libc::c_uint)
                    as libc::c_uchar;
            *((*emitter).raw_buffer.last).wrapping_offset((low + 2_i32) as isize) =
                (value & 0xff_i32 as libc::c_uint) as libc::c_uchar;
            let fresh7 = addr_of_mut!((*emitter).raw_buffer.last);
            *fresh7 = (*fresh7).wrapping_offset(4_isize);
        }
    }
    if ((*emitter).write_handler).expect("non-null function pointer")(
        (*emitter).write_handler_data,
        (*emitter).raw_buffer.start,
        ((*emitter).raw_buffer.last).c_offset_from((*emitter).raw_buffer.start) as libc::c_long
            as size_t,
    ) != 0
    {
        let fresh8 = addr_of_mut!((*emitter).buffer.last);
        *fresh8 = (*emitter).buffer.start;
        let fresh9 = addr_of_mut!((*emitter).buffer.pointer);
        *fresh9 = (*emitter).buffer.start;
        let fresh10 = addr_of_mut!((*emitter).raw_buffer.last);
        *fresh10 = (*emitter).raw_buffer.start;
        let fresh11 = addr_of_mut!((*emitter).raw_buffer.pointer);
        *fresh11 = (*emitter).raw_buffer.start;
        1_i32
    } else {
        yaml_emitter_set_writer_error(
            emitter,
            b"write error\0" as *const u8 as *const libc::c_char,
        )
    }
}
