use std::char::REPLACEMENT_CHARACTER;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

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

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let incoming = String::from_utf8_lossy(&buffer[..]);
    let body: &str = incoming.split("\r\n\r\n").nth(1).expect("No body sent!!!");

    let message = Message {
        kind: Kind::Todo,
        content: &incoming,
    };
    println!("Request: {:?}", message);
    println!("parts: {:?}", body);
    let contents = "Recorded!".to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
