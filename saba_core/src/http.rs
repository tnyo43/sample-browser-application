use crate::error::Error;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<Header>,
    body: String,
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Result<Self, Error> {
        let preprocessed_response = raw_response.trim_start().replace("\n\r", "\n");
        let (status_line, remaining) = match preprocessed_response.split_once('\n') {
            Some((s, r)) => (s, r),
            None => {
                return Err(Error::Network(format!(
                    "invalid http response: {}",
                    preprocessed_response
                )));
            }
        };

        let statuses: Vec<&str> = status_line.split(' ').collect();

        let (headers, body) = match remaining.split_once("\n\n") {
            Some((h, body)) => {
                let mut headers = Vec::new();
                for header in h.split('\n') {
                    let splitted_header: Vec<&str> = header.splitn(2, ':').collect();
                    headers.push(Header::new(
                        String::from(splitted_header[0].trim()),
                        String::from(splitted_header[1].trim()),
                    ));
                }

                (headers, body)
            }
            None => (Vec::new(), remaining.trim_start_matches("\n")),
        };

        Ok(Self {
            version: statuses[0].to_string(),
            status_code: statuses[1].parse().unwrap_or(404),
            reason: statuses[2].to_string(),
            headers,
            body: body.to_string(),
        })
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn status_code(&self) -> u32 {
        self.status_code
    }

    pub fn reason(&self) -> String {
        self.reason.clone()
    }

    pub fn headers(&self) -> Vec<Header> {
        self.headers.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn header_value(&self, name: &str) -> Result<String, String> {
        for h in &self.headers {
            if h.name == name {
                return Ok(h.value.clone());
            }
        }

        Err(format!("failed to find {} in headers", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_error_if_response_does_not_contain_newline() {
        let raw = "HTTP/1.1 200 OK".to_string();
        let result = HttpResponse::new(raw);

        assert!(result.is_err());
    }

    #[test]
    fn parse_status_line_correctly() {
        let raw = "HTTP/1.1 200 OK\n\n".to_string();
        let result = HttpResponse::new(raw).unwrap();

        assert_eq!(result.version(), "HTTP/1.1");
        assert_eq!(result.status_code(), 200);
        assert_eq!(result.reason(), "OK");
    }

    #[test]
    fn status_code_should_be_404_when_invalid_number_comes() {
        let raw = "HTTP/1.1 xxx OK\n\n".to_string();
        let result = HttpResponse::new(raw).unwrap();

        assert_eq!(result.status_code(), 404);
    }

    mod parse_headers_and_body {
        use super::*;

        #[test]
        fn should_be_empty_vec_if_no_header_is_given() {
            let raw = "HTTP/1.1 xxx OK\n\nhello world".to_string();
            let result = HttpResponse::new(raw).unwrap();

            assert_eq!(result.headers(), Vec::new());
            assert_eq!(result.body(), "hello world".to_string());
        }

        #[test]
        fn should_parse_list_of_headers() {
            let raw = "HTTP/1.1 xxx OK\nHost: localhost:80  \n User-Agent : Mozilla/5.0\nAccept: text/html, application/xhtml+xml\nContent-Length:11\n\nhello world".to_string();
            let result = HttpResponse::new(raw).unwrap();

            assert_eq!(
                result.headers(),
                Vec::from([
                    Header {
                        name: "Host".to_string(),
                        value: "localhost:80".to_string(),
                    },
                    Header {
                        name: "User-Agent".to_string(),
                        value: "Mozilla/5.0".to_string(),
                    },
                    Header {
                        name: "Accept".to_string(),
                        value: "text/html, application/xhtml+xml".to_string(),
                    },
                    Header {
                        name: "Content-Length".to_string(),
                        value: "11".to_string(),
                    }
                ])
            );
            assert_eq!(result.body(), "hello world".to_string());
        }

        #[test]
        fn header_value_can_be_obtained() {
            let raw = "HTTP/1.1 xxx OK\nHost: localhost:80  \n User-Agent : Mozilla/5.0\nAccept: text/html, application/xhtml+xml\nContent-Length:11\n\nhello world".to_string();
            let header_response = HttpResponse::new(raw).unwrap();

            assert_eq!(
                header_response.header_value("Host"),
                Ok("localhost:80".to_string())
            );
            assert_eq!(
                header_response.header_value("User-Agent"),
                Ok("Mozilla/5.0".to_string())
            );
            assert_eq!(
                header_response.header_value("Accept"),
                Ok("text/html, application/xhtml+xml".to_string())
            );
            assert_eq!(
                header_response.header_value("Content-Length"),
                Ok("11".to_string())
            );
            assert_eq!(
                header_response.header_value("Connection"),
                Err("failed to find Connection in headers".to_string())
            );
        }
    }
}
