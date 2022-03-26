use std::rc::Rc;

use crate::{
    env::Env,
    types::{Ret, Type},
};

pub mod env;
mod printer;
mod reader;
mod types;

fn read(input: &str) -> Type {
    reader::read_str(input)
}

fn eval(ast: Type, env: &mut Rc<Env>) -> Ret {
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
                                Type::Symbol(key) => key,
                                _ => return Err(format!("First def! argument must be a symbol")),
                            };
                            let value = *list[2].clone();
                            let value = eval(value, env)?;
                            Rc::get_mut(env).unwrap().set(&key, value.clone());
                            Ok(value)
                        }

                        "let*" => {
                            if list.len() != 3 {
                                return Err(format!("let* must be called with 2 arguments"));
                            }

                            let mut scope_env = Rc::new(Env::new(Some(Rc::clone(env)), &[], &[]));

                            let binding_list = match *list[1].clone() {
                                Type::List(list) => list,
                                _ => return Err(format!("First let* argument must be a list")),
                            };
                            if binding_list.len() % 2 != 0 {
                                return Err(format!("let* binding list must be composed of pairs"));
                            }
                            let mut i = 0;
                            while i + 1 < binding_list.len() {
                                let symbol = match *binding_list[i].clone() {
                                    Type::Symbol(symbol) => symbol,
                                    _ => {
                                        return Err(format!("let* variable names must be symbols"))
                                    }
                                };

                                let value = *binding_list[i + 1].clone();
                                let value = eval(value, &mut scope_env)?;

                                Rc::get_mut(&mut scope_env).unwrap().set(&symbol, value);

                                i += 2;
                            }

                            let scoped_code = *list[2].clone();
                            eval(scoped_code, &mut scope_env)
                        }

                        "do" => {
                            let do_list = Type::List(list[1..].to_vec());
                            match eval_ast(do_list, env)? {
                                Type::List(list) => match list.last() {
                                    Some(element) => Ok(*element.clone()),
                                    None => Ok(Type::Nil),
                                },
                                _ => Err(format!("Malformed do expression")),
                            }
                        }

                        "if" => {
                            if list.len() < 3 || list.len() > 4 {
                                return Err(format!("Malformed if expression"));
                            }

                            let cond = *list.get(1).unwrap().clone();
                            let if_clause = *list.get(2).unwrap().clone();
                            let else_clause = list.get(3);

                            let cond = match eval(cond, env)? {
                                Type::Bool(false) | Type::Nil => false,
                                _ => true,
                            };

                            if cond {
                                eval(if_clause, env)
                            } else {
                                if else_clause.is_some() {
                                    let else_clause = *else_clause.unwrap().clone();
                                    eval(else_clause, env)
                                } else {
                                    Ok(Type::Nil)
                                }
                            }
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

fn eval_ast(ast: Type, env: &mut Rc<Env>) -> Ret {
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
        Ok(ast) => printer::pr_str(ast, true),
        Err(e) => format!("{}", e),
    }
}

pub fn rep(input: &str, env: &mut Rc<Env>) -> String {
    print(eval(read(input), env))
}
