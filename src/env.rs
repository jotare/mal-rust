use std::collections::HashMap;
use std::rc::Rc;

use crate::types::Type;
use crate::core::Namespace;

#[derive(Debug, PartialEq, Clone)]
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
        let ns = Namespace::new_default();
        for (sym, fun) in ns {
            env.set(&sym, Type::Fun(fun));
        }
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
