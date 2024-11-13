use alloc::string::{String, ToString};

#[derive(Debug, PartialEq, Clone)]
pub struct Url {
    url: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    fn is_http(&self) -> bool {
        if self.url.starts_with("http://") {
            return true;
        }
        false
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported".to_string());
        }

        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_function_checks_if_scheme_is_http {
        use super::*;

        #[test]
        fn return_error_if_url_starts_other_than_http() {
            assert_eq!(
                Url::new("hello world".to_string()).parse(),
                Err("Only HTTP scheme is supported".to_string())
            );
            assert_eq!(
                Url::new("https://example.com".to_string()).parse(),
                Err("Only HTTP scheme is supported".to_string())
            );
        }

        #[test]
        fn return_url_object_if_url_starts_with_http() {
            let url = "http://example.com".to_string();
            let expected = Ok(Url {
                url: "http://example.com".to_string(),
            });
            assert_eq!(Url::new(url).parse(), expected)
        }
    }
}
