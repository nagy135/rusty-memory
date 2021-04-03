use regex::{Captures, Regex};

#[derive(Debug)]
pub struct KeyVal {
    pub key: String,
    pub value: String,
}

impl KeyVal {
    pub fn new(key: String, value: String) -> KeyVal {
        KeyVal { key, value }
    }
}

pub fn parse_request<'b>(incoming: String) -> Result<Vec<KeyVal>, &'b str> {
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
    let length: Captures = match re.captures_iter(header).next() {
        Some(length) => length,
        None => return Err("No Content-Length sent!"),
    };
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
        None => return Err("Type not sent! (before ::)"),
    };

    let mut type_iter = message_type.split(":").take(2);
    let type_type: &str = match type_iter.next() {
        Some(type_type) => type_type,
        None => return Err("Type key not sent!"),
    };
    let type_content: &str = match type_iter.next() {
        Some(type_content) => type_content,
        None => return Err("Type content not sent!"),
    };

    let message_content: &str = match body_iter.next() {
        Some(message_content) => message_content,
        None => return Err("Content not sent! (after ::)"),
    };

    Ok(vec![
        KeyVal::new(type_type.to_string(), type_content.to_string()),
        KeyVal::new("content".to_string(), message_content.to_string()),
    ])
}
