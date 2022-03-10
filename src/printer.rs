use crate::types::{Ast, MalType};

/// Format an Ast and returns it's string representation
pub fn pr_str(ast: Ast) -> String {
    pr_type(&ast.root())
}

/// Recursively format a MalType and returns it's string representation
fn pr_type(t: &MalType) -> String {
    let mut s = String::new();
    match t {
        MalType::Nil => s.push_str("nil"),
        MalType::True => s.push_str("true"),
        MalType::False => s.push_str("false"),
        MalType::Integer(integer) => s.push_str(&format!("{}", integer)),
        MalType::Symbol(symbol) => s.push_str(&format!("{}", symbol)),
        MalType::List(list) => {
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
