mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_works() {
        let a = Parser::new();
        
        match a {
            Ok(a) => {
                assert!(true);
            },
            Err(e) => {
                println!("{:?}", e);
            },
        }
    }
}
