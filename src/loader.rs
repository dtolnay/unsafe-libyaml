use crate::externs::{memset, strcmp};
use crate::yaml::{unnamed_yaml_node_s_data, unnamed_yaml_node_s_data_scalar};
use crate::{
    libc, yaml_alias_data_t, yaml_char_t, yaml_document_delete, yaml_document_t, yaml_event_t,
    yaml_free, yaml_malloc, yaml_mark_t, yaml_node_item_t, yaml_node_pair_t, yaml_node_t,
    yaml_parser_parse, yaml_parser_t, yaml_stack_extend, yaml_strdup, PointerExt,
    YAML_ANY_SCALAR_STYLE, YAML_COMPOSER_ERROR, YAML_DOCUMENT_END_EVENT, YAML_DOCUMENT_START_EVENT,
    YAML_MAPPING_NODE, YAML_MEMORY_ERROR, YAML_NO_NODE, YAML_SCALAR_NODE, YAML_SEQUENCE_NODE,
    YAML_STREAM_END_EVENT, YAML_STREAM_START_EVENT,
};
use std::mem::{size_of, MaybeUninit};
use std::ptr::{self, addr_of_mut};
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct loader_ctx {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct Unnamed_35 {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct Unnamed_36 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub unsafe fn yaml_parser_load(
    mut parser: *mut yaml_parser_t,
    document: *mut yaml_document_t,
) -> libc::c_int {
    let current_block: u64;
    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    __assert!(!parser.is_null());
    __assert!(!document.is_null());
    memset(
        document as *mut libc::c_void,
        0 as libc::c_int,
        size_of::<yaml_document_t>() as libc::c_ulong,
    );
    let fresh0 = addr_of_mut!((*document).nodes.start);
    *fresh0 = yaml_malloc(
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(size_of::<yaml_node_t>() as libc::c_ulong),
    ) as *mut yaml_node_t;
    if !(if !(*fresh0).is_null() {
        let fresh1 = addr_of_mut!((*document).nodes.top);
        *fresh1 = (*document).nodes.start;
        let fresh2 = addr_of_mut!((*document).nodes.end);
        *fresh2 = ((*document).nodes.start).wrapping_offset(16 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        if (*parser).stream_start_produced == 0 {
            if yaml_parser_parse(parser, event) == 0 {
                current_block = 6234624449317607669;
            } else {
                __assert!(
                    (*event).type_0 as libc::c_uint
                        == YAML_STREAM_START_EVENT as libc::c_int as libc::c_uint
                );
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
                if !(yaml_parser_parse(parser, event) == 0) {
                    if (*event).type_0 as libc::c_uint
                        == YAML_STREAM_END_EVENT as libc::c_int as libc::c_uint
                    {
                        return 1 as libc::c_int;
                    }
                    let fresh3 = addr_of_mut!((*parser).aliases.start);
                    *fresh3 =
                        yaml_malloc((16 as libc::c_int as libc::c_ulong).wrapping_mul(size_of::<
                            yaml_alias_data_t,
                        >(
                        )
                            as libc::c_ulong)) as *mut yaml_alias_data_t;
                    if !(if !(*fresh3).is_null() {
                        let fresh4 = addr_of_mut!((*parser).aliases.top);
                        *fresh4 = (*parser).aliases.start;
                        let fresh5 = addr_of_mut!((*parser).aliases.end);
                        *fresh5 =
                            ((*parser).aliases.start).wrapping_offset(16 as libc::c_int as isize);
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        let fresh6 = addr_of_mut!((*parser).document);
                        *fresh6 = document;
                        if !(yaml_parser_load_document(parser, event) == 0) {
                            yaml_parser_delete_aliases(parser);
                            let fresh7 = addr_of_mut!((*parser).document);
                            *fresh7 = ptr::null_mut::<yaml_document_t>();
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_parser_delete_aliases(parser);
    yaml_document_delete(document);
    let fresh8 = addr_of_mut!((*parser).document);
    *fresh8 = ptr::null_mut::<yaml_document_t>();
    0 as libc::c_int
}
unsafe fn yaml_parser_set_composer_error(
    mut parser: *mut yaml_parser_t,
    problem: *const libc::c_char,
    problem_mark: yaml_mark_t,
) -> libc::c_int {
    (*parser).error = YAML_COMPOSER_ERROR;
    let fresh9 = addr_of_mut!((*parser).problem);
    *fresh9 = problem;
    (*parser).problem_mark = problem_mark;
    0 as libc::c_int
}
unsafe fn yaml_parser_set_composer_error_context(
    mut parser: *mut yaml_parser_t,
    context: *const libc::c_char,
    context_mark: yaml_mark_t,
    problem: *const libc::c_char,
    problem_mark: yaml_mark_t,
) -> libc::c_int {
    (*parser).error = YAML_COMPOSER_ERROR;
    let fresh10 = addr_of_mut!((*parser).context);
    *fresh10 = context;
    (*parser).context_mark = context_mark;
    let fresh11 = addr_of_mut!((*parser).problem);
    *fresh11 = problem;
    (*parser).problem_mark = problem_mark;
    0 as libc::c_int
}
unsafe fn yaml_parser_delete_aliases(parser: *mut yaml_parser_t) {
    while !((*parser).aliases.start == (*parser).aliases.top) {
        let fresh12 = addr_of_mut!((*parser).aliases.top);
        *fresh12 = (*fresh12).wrapping_offset(-1);
        yaml_free((**fresh12).anchor as *mut libc::c_void);
    }
    yaml_free((*parser).aliases.start as *mut libc::c_void);
    let fresh13 = addr_of_mut!((*parser).aliases.end);
    *fresh13 = ptr::null_mut::<yaml_alias_data_t>();
    let fresh14 = addr_of_mut!((*parser).aliases.top);
    *fresh14 = *fresh13;
    let fresh15 = addr_of_mut!((*parser).aliases.start);
    *fresh15 = *fresh14;
}
unsafe fn yaml_parser_load_document(
    mut parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> libc::c_int {
    let mut ctx = loader_ctx {
        start: ptr::null_mut::<libc::c_int>(),
        end: ptr::null_mut::<libc::c_int>(),
        top: ptr::null_mut::<libc::c_int>(),
    };
    __assert!(
        (*event).type_0 as libc::c_uint == YAML_DOCUMENT_START_EVENT as libc::c_int as libc::c_uint
    );
    let fresh16 = addr_of_mut!((*(*parser).document).version_directive);
    *fresh16 = (*event).data.document_start.version_directive;
    let fresh17 = addr_of_mut!((*(*parser).document).tag_directives.start);
    *fresh17 = (*event).data.document_start.tag_directives.start;
    let fresh18 = addr_of_mut!((*(*parser).document).tag_directives.end);
    *fresh18 = (*event).data.document_start.tag_directives.end;
    (*(*parser).document).start_implicit = (*event).data.document_start.implicit;
    (*(*parser).document).start_mark = (*event).start_mark;
    ctx.start = yaml_malloc(
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if if !(ctx.start).is_null() {
        ctx.top = ctx.start;
        ctx.end = (ctx.start).wrapping_offset(16 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    if yaml_parser_load_nodes(parser, addr_of_mut!(ctx)) == 0 {
        yaml_free(ctx.start as *mut libc::c_void);
        ctx.end = ptr::null_mut::<libc::c_int>();
        ctx.top = ctx.end;
        ctx.start = ctx.top;
        return 0 as libc::c_int;
    }
    yaml_free(ctx.start as *mut libc::c_void);
    ctx.end = ptr::null_mut::<libc::c_int>();
    ctx.top = ctx.end;
    ctx.start = ctx.top;
    1 as libc::c_int
}
unsafe fn yaml_parser_load_nodes(
    mut parser: *mut yaml_parser_t,
    ctx: *mut loader_ctx,
) -> libc::c_int {
    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    loop {
        if yaml_parser_parse(parser, event) == 0 {
            return 0 as libc::c_int;
        }
        match (*event).type_0 as libc::c_uint {
            5 => {
                if yaml_parser_load_alias(parser, event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            6 => {
                if yaml_parser_load_scalar(parser, event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            7 => {
                if yaml_parser_load_sequence(parser, event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            8 => {
                if yaml_parser_load_sequence_end(parser, event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            9 => {
                if yaml_parser_load_mapping(parser, event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            10 => {
                if yaml_parser_load_mapping_end(parser, event, ctx) == 0 {
                    return 0 as libc::c_int;
                }
            }
            4 => {}
            _ => {
                __assert!(false);
            }
        }
        if !((*event).type_0 as libc::c_uint
            != YAML_DOCUMENT_END_EVENT as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    (*(*parser).document).end_implicit = (*event).data.document_end.implicit;
    (*(*parser).document).end_mark = (*event).end_mark;
    1 as libc::c_int
}
unsafe fn yaml_parser_register_anchor(
    mut parser: *mut yaml_parser_t,
    index: libc::c_int,
    anchor: *mut yaml_char_t,
) -> libc::c_int {
    let mut data = yaml_alias_data_t {
        anchor: ptr::null_mut::<yaml_char_t>(),
        index: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    let mut alias_data: *mut yaml_alias_data_t;
    if anchor.is_null() {
        return 1 as libc::c_int;
    }
    data.anchor = anchor;
    data.index = index;
    data.mark = (*((*(*parser).document).nodes.start)
        .wrapping_offset((index - 1 as libc::c_int) as isize))
    .start_mark;
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp(
            (*alias_data).anchor as *mut libc::c_char,
            anchor as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            yaml_free(anchor as *mut libc::c_void);
            return yaml_parser_set_composer_error_context(
                parser,
                b"found duplicate anchor; first occurrence\0" as *const u8 as *const libc::c_char,
                (*alias_data).mark,
                b"second occurrence\0" as *const u8 as *const libc::c_char,
                data.mark,
            );
        }
        alias_data = alias_data.wrapping_offset(1);
    }
    if if (*parser).aliases.top != (*parser).aliases.end
        || yaml_stack_extend(
            addr_of_mut!((*parser).aliases.start) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).aliases.top) as *mut *mut libc::c_void,
            addr_of_mut!((*parser).aliases.end) as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh19 = addr_of_mut!((*parser).aliases.top);
        let fresh20 = *fresh19;
        *fresh19 = (*fresh19).wrapping_offset(1);
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
    1 as libc::c_int
}
unsafe fn yaml_parser_load_node_add(
    mut parser: *mut yaml_parser_t,
    ctx: *mut loader_ctx,
    index: libc::c_int,
) -> libc::c_int {
    if (*ctx).start == (*ctx).top {
        return 1 as libc::c_int;
    }
    let parent_index: libc::c_int = *((*ctx).top).wrapping_offset(-(1 as libc::c_int as isize));
    let parent: *mut yaml_node_t = addr_of_mut!(*((*(*parser).document).nodes.start)
        .wrapping_offset((parent_index - 1 as libc::c_int) as isize));
    let current_block_17: u64;
    match (*parent).type_0 as libc::c_uint {
        2 => {
            if if (((*parent).data.sequence.items.top)
                .c_offset_from((*parent).data.sequence.items.start)
                as libc::c_long)
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
                    addr_of_mut!((*parent).data.sequence.items.start) as *mut *mut libc::c_void,
                    addr_of_mut!((*parent).data.sequence.items.top) as *mut *mut libc::c_void,
                    addr_of_mut!((*parent).data.sequence.items.end) as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh21 = addr_of_mut!((*parent).data.sequence.items.top);
                let fresh22 = *fresh21;
                *fresh21 = (*fresh21).wrapping_offset(1);
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
            let mut pair = yaml_node_pair_t { key: 0, value: 0 };
            if !((*parent).data.mapping.pairs.start == (*parent).data.mapping.pairs.top) {
                let mut p: *mut yaml_node_pair_t = ((*parent).data.mapping.pairs.top)
                    .wrapping_offset(-(1 as libc::c_int as isize));
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
                        .c_offset_from((*parent).data.mapping.pairs.start)
                        as libc::c_long)
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
                    if if (*parent).data.mapping.pairs.top != (*parent).data.mapping.pairs.end
                        || yaml_stack_extend(
                            addr_of_mut!((*parent).data.mapping.pairs.start)
                                as *mut *mut libc::c_void,
                            addr_of_mut!((*parent).data.mapping.pairs.top)
                                as *mut *mut libc::c_void,
                            addr_of_mut!((*parent).data.mapping.pairs.end)
                                as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh23 = addr_of_mut!((*parent).data.mapping.pairs.top);
                        let fresh24 = *fresh23;
                        *fresh23 = (*fresh23).wrapping_offset(1);
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
            __assert!(false);
        }
    }
    1 as libc::c_int
}
unsafe fn yaml_parser_load_alias(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> libc::c_int {
    let anchor: *mut yaml_char_t = (*event).data.alias.anchor;
    let mut alias_data: *mut yaml_alias_data_t;
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp(
            (*alias_data).anchor as *mut libc::c_char,
            anchor as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            yaml_free(anchor as *mut libc::c_void);
            return yaml_parser_load_node_add(parser, ctx, (*alias_data).index);
        }
        alias_data = alias_data.wrapping_offset(1);
    }
    yaml_free(anchor as *mut libc::c_void);
    yaml_parser_set_composer_error(
        parser,
        b"found undefined alias\0" as *const u8 as *const libc::c_char,
        (*event).start_mark,
    )
}
unsafe fn yaml_parser_load_scalar(
    mut parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> libc::c_int {
    let current_block: u64;
    let mut node = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: ptr::null_mut::<yaml_char_t>(),
        data: unnamed_yaml_node_s_data {
            scalar: unnamed_yaml_node_s_data_scalar {
                value: ptr::null_mut::<yaml_char_t>(),
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
    let index: libc::c_int;
    let mut tag: *mut yaml_char_t = (*event).data.scalar.tag;
    if !(if (((*(*parser).document).nodes.top).c_offset_from((*(*parser).document).nodes.start)
        as libc::c_long)
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
                b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
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
                    addr_of_mut!(node) as *mut libc::c_void,
                    0 as libc::c_int,
                    size_of::<yaml_node_t>() as libc::c_ulong,
                );
                node.type_0 = YAML_SCALAR_NODE;
                node.tag = tag;
                node.start_mark = (*event).start_mark;
                node.end_mark = (*event).end_mark;
                node.data.scalar.value = (*event).data.scalar.value;
                node.data.scalar.length = (*event).data.scalar.length;
                node.data.scalar.style = (*event).data.scalar.style;
                if !(if (*(*parser).document).nodes.top != (*(*parser).document).nodes.end
                    || yaml_stack_extend(
                        addr_of_mut!((*(*parser).document).nodes.start) as *mut *mut libc::c_void,
                        addr_of_mut!((*(*parser).document).nodes.top) as *mut *mut libc::c_void,
                        addr_of_mut!((*(*parser).document).nodes.end) as *mut *mut libc::c_void,
                    ) != 0
                {
                    let fresh25 = addr_of_mut!((*(*parser).document).nodes.top);
                    let fresh26 = *fresh25;
                    *fresh25 = (*fresh25).wrapping_offset(1);
                    *fresh26 = node;
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    index = ((*(*parser).document).nodes.top)
                        .c_offset_from((*(*parser).document).nodes.start)
                        as libc::c_long as libc::c_int;
                    if yaml_parser_register_anchor(parser, index, (*event).data.scalar.anchor) == 0
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
    0 as libc::c_int
}
unsafe fn yaml_parser_load_sequence(
    mut parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> libc::c_int {
    let current_block: u64;
    let mut node = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: ptr::null_mut::<yaml_char_t>(),
        data: unnamed_yaml_node_s_data {
            scalar: unnamed_yaml_node_s_data_scalar {
                value: ptr::null_mut::<yaml_char_t>(),
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
    let mut items: Unnamed_36 = Unnamed_36 {
        start: ptr::null_mut::<yaml_node_item_t>(),
        end: ptr::null_mut::<yaml_node_item_t>(),
        top: ptr::null_mut::<yaml_node_item_t>(),
    };
    let index: libc::c_int;
    let mut tag: *mut yaml_char_t = (*event).data.sequence_start.tag;
    if !(if (((*(*parser).document).nodes.top).c_offset_from((*(*parser).document).nodes.start)
        as libc::c_long)
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
                b"tag:yaml.org,2002:seq\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
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
                items.start =
                    yaml_malloc((16 as libc::c_int as libc::c_ulong).wrapping_mul(size_of::<
                        yaml_node_item_t,
                    >(
                    )
                        as libc::c_ulong)) as *mut yaml_node_item_t;
                if !(if !(items.start).is_null() {
                    items.top = items.start;
                    items.end = (items.start).wrapping_offset(16 as libc::c_int as isize);
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    memset(
                        addr_of_mut!(node) as *mut libc::c_void,
                        0 as libc::c_int,
                        size_of::<yaml_node_t>() as libc::c_ulong,
                    );
                    node.type_0 = YAML_SEQUENCE_NODE;
                    node.tag = tag;
                    node.start_mark = (*event).start_mark;
                    node.end_mark = (*event).end_mark;
                    node.data.sequence.items.start = items.start;
                    node.data.sequence.items.end = items.end;
                    node.data.sequence.items.top = items.start;
                    node.data.sequence.style = (*event).data.sequence_start.style;
                    if !(if (*(*parser).document).nodes.top != (*(*parser).document).nodes.end
                        || yaml_stack_extend(
                            addr_of_mut!((*(*parser).document).nodes.start)
                                as *mut *mut libc::c_void,
                            addr_of_mut!((*(*parser).document).nodes.top) as *mut *mut libc::c_void,
                            addr_of_mut!((*(*parser).document).nodes.end) as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh27 = addr_of_mut!((*(*parser).document).nodes.top);
                        let fresh28 = *fresh27;
                        *fresh27 = (*fresh27).wrapping_offset(1);
                        *fresh28 = node;
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        index = ((*(*parser).document).nodes.top)
                            .c_offset_from((*(*parser).document).nodes.start)
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
                        if if (((*ctx).top).c_offset_from((*ctx).start) as libc::c_long)
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
                        if if (*ctx).top != (*ctx).end
                            || yaml_stack_extend(
                                addr_of_mut!((*ctx).start) as *mut *mut libc::c_void,
                                addr_of_mut!((*ctx).top) as *mut *mut libc::c_void,
                                addr_of_mut!((*ctx).end) as *mut *mut libc::c_void,
                            ) != 0
                        {
                            let fresh29 = addr_of_mut!((*ctx).top);
                            let fresh30 = *fresh29;
                            *fresh29 = (*fresh29).wrapping_offset(1);
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
    0 as libc::c_int
}
unsafe fn yaml_parser_load_sequence_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> libc::c_int {
    __assert!(
        ((*ctx).top).c_offset_from((*ctx).start) as libc::c_long > 0 as libc::c_int as libc::c_long
    );
    let index: libc::c_int = *((*ctx).top).wrapping_offset(-(1 as libc::c_int as isize));
    __assert!(
        (*((*(*parser).document).nodes.start).wrapping_offset((index - 1 as libc::c_int) as isize))
            .type_0 as libc::c_uint
            == YAML_SEQUENCE_NODE as libc::c_int as libc::c_uint
    );
    (*((*(*parser).document).nodes.start).wrapping_offset((index - 1 as libc::c_int) as isize))
        .end_mark = (*event).end_mark;
    let fresh31 = addr_of_mut!((*ctx).top);
    *fresh31 = (*fresh31).wrapping_offset(-1);
    1 as libc::c_int
}
unsafe fn yaml_parser_load_mapping(
    mut parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> libc::c_int {
    let current_block: u64;
    let mut node = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: ptr::null_mut::<yaml_char_t>(),
        data: unnamed_yaml_node_s_data {
            scalar: unnamed_yaml_node_s_data_scalar {
                value: ptr::null_mut::<yaml_char_t>(),
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
    let mut pairs: Unnamed_35 = Unnamed_35 {
        start: ptr::null_mut::<yaml_node_pair_t>(),
        end: ptr::null_mut::<yaml_node_pair_t>(),
        top: ptr::null_mut::<yaml_node_pair_t>(),
    };
    let index: libc::c_int;
    let mut tag: *mut yaml_char_t = (*event).data.mapping_start.tag;
    if !(if (((*(*parser).document).nodes.top).c_offset_from((*(*parser).document).nodes.start)
        as libc::c_long)
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
                b"tag:yaml.org,2002:map\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
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
                pairs.start =
                    yaml_malloc((16 as libc::c_int as libc::c_ulong).wrapping_mul(size_of::<
                        yaml_node_pair_t,
                    >(
                    )
                        as libc::c_ulong)) as *mut yaml_node_pair_t;
                if !(if !(pairs.start).is_null() {
                    pairs.top = pairs.start;
                    pairs.end = (pairs.start).wrapping_offset(16 as libc::c_int as isize);
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    memset(
                        addr_of_mut!(node) as *mut libc::c_void,
                        0 as libc::c_int,
                        size_of::<yaml_node_t>() as libc::c_ulong,
                    );
                    node.type_0 = YAML_MAPPING_NODE;
                    node.tag = tag;
                    node.start_mark = (*event).start_mark;
                    node.end_mark = (*event).end_mark;
                    node.data.mapping.pairs.start = pairs.start;
                    node.data.mapping.pairs.end = pairs.end;
                    node.data.mapping.pairs.top = pairs.start;
                    node.data.mapping.style = (*event).data.mapping_start.style;
                    if !(if (*(*parser).document).nodes.top != (*(*parser).document).nodes.end
                        || yaml_stack_extend(
                            addr_of_mut!((*(*parser).document).nodes.start)
                                as *mut *mut libc::c_void,
                            addr_of_mut!((*(*parser).document).nodes.top) as *mut *mut libc::c_void,
                            addr_of_mut!((*(*parser).document).nodes.end) as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh32 = addr_of_mut!((*(*parser).document).nodes.top);
                        let fresh33 = *fresh32;
                        *fresh32 = (*fresh32).wrapping_offset(1);
                        *fresh33 = node;
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        index = ((*(*parser).document).nodes.top)
                            .c_offset_from((*(*parser).document).nodes.start)
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
                        if if (((*ctx).top).c_offset_from((*ctx).start) as libc::c_long)
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
                        if if (*ctx).top != (*ctx).end
                            || yaml_stack_extend(
                                addr_of_mut!((*ctx).start) as *mut *mut libc::c_void,
                                addr_of_mut!((*ctx).top) as *mut *mut libc::c_void,
                                addr_of_mut!((*ctx).end) as *mut *mut libc::c_void,
                            ) != 0
                        {
                            let fresh34 = addr_of_mut!((*ctx).top);
                            let fresh35 = *fresh34;
                            *fresh34 = (*fresh34).wrapping_offset(1);
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
    0 as libc::c_int
}
unsafe fn yaml_parser_load_mapping_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> libc::c_int {
    __assert!(
        ((*ctx).top).c_offset_from((*ctx).start) as libc::c_long > 0 as libc::c_int as libc::c_long
    );
    let index: libc::c_int = *((*ctx).top).wrapping_offset(-(1 as libc::c_int as isize));
    __assert!(
        (*((*(*parser).document).nodes.start).wrapping_offset((index - 1 as libc::c_int) as isize))
            .type_0 as libc::c_uint
            == YAML_MAPPING_NODE as libc::c_int as libc::c_uint
    );
    (*((*(*parser).document).nodes.start).wrapping_offset((index - 1 as libc::c_int) as isize))
        .end_mark = (*event).end_mark;
    let fresh36 = addr_of_mut!((*ctx).top);
    *fresh36 = (*fresh36).wrapping_offset(-1);
    1 as libc::c_int
}
