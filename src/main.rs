mod parser;
mod server;

fn main() {
    server::create_server();
}

#[cfg(test)]
mod tests {
    use super::parser;
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }
}
