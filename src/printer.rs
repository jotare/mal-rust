use crate::types::Type;

/// Format an Type and returns it's string representation
pub fn pr_str(ast: Type, print_readably: bool) -> String {
    pr_type(&ast, print_readably)
}

/// Recursively format a Type and returns it's string representation
fn pr_type(t: &Type, print_readably: bool) -> String {
    let mut s = String::new();
    match t {
        Type::Nil => s.push_str("nil"),
        Type::Bool(b) => {
            if *b {
                s.push_str("true")
            } else {
                s.push_str("false")
            }
        }
        Type::Int(integer) => s.push_str(&format!("{}", integer)),
        Type::Float(float) => {
            if float - float.floor() > 0.0 {
                s.push_str(&format!("{}", float))
            } else {
                s.push_str(&format!("{}.0", float))
            }
        },
        Type::Symbol(symbol) => s.push_str(&format!("{}", symbol)),
        Type::Keyword(keyword) => s.push_str(&format!(":{}", keyword)),
        Type::List(list) => {
            s.push('(');
            s.push_str(pr_seq(&list, print_readably).as_str());
            s.push(')');
        }
        Type::Vector(list) => {
            s.push('[');
            s.push_str(pr_seq(&list, print_readably).as_str());
            s.push(']');
        }
        Type::Fun(_) => s.push_str("#<function>"),
        Type::Closure { .. } => s.push_str("#<closure>"),
    };
    s
}

fn pr_seq(seq: &Vec<Box<Type>>, print_readably: bool) -> String {
    seq
        .iter()
        .map(|element| pr_type(&*element, print_readably))
        .collect::<Vec<String>>()
        .join(" ")
}
