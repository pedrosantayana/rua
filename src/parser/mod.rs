use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "parser/lua.pest"]
pub struct LuaParser;

#[test]
fn create_parser() {
    let file = LuaParser::parse(Rule::chunk, "goto a")
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::stat => {
                println!("{}", line.as_str());
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
