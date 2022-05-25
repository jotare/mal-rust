use crate::types::Type;
use crate::utils::escape_string;

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
        }
        Type::Symbol(symbol) => s.push_str(symbol),
        Type::Keyword(keyword) => s.push_str(&format!(":{}", keyword)),
        Type::String(string) => {
            if print_readably {
                s.push_str(&format!("\"{}\"", escape_string(string)));
            } else {
                s.push_str(string);
            }
        }
        Type::Atom(atom) => {
            s.push_str(&format!(
                "(atom {})",
                pr_type(&atom.borrow(), print_readably)
            ));
        }
        Type::List(list) => {
            s.push_str(&format!("({})", pr_seq(list, print_readably).as_str()));
        }
        Type::Vector(vector) => {
            s.push_str(&format!("[{}]", pr_seq(vector, print_readably).as_str()));
        }
        Type::HashMap(hashmap) => {
            s.push('{');
            s.push_str(
                &hashmap
                    .iter()
                    .map(|(k, v)| format!("{} {}", k, pr_type(&*v, print_readably)))
                    .collect::<Vec<String>>()
                    .join(" "),
            );
            s.push('}');
        }
        Type::Fun(_) => s.push_str("#<function>"),
        Type::Closure { params, body, .. } => {
            s.push_str(&format!("(fn* {} {})", pr_type(params, true), pr_type(body, true)));
        },
    };
    s
}

fn pr_seq(seq: &[Type], print_readably: bool) -> String {
    seq.iter()
        .map(|element| pr_type(&*element, print_readably))
        .collect::<Vec<String>>()
        .join(" ")
}
