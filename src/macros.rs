macro_rules! IS_BLANK_AT {
    ($string:expr, $offset:expr) => {
        *$string.pointer.wrapping_offset($offset as isize) as libc::c_int
            == ' ' as i32 as yaml_char_t as libc::c_int
            || *$string.pointer.wrapping_offset($offset as isize) as libc::c_int
                == '\t' as i32 as yaml_char_t as libc::c_int
    };
}

macro_rules! IS_BLANK {
    ($string:expr) => {
        IS_BLANK_AT!($string, 0)
    };
}

macro_rules! IS_BLANKZ_AT {
    ($string:expr, $offset:expr) => {
        IS_BLANK_AT!($string, $offset)
            || (*$string.pointer.wrapping_offset($offset as isize) as libc::c_int
                == '\r' as i32 as yaml_char_t as libc::c_int
                || *$string.pointer.wrapping_offset($offset as isize) as libc::c_int
                    == '\n' as i32 as yaml_char_t as libc::c_int
                || *$string.pointer.wrapping_offset($offset as isize) as libc::c_int
                    == -62i32 as yaml_char_t as libc::c_int
                    && *$string.pointer.wrapping_offset($offset as isize + 1) as libc::c_int
                        == -123i32 as yaml_char_t as libc::c_int
                || *$string.pointer.wrapping_offset($offset as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *$string.pointer.wrapping_offset($offset as isize + 1) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *$string.pointer.wrapping_offset($offset as isize + 2) as libc::c_int
                        == -88i32 as yaml_char_t as libc::c_int
                || *$string.pointer.wrapping_offset($offset as isize) as libc::c_int
                    == -30i32 as yaml_char_t as libc::c_int
                    && *$string.pointer.wrapping_offset($offset as isize + 1) as libc::c_int
                        == -128i32 as yaml_char_t as libc::c_int
                    && *$string.pointer.wrapping_offset($offset as isize + 2) as libc::c_int
                        == -87i32 as yaml_char_t as libc::c_int
                || *$string.pointer.wrapping_offset($offset as isize) as libc::c_int
                    == '\0' as i32 as yaml_char_t as libc::c_int)
    };
}

macro_rules! IS_BLANKZ {
    ($string:expr) => {
        IS_BLANKZ_AT!($string, 0)
    };
}

macro_rules! MOVE {
    ($string:expr) => {
        $string.pointer = $string.pointer.wrapping_offset(
            (if *$string.pointer as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if *$string.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
                2_i32
            } else if *$string.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
                3_i32
            } else if *$string.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
                4_i32
            } else {
                0_i32
            }) as isize,
        )
    };
}

macro_rules! COPY {
    ($string_a:expr, $string_b:expr) => {
        if *$string_b.pointer as libc::c_int & 0x80_i32 == 0_i32 {
            let fresh77 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh78 = addr_of_mut!($string_a.pointer);
            let fresh79 = *fresh78;
            *fresh78 = (*fresh78).wrapping_offset(1);
            *fresh79 = *fresh77;
        } else if *$string_b.pointer as libc::c_int & 0xe0_i32 == 0xc0_i32 {
            let fresh80 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh81 = addr_of_mut!($string_a.pointer);
            let fresh82 = *fresh81;
            *fresh81 = (*fresh81).wrapping_offset(1);
            *fresh82 = *fresh80;
            let fresh83 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh84 = addr_of_mut!($string_a.pointer);
            let fresh85 = *fresh84;
            *fresh84 = (*fresh84).wrapping_offset(1);
            *fresh85 = *fresh83;
        } else if *$string_b.pointer as libc::c_int & 0xf0_i32 == 0xe0_i32 {
            let fresh86 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh87 = addr_of_mut!($string_a.pointer);
            let fresh88 = *fresh87;
            *fresh87 = (*fresh87).wrapping_offset(1);
            *fresh88 = *fresh86;
            let fresh89 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh90 = addr_of_mut!($string_a.pointer);
            let fresh91 = *fresh90;
            *fresh90 = (*fresh90).wrapping_offset(1);
            *fresh91 = *fresh89;
            let fresh92 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh93 = addr_of_mut!($string_a.pointer);
            let fresh94 = *fresh93;
            *fresh93 = (*fresh93).wrapping_offset(1);
            *fresh94 = *fresh92;
        } else if *$string_b.pointer as libc::c_int & 0xf8_i32 == 0xf0_i32 {
            let fresh95 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh96 = addr_of_mut!($string_a.pointer);
            let fresh97 = *fresh96;
            *fresh96 = (*fresh96).wrapping_offset(1);
            *fresh97 = *fresh95;
            let fresh98 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh99 = addr_of_mut!($string_a.pointer);
            let fresh100 = *fresh99;
            *fresh99 = (*fresh99).wrapping_offset(1);
            *fresh100 = *fresh98;
            let fresh101 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh102 = addr_of_mut!($string_a.pointer);
            let fresh103 = *fresh102;
            *fresh102 = (*fresh102).wrapping_offset(1);
            *fresh103 = *fresh101;
            let fresh104 = $string_b.pointer;
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            let fresh105 = addr_of_mut!($string_a.pointer);
            let fresh106 = *fresh105;
            *fresh105 = (*fresh105).wrapping_offset(1);
            *fresh106 = *fresh104;
        }
    };
}
