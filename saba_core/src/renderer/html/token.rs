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
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            input: html.chars().collect(),
            pos: 0,
            state: State::Data,
            re_consume: false,
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
                        todo!("")
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    return Some(HtmlToken::Char(c));
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
}
