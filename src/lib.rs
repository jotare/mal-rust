mod printer;
mod reader;
mod types;

use std::collections::HashMap;

use crate::types::{Args, Function, Ret, Type};

fn read(input: &str) -> Type {
    reader::read_str(input)
}

fn eval(ast: Type, env: HashMap<&str, Function>) -> Ret {
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

fn eval_ast(ast: Type, env: &HashMap<&str, Function>) -> Ret {
    match ast {
        Type::Symbol(sym) => match env.get(sym.as_str()) {
            Some(fun) => Ok(Type::Fun(*fun)),
            None => Err(format!("Symbol '{}' not found in the environment", sym)),
        },

        Type::List(list) => {
            let mut evaluated = Vec::with_capacity(list.len());
            for elem in list {
                let elem = eval(*elem, env.clone())?;
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
    let mut repl_env: HashMap<&str, Function> = HashMap::new();

    fn sum(args: Args) -> Ret {
        match (args.get(0), args.get(1)) {
            (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a + *b)),
            _ => Err(String::from("'+' operation failed. Need two Int arguments")),
        }
    }
    repl_env.insert("+", sum);

    fn sub(args: Args) -> Ret {
        match (args.get(0), args.get(1)) {
            (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a - *b)),
            _ => Err(String::from("'-' operation failed. Need two Int arguments")),
        }
    }
    repl_env.insert("-", sub);

    fn mul(args: Args) -> Ret {
        match (args.get(0), args.get(1)) {
            (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a * *b)),
            _ => Err(String::from("'*' operation failed. Need two Int arguments")),
        }
    }
    repl_env.insert("*", mul);

    fn div(args: Args) -> Ret {
        match (args.get(0), args.get(1)) {
            (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a / *b)),
            _ => Err(String::from("'/' operation failed. Need two Int arguments")),
        }
    }
    repl_env.insert("/", div);

    print(eval(read(input), repl_env))
}
