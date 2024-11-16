use crate::error::Error;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Result<Self, Error> {
        let preprocessed_response = raw_response.trim_start().replace("\n\r", "\n");
        let (status_line, _remaining) = match preprocessed_response.split_once('\n') {
            Some((s, r)) => (s, r),
            None => {
                return Err(Error::Network(format!(
                    "invalid http response: {}",
                    preprocessed_response
                )));
            }
        };

        let statuses: Vec<&str> = status_line.split(' ').collect();

        Ok(Self {
            version: statuses[0].to_string(),
            status_code: statuses[1].parse().unwrap_or(404),
            reason: statuses[2].to_string(),
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
}
