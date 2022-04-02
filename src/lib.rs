use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    env::Env,
    types::{Ret, Type},
};

mod core;
pub mod env;
mod printer;
mod reader;
mod types;

fn read(input: &str) -> Option<Type> {
    if input.starts_with(";") {
        return None;
    }
    Some(reader::read_str(input))
}

struct TcoVals {
    ast: Option<Type>,
    env: Option<Rc<RefCell<Env>>>,
}

fn eval(ast: Type, env: &Rc<RefCell<Env>>) -> Ret {
    let mut ast = ast;
    let mut tco_env = env.clone(); // use as owner
    let mut env; // reuse parameter names but use them as mutable

    let result;
    let mut tco_values = None;

    'tco: loop {
        if let Some(TcoVals {
            ast: new_ast,
            env: new_env,
        }) = tco_values
        {
            ast = new_ast.unwrap_or(ast);
            tco_env = new_env.unwrap_or(tco_env);
        }

        env = &tco_env;

        result = match ast {
            Type::List(ref list) => {
                if list.len() == 0 {
                    Ok(ast)
                } else {
                    match *list[0].to_owned() {
                        Type::Symbol(symbol) if symbol == "def!" => {
                            if list.len() != 3 {
                                return Err(format!("def! must be called with 2 arguments"));
                            }

                            let key = match *list[1].to_owned() {
                                Type::Symbol(key) => key,
                                _ => return Err(format!("First def! argument must be a symbol")),
                            };
                            let value = *list[2].to_owned();
                            let value = eval(value, env)?;
                            env.borrow_mut().set(&key, value.clone());
                            Ok(value)
                        }

                        Type::Symbol(symbol) if symbol == "let*" => {
                            if list.len() != 3 {
                                return Err(format!("let* must be called with 2 arguments"));
                            }

                            let scope_env = Rc::new(RefCell::new(Env::new(
                                Some(Rc::new(env.borrow().clone())),
                                &[],
                                &[],
                            )));

                            let binding_list = match *list[1].to_owned() {
                                Type::List(seq) | Type::Vector(seq) => seq,
                                _ => return Err(format!("First let* argument must be a list")),
                            };
                            if binding_list.len() % 2 != 0 {
                                return Err(format!("let* binding list must be composed of pairs"));
                            }
                            let mut i = 0;
                            while i + 1 < binding_list.len() {
                                let symbol = match *binding_list[i].to_owned() {
                                    Type::Symbol(symbol) => symbol,
                                    _ => {
                                        return Err(format!("let* variable names must be symbols"))
                                    }
                                };

                                let value = *binding_list[i + 1].to_owned();
                                let value = eval(value, &scope_env)?;

                                scope_env.borrow_mut().set(&symbol, value);

                                i += 2;
                            }

                            let scoped_code = *list[2].to_owned();

                            tco_values = Some(TcoVals {
                                ast: Some(scoped_code),
                                env: Some(scope_env),
                            });
                            // eval(scoped_code, &scope_env)
                            continue 'tco;
                        }

                        Type::Symbol(symbol) if symbol == "do" => {
                            let (last, do_list) = list.split_last().unwrap();
                            let do_list = Type::List(do_list[1..].to_vec());
                            eval_ast(do_list, env)?;
                            tco_values = Some(TcoVals {
                                ast: Some(*last.to_owned()),
                                env: None,
                            });
                            continue 'tco;
                        }

                        Type::Symbol(symbol) if symbol == "if" => {
                            if list.len() < 3 || list.len() > 4 {
                                return Err(format!("Malformed if expression"));
                            }

                            let cond = *list.get(1).unwrap().to_owned();
                            let if_clause = *list.get(2).unwrap().to_owned();
                            let else_clause = list.get(3);

                            let cond = match eval(cond, env)? {
                                Type::Bool(false) | Type::Nil => false,
                                _ => true,
                            };

                            if cond {
                                eval(if_clause, env)
                            } else {
                                if else_clause.is_some() {
                                    let else_clause = *else_clause.unwrap().to_owned();
                                    tco_values = Some(TcoVals {
                                        ast: Some(else_clause),
                                        env: None,
                                    });
                                    continue 'tco;
                                } else {
                                    Ok(Type::Nil)
                                }
                            }
                        }

                        Type::Symbol(symbol) if symbol == "fn*" => {
                            if list.len() != 3 {
                                return Err(format!("Malformed fn* expression"));
                            }

                            let params = match *list[1] {
                                Type::List(_) | Type::Vector(_) => *list[1].clone(),
                                _ => {
                                    return Err(format!(
                                        "fn* must be defined with a sequence as parameter"
                                    ))
                                }
                            };
                            let body = *list[2].clone();

                            let closure = Type::Closure {
                                params: Box::new(params),
                                body: Box::new(body),
                                env: Rc::clone(env),
                            };

                            Ok(closure)
                        }

                        _ => {
                            // eval list and call first item as a
                            // function and the rest as its arguments
                            let list = eval_ast(ast.clone(), env)?;
                            let list = match list {
                                Type::List(list) => list,
                                _ => return Err(format!("Type can't not be a List")),
                            };

                            let (f, args) = list.split_first().unwrap();
                            let f = *f.to_owned();
                            let args = args.iter().map(|arg| *arg.to_owned()).collect();

                            match f {
                                Type::Fun(fun) => fun(args),

                                Type::Closure {
                                    ref params,
                                    ref body,
                                    ref env,
                                } => {
                                    let params = match **params {
                                        Type::List(ref l) | Type::Vector(ref l) => {
                                            let param_list: Vec<&str> = l
                                                .iter()
                                                .map(|elem| match **elem {
                                                    Type::Symbol(ref sym) => sym.as_str(),
                                                    _ => "",
                                                })
                                                .filter(|elem| elem.len() > 0)
                                                .collect();
                                            param_list
                                        }
                                        _ => {
                                            return Err(format!(
                                                "Interpreter error: malformed closure!"
                                            ))
                                        }
                                    };

                                    let fun_env = Env::new(
                                        Some(Rc::new(env.borrow().to_owned())),
                                        &params,
                                        args.as_slice(),
                                    );

                                    tco_values = Some(TcoVals {
                                        ast: Some(*body.to_owned()),
                                        env: Some(Rc::new(RefCell::new(fun_env))),
                                    });
                                    continue 'tco;
                                }
                                _ => Err(format!("Type error: first argument must be a function")),
                            }
                        }
                    }
                }
            }
            _ => eval_ast(ast, env),
        };

        // end Tail call optimization
        break;
    }
    result
}

fn eval_ast(ast: Type, env: &Rc<RefCell<Env>>) -> Ret {
    match ast {
        Type::Symbol(sym) => match env.borrow().get(sym.as_str()) {
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

        Type::Vector(vector) => {
            let mut evaluated = Vec::with_capacity(vector.len());
            for elem in vector {
                let elem = eval(*elem, env)?;
                evaluated.push(Box::new(elem));
            }
            Ok(Type::Vector(evaluated))
        }

        Type::HashMap(hash_map) => {
            let mut evaluated = HashMap::with_capacity(hash_map.len());
            for (key, value) in hash_map {
                let k = key.clone();
                let v = Box::new(eval(*value.to_owned(), env)?);
                evaluated.insert(k, v);
            }
            Ok(Type::HashMap(evaluated))
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

pub fn rep(input: &str, env: &Rc<RefCell<Env>>) -> String {
    let parsed_input = read(input);
    match parsed_input {
        Some(ast) => print(eval(ast, env)),
        None => String::new(),
    }
}
