use alloc::string::String;

#[derive(Debug)]
pub enum Error {
    Network(String),
    InvalidUI(String),
    UnexpectedInput(String),
    Other(String),
}
