use std::fmt;

trait JsonPart: Sized {
    fn repr(&self) -> String {
        "haha".to_string()
    }
}

#[derive(Debug)]
pub struct Array {}

#[derive(Debug, PartialEq)]
pub struct Object<'a> {
    key_vals: Vec<Box<KeyVal<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct KeyVal<'a> {
    key: &'a str,
    val: Box<JsonPart>,
}

impl JsonPart for Array {}
impl JsonPart for Object<'_> {}
impl JsonPart for KeyVal<'_> {}
impl JsonPart for &str {}

impl fmt::Debug for dyn JsonPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hi")
    }
}

pub fn parse(data: &str) -> Object {
    println!("ahahahah {}", data);
    Object {
        key_vals: vec![Box::new(KeyVal {
            key: "key",
            val: Box::new("val"),
        })],
    }
}
