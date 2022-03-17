#![allow(dead_code)]

use std::{collections::HashMap};

use crate::types::Type;

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
