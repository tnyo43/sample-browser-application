use alloc::{string::String, vec::Vec};

use super::attribute::Attribute;

#[derive(Clone)]
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

    fn re_consume_input(&mut self) -> char {
        self.re_consume = false;
        self.input[self.pos - 1]
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

    fn create_end_tag(&mut self) {
        self.latest_token = Some(HtmlToken::EndTag { tag: String::new() })
    }

    fn append_tag_name(&mut self, c: char) {
        assert!(self.latest_token.is_some());

        if let Some(token) = self.latest_token.as_mut() {
            match token {
                HtmlToken::StartTag {
                    ref mut tag,
                    self_closing: _,
                    attributes: _,
                }
                | HtmlToken::EndTag { ref mut tag } => tag.push(c),
                _ => panic!("`latest_token` should be either StartTag or EndTag"),
            }
        }
    }

    fn take_latest_token(&mut self) -> Option<HtmlToken> {
        assert!(self.latest_token.is_some());

        let token = self.latest_token.as_ref().cloned();
        self.latest_token = None;

        token
    }

    fn start_new_attribute(&mut self) {
        assert!(self.latest_token.is_some());

        if let Some(token) = self.latest_token.as_mut() {
            match token {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    attributes.push(Attribute::new());
                }
                _ => panic!("`latest_token` should be either StartTag"),
            }
        }
    }
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        };

        loop {
            let c = match self.re_consume {
                true => self.re_consume_input(),
                false => self.consume_next_input(),
            };

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
                State::EndTagOpen => {
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    if c.is_alphabetic() {
                        self.re_consume = true;
                        self.state = State::TagName;
                        self.create_end_tag();
                        continue;
                    }
                }
                State::TagName => {
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }

                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }

                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if c.is_ascii_uppercase() {
                        self.append_tag_name(c.to_ascii_lowercase());
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_tag_name(c);
                }
                State::BeforeAttributeName => {
                    self.re_consume = true;

                    if c == '/' || c == '>' || self.is_eof() {
                        self.state = State::AfterAttributeName;
                        continue;
                    }

                    self.state = State::AttributeName;
                    self.start_new_attribute();
                }
                _ => {
                    return None;
                }
            }
        }
    }
}
