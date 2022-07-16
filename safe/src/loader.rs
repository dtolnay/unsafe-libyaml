use crate::cstr::CStr;
use crate::error::{Error, Mark, Result};
use crate::util::Owned;
use std::borrow::Cow;
use std::ptr::{self, addr_of, addr_of_mut, NonNull};
use std::slice;
use unsafe_libyaml as sys;

pub struct Parser<'input> {
    pin: Owned<ParserPinned<'input>>,
}

struct ParserPinned<'input> {
    sys: sys::yaml_parser_t,
    input: Cow<'input, [u8]>,
}

pub struct Document<'input> {
    pin: Owned<DocumentPinned<'input>>,
}

struct DocumentPinned<'input> {
    sys: sys::yaml_document_t,
    input: Cow<'input, [u8]>,
}

#[derive(Copy, Clone)]
pub enum Node<'input, 'document> {
    Scalar(Scalar<'input, 'document>),
    Sequence(Sequence<'input, 'document>),
    Mapping(Mapping<'input, 'document>),
}

#[derive(Copy, Clone)]
pub struct Scalar<'input, 'document> {
    sys: &'document sys::yaml_node_t,
    document: &'document DocumentPinned<'input>,
}

#[derive(Copy, Clone)]
pub struct Sequence<'input, 'document> {
    sys: &'document sys::yaml_node_t,
    document: &'document DocumentPinned<'input>,
}

#[derive(Copy, Clone)]
pub struct Mapping<'input, 'document> {
    sys: &'document sys::yaml_node_t,
    document: &'document DocumentPinned<'input>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ScalarStyle {
    Plain,
    SingleQuoted,
    DoubleQuoted,
    Literal,
    Folded,
}

impl<'input> Parser<'input> {
    pub fn new(input: Cow<'input, [u8]>) -> Result<Parser<'input>> {
        let owned = Owned::<ParserPinned>::new_uninit();
        let pin = unsafe {
            let parser = addr_of_mut!((*owned.ptr).sys);
            if sys::yaml_parser_initialize(parser) == 0 {
                return Err(Error::parse_error(parser));
            }
            sys::yaml_parser_set_encoding(parser, sys::YAML_UTF8_ENCODING);
            sys::yaml_parser_set_input_string(parser, input.as_ptr(), input.len() as u64);
            addr_of_mut!((*owned.ptr).input).write(input);
            Owned::assume_init(owned)
        };
        Ok(Parser { pin })
    }

    pub fn next_document(&mut self) -> Result<Document<'input>> {
        let owned = Owned::<DocumentPinned>::new_uninit();
        let pin = unsafe {
            let parser = addr_of_mut!((*self.pin.ptr).sys);
            let document = addr_of_mut!((*owned.ptr).sys);
            if sys::yaml_parser_load(parser, document) == 0 {
                return Err(Error::parse_error(parser));
            }
            ptr::copy_nonoverlapping(
                addr_of!((*self.pin.ptr).input),
                addr_of_mut!((*owned.ptr).input),
                1,
            );
            Owned::assume_init(owned)
        };
        Ok(Document { pin })
    }
}

impl<'input> Document<'input> {
    pub fn root_node<'document>(&'document self) -> Option<Node<'input, 'document>> {
        unsafe {
            let pin = &*self.pin.ptr;
            let document = addr_of!(pin.sys) as *mut _;
            let node = sys::yaml_document_get_root_node(document).as_ref()?;
            Some(convert_node(pin, node))
        }
    }
}

impl<'input, 'document> Node<'input, 'document> {
    pub fn tag(&self) -> Option<&'document [u8]> {
        match self {
            Node::Scalar(scalar) => scalar.tag(),
            Node::Sequence(sequence) => sequence.tag(),
            Node::Mapping(mapping) => mapping.tag(),
        }
    }

    pub fn mark(&self) -> Mark {
        match self {
            Node::Scalar(scalar) => scalar.mark(),
            Node::Sequence(sequence) => sequence.mark(),
            Node::Mapping(mapping) => mapping.mark(),
        }
    }
}

impl<'input, 'document> Scalar<'input, 'document> {
    pub fn style(&self) -> ScalarStyle {
        match unsafe { self.sys.data.scalar.style } {
            sys::YAML_PLAIN_SCALAR_STYLE => ScalarStyle::Plain,
            sys::YAML_SINGLE_QUOTED_SCALAR_STYLE => ScalarStyle::SingleQuoted,
            sys::YAML_DOUBLE_QUOTED_SCALAR_STYLE => ScalarStyle::DoubleQuoted,
            sys::YAML_LITERAL_SCALAR_STYLE => ScalarStyle::Literal,
            sys::YAML_FOLDED_SCALAR_STYLE => ScalarStyle::Folded,
            sys::YAML_ANY_SCALAR_STYLE | _ => unreachable!(),
        }
    }

