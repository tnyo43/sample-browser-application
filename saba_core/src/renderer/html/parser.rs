use super::token::HtmlTokenizer;

pub struct HtmlParser {
    t: HtmlTokenizer,
}

impl HtmlParser {
    pub fn new(t: HtmlTokenizer) -> Self {
        Self { t }
    }
}
