mod parser;

#[cfg(test)]
mod tests {
    pub use crate::parser::AST;
    
    #[test]
    fn test_var() {
        let ast = AST::from_string("local varName1 = 100");
    }


}
