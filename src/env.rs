use std::{collections::HashMap};

use crate::types::{Args, Ret, Type};

#[derive(Debug)]
pub struct Env<'a> {
    data: HashMap<String, Type>,
    outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn new(outer: Option<&'a Env>) -> Env<'a> {
        Env {
            data: HashMap::new(),
            outer: outer,
        }
    }

    pub fn new_default() -> Env<'a> {
        let mut env = Env::new(None);
        env.set("+", Type::Fun(sum));
        env.set("-", Type::Fun(sub));
        env.set("*", Type::Fun(mul));
        env.set("/", Type::Fun(div));
        env
    }

    pub fn set(&mut self, symbol: &str, value: Type) {
        self.data.insert(String::from(symbol), value);
    }

    pub fn find(&self, symbol: &str) -> Option<&'a Env> {
        match self.data.get(symbol) {
            Some(_) => Some(&self),
            None => {
                match self.outer {
                    Some(env) => env.find(symbol),
                    None => None,
                }
            },
        }
    }

    pub fn get(&self, symbol: &str) -> Result<Type, String>  {
        match self.find(symbol) {
            Some(env) => {
                match env.data.get(symbol) {
                    Some(value) => Ok(value.clone()),
                    None => Err(format!("Env should have the symbol '{}'", symbol)),
                }
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
        _ => Err(String::from("'+' operation failed. Types must be numeric (Int or Float)")),
    }
}

fn sub(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a - *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a - *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 - *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a - *b)),
        _ => Err(String::from("'-' operation failed. Types must be numeric (Int or Float)")),
    }
}

fn mul(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a * *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a * *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 * *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a * *b)),
        _ => Err(String::from("'*' operation failed. Types must be numeric (Int or Float)")),
    }
}

fn div(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a / *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a / *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 / *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a / *b)),
        _ => Err(String::from("'/' operation failed. Types must be numeric (Int or Float)")),
    }
}
