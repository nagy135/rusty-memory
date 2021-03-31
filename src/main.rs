use regex::{Captures, Regex};
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
    let mut request_iter = incoming.split("\r\n\r\n").take(2);
    let header: &str = request_iter.next().expect("No header sent!");
    let body_with_padding: &str = request_iter.next().expect("No body sent!");

    // get body {{{
    let re = Regex::new(r"Content-Length: (\d*)").unwrap();
    let length: Captures = re.captures_iter(header).nth(0).unwrap();
    let length: &str = &length[1];
    let length: usize = length.parse().expect("NAN Content-Type length");
    let (body, _) = body_with_padding.split_at(length);
    // }}}

    let mut body_iter = body.split("::").take(2);
    let message_type: &str = body_iter.next().expect("No type sent!");
    let message_content: &str = body_iter.next().expect("No content sent!");

    let message_kind = match message_type {
        "type:note" => Kind::Note,
        _ => Kind::Todo,
    };
    let message = Message {
        kind: message_kind,
        content: &message_content,
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
}
