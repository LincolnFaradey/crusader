mod chat;

use chat::message::Message;
use chat::kind::Kind;
use chat::chatter::Postman;
use std::io;

// const HOST: &'static str = "178.62.229.44:8080";
const HOST: &'static str = "127.0.0.1:8080";

fn main() {
    let mut pm = Postman::new(HOST).unwrap();

    loop {
        println!("Please Enter some text:");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let msg = Message::new(Kind::Text, &line.into_bytes());
        pm.send(msg);

        let msg = pm.receive();

        match msg.kind() {
            &Kind::Text => {
                println!("@Response:\n\t{}", msg.to_string());
            },
            &Kind::File => {
                // TODO: needs to be implemented 
                // - [header[kind][filename length][filename]][----FILE----][filename] 
            },
            &Kind::Info => {
                println!("\t\t==Info: =={}==", msg.to_string());
            },
            &Kind::Debug => {
                // TODO: needs to be implemented
                // if the -d flag passed to the program
                // Degug should be written in a file
                println!("\t\t||Debug: ||{}||", msg.to_string()); 
            }
        }
    }
}
