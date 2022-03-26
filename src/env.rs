use std::collections::HashMap;
use std::rc::Rc;

use crate::types::{Args, Ret, Type};

#[derive(Debug)]
pub struct Env {
    data: HashMap<String, Type>,
    outer: Option<Rc<Env>>,
}

impl Env {
    pub fn new(outer: Option<Rc<Env>>, binds: &[&str], exprs: &[Type]) -> Env {
        if binds.len() != exprs.len() {
            panic!("`binds` and `exprs` must have the same length");
        }

        let mut env = Env {
            data: HashMap::new(),
            outer: outer,
        };
        for i in 0..binds.len() {
            env.set(binds[i], exprs[i].clone());
        }
        env
    }

    pub fn new_default() -> Env {
        let mut env = Env::new(None, &[], &[]);
        env.set("+", Type::Fun(sum));
        env.set("-", Type::Fun(sub));
        env.set("*", Type::Fun(mul));
        env.set("/", Type::Fun(div));
        env
    }

    pub fn set(&mut self, symbol: &str, value: Type) {
        self.data.insert(symbol.to_owned(), value);
    }

    fn find(&self, symbol: &str) -> Option<&Env> {
        match self.data.get(symbol) {
            Some(_) => Some(self),
            None => match self.outer {
                Some(ref env) => env.find(symbol),
                None => None,
            },
        }
    }

    pub fn get(&self, symbol: &str) -> Result<Type, String> {
        match self.find(symbol) {
            Some(env) => match env.data.get(symbol) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Env should have the symbol '{}'", symbol)),
            },
            None => Err(format!("Symbol '{}' not found in any environment", symbol)),
        }
    }
}

fn sum(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a + *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a + *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 + *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a + *b)),
        _ => Err(String::from(
            "'+' operation failed. Types must be numeric (Int or Float)",
        )),
    }
}

fn sub(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a - *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a - *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 - *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a - *b)),
        _ => Err(String::from(
            "'-' operation failed. Types must be numeric (Int or Float)",
        )),
    }
}

fn mul(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a * *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a * *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 * *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a * *b)),
        _ => Err(String::from(
            "'*' operation failed. Types must be numeric (Int or Float)",
        )),
    }
}

fn div(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a / *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a / *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 / *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a / *b)),
        _ => Err(String::from(
            "'/' operation failed. Types must be numeric (Int or Float)",
        )),
    }
}
