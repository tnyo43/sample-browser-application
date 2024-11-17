use alloc::{string::String, vec::Vec};

#[derive(Clone, PartialEq, Debug)]
pub enum HtmlToken {
    StartTag { tag: String, self_closing: bool },
    EndTag { tag: String },
    Char(char),
    Eof,
}

#[derive(PartialEq)]
enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
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
                }
                | HtmlToken::EndTag { ref mut tag } => {
                    tag.push(c);
                }
                _ => panic!("`last_token` should be either StartTag or EndTag"),
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
                        todo!("");
                        continue;
                    }
                    if c == '/' {
                        todo!("implement self closing tags");
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
                _ => return None,
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
            })
        );
        assert_eq!(
            tokenizer.next(),
            Some(HtmlToken::EndTag {
                tag: "body".to_string()
            })
        );
    }
}
