use nom::{IResult, bytes::complete::tag, character::complete::one_of, branch::alt, combinator::map, sequence::tuple};

pub struct AST {
    chunk: Chunk
}

impl AST {
    pub fn from_string(input: &str) {
        todo!();
    }
}

struct Chunk;













enum Unop {
    Minus,
    Not,
    Sharp,
    Til
}

impl Unop {
    pub fn new(input: &str) -> IResult<&str, Unop> {
        //let (input, res) = tag("-")(input)?;

        let (input, res) = alt((tag("not"), one_of("-#~")))(input).unwrap();
        match res {
            '-' => Ok((input, Unop::Minus)),
            "not" => Ok((input, Unop::Not)),
            '#' => Ok((input, Unop::Sharp)),
            '~' => Ok((input, Unop::Til)),
            _ => Err()
        }
    }
}