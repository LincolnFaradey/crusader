mod chat;

use chat::message::{Message, Kind};
use chat::chatter::Postman;
use std::io;

const HOST: &'static str = "127.0.0.1:8080";

fn main() {
    let mut pm = Postman::new(HOST).unwrap();

    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line)
            .expect("Failed to read line");

        let m = Message::new(Kind::Text, &line.into_bytes());
        pm.send(m);
        

        let v = pm.receive();
        print!("{}", v.to_string());
    }
}
