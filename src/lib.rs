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
                // eval list and call first item as a function and the
                // rest as its arguments
                let list = match eval_ast(Type::List(list), env) {
                    Ok(Type::List(list)) => list,
                    Ok(_) => return Err(String::from("Type can't not be a List")),
                    Err(e) => return Err(e),
                };

                let fun = list
                    .get(0)
                    .ok_or(format!("First argument must be a function!"))?;
                let fun = *fun.clone();
                match fun {
                    Type::Fun(fun) => {
                        let args = list[1..].iter().map(|a| *a.clone()).collect();
                        let ret = fun(args);
                        ret
                    }
                    _ => Err(format!("First argument must be a function!")),
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
