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

    // separate header and body {{{
    let mut request_iter = incoming.split("\r\n\r\n").take(2);
    let header: &str = match request_iter.next() {
        Some(header) => header,
        None => {
            return Err("Header not found!");
        }
    };
    let body_with_padding: &str = match request_iter.next() {
        Some(header) => header,
        None => {
            return Err("Body not found!");
        }
    };
    // }}}

    // clean up body {{{
    let re = Regex::new(r"Content-Length: (\d*)").unwrap();
    let length: Captures = re.captures_iter(header).nth(0).unwrap();
    let length: &str = &length[1];
    let length: usize = match length.parse() {
        Ok(length) => length,
        Err(_) => return Err("NAN Content-Type length"),
    };
    let (body, _) = body_with_padding.split_at(length);
    // }}}

    let mut body_iter = body.split("::").take(2);
    let message_type: &str = match body_iter.next() {
        Some(message_type) => message_type,
        None => return Err("Type not sent!"),
    };
    let message_content: &str = match body_iter.next() {
        Some(message_content) => message_content,
        None => return Err("Content not sent!"),
    };

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
    Ok("")
}
