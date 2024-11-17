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

enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    ScriptData,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEmdTagName,
    TemporaryBuffer,
}

pub struct HtmlTokenizer {
    input: Vec<char>,
    pos: usize,
    state: State,
    re_consume: bool,
    latest_token: Option<HtmlToken>,
}

impl HtmlTokenizer {
    fn consume_next_input(&mut self) -> char {
        let c = self.input[self.pos];
        self.pos += 1;
        c
    }

    fn is_eof(&self) -> bool {
        self.pos == self.input.len()
    }

    fn create_start_tag(&mut self) {
        self.latest_token = Some(HtmlToken::StartTag {
            tag: String::new(),
            self_closing: false,
            attributes: Vec::new(),
        });
    }
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        };

        loop {
            let c = self.consume_next_input();

            match self.state {
                State::Data => {
                    if c == '<' {
                        self.state = State::TagOpen;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    return Some(HtmlToken::Char(c));
                }
                State::TagOpen => {
                    if c == '/' {
                        self.state = State::EndTagOpen;
                        continue;
                    }

                    if c.is_ascii_alphabetic() {
                        self.re_consume = true;
                        self.state = State::TagName;
                        self.create_start_tag();
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.re_consume = true;
                    self.state = State::Data;
                }
                _ => {
                    return None;
                }
            }
        }
    }
}
