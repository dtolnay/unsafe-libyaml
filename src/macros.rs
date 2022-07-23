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

macro_rules! CLEAR {
    ($string:expr) => {{
        $string.pointer = $string.start;
        memset(
            $string.start as *mut libc::c_void,
            0_i32,
            $string.end.c_offset_from($string.start) as libc::c_ulong,
        );
    }};
}

macro_rules! JOIN {
    ($context:expr, $string_a:expr, $string_b:expr) => {
        if yaml_string_join(
            addr_of_mut!($string_a.start),
            addr_of_mut!($string_a.pointer),
            addr_of_mut!($string_a.end),
            addr_of_mut!($string_b.start),
            addr_of_mut!($string_b.pointer),
            addr_of_mut!($string_b.end),
        ) != 0
        {
            $string_b.pointer = $string_b.start;
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    };
}

macro_rules! CHECK_AT {
    ($string:expr, $octet:expr, $offset:expr) => {
        *$string.pointer.offset($offset as isize) == $octet as yaml_char_t
    };
}

macro_rules! CHECK {
    ($string:expr, $octet:expr) => {
        *$string.pointer == $octet as yaml_char_t
    };
}

macro_rules! IS_ALPHA {
    ($string:expr) => {
        *$string.pointer >= b'0' && *$string.pointer <= b'9'
            || *$string.pointer >= b'A' && *$string.pointer <= b'Z'
            || *$string.pointer >= b'a' && *$string.pointer <= b'z'
            || *$string.pointer == b'_'
            || *$string.pointer == b'-'
    };
}

macro_rules! IS_DIGIT {
    ($string:expr) => {
        *$string.pointer >= b'0' && *$string.pointer <= b'9'
    };
}

macro_rules! AS_DIGIT {
    ($string:expr) => {
        (*$string.pointer - b'0') as libc::c_int
    };
}

macro_rules! IS_HEX_AT {
    ($string:expr, $offset:expr) => {
        *$string.pointer.wrapping_offset($offset) >= b'0'
            && *$string.pointer.wrapping_offset($offset) <= b'9'
            || *$string.pointer.wrapping_offset($offset) >= b'A'
                && *$string.pointer.wrapping_offset($offset) <= b'F'
            || *$string.pointer.wrapping_offset($offset) >= b'a'
                && *$string.pointer.wrapping_offset($offset) <= b'f'
    };
}

macro_rules! AS_HEX_AT {
    ($string:expr, $offset:expr) => {
        if *$string.pointer.wrapping_offset($offset) >= b'A'
            && *$string.pointer.wrapping_offset($offset) <= b'F'
        {
            *$string.pointer.wrapping_offset($offset) - b'A' + 10
        } else if *$string.pointer.wrapping_offset($offset) >= b'a'
            && *$string.pointer.wrapping_offset($offset) <= b'f'
        {
            *$string.pointer.wrapping_offset($offset) - b'a' + 10
        } else {
            *$string.pointer.wrapping_offset($offset) - b'0'
        } as libc::c_int
    };
}

macro_rules! IS_ASCII {
    ($string:expr) => {
        *$string.pointer <= b'\x7F'
    };
}

macro_rules! IS_PRINTABLE_AT {
    () => {}; // TODO
}

macro_rules! IS_PRINTABLE {
    ($string:expr) => {
        (*$string.pointer as libc::c_int == 0xa
            || *$string.pointer as libc::c_int >= 0x20 && *$string.pointer as libc::c_int <= 0x7e
            || *$string.pointer as libc::c_int == 0xc2
                && *$string.pointer.wrapping_offset(1) as libc::c_int >= 0xa0
            || *$string.pointer as libc::c_int > 0xc2 && (*$string.pointer as libc::c_int) < 0xed
            || *$string.pointer as libc::c_int == 0xed
                && (*$string.pointer.wrapping_offset(1) as libc::c_int) < 0xa0
            || *$string.pointer as libc::c_int == 0xee
            || *$string.pointer as libc::c_int == 0xef
                && !(*$string.pointer.wrapping_offset(1) as libc::c_int == 0xbb
                    && *$string.pointer.wrapping_offset(2) as libc::c_int == 0xbf)
                && !(*$string.pointer.wrapping_offset(1) as libc::c_int == 0xbf
                    && (*$string.pointer.wrapping_offset(2) as libc::c_int == 0xbe
                        || *$string.pointer.wrapping_offset(2) as libc::c_int == 0xbf)))
    };
}

macro_rules! IS_Z_AT {
    ($string:expr, $offset:expr) => {
        CHECK_AT!($string, '\0', $offset)
    };
}

macro_rules! IS_Z {
    ($string:expr) => {
        IS_Z_AT!($string, 0)
    };
}

macro_rules! IS_BOM {
    ($string:expr) => {
        CHECK_AT!($string, b'\xEF', 0)
            && CHECK_AT!($string, b'\xBB', 1)
            && CHECK_AT!($string, b'\xBF', 2)
    };
}

macro_rules! IS_SPACE_AT {
    ($string:expr, $offset:expr) => {
        CHECK_AT!($string, ' ', $offset)
    };
}

macro_rules! IS_SPACE {
    ($string:expr) => {
        IS_SPACE_AT!($string, 0)
    };
}

macro_rules! IS_TAB_AT {
    ($string:expr, $offset:expr) => {
        CHECK_AT!($string, '\t', $offset)
    };
}

macro_rules! IS_TAB {
    ($string:expr) => {
        IS_TAB_AT!($string, 0)
    };
}

macro_rules! IS_BLANK_AT {
    ($string:expr, $offset:expr) => {
        IS_SPACE_AT!($string, $offset) || IS_TAB_AT!($string, $offset)
    };
}

macro_rules! IS_BLANK {
    ($string:expr) => {
        IS_BLANK_AT!($string, 0)
    };
}

macro_rules! IS_BREAK_AT {
    ($string:expr, $offset:expr) => {
        CHECK_AT!($string, '\r', $offset)
            || CHECK_AT!($string, '\n', $offset)
            || CHECK_AT!($string, b'\xC2', $offset) && CHECK_AT!($string, b'\x85', $offset + 1)
            || CHECK_AT!($string, b'\xE2', $offset)
                && CHECK_AT!($string, b'\x80', $offset + 1)
                && CHECK_AT!($string, b'\xA8', $offset + 2)
            || CHECK_AT!($string, b'\xE2', $offset)
                && CHECK_AT!($string, b'\x80', $offset + 1)
                && CHECK_AT!($string, b'\xA9', $offset + 2)
    };
}

macro_rules! IS_BREAK {
    ($string:expr) => {
        IS_BREAK_AT!($string, 0)
    };
}

macro_rules! IS_CRLF {
    ($string:expr) => {
        CHECK_AT!($string, '\r', 0) && CHECK_AT!($string, '\n', 1)
    };
}

macro_rules! IS_BREAKZ_AT {
    ($string:expr, $offset:expr) => {
        IS_BREAK_AT!($string, $offset) || IS_Z_AT!($string, $offset)
    };
}

macro_rules! IS_BREAKZ {
    ($string:expr) => {
        IS_BREAKZ_AT!($string, 0)
    };
}

macro_rules! IS_BLANKZ_AT {
    ($string:expr, $offset:expr) => {
        IS_BLANK_AT!($string, $offset) || IS_BREAKZ_AT!($string, $offset)
    };
}

macro_rules! IS_BLANKZ {
    ($string:expr) => {
        IS_BLANKZ_AT!($string, 0)
    };
}

macro_rules! WIDTH_AT {
    ($string:expr, $offset:expr) => {
        if *$string.pointer.wrapping_offset($offset as isize) as libc::c_int & 0x80_i32 == 0_i32 {
            1_i32
        } else if *$string.pointer.wrapping_offset($offset as isize) as libc::c_int & 0xe0_i32
            == 0xc0_i32
        {
            2_i32
        } else if *$string.pointer.wrapping_offset($offset as isize) as libc::c_int & 0xf0_i32
            == 0xe0_i32
        {
            3_i32
        } else if *$string.pointer.wrapping_offset($offset as isize) as libc::c_int & 0xf8_i32
            == 0xf0_i32
        {
            4_i32
        } else {
            0_i32
        }
    };
}

macro_rules! WIDTH {
    ($string:expr) => {
        WIDTH_AT!($string, 0)
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

macro_rules! STACK_INIT {
    ($context:expr, $stack:expr, $type:ty) => {{
        $stack.start = yaml_malloc(16 * size_of::<$type>() as libc::c_ulong) as *mut $type;
        if !$stack.start.is_null() {
            $stack.top = $stack.start;
            $stack.end = $stack.start.offset(16_isize);
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    }};
}

macro_rules! STACK_DEL {
    ($stack:expr) => {
        yaml_free($stack.start as *mut libc::c_void);
        $stack.end = ptr::null_mut();
        $stack.top = ptr::null_mut();
        $stack.start = ptr::null_mut();
    };
}

macro_rules! STACK_EMPTY {
    ($stack:expr) => {
        $stack.start == $stack.top
    };
}

macro_rules! STACK_LIMIT {
    ($context:expr, $stack:expr) => {
        if $stack.top.c_offset_from($stack.start) < libc::c_int::MAX as isize - 1 {
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    };
}

macro_rules! PUSH {
    (do $context:expr, $stack:expr, $push:expr) => {
        if $stack.top != $stack.end
            || yaml_stack_extend(
                addr_of_mut!($stack.start) as *mut *mut libc::c_void,
                addr_of_mut!($stack.top) as *mut *mut libc::c_void,
                addr_of_mut!($stack.end) as *mut *mut libc::c_void,
            ) != 0
        {
            $push;
            $stack.top = $stack.top.wrapping_offset(1);
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    };
    ($context:expr, $stack:expr, *$value:expr) => {
        PUSH!(do $context, $stack, ptr::copy_nonoverlapping($value, $stack.top, 1))
    };
    ($context:expr, $stack:expr, $value:expr) => {
        PUSH!(do $context, $stack, ptr::write($stack.top, $value))
    };
}

macro_rules! POP {
    ($stack:expr) => {
        *{
            $stack.top = $stack.top.offset(-1);
            $stack.top
        }
    };
}

macro_rules! QUEUE_INIT {
    ($context:expr, $queue:expr, $type:ty) => {{
        $queue.start = yaml_malloc(16 * size_of::<$type>() as libc::c_ulong) as *mut $type;
        if !$queue.start.is_null() {
            $queue.tail = $queue.start;
            $queue.head = $queue.tail;
            $queue.end = $queue.start.offset(16_isize);
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    }};
}

macro_rules! QUEUE_DEL {
    ($queue:expr) => {
        yaml_free($queue.start as *mut libc::c_void);
        $queue.end = ptr::null_mut();
        $queue.tail = ptr::null_mut();
        $queue.head = ptr::null_mut();
        $queue.start = ptr::null_mut();
    };
}

macro_rules! QUEUE_EMPTY {
    ($queue:expr) => {
        $queue.head == $queue.tail
    };
}

macro_rules! ENQUEUE {
    (do $context:expr, $queue:expr, $enqueue:expr) => {
        if $queue.tail != $queue.end
            || yaml_queue_extend(
                addr_of_mut!($queue.start) as *mut *mut libc::c_void,
                addr_of_mut!($queue.head) as *mut *mut libc::c_void,
                addr_of_mut!($queue.tail) as *mut *mut libc::c_void,
                addr_of_mut!($queue.end) as *mut *mut libc::c_void,
            ) != 0
        {
            $enqueue;
            $queue.tail = $queue.tail.wrapping_offset(1);
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    };
    ($context:expr, $queue:expr, *$value:expr) => {
        ENQUEUE!(do $context, $queue, ptr::copy_nonoverlapping($value, $queue.tail, 1))
    };
    ($context:expr, $queue:expr, $value:expr) => {
        ENQUEUE!(do $context, $queue, ptr::write($queue.tail, $value))
    };
}

macro_rules! DEQUEUE {
    ($queue:expr) => {
        *{
            let head = $queue.head;
            $queue.head = $queue.head.wrapping_offset(1);
            head
        }
    };
}

macro_rules! QUEUE_INSERT {
    ($context:expr, $queue:expr, $index:expr, $value:expr) => {
        if $queue.tail != $queue.end
            || yaml_queue_extend(
                addr_of_mut!($queue.start) as *mut *mut libc::c_void,
                addr_of_mut!($queue.head) as *mut *mut libc::c_void,
                addr_of_mut!($queue.tail) as *mut *mut libc::c_void,
                addr_of_mut!($queue.end) as *mut *mut libc::c_void,
            ) != 0
        {
            memmove(
                ($queue.head)
                    .wrapping_offset($index as isize)
                    .wrapping_offset(1_isize) as *mut libc::c_void,
                ($queue.head).wrapping_offset($index as isize) as *const libc::c_void,
                (($queue.tail).c_offset_from($queue.head) as libc::c_long as libc::c_ulong)
                    .wrapping_sub($index)
                    .wrapping_mul(size_of::<yaml_token_t>() as libc::c_ulong),
            );
            *($queue.head).wrapping_offset($index as isize) = $value;
            let fresh14 = addr_of_mut!($queue.tail);
            *fresh14 = (*fresh14).wrapping_offset(1);
            1_i32
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            0_i32
        }
    };
}
