use std::collections::HashMap;
use std::rc::Rc;

use error::{nargs_check, Exception};

use crate::{
    env::Env,
    types::{Ret, Type},
};

mod core;
pub mod env;
mod error;
mod printer;
mod reader;
mod types;
mod utils;

fn read(input: &str) -> Result<Option<Type>, String> {
    reader::read_str(input)
}

struct TcoVals {
    ast: Option<Type>,
    env: Option<Rc<Env>>,
}

fn eval(ast: Type, env: &Rc<Env>) -> Ret {
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

        ast = macroexpand(ast, env)?;

        result = match ast {
            Type::List(ref list) => {
                if list.is_empty() {
                    Ok(ast)
                } else {
                    match list[0].to_owned() {
                        Type::Symbol(symbol) if symbol == "def!" => {
                            nargs_check("def!", 2, list.len() - 1)?;

                            let name = match list[1].to_owned() {
                                Type::Symbol(name) => name,
                                _ => {
                                    return Err(Exception::type_error(
                                        "first def! argument must be a symbol",
                                    ))
                                }
                            };
                            let value = list[2].to_owned();
                            let value = eval(value, env)?;
                            env.set(&name, value.clone());
                            Ok(value)
                        }

                        Type::Symbol(symbol) if symbol == "let*" => {
                            nargs_check("let*", 2, list.len() - 1)?;

                            let scope_env = Rc::new(Env::new(Some(env.clone()), &[], &[]));

                            let binding_list = match list[1].to_owned() {
                                Type::List(seq) | Type::Vector(seq) => seq,
                                _ => {
                                    return Err(Exception::type_error(
                                        "first let* argument must be a list",
                                    ))
                                }
                            };
                            if binding_list.len() % 2 != 0 {
                                return Err(Exception::builtin(
                                    "let* binding list must be composed of pairs",
                                ));
                            }
                            let mut i = 0;
                            while i + 1 < binding_list.len() {
                                let symbol = match binding_list[i].to_owned() {
                                    Type::Symbol(symbol) => symbol,
                                    _ => {
                                        return Err(Exception::builtin(
                                            "let* variable names must be symbols",
                                        ))
                                    }
                                };

                                let value = binding_list[i + 1].to_owned();
                                let value = eval(value, &scope_env)?;

                                scope_env.set(&symbol, value);

                                i += 2;
                            }

                            let scoped_code = list[2].to_owned();

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
                                ast: Some(last.to_owned()),
                                env: None,
                            });
                            continue 'tco;
                        }

                        Type::Symbol(symbol) if symbol == "if" => {
                            if list.len() < 3 || list.len() > 4 {
                                return Err(Exception::builtin("Malformed if expression"));
                            }

                            let cond = list.get(1).unwrap().to_owned();
                            let if_clause = list.get(2).unwrap().to_owned();
                            let else_clause = list.get(3);

                            if !matches!(eval(cond, env)?, Type::Bool(false) | Type::Nil) {
                                eval(if_clause, env)
                            } else if else_clause.is_some() {
                                let else_clause = else_clause.unwrap().to_owned();
                                tco_values = Some(TcoVals {
                                    ast: Some(else_clause),
                                    env: None,
                                });
                                continue 'tco;
                            } else {
                                Ok(Type::Nil)
                            }
                        }

                        Type::Symbol(symbol) if symbol == "fn*" => {
                            nargs_check("fn*", 2, list.len() - 1)?;

                            let params = match list[1] {
                                Type::List(_) | Type::Vector(_) => list[1].clone(),
                                _ => {
                                    return Err(Exception::builtin(
                                        "fn* must be defined with a sequence as parameter",
                                    ))
                                }
                            };
                            let body = list[2].clone();

                            let closure = Type::Closure {
                                params: Box::new(params),
                                body: Box::new(body),
                                env: env.clone(),
                                is_macro: false,
                            };

                            Ok(closure)
                        }

                        Type::Symbol(symbol) if symbol == "eval" => {
                            nargs_check("eval", 1, list.len() - 1)?;

                            tco_values = Some(TcoVals {
                                ast: Some(eval(list[1].to_owned(), env)?),
                                env: Some(env.outermost()),
                            });

                            continue 'tco;
                        }

                        Type::Symbol(symbol) if symbol == "quote" => {
                            nargs_check("quote", 1, list.len() - 1)?;

                            Ok(list[1].to_owned())
                        }

                        Type::Symbol(symbol) if symbol == "quasiquote" => {
                            tco_values = Some(TcoVals {
                                ast: Some(quasiquote(list[1].to_owned())?),
                                env: None,
                            });

                            continue 'tco;
                        }

                        Type::Symbol(symbol) if symbol == "quasiquoteexpand" => {
                            Ok(quasiquote(list[1].to_owned())?)
                        }

                        Type::Symbol(symbol) if symbol == "defmacro!" => {
                            nargs_check("defmacro!", 2, list.len() - 1)?;

                            let name = match list[1].to_owned() {
                                Type::Symbol(name) => name,
                                _ => {
                                    return Err(Exception::type_error(
                                        "First defmacro! argument must be a symbol",
                                    ))
                                }
                            };

                            let value = match eval(list[2].to_owned(), env)? {
                                Type::Closure {
                                    env, params, body, ..
                                } => Type::Closure {
                                    env,
                                    params,
                                    body,
                                    is_macro: true,
                                },
                                _ => {
                                    return Err(Exception::type_error(
                                        "Type error defmacro! must be called with a function",
                                    ))
                                }
                            };

                            env.set(&name, value.clone());
                            Ok(value)
                        }

                        Type::Symbol(symbol) if symbol == "macroexpand" => {
                            nargs_check("macroexpand", 1, list.len() - 1)?;

                            Ok(macroexpand(list[1].to_owned(), env)?)
                        }

                        Type::Symbol(symbol) if symbol == "try*" => {
                            // (try* A (catch* B C))
                            nargs_check("try*", 1, list.len() - 1)
                                .or_else(|_| error::nargs_check("try*", 2, list.len() - 1))?;

                            // try*
                            let try_body = list[1].to_owned();
                            let try_result = eval(try_body, env);

                            if try_result.is_ok() || list.len() == 2 {
                                return try_result;
                            }

                            // catch*
                            let exception = match try_result.unwrap_err() {
                                Exception::Builtin(s) => Type::String(s),
                                Exception::Custom(t) => t,
                            };

                            let catch = match list[2].to_owned() {
                                Type::List(seq) | Type::Vector(seq) => seq,
                                _ => {
                                    return Err(Exception::type_error(
                                        "catch* block must be a list",
                                    ))
                                }
                            };
                            nargs_check("catch*", 2, catch.len() - 1)?;

                            let catch_var = match &catch[1] {
                                Type::Symbol(s) => s,
                                _ => {
                                    return Err(Exception::type_error(
                                        "First catch* argument must be a symbol",
                                    ))
                                }
                            };
                            let catch_body = catch[2].to_owned();

                            let exc_env = Env::new(Some(env.clone()), &[catch_var], &[exception]);

                            Ok(eval(catch_body, &Rc::new(exc_env))?)
                        }

                        _ => {
                            // eval list and call first item as a
                            // function and the rest as its arguments
                            let list = eval_ast(ast.clone(), env)?;
                            let list = match list {
                                Type::List(list) => list,
                                _ => return Err(Exception::type_error("Type can't not be a List")),
                            };

                            let (f, args) = list.split_first().unwrap();
                            let f = f.to_owned();
                            let args = args.iter().map(|arg| arg.to_owned()).collect();

                            match f {
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
                                        _ => {
                                            return Err(Exception::interpreter_error(
                                                "Interpreter error: malformed closure!",
                                            ))
                                        }
                                    };

                                    let fun_env =
                                        Env::new(Some(env.clone()), &params, args.as_slice());

                                    tco_values = Some(TcoVals {
                                        ast: Some(*body.to_owned()),
                                        env: Some(Rc::new(fun_env)),
                                    });
                                    continue 'tco;
                                }
                                _ => {
                                    Err(Exception::type_error("first argument must be a function"))
                                }
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

fn eval_ast(ast: Type, env: &Rc<Env>) -> Ret {
    match ast {
        Type::Symbol(sym) => match env.get(sym.as_str()) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        },

        Type::List(list) => {
            let mut evaluated = Vec::with_capacity(list.len());
            for elem in list {
                let elem = eval(elem, env)?;
                evaluated.push(elem);
            }
            Ok(Type::List(evaluated))
        }

        Type::Vector(vector) => {
            let mut evaluated = Vec::with_capacity(vector.len());
            for elem in vector {
                let elem = eval(elem, env)?;
                evaluated.push(elem);
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

fn quasiquote_seq(seq: Vec<Type>) -> Ret {
    let mut result = Type::List(vec![]);
    for elt in seq.iter().rev() {
        if elt.is_list() {
            let list = elt.convert_to_vec()?;
            if list.len() >= 2 && list[0] == Type::Symbol("splice-unquote".to_string()) {
                result = Type::List(vec![
                    Type::Symbol("concat".to_string()),
                    list[1].to_owned(),
                    result,
                ]);
                continue;
            }
        }
        result = Type::List(vec![
            Type::Symbol("cons".to_string()),
            quasiquote(elt.to_owned())?,
            result,
        ])
    }
    Ok(result)
}

fn quasiquote(ast: Type) -> Ret {
    match ast {
        Type::List(list) => {
            if list.len() >= 2 && list[0] == Type::Symbol("unquote".to_string()) {
                Ok(list[1].to_owned())
            } else {
                quasiquote_seq(list)
            }
        }
        Type::Vector(vector) => Ok(Type::List(vec![
            Type::Symbol("vec".to_string()),
            quasiquote_seq(vector)?,
        ])),
        Type::HashMap(_) | Type::Symbol(_) => {
            Ok(Type::List(vec![Type::Symbol("quote".to_string()), ast]))
        }
        _ => Ok(ast),
    }
}

fn is_macro_call(ast: &Type, env: &Rc<Env>) -> bool {
    let mut is_macro_call = false;

    if let Type::List(list) = ast {
        if let Some(Type::Symbol(sym)) = list.get(0) {
            is_macro_call = matches!(env.get(sym), Ok(Type::Closure { is_macro: true, .. }));
        }
    }

    is_macro_call
}

fn macroexpand(ast: Type, env: &Rc<Env>) -> Ret {
    let mut ast = ast;
    while is_macro_call(&ast, env) {
        let list = ast.convert_to_vec()?;
        let sym = match list[0] {
            Type::Symbol(ref sym) => sym,
            _ => return Err(Exception::interpreter_error("impossible situation")),
        };
        let macro_fun = env.get(sym)?;
        let args = if list.len() > 1 {
            list[1..].to_vec()
        } else {
            vec![]
        };
        ast = macro_fun.apply(args)?;
    }

    Ok(ast)
}

fn print(ast: Result<Type, Exception>) -> String {
    match ast {
        Ok(ast) => printer::pr_str(ast, true),
        Err(exc) => format!("Error: {}", exc.to_string()),
    }
}

pub fn rep(input: &str, env: &Rc<Env>) -> String {
    let parsed_input = read(input);
    match parsed_input {
        Ok(Some(ast)) => print(eval(ast, env)),
        Ok(None) => String::new(),
        Err(exc) => exc,
    }
}
