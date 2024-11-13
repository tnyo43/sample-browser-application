use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Url {
    url: String,
    host: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
        }
    }

    fn is_http(&self) -> bool {
        if self.url.starts_with("http://") {
            return true;
        }
        false
    }

    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(":") {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported".to_string());
        }

        self.host = self.extract_host();
        Ok(self.clone())
    }

    pub fn host(&self) -> String {
        self.host.clone()
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
                host: "example.com".to_string(),
            });
            assert_eq!(Url::new(url).parse(), expected)
        }
    }

    #[test]
    fn extract_host_from_url() {
        assert_eq!(
            Url::new("http://example.com".to_string())
                .parse()
                .unwrap()
                .host(),
            "example.com".to_string()
        );
        assert_eq!(
            Url::new("http://github.com:8080/foo/bar?page=2&order=asc".to_string())
                .parse()
                .unwrap()
                .host(),
            "github.com".to_string()
        );
    }
}
