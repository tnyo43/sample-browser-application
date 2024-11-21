use alloc::string::String;

#[derive(Clone, PartialEq, Debug, Eq)]
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

    pub fn add_name_char(&mut self, c: char) {
        self.name.push(c);
    }

    pub fn add_value_char(&mut self, c: char) {
        self.value.push(c);
    }
}
