use ebnf::Expression;

mod utils;

pub struct Parser {
    pub grammar: ebnf::Grammar,
}

impl Parser {
    pub fn new() -> Result<Self, nom::Err<nom::error::VerboseError<&'static str>>> {
        match ebnf::get_grammar(utils::SOURCE) {
            Ok(grammar) => Ok(Self { grammar }),
            Err(e) => Err(e),
        }
    }
    pub fn get_grammar(&self) {
        for exp in &self.grammar.expressions {
            println!("{}", exp.lhs);
        }
    }
}