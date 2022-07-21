macro_rules! MOVE {
    ($string:expr) => {
        $string.pointer = $string.pointer.wrapping_offset(
            (if *$string.pointer.wrapping_offset(0_isize) as libc::c_int & 0x80_i32 == 0_i32 {
                1_i32
            } else if *$string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xe0_i32
                == 0xc0_i32
            {
                2_i32
            } else if *$string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf0_i32
                == 0xe0_i32
            {
                3_i32
            } else if *$string.pointer.wrapping_offset(0_isize) as libc::c_int & 0xf8_i32
                == 0xf0_i32
            {
                4_i32
            } else {
                0_i32
            }) as isize,
        )
    };
}
