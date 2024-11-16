use alloc::string::String;

#[derive(Debug)]
pub enum Error {
    Network(String),
    Other(String),
}
