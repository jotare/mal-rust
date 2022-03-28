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
