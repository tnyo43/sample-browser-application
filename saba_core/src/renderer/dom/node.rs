use core::{cell::RefCell, str::FromStr};

use alloc::{
    format,
    rc::{Rc, Weak},
    string::String,
    vec::Vec,
};

use crate::renderer::html::attribute::Attribute;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ElementKind {
    Html,
    Head,
    Style,
    Body,
    P,
    A,
    H1,
    H2,
}

impl FromStr for ElementKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "html" => Ok(ElementKind::Html),
            "head" => Ok(ElementKind::Head),
            "style" => Ok(ElementKind::Style),
            "body" => Ok(ElementKind::Body),
            "p" => Ok(ElementKind::P),
            "a" => Ok(ElementKind::A),
            "h1" => Ok(ElementKind::H1),
            "h2" => Ok(ElementKind::H2),
            _ => Err(format!("unimplemented element name {:?}", s)),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Element {
    kind: ElementKind,
    attributes: Vec<Attribute>,
}

impl Element {
    pub fn new(tag: &str, attributes: Vec<Attribute>) -> Self {
        Self {
            kind: ElementKind::from_str(tag).expect("failed to convert string to Element Kind"),
            attributes,
        }
    }

    pub fn kind(&self) -> ElementKind {
        self.kind
    }
}

#[derive(Debug)]
pub enum NodeKind {
    Document,
    Element(Element),
    Text(String),
}

impl PartialEq for NodeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Document, Self::Document) => true,
            (Self::Element(l0), Self::Element(r0)) => l0.kind == r0.kind,
            (Self::Text(l0), Self::Text(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    window: Weak<RefCell<Node>>,
    parent: Weak<RefCell<Node>>,
    first_child: Option<Rc<RefCell<Node>>>,
    last_child: Weak<RefCell<Node>>,
    previous_sibling: Weak<RefCell<Node>>,
    next_sibling: Option<Rc<RefCell<Node>>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
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

    pub fn element_kind(&self) -> Option<ElementKind> {
        match self.kind {
            NodeKind::Element(ref element) => Some(element.kind()),
            _ => None,
        }
    }

    pub fn set_parent(&mut self, node: Weak<RefCell<Node>>) {
        self.parent = node
    }

    pub fn parent(&self) -> Weak<RefCell<Node>> {
        self.parent.clone()
    }

    pub fn set_first_child(&mut self, node: Option<Rc<RefCell<Node>>>) {
        self.first_child = node
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.first_child.as_ref().cloned()
    }

    pub fn set_last_child(&mut self, node: Weak<RefCell<Node>>) {
        self.last_child = node
    }

    pub fn last_child(&self) -> Weak<RefCell<Node>> {
        self.last_child.clone()
    }

    pub fn set_previous_sibling(&mut self, node: Weak<RefCell<Node>>) {
        self.previous_sibling = node
    }

    pub fn previous_sibling(&self) -> Weak<RefCell<Node>> {
        self.previous_sibling.clone()
    }

    pub fn set_next_sibling(&mut self, node: Option<Rc<RefCell<Node>>>) {
        self.next_sibling = node
    }

    pub fn next_sibling(&self) -> Option<Rc<RefCell<Node>>> {
        self.next_sibling.as_ref().cloned()
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    document: Rc<RefCell<Node>>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            document: Rc::new(RefCell::new(Node::new(NodeKind::Document))),
        }
    }

    pub fn document(&self) -> Rc<RefCell<Node>> {
        self.document.clone()
    }
}
