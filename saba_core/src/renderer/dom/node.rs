use core::cell::RefCell;

use alloc::{
    rc::{Rc, Weak},
    string::String,
    vec::Vec,
};

use crate::renderer::html::attribute::Attribute;

pub enum ElementKind {
    Html,
    Head,
    Style,
    Body,
}

pub struct Element {
    kind: ElementKind,
    attributes: Vec<Attribute>,
}

pub enum NodeKind {
    Document,
    Element(Element),
    Text(String),
}

pub struct Node {
    pub kind: NodeKind,
    window: Weak<RefCell<Node>>,
    parent: Weak<RefCell<Node>>,
    first_child: Option<Rc<RefCell<Node>>>,
    last_child: Weak<RefCell<Node>>,
    previous_sibling: Weak<RefCell<Node>>,
    next_sibling: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            window: Weak::new(),
            parent: Weak::new(),
            first_child: None,
            last_child: Weak::new(),
            previous_sibling: Weak::new(),
            next_sibling: None,
        }
    }
}

pub struct Window {
    document: Rc<RefCell<Node>>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            document: Rc::new(RefCell::new(Node::new(NodeKind::Document))),
        }
    }
}
