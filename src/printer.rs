use crate::types::{Ast, Type};

/// Format an Ast and returns it's string representation
pub fn pr_str(ast: Ast) -> String {
    pr_type(&ast.root())
}

/// Recursively format a Type and returns it's string representation
fn pr_type(t: &Type) -> String {
    let mut s = String::new();
    match t {
        Type::Nil => s.push_str("nil"),
        Type::Bool(b) => {
            if *b {
                s.push_str("true")
            } else {
                s.push_str("false")
            }
        },
        Type::Int(integer) => s.push_str(&format!("{}", integer)),
        Type::Symbol(symbol) => s.push_str(&format!("{}", symbol)),
        Type::List(list) => {
            s.push('(');
            s.push_str(
                &list.iter()
                    .map(|element| pr_type(&*element))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            s.push(')');
        }
    };
    s
}
