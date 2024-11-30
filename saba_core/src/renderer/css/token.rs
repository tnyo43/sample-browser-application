use alloc::{string::String, vec::Vec};

#[derive(PartialEq, Debug)]
pub enum CssToken {
    HashToken(String),
    Delim(char),
    Number(f64),
    Colon,
    SemiColon,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurly,
    CloseCurly,
    Ident(String),
    StringToken(String),
    AtKeyword(String),
}

pub struct CssTokenizer {
    pos: usize,
    input: Vec<char>,
}

impl CssTokenizer {
    pub fn new(css: String) -> Self {
        Self {
            pos: 0,
            input: css.chars().collect(),
        }
    }

    fn consume_string_token(&mut self) -> String {
        self.pos += 1;

        let mut s = String::new();

        loop {
            if self.pos >= self.input.len() {
                return s;
            }

            let c = self.input[self.pos];
            match c {
                '"' | '\'' => break,
                c => s.push(c),
            }

            self.pos += 1;
        }

        s
    }

    fn consume_numeric_token(&mut self) -> f64 {
        let mut n = 0f64;
        let mut is_floating = false;
        let mut floating_digit = 1f64;

        loop {
            if self.pos >= self.input.len() {
                return n;
            }

            let c = self.input[self.pos];
            match c {
                '0'..='9' => {
                    let d = c.to_digit(10).unwrap() as f64;
                    if is_floating {
                        floating_digit /= 10f64;
                        n += d * floating_digit;
                    } else {
                        n = n * 10f64 + d;
                    }
                }
                '.' => {
                    is_floating = true;
                }
                _ => {
                    self.pos -= 1;
                    break;
                }
            }

            self.pos += 1;
        }

        n
    }
}

impl Iterator for CssTokenizer {
    type Item = CssToken;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pos >= self.input.len() {
                return None;
            }

            let token = match self.input[self.pos] {
                '(' => CssToken::OpenParenthesis,
                ')' => CssToken::CloseParenthesis,
                ',' => CssToken::Delim(','),
                '.' => CssToken::Delim('.'),
                ':' => CssToken::Colon,
                ';' => CssToken::SemiColon,
                '{' => CssToken::OpenCurly,
                '}' => CssToken::CloseCurly,
                ' ' | '\n' => {
                    self.pos += 1;
                    continue;
                }
                '"' | '\'' => CssToken::StringToken(self.consume_string_token()),
                '0'..='9' => CssToken::Number(self.consume_numeric_token()),
                _ => {
                    todo!()
                }
            };
            self.pos += 1;

            return Some(token);
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn parse_symbols() {
        let css = "(),.:;{}".to_string();
        let mut tokenizer = CssTokenizer::new(css);

        assert_eq!(tokenizer.next(), Some(CssToken::OpenParenthesis));
        assert_eq!(tokenizer.next(), Some(CssToken::CloseParenthesis));
        assert_eq!(tokenizer.next(), Some(CssToken::Delim(',')));
        assert_eq!(tokenizer.next(), Some(CssToken::Delim('.')));
        assert_eq!(tokenizer.next(), Some(CssToken::Colon));
        assert_eq!(tokenizer.next(), Some(CssToken::SemiColon));
        assert_eq!(tokenizer.next(), Some(CssToken::OpenCurly));
        assert_eq!(tokenizer.next(), Some(CssToken::CloseCurly));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn parse_ignore_space_and_new_line() {
        let css = "( \n)".to_string();
        let mut tokenizer = CssTokenizer::new(css);

        assert_eq!(tokenizer.next(), Some(CssToken::OpenParenthesis));
        assert_eq!(tokenizer.next(), Some(CssToken::CloseParenthesis));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn parse_string_token() {
        let css = "\'hello\' \"world\"".to_string();
        let mut tokenizer = CssTokenizer::new(css);

        assert_eq!(
            tokenizer.next(),
            Some(CssToken::StringToken("hello".to_string()))
        );
        assert_eq!(
            tokenizer.next(),
            Some(CssToken::StringToken("world".to_string()))
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn parse_number() {
        let css = "123 45.67".to_string();
        let mut tokenizer = CssTokenizer::new(css);

        assert_eq!(tokenizer.next(), Some(CssToken::Number(123f64)));
        assert_eq!(tokenizer.next(), Some(CssToken::Number(45.67f64)));
        assert_eq!(tokenizer.next(), None);
    }
}
