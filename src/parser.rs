use nom::{
    alt,
    tag,
    oneof
}

pub struct AST {
    chunk: Chunk
}

impl AST {
    pub fn from_string(input: &str) {
        todo!();
    }
}















enum Unop {
    Minus,
    Not,
    Sharp,
    Til
}

impl Unop {
    pub fn new(input: &str) -> IResult<&str, Unop> {
        let (input, res) = alt(tag("not"), oneof("-#~"));
        match res {
            "-" => Ok((input, Unop::Minus)),
            "not" => Ok((input, Unop::Not)),
            "#" => Ok((input, Unop::Sharp)),
            "~" => Ok((input, Unop::Til))
        }
    }
}