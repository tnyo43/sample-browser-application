use core::cell::RefCell;

use alloc::{rc::Rc, vec::Vec};

use crate::renderer::dom::node::{Node, Window};

use super::token::{HtmlToken, HtmlTokenizer};

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
                InsertionMode::BeforeHtml => todo!(),
                InsertionMode::BeforeHead => todo!(),
                InsertionMode::InHead => todo!(),
                InsertionMode::AfterHead => todo!(),
                InsertionMode::InBody => todo!(),
                InsertionMode::Text => todo!(),
                InsertionMode::AfterBody => todo!(),
                InsertionMode::AfterAfterBody => todo!(),
            }
        }

        self.window.clone()
    }
}
