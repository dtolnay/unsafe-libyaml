use crate::externs::*;
use crate::libc;
use crate::yaml::*;
extern "C" {
    fn yaml_document_delete(document: *mut yaml_document_t);
    fn yaml_parser_parse(
        parser: *mut yaml_parser_t,
        event: *mut yaml_event_t,
    ) -> libc::c_int;
    fn yaml_free(ptr: *mut libc::c_void);
    fn yaml_stack_extend(
        start: *mut *mut libc::c_void,
        top: *mut *mut libc::c_void,
        end: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn yaml_malloc(size: size_t) -> *mut libc::c_void;
    fn yaml_strdup(_: *const yaml_char_t) -> *mut yaml_char_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loader_ctx {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_35 {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_36 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_load(
    mut parser: *mut yaml_parser_t,
    mut document: *mut yaml_document_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: unnamed_yaml_event_s_data {
            stream_start: unnamed_yaml_event_s_data_stream_start {
                encoding: YAML_ANY_ENCODING,
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
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"loader.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"loader.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    memset(
        document as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_document_t>() as libc::c_ulong,
    );
    let ref mut fresh0 = (*document).nodes.start;
    *fresh0 = yaml_malloc(
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<yaml_node_t>() as libc::c_ulong),
    ) as *mut yaml_node_t;
    if !(if !(*fresh0).is_null() {
        let ref mut fresh1 = (*document).nodes.top;
        *fresh1 = (*document).nodes.start;
        let ref mut fresh2 = (*document).nodes.end;
        *fresh2 = ((*document).nodes.start).offset(16 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        if (*parser).stream_start_produced == 0 {
            if yaml_parser_parse(parser, &mut event) == 0 {
                current_block = 6234624449317607669;
            } else {
                if event.type_0 as libc::c_uint
                    == YAML_STREAM_START_EVENT as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"event.type == YAML_STREAM_START_EVENT\0" as *const u8
                            as *const libc::c_char,
                        b"loader.c\0" as *const u8 as *const libc::c_char,
                        100 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 57],
                            &[libc::c_char; 57],
                        >(b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0"))
                            .as_ptr(),
                    );
                }
                current_block = 7815301370352969686;
            }
        } else {
            current_block = 7815301370352969686;
        }
        match current_block {
            6234624449317607669 => {}
            _ => {
                if (*parser).stream_end_produced != 0 {
                    return 1 as libc::c_int;
                }
                if !(yaml_parser_parse(parser, &mut event) == 0) {
                    if event.type_0 as libc::c_uint
                        == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint
                    {
                        return 1 as libc::c_int;
                    }
                    let ref mut fresh3 = (*parser).aliases.start;
                    *fresh3 = yaml_malloc(
                        (16 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<yaml_alias_data_t>() as libc::c_ulong,
                            ),
                    ) as *mut yaml_alias_data_t;
                    if !(if !(*fresh3).is_null() {
                        let ref mut fresh4 = (*parser).aliases.top;
                        *fresh4 = (*parser).aliases.start;
                        let ref mut fresh5 = (*parser).aliases.end;
                        *fresh5 = ((*parser).aliases.start)
                            .offset(16 as libc::c_int as isize);
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        let ref mut fresh6 = (*parser).document;
                        *fresh6 = document;
                        if !(yaml_parser_load_document(parser, &mut event) == 0) {
                            yaml_parser_delete_aliases(parser);
                            let ref mut fresh7 = (*parser).document;
                            *fresh7 = 0 as *mut yaml_document_t;
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_parser_delete_aliases(parser);
    yaml_document_delete(document);
    let ref mut fresh8 = (*parser).document;
    *fresh8 = 0 as *mut yaml_document_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_set_composer_error(
    mut parser: *mut yaml_parser_t,
    mut problem: *const libc::c_char,
    mut problem_mark: yaml_mark_t,
) -> libc::c_int {
    (*parser).error = YAML_COMPOSER_ERROR;
    let ref mut fresh9 = (*parser).problem;
    *fresh9 = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_set_composer_error_context(
    mut parser: *mut yaml_parser_t,
    mut context: *const libc::c_char,
    mut context_mark: yaml_mark_t,
    mut problem: *const libc::c_char,
    mut problem_mark: yaml_mark_t,
) -> libc::c_int {
    (*parser).error = YAML_COMPOSER_ERROR;
    let ref mut fresh10 = (*parser).context;
    *fresh10 = context;
    (*parser).context_mark = context_mark;
    let ref mut fresh11 = (*parser).problem;
    *fresh11 = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_delete_aliases(mut parser: *mut yaml_parser_t) {
    while !((*parser).aliases.start == (*parser).aliases.top) {
        let ref mut fresh12 = (*parser).aliases.top;
        *fresh12 = (*fresh12).offset(-1);
        yaml_free((**fresh12).anchor as *mut libc::c_void);
    }
    yaml_free((*parser).aliases.start as *mut libc::c_void);
    let ref mut fresh13 = (*parser).aliases.end;
    *fresh13 = 0 as *mut yaml_alias_data_t;
    let ref mut fresh14 = (*parser).aliases.top;
    *fresh14 = *fresh13;
    let ref mut fresh15 = (*parser).aliases.start;
    *fresh15 = *fresh14;
}
unsafe extern "C" fn yaml_parser_load_document(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut ctx: loader_ctx = {
        let mut init = loader_ctx {
            start: 0 as *mut libc::c_int,
            end: 0 as *mut libc::c_int,
            top: 0 as *mut libc::c_int,
        };
        init
    };
    if (*event).type_0 as libc::c_uint
        == YAML_DOCUMENT_START_EVENT as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"event->type == YAML_DOCUMENT_START_EVENT\0" as *const u8
                as *const libc::c_char,
            b"loader.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"int yaml_parser_load_document(yaml_parser_t *, yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh16 = (*(*parser).document).version_directive;
    *fresh16 = (*event).data.document_start.version_directive;
    let ref mut fresh17 = (*(*parser).document).tag_directives.start;
    *fresh17 = (*event).data.document_start.tag_directives.start;
    let ref mut fresh18 = (*(*parser).document).tag_directives.end;
    *fresh18 = (*event).data.document_start.tag_directives.end;
    (*(*parser).document).start_implicit = (*event).data.document_start.implicit;
    (*(*parser).document).start_mark = (*event).start_mark;
    ctx
        .start = yaml_malloc(
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if if !(ctx.start).is_null() {
        ctx.top = ctx.start;
        ctx.end = (ctx.start).offset(16 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    if yaml_parser_load_nodes(parser, &mut ctx) == 0 {
        yaml_free(ctx.start as *mut libc::c_void);
        ctx.end = 0 as *mut libc::c_int;
        ctx.top = ctx.end;
        ctx.start = ctx.top;
        return 0 as libc::c_int;
    }
    yaml_free(ctx.start as *mut libc::c_void);
    ctx.end = 0 as *mut libc::c_int;
    ctx.top = ctx.end;
    ctx.start = ctx.top;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_load_nodes(
    mut parser: *mut yaml_parser_t,
    mut ctx: *mut loader_ctx,
) -> libc::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: unnamed_yaml_event_s_data {
            stream_start: unnamed_yaml_event_s_data_stream_start {
                encoding: YAML_ANY_ENCODING,
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
    loop {
        if yaml_parser_parse(parser, &mut event) == 0 {
            return 0 as libc::c_int;
        }
        match event.type_0 as libc::c_uint {
            5 => {
                if yaml_parser_load_alias(parser, &mut event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            6 => {
                if yaml_parser_load_scalar(parser, &mut event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            7 => {
                if yaml_parser_load_sequence(parser, &mut event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            8 => {
                if yaml_parser_load_sequence_end(parser, &mut event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            9 => {
                if yaml_parser_load_mapping(parser, &mut event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            10 => {
                if yaml_parser_load_mapping_end(parser, &mut event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            4 => {}
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"loader.c\0" as *const u8 as *const libc::c_char,
                    246 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"int yaml_parser_load_nodes(yaml_parser_t *, struct loader_ctx *)\0",
                    ))
                        .as_ptr(),
                );
                return 0 as libc::c_int;
            }
        }
        if !(event.type_0 as libc::c_uint
            != YAML_DOCUMENT_END_EVENT as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    (*(*parser).document).end_implicit = event.data.document_end.implicit;
    (*(*parser).document).end_mark = event.end_mark;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_register_anchor(
    mut parser: *mut yaml_parser_t,
    mut index: libc::c_int,
    mut anchor: *mut yaml_char_t,
) -> libc::c_int {
    let mut data: yaml_alias_data_t = yaml_alias_data_t {
        anchor: 0 as *mut yaml_char_t,
        index: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    let mut alias_data: *mut yaml_alias_data_t = 0 as *mut yaml_alias_data_t;
    if anchor.is_null() {
        return 1 as libc::c_int;
    }
    data.anchor = anchor;
    data.index = index;
    data
        .mark = (*((*(*parser).document).nodes.start)
        .offset((index - 1 as libc::c_int) as isize))
        .start_mark;
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp((*alias_data).anchor as *mut libc::c_char, anchor as *mut libc::c_char)
            == 0 as libc::c_int
        {
            yaml_free(anchor as *mut libc::c_void);
            return yaml_parser_set_composer_error_context(
                parser,
                b"found duplicate anchor; first occurrence\0" as *const u8
                    as *const libc::c_char,
                (*alias_data).mark,
                b"second occurrence\0" as *const u8 as *const libc::c_char,
                data.mark,
            );
        }
        alias_data = alias_data.offset(1);
    }
    if if (*parser).aliases.top != (*parser).aliases.end
        || yaml_stack_extend(
            &mut (*parser).aliases.start as *mut *mut yaml_alias_data_t
                as *mut *mut libc::c_void,
            &mut (*parser).aliases.top as *mut *mut yaml_alias_data_t
                as *mut *mut libc::c_void,
            &mut (*parser).aliases.end as *mut *mut yaml_alias_data_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh19 = (*parser).aliases.top;
        let fresh20 = *fresh19;
        *fresh19 = (*fresh19).offset(1);
        *fresh20 = data;
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        yaml_free(anchor as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_load_node_add(
    mut parser: *mut yaml_parser_t,
    mut ctx: *mut loader_ctx,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut parent: *mut yaml_node_s = 0 as *mut yaml_node_s;
    let mut parent_index: libc::c_int = 0;
    if (*ctx).start == (*ctx).top {
        return 1 as libc::c_int;
    }
    parent_index = *((*ctx).top).offset(-(1 as libc::c_int as isize));
    parent = &mut *((*(*parser).document).nodes.start)
        .offset((parent_index - 1 as libc::c_int) as isize) as *mut yaml_node_t;
    let mut current_block_17: u64;
    match (*parent).type_0 as libc::c_uint {
        2 => {
            if if (((*parent).data.sequence.items.top)
                .offset_from((*parent).data.sequence.items.start) as libc::c_long)
                < (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_long
            {
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
            if if (*parent).data.sequence.items.top != (*parent).data.sequence.items.end
                || yaml_stack_extend(
                    &mut (*parent).data.sequence.items.start
                        as *mut *mut yaml_node_item_t as *mut *mut libc::c_void,
                    &mut (*parent).data.sequence.items.top as *mut *mut yaml_node_item_t
                        as *mut *mut libc::c_void,
                    &mut (*parent).data.sequence.items.end as *mut *mut yaml_node_item_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let ref mut fresh21 = (*parent).data.sequence.items.top;
                let fresh22 = *fresh21;
                *fresh21 = (*fresh21).offset(1);
                *fresh22 = index;
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0
            {
                return 0 as libc::c_int;
            }
        }
        3 => {
            let mut pair: yaml_node_pair_t = yaml_node_pair_t {
                key: 0,
                value: 0,
            };
            if !((*parent).data.mapping.pairs.start == (*parent).data.mapping.pairs.top)
            {
                let mut p: *mut yaml_node_pair_t = ((*parent).data.mapping.pairs.top)
                    .offset(-(1 as libc::c_int as isize));
                if (*p).key != 0 as libc::c_int && (*p).value == 0 as libc::c_int {
                    (*p).value = index;
                    current_block_17 = 11307063007268554308;
                } else {
                    current_block_17 = 17407779659766490442;
                }
            } else {
                current_block_17 = 17407779659766490442;
            }
            match current_block_17 {
                11307063007268554308 => {}
                _ => {
                    pair.key = index;
                    pair.value = 0 as libc::c_int;
                    if if (((*parent).data.mapping.pairs.top)
                        .offset_from((*parent).data.mapping.pairs.start) as libc::c_long)
                        < (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_long
                    {
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0
                    {
                        return 0 as libc::c_int;
                    }
                    if if (*parent).data.mapping.pairs.top
                        != (*parent).data.mapping.pairs.end
                        || yaml_stack_extend(
                            &mut (*parent).data.mapping.pairs.start
                                as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
                            &mut (*parent).data.mapping.pairs.top
                                as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
                            &mut (*parent).data.mapping.pairs.end
                                as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let ref mut fresh23 = (*parent).data.mapping.pairs.top;
                        let fresh24 = *fresh23;
                        *fresh23 = (*fresh23).offset(1);
                        *fresh24 = pair;
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0
                    {
                        return 0 as libc::c_int;
                    }
                }
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"loader.c\0" as *const u8 as *const libc::c_char,
                340 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"int yaml_parser_load_node_add(yaml_parser_t *, struct loader_ctx *, int)\0",
                ))
                    .as_ptr(),
            );
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_load_alias(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> libc::c_int {
    let mut anchor: *mut yaml_char_t = (*event).data.alias.anchor;
    let mut alias_data: *mut yaml_alias_data_t = 0 as *mut yaml_alias_data_t;
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp((*alias_data).anchor as *mut libc::c_char, anchor as *mut libc::c_char)
            == 0 as libc::c_int
        {
            yaml_free(anchor as *mut libc::c_void);
            return yaml_parser_load_node_add(parser, ctx, (*alias_data).index);
        }
        alias_data = alias_data.offset(1);
    }
    yaml_free(anchor as *mut libc::c_void);
    return yaml_parser_set_composer_error(
        parser,
        b"found undefined alias\0" as *const u8 as *const libc::c_char,
        (*event).start_mark,
    );
}
unsafe extern "C" fn yaml_parser_load_scalar(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> libc::c_int {
    let mut current_block: u64;
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
    let mut index: libc::c_int = 0;
    let mut tag: *mut yaml_char_t = (*event).data.scalar.tag;
    if !(if (((*(*parser).document).nodes.top)
        .offset_from((*(*parser).document).nodes.start) as libc::c_long)
        < (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_long
    {
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        if tag.is_null()
            || strcmp(
                tag as *mut libc::c_char,
                b"!\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            yaml_free(tag as *mut libc::c_void);
            tag = yaml_strdup(
                b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char
                    as *mut yaml_char_t,
            );
            if tag.is_null() {
                current_block = 10579931339944277179;
            } else {
                current_block = 11006700562992250127;
            }
        } else {
            current_block = 11006700562992250127;
        }
        match current_block {
            10579931339944277179 => {}
            _ => {
                memset(
                    &mut node as *mut yaml_node_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<yaml_node_t>() as libc::c_ulong,
                );
                node.type_0 = YAML_SCALAR_NODE;
                node.tag = tag;
                node.start_mark = (*event).start_mark;
                node.end_mark = (*event).end_mark;
                node.data.scalar.value = (*event).data.scalar.value;
                node.data.scalar.length = (*event).data.scalar.length;
                node.data.scalar.style = (*event).data.scalar.style;
                if !(if (*(*parser).document).nodes.top
                    != (*(*parser).document).nodes.end
                    || yaml_stack_extend(
                        &mut (*(*parser).document).nodes.start as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*(*parser).document).nodes.top as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*(*parser).document).nodes.end as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let ref mut fresh25 = (*(*parser).document).nodes.top;
                    let fresh26 = *fresh25;
                    *fresh25 = (*fresh25).offset(1);
                    *fresh26 = node;
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    index = ((*(*parser).document).nodes.top)
                        .offset_from((*(*parser).document).nodes.start) as libc::c_long
                        as libc::c_int;
                    if yaml_parser_register_anchor(
                        parser,
                        index,
                        (*event).data.scalar.anchor,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                    return yaml_parser_load_node_add(parser, ctx, index);
                }
            }
        }
    }
    yaml_free(tag as *mut libc::c_void);
    yaml_free((*event).data.scalar.anchor as *mut libc::c_void);
    yaml_free((*event).data.scalar.value as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_load_sequence(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> libc::c_int {
    let mut current_block: u64;
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
    let mut items: Unnamed_36 = {
        let mut init = Unnamed_36 {
            start: 0 as *mut yaml_node_item_t,
            end: 0 as *mut yaml_node_item_t,
            top: 0 as *mut yaml_node_item_t,
        };
        init
    };
    let mut index: libc::c_int = 0;
    let mut tag: *mut yaml_char_t = (*event).data.sequence_start.tag;
    if !(if (((*(*parser).document).nodes.top)
        .offset_from((*(*parser).document).nodes.start) as libc::c_long)
        < (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_long
    {
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        if tag.is_null()
            || strcmp(
                tag as *mut libc::c_char,
                b"!\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            yaml_free(tag as *mut libc::c_void);
            tag = yaml_strdup(
                b"tag:yaml.org,2002:seq\0" as *const u8 as *const libc::c_char
                    as *mut yaml_char_t,
            );
            if tag.is_null() {
                current_block = 13474536459355229096;
            } else {
                current_block = 6937071982253665452;
            }
        } else {
            current_block = 6937071982253665452;
        }
        match current_block {
            13474536459355229096 => {}
            _ => {
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
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    memset(
                        &mut node as *mut yaml_node_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_node_t>() as libc::c_ulong,
                    );
                    node.type_0 = YAML_SEQUENCE_NODE;
                    node.tag = tag;
                    node.start_mark = (*event).start_mark;
                    node.end_mark = (*event).end_mark;
                    node.data.sequence.items.start = items.start;
                    node.data.sequence.items.end = items.end;
                    node.data.sequence.items.top = items.start;
                    node.data.sequence.style = (*event).data.sequence_start.style;
                    if !(if (*(*parser).document).nodes.top
                        != (*(*parser).document).nodes.end
                        || yaml_stack_extend(
                            &mut (*(*parser).document).nodes.start
                                as *mut *mut yaml_node_t as *mut *mut libc::c_void,
                            &mut (*(*parser).document).nodes.top as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                            &mut (*(*parser).document).nodes.end as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let ref mut fresh27 = (*(*parser).document).nodes.top;
                        let fresh28 = *fresh27;
                        *fresh27 = (*fresh27).offset(1);
                        *fresh28 = node;
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        index = ((*(*parser).document).nodes.top)
                            .offset_from((*(*parser).document).nodes.start)
                            as libc::c_long as libc::c_int;
                        if yaml_parser_register_anchor(
                            parser,
                            index,
                            (*event).data.sequence_start.anchor,
                        ) == 0
                        {
                            return 0 as libc::c_int;
                        }
                        if yaml_parser_load_node_add(parser, ctx, index) == 0 {
                            return 0 as libc::c_int;
                        }
                        if if (((*ctx).top).offset_from((*ctx).start) as libc::c_long)
                            < (2147483647 as libc::c_int - 1 as libc::c_int)
                                as libc::c_long
                        {
                            1 as libc::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0
                        {
                            return 0 as libc::c_int;
                        }
                        if if (*ctx).top != (*ctx).end
                            || yaml_stack_extend(
                                &mut (*ctx).start as *mut *mut libc::c_int
                                    as *mut *mut libc::c_void,
                                &mut (*ctx).top as *mut *mut libc::c_int
                                    as *mut *mut libc::c_void,
                                &mut (*ctx).end as *mut *mut libc::c_int
                                    as *mut *mut libc::c_void,
                            ) != 0
                        {
                            let ref mut fresh29 = (*ctx).top;
                            let fresh30 = *fresh29;
                            *fresh29 = (*fresh29).offset(1);
                            *fresh30 = index;
                            1 as libc::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0
                        {
                            return 0 as libc::c_int;
                        }
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    yaml_free(tag as *mut libc::c_void);
    yaml_free((*event).data.sequence_start.anchor as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_load_sequence_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> libc::c_int {
    let mut index: libc::c_int = 0;
    if ((*ctx).top).offset_from((*ctx).start) as libc::c_long
        > 0 as libc::c_int as libc::c_long
    {} else {
        __assert_fail(
            b"((*ctx).top - (*ctx).start) > 0\0" as *const u8 as *const libc::c_char,
            b"loader.c\0" as *const u8 as *const libc::c_char,
            467 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"int yaml_parser_load_sequence_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
            ))
                .as_ptr(),
        );
    }
    index = *((*ctx).top).offset(-(1 as libc::c_int as isize));
    if (*((*(*parser).document).nodes.start).offset((index - 1 as libc::c_int) as isize))
        .type_0 as libc::c_uint == YAML_SEQUENCE_NODE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"parser->document->nodes.start[index-1].type == YAML_SEQUENCE_NODE\0"
                as *const u8 as *const libc::c_char,
            b"loader.c\0" as *const u8 as *const libc::c_char,
            470 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"int yaml_parser_load_sequence_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
            ))
                .as_ptr(),
        );
    }
    (*((*(*parser).document).nodes.start).offset((index - 1 as libc::c_int) as isize))
        .end_mark = (*event).end_mark;
    let ref mut fresh31 = (*ctx).top;
    *fresh31 = (*fresh31).offset(-1);
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_load_mapping(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> libc::c_int {
    let mut current_block: u64;
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
    let mut pairs: Unnamed_35 = {
        let mut init = Unnamed_35 {
            start: 0 as *mut yaml_node_pair_t,
            end: 0 as *mut yaml_node_pair_t,
            top: 0 as *mut yaml_node_pair_t,
        };
        init
    };
    let mut index: libc::c_int = 0;
    let mut tag: *mut yaml_char_t = (*event).data.mapping_start.tag;
    if !(if (((*(*parser).document).nodes.top)
        .offset_from((*(*parser).document).nodes.start) as libc::c_long)
        < (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_long
    {
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        if tag.is_null()
            || strcmp(
                tag as *mut libc::c_char,
                b"!\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            yaml_free(tag as *mut libc::c_void);
            tag = yaml_strdup(
                b"tag:yaml.org,2002:map\0" as *const u8 as *const libc::c_char
                    as *mut yaml_char_t,
            );
            if tag.is_null() {
                current_block = 13635467803606088781;
            } else {
                current_block = 6937071982253665452;
            }
        } else {
            current_block = 6937071982253665452;
        }
        match current_block {
            13635467803606088781 => {}
            _ => {
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
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    memset(
                        &mut node as *mut yaml_node_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_node_t>() as libc::c_ulong,
                    );
                    node.type_0 = YAML_MAPPING_NODE;
                    node.tag = tag;
                    node.start_mark = (*event).start_mark;
                    node.end_mark = (*event).end_mark;
                    node.data.mapping.pairs.start = pairs.start;
                    node.data.mapping.pairs.end = pairs.end;
                    node.data.mapping.pairs.top = pairs.start;
                    node.data.mapping.style = (*event).data.mapping_start.style;
                    if !(if (*(*parser).document).nodes.top
                        != (*(*parser).document).nodes.end
                        || yaml_stack_extend(
                            &mut (*(*parser).document).nodes.start
                                as *mut *mut yaml_node_t as *mut *mut libc::c_void,
                            &mut (*(*parser).document).nodes.top as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                            &mut (*(*parser).document).nodes.end as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let ref mut fresh32 = (*(*parser).document).nodes.top;
                        let fresh33 = *fresh32;
                        *fresh32 = (*fresh32).offset(1);
                        *fresh33 = node;
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        index = ((*(*parser).document).nodes.top)
                            .offset_from((*(*parser).document).nodes.start)
                            as libc::c_long as libc::c_int;
                        if yaml_parser_register_anchor(
                            parser,
                            index,
                            (*event).data.mapping_start.anchor,
                        ) == 0
                        {
                            return 0 as libc::c_int;
                        }
                        if yaml_parser_load_node_add(parser, ctx, index) == 0 {
                            return 0 as libc::c_int;
                        }
                        if if (((*ctx).top).offset_from((*ctx).start) as libc::c_long)
                            < (2147483647 as libc::c_int - 1 as libc::c_int)
                                as libc::c_long
                        {
                            1 as libc::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0
                        {
                            return 0 as libc::c_int;
                        }
                        if if (*ctx).top != (*ctx).end
                            || yaml_stack_extend(
                                &mut (*ctx).start as *mut *mut libc::c_int
                                    as *mut *mut libc::c_void,
                                &mut (*ctx).top as *mut *mut libc::c_int
                                    as *mut *mut libc::c_void,
                                &mut (*ctx).end as *mut *mut libc::c_int
                                    as *mut *mut libc::c_void,
                            ) != 0
                        {
                            let ref mut fresh34 = (*ctx).top;
                            let fresh35 = *fresh34;
                            *fresh34 = (*fresh34).offset(1);
                            *fresh35 = index;
                            1 as libc::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0
                        {
                            return 0 as libc::c_int;
                        }
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    yaml_free(tag as *mut libc::c_void);
    yaml_free((*event).data.mapping_start.anchor as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_parser_load_mapping_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> libc::c_int {
    let mut index: libc::c_int = 0;
    if ((*ctx).top).offset_from((*ctx).start) as libc::c_long
        > 0 as libc::c_int as libc::c_long
    {} else {
        __assert_fail(
            b"((*ctx).top - (*ctx).start) > 0\0" as *const u8 as *const libc::c_char,
            b"loader.c\0" as *const u8 as *const libc::c_char,
            535 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"int yaml_parser_load_mapping_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
            ))
                .as_ptr(),
        );
    }
    index = *((*ctx).top).offset(-(1 as libc::c_int as isize));
    if (*((*(*parser).document).nodes.start).offset((index - 1 as libc::c_int) as isize))
        .type_0 as libc::c_uint == YAML_MAPPING_NODE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"parser->document->nodes.start[index-1].type == YAML_MAPPING_NODE\0"
                as *const u8 as *const libc::c_char,
            b"loader.c\0" as *const u8 as *const libc::c_char,
            538 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"int yaml_parser_load_mapping_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
            ))
                .as_ptr(),
        );
    }
    (*((*(*parser).document).nodes.start).offset((index - 1 as libc::c_int) as isize))
        .end_mark = (*event).end_mark;
    let ref mut fresh36 = (*ctx).top;
    *fresh36 = (*fresh36).offset(-1);
    return 1 as libc::c_int;
}
