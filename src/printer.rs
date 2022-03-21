use crate::types::Type;

/// Format an Type and returns it's string representation
pub fn pr_str(ast: Type) -> String {
    pr_type(&ast)
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
        Type::List(list) => {
            s.push('(');
            s.push_str(
                &list
                    .iter()
                    .map(|element| pr_type(&*element))
                    .collect::<Vec<String>>()
                    .join(" "),
            );
            s.push(')');
        }
        Type::Fun(_) => s.push_str("#<function>"),
    };
    s
}