    pub fn value(&self) -> &'document [u8] {
        unsafe {
            let ptr = self.sys.data.scalar.value;
            let len = self.sys.data.scalar.length;
            slice::from_raw_parts(ptr, len as usize)
        }
    }

    pub fn repr(&self) -> Option<&'input [u8]> {
        let borrowed = match self.document.input {
            Cow::Borrowed(borrowed) => borrowed,
            Cow::Owned(_) => return None,
        };
        let start = self.sys.start_mark.index as usize;
        let end = self.sys.end_mark.index as usize;
        Some(&borrowed[start..end])
    }

    pub fn tag(&self) -> Option<&'document [u8]> {
        let tag = NonNull::new(self.sys.tag.cast())?;
        let cstr = unsafe { CStr::from_ptr(tag) };
        Some(cstr.to_bytes())
    }

    pub fn mark(&self) -> Mark {
        Mark {
            sys: self.sys.start_mark,
        }
    }
}

impl<'input, 'document> Sequence<'input, 'document> {
    pub fn tag(&self) -> Option<&'document [u8]> {
        let tag = NonNull::new(self.sys.tag.cast())?;
        let cstr = unsafe { CStr::from_ptr(tag) };
        Some(cstr.to_bytes())
    }

    pub fn mark(&self) -> Mark {
        Mark {
            sys: self.sys.start_mark,
        }
    }
}

impl<'input, 'document> Mapping<'input, 'document> {
    pub fn tag(&self) -> Option<&'document [u8]> {
        let tag = NonNull::new(self.sys.tag.cast())?;
        let cstr = unsafe { CStr::from_ptr(tag) };
        Some(cstr.to_bytes())
    }

    pub fn mark(&self) -> Mark {
        Mark {
            sys: self.sys.start_mark,
        }
    }
}

pub struct SequenceIter<'input, 'document> {
    items: &'document [sys::yaml_node_item_t],
    document: &'document DocumentPinned<'input>,
}

impl<'input, 'document> IntoIterator for Sequence<'input, 'document> {
    type Item = Node<'input, 'document>;
    type IntoIter = SequenceIter<'input, 'document>;

    fn into_iter(self) -> Self::IntoIter {
        SequenceIter {
            items: unsafe { stack_as_slice(&self.sys.data.sequence.items) },
            document: self.document,
        }
    }
}

impl<'input, 'document> Iterator for SequenceIter<'input, 'document> {
    type Item = Node<'input, 'document>;

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.items.split_first()?;
        self.items = rest;
        let document = addr_of!(self.document.sys) as *mut _;
        Some(unsafe {
            let node = &*sys::yaml_document_get_node(document, *first);
            convert_node(self.document, node)
        })
    }
}

pub struct MappingIter<'input, 'document> {
    items: &'document [sys::yaml_node_pair_t],
    document: &'document DocumentPinned<'input>,
}

impl<'input, 'document> IntoIterator for Mapping<'input, 'document> {
    type Item = (Node<'input, 'document>, Node<'input, 'document>);
    type IntoIter = MappingIter<'input, 'document>;

    fn into_iter(self) -> Self::IntoIter {
        MappingIter {
            items: unsafe { stack_as_slice(&self.sys.data.mapping.pairs) },
            document: self.document,
        }
    }
}

impl<'input, 'document> Iterator for MappingIter<'input, 'document> {
    type Item = (Node<'input, 'document>, Node<'input, 'document>);

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.items.split_first()?;
        self.items = rest;
        let document = addr_of!(self.document.sys) as *mut _;
        Some(unsafe {
            let key = &*sys::yaml_document_get_node(document, first.key);
            let value = &*sys::yaml_document_get_node(document, first.value);
            (
                convert_node(self.document, key),
                convert_node(self.document, value),
            )
        })
    }
}

unsafe fn convert_node<'input, 'document>(
    document: &'document DocumentPinned<'input>,
    node: &'document sys::yaml_node_t,
) -> Node<'input, 'document> {
    match node.type_ {
        sys::YAML_SCALAR_NODE => Node::Scalar(Scalar {
            sys: node,
            document,
        }),
        sys::YAML_SEQUENCE_NODE => Node::Sequence(Sequence {
            sys: node,
            document,
        }),
        sys::YAML_MAPPING_NODE => Node::Mapping(Mapping {
            sys: node,
            document,
        }),
        sys::YAML_NO_NODE | _ => unreachable!(),
    }
}

unsafe fn stack_as_slice<T>(stack: &sys::yaml_stack_t<T>) -> &[T] {
    let len = stack.top.offset_from(stack.start);
    slice::from_raw_parts(stack.start, len as usize)
}

impl<'input> Drop for ParserPinned<'input> {
    fn drop(&mut self) {
        unsafe { sys::yaml_parser_delete(&mut self.sys) }
    }
}

impl<'input> Drop for DocumentPinned<'input> {
    fn drop(&mut self) {
        unsafe { sys::yaml_document_delete(&mut self.sys) }
    }
}
