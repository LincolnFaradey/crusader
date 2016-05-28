extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};
use chat::kind::Kind;

const SIZE: usize = 8;

#[derive(Debug)]
pub struct Message {
    kind: Kind,
    header: [u8; SIZE],
    content: Vec<u8>,
}

impl Message {
    pub fn new(kind: Kind, content: &Vec<u8>) -> Message {
        let mut header = [0u8; SIZE];
        BigEndian::write_u64(&mut header, (content.len() as u64));

        Message {
            kind: kind,
            header: header,
            content: content.clone(),
        }
    }

    pub fn content(&self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn header(&self) -> [u8; SIZE] {
        self.header.clone()
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        match String::from_utf8(self.content.clone()) {
            Ok(val) => val,
            Err(_) => String::from(""),
        }
    }
}
