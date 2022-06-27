pub struct Tokenizer;

pub struct TokenStream(Vec<Token>);

struct SourceStream(Vec<&'static str>);

impl SourceStream {
    pub fn new(s: &'static str) -> Self {
        let mut stream = s.split_ascii_whitespace();
        SourceStream(stream.collect())
    }
}

impl Tokenizer {
    pub fn from_str(s: &'static str) -> Result<TokenStream, TokenizeError> {
        let source_stream = SourceStream::new(s);
        for source in source_stream.0 {

        }
        unimplemented!()
    }


}

pub struct TokenizeError;

struct Token {
    token_type: TokenType,
    value: &'static str
}

impl Token {
    pub fn new(s: &'static str) -> Self {
        unimplemented!()
    }
}

enum TokenType {
    Fieldsep,
    Binop,
    Unop
}

#[test]
fn tokenizer() {
    let tokenizer = Tokenizer::from_str(r#"local a = "string""#);
}