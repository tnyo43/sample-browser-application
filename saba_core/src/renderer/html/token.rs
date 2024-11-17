use alloc::{string::String, vec::Vec};

use super::attribute::Attribute;

pub enum HtmlToken {
    StartTag {
        tag: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
    },
    EndTag {
        tag: String,
    },
    Char(char),
    Eof,
}

pub struct HtmlTokenizer {}
