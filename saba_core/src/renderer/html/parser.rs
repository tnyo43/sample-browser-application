use core::{cell::RefCell, str::FromStr};

use alloc::{rc::Rc, string::String, vec::Vec};

use crate::renderer::dom::node::{Element, ElementKind, Node, NodeKind, Window};

use super::{
    attribute::Attribute,
    token::{HtmlToken, HtmlTokenizer},
};

#[derive(Clone, Copy)]
enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    AfterHead,
    InBody,
    Text,
    AfterBody,
    AfterAfterBody,
}

pub struct HtmlParser {
    window: Rc<RefCell<Window>>,
    mode: InsertionMode,
    original_intersection_mode: InsertionMode,
    stack_of_open_elements: Vec<Rc<RefCell<Node>>>,
    t: HtmlTokenizer,
}

impl HtmlParser {
    pub fn new(t: HtmlTokenizer) -> Self {
        Self {
            t,
            window: Rc::new(RefCell::new(Window::new())),
            mode: InsertionMode::Initial,
            original_intersection_mode: InsertionMode::Initial,
            stack_of_open_elements: Vec::new(),
        }
    }

    fn contain_in_stack(&self, element_kind: ElementKind) -> bool {
        for i in 0..self.stack_of_open_elements.len() {
            if self.stack_of_open_elements[i].borrow().element_kind() == Some(element_kind) {
                return true;
            }
        }

        false
    }

    fn pop_until(&mut self, element_kind: ElementKind) {
        assert!(
            self.contain_in_stack(element_kind),
            "stack doesn't have an element {:?}",
            element_kind
        );

        loop {
            let current = match self.stack_of_open_elements.pop() {
                Some(n) => n,
                None => unreachable!("element should be found"),
            };

            if current.borrow().element_kind() == Some(element_kind) {
                return;
            }
        }
    }

    fn pop_current_node(&mut self, element_kind: ElementKind) -> bool {
        let current = match self.stack_of_open_elements.last() {
            Some(n) => n,
            None => return false,
        };

        if current.borrow().element_kind() == Some(element_kind) {
            self.stack_of_open_elements.pop();
            return true;
        }

        false
    }

    fn create_element(&self, tag: &str, attributes: Vec<Attribute>) -> Node {
        Node::new(NodeKind::Element(Element::new(tag, attributes)))
    }

    fn insert_element(&mut self, tag: &str, attributes: Vec<Attribute>) {
        let window = self.window.borrow();
        let mut current = window.document();
        loop {
            match self.stack_of_open_elements.last() {
                Some(n) => {
                    let node = n.clone();
                    if let NodeKind::Text(_) = node.borrow().kind {
                        self.stack_of_open_elements.pop();
                        continue;
                    };
                    current = node;
                    break;
                }
                None => {
                    break;
                }
            }
        }

        let node = Rc::new(RefCell::new(self.create_element(tag, attributes)));

        if current.borrow().first_child().is_some() {
            let mut last_sibling = current.borrow().first_child();
            loop {
                last_sibling = match last_sibling {
                    Some(ref node) => {
                        if node.borrow().next_sibling().is_some() {
                            node.borrow().next_sibling()
                        } else {
                            break;
                        }
                    }
                    None => unimplemented!("last_sibling should be Some"),
                }
            }

            last_sibling
                .as_ref()
                .unwrap()
                .borrow_mut()
                .set_next_sibling(Some(node.clone()));
            node.borrow_mut()
                .set_previous_sibling(Rc::downgrade(&last_sibling.unwrap()));
        } else {
            current.borrow_mut().set_first_child(Some(node.clone()));
        }

        current.borrow_mut().set_last_child(Rc::downgrade(&node));
        node.borrow_mut().set_parent(Rc::downgrade(&current));

        self.stack_of_open_elements.push(node);
    }

    fn create_char(&self, c: char) -> Node {
        let mut s = String::new();
        s.push(c);
        Node::new(NodeKind::Text(s))
    }

