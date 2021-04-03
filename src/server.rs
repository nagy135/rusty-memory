use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::parser;

const URL: &'static str = "127.0.0.1";
const PORT: &'static str = "7878";

#[derive(Debug)]
enum Kind {
    Todo,
    Note,
}

#[derive(Debug)]
struct Message<'a> {
    kind: Kind,
    content: &'a str,
}

pub fn create_server() {
    let address: &str = &format!("{}:{}", URL, PORT);
    let listener = TcpListener::bind(address).unwrap();
    println!("Running server on {}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        match handle_connection(stream) {
            Ok(_) => {}
            Err(reason) => println!("Client request crashed: {}", reason),
        }
    }
}

fn handle_connection<'a>(mut stream: TcpStream) -> Result<&'a str, &'a str> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let incoming = String::from_utf8_lossy(&buffer[..]);

    let key_vals: Vec<parser::KeyVal> = parser::parse_request(incoming.into_owned())?;
    println!("key_vals {:?}", key_vals[0].key);

    let message_kind = match key_vals[0].value.as_str() {
        "note" => Kind::Note,
        _ => Kind::Todo,
    };
    let message = Message {
        kind: message_kind,
        content: &key_vals[1].value,
    };
    println!("message: {:?}", message);

    // Respond {{{
    let contents = "Recorded!".to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // }}}
    Ok("")
}
