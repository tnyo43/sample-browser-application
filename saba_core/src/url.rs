use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
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

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(":") {
            url_parts[0][index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_search_parts: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        path_and_search_parts[0].to_string()
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        Ok(self.clone())
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }
    pub fn port(&self) -> String {
        self.port.clone()
    }
    pub fn path(&self) -> String {
        self.path.clone()
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
                port: "80".to_string(),
                path: "".to_string(),
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

    mod extract_port_from_url {
        use super::*;

        #[test]
        fn should_be_80_by_default_if_port_is_not_specified() {
            assert_eq!(
                Url::new("http://example.com".to_string())
                    .parse()
                    .unwrap()
                    .port(),
                "80".to_string()
            );
        }

        #[test]
        fn should_be_the_after_the_colon() {
            assert_eq!(
                Url::new("http://example.com:200".to_string())
                    .parse()
                    .unwrap()
                    .port(),
                "200".to_string()
            );
            assert_eq!(
                Url::new("http://github.com:8080/foo/bar?page=2&order=asc".to_string())
                    .parse()
                    .unwrap()
                    .port(),
                "8080".to_string()
            );
        }
    }

    #[test]
    fn extract_path_from_url() {
        assert_eq!(
            Url::new("http://example.com".to_string())
                .parse()
                .unwrap()
                .path(),
            "".to_string()
        );
        assert_eq!(
            Url::new("http://github.com:8080/foo/bar?page=2&order=asc".to_string())
                .parse()
                .unwrap()
                .path(),
            "foo/bar".to_string()
        );
    }
}