    fn insert_char(&mut self, c: char) {
        let current = match self.stack_of_open_elements.last() {
            Some(n) => n.clone(),
            None => return,
        };

        if let NodeKind::Text(ref mut s) = current.borrow_mut().kind {
            s.push(c);
            return;
        };

        if c == ' ' || c == '\n' {
            return;
        }

        let node = Rc::new(RefCell::new(self.create_char(c)));

        if current.borrow().first_child().is_some() {
            let mut last_sibling = current.borrow().first_child();
            loop {
                last_sibling = match last_sibling {
                    Some(ref node) => {
                        if node.borrow().next_sibling().is_some() {
                            node.borrow().next_sibling()
                        } else {
                            break;
                        }
                    }
                    None => unimplemented!("last_sibling should be Some"),
                }
            }

            last_sibling
                .as_ref()
                .unwrap()
                .borrow_mut()
                .set_next_sibling(Some(node.clone()));
            node.borrow_mut()
                .set_previous_sibling(Rc::downgrade(&last_sibling.unwrap()));
        } else {
            current.borrow_mut().set_first_child(Some(node.clone()));
        }

        current.borrow_mut().set_last_child(Rc::downgrade(&node));
        node.borrow_mut().set_parent(Rc::downgrade(&current));

        self.stack_of_open_elements.push(node);
    }

