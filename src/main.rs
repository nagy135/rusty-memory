mod json;
mod server;

fn main() {
    server::create_server();
}

#[cfg(test)]
mod tests {
    use super::json;
    #[test]
    fn obj() {
        assert_eq!(
            json::parse("{\"key\": \"val\"}"),
            json::Object {
                key_vals: vec![Box::new(json::KeyVal {
                    key: "key",
                    val: Box::new("val")
                })]
            }
        );
        ()
    }
}
