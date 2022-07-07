use crate::api::{yaml_document_delete, yaml_free, yaml_malloc};
use crate::emitter::yaml_emitter_emit;
use crate::externs::*;
use crate::libc;
use crate::yaml::*;
use crate::PointerExt;
use std::io::Write;
use std::slice;
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_open(mut emitter: *mut yaml_emitter_t) -> libc::c_int {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    __assert!(!emitter.is_null());
    __assert!((*emitter).opened == 0);
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_STREAM_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.stream_start.encoding = YAML_ANY_ENCODING;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    (*emitter).opened = 1 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_close(mut emitter: *mut yaml_emitter_t) -> libc::c_int {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    __assert!(!emitter.is_null());
    __assert!((*emitter).opened != 0);
    if (*emitter).closed != 0 {
        return 1 as libc::c_int;
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_STREAM_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    (*emitter).closed = 1 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_dump(
    mut emitter: *mut yaml_emitter_t,
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    __assert!(!emitter.is_null());
    __assert!(!document.is_null());
    let ref mut fresh0 = (*emitter).document;
    *fresh0 = document;
    if (*emitter).opened == 0 {
        if yaml_emitter_open(emitter) == 0 {
            current_block = 5018439318894558507;
        } else {
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            if (*document).nodes.start == (*document).nodes.top {
                if !(yaml_emitter_close(emitter) == 0) {
                    yaml_emitter_delete_document_and_anchors(emitter);
                    return 1 as libc::c_int;
                }
            } else {
                __assert!((*emitter).opened != 0);
                let ref mut fresh1 = (*emitter).anchors;
                *fresh1 = yaml_malloc(
                    (::std::mem::size_of::<yaml_anchors_t>() as libc::c_ulong).wrapping_mul(
                        ((*document).nodes.top).c_offset_from((*document).nodes.start)
                            as libc::c_long as libc::c_ulong,
                    ),
                ) as *mut yaml_anchors_t;
                if !((*emitter).anchors).is_null() {
                    memset(
                        (*emitter).anchors as *mut libc::c_void,
                        0 as libc::c_int,
                        (::std::mem::size_of::<yaml_anchors_t>() as libc::c_ulong).wrapping_mul(
                            ((*document).nodes.top).c_offset_from((*document).nodes.start)
                                as libc::c_long as libc::c_ulong,
                        ),
                    );
                    memset(
                        &mut event as *mut yaml_event_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    event.type_0 = YAML_DOCUMENT_START_EVENT;
                    event.start_mark = mark;
                    event.end_mark = mark;
                    event.data.document_start.version_directive = (*document).version_directive;
                    event.data.document_start.tag_directives.start =
                        (*document).tag_directives.start;
                    event.data.document_start.tag_directives.end = (*document).tag_directives.end;
                    event.data.document_start.implicit = (*document).start_implicit;
                    if !(yaml_emitter_emit(emitter, &mut event) == 0) {
                        yaml_emitter_anchor_node(emitter, 1 as libc::c_int);
                        if !(yaml_emitter_dump_node(emitter, 1 as libc::c_int) == 0) {
                            memset(
                                &mut event as *mut yaml_event_t as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            event.type_0 = YAML_DOCUMENT_END_EVENT;
                            event.start_mark = mark;
                            event.end_mark = mark;
                            event.data.document_end.implicit = (*document).end_implicit;
                            if !(yaml_emitter_emit(emitter, &mut event) == 0) {
                                yaml_emitter_delete_document_and_anchors(emitter);
                                return 1 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    yaml_emitter_delete_document_and_anchors(emitter);
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_delete_document_and_anchors(mut emitter: *mut yaml_emitter_t) {
    let mut index: libc::c_int = 0;
    if ((*emitter).anchors).is_null() {
        yaml_document_delete((*emitter).document);
        let ref mut fresh2 = (*emitter).document;
        *fresh2 = 0 as *mut yaml_document_t;
        return;
    }
    index = 0 as libc::c_int;
    while ((*(*emitter).document).nodes.start).c_offset(index as isize)
        < (*(*emitter).document).nodes.top
    {
        let mut node: yaml_node_t = *((*(*emitter).document).nodes.start).c_offset(index as isize);
        if (*((*emitter).anchors).c_offset(index as isize)).serialized == 0 {
            yaml_free(node.tag as *mut libc::c_void);
            if node.type_0 as libc::c_uint == YAML_SCALAR_NODE as libc::c_int as libc::c_uint {
                yaml_free(node.data.scalar.value as *mut libc::c_void);
            }
        }
        if node.type_0 as libc::c_uint == YAML_SEQUENCE_NODE as libc::c_int as libc::c_uint {
            yaml_free(node.data.sequence.items.start as *mut libc::c_void);
            node.data.sequence.items.end = 0 as *mut yaml_node_item_t;
            node.data.sequence.items.top = node.data.sequence.items.end;
            node.data.sequence.items.start = node.data.sequence.items.top;
        }
        if node.type_0 as libc::c_uint == YAML_MAPPING_NODE as libc::c_int as libc::c_uint {
            yaml_free(node.data.mapping.pairs.start as *mut libc::c_void);
            node.data.mapping.pairs.end = 0 as *mut yaml_node_pair_t;
            node.data.mapping.pairs.top = node.data.mapping.pairs.end;
            node.data.mapping.pairs.start = node.data.mapping.pairs.top;
        }
        index += 1;
    }
    yaml_free((*(*emitter).document).nodes.start as *mut libc::c_void);
    let ref mut fresh3 = (*(*emitter).document).nodes.end;
    *fresh3 = 0 as *mut yaml_node_t;
    let ref mut fresh4 = (*(*emitter).document).nodes.top;
    *fresh4 = *fresh3;
    let ref mut fresh5 = (*(*emitter).document).nodes.start;
    *fresh5 = *fresh4;
    yaml_free((*emitter).anchors as *mut libc::c_void);
    let ref mut fresh6 = (*emitter).anchors;
    *fresh6 = 0 as *mut yaml_anchors_t;
    (*emitter).last_anchor_id = 0 as libc::c_int;
    let ref mut fresh7 = (*emitter).document;
    *fresh7 = 0 as *mut yaml_document_t;
}
unsafe extern "C" fn yaml_emitter_anchor_node(
    mut emitter: *mut yaml_emitter_t,
    mut index: libc::c_int,
) {
    let mut node: *mut yaml_node_t = ((*(*emitter).document).nodes.start)
        .c_offset(index as isize)
        .c_offset(-(1 as libc::c_int as isize));
    let mut item: *mut yaml_node_item_t = 0 as *mut yaml_node_item_t;
    let mut pair: *mut yaml_node_pair_t = 0 as *mut yaml_node_pair_t;
    let ref mut fresh8 =
        (*((*emitter).anchors).c_offset((index - 1 as libc::c_int) as isize)).references;
    *fresh8 += 1;
    if (*((*emitter).anchors).c_offset((index - 1 as libc::c_int) as isize)).references
        == 1 as libc::c_int
    {
        match (*node).type_0 as libc::c_uint {
            2 => {
                item = (*node).data.sequence.items.start;
                while item < (*node).data.sequence.items.top {
                    yaml_emitter_anchor_node(emitter, *item);
                    item = item.c_offset(1);
                }
            }
            3 => {
                pair = (*node).data.mapping.pairs.start;
                while pair < (*node).data.mapping.pairs.top {
                    yaml_emitter_anchor_node(emitter, (*pair).key);
                    yaml_emitter_anchor_node(emitter, (*pair).value);
                    pair = pair.c_offset(1);
                }
            }
            _ => {}
        }
    } else if (*((*emitter).anchors).c_offset((index - 1 as libc::c_int) as isize)).references
        == 2 as libc::c_int
    {
        let ref mut fresh9 = (*emitter).last_anchor_id;
        *fresh9 += 1;
        (*((*emitter).anchors).c_offset((index - 1 as libc::c_int) as isize)).anchor = *fresh9;
    }
}
unsafe extern "C" fn yaml_emitter_generate_anchor(
    mut emitter: *mut yaml_emitter_t,
    mut anchor_id: libc::c_int,
) -> *mut yaml_char_t {
    let mut anchor: *mut yaml_char_t = yaml_malloc(16 as libc::c_int as size_t) as *mut yaml_char_t;
    if anchor.is_null() {
        return 0 as *mut yaml_char_t;
    }
    let mut buffer = slice::from_raw_parts_mut(anchor.cast(), 16);
    let _ = write!(buffer, "id{:03}\0", anchor_id);
    return anchor;
}
unsafe extern "C" fn yaml_emitter_dump_node(
    mut emitter: *mut yaml_emitter_t,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut node: *mut yaml_node_t = ((*(*emitter).document).nodes.start)
        .c_offset(index as isize)
        .c_offset(-(1 as libc::c_int as isize));
    let mut anchor_id: libc::c_int =
        (*((*emitter).anchors).c_offset((index - 1 as libc::c_int) as isize)).anchor;
    let mut anchor: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if anchor_id != 0 {
        anchor = yaml_emitter_generate_anchor(emitter, anchor_id);
        if anchor.is_null() {
            return 0 as libc::c_int;
        }
    }
    if (*((*emitter).anchors).c_offset((index - 1 as libc::c_int) as isize)).serialized != 0 {
        return yaml_emitter_dump_alias(emitter, anchor);
    }
    (*((*emitter).anchors).c_offset((index - 1 as libc::c_int) as isize)).serialized =
        1 as libc::c_int;
    match (*node).type_0 as libc::c_uint {
        1 => return yaml_emitter_dump_scalar(emitter, node, anchor),
        2 => return yaml_emitter_dump_sequence(emitter, node, anchor),
        3 => return yaml_emitter_dump_mapping(emitter, node, anchor),
        _ => {
            __assert!(false);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_dump_alias(
    mut emitter: *mut yaml_emitter_t,
    mut anchor: *mut yaml_char_t,
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_ALIAS_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.alias.anchor = anchor;
    return yaml_emitter_emit(emitter, &mut event);
}
unsafe extern "C" fn yaml_emitter_dump_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut plain_implicit: libc::c_int = (strcmp(
        (*node).tag as *mut libc::c_char,
        b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    let mut quoted_implicit: libc::c_int = (strcmp(
        (*node).tag as *mut libc::c_char,
        b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_SCALAR_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.scalar.anchor = anchor;
    event.data.scalar.tag = (*node).tag;
    event.data.scalar.value = (*node).data.scalar.value;
    event.data.scalar.length = (*node).data.scalar.length;
    event.data.scalar.plain_implicit = plain_implicit;
    event.data.scalar.quoted_implicit = quoted_implicit;
    event.data.scalar.style = (*node).data.scalar.style;
    return yaml_emitter_emit(emitter, &mut event);
}
unsafe extern "C" fn yaml_emitter_dump_sequence(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut implicit: libc::c_int = (strcmp(
        (*node).tag as *mut libc::c_char,
        b"tag:yaml.org,2002:seq\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    let mut item: *mut yaml_node_item_t = 0 as *mut yaml_node_item_t;
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_SEQUENCE_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.sequence_start.anchor = anchor;
    event.data.sequence_start.tag = (*node).tag;
    event.data.sequence_start.implicit = implicit;
    event.data.sequence_start.style = (*node).data.sequence.style;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    item = (*node).data.sequence.items.start;
    while item < (*node).data.sequence.items.top {
        if yaml_emitter_dump_node(emitter, *item) == 0 {
            return 0 as libc::c_int;
        }
        item = item.c_offset(1);
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_SEQUENCE_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_emitter_dump_mapping(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut implicit: libc::c_int = (strcmp(
        (*node).tag as *mut libc::c_char,
        b"tag:yaml.org,2002:map\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    let mut pair: *mut yaml_node_pair_t = 0 as *mut yaml_node_pair_t;
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_MAPPING_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.mapping_start.anchor = anchor;
    event.data.mapping_start.tag = (*node).tag;
    event.data.mapping_start.implicit = implicit;
    event.data.mapping_start.style = (*node).data.mapping.style;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    pair = (*node).data.mapping.pairs.start;
    while pair < (*node).data.mapping.pairs.top {
        if yaml_emitter_dump_node(emitter, (*pair).key) == 0 {
            return 0 as libc::c_int;
        }
        if yaml_emitter_dump_node(emitter, (*pair).value) == 0 {
            return 0 as libc::c_int;
        }
        pair = pair.c_offset(1);
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    event.type_0 = YAML_MAPPING_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
