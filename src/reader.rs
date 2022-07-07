use crate::externs::*;
use crate::libc;
use crate::yaml::*;
use crate::PointerExt;
unsafe extern "C" fn yaml_parser_set_reader_error(
    mut parser: *mut yaml_parser_t,
    mut problem: *const libc::c_char,
    mut offset: size_t,
    mut value: libc::c_int,
) -> libc::c_int {
    (*parser).error = YAML_READER_ERROR;
    let ref mut fresh0 = (*parser).problem;
    *fresh0 = problem;
    (*parser).problem_offset = offset;
    (*parser).problem_value = value;
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_determine_encoding(mut parser: *mut yaml_parser_t) -> libc::c_int {
    while (*parser).eof == 0
        && (((*parser).raw_buffer.last).c_offset_from((*parser).raw_buffer.pointer) as libc::c_long)
            < 3 as libc::c_int as libc::c_long
    {
        if yaml_parser_update_raw_buffer(parser) == 0 {
            return 0 as libc::c_int;
        }
    }
    if ((*parser).raw_buffer.last).c_offset_from((*parser).raw_buffer.pointer) as libc::c_long
        >= 2 as libc::c_int as libc::c_long
        && memcmp(
            (*parser).raw_buffer.pointer as *const libc::c_void,
            b"\xFF\xFE\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        (*parser).encoding = YAML_UTF16LE_ENCODING;
        let ref mut fresh1 = (*parser).raw_buffer.pointer;
        *fresh1 = (*fresh1).c_offset(2 as libc::c_int as isize);
        let ref mut fresh2 = (*parser).offset;
        *fresh2 = (*fresh2 as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else if ((*parser).raw_buffer.last).c_offset_from((*parser).raw_buffer.pointer)
        as libc::c_long
        >= 2 as libc::c_int as libc::c_long
        && memcmp(
            (*parser).raw_buffer.pointer as *const libc::c_void,
            b"\xFE\xFF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        (*parser).encoding = YAML_UTF16BE_ENCODING;
        let ref mut fresh3 = (*parser).raw_buffer.pointer;
        *fresh3 = (*fresh3).c_offset(2 as libc::c_int as isize);
        let ref mut fresh4 = (*parser).offset;
        *fresh4 = (*fresh4 as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else if ((*parser).raw_buffer.last).c_offset_from((*parser).raw_buffer.pointer)
        as libc::c_long
        >= 3 as libc::c_int as libc::c_long
        && memcmp(
            (*parser).raw_buffer.pointer as *const libc::c_void,
            b"\xEF\xBB\xBF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        (*parser).encoding = YAML_UTF8_ENCODING;
        let ref mut fresh5 = (*parser).raw_buffer.pointer;
        *fresh5 = (*fresh5).c_offset(3 as libc::c_int as isize);
        let ref mut fresh6 = (*parser).offset;
        *fresh6 = (*fresh6 as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else {
        (*parser).encoding = YAML_UTF8_ENCODING;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_update_raw_buffer(mut parser: *mut yaml_parser_t) -> libc::c_int {
    let mut size_read: size_t = 0 as libc::c_int as size_t;
    if (*parser).raw_buffer.start == (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.last == (*parser).raw_buffer.end
    {
        return 1 as libc::c_int;
    }
    if (*parser).eof != 0 {
        return 1 as libc::c_int;
    }
    if (*parser).raw_buffer.start < (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.pointer < (*parser).raw_buffer.last
    {
        memmove(
            (*parser).raw_buffer.start as *mut libc::c_void,
            (*parser).raw_buffer.pointer as *const libc::c_void,
            ((*parser).raw_buffer.last).c_offset_from((*parser).raw_buffer.pointer) as libc::c_long
                as libc::c_ulong,
        );
    }
    let ref mut fresh7 = (*parser).raw_buffer.last;
    *fresh7 = (*fresh7).c_offset(
        -(((*parser).raw_buffer.pointer).c_offset_from((*parser).raw_buffer.start) as libc::c_long
            as isize),
    );
    let ref mut fresh8 = (*parser).raw_buffer.pointer;
    *fresh8 = (*parser).raw_buffer.start;
    if ((*parser).read_handler).expect("non-null function pointer")(
        (*parser).read_handler_data,
        (*parser).raw_buffer.last,
        ((*parser).raw_buffer.end).c_offset_from((*parser).raw_buffer.last) as libc::c_long
            as size_t,
        &mut size_read,
    ) == 0
    {
        return yaml_parser_set_reader_error(
            parser,
            b"input error\0" as *const u8 as *const libc::c_char,
            (*parser).offset,
            -(1 as libc::c_int),
        );
    }
    let ref mut fresh9 = (*parser).raw_buffer.last;
    *fresh9 = (*fresh9).c_offset(size_read as isize);
    if size_read == 0 {
        (*parser).eof = 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_update_buffer(
    mut parser: *mut yaml_parser_t,
    mut length: size_t,
) -> libc::c_int {
    let mut first: libc::c_int = 1 as libc::c_int;
    __assert!(((*parser).read_handler).is_some());
    if (*parser).eof != 0 && (*parser).raw_buffer.pointer == (*parser).raw_buffer.last {
        return 1 as libc::c_int;
    }
    if (*parser).unread >= length {
        return 1 as libc::c_int;
    }
    if (*parser).encoding as u64 == 0 {
        if yaml_parser_determine_encoding(parser) == 0 {
            return 0 as libc::c_int;
        }
    }
    if (*parser).buffer.start < (*parser).buffer.pointer
        && (*parser).buffer.pointer < (*parser).buffer.last
    {
        let mut size: size_t = ((*parser).buffer.last).c_offset_from((*parser).buffer.pointer)
            as libc::c_long as size_t;
        memmove(
            (*parser).buffer.start as *mut libc::c_void,
            (*parser).buffer.pointer as *const libc::c_void,
            size,
        );
        let ref mut fresh10 = (*parser).buffer.pointer;
        *fresh10 = (*parser).buffer.start;
        let ref mut fresh11 = (*parser).buffer.last;
        *fresh11 = ((*parser).buffer.start).c_offset(size as isize);
    } else if (*parser).buffer.pointer == (*parser).buffer.last {
        let ref mut fresh12 = (*parser).buffer.pointer;
        *fresh12 = (*parser).buffer.start;
        let ref mut fresh13 = (*parser).buffer.last;
        *fresh13 = (*parser).buffer.start;
    }
    while (*parser).unread < length {
        if first == 0 || (*parser).raw_buffer.pointer == (*parser).raw_buffer.last {
            if yaml_parser_update_raw_buffer(parser) == 0 {
                return 0 as libc::c_int;
            }
        }
        first = 0 as libc::c_int;
        while (*parser).raw_buffer.pointer != (*parser).raw_buffer.last {
            let mut value: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut value2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut incomplete: libc::c_int = 0 as libc::c_int;
            let mut octet: libc::c_uchar = 0;
            let mut width: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut low: libc::c_int = 0;
            let mut high: libc::c_int = 0;
            let mut k: size_t = 0;
            let mut raw_unread: size_t = ((*parser).raw_buffer.last)
                .c_offset_from((*parser).raw_buffer.pointer)
                as libc::c_long as size_t;
            match (*parser).encoding as libc::c_uint {
                1 => {
                    octet = *((*parser).raw_buffer.pointer).c_offset(0 as libc::c_int as isize);
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
                    if width == 0 {
                        return yaml_parser_set_reader_error(
                            parser,
                            b"invalid leading UTF-8 octet\0" as *const u8 as *const libc::c_char,
                            (*parser).offset,
                            octet as libc::c_int,
                        );
                    }
                    if width as libc::c_ulong > raw_unread {
                        if (*parser).eof != 0 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-8 octet sequence\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                -(1 as libc::c_int),
                            );
                        }
                        incomplete = 1 as libc::c_int;
                    } else {
                        value = (if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
                            octet as libc::c_int & 0x7f as libc::c_int
                        } else if octet as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int
                        {
                            octet as libc::c_int & 0x1f as libc::c_int
                        } else if octet as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int
                        {
                            octet as libc::c_int & 0xf as libc::c_int
                        } else if octet as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int
                        {
                            octet as libc::c_int & 0x7 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as libc::c_uint;
                        k = 1 as libc::c_int as size_t;
                        while k < width as libc::c_ulong {
                            octet = *((*parser).raw_buffer.pointer).c_offset(k as isize);
                            if octet as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int {
                                return yaml_parser_set_reader_error(
                                    parser,
                                    b"invalid trailing UTF-8 octet\0" as *const u8
                                        as *const libc::c_char,
                                    ((*parser).offset).wrapping_add(k),
                                    octet as libc::c_int,
                                );
                            }
                            value = (value << 6 as libc::c_int).wrapping_add(
                                (octet as libc::c_int & 0x3f as libc::c_int) as libc::c_uint,
                            );
                            k = k.wrapping_add(1);
                        }
                        if !(width == 1 as libc::c_int as libc::c_uint
                            || width == 2 as libc::c_int as libc::c_uint
                                && value >= 0x80 as libc::c_int as libc::c_uint
                            || width == 3 as libc::c_int as libc::c_uint
                                && value >= 0x800 as libc::c_int as libc::c_uint
                            || width == 4 as libc::c_int as libc::c_uint
                                && value >= 0x10000 as libc::c_int as libc::c_uint)
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid length of a UTF-8 sequence\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                -(1 as libc::c_int),
                            );
                        }
                        if value >= 0xd800 as libc::c_int as libc::c_uint
                            && value <= 0xdfff as libc::c_int as libc::c_uint
                            || value > 0x10ffff as libc::c_int as libc::c_uint
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid Unicode character\0" as *const u8 as *const libc::c_char,
                                (*parser).offset,
                                value as libc::c_int,
                            );
                        }
                    }
                }
                2 | 3 => {
                    low = if (*parser).encoding as libc::c_uint
                        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
                    {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    high = if (*parser).encoding as libc::c_uint
                        == YAML_UTF16LE_ENCODING as libc::c_int as libc::c_uint
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    if raw_unread < 2 as libc::c_int as libc::c_ulong {
                        if (*parser).eof != 0 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-16 character\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                -(1 as libc::c_int),
                            );
                        }
                        incomplete = 1 as libc::c_int;
                    } else {
                        value = (*((*parser).raw_buffer.pointer).c_offset(low as isize)
                            as libc::c_int
                            + ((*((*parser).raw_buffer.pointer).c_offset(high as isize)
                                as libc::c_int)
                                << 8 as libc::c_int))
                            as libc::c_uint;
                        if value & 0xfc00 as libc::c_int as libc::c_uint
                            == 0xdc00 as libc::c_int as libc::c_uint
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"unexpected low surrogate area\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                value as libc::c_int,
                            );
                        }
                        if value & 0xfc00 as libc::c_int as libc::c_uint
                            == 0xd800 as libc::c_int as libc::c_uint
                        {
                            width = 4 as libc::c_int as libc::c_uint;
                            if raw_unread < 4 as libc::c_int as libc::c_ulong {
                                if (*parser).eof != 0 {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"incomplete UTF-16 surrogate pair\0" as *const u8
                                            as *const libc::c_char,
                                        (*parser).offset,
                                        -(1 as libc::c_int),
                                    );
                                }
                                incomplete = 1 as libc::c_int;
                            } else {
                                value2 = (*((*parser).raw_buffer.pointer)
                                    .c_offset((low + 2 as libc::c_int) as isize)
                                    as libc::c_int
                                    + ((*((*parser).raw_buffer.pointer)
                                        .c_offset((high + 2 as libc::c_int) as isize)
                                        as libc::c_int)
                                        << 8 as libc::c_int))
                                    as libc::c_uint;
                                if value2 & 0xfc00 as libc::c_int as libc::c_uint
                                    != 0xdc00 as libc::c_int as libc::c_uint
                                {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"expected low surrogate area\0" as *const u8
                                            as *const libc::c_char,
                                        ((*parser).offset)
                                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                                        value2 as libc::c_int,
                                    );
                                }
                                value = (0x10000 as libc::c_int as libc::c_uint)
                                    .wrapping_add(
                                        (value & 0x3ff as libc::c_int as libc::c_uint)
                                            << 10 as libc::c_int,
                                    )
                                    .wrapping_add(value2 & 0x3ff as libc::c_int as libc::c_uint);
                            }
                        } else {
                            width = 2 as libc::c_int as libc::c_uint;
                        }
                    }
                }
                _ => {}
            }
            if incomplete != 0 {
                break;
            }
            if !(value == 0x9 as libc::c_int as libc::c_uint
                || value == 0xa as libc::c_int as libc::c_uint
                || value == 0xd as libc::c_int as libc::c_uint
                || value >= 0x20 as libc::c_int as libc::c_uint
                    && value <= 0x7e as libc::c_int as libc::c_uint
                || value == 0x85 as libc::c_int as libc::c_uint
                || value >= 0xa0 as libc::c_int as libc::c_uint
                    && value <= 0xd7ff as libc::c_int as libc::c_uint
                || value >= 0xe000 as libc::c_int as libc::c_uint
                    && value <= 0xfffd as libc::c_int as libc::c_uint
                || value >= 0x10000 as libc::c_int as libc::c_uint
                    && value <= 0x10ffff as libc::c_int as libc::c_uint)
            {
                return yaml_parser_set_reader_error(
                    parser,
                    b"control characters are not allowed\0" as *const u8 as *const libc::c_char,
                    (*parser).offset,
                    value as libc::c_int,
                );
            }
            let ref mut fresh14 = (*parser).raw_buffer.pointer;
            *fresh14 = (*fresh14).c_offset(width as isize);
            let ref mut fresh15 = (*parser).offset;
            *fresh15 = (*fresh15 as libc::c_ulong).wrapping_add(width as libc::c_ulong) as size_t
                as size_t;
            if value <= 0x7f as libc::c_int as libc::c_uint {
                let ref mut fresh16 = (*parser).buffer.last;
                let fresh17 = *fresh16;
                *fresh16 = (*fresh16).c_offset(1);
                *fresh17 = value as yaml_char_t;
            } else if value <= 0x7ff as libc::c_int as libc::c_uint {
                let ref mut fresh18 = (*parser).buffer.last;
                let fresh19 = *fresh18;
                *fresh18 = (*fresh18).c_offset(1);
                *fresh19 = (0xc0 as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 6 as libc::c_int)
                    as yaml_char_t;
                let ref mut fresh20 = (*parser).buffer.last;
                let fresh21 = *fresh20;
                *fresh20 = (*fresh20).c_offset(1);
                *fresh21 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(value & 0x3f as libc::c_int as libc::c_uint)
                    as yaml_char_t;
            } else if value <= 0xffff as libc::c_int as libc::c_uint {
                let ref mut fresh22 = (*parser).buffer.last;
                let fresh23 = *fresh22;
                *fresh22 = (*fresh22).c_offset(1);
                *fresh23 = (0xe0 as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 12 as libc::c_int)
                    as yaml_char_t;
                let ref mut fresh24 = (*parser).buffer.last;
                let fresh25 = *fresh24;
                *fresh24 = (*fresh24).c_offset(1);
                *fresh25 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                    as yaml_char_t;
                let ref mut fresh26 = (*parser).buffer.last;
                let fresh27 = *fresh26;
                *fresh26 = (*fresh26).c_offset(1);
                *fresh27 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(value & 0x3f as libc::c_int as libc::c_uint)
                    as yaml_char_t;
            } else {
                let ref mut fresh28 = (*parser).buffer.last;
                let fresh29 = *fresh28;
                *fresh28 = (*fresh28).c_offset(1);
                *fresh29 = (0xf0 as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 18 as libc::c_int)
                    as yaml_char_t;
                let ref mut fresh30 = (*parser).buffer.last;
                let fresh31 = *fresh30;
                *fresh30 = (*fresh30).c_offset(1);
                *fresh31 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                    as yaml_char_t;
                let ref mut fresh32 = (*parser).buffer.last;
                let fresh33 = *fresh32;
                *fresh32 = (*fresh32).c_offset(1);
                *fresh33 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(value >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                    as yaml_char_t;
                let ref mut fresh34 = (*parser).buffer.last;
                let fresh35 = *fresh34;
                *fresh34 = (*fresh34).c_offset(1);
                *fresh35 = (0x80 as libc::c_int as libc::c_uint)
                    .wrapping_add(value & 0x3f as libc::c_int as libc::c_uint)
                    as yaml_char_t;
            }
            let ref mut fresh36 = (*parser).unread;
            *fresh36 = (*fresh36).wrapping_add(1);
        }
        if (*parser).eof != 0 {
            let ref mut fresh37 = (*parser).buffer.last;
            let fresh38 = *fresh37;
            *fresh37 = (*fresh37).c_offset(1);
            *fresh38 = '\0' as i32 as yaml_char_t;
            let ref mut fresh39 = (*parser).unread;
            *fresh39 = (*fresh39).wrapping_add(1);
            return 1 as libc::c_int;
        }
    }
    if (*parser).offset
        >= (!(0 as libc::c_int as size_t)).wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        return yaml_parser_set_reader_error(
            parser,
            b"input is too long\0" as *const u8 as *const libc::c_char,
            (*parser).offset,
            -(1 as libc::c_int),
        );
    }
    return 1 as libc::c_int;
}
