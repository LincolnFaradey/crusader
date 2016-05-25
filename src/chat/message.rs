extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};

const SIZE: usize = 8;

#[derive(Debug)]
pub enum Kind {
    Text = 0,
    File,
}

#[derive(Debug)]
pub struct Message {
    kind: Kind,
    header: [u8; SIZE],
    content: Vec<u8>,
}

impl Message {
    pub fn new(kind: Kind, content: &Vec<u8>) -> Message {
    	let mut buf = [0u8; SIZE];
        println!("{:?}", Kind::File as i32);
    	BigEndian::write_u64(&mut buf, (content.len() as u64));
    	return Message {
            kind: kind,
    		header: buf,
    		content: content.clone(),
    	};
    }

    pub fn get_content(self) -> Vec<u8> {
        return self.content.clone()
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
    	return match String::from_utf8(self.content.clone()) {
    		Ok(val) => val,
    		Err(_) => String::from(""),
    	};
    }
}