use std::collections::HashMap;
use std::rc::Rc;

use crate::env::Env;

#[derive(Clone, Debug)]
pub enum Type {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f64),
    Symbol(String),
    Keyword(String),
    String(String),
    List(Vec<Box<Type>>),
    Vector(Vec<Box<Type>>),
    HashMap(HashMap<String, Box<Type>>),
    Fun(Function),
    Closure {
        env: Rc<Env>,
        params: Box<Type>,
        body: Box<Type>,
    },
}

pub type Args = Vec<Type>;
pub type Ret = Result<Type, String>;
pub type Function = fn(Args) -> Ret;

impl Type {
    /// Convert a Type instance to it's f64 representation. Only calls
    /// with types Int or Float will be successful
    pub fn convert_to_f64(&self) -> Result<f64, String> {
        match self {
            Type::Int(num) => Ok(*num as f64),
            Type::Float(num) => Ok(*num),
            _ => Err(format!("Type error: type must be a number (Int or Float)")),
        }
    }

    /// Convert type to Vec. Type must be a sequence (List or Vector)
    pub fn convert_to_vec(&self) -> Result<Vec<Box<Type>>, String> {
        match self {
            Type::List(seq) | Type::Vector(seq) => Ok(seq.clone()),
            _ => Err(format!(
                "Type error: type must be a sequence (List or Vector)"
            )),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use super::Type::*;

        match (self, other) {
            (Nil, Nil) => true,
            (Bool(a), Bool(b)) => a == b,
            (Int(_), Int(_)) | (Int(_), Float(_)) | (Float(_), Int(_)) | (Float(_), Float(_)) => {
                self.convert_to_f64() == other.convert_to_f64()
            }
            (Symbol(a), Symbol(b)) => a == b,
            (Keyword(a), Keyword(b)) => a == b,
            (String(a), String(b)) => a == b,
            (List(_), List(_))
            | (List(_), Vector(_))
            | (Vector(_), List(_))
            | (Vector(_), Vector(_)) => self.convert_to_vec() == other.convert_to_vec(),
            _ => false,
        }
    }
}
