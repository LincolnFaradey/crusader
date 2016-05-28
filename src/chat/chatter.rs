extern crate byteorder;

use chat::message::{Message, Kind};
use std::net::TcpStream;
use std::io::prelude::*;
use std::error;
use self::byteorder::{ByteOrder, BigEndian};

const HEADER_SIZE: usize = 8;
const KIND_SIZE: usize = 1;

#[derive(Debug)]
pub struct Postman<'a> {
    addr: &'a str,
    stream: TcpStream
}

impl<'a> Postman<'a> {
    pub fn new(addr: &'a str) -> Result<Postman, Box<error::Error>> {
    	match TcpStream::connect(addr) {
    		Ok(stm) => {
    			let pm =  Postman {   
                    addr: addr,
                    stream: stm,
                                 };
    			 Ok(pm)
    		},
    		Err(err) => Err( Box::new(err) )
    	}
    }

    pub fn send(&mut self, msg: Message) {
        let mut buf = vec!();

        //Building a message packet
        buf.extend(msg.get_header().iter().cloned());
        buf.push(msg.get_u8_kind());
        buf.extend(msg.get_content().iter().cloned());

		self.stream.write_all(buf.as_slice()).unwrap();
	}

	pub fn receive(&mut self) -> Message {
		let mut stream_ref = &mut self.stream;
		 // First byte returns the size for following data
        let mut buf = [0u8; HEADER_SIZE];
        stream_ref.take(HEADER_SIZE as u64).read(&mut buf).unwrap();

        // Getting type of the message
        let mut tp = [0u8; KIND_SIZE];
        stream_ref.take(KIND_SIZE as u64).read(&mut tp).unwrap();

        let  kind = match tp[0]{
            0 => Kind::Text,
            _ => Kind::File,
        };
        println!("Message Kind: {:?}", kind);

        let len = BigEndian::read_u64(&buf);
        println!("Recieved Message with Length: {:?}", len);

        let mut buf = vec!();
        stream_ref.take(len as u64).read_to_end(&mut buf).unwrap();

        Message::new(kind, &buf)
	}
}
