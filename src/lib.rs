pub mod env;
mod printer;
mod reader;
mod types;

use crate::{
    env::Env,
    types::{Ret, Type},
};

fn read(input: &str) -> Type {
    reader::read_str(input)
}

fn eval(ast: Type, env: &mut Env) -> Ret {
    match ast {
        Type::List(list) => {
            if list.len() == 0 {
                Ok(Type::List(list))
            } else {
                let symbol = *list[0].clone();
                if let Type::Symbol(symbol) = symbol {
                    match symbol.as_str() {
                        "def!" => {
                            if list.len() != 3 {
                                return Err(format!("def! must be called with 2 arguments"));
                            }

                            let key = match *list[1].clone() {
                                Type::Symbol(key) => key.clone(),
                                _ => return Err(format!("First def! argument must be a symbol")),
                            };
                            let value = *list[2].clone();
                            let value = eval(value, env)?;
                            env.set(&key, value.clone());
                            Ok(value)
                        }

                        _ => {
                            // eval list and call first item as a
                            // function and the rest as its arguments
                            let list = eval_ast(Type::List(list), env)?;
                            let list = match list {
                                Type::List(list) => list,
                                _ => return Err(format!("Type can't not be a List")),
                            };

                            match *list[0] {
                                Type::Fun(fun) => {
                                    let args = list[1..].iter().map(|a| *a.clone()).collect();
                                    let ret = fun(args);
                                    ret
                                }
                                _ => Err(format!("First argument must be a function!")),
                            }
                        }
                    }
                } else {
                    Err(format!("First argument must be a symbol"))
                }

            }
        }
        other => eval_ast(other, env),
    }
}

fn eval_ast(ast: Type, env: &mut Env) -> Ret {
    match ast {
        Type::Symbol(sym) => match env.get(sym.as_str()) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        },

        Type::List(list) => {
            let mut evaluated = Vec::with_capacity(list.len());
            for elem in list {
                let elem = eval(*elem, env)?;
                evaluated.push(Box::new(elem));
            }
            Ok(Type::List(evaluated))
        }

        _ => Ok(ast),
    }
}

fn print(ast: Result<Type, String>) -> String {
    match ast {
        Ok(ast) => printer::pr_str(ast),
        Err(e) => format!("{}", e),
    }
}

pub fn rep(input: &str, env: &mut Env) -> String {
    print(eval(read(input), env))
}
