use std::collections::HashMap;

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
