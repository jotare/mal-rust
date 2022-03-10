mod printer;
mod reader;
mod types;

use crate::types::Ast;


fn read(input: &str) -> Ast {
    reader::read_str(input)
}

fn eval(ast: Ast) -> Ast {
    ast
}

fn print(ast: Ast) -> String {
    printer::pr_str(ast)
}

pub fn rep(input: &str) -> String {
    print(eval(read(input)))
}
