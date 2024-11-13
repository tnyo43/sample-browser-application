use alloc::string::String;

pub struct Url {
    url: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
