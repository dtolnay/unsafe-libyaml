macro_rules! BUFFER_INIT {
    ($context:expr, $buffer:expr, $size:expr) => {{
        let start = addr_of_mut!($buffer.start);
        *start = yaml_malloc($size as size_t) as *mut yaml_char_t;
        if !(*start).is_null() {
            let pointer = addr_of_mut!($buffer.pointer);
            *pointer = $buffer.start;
            let last = addr_of_mut!($buffer.last);
            *last = *pointer;
            let end = addr_of_mut!($buffer.end);
            *end = $buffer.start.wrapping_add($size as usize);
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    }};
}

macro_rules! BUFFER_DEL {
    ($buffer:expr) => {{
        yaml_free($buffer.start as *mut libc::c_void);
        let end = addr_of_mut!($buffer.end);
        *end = ptr::null_mut::<yaml_char_t>();
        let pointer = addr_of_mut!($buffer.pointer);
        *pointer = *end;
        let start = addr_of_mut!($buffer.start);
        *start = *pointer;
    }};
}

macro_rules! NULL_STRING {
    () => {
        yaml_string_t {
            start: ptr::null_mut::<yaml_char_t>(),
            end: ptr::null_mut::<yaml_char_t>(),
            pointer: ptr::null_mut::<yaml_char_t>(),
        }
    };
}

macro_rules! STRING_ASSIGN {
    ($string:expr, $length:expr) => {
        yaml_string_t {
            start: $string,
            end: $string.wrapping_offset($length as isize),
            pointer: $string,
        }
    };
}

macro_rules! STRING_INIT {
    ($context:expr, $string:expr) => {{
        $string.start = yaml_malloc(16) as *mut yaml_char_t;
        if !$string.start.is_null() {
            $string.pointer = $string.start;
            $string.end = $string.start.wrapping_add(16);
            memset($string.start as *mut libc::c_void, 0, 16);
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    }};
}

macro_rules! STRING_DEL {
    ($string:expr) => {{
        yaml_free($string.start as *mut libc::c_void);
        $string.end = ptr::null_mut::<yaml_char_t>();
        $string.pointer = $string.end;
        $string.start = $string.pointer;
    }};
}

macro_rules! STRING_EXTEND {
    ($context:expr, $string:expr) => {
        if $string.pointer.wrapping_add(5) < $string.end
            || yaml_string_extend(
                addr_of_mut!($string.start),
                addr_of_mut!($string.pointer),
                addr_of_mut!($string.end),
            ) != 0
        {
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    };
}

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
