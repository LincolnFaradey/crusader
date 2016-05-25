extern crate byteorder;

use chat::message::{Message, Kind};
use std::net::TcpStream;
use std::io::prelude::*;
use std::error;
use self::byteorder::{ByteOrder, BigEndian};

const HEADER_SIZE: usize = 8;

#[derive(Debug)]
pub struct Postman<'a> {
    addr: &'a str,
    stream: TcpStream
}

impl<'a> Postman<'a> {
    pub fn new(addr: &'a str) -> Result<Postman, Box<error::Error>> {
    	match TcpStream::connect(addr) {
    		Ok(stm) => {
    			let pm =  Postman {   addr: addr,
    			  				 	stream: stm,
    			  				};
    			 Ok(pm)
    		},
    		Err(err) => Err( Box::new(err) )
    	}
    }

    pub fn send(&mut self, msg: Message) {
		self.stream.write_all(msg.get_content().as_slice()).unwrap();
	}

	pub fn receive(&mut self) -> Message {
		let mut s_ref = &mut self.stream;
		 // First byte returns the size for following data
        let mut buf = [0u8; HEADER_SIZE];
        s_ref.take(buf.len() as u64).read(&mut buf).unwrap();

        let l = BigEndian::read_u64(&buf);

        let mut buf = vec!();
        s_ref.take(l as u64).read_to_end(&mut buf).unwrap();

        return Message::new(Kind::Text, &buf);
	}
}