    pub fn construct_tree(&mut self) -> Rc<RefCell<Window>> {
        let mut token = self.t.next();

        while token.is_some() {
            match self.mode {
                InsertionMode::Initial => {
                    if let Some(HtmlToken::Char(_)) = token {
                        token = self.t.next();
                        continue;
                    }

                    self.mode = InsertionMode::BeforeHtml;
                    continue;
                }
                InsertionMode::BeforeHtml => {
                    match token {
                        Some(HtmlToken::Char(c)) => {
                            if c == ' ' || c == '\n' {
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HtmlToken::StartTag {
                            ref tag,
                            self_closing: _,
                            ref attributes,
                        }) => {
                            if tag == "html" {
                                self.insert_element(tag, attributes.to_vec());
                                self.mode = InsertionMode::BeforeHead;
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HtmlToken::Eof) => {
                            return self.window.clone();
                        }
                        _ => {}
                    }
                    self.insert_element("html", Vec::new());
                    self.mode = InsertionMode::BeforeHead;
                    continue;
                }
                InsertionMode::BeforeHead => {
                    match token {
                        Some(HtmlToken::Char(c)) => {
                            if c == ' ' || c == '\n' {
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HtmlToken::StartTag {
                            ref tag,
                            self_closing: _,
                            ref attributes,
                        }) => {
                            if tag == "head" {
                                self.insert_element(tag, attributes.to_vec());
                                self.mode = InsertionMode::InHead;
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HtmlToken::Eof) | None => return self.window.clone(),
                        _ => {}
                    }
                    self.insert_element("head", Vec::new());
                    self.mode = InsertionMode::InHead;
                    continue;
                }
                InsertionMode::InHead => {
                    match token {
                        Some(HtmlToken::Char(c)) => {
                            if c == ' ' || c == '\n' {
                                self.insert_char(c);
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HtmlToken::StartTag {
                            ref tag,
                            self_closing: _,
                            ref attributes,
                        }) => {
                            if tag == "style" {
                                self.insert_element(tag, attributes.to_vec());
                                self.original_intersection_mode = self.mode;
                                self.mode = InsertionMode::Text;
                                token = self.t.next();
                                continue;
                            }

                            if tag == "body" {
                                self.pop_until(ElementKind::Head);
                                self.mode = InsertionMode::AfterHead;
                                continue;
                            }
                        }
                        Some(HtmlToken::EndTag { ref tag }) => {
                            if tag == "head" {
                                self.mode = InsertionMode::AfterHead;
                                token = self.t.next();
                                self.pop_until(ElementKind::Head);
                                continue;
                            }
                        }
                        Some(HtmlToken::Eof) | None => return self.window.clone(),
                    }

                    token = self.t.next();
                    continue;
                }
                InsertionMode::AfterHead => {
                    match token {
                        Some(HtmlToken::Char(c)) => {
                            if c == ' ' || c == '\n' {
                                self.insert_char(c);
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HtmlToken::StartTag {
                            ref tag,
                            self_closing: _,
                            ref attributes,
                        }) => {
                            if tag == "body" {
                                self.insert_element("body", attributes.to_vec());
                                token = self.t.next();
                                self.mode = InsertionMode::InBody;
                                continue;
                            }
                        }
                        Some(HtmlToken::Eof) => return self.window.clone(),
                        _ => {}
                    }

                    self.insert_element("body", Vec::new());
                    self.mode = InsertionMode::InBody;
                    continue;
                }
                InsertionMode::InBody => {
                    match token {
                        Some(HtmlToken::StartTag {
                            ref tag,
                            self_closing: _,
                            ref attributes,
                        }) => match tag.as_str() {
                            "p" | "a" | "h1" | "h2" => {
                                self.insert_element(tag, attributes.to_vec());
                                token = self.t.next();
                                continue;
                            }
                            _ => {
                                token = self.t.next();
                            }
                        },
                        Some(HtmlToken::EndTag { ref tag }) => match tag.as_str() {
                            "body" => {
                                self.mode = InsertionMode::AfterBody;
                                token = self.t.next();
                                if !self.contain_in_stack(ElementKind::Body) {
                                    continue;
                                }
                                self.pop_until(ElementKind::Body);
                                continue;
                            }
                            "html" => {
                                if self.pop_current_node(ElementKind::Body) {
                                    self.mode = InsertionMode::AfterAfterBody;
                                    assert!(self.pop_current_node(ElementKind::Html));
                                };
                            }
                            "p" | "a" | "h1" | "h2" => {
                                let element_kind = ElementKind::from_str(tag)
                                    .expect("failed to convert string to ElementKind");
                                self.pop_until(element_kind);
                                token = self.t.next();
                                continue;
                            }
                            _ => {
                                token = self.t.next();
                            }
                        },
                        Some(HtmlToken::Char(c)) => {
                            self.insert_char(c);
                            token = self.t.next();
                            continue;
                        }
                        Some(HtmlToken::Eof) | None => {
                            return self.window.clone();
                        }
                    }

                    continue;
                }
                InsertionMode::Text => {
                    match token {
                        Some(HtmlToken::Eof) | None => return self.window.clone(),
                        Some(HtmlToken::EndTag { ref tag }) => {
                            if tag == "style" {
                                self.pop_until(ElementKind::Style);
                                self.mode = self.original_intersection_mode;
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HtmlToken::Char(c)) => {
                            self.insert_char(c);
                            token = self.t.next();
                            continue;
                        }
                        _ => {}
                    }

                    self.mode = self.original_intersection_mode;
                    continue;
                }
                InsertionMode::AfterBody => {
                    match token {
                        Some(HtmlToken::Char(_)) => {
                            token = self.t.next();
                            continue;
                        }
                        Some(HtmlToken::EndTag { ref tag }) => {
                            if tag == "html" {
                                self.mode = InsertionMode::AfterAfterBody;
                                token = self.t.next();
                                continue;
                            }
                        }
                        Some(HtmlToken::Eof) | None => {
                            return self.window.clone();
                        }
                        _ => {}
                    }

                    self.mode = InsertionMode::InBody;
                }
                InsertionMode::AfterAfterBody => {
                    match token {
                        Some(HtmlToken::Char(_)) => {
                            token = self.t.next();
                            continue;
                        }
                        Some(HtmlToken::Eof) | None => {
                            return self.window.clone();
                        }
                        _ => {}
                    }

                    self.mode = InsertionMode::InBody;
                }
            }
        }

        self.window.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc::string::ToString;

    #[test]
    fn test_empty() {
        let html = "".to_string();
        let t = HtmlTokenizer::new(html);
        let window = HtmlParser::new(t).construct_tree();

        let document = window.borrow().document();
        assert_eq!(
            document,
            Rc::new(RefCell::new(Node::new(NodeKind::Document)))
        );
    }

    #[test]
    fn test_html_head_body() {
        let html = "<html><head></head><body></body></html>".to_string();
        let t = HtmlTokenizer::new(html);
        let window = HtmlParser::new(t).construct_tree();

        let document = window.borrow().document();
        assert_eq!(
            document,
            Rc::new(RefCell::new(Node::new(NodeKind::Document)))
        );

        let html = document.borrow().first_child().unwrap();
        assert_eq!(
            html,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "html",
                Vec::new()
            )))))
        );

        let head = html.borrow().first_child().unwrap();
        assert_eq!(
            head,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "head",
                Vec::new()
            )))))
        );

        let body = head.borrow().next_sibling().unwrap();
        assert_eq!(
            body,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "body",
                Vec::new()
            )))))
        );
    }

    #[test]
    fn test_without_html() {
        let html = "<body></body>".to_string();
        let t = HtmlTokenizer::new(html);
        let window = HtmlParser::new(t).construct_tree();

        let document = window.borrow().document();
        assert_eq!(
            document,
            Rc::new(RefCell::new(Node::new(NodeKind::Document)))
        );

        let html = document.borrow().first_child().unwrap();
        assert_eq!(
            html,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "html",
                Vec::new()
            )))))
        );

        let head = html.borrow().first_child().unwrap();
        assert_eq!(
            head,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "head",
                Vec::new()
            )))))
        );

        let body = head.borrow().next_sibling().unwrap();
        assert_eq!(
            body,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "body",
                Vec::new()
            )))))
        );
    }

    #[test]
    fn test_text() {
        let html = "<html><head></head><body><p>hello world<p></body></html>".to_string();
        let t = HtmlTokenizer::new(html);
        let window = HtmlParser::new(t).construct_tree();

        let document = window.borrow().document();
        let body = document
            .borrow()
            .first_child()
            .unwrap() // html
            .borrow()
            .first_child()
            .unwrap() // head
            .borrow()
            .next_sibling()
            .unwrap();
        assert_eq!(
            body,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "body",
                Vec::new()
            )))))
        );

        let p = body.borrow().first_child().unwrap();
        assert_eq!(
            p,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "p",
                Vec::new()
            )))))
        );

        let text = p.borrow().first_child().unwrap();
        assert_eq!(
            text,
            Rc::new(RefCell::new(Node::new(NodeKind::Text(
                "hello world".to_string()
            ))))
        );
    }

    #[test]
    fn test_next_and_previous_siblings() {
        let html = "<html><head></head><body><h1></h1><a></a><p></p></body></html>".to_string();
        let t = HtmlTokenizer::new(html);
        let window = HtmlParser::new(t).construct_tree();

        let document = window.borrow().document();
        let body = document
            .borrow()
            .first_child()
            .unwrap() // html
            .borrow()
            .first_child()
            .unwrap() // head
            .borrow()
            .next_sibling()
            .unwrap();
        assert_eq!(
            body,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "body",
                Vec::new()
            )))))
        );

        let h1 = body.borrow().first_child().unwrap();
        let a = h1.borrow().next_sibling().unwrap();
        let p = a.borrow().next_sibling().unwrap();

        assert_eq!(
            h1,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "h1",
                Vec::new()
            )))))
        );
        assert_eq!(
            a,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "a",
                Vec::new()
            )))))
        );
        assert_eq!(
            p,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "p",
                Vec::new()
            )))))
        );

        assert!(a.borrow().previous_sibling().ptr_eq(&Rc::downgrade(&h1)));
        assert!(p.borrow().previous_sibling().ptr_eq(&Rc::downgrade(&a)));
    }

    #[test]
    fn text_next_and_previous_siblings_with_text() {
        let html = "<html><head></head><body><a></a>hello<p></p></body></html>".to_string();
        let t = HtmlTokenizer::new(html);
        let window = HtmlParser::new(t).construct_tree();

        let document = window.borrow().document();
        let body = document
            .borrow()
            .first_child()
            .unwrap() // html
            .borrow()
            .first_child()
            .unwrap() // head
            .borrow()
            .next_sibling()
            .unwrap();
        assert_eq!(
            body,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "body",
                Vec::new()
            )))))
        );

        let a = body.borrow().first_child().unwrap();
        let hello = a.borrow().next_sibling().unwrap();
        let p = hello.borrow().next_sibling().unwrap();

        assert_eq!(
            a,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "a",
                Vec::new()
            )))))
        );
        assert_eq!(
            hello,
            Rc::new(RefCell::new(Node::new(NodeKind::Text("hello".to_string()))))
        );
        assert_eq!(
            p,
            Rc::new(RefCell::new(Node::new(NodeKind::Element(Element::new(
                "p",
                Vec::new()
            )))))
        );

        assert!(hello.borrow().previous_sibling().ptr_eq(&Rc::downgrade(&a)));
        assert!(p.borrow().previous_sibling().ptr_eq(&Rc::downgrade(&hello)));
    }
}
