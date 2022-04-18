use enum_iterator::IntoEnumIterator;

static KEYWORDS: Vec<&str> = vec!["and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in", "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while"];
static SYMBOLS: Vec<&str> = vec!["...", "..", "==", "~=", "<=", ">=", "+", "-", "*", "/", "%", "^", "#", "<", ">", "=", "(", ")", "{", "}", "[", "]", ";", ":", ",", "."];



#[derive(Debug, IntoEnumIterator, PartialEq)]
enum Keyword {
    AND,
    BREAK,
    DO,
    ELSE,
    ELSEIF,
    END,
    FALSE,
    FOR,
    FUNCTION,
    IF,
    IN,
    LOCAL,
    NIL,
    NOT,
    OR,
    REPEAT,
    RETURN,
    THEN,
    TRUE,
    UNTIL,
    WHILE
}

impl Keyword {
    fn value(&self) -> &str {
        match *self {
            Keyword::AND => "and",
            Keyword::BREAK => "break",
            Keyword::DO => "do",
            Keyword::ELSE => "else",
            Keyword::ELSEIF => "elseif",
            Keyword::END => "end",
            Keyword::FALSE => "false",
            Keyword::FOR => "for",
            Keyword::FUNCTION => "function",
            Keyword::IF => "if",
            Keyword::IN => "in",
            Keyword::LOCAL => "local",
            Keyword::NIL => "nil",
            Keyword::NOT => "not",
            Keyword::OR => "or",
            Keyword::REPEAT => "repeat",
            Keyword::RETURN => "return",
            Keyword::THEN => "then",
            Keyword::TRUE => "true",
            Keyword::UNTIL => "until",
            Keyword::WHILE => "while"
        }
    }

    pub fn from_str(s: &str) -> Option<Self>{
        Keyword::into_enum_iter().map(|kw| {
            if kw == s {
                Some(kw)
            }

        }).collect()::<Option<Self>>()?
    }
}




struct Parser {

}

struct Lexer;

impl Lexer {

}

struct Tokenizer;

impl Tokenizer {
    pub fn from_str(s: &str) -> Result<Self, Vec<Token>> {


        s.split_ascii_whitespace().map(|w| {
            if KEYWORDS.contains(w) {
                let token_type = TokenType::Keyword;
                Token::from_keyword(kw)
            }
            
        }).collect()
    }

    fn word_to_token(w: &str) -> Token {
        Keyword.map(|kw| {
            if w == kw {
                Token::from_keyword(kw)
            }
        }).collect().;
    }
}


struct Token {
    Type: TokenType,
    Value: String
}

impl Token {
    pub fn from_keyword(kw: Keyword) -> Self {
        *Self.Type = TokenType::Keyword
    }
}

enum TokenType {
    Keyword,
}

impl TokenType {
    pub fn from_str(s: &str) -> Self {
        
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
