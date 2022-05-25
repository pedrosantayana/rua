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


struct ReturnStatement {
    explist: ExpressionList

}

impl ReturnStatement {
    fn new(input: &str) -> ReturnStatement {
        tag("return")(input)?;
        let (input, _) = take_until(";")(input)?;
        let (input, _) = tag(";")(input)?;
        ReturnStatement {
            
        }
    }
}