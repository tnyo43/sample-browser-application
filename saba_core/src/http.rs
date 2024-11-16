use alloc::string::{String, ToString};

pub struct HttpResponse {}

impl HttpResponse {
    pub fn new(raw_response: String) -> Result<Self, String> {
        Err("Error".to_string())
    }
}
