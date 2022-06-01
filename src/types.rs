use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::env::Env;
use crate::error::Exception;
use crate::eval;

#[derive(Clone, Debug)]
pub enum Type {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f64),
    Symbol(String),
    Keyword(String),
    String(String),
    Atom(Rc<RefCell<Type>>),
    List(Vec<Type>),
    Vector(Vec<Type>),
    HashMap(HashMap<String, Box<Type>>),
    Fun(Function),
    Closure {
        env: Rc<Env>,
        params: Box<Type>,
        body: Box<Type>,
        is_macro: bool,
    },
}

pub type Args = Vec<Type>;
pub type Ret = Result<Type, Exception>;
pub type Function = fn(Args) -> Ret;

impl Type {
    /// Apply callable with args and return its result. Type must be
    /// Fun or Closure.
    pub fn apply(&self, args: Vec<Type>) -> Ret {
        match self {
            Type::Fun(fun) => fun(args),

            Type::Closure {
                ref params,
                ref body,
                ref env,
                ..
            } => {
                let params = match **params {
                    Type::List(ref l) | Type::Vector(ref l) => {
                        let param_list: Vec<&str> = l
                            .iter()
                            .map(|elem| match *elem {
                                Type::Symbol(ref sym) => sym.as_str(),
                                _ => "",
                            })
                            .filter(|elem| !elem.is_empty())
                            .collect();
                        param_list
                    }
                    _ => return Err(Exception::interpreter_error("malformed closure")),
                };

                let fun_env = Env::new(Some(env.clone()), &params, args.as_slice());

                eval(*body.to_owned(), &Rc::new(fun_env))
            }
            _ => Err(Exception::type_error("first argument must be a function")),
        }
    }

    /// Convert a Type instance to it's f64 representation. Only calls
    /// with types Int or Float will be successful
    pub fn convert_to_f64(&self) -> Result<f64, Exception> {
        match self {
            Type::Int(num) => Ok(*num as f64),
            Type::Float(num) => Ok(*num),
            _ => Err(Exception::type_error(
                "type must be a number (Int or Float)",
            )),
        }
    }

    pub fn convert_to_string(&self) -> Result<String, Exception> {
        match self {
            Type::String(s) => Ok(s.to_owned()),
            _ => Err(Exception::type_error("type must be a string")),
        }
    }

    /// Convert type to Vec. Type must be a sequence (List or Vector)
    pub fn convert_to_vec(&self) -> Result<Vec<Type>, String> {
        match self {
            Type::List(seq) | Type::Vector(seq) => Ok(seq.clone()),
            _ => Err("Type error: type must be a sequence (List or Vector)".to_string()),
        }
    }

    pub fn convert_to_map(&self) -> Result<HashMap<String, Box<Type>>, String> {
        match self {
            Type::HashMap(hm)=> Ok(hm.clone()),
            _ => Err("Type error: type must be a hash map".to_string()),
        }
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Type::Symbol(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Type::List(_))
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, Type::Vector(_))
    }

    pub fn is_sequence(&self) -> bool {
        self.is_list() || self.is_vector()
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Type::HashMap(_))
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use super::Type::*;

        match (self, other) {
            (Nil, Nil) => true,
            (Bool(a), Bool(b)) => a == b,
            (Int(_), Int(_)) | (Int(_), Float(_)) | (Float(_), Int(_)) | (Float(_), Float(_)) => {
                self.convert_to_f64().unwrap() == other.convert_to_f64().unwrap()
            }
            (Symbol(a), Symbol(b)) => a == b,
            (Keyword(a), Keyword(b)) => a == b,
            (String(a), String(b)) => a == b,
            (List(_), List(_))
            | (List(_), Vector(_))
            | (Vector(_), List(_))
            | (Vector(_), Vector(_)) => self.convert_to_vec() == other.convert_to_vec(),
            (HashMap(a), HashMap(b)) => {
                a.len() == b.len() && a.iter().all(|(k,v)| {
                    b.contains_key(k) && *b[k] == **v
                })
            }
            _ => false,
        }
    }
}
