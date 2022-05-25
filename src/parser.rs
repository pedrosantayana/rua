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


use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n, take_until},
    combinator::map_res,
    sequence::tuple,
    character::complete::{
      one_of,
      char,
    },
    bytes::complete::is_not,
    
  };



enum Grammar {
    Chuck(Box<crate::parser::Grammar>),
    Block {
        stat: Vec<Statement>,
        retstat: Option<ReturnStatement>,
    },
    
}

struct Chunk {
    block: Block,
}

struct Block {
    stat: Vec<Statement>,
    retstat: Option<ReturnStatement>,
}

enum Statement {
    SemiColon,
    VariableAttribution {
        varlist: Vec<Variable>,
        explist: Vec<Expression>,
    },
    FunctionCall(FunctionCall),
    Label(Label),
    Break,
    GoTo{
        exp: Expression,
        block: Block
    },
    Do {
        block: Block,
        exp: Expression
    },
    While {
        exp: Expression,
        block: Block
    },
    Repeat {
        block: Block,
        exp: Expression
    },
    If {
        exp: Expression,
        block: Block,
        elseif: Vec<ElseIf>,
        elseblock: Option<Block>
    },
    For {
        var: Name,
        initial: Expression,
        limit: Expression,
        step: Option<Expression>,
        block: Block
    },
    ForIn {
        namelist: Vec<Name>,
        explist: Vec<Expression>,
        block: Block
    },
    Function {
        funcname: FuncName,
        funcbody: FunctionBody
    },
    FunctionDeclaration {
        funcname: Name,
        funcbody: FunctionBody
    },
    ListAttribution {
        attnamelist: AttributeNameList,
        explist: Option<Vec<Expression>>,
    }
}

struct AttributeNameList {
    name: Name,
    attrib: Attribute,
    attnamelist: Vec<AttributeNameList>,
}



// --------------------------------


struct ReturnStatement {";")(input)?;
        let (input, _) = tag(";")(input)?;
        ReturnStatement {
            
        }
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