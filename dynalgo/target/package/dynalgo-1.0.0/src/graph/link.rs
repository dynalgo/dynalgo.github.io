#[derive(Copy, Clone)]
pub struct Link {
    name: char,
    from: char,
    to: char,
    bidirect: bool,
    value: Option<u8>,
}

impl Link {
    pub fn new(name: char, from: char, to: char, bidirect: bool, value: Option<u8>) -> Link {
        Link {
            name,
            from,
            to,
            bidirect,
            value,
        }
    }

    pub fn name(&self) -> char {
        self.name
    }

    pub fn from(&self) -> char {
        self.from
    }

    pub fn to(&self) -> char {
        self.to
    }

    pub fn bidirect(&self) -> bool {
        self.bidirect
    }

    pub fn value(&self) -> Option<u8> {
        self.value
    }
}
