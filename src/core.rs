use std::collections::HashMap;

use crate::printer::pr_str;
use crate::types::{Args, Function, Ret, Type};

pub struct Namespace {
    data: HashMap<String, Function>
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace {
            data: HashMap::new()
        }
    }

    pub fn new_default() -> Namespace {
        let mut ns = Namespace::new();
        ns.data.insert(String::from("+"), sum);
        ns.data.insert(String::from("-"), sub);
        ns.data.insert(String::from("*"), mul);
        ns.data.insert(String::from("/"), div);
        ns.data.insert(String::from("prn"), prn);
        ns.data.insert(String::from("list"), list);
        ns.data.insert(String::from("list?"), is_list);
        ns.data.insert(String::from("empty?"), is_empty);
        ns.data.insert(String::from("count"), count);
        ns.data.insert(String::from("="), eq);
        ns.data.insert(String::from("<"), lt);
        ns.data.insert(String::from("<="), lte);
        ns.data.insert(String::from(">"), gt);
        ns.data.insert(String::from(">="), gte);
        ns
    }
}

impl IntoIterator for Namespace {
    type Item = (String, Function);
    type IntoIter = std::collections::hash_map::IntoIter<String, Function>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

fn sum(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a + *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a + *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 + *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a + *b)),
        _ => Err(String::from("Type error: '+' is only supported for Int and Float")),
    }
}

fn sub(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a - *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a - *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 - *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a - *b)),
        _ => Err(String::from("Type error: '-' is only supported for Int and Float")),
    }
}

fn mul(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a * *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a * *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 * *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a * *b)),
        _ => Err(String::from("Type error: '*' is only supported for Int and Float")),
    }
}

fn div(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a / *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a / *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 / *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a / *b)),
        _ => Err(String::from("Type error: '/' is only supported for Int and Float")),
    }
}

fn prn(args: Args) -> Ret {
    match args.get(0) {
        Some(value) => {
            println!("{}", pr_str(value.clone(), true));
            Ok(Type::Nil)
        },
        None => Err(format!("Must pass an argument to 'prn'")),
    }
}

fn list(args: Args) -> Ret {
    Ok(Type::List(
        args.iter()
            .map(|a| Box::new(a.clone()))
            .collect()
    ))
}

fn is_list(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::List(_)) => Ok(Type::Bool(true)),
        _ => Ok(Type::Bool(false))
    }
}

fn is_empty(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::List(seq) | Type::Vector(seq))=> {
            Ok(Type::Bool(seq.len() == 0))
        }
        _ => Err(format!("Type error: 'empty?' is only supported for sequences"))
    }
}

fn count(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::List(seq) | Type::Vector(seq)) => {
            Ok(Type::Int(seq.len() as i32))
        }
        Some(Type::Nil) => Ok(Type::Int(0)),
        _ => Err(format!("Type error: 'count' is only supported for List"))
    }
}

fn eq(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            match (a, b) {
                (Type::Int(_)|Type::Float(_),Type::Int(_)|Type::Float(_)) => {
                    let a = a.convert_to_f64()?;
                    let b = b.convert_to_f64()?;
                    Ok(Type::Bool(a == b))
                }
                (Type::List(_)|Type::Vector(_), Type::Vector(_)|Type::List(_)) => {
                    let a = a.convert_to_vec()?;
                    let b = b.convert_to_vec()?;
                    Ok(Type::Bool(a == b))
                }
                _ => Ok(Type::Bool(a == b)),
            }
        }
        _ => Err(format!("Value error: must pass 2 arguments to '='"))
    }
}

fn lt(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a < b))
        }
        _ => Err(format!("Value error: must pass 2 arguments to '<'"))
    }
}


fn lte(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a <= b))
        }
        _ => Err(format!("Value error: must pass 2 arguments to '<='"))
    }
}

fn gt(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a > b))
        }
        _ => Err(format!("Value error: must pass 2 arguments to '>'"))
    }
}

fn gte(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a >= b))
        }
        _ => Err(format!("Value error: must pass 2 arguments to '>='"))
    }
}
