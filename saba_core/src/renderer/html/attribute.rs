use alloc::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct Attribute {
    name: String,
    value: String,
}

impl Attribute {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
        }
    }

    pub fn add_char(&mut self, c: char, is_name: bool) {
        match is_name {
            true => self.name.push(c),
            false => self.value.push(c),
        }
    }
}
