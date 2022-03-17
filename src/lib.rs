mod env;
mod printer;
mod reader;
mod types;

use crate::{types::{Args, Ret, Type}, env::Env};

fn read(input: &str) -> Type {
    reader::read_str(input)
}

fn eval(ast: Type, env: &Env) -> Ret {
    match ast {
        Type::List(list) => {
            if list.len() == 0 {
                Ok(Type::List(list))
            } else {
                // eval list and call first item as a function and the
                // rest as its arguments
                let list = match eval_ast(Type::List(list), &env) {
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
        other => eval_ast(other, &env),
    }
}

fn eval_ast(ast: Type, env: &Env) -> Ret {
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

pub fn rep(input: &str) -> String {
    fn sum(args: Args) -> Ret {
        match (args.get(0), args.get(1)) {
            (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a + *b)),
            (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a + *b as f64)),
            (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 + *b)),
            (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a + *b)),
            _ => Err(String::from("'+' operation failed. Types must be numeric (Int or Float)")),
        }
    }

    fn sub(args: Args) -> Ret {
        match (args.get(0), args.get(1)) {
            (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a - *b)),
            (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a - *b as f64)),
            (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 - *b)),
            (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a - *b)),
            _ => Err(String::from("'-' operation failed. Types must be numeric (Int or Float)")),
        }
    }

    fn mul(args: Args) -> Ret {
        match (args.get(0), args.get(1)) {
            (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a * *b)),
            (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a * *b as f64)),
            (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 * *b)),
            (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a * *b)),
            _ => Err(String::from("'*' operation failed. Types must be numeric (Int or Float)")),
        }
    }

    fn div(args: Args) -> Ret {
        match (args.get(0), args.get(1)) {
            (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a / *b)),
            (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a / *b as f64)),
            (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 / *b)),
            (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a / *b)),
            _ => Err(String::from("'/' operation failed. Types must be numeric (Int or Float)")),
        }
    }

    let mut repl_env = Env::new(None);
    repl_env.set("+", Type::Fun(sum));
    repl_env.set("-", Type::Fun(sub));
    repl_env.set("*", Type::Fun(mul));
    repl_env.set("/", Type::Fun(div));

    print(eval(read(input), &repl_env))
}
