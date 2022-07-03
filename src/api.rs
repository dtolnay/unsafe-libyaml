use crate::externs::__assert_fail;
use crate::libc;
use crate::yaml::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_get_version_string() -> *const libc::c_char {
    return b"0.2.5\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_get_version(
    mut major: *mut libc::c_int,
    mut minor: *mut libc::c_int,
    mut patch: *mut libc::c_int,
) {
    *major = 0 as libc::c_int;
    *minor = 2 as libc::c_int;
    *patch = 5 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_malloc(mut size: size_t) -> *mut libc::c_void {
    return malloc(if size != 0 { size } else { 1 as libc::c_int as libc::c_ulong });
}
#[no_mangle]
pub unsafe extern "C" fn yaml_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return if !ptr.is_null() {
        realloc(ptr, if size != 0 { size } else { 1 as libc::c_int as libc::c_ulong })
    } else {
        malloc(if size != 0 { size } else { 1 as libc::c_int as libc::c_ulong })
    };
}
#[no_mangle]
pub unsafe extern "C" fn yaml_free(mut ptr: *mut libc::c_void) {
    if !ptr.is_null() {
        free(ptr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn yaml_strdup(mut str: *const yaml_char_t) -> *mut yaml_char_t {
    if str.is_null() {
        return 0 as *mut yaml_char_t;
    }
    return strdup(str as *mut libc::c_char) as *mut yaml_char_t;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_string_extend(
    mut start: *mut *mut yaml_char_t,
    mut pointer: *mut *mut yaml_char_t,
    mut end: *mut *mut yaml_char_t,
) -> libc::c_int {
    let mut new_start: *mut yaml_char_t = yaml_realloc(
        *start as *mut libc::c_void,
        ((*end).offset_from(*start) as libc::c_long * 2 as libc::c_int as libc::c_long)
            as size_t,
    ) as *mut yaml_char_t;
    if new_start.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        new_start.offset((*end).offset_from(*start) as libc::c_long as isize)
            as *mut libc::c_void,
        0 as libc::c_int,
        (*end).offset_from(*start) as libc::c_long as libc::c_ulong,
    );
    *pointer = new_start.offset((*pointer).offset_from(*start) as libc::c_long as isize);
    *end = new_start
        .offset(
            ((*end).offset_from(*start) as libc::c_long
                * 2 as libc::c_int as libc::c_long) as isize,
        );
    *start = new_start;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_string_join(
    mut a_start: *mut *mut yaml_char_t,
    mut a_pointer: *mut *mut yaml_char_t,
    mut a_end: *mut *mut yaml_char_t,
    mut b_start: *mut *mut yaml_char_t,
    mut b_pointer: *mut *mut yaml_char_t,
    mut b_end: *mut *mut yaml_char_t,
) -> libc::c_int {
    if *b_start == *b_pointer {
        return 1 as libc::c_int;
    }
    while (*a_end).offset_from(*a_pointer) as libc::c_long
        <= (*b_pointer).offset_from(*b_start) as libc::c_long
    {
        if yaml_string_extend(a_start, a_pointer, a_end) == 0 {
            return 0 as libc::c_int;
        }
    }
    memcpy(
        *a_pointer as *mut libc::c_void,
        *b_start as *const libc::c_void,
        (*b_pointer).offset_from(*b_start) as libc::c_long as libc::c_ulong,
    );
    *a_pointer = (*a_pointer)
        .offset((*b_pointer).offset_from(*b_start) as libc::c_long as isize);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_stack_extend(
    mut start: *mut *mut libc::c_void,
    mut top: *mut *mut libc::c_void,
    mut end: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut new_start: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
        as libc::c_long >= (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_long
    {
        return 0 as libc::c_int;
    }
    new_start = yaml_realloc(
        *start,
        ((*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
            as libc::c_long * 2 as libc::c_int as libc::c_long) as size_t,
    );
    if new_start.is_null() {
        return 0 as libc::c_int;
    }
    *top = (new_start as *mut libc::c_char)
        .offset(
            (*top as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                as libc::c_long as isize,
        ) as *mut libc::c_void;
    *end = (new_start as *mut libc::c_char)
        .offset(
            ((*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                as libc::c_long * 2 as libc::c_int as libc::c_long) as isize,
        ) as *mut libc::c_void;
    *start = new_start;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_queue_extend(
    mut start: *mut *mut libc::c_void,
    mut head: *mut *mut libc::c_void,
    mut tail: *mut *mut libc::c_void,
    mut end: *mut *mut libc::c_void,
) -> libc::c_int {
    if *start == *head && *tail == *end {
        let mut new_start: *mut libc::c_void = yaml_realloc(
            *start,
            ((*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                as libc::c_long * 2 as libc::c_int as libc::c_long) as size_t,
        );
        if new_start.is_null() {
            return 0 as libc::c_int;
        }
        *head = (new_start as *mut libc::c_char)
            .offset(
                (*head as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                    as libc::c_long as isize,
            ) as *mut libc::c_void;
        *tail = (new_start as *mut libc::c_char)
            .offset(
                (*tail as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                    as libc::c_long as isize,
            ) as *mut libc::c_void;
        *end = (new_start as *mut libc::c_char)
            .offset(
                ((*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                    as libc::c_long * 2 as libc::c_int as libc::c_long) as isize,
            ) as *mut libc::c_void;
        *start = new_start;
    }
    if *tail == *end {
        if *head != *tail {
            memmove(
                *start,
                *head,
                (*tail as *mut libc::c_char).offset_from(*head as *mut libc::c_char)
                    as libc::c_long as libc::c_ulong,
            );
        }
        *tail = (*start as *mut libc::c_char)
            .offset(
                (*tail as *mut libc::c_char).offset_from(*head as *mut libc::c_char)
                    as libc::c_long as isize,
            ) as *mut libc::c_void;
        *head = *start;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_initialize(
    mut parser: *mut yaml_parser_t,
) -> libc::c_int {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"int yaml_parser_initialize(yaml_parser_t *)\0"))
                .as_ptr(),
        );
    }
    memset(
        parser as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_parser_t>() as libc::c_ulong,
    );
    let ref mut fresh0 = (*parser).raw_buffer.start;
    *fresh0 = yaml_malloc(16384 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !(*fresh0).is_null() {
        let ref mut fresh1 = (*parser).raw_buffer.pointer;
        *fresh1 = (*parser).raw_buffer.start;
        let ref mut fresh2 = (*parser).raw_buffer.last;
        *fresh2 = *fresh1;
        let ref mut fresh3 = (*parser).raw_buffer.end;
        *fresh3 = ((*parser).raw_buffer.start).offset(16384 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        let ref mut fresh4 = (*parser).buffer.start;
        *fresh4 = yaml_malloc((16384 as libc::c_int * 3 as libc::c_int) as size_t)
            as *mut yaml_char_t;
        if !(if !(*fresh4).is_null() {
            let ref mut fresh5 = (*parser).buffer.pointer;
            *fresh5 = (*parser).buffer.start;
            let ref mut fresh6 = (*parser).buffer.last;
            *fresh6 = *fresh5;
            let ref mut fresh7 = (*parser).buffer.end;
            *fresh7 = ((*parser).buffer.start)
                .offset((16384 as libc::c_int * 3 as libc::c_int) as isize);
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0)
        {
            let ref mut fresh8 = (*parser).tokens.start;
            *fresh8 = yaml_malloc(
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<yaml_token_t>() as libc::c_ulong),
            ) as *mut yaml_token_t;
            if !(if !(*fresh8).is_null() {
                let ref mut fresh9 = (*parser).tokens.tail;
                *fresh9 = (*parser).tokens.start;
                let ref mut fresh10 = (*parser).tokens.head;
                *fresh10 = *fresh9;
                let ref mut fresh11 = (*parser).tokens.end;
                *fresh11 = ((*parser).tokens.start).offset(16 as libc::c_int as isize);
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                let ref mut fresh12 = (*parser).indents.start;
                *fresh12 = yaml_malloc(
                    (16 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_int;
                if !(if !(*fresh12).is_null() {
                    let ref mut fresh13 = (*parser).indents.top;
                    *fresh13 = (*parser).indents.start;
                    let ref mut fresh14 = (*parser).indents.end;
                    *fresh14 = ((*parser).indents.start)
                        .offset(16 as libc::c_int as isize);
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    let ref mut fresh15 = (*parser).simple_keys.start;
                    *fresh15 = yaml_malloc(
                        (16 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<yaml_simple_key_t>() as libc::c_ulong,
                            ),
                    ) as *mut yaml_simple_key_t;
                    if !(if !(*fresh15).is_null() {
                        let ref mut fresh16 = (*parser).simple_keys.top;
                        *fresh16 = (*parser).simple_keys.start;
                        let ref mut fresh17 = (*parser).simple_keys.end;
                        *fresh17 = ((*parser).simple_keys.start)
                            .offset(16 as libc::c_int as isize);
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        let ref mut fresh18 = (*parser).states.start;
                        *fresh18 = yaml_malloc(
                            (16 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<yaml_parser_state_t>()
                                        as libc::c_ulong,
                                ),
                        ) as *mut yaml_parser_state_t;
                        if !(if !(*fresh18).is_null() {
                            let ref mut fresh19 = (*parser).states.top;
                            *fresh19 = (*parser).states.start;
                            let ref mut fresh20 = (*parser).states.end;
                            *fresh20 = ((*parser).states.start)
                                .offset(16 as libc::c_int as isize);
                            1 as libc::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0)
                        {
                            let ref mut fresh21 = (*parser).marks.start;
                            *fresh21 = yaml_malloc(
                                (16 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<yaml_mark_t>() as libc::c_ulong,
                                    ),
                            ) as *mut yaml_mark_t;
                            if !(if !(*fresh21).is_null() {
                                let ref mut fresh22 = (*parser).marks.top;
                                *fresh22 = (*parser).marks.start;
                                let ref mut fresh23 = (*parser).marks.end;
                                *fresh23 = ((*parser).marks.start)
                                    .offset(16 as libc::c_int as isize);
                                1 as libc::c_int
                            } else {
                                (*parser).error = YAML_MEMORY_ERROR;
                                0 as libc::c_int
                            } == 0)
                            {
                                let ref mut fresh24 = (*parser).tag_directives.start;
                                *fresh24 = yaml_malloc(
                                    (16 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<yaml_tag_directive_t>()
                                                as libc::c_ulong,
                                        ),
                                ) as *mut yaml_tag_directive_t;
                                if !(if !(*fresh24).is_null() {
                                    let ref mut fresh25 = (*parser).tag_directives.top;
                                    *fresh25 = (*parser).tag_directives.start;
                                    let ref mut fresh26 = (*parser).tag_directives.end;
                                    *fresh26 = ((*parser).tag_directives.start)
                                        .offset(16 as libc::c_int as isize);
                                    1 as libc::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as libc::c_int
                                } == 0)
                                {
                                    return 1 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free((*parser).raw_buffer.start as *mut libc::c_void);
    let ref mut fresh27 = (*parser).raw_buffer.end;
    *fresh27 = 0 as *mut libc::c_uchar;
    let ref mut fresh28 = (*parser).raw_buffer.pointer;
    *fresh28 = *fresh27;
    let ref mut fresh29 = (*parser).raw_buffer.start;
    *fresh29 = *fresh28;
    yaml_free((*parser).buffer.start as *mut libc::c_void);
    let ref mut fresh30 = (*parser).buffer.end;
    *fresh30 = 0 as *mut yaml_char_t;
    let ref mut fresh31 = (*parser).buffer.pointer;
    *fresh31 = *fresh30;
    let ref mut fresh32 = (*parser).buffer.start;
    *fresh32 = *fresh31;
    yaml_free((*parser).tokens.start as *mut libc::c_void);
    let ref mut fresh33 = (*parser).tokens.end;
    *fresh33 = 0 as *mut yaml_token_t;
    let ref mut fresh34 = (*parser).tokens.tail;
    *fresh34 = *fresh33;
    let ref mut fresh35 = (*parser).tokens.head;
    *fresh35 = *fresh34;
    let ref mut fresh36 = (*parser).tokens.start;
    *fresh36 = *fresh35;
    yaml_free((*parser).indents.start as *mut libc::c_void);
    let ref mut fresh37 = (*parser).indents.end;
    *fresh37 = 0 as *mut libc::c_int;
    let ref mut fresh38 = (*parser).indents.top;
    *fresh38 = *fresh37;
    let ref mut fresh39 = (*parser).indents.start;
    *fresh39 = *fresh38;
    yaml_free((*parser).simple_keys.start as *mut libc::c_void);
    let ref mut fresh40 = (*parser).simple_keys.end;
    *fresh40 = 0 as *mut yaml_simple_key_t;
    let ref mut fresh41 = (*parser).simple_keys.top;
    *fresh41 = *fresh40;
    let ref mut fresh42 = (*parser).simple_keys.start;
    *fresh42 = *fresh41;
    yaml_free((*parser).states.start as *mut libc::c_void);
    let ref mut fresh43 = (*parser).states.end;
    *fresh43 = 0 as *mut yaml_parser_state_t;
    let ref mut fresh44 = (*parser).states.top;
    *fresh44 = *fresh43;
    let ref mut fresh45 = (*parser).states.start;
    *fresh45 = *fresh44;
    yaml_free((*parser).marks.start as *mut libc::c_void);
    let ref mut fresh46 = (*parser).marks.end;
    *fresh46 = 0 as *mut yaml_mark_t;
    let ref mut fresh47 = (*parser).marks.top;
    *fresh47 = *fresh46;
    let ref mut fresh48 = (*parser).marks.start;
    *fresh48 = *fresh47;
    yaml_free((*parser).tag_directives.start as *mut libc::c_void);
    let ref mut fresh49 = (*parser).tag_directives.end;
    *fresh49 = 0 as *mut yaml_tag_directive_t;
    let ref mut fresh50 = (*parser).tag_directives.top;
    *fresh50 = *fresh49;
    let ref mut fresh51 = (*parser).tag_directives.start;
    *fresh51 = *fresh50;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_delete(mut parser: *mut yaml_parser_t) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void yaml_parser_delete(yaml_parser_t *)\0"))
                .as_ptr(),
        );
    }
    yaml_free((*parser).raw_buffer.start as *mut libc::c_void);
    let ref mut fresh52 = (*parser).raw_buffer.end;
    *fresh52 = 0 as *mut libc::c_uchar;
    let ref mut fresh53 = (*parser).raw_buffer.pointer;
    *fresh53 = *fresh52;
    let ref mut fresh54 = (*parser).raw_buffer.start;
    *fresh54 = *fresh53;
    yaml_free((*parser).buffer.start as *mut libc::c_void);
    let ref mut fresh55 = (*parser).buffer.end;
    *fresh55 = 0 as *mut yaml_char_t;
    let ref mut fresh56 = (*parser).buffer.pointer;
    *fresh56 = *fresh55;
    let ref mut fresh57 = (*parser).buffer.start;
    *fresh57 = *fresh56;
    while !((*parser).tokens.head == (*parser).tokens.tail) {
        let ref mut fresh58 = (*parser).tokens.head;
        let fresh59 = *fresh58;
        *fresh58 = (*fresh58).offset(1);
        yaml_token_delete(fresh59);
    }
    yaml_free((*parser).tokens.start as *mut libc::c_void);
    let ref mut fresh60 = (*parser).tokens.end;
    *fresh60 = 0 as *mut yaml_token_t;
    let ref mut fresh61 = (*parser).tokens.tail;
    *fresh61 = *fresh60;
    let ref mut fresh62 = (*parser).tokens.head;
    *fresh62 = *fresh61;
    let ref mut fresh63 = (*parser).tokens.start;
    *fresh63 = *fresh62;
    yaml_free((*parser).indents.start as *mut libc::c_void);
    let ref mut fresh64 = (*parser).indents.end;
    *fresh64 = 0 as *mut libc::c_int;
    let ref mut fresh65 = (*parser).indents.top;
    *fresh65 = *fresh64;
    let ref mut fresh66 = (*parser).indents.start;
    *fresh66 = *fresh65;
    yaml_free((*parser).simple_keys.start as *mut libc::c_void);
    let ref mut fresh67 = (*parser).simple_keys.end;
    *fresh67 = 0 as *mut yaml_simple_key_t;
    let ref mut fresh68 = (*parser).simple_keys.top;
    *fresh68 = *fresh67;
    let ref mut fresh69 = (*parser).simple_keys.start;
    *fresh69 = *fresh68;
    yaml_free((*parser).states.start as *mut libc::c_void);
    let ref mut fresh70 = (*parser).states.end;
    *fresh70 = 0 as *mut yaml_parser_state_t;
    let ref mut fresh71 = (*parser).states.top;
    *fresh71 = *fresh70;
    let ref mut fresh72 = (*parser).states.start;
    *fresh72 = *fresh71;
    yaml_free((*parser).marks.start as *mut libc::c_void);
    let ref mut fresh73 = (*parser).marks.end;
    *fresh73 = 0 as *mut yaml_mark_t;
    let ref mut fresh74 = (*parser).marks.top;
    *fresh74 = *fresh73;
    let ref mut fresh75 = (*parser).marks.start;
    *fresh75 = *fresh74;
    while !((*parser).tag_directives.start == (*parser).tag_directives.top) {
        let ref mut fresh76 = (*parser).tag_directives.top;
        *fresh76 = (*fresh76).offset(-1);
        let mut tag_directive: yaml_tag_directive_t = **fresh76;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    yaml_free((*parser).tag_directives.start as *mut libc::c_void);
    let ref mut fresh77 = (*parser).tag_directives.end;
    *fresh77 = 0 as *mut yaml_tag_directive_t;
    let ref mut fresh78 = (*parser).tag_directives.top;
    *fresh78 = *fresh77;
    let ref mut fresh79 = (*parser).tag_directives.start;
    *fresh79 = *fresh78;
    memset(
        parser as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_parser_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn yaml_string_read_handler(
    mut data: *mut libc::c_void,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
    mut size_read: *mut size_t,
) -> libc::c_int {
    let mut parser: *mut yaml_parser_t = data as *mut yaml_parser_t;
    if (*parser).input.string.current == (*parser).input.string.end {
        *size_read = 0 as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    if size
        > ((*parser).input.string.end).offset_from((*parser).input.string.current)
            as libc::c_long as size_t
    {
        size = ((*parser).input.string.end).offset_from((*parser).input.string.current)
            as libc::c_long as size_t;
    }
    memcpy(
        buffer as *mut libc::c_void,
        (*parser).input.string.current as *const libc::c_void,
        size,
    );
    let ref mut fresh80 = (*parser).input.string.current;
    *fresh80 = (*fresh80).offset(size as isize);
    *size_read = size;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_file_read_handler(
    mut data: *mut libc::c_void,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
    mut size_read: *mut size_t,
) -> libc::c_int {
    let mut parser: *mut yaml_parser_t = data as *mut yaml_parser_t;
    *size_read = fread(
        buffer as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        size,
        (*parser).input.file,
    );
    return (ferror((*parser).input.file) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_input_string(
    mut parser: *mut yaml_parser_t,
    mut input: *const libc::c_uchar,
    mut size: size_t,
) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void yaml_parser_set_input_string(yaml_parser_t *, const unsigned char *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*parser).read_handler).is_none() {} else {
        __assert_fail(
            b"!parser->read_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void yaml_parser_set_input_string(yaml_parser_t *, const unsigned char *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    if !input.is_null() {} else {
        __assert_fail(
            b"input\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void yaml_parser_set_input_string(yaml_parser_t *, const unsigned char *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh81 = (*parser).read_handler;
    *fresh81 = Some(
        yaml_string_read_handler
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                size_t,
                *mut size_t,
            ) -> libc::c_int,
    );
    let ref mut fresh82 = (*parser).read_handler_data;
    *fresh82 = parser as *mut libc::c_void;
    let ref mut fresh83 = (*parser).input.string.start;
    *fresh83 = input;
    let ref mut fresh84 = (*parser).input.string.current;
    *fresh84 = input;
    let ref mut fresh85 = (*parser).input.string.end;
    *fresh85 = input.offset(size as isize);
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_input_file(
    mut parser: *mut yaml_parser_t,
    mut file: *mut FILE,
) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void yaml_parser_set_input_file(yaml_parser_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    if ((*parser).read_handler).is_none() {} else {
        __assert_fail(
            b"!parser->read_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void yaml_parser_set_input_file(yaml_parser_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    if !file.is_null() {} else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            313 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void yaml_parser_set_input_file(yaml_parser_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh86 = (*parser).read_handler;
    *fresh86 = Some(
        yaml_file_read_handler
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                size_t,
                *mut size_t,
            ) -> libc::c_int,
    );
    let ref mut fresh87 = (*parser).read_handler_data;
    *fresh87 = parser as *mut libc::c_void;
    let ref mut fresh88 = (*parser).input.file;
    *fresh88 = file;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_input(
    mut parser: *mut yaml_parser_t,
    mut handler: Option::<yaml_read_handler_t>,
    mut data: *mut libc::c_void,
) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void yaml_parser_set_input(yaml_parser_t *, yaml_read_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*parser).read_handler).is_none() {} else {
        __assert_fail(
            b"!parser->read_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void yaml_parser_set_input(yaml_parser_t *, yaml_read_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    if handler.is_some() {} else {
        __assert_fail(
            b"handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            331 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void yaml_parser_set_input(yaml_parser_t *, yaml_read_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh89 = (*parser).read_handler;
    *fresh89 = handler;
    let ref mut fresh90 = (*parser).read_handler_data;
    *fresh90 = data;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_encoding(
    mut parser: *mut yaml_parser_t,
    mut encoding: yaml_encoding_t,
) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void yaml_parser_set_encoding(yaml_parser_t *, yaml_encoding_t)\0"))
                .as_ptr(),
        );
    }
    if (*parser).encoding as u64 == 0 {} else {
        __assert_fail(
            b"!parser->encoding\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            345 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void yaml_parser_set_encoding(yaml_parser_t *, yaml_encoding_t)\0"))
                .as_ptr(),
        );
    }
    (*parser).encoding = encoding;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_initialize(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            357 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int yaml_emitter_initialize(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    memset(
        emitter as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_emitter_t>() as libc::c_ulong,
    );
    let ref mut fresh91 = (*emitter).buffer.start;
    *fresh91 = yaml_malloc(16384 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !(*fresh91).is_null() {
        let ref mut fresh92 = (*emitter).buffer.pointer;
        *fresh92 = (*emitter).buffer.start;
        let ref mut fresh93 = (*emitter).buffer.last;
        *fresh93 = *fresh92;
        let ref mut fresh94 = (*emitter).buffer.end;
        *fresh94 = ((*emitter).buffer.start).offset(16384 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        let ref mut fresh95 = (*emitter).raw_buffer.start;
        *fresh95 = yaml_malloc(
            (16384 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as size_t,
        ) as *mut yaml_char_t;
        if !(if !(*fresh95).is_null() {
            let ref mut fresh96 = (*emitter).raw_buffer.pointer;
            *fresh96 = (*emitter).raw_buffer.start;
            let ref mut fresh97 = (*emitter).raw_buffer.last;
            *fresh97 = *fresh96;
            let ref mut fresh98 = (*emitter).raw_buffer.end;
            *fresh98 = ((*emitter).raw_buffer.start)
                .offset(
                    (16384 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as isize,
                );
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0)
        {
            let ref mut fresh99 = (*emitter).states.start;
            *fresh99 = yaml_malloc(
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<yaml_emitter_state_t>() as libc::c_ulong,
                    ),
            ) as *mut yaml_emitter_state_t;
            if !(if !(*fresh99).is_null() {
                let ref mut fresh100 = (*emitter).states.top;
                *fresh100 = (*emitter).states.start;
                let ref mut fresh101 = (*emitter).states.end;
                *fresh101 = ((*emitter).states.start).offset(16 as libc::c_int as isize);
                1 as libc::c_int
            } else {
                (*emitter).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                let ref mut fresh102 = (*emitter).events.start;
                *fresh102 = yaml_malloc(
                    (16 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                        ),
                ) as *mut yaml_event_t;
                if !(if !(*fresh102).is_null() {
                    let ref mut fresh103 = (*emitter).events.tail;
                    *fresh103 = (*emitter).events.start;
                    let ref mut fresh104 = (*emitter).events.head;
                    *fresh104 = *fresh103;
                    let ref mut fresh105 = (*emitter).events.end;
                    *fresh105 = ((*emitter).events.start)
                        .offset(16 as libc::c_int as isize);
                    1 as libc::c_int
                } else {
                    (*emitter).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    let ref mut fresh106 = (*emitter).indents.start;
                    *fresh106 = yaml_malloc(
                        (16 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_int;
                    if !(if !(*fresh106).is_null() {
                        let ref mut fresh107 = (*emitter).indents.top;
                        *fresh107 = (*emitter).indents.start;
                        let ref mut fresh108 = (*emitter).indents.end;
                        *fresh108 = ((*emitter).indents.start)
                            .offset(16 as libc::c_int as isize);
                        1 as libc::c_int
                    } else {
                        (*emitter).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        let ref mut fresh109 = (*emitter).tag_directives.start;
                        *fresh109 = yaml_malloc(
                            (16 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<yaml_tag_directive_t>()
                                        as libc::c_ulong,
                                ),
                        ) as *mut yaml_tag_directive_t;
                        if !(if !(*fresh109).is_null() {
                            let ref mut fresh110 = (*emitter).tag_directives.top;
                            *fresh110 = (*emitter).tag_directives.start;
                            let ref mut fresh111 = (*emitter).tag_directives.end;
                            *fresh111 = ((*emitter).tag_directives.start)
                                .offset(16 as libc::c_int as isize);
                            1 as libc::c_int
                        } else {
                            (*emitter).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0)
                        {
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_free((*emitter).buffer.start as *mut libc::c_void);
    let ref mut fresh112 = (*emitter).buffer.end;
    *fresh112 = 0 as *mut yaml_char_t;
    let ref mut fresh113 = (*emitter).buffer.pointer;
    *fresh113 = *fresh112;
    let ref mut fresh114 = (*emitter).buffer.start;
    *fresh114 = *fresh113;
    yaml_free((*emitter).raw_buffer.start as *mut libc::c_void);
    let ref mut fresh115 = (*emitter).raw_buffer.end;
    *fresh115 = 0 as *mut libc::c_uchar;
    let ref mut fresh116 = (*emitter).raw_buffer.pointer;
    *fresh116 = *fresh115;
    let ref mut fresh117 = (*emitter).raw_buffer.start;
    *fresh117 = *fresh116;
    yaml_free((*emitter).states.start as *mut libc::c_void);
    let ref mut fresh118 = (*emitter).states.end;
    *fresh118 = 0 as *mut yaml_emitter_state_t;
    let ref mut fresh119 = (*emitter).states.top;
    *fresh119 = *fresh118;
    let ref mut fresh120 = (*emitter).states.start;
    *fresh120 = *fresh119;
    yaml_free((*emitter).events.start as *mut libc::c_void);
    let ref mut fresh121 = (*emitter).events.end;
    *fresh121 = 0 as *mut yaml_event_t;
    let ref mut fresh122 = (*emitter).events.tail;
    *fresh122 = *fresh121;
    let ref mut fresh123 = (*emitter).events.head;
    *fresh123 = *fresh122;
    let ref mut fresh124 = (*emitter).events.start;
    *fresh124 = *fresh123;
    yaml_free((*emitter).indents.start as *mut libc::c_void);
    let ref mut fresh125 = (*emitter).indents.end;
    *fresh125 = 0 as *mut libc::c_int;
    let ref mut fresh126 = (*emitter).indents.top;
    *fresh126 = *fresh125;
    let ref mut fresh127 = (*emitter).indents.start;
    *fresh127 = *fresh126;
    yaml_free((*emitter).tag_directives.start as *mut libc::c_void);
    let ref mut fresh128 = (*emitter).tag_directives.end;
    *fresh128 = 0 as *mut yaml_tag_directive_t;
    let ref mut fresh129 = (*emitter).tag_directives.top;
    *fresh129 = *fresh128;
    let ref mut fresh130 = (*emitter).tag_directives.start;
    *fresh130 = *fresh129;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_delete(mut emitter: *mut yaml_emitter_t) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            394 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"void yaml_emitter_delete(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    yaml_free((*emitter).buffer.start as *mut libc::c_void);
    let ref mut fresh131 = (*emitter).buffer.end;
    *fresh131 = 0 as *mut yaml_char_t;
    let ref mut fresh132 = (*emitter).buffer.pointer;
    *fresh132 = *fresh131;
    let ref mut fresh133 = (*emitter).buffer.start;
    *fresh133 = *fresh132;
    yaml_free((*emitter).raw_buffer.start as *mut libc::c_void);
    let ref mut fresh134 = (*emitter).raw_buffer.end;
    *fresh134 = 0 as *mut libc::c_uchar;
    let ref mut fresh135 = (*emitter).raw_buffer.pointer;
    *fresh135 = *fresh134;
    let ref mut fresh136 = (*emitter).raw_buffer.start;
    *fresh136 = *fresh135;
    yaml_free((*emitter).states.start as *mut libc::c_void);
    let ref mut fresh137 = (*emitter).states.end;
    *fresh137 = 0 as *mut yaml_emitter_state_t;
    let ref mut fresh138 = (*emitter).states.top;
    *fresh138 = *fresh137;
    let ref mut fresh139 = (*emitter).states.start;
    *fresh139 = *fresh138;
    while !((*emitter).events.head == (*emitter).events.tail) {
        let ref mut fresh140 = (*emitter).events.head;
        let fresh141 = *fresh140;
        *fresh140 = (*fresh140).offset(1);
        yaml_event_delete(fresh141);
    }
    yaml_free((*emitter).events.start as *mut libc::c_void);
    let ref mut fresh142 = (*emitter).events.end;
    *fresh142 = 0 as *mut yaml_event_t;
    let ref mut fresh143 = (*emitter).events.tail;
    *fresh143 = *fresh142;
    let ref mut fresh144 = (*emitter).events.head;
    *fresh144 = *fresh143;
    let ref mut fresh145 = (*emitter).events.start;
    *fresh145 = *fresh144;
    yaml_free((*emitter).indents.start as *mut libc::c_void);
    let ref mut fresh146 = (*emitter).indents.end;
    *fresh146 = 0 as *mut libc::c_int;
    let ref mut fresh147 = (*emitter).indents.top;
    *fresh147 = *fresh146;
    let ref mut fresh148 = (*emitter).indents.start;
    *fresh148 = *fresh147;
    while !((*emitter).tag_directives.start == (*emitter).tag_directives.top) {
        let ref mut fresh149 = (*emitter).tag_directives.top;
        *fresh149 = (*fresh149).offset(-1);
        let mut tag_directive: yaml_tag_directive_t = **fresh149;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    yaml_free((*emitter).tag_directives.start as *mut libc::c_void);
    let ref mut fresh150 = (*emitter).tag_directives.end;
    *fresh150 = 0 as *mut yaml_tag_directive_t;
    let ref mut fresh151 = (*emitter).tag_directives.top;
    *fresh151 = *fresh150;
    let ref mut fresh152 = (*emitter).tag_directives.start;
    *fresh152 = *fresh151;
    yaml_free((*emitter).anchors as *mut libc::c_void);
    memset(
        emitter as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_emitter_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn yaml_string_write_handler(
    mut data: *mut libc::c_void,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
) -> libc::c_int {
    let mut emitter: *mut yaml_emitter_t = data as *mut yaml_emitter_t;
    if ((*emitter).output.string.size)
        .wrapping_sub(*(*emitter).output.string.size_written) < size
    {
        memcpy(
            ((*emitter).output.string.buffer)
                .offset(*(*emitter).output.string.size_written as isize)
                as *mut libc::c_void,
            buffer as *const libc::c_void,
            ((*emitter).output.string.size)
                .wrapping_sub(*(*emitter).output.string.size_written),
        );
        *(*emitter).output.string.size_written = (*emitter).output.string.size;
        return 0 as libc::c_int;
    }
    memcpy(
        ((*emitter).output.string.buffer)
            .offset(*(*emitter).output.string.size_written as isize)
            as *mut libc::c_void,
        buffer as *const libc::c_void,
        size,
    );
    let ref mut fresh153 = *(*emitter).output.string.size_written;
    *fresh153 = (*fresh153 as libc::c_ulong).wrapping_add(size) as size_t as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_file_write_handler(
    mut data: *mut libc::c_void,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
) -> libc::c_int {
    let mut emitter: *mut yaml_emitter_t = data as *mut yaml_emitter_t;
    return (fwrite(
        buffer as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        size,
        (*emitter).output.file,
    ) == size) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_output_string(
    mut emitter: *mut yaml_emitter_t,
    mut output: *mut libc::c_uchar,
    mut size: size_t,
    mut size_written: *mut size_t,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            460 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void yaml_emitter_set_output_string(yaml_emitter_t *, unsigned char *, size_t, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*emitter).write_handler).is_none() {} else {
        __assert_fail(
            b"!emitter->write_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void yaml_emitter_set_output_string(yaml_emitter_t *, unsigned char *, size_t, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if !output.is_null() {} else {
        __assert_fail(
            b"output\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            462 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void yaml_emitter_set_output_string(yaml_emitter_t *, unsigned char *, size_t, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh154 = (*emitter).write_handler;
    *fresh154 = Some(
        yaml_string_write_handler
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                size_t,
            ) -> libc::c_int,
    );
    let ref mut fresh155 = (*emitter).write_handler_data;
    *fresh155 = emitter as *mut libc::c_void;
    let ref mut fresh156 = (*emitter).output.string.buffer;
    *fresh156 = output;
    (*emitter).output.string.size = size;
    let ref mut fresh157 = (*emitter).output.string.size_written;
    *fresh157 = size_written;
    *size_written = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_output_file(
    mut emitter: *mut yaml_emitter_t,
    mut file: *mut FILE,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            480 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void yaml_emitter_set_output_file(yaml_emitter_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    if ((*emitter).write_handler).is_none() {} else {
        __assert_fail(
            b"!emitter->write_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            481 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void yaml_emitter_set_output_file(yaml_emitter_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    if !file.is_null() {} else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            482 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void yaml_emitter_set_output_file(yaml_emitter_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh158 = (*emitter).write_handler;
    *fresh158 = Some(
        yaml_file_write_handler
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                size_t,
            ) -> libc::c_int,
    );
    let ref mut fresh159 = (*emitter).write_handler_data;
    *fresh159 = emitter as *mut libc::c_void;
    let ref mut fresh160 = (*emitter).output.file;
    *fresh160 = file;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_output(
    mut emitter: *mut yaml_emitter_t,
    mut handler: Option::<yaml_write_handler_t>,
    mut data: *mut libc::c_void,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            498 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void yaml_emitter_set_output(yaml_emitter_t *, yaml_write_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*emitter).write_handler).is_none() {} else {
        __assert_fail(
            b"!emitter->write_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            499 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void yaml_emitter_set_output(yaml_emitter_t *, yaml_write_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    if handler.is_some() {} else {
        __assert_fail(
            b"handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            500 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void yaml_emitter_set_output(yaml_emitter_t *, yaml_write_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh161 = (*emitter).write_handler;
    *fresh161 = handler;
    let ref mut fresh162 = (*emitter).write_handler_data;
    *fresh162 = data;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_encoding(
    mut emitter: *mut yaml_emitter_t,
    mut encoding: yaml_encoding_t,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            513 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void yaml_emitter_set_encoding(yaml_emitter_t *, yaml_encoding_t)\0"))
                .as_ptr(),
        );
    }
    if (*emitter).encoding as u64 == 0 {} else {
        __assert_fail(
            b"!emitter->encoding\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            514 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void yaml_emitter_set_encoding(yaml_emitter_t *, yaml_encoding_t)\0"))
                .as_ptr(),
        );
    }
    (*emitter).encoding = encoding;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_canonical(
    mut emitter: *mut yaml_emitter_t,
    mut canonical: libc::c_int,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void yaml_emitter_set_canonical(yaml_emitter_t *, int)\0"))
                .as_ptr(),
        );
    }
    (*emitter).canonical = (canonical != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_indent(
    mut emitter: *mut yaml_emitter_t,
    mut indent: libc::c_int,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            538 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void yaml_emitter_set_indent(yaml_emitter_t *, int)\0"))
                .as_ptr(),
        );
    }
    (*emitter)
        .best_indent = if (1 as libc::c_int) < indent && indent < 10 as libc::c_int {
        indent
    } else {
        2 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_width(
    mut emitter: *mut yaml_emitter_t,
    mut width: libc::c_int,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void yaml_emitter_set_width(yaml_emitter_t *, int)\0"))
                .as_ptr(),
        );
    }
    (*emitter)
        .best_width = if width >= 0 as libc::c_int {
        width
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_unicode(
    mut emitter: *mut yaml_emitter_t,
    mut unicode: libc::c_int,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            562 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void yaml_emitter_set_unicode(yaml_emitter_t *, int)\0"))
                .as_ptr(),
        );
    }
    (*emitter).unicode = (unicode != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_break(
    mut emitter: *mut yaml_emitter_t,
    mut line_break: yaml_break_t,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            574 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void yaml_emitter_set_break(yaml_emitter_t *, yaml_break_t)\0"))
                .as_ptr(),
        );
    }
    (*emitter).line_break = line_break;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_token_delete(mut token: *mut yaml_token_t) {
    if !token.is_null() {} else {
        __assert_fail(
            b"token\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            586 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void yaml_token_delete(yaml_token_t *)\0"))
                .as_ptr(),
        );
    }
    match (*token).type_0 as libc::c_uint {
        4 => {
            yaml_free((*token).data.tag_directive.handle as *mut libc::c_void);
            yaml_free((*token).data.tag_directive.prefix as *mut libc::c_void);
        }
        18 => {
            yaml_free((*token).data.alias.value as *mut libc::c_void);
        }
        19 => {
            yaml_free((*token).data.anchor.value as *mut libc::c_void);
        }
        20 => {
            yaml_free((*token).data.tag.handle as *mut libc::c_void);
            yaml_free((*token).data.tag.suffix as *mut libc::c_void);
        }
        21 => {
            yaml_free((*token).data.scalar.value as *mut libc::c_void);
        }
        _ => {}
    }
    memset(
        token as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn yaml_check_utf8(
    mut start: *const yaml_char_t,
    mut length: size_t,
) -> libc::c_int {
    let mut end: *const yaml_char_t = start.offset(length as isize);
    let mut pointer: *const yaml_char_t = start;
    while pointer < end {
        let mut octet: libc::c_uchar = 0;
        let mut width: libc::c_uint = 0;
        let mut value: libc::c_uint = 0;
        let mut k: size_t = 0;
        octet = *pointer.offset(0 as libc::c_int as isize);
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
        if width == 0 {
            return 0 as libc::c_int;
        }
        if pointer.offset(width as isize) > end {
            return 0 as libc::c_int;
        }
        k = 1 as libc::c_int as size_t;
        while k < width as libc::c_ulong {
            octet = *pointer.offset(k as isize);
            if octet as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int {
                return 0 as libc::c_int;
            }
            value = (value << 6 as libc::c_int)
                .wrapping_add(
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
            return 0 as libc::c_int;
        }
        pointer = pointer.offset(width as isize);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_stream_start_event_initialize(
    mut event: *mut yaml_event_t,
    mut encoding: yaml_encoding_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            674 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_stream_start_event_initialize(yaml_event_t *, yaml_encoding_t)\0",
            ))
                .as_ptr(),
        );
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_STREAM_START_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.stream_start.encoding = encoding;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_stream_end_event_initialize(
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            690 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"int yaml_stream_end_event_initialize(yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_STREAM_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_start_event_initialize(
    mut event: *mut yaml_event_t,
    mut version_directive: *mut yaml_version_directive_t,
    mut tag_directives_start: *mut yaml_tag_directive_t,
    mut tag_directives_end: *mut yaml_tag_directive_t,
    mut implicit: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut context: Unnamed_17 = Unnamed_17 {
        error: YAML_NO_ERROR,
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut version_directive_copy: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut tag_directives_copy: Unnamed_16 = {
        let mut init = Unnamed_16 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        };
        init
    };
    let mut value: yaml_tag_directive_t = {
        let mut init = yaml_tag_directive_s {
            handle: 0 as *mut yaml_char_t,
            prefix: 0 as *mut yaml_char_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            720 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 138],
                &[libc::c_char; 138],
            >(
                b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if !tag_directives_start.is_null() && !tag_directives_end.is_null()
        || tag_directives_start == tag_directives_end
    {} else {
        __assert_fail(
            b"(tag_directives_start && tag_directives_end) || (tag_directives_start == tag_directives_end)\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            722 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 138],
                &[libc::c_char; 138],
            >(
                b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if !version_directive.is_null() {
        version_directive_copy = yaml_malloc(
            ::std::mem::size_of::<yaml_version_directive_t>() as libc::c_ulong,
        ) as *mut yaml_version_directive_t;
        if version_directive_copy.is_null() {
            current_block = 14964981520188694172;
        } else {
            (*version_directive_copy).major = (*version_directive).major;
            (*version_directive_copy).minor = (*version_directive).minor;
            current_block = 1394248824506584008;
        }
    } else {
        current_block = 1394248824506584008;
    }
    match current_block {
        1394248824506584008 => {
            if tag_directives_start != tag_directives_end {
                let mut tag_directive: *mut yaml_tag_directive_t = 0
                    as *mut yaml_tag_directive_t;
                tag_directives_copy
                    .start = yaml_malloc(
                    (16 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<yaml_tag_directive_t>()
                                as libc::c_ulong,
                        ),
                ) as *mut yaml_tag_directive_t;
                if if !(tag_directives_copy.start).is_null() {
                    tag_directives_copy.top = tag_directives_copy.start;
                    tag_directives_copy
                        .end = (tag_directives_copy.start)
                        .offset(16 as libc::c_int as isize);
                    1 as libc::c_int
                } else {
                    context.error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0
                {
                    current_block = 14964981520188694172;
                } else {
                    tag_directive = tag_directives_start;
                    loop {
                        if !(tag_directive != tag_directives_end) {
                            current_block = 16203760046146113240;
                            break;
                        }
                        if !((*tag_directive).handle).is_null() {} else {
                            __assert_fail(
                                b"tag_directive->handle\0" as *const u8
                                    as *const libc::c_char,
                                b"api.c\0" as *const u8 as *const libc::c_char,
                                738 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 138],
                                    &[libc::c_char; 138],
                                >(
                                    b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        if !((*tag_directive).prefix).is_null() {} else {
                            __assert_fail(
                                b"tag_directive->prefix\0" as *const u8
                                    as *const libc::c_char,
                                b"api.c\0" as *const u8 as *const libc::c_char,
                                739 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 138],
                                    &[libc::c_char; 138],
                                >(
                                    b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        if yaml_check_utf8(
                            (*tag_directive).handle,
                            strlen((*tag_directive).handle as *mut libc::c_char),
                        ) == 0
                        {
                            current_block = 14964981520188694172;
                            break;
                        }
                        if yaml_check_utf8(
                            (*tag_directive).prefix,
                            strlen((*tag_directive).prefix as *mut libc::c_char),
                        ) == 0
                        {
                            current_block = 14964981520188694172;
                            break;
                        }
                        value.handle = yaml_strdup((*tag_directive).handle);
                        value.prefix = yaml_strdup((*tag_directive).prefix);
                        if (value.handle).is_null() || (value.prefix).is_null() {
                            current_block = 14964981520188694172;
                            break;
                        }
                        if if tag_directives_copy.top != tag_directives_copy.end
                            || yaml_stack_extend(
                                &mut tag_directives_copy.start
                                    as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                &mut tag_directives_copy.top
                                    as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                &mut tag_directives_copy.end
                                    as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                            ) != 0
                        {
                            let fresh163 = tag_directives_copy.top;
                            tag_directives_copy
                                .top = (tag_directives_copy.top).offset(1);
                            *fresh163 = value;
                            1 as libc::c_int
                        } else {
                            context.error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0
                        {
                            current_block = 14964981520188694172;
                            break;
                        }
                        value.handle = 0 as *mut yaml_char_t;
                        value.prefix = 0 as *mut yaml_char_t;
                        tag_directive = tag_directive.offset(1);
                    }
                }
            } else {
                current_block = 16203760046146113240;
            }
            match current_block {
                14964981520188694172 => {}
                _ => {
                    memset(
                        event as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    (*event).type_0 = YAML_DOCUMENT_START_EVENT;
                    (*event).start_mark = mark;
                    (*event).end_mark = mark;
                    let ref mut fresh164 = (*event)
                        .data
                        .document_start
                        .version_directive;
                    *fresh164 = version_directive_copy;
                    let ref mut fresh165 = (*event)
                        .data
                        .document_start
                        .tag_directives
                        .start;
                    *fresh165 = tag_directives_copy.start;
                    let ref mut fresh166 = (*event)
                        .data
                        .document_start
                        .tag_directives
                        .end;
                    *fresh166 = tag_directives_copy.top;
                    (*event).data.document_start.implicit = implicit;
                    return 1 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    yaml_free(version_directive_copy as *mut libc::c_void);
    while !(tag_directives_copy.start == tag_directives_copy.top) {
        tag_directives_copy.top = (tag_directives_copy.top).offset(-1);
        let mut value_0: yaml_tag_directive_t = *tag_directives_copy.top;
        yaml_free(value_0.handle as *mut libc::c_void);
        yaml_free(value_0.prefix as *mut libc::c_void);
    }
    yaml_free(tag_directives_copy.start as *mut libc::c_void);
    tag_directives_copy.end = 0 as *mut yaml_tag_directive_t;
    tag_directives_copy.top = tag_directives_copy.end;
    tag_directives_copy.start = tag_directives_copy.top;
    yaml_free(value.handle as *mut libc::c_void);
    yaml_free(value.prefix as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_end_event_initialize(
    mut event: *mut yaml_event_t,
    mut implicit: libc::c_int,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            785 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"int yaml_document_end_event_initialize(yaml_event_t *, int)\0"))
                .as_ptr(),
        );
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_DOCUMENT_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.document_end.implicit = implicit;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_alias_event_initialize(
    mut event: *mut yaml_event_t,
    mut anchor: *const yaml_char_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut anchor_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            802 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"int yaml_alias_event_initialize(yaml_event_t *, const yaml_char_t *)\0"))
                .as_ptr(),
        );
    }
    if !anchor.is_null() {} else {
        __assert_fail(
            b"anchor\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            803 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"int yaml_alias_event_initialize(yaml_event_t *, const yaml_char_t *)\0"))
                .as_ptr(),
        );
    }
    if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
        return 0 as libc::c_int;
    }
    anchor_copy = yaml_strdup(anchor);
    if anchor_copy.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_ALIAS_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    let ref mut fresh167 = (*event).data.alias.anchor;
    *fresh167 = anchor_copy;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_scalar_event_initialize(
    mut event: *mut yaml_event_t,
    mut anchor: *const yaml_char_t,
    mut tag: *const yaml_char_t,
    mut value: *const yaml_char_t,
    mut length: libc::c_int,
    mut plain_implicit: libc::c_int,
    mut quoted_implicit: libc::c_int,
    mut style: yaml_scalar_style_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut anchor_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut value_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            832 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"int yaml_scalar_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, const yaml_char_t *, int, int, int, yaml_scalar_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    if !value.is_null() {} else {
        __assert_fail(
            b"value\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            833 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"int yaml_scalar_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, const yaml_char_t *, int, int, int, yaml_scalar_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
            current_block = 16285396129609901221;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 16285396129609901221;
            } else {
                current_block = 8515828400728868193;
            }
        }
    } else {
        current_block = 8515828400728868193;
    }
    match current_block {
        8515828400728868193 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0 {
                    current_block = 16285396129609901221;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 16285396129609901221;
                    } else {
                        current_block = 12800627514080957624;
                    }
                }
            } else {
                current_block = 12800627514080957624;
            }
            match current_block {
                16285396129609901221 => {}
                _ => {
                    if length < 0 as libc::c_int {
                        length = strlen(value as *mut libc::c_char) as libc::c_int;
                    }
                    if !(yaml_check_utf8(value, length as size_t) == 0) {
                        value_copy = yaml_malloc((length + 1 as libc::c_int) as size_t)
                            as *mut yaml_char_t;
                        if !value_copy.is_null() {
                            memcpy(
                                value_copy as *mut libc::c_void,
                                value as *const libc::c_void,
                                length as libc::c_ulong,
                            );
                            *value_copy
                                .offset(length as isize) = '\0' as i32 as yaml_char_t;
                            memset(
                                event as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_0 = YAML_SCALAR_EVENT;
                            (*event).start_mark = mark;
                            (*event).end_mark = mark;
                            let ref mut fresh168 = (*event).data.scalar.anchor;
                            *fresh168 = anchor_copy;
                            let ref mut fresh169 = (*event).data.scalar.tag;
                            *fresh169 = tag_copy;
                            let ref mut fresh170 = (*event).data.scalar.value;
                            *fresh170 = value_copy;
                            (*event).data.scalar.length = length as size_t;
                            (*event).data.scalar.plain_implicit = plain_implicit;
                            (*event).data.scalar.quoted_implicit = quoted_implicit;
                            (*event).data.scalar.style = style;
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    yaml_free(value_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_sequence_start_event_initialize(
    mut event: *mut yaml_event_t,
    mut anchor: *const yaml_char_t,
    mut tag: *const yaml_char_t,
    mut implicit: libc::c_int,
    mut style: yaml_sequence_style_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut anchor_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            883 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 127],
                &[libc::c_char; 127],
            >(
                b"int yaml_sequence_start_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_sequence_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
            current_block = 8817775685815971442;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 8817775685815971442;
            } else {
                current_block = 11006700562992250127;
            }
        }
    } else {
        current_block = 11006700562992250127;
    }
    match current_block {
        11006700562992250127 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0 {
                    current_block = 8817775685815971442;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 8817775685815971442;
                    } else {
                        current_block = 7651349459974463963;
                    }
                }
            } else {
                current_block = 7651349459974463963;
            }
            match current_block {
                8817775685815971442 => {}
                _ => {
                    memset(
                        event as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    (*event).type_0 = YAML_SEQUENCE_START_EVENT;
                    (*event).start_mark = mark;
                    (*event).end_mark = mark;
                    let ref mut fresh171 = (*event).data.sequence_start.anchor;
                    *fresh171 = anchor_copy;
                    let ref mut fresh172 = (*event).data.sequence_start.tag;
                    *fresh172 = tag_copy;
                    (*event).data.sequence_start.implicit = implicit;
                    (*event).data.sequence_start.style = style;
                    return 1 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_sequence_end_event_initialize(
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            918 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int yaml_sequence_end_event_initialize(yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_SEQUENCE_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_mapping_start_event_initialize(
    mut event: *mut yaml_event_t,
    mut anchor: *const yaml_char_t,
    mut tag: *const yaml_char_t,
    mut implicit: libc::c_int,
    mut style: yaml_mapping_style_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut anchor_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            938 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 125],
                &[libc::c_char; 125],
            >(
                b"int yaml_mapping_start_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_mapping_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
            current_block = 14748279734549812740;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 14748279734549812740;
            } else {
                current_block = 11006700562992250127;
            }
        }
    } else {
        current_block = 11006700562992250127;
    }
    match current_block {
        11006700562992250127 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0 {
                    current_block = 14748279734549812740;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 14748279734549812740;
                    } else {
                        current_block = 7651349459974463963;
                    }
                }
            } else {
                current_block = 7651349459974463963;
            }
            match current_block {
                14748279734549812740 => {}
                _ => {
                    memset(
                        event as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    (*event).type_0 = YAML_MAPPING_START_EVENT;
                    (*event).start_mark = mark;
                    (*event).end_mark = mark;
                    let ref mut fresh173 = (*event).data.mapping_start.anchor;
                    *fresh173 = anchor_copy;
                    let ref mut fresh174 = (*event).data.mapping_start.tag;
                    *fresh174 = tag_copy;
                    (*event).data.mapping_start.implicit = implicit;
                    (*event).data.mapping_start.style = style;
                    return 1 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_mapping_end_event_initialize(
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            973 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int yaml_mapping_end_event_initialize(yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_MAPPING_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_event_delete(mut event: *mut yaml_event_t) {
    let mut tag_directive: *mut yaml_tag_directive_t = 0 as *mut yaml_tag_directive_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            989 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void yaml_event_delete(yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    match (*event).type_0 as libc::c_uint {
        3 => {
            yaml_free(
                (*event).data.document_start.version_directive as *mut libc::c_void,
            );
            tag_directive = (*event).data.document_start.tag_directives.start;
            while tag_directive != (*event).data.document_start.tag_directives.end {
                yaml_free((*tag_directive).handle as *mut libc::c_void);
                yaml_free((*tag_directive).prefix as *mut libc::c_void);
                tag_directive = tag_directive.offset(1);
            }
            yaml_free(
                (*event).data.document_start.tag_directives.start as *mut libc::c_void,
            );
        }
        5 => {
            yaml_free((*event).data.alias.anchor as *mut libc::c_void);
        }
        6 => {
            yaml_free((*event).data.scalar.anchor as *mut libc::c_void);
            yaml_free((*event).data.scalar.tag as *mut libc::c_void);
            yaml_free((*event).data.scalar.value as *mut libc::c_void);
        }
        7 => {
            yaml_free((*event).data.sequence_start.anchor as *mut libc::c_void);
            yaml_free((*event).data.sequence_start.tag as *mut libc::c_void);
        }
        9 => {
            yaml_free((*event).data.mapping_start.anchor as *mut libc::c_void);
            yaml_free((*event).data.mapping_start.tag as *mut libc::c_void);
        }
        _ => {}
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_initialize(
    mut document: *mut yaml_document_t,
    mut version_directive: *mut yaml_version_directive_t,
    mut tag_directives_start: *mut yaml_tag_directive_t,
    mut tag_directives_end: *mut yaml_tag_directive_t,
    mut start_implicit: libc::c_int,
    mut end_implicit: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut context: Unnamed_28 = Unnamed_28 {
        error: YAML_NO_ERROR,
    };
    let mut nodes: Unnamed_27 = {
        let mut init = Unnamed_27 {
            start: 0 as *mut yaml_node_t,
            end: 0 as *mut yaml_node_t,
            top: 0 as *mut yaml_node_t,
        };
        init
    };
    let mut version_directive_copy: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut tag_directives_copy: Unnamed_26 = {
        let mut init = Unnamed_26 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        };
        init
    };
    let mut value: yaml_tag_directive_t = {
        let mut init = yaml_tag_directive_s {
            handle: 0 as *mut yaml_char_t,
            prefix: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1059 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 134],
                &[libc::c_char; 134],
            >(
                b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if !tag_directives_start.is_null() && !tag_directives_end.is_null()
        || tag_directives_start == tag_directives_end
    {} else {
        __assert_fail(
            b"(tag_directives_start && tag_directives_end) || (tag_directives_start == tag_directives_end)\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1061 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 134],
                &[libc::c_char; 134],
            >(
                b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    nodes
        .start = yaml_malloc(
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<yaml_node_t>() as libc::c_ulong),
    ) as *mut yaml_node_t;
    if !(if !(nodes.start).is_null() {
        nodes.top = nodes.start;
        nodes.end = (nodes.start).offset(16 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        context.error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        if !version_directive.is_null() {
            version_directive_copy = yaml_malloc(
                ::std::mem::size_of::<yaml_version_directive_t>() as libc::c_ulong,
            ) as *mut yaml_version_directive_t;
            if version_directive_copy.is_null() {
                current_block = 8142820162064489797;
            } else {
                (*version_directive_copy).major = (*version_directive).major;
                (*version_directive_copy).minor = (*version_directive).minor;
                current_block = 7746791466490516765;
            }
        } else {
            current_block = 7746791466490516765;
        }
        match current_block {
            8142820162064489797 => {}
            _ => {
                if tag_directives_start != tag_directives_end {
                    let mut tag_directive: *mut yaml_tag_directive_t = 0
                        as *mut yaml_tag_directive_t;
                    tag_directives_copy
                        .start = yaml_malloc(
                        (16 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<yaml_tag_directive_t>()
                                    as libc::c_ulong,
                            ),
                    ) as *mut yaml_tag_directive_t;
                    if if !(tag_directives_copy.start).is_null() {
                        tag_directives_copy.top = tag_directives_copy.start;
                        tag_directives_copy
                            .end = (tag_directives_copy.start)
                            .offset(16 as libc::c_int as isize);
                        1 as libc::c_int
                    } else {
                        context.error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0
                    {
                        current_block = 8142820162064489797;
                    } else {
                        tag_directive = tag_directives_start;
                        loop {
                            if !(tag_directive != tag_directives_end) {
                                current_block = 14818589718467733107;
                                break;
                            }
                            if !((*tag_directive).handle).is_null() {} else {
                                __assert_fail(
                                    b"tag_directive->handle\0" as *const u8
                                        as *const libc::c_char,
                                    b"api.c\0" as *const u8 as *const libc::c_char,
                                    1079 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 134],
                                        &[libc::c_char; 134],
                                    >(
                                        b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            if !((*tag_directive).prefix).is_null() {} else {
                                __assert_fail(
                                    b"tag_directive->prefix\0" as *const u8
                                        as *const libc::c_char,
                                    b"api.c\0" as *const u8 as *const libc::c_char,
                                    1080 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 134],
                                        &[libc::c_char; 134],
                                    >(
                                        b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            if yaml_check_utf8(
                                (*tag_directive).handle,
                                strlen((*tag_directive).handle as *mut libc::c_char),
                            ) == 0
                            {
                                current_block = 8142820162064489797;
                                break;
                            }
                            if yaml_check_utf8(
                                (*tag_directive).prefix,
                                strlen((*tag_directive).prefix as *mut libc::c_char),
                            ) == 0
                            {
                                current_block = 8142820162064489797;
                                break;
                            }
                            value.handle = yaml_strdup((*tag_directive).handle);
                            value.prefix = yaml_strdup((*tag_directive).prefix);
                            if (value.handle).is_null() || (value.prefix).is_null() {
                                current_block = 8142820162064489797;
                                break;
                            }
                            if if tag_directives_copy.top != tag_directives_copy.end
                                || yaml_stack_extend(
                                    &mut tag_directives_copy.start
                                        as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                    &mut tag_directives_copy.top
                                        as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                    &mut tag_directives_copy.end
                                        as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                ) != 0
                            {
                                let fresh175 = tag_directives_copy.top;
                                tag_directives_copy
                                    .top = (tag_directives_copy.top).offset(1);
                                *fresh175 = value;
                                1 as libc::c_int
                            } else {
                                context.error = YAML_MEMORY_ERROR;
                                0 as libc::c_int
                            } == 0
                            {
                                current_block = 8142820162064489797;
                                break;
                            }
                            value.handle = 0 as *mut yaml_char_t;
                            value.prefix = 0 as *mut yaml_char_t;
                            tag_directive = tag_directive.offset(1);
                        }
                    }
                } else {
                    current_block = 14818589718467733107;
                }
                match current_block {
                    8142820162064489797 => {}
                    _ => {
                        memset(
                            document as *mut libc::c_void,
                            0 as libc::c_int,
                            ::std::mem::size_of::<yaml_document_t>() as libc::c_ulong,
                        );
                        let ref mut fresh176 = (*document).nodes.start;
                        *fresh176 = nodes.start;
                        let ref mut fresh177 = (*document).nodes.end;
                        *fresh177 = nodes.end;
                        let ref mut fresh178 = (*document).nodes.top;
                        *fresh178 = nodes.start;
                        let ref mut fresh179 = (*document).version_directive;
                        *fresh179 = version_directive_copy;
                        let ref mut fresh180 = (*document).tag_directives.start;
                        *fresh180 = tag_directives_copy.start;
                        let ref mut fresh181 = (*document).tag_directives.end;
                        *fresh181 = tag_directives_copy.top;
                        (*document).start_implicit = start_implicit;
                        (*document).end_implicit = end_implicit;
                        (*document).start_mark = mark;
                        (*document).end_mark = mark;
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    yaml_free(nodes.start as *mut libc::c_void);
    nodes.end = 0 as *mut yaml_node_t;
    nodes.top = nodes.end;
    nodes.start = nodes.top;
    yaml_free(version_directive_copy as *mut libc::c_void);
    while !(tag_directives_copy.start == tag_directives_copy.top) {
        tag_directives_copy.top = (tag_directives_copy.top).offset(-1);
        let mut value_0: yaml_tag_directive_t = *tag_directives_copy.top;
        yaml_free(value_0.handle as *mut libc::c_void);
        yaml_free(value_0.prefix as *mut libc::c_void);
    }
    yaml_free(tag_directives_copy.start as *mut libc::c_void);
    tag_directives_copy.end = 0 as *mut yaml_tag_directive_t;
    tag_directives_copy.top = tag_directives_copy.end;
    tag_directives_copy.start = tag_directives_copy.top;
    yaml_free(value.handle as *mut libc::c_void);
    yaml_free(value.prefix as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_delete(mut document: *mut yaml_document_t) {
    let mut tag_directive: *mut yaml_tag_directive_t = 0 as *mut yaml_tag_directive_t;
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void yaml_document_delete(yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    while !((*document).nodes.start == (*document).nodes.top) {
        let ref mut fresh182 = (*document).nodes.top;
        *fresh182 = (*fresh182).offset(-1);
        let mut node: yaml_node_t = **fresh182;
        yaml_free(node.tag as *mut libc::c_void);
        match node.type_0 as libc::c_uint {
            1 => {
                yaml_free(node.data.scalar.value as *mut libc::c_void);
            }
            2 => {
                yaml_free(node.data.sequence.items.start as *mut libc::c_void);
                node.data.sequence.items.end = 0 as *mut yaml_node_item_t;
                node.data.sequence.items.top = node.data.sequence.items.end;
                node.data.sequence.items.start = node.data.sequence.items.top;
            }
            3 => {
                yaml_free(node.data.mapping.pairs.start as *mut libc::c_void);
                node.data.mapping.pairs.end = 0 as *mut yaml_node_pair_t;
                node.data.mapping.pairs.top = node.data.mapping.pairs.end;
                node.data.mapping.pairs.start = node.data.mapping.pairs.top;
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"api.c\0" as *const u8 as *const libc::c_char,
                    1143 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 45],
                        &[libc::c_char; 45],
                    >(b"void yaml_document_delete(yaml_document_t *)\0"))
                        .as_ptr(),
                );
            }
        }
    }
    yaml_free((*document).nodes.start as *mut libc::c_void);
    let ref mut fresh183 = (*document).nodes.end;
    *fresh183 = 0 as *mut yaml_node_t;
    let ref mut fresh184 = (*document).nodes.top;
    *fresh184 = *fresh183;
    let ref mut fresh185 = (*document).nodes.start;
    *fresh185 = *fresh184;
    yaml_free((*document).version_directive as *mut libc::c_void);
    tag_directive = (*document).tag_directives.start;
    while tag_directive != (*document).tag_directives.end {
        yaml_free((*tag_directive).handle as *mut libc::c_void);
        yaml_free((*tag_directive).prefix as *mut libc::c_void);
        tag_directive = tag_directive.offset(1);
    }
    yaml_free((*document).tag_directives.start as *mut libc::c_void);
    memset(
        document as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_document_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_get_node(
    mut document: *mut yaml_document_t,
    mut index: libc::c_int,
) -> *mut yaml_node_t {
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1167 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"yaml_node_t *yaml_document_get_node(yaml_document_t *, int)\0"))
                .as_ptr(),
        );
    }
    if index > 0 as libc::c_int
        && ((*document).nodes.start).offset(index as isize) <= (*document).nodes.top
    {
        return ((*document).nodes.start)
            .offset(index as isize)
            .offset(-(1 as libc::c_int as isize));
    }
    return 0 as *mut yaml_node_t;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_get_root_node(
    mut document: *mut yaml_document_t,
) -> *mut yaml_node_t {
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1182 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"yaml_node_t *yaml_document_get_root_node(yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    if (*document).nodes.top != (*document).nodes.start {
        return (*document).nodes.start;
    }
    return 0 as *mut yaml_node_t;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_add_scalar(
    mut document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    mut value: *const yaml_char_t,
    mut length: libc::c_int,
    mut style: yaml_scalar_style_t,
) -> libc::c_int {
    let mut context: Unnamed_29 = Unnamed_29 {
        error: YAML_NO_ERROR,
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut value_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: 0 as *mut yaml_char_t,
        data: unnamed_yaml_node_s_data {
            scalar: unnamed_yaml_node_s_data_scalar {
                value: 0 as *mut yaml_char_t,
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1207 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 116],
                &[libc::c_char; 116],
            >(
                b"int yaml_document_add_scalar(yaml_document_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_scalar_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    if !value.is_null() {} else {
        __assert_fail(
            b"value\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1208 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 116],
                &[libc::c_char; 116],
            >(
                b"int yaml_document_add_scalar(yaml_document_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_scalar_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char
            as *mut yaml_char_t;
    }
    if !(yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0) {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            if length < 0 as libc::c_int {
                length = strlen(value as *mut libc::c_char) as libc::c_int;
            }
            if !(yaml_check_utf8(value, length as size_t) == 0) {
                value_copy = yaml_malloc((length + 1 as libc::c_int) as size_t)
                    as *mut yaml_char_t;
                if !value_copy.is_null() {
                    memcpy(
                        value_copy as *mut libc::c_void,
                        value as *const libc::c_void,
                        length as libc::c_ulong,
                    );
                    *value_copy.offset(length as isize) = '\0' as i32 as yaml_char_t;
                    memset(
                        &mut node as *mut yaml_node_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_node_t>() as libc::c_ulong,
                    );
                    node.type_0 = YAML_SCALAR_NODE;
                    node.tag = tag_copy;
                    node.start_mark = mark;
                    node.end_mark = mark;
                    node.data.scalar.value = value_copy;
                    node.data.scalar.length = length as size_t;
                    node.data.scalar.style = style;
                    if !(if (*document).nodes.top != (*document).nodes.end
                        || yaml_stack_extend(
                            &mut (*document).nodes.start as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                            &mut (*document).nodes.top as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                            &mut (*document).nodes.end as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let ref mut fresh186 = (*document).nodes.top;
                        let fresh187 = *fresh186;
                        *fresh186 = (*fresh186).offset(1);
                        *fresh187 = node;
                        1 as libc::c_int
                    } else {
                        context.error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        return ((*document).nodes.top)
                            .offset_from((*document).nodes.start) as libc::c_long
                            as libc::c_int;
                    }
                }
            }
        }
    }
    yaml_free(tag_copy as *mut libc::c_void);
    yaml_free(value_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_add_sequence(
    mut document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    mut style: yaml_sequence_style_t,
) -> libc::c_int {
    let mut context: Unnamed_31 = Unnamed_31 {
        error: YAML_NO_ERROR,
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut items: Unnamed_30 = {
        let mut init = Unnamed_30 {
            start: 0 as *mut yaml_node_item_t,
            end: 0 as *mut yaml_node_item_t,
            top: 0 as *mut yaml_node_item_t,
        };
        init
    };
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: 0 as *mut yaml_char_t,
        data: unnamed_yaml_node_s_data {
            scalar: unnamed_yaml_node_s_data_scalar {
                value: 0 as *mut yaml_char_t,
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1260 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"int yaml_document_add_sequence(yaml_document_t *, const yaml_char_t *, yaml_sequence_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:seq\0" as *const u8 as *const libc::c_char
            as *mut yaml_char_t;
    }
    if !(yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0) {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            items
                .start = yaml_malloc(
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<yaml_node_item_t>() as libc::c_ulong,
                    ),
            ) as *mut yaml_node_item_t;
            if !(if !(items.start).is_null() {
                items.top = items.start;
                items.end = (items.start).offset(16 as libc::c_int as isize);
                1 as libc::c_int
            } else {
                context.error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                memset(
                    &mut node as *mut yaml_node_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<yaml_node_t>() as libc::c_ulong,
                );
                node.type_0 = YAML_SEQUENCE_NODE;
                node.tag = tag_copy;
                node.start_mark = mark;
                node.end_mark = mark;
                node.data.sequence.items.start = items.start;
                node.data.sequence.items.end = items.end;
                node.data.sequence.items.top = items.start;
                node.data.sequence.style = style;
                if !(if (*document).nodes.top != (*document).nodes.end
                    || yaml_stack_extend(
                        &mut (*document).nodes.start as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*document).nodes.top as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*document).nodes.end as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let ref mut fresh188 = (*document).nodes.top;
                    let fresh189 = *fresh188;
                    *fresh188 = (*fresh188).offset(1);
                    *fresh189 = node;
                    1 as libc::c_int
                } else {
                    context.error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    return ((*document).nodes.top).offset_from((*document).nodes.start)
                        as libc::c_long as libc::c_int;
                }
            }
        }
    }
    yaml_free(items.start as *mut libc::c_void);
    items.end = 0 as *mut yaml_node_item_t;
    items.top = items.end;
    items.start = items.top;
    yaml_free(tag_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_add_mapping(
    mut document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    mut style: yaml_mapping_style_t,
) -> libc::c_int {
    let mut context: Unnamed_33 = Unnamed_33 {
        error: YAML_NO_ERROR,
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut pairs: Unnamed_32 = {
        let mut init = Unnamed_32 {
            start: 0 as *mut yaml_node_pair_t,
            end: 0 as *mut yaml_node_pair_t,
            top: 0 as *mut yaml_node_pair_t,
        };
        init
    };
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: 0 as *mut yaml_char_t,
        data: unnamed_yaml_node_s_data {
            scalar: unnamed_yaml_node_s_data_scalar {
                value: 0 as *mut yaml_char_t,
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1305 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 92],
                &[libc::c_char; 92],
            >(
                b"int yaml_document_add_mapping(yaml_document_t *, const yaml_char_t *, yaml_mapping_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:map\0" as *const u8 as *const libc::c_char
            as *mut yaml_char_t;
    }
    if !(yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0) {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            pairs
                .start = yaml_malloc(
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<yaml_node_pair_t>() as libc::c_ulong,
                    ),
            ) as *mut yaml_node_pair_t;
            if !(if !(pairs.start).is_null() {
                pairs.top = pairs.start;
                pairs.end = (pairs.start).offset(16 as libc::c_int as isize);
                1 as libc::c_int
            } else {
                context.error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                memset(
                    &mut node as *mut yaml_node_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<yaml_node_t>() as libc::c_ulong,
                );
                node.type_0 = YAML_MAPPING_NODE;
                node.tag = tag_copy;
                node.start_mark = mark;
                node.end_mark = mark;
                node.data.mapping.pairs.start = pairs.start;
                node.data.mapping.pairs.end = pairs.end;
                node.data.mapping.pairs.top = pairs.start;
                node.data.mapping.style = style;
                if !(if (*document).nodes.top != (*document).nodes.end
                    || yaml_stack_extend(
                        &mut (*document).nodes.start as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*document).nodes.top as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*document).nodes.end as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let ref mut fresh190 = (*document).nodes.top;
                    let fresh191 = *fresh190;
                    *fresh190 = (*fresh190).offset(1);
                    *fresh191 = node;
                    1 as libc::c_int
                } else {
                    context.error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    return ((*document).nodes.top).offset_from((*document).nodes.start)
                        as libc::c_long as libc::c_int;
                }
            }
        }
    }
    yaml_free(pairs.start as *mut libc::c_void);
    pairs.end = 0 as *mut yaml_node_pair_t;
    pairs.top = pairs.end;
    pairs.start = pairs.top;
    yaml_free(tag_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_append_sequence_item(
    mut document: *mut yaml_document_t,
    mut sequence: libc::c_int,
    mut item: libc::c_int,
) -> libc::c_int {
    let mut context: Unnamed_34 = Unnamed_34 {
        error: YAML_NO_ERROR,
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1342 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    if sequence > 0 as libc::c_int
        && ((*document).nodes.start).offset(sequence as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"sequence > 0 && document->nodes.start + sequence <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    if (*((*document).nodes.start).offset((sequence - 1 as libc::c_int) as isize)).type_0
        as libc::c_uint == YAML_SEQUENCE_NODE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"document->nodes.start[sequence-1].type == YAML_SEQUENCE_NODE\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1346 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    if item > 0 as libc::c_int
        && ((*document).nodes.start).offset(item as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"item > 0 && document->nodes.start + item <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1348 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    if if (*((*document).nodes.start).offset((sequence - 1 as libc::c_int) as isize))
        .data
        .sequence
        .items
        .top
        != (*((*document).nodes.start).offset((sequence - 1 as libc::c_int) as isize))
            .data
            .sequence
            .items
            .end
        || yaml_stack_extend(
            &mut (*((*document).nodes.start)
                .offset((sequence - 1 as libc::c_int) as isize))
                .data
                .sequence
                .items
                .start as *mut *mut yaml_node_item_t as *mut *mut libc::c_void,
            &mut (*((*document).nodes.start)
                .offset((sequence - 1 as libc::c_int) as isize))
                .data
                .sequence
                .items
                .top as *mut *mut yaml_node_item_t as *mut *mut libc::c_void,
            &mut (*((*document).nodes.start)
                .offset((sequence - 1 as libc::c_int) as isize))
                .data
                .sequence
                .items
                .end as *mut *mut yaml_node_item_t as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh192 = (*((*document).nodes.start)
            .offset((sequence - 1 as libc::c_int) as isize))
            .data
            .sequence
            .items
            .top;
        let fresh193 = *fresh192;
        *fresh192 = (*fresh192).offset(1);
        *fresh193 = item;
        1 as libc::c_int
    } else {
        context.error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_document_append_mapping_pair(
    mut document: *mut yaml_document_t,
    mut mapping: libc::c_int,
    mut key: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut context: Unnamed_35 = Unnamed_35 {
        error: YAML_NO_ERROR,
    };
    let mut pair: yaml_node_pair_t = yaml_node_pair_t {
        key: 0,
        value: 0,
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1372 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if mapping > 0 as libc::c_int
        && ((*document).nodes.start).offset(mapping as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"mapping > 0 && document->nodes.start + mapping <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1374 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if (*((*document).nodes.start).offset((mapping - 1 as libc::c_int) as isize)).type_0
        as libc::c_uint == YAML_MAPPING_NODE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"document->nodes.start[mapping-1].type == YAML_MAPPING_NODE\0" as *const u8
                as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1376 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if key > 0 as libc::c_int
        && ((*document).nodes.start).offset(key as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"key > 0 && document->nodes.start + key <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1378 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if value > 0 as libc::c_int
        && ((*document).nodes.start).offset(value as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"value > 0 && document->nodes.start + value <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1380 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    pair.key = key;
    pair.value = value;
    if if (*((*document).nodes.start).offset((mapping - 1 as libc::c_int) as isize))
        .data
        .mapping
        .pairs
        .top
        != (*((*document).nodes.start).offset((mapping - 1 as libc::c_int) as isize))
            .data
            .mapping
            .pairs
            .end
        || yaml_stack_extend(
            &mut (*((*document).nodes.start)
                .offset((mapping - 1 as libc::c_int) as isize))
                .data
                .mapping
                .pairs
                .start as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
            &mut (*((*document).nodes.start)
                .offset((mapping - 1 as libc::c_int) as isize))
                .data
                .mapping
                .pairs
                .top as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
            &mut (*((*document).nodes.start)
                .offset((mapping - 1 as libc::c_int) as isize))
                .data
                .mapping
                .pairs
                .end as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh194 = (*((*document).nodes.start)
            .offset((mapping - 1 as libc::c_int) as isize))
            .data
            .mapping
            .pairs
            .top;
        let fresh195 = *fresh194;
        *fresh194 = (*fresh194).offset(1);
        *fresh195 = pair;
        1 as libc::c_int
    } else {
        context.error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
