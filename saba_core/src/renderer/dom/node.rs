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

pub struct Window {
    document: Rc<RefCell<Window>>,
}
