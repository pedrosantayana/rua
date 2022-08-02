use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/lua.pest"]
pub struct LuaParser;