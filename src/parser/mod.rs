use pest::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"] // relative to src
struct MyParser;