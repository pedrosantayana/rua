mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn test_var() {
        let a = parser::LuaParser::parse();
    }
}
