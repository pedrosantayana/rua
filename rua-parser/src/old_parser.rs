// use nom::{
//     IResult,
//     bytes::complete::{tag, take_while_m_n, take_until},
//     combinator::map_res,
//     sequence::tuple,
//     character::complete::{
//         one_of,
//         char,
//     },
//     bytes::complete::is_not, branch::alt, error::{ParseError, Error, ErrorKind},
// };

// pub struct AST {
//     chunk: Chunk
// }


// impl AST {
//     pub fn from_string(input: &str) {
//         todo!();
//     }
// }

// struct Chunk {
//     block: Block,
// }

// struct Block {
//     stat: Vec<Statement>,
//     retstat: Option<ReturnStatement>,
// }

// enum Statement {
//     SemiColon,
//     VariableAttribution {
//         varlist: Vec<Variable>,
//         explist: Vec<Expression>,
//     },
//     FunctionCall(FunctionCall),
//     Label(Label),
//     Break,
//     GoTo{
//         exp: Expression,
//         block: Block
//     },
//     Do {
//         block: Block,
//         exp: Expression
//     },
//     While {
//         exp: Expression,
//         block: Block
//     },
//     Repeat {
//         block: Block,
//         exp: Expression
//     },
//     If {
//         exp: Expression,
//         block: Block,
//         elseif: Vec<ElseIf>,
//         elseblock: Option<Block>
//     },
//     For {
//         var: Name,
//         initial: Expression,
//         limit: Expression,
//         step: Option<Expression>,
//         block: Block
//     },
//     ForIn {
//         namelist: Vec<Name>,
//         explist: Vec<Expression>,
//         block: Block
//     },
//     Function {
//         funcname: FuncName,
//         funcbody: FunctionBody
//     },
//     FunctionDeclaration {
//         funcname: Name,
//         funcbody: FunctionBody
//     },
//     ListAttribution {
//         attnamelist: AttributeNameList,
//         explist: Option<Vec<Expression>>,
//     }
// }

// struct AttributeNameList {
//     name: Name,
//     attrib: Attribute,
//     attnamelist: Vec<AttributeNameList>,
// }



// // --------------------------------


// // struct ReturnStatement {";")(input)?;
// //         let (input, _) = tag(";")(input)?;
// //         ReturnStatement {
            
// //         }
// //     }
// // }










// enum Unop {
//     Minus,
//     Not,
//     Sharp,
//     Til
// }

// // impl Unop {
// //     pub fn new(input: &str) -> IResult<&str, Unop> {
// //         match tag::<&str, &str, nom::Err<_>>("not")(input) {
// //             Ok(res) => Ok((res.0, Unop::Not)),
// //             Err(er) => match one_of("-#~")(input) {
// //                 Ok(res) => match res.1 {
// //                     '-' => Ok((res.0, Unop::Minus)),
// //                     '#' => Ok((res.0, Unop::Sharp)),
// //                     '~' => Ok((res.0, Unop::Til)),
// //                 }
// //                 Err(e) => Err(e)
// //             },
// //         }
// //     }
// // }