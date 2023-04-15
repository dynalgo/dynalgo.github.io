pub struct Node {
    name: char,
    value: Option<u8>,
}

impl Node {
    pub fn new(name: char, value: Option<u8>) -> Node {
        Node { name, value }
    }

    pub fn name(&self) -> char {
        self.name
    }

    pub fn value(&self) -> Option<u8> {
        self.value
    }
}
