use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::Namespace;
use crate::types::Type;

#[derive(Debug, PartialEq, Clone)]
pub struct Env {
    data: RefCell<HashMap<String, Type>>,
    outer: Option<Rc<Env>>,
}

impl Env {
    /// Create a new environment with outer as its outer
    /// environment. Pass binds and exprs to directly set key-value
    /// pairs in the environment.
    ///
    /// Panics if binds and exprs don't have the same size
    pub fn new(outer: Option<Rc<Env>>, binds: &[&str], exprs: &[Type]) -> Env {
        if binds.len() != exprs.len() {
            panic!("`binds` and `exprs` must have the same length");
        }

        let env = Env {
            data: RefCell::new(HashMap::new()),
            outer: outer,
        };
        for i in 0..binds.len() {
            env.set(binds[i], exprs[i].clone());
        }
        env
    }

    /// Create a new environment with the default built-in symbols and functions
    pub fn new_default() -> Env {
        let env = Env::new(None, &[], &[]);
        let ns = Namespace::new_default();
        for (sym, fun) in ns {
            env.set(&sym, Type::Fun(fun));
        }
        env
    }

    /// Set a symbol to a value in the environment
    pub fn set(&self, symbol: &str, value: Type) {
        self.data.borrow_mut().insert(symbol.to_owned(), value);
    }

    fn find(&self, symbol: &str) -> Option<&Env> {
        match self.data.borrow().get(symbol) {
            Some(_) => Some(self),
            None => match self.outer {
                Some(ref env) => env.find(symbol),
                None => None,
            },
        }
    }

    /// Return the value assigned to the symbol in this or any nested
    /// environment.
    ///
    /// Returns an error if the symbol is not found
    pub fn get(&self, symbol: &str) -> Result<Type, String> {
        match self.find(symbol) {
            Some(env) => match env.data.borrow().get(symbol) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Env should have the symbol '{}'", symbol)),
            },
            None => Err(format!("Symbol '{}' not found in any environment", symbol)),
        }
    }

    fn _outermost(self: &Rc<Env>) -> Option<Rc<Env>> {
        match self.outer {
            Some(ref outer) => {
                let outermost = outer._outermost();
                match outermost {
                    Some(_) => outermost,
                    None => Some(outer.clone()),
                }
            }
            None => None
        }
    }

    /// Returns the outermost environment (may be itself)
    pub fn outermost(self: &Rc<Env>) -> Rc<Env> {
        match self._outermost() {
            Some(env) => env,
            None => self.clone(),
        }
    }
}
