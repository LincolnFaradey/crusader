mod chat;

use chat::message::{Message, Kind};
use chat::chatter::Postman;
use std::io;

// const HOST: &'static str = "178.62.229.44:8080";
const HOST: &'static str = "178.62.229.44:8080";

fn main() {
    let mut pm = Postman::new(HOST).unwrap();

    loop {
        println!("Please Enter some text:");

        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");

        let msg = Message::new(Kind::Text, &line.into_bytes());
        pm.send(msg);
        
        let v = pm.receive();
        print!("{}", v.to_string());
    }
}
