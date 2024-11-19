use crate::renderer::html::attribute::Attribute;
use alloc::{string::String, vec::Vec};

#[derive(Clone, PartialEq, Debug)]
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

#[derive(PartialEq)]
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
}

pub struct HtmlTokenizer {
    input: Vec<char>,
    pos: usize,
    state: State,
    re_consume: bool,
    last_token: Option<HtmlToken>,
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            input: html.chars().collect(),
            pos: 0,
            state: State::Data,
            re_consume: false,
            last_token: None,
        }
    }

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
        self.pos > self.input.len()
    }

    fn create_start_tag(&mut self) {
        self.last_token = Some(HtmlToken::StartTag {
            tag: String::new(),
            self_closing: false,
            attributes: Vec::new(),
        })
    }

    fn create_end_tag(&mut self) {
        self.last_token = Some(HtmlToken::EndTag { tag: String::new() })
    }

    fn take_last_token(&mut self) -> HtmlToken {
        assert!(self.last_token.is_some());

        let token = self.last_token.clone().unwrap();
        self.last_token = None;
        token
    }

    fn append_tag_name(&mut self, c: char) {
        assert!(self.last_token.is_some());

        if let Some(token) = self.last_token.as_mut() {
            match token {
                HtmlToken::StartTag {
                    ref mut tag,
                    self_closing: _,
                    attributes: _,
                }
                | HtmlToken::EndTag { ref mut tag } => {
                    tag.push(c);
                }
                _ => panic!("`last_token` should be either StartTag or EndTag"),
            }
        }
    }

    fn set_self_closing_flag(&mut self) {
        assert!(self.last_token.is_some());

        if let Some(token) = self.last_token.as_mut() {
            match token {
                HtmlToken::StartTag {
                    tag: _,
                    ref mut self_closing,
                    attributes: _,
                } => *self_closing = true,
                _ => panic!("`last_token` should be either StartTag"),
            }
        }
    }

    fn start_new_attribute(&mut self) {
        assert!(self.last_token.is_some());

        if let Some(token) = self.last_token.as_mut() {
            match token {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    attributes.push(Attribute::new());
                }
                _ => panic!("`last_token` should be either StartTag"),
            }
        }
    }

    fn append_attribute_char(&mut self, c: char, is_name: bool) {
        assert!(self.last_token.is_some());

        if let Some(token) = self.last_token.as_mut() {
            match token {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    let len = attributes.len();
                    assert!(len > 0);

                    if is_name {
                        attributes[len - 1].add_name_char(c);
                    } else {
                        attributes[len - 1].add_value_char(c);
                    }
                }
                _ => panic!("`last_token` should be either StartTag"),
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

                    if c.is_ascii_alphabetic() {
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
                        return Some(self.take_last_token());
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_tag_name(c.to_ascii_lowercase());
                }
                State::BeforeAttributeName => {
                    if c == '/' || c == '>' || self.is_eof() {
                        self.re_consume = true;
                        self.state = State::AfterAttributeName;
                        continue;
                    }

                    self.re_consume = true;
                    self.state = State::AttributeName;
                    self.start_new_attribute();
                }
                State::AttributeName => {
                    if c == ' ' || c == '/' || c == '>' || self.is_eof() {
                        self.re_consume = true;
                        self.state = State::AfterAttributeName;
                        continue;
                    }

                    if c == '=' {
                        self.state = State::BeforeAttributeValue;
                        continue;
                    }

                    if c.is_ascii_uppercase() {
                        self.append_attribute_char(c.to_ascii_lowercase(), true);
                        continue;
                    }

                    self.append_attribute_char(c, true);
                }
                State::AfterAttributeName => {
                    if c == ' ' {
                        continue;
                    }

                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }

                    if c == '=' {
                        self.state = State::BeforeAttributeValue;
                        continue;
                    }

                    if c == '>' {
                        self.state = State::Data;
                        return Some(self.take_last_token());
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.re_consume = true;
                    self.state = State::AttributeName;
                    self.start_new_attribute();
                }
                State::BeforeAttributeValue => {
                    if c == ' ' {
                        continue;
                    }

                    if c == '"' {
                        self.state = State::AttributeValueDoubleQuoted;
                        continue;
                    }

                    if c == '\'' {
                        self.state = State::AttributeValueSingleQuoted;
                        continue;
                    }

                    self.re_consume = true;
                    self.state = State::AttributeValueUnquoted;
                }
                State::AttributeValueDoubleQuoted => {
                    if c == '"' {
                        self.state = State::AfterAttributeValueQuoted;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_attribute_char(c, false);
                }
                State::AttributeValueSingleQuoted => {
                    if c == '\'' {
                        self.state = State::AfterAttributeValueQuoted;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_attribute_char(c, false);
                }
                State::AttributeValueUnquoted => {
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }

                    if c == '>' {
                        self.state = State::Data;
                        return Some(self.take_last_token());
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_attribute_char(c, false);
                }
                State::AfterAttributeValueQuoted => {
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
                        return Some(self.take_last_token());
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.re_consume = true;
                    self.state = State::BeforeAttributeName;
                }
                State::SelfClosingStartTag => {
                    if c == '>' {
                        self.set_self_closing_flag();
                        self.state = State::Data;
                        return Some(self.take_last_token());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn parse_html_without_tag() {
        let html = "hello".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('h')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('e')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('l')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('l')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('o')));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn parse_html_with_empty_body_tag() {
        let html = "<body></body>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::StartTag {
                tag: "body".to_string(),
                self_closing: false,
                attributes: Vec::new()
            })
        );
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::EndTag {
                tag: "body".to_string()
            })
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn parse_html_with_self_closing_img_tag() {
        let html = "<img/>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::StartTag {
                tag: "img".to_string(),
                self_closing: true,
                attributes: Vec::new()
            })
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn parse_nested_html() {
        let html = "<body><p>ru<br/>st</p></body>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::StartTag {
                tag: "body".to_string(),
                self_closing: false,
                attributes: Vec::new()
            })
        );
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::StartTag {
                tag: "p".to_string(),
                self_closing: false,
                attributes: Vec::new()
            })
        );
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('r')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('u')));
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::StartTag {
                tag: "br".to_string(),
                self_closing: true,
                attributes: Vec::new()
            })
        );
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('s')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('t')));
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::EndTag {
                tag: "p".to_string()
            })
        );
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::EndTag {
                tag: "body".to_string()
            })
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn parse_tag_with_attributes() {
        let html = "<button disabled><img class=\"x y\" size='16' />click</button>".to_string();

        let mut tokenizer = HtmlTokenizer::new(html);

        let mut disabled_attribute = Attribute::new();
        disabled_attribute.add_name_char('d');
        disabled_attribute.add_name_char('i');
        disabled_attribute.add_name_char('s');
        disabled_attribute.add_name_char('a');
        disabled_attribute.add_name_char('b');
        disabled_attribute.add_name_char('l');
        disabled_attribute.add_name_char('e');
        disabled_attribute.add_name_char('d');

        let mut class_x_y_attribute = Attribute::new();
        class_x_y_attribute.add_name_char('c');
        class_x_y_attribute.add_name_char('l');
        class_x_y_attribute.add_name_char('a');
        class_x_y_attribute.add_name_char('s');
        class_x_y_attribute.add_name_char('s');
        class_x_y_attribute.add_value_char('x');
        class_x_y_attribute.add_value_char(' ');
        class_x_y_attribute.add_value_char('y');

        let mut size_16_attribute = Attribute::new();
        size_16_attribute.add_name_char('s');
        size_16_attribute.add_name_char('i');
        size_16_attribute.add_name_char('z');
        size_16_attribute.add_name_char('e');
        size_16_attribute.add_value_char('1');
        size_16_attribute.add_value_char('6');

        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::StartTag {
                tag: "button".to_string(),
                self_closing: false,
                attributes: Vec::from([disabled_attribute])
            })
        );
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::StartTag {
                tag: "img".to_string(),
                self_closing: true,
                attributes: Vec::from([class_x_y_attribute, size_16_attribute])
            })
        );
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('c')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('l')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('i')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('c')));
        assert_eq!(tokenizer.next(), Some(HtmlToken::Char('k')));
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::EndTag {
                tag: "button".to_string()
            })
        );
        assert_eq!(tokenizer.next(), None);
    }
}
