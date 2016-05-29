
#[derive(Debug)]
pub enum Kind {
    Text = 0,
    File,
    Info,
    Debug,
}

impl Kind {
    pub fn from_u8(byte: u8) -> Kind {
        match byte {
            0 => Kind::Text,
            1 => Kind::File,
            2 => Kind::Info,
            _ => Kind::Debug,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            &Kind::Text => 0u8,
            &Kind::File => 1u8,
            &Kind::Info => 2u8,
            &Kind::Debug => 3u8,
        }
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Kind) -> bool {
        self.to_u8() == other.to_u8()
    }
}
