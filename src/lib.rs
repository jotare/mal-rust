mod printer;
mod reader;
mod types;

use crate::types::Type;

fn read(input: &str) -> Type {
    reader::read_str(input)
}

fn eval(ast: Type) -> Type {
    ast
}

fn print(ast: Type) -> String {
    printer::pr_str(ast)
}

pub fn rep(input: &str) -> String {
    print(eval(read(input)))
}
