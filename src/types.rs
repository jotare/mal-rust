use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f64),
    Symbol(String),
    List(Vec<Box<Type>>),
    Vector(Vec<Box<Type>>),
    Fun(Function),
    Closure {
        env: Rc<RefCell<Env>>,
        params: Box<Type>,
        body: Box<Type>,
    }
}

pub type Args = Vec<Type>;
pub type Ret = Result<Type, String>;
pub type Function = fn(Args) -> Ret;

impl Type {

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
            Type::List(seq)|Type::Vector(seq) => Ok(seq.clone()),
            _ => Err(format!("Type error: type must be a sequence (List or Vector)")),
        }
    }
}
