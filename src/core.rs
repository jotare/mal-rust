use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::printer::pr_str;
use crate::types::{Args, Function, Ret, Type};

pub struct Namespace {
    data: HashMap<String, Function>,
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace {
            data: HashMap::new(),
        }
    }

    pub fn new_default() -> Namespace {
        let mut ns = Namespace::new();
        ns.data.insert(String::from("+"), sum);
        ns.data.insert(String::from("-"), sub);
        ns.data.insert(String::from("*"), mul);
        ns.data.insert(String::from("/"), div);
        ns.data.insert(String::from("pr-str"), pr_str_fun);
        ns.data.insert(String::from("str"), str_fun);
        ns.data.insert(String::from("prn"), prn);
        ns.data.insert(String::from("println"), println);
        ns.data.insert(String::from("list"), list);
        ns.data.insert(String::from("list?"), is_list);
        ns.data.insert(String::from("empty?"), is_empty);
        ns.data.insert(String::from("count"), count);
        ns.data.insert(String::from("="), eq);
        ns.data.insert(String::from("<"), lt);
        ns.data.insert(String::from("<="), lte);
        ns.data.insert(String::from(">"), gt);
        ns.data.insert(String::from(">="), gte);
        ns
    }
}

impl IntoIterator for Namespace {
    type Item = (String, Function);
    type IntoIter = std::collections::hash_map::IntoIter<String, Function>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

fn sum(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a + *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a + *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 + *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a + *b)),
        _ => Err(String::from(
            "Type error: '+' is only supported for Int and Float",
        )),
    }
}

fn sub(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a - *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a - *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 - *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a - *b)),
        _ => Err(String::from(
            "Type error: '-' is only supported for Int and Float",
        )),
    }
}

fn mul(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a * *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a * *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 * *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a * *b)),
        _ => Err(String::from(
            "Type error: '*' is only supported for Int and Float",
        )),
    }
}

fn div(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(Type::Int(a)), Some(Type::Int(b))) => Ok(Type::Int(*a / *b)),
        (Some(Type::Float(a)), Some(Type::Int(b))) => Ok(Type::Float(*a / *b as f64)),
        (Some(Type::Int(a)), Some(Type::Float(b))) => Ok(Type::Float(*a as f64 / *b)),
        (Some(Type::Float(a)), Some(Type::Float(b))) => Ok(Type::Float(*a / *b)),
        _ => Err(String::from(
            "Type error: '/' is only supported for Int and Float",
        )),
    }
}

fn pr_str_fun(args: Args) -> Ret {
    let s = args
        .iter()
        .map(|arg| pr_str(arg.clone(), true))
        .collect::<Vec<String>>()
        .join(" ");

    Ok(Type::String(s))
}

fn str_fun(args: Args) -> Ret {
    let s = args
        .iter()
        .map(|arg| {
            let mut s = pr_str(arg.clone(), false);
            lazy_static! {
                static ref RE: Regex = Regex::new(r#"(["][^"])|([^"]["])"#).unwrap();
            }
            s = RE
                .replace_all(&s, |cap: &Captures| match &cap[0] {
                    _ if cap[0].starts_with('"') => cap[0].replacen("\"", "", 1),
                    _ if cap[0].ends_with('"') => cap[0].replacen("\"", "", 1),
                    _ => panic!("Impossible capture {}", &cap[0]),
                })
                .to_string();

            if s.len() > 0 && s.starts_with('"') {
                s[1..s.len() - 1].to_string()
            } else {
                s
            }
        })
        .collect::<Vec<String>>()
        .join("");

    Ok(Type::String(s))
}

fn prn(args: Args) -> Ret {
    let s = args
        .iter()
        .map(|arg| pr_str(arg.clone(), true))
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", s);
    Ok(Type::Nil)
}

fn println(args: Args) -> Ret {
    let s = args
        .iter()
        .map(|arg| {
            let mut s = pr_str(arg.clone(), false);
            let re = Regex::new(r#"(["][^"])|([^"]["])"#).unwrap();
            s = re
                .replace_all(&s, |cap: &Captures| match &cap[0] {
                    _ if cap[0].starts_with('"') => cap[0].replacen("\"", "", 1),
                    _ if cap[0].ends_with('"') => cap[0].replacen("\"", "", 1),
                    _ => panic!("Impossible capture {}", &cap[0]),
                })
                .to_string();

            if s.len() > 0 && s.starts_with('"') {
                s[1..s.len() - 1].to_string()
            } else {
                s
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", s);
    Ok(Type::Nil)
}

fn list(args: Args) -> Ret {
    Ok(Type::List(
        args.iter().map(|a| Box::new(a.clone())).collect(),
    ))
}

fn is_list(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::List(_)) => Ok(Type::Bool(true)),
        _ => Ok(Type::Bool(false)),
    }
}

fn is_empty(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::List(seq) | Type::Vector(seq)) => Ok(Type::Bool(seq.len() == 0)),
        _ => Err(format!(
            "Type error: 'empty?' is only supported for sequences"
        )),
    }
}

fn count(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::List(seq) | Type::Vector(seq)) => Ok(Type::Int(seq.len() as i32)),
        Some(Type::Nil) => Ok(Type::Int(0)),
        _ => Err(format!("Type error: 'count' is only supported for List")),
    }
}

fn eq(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => Ok(Type::Bool(a == b)),
        _ => Err(format!("Value error: must pass 2 arguments to '='")),
    }
}

fn lt(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a < b))
        }
        _ => Err(format!("Value error: must pass 2 arguments to '<'")),
    }
}

fn lte(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a <= b))
        }
        _ => Err(format!("Value error: must pass 2 arguments to '<='")),
    }
}

fn gt(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a > b))
        }
        _ => Err(format!("Value error: must pass 2 arguments to '>'")),
    }
}

fn gte(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a >= b))
        }
        _ => Err(format!("Value error: must pass 2 arguments to '>='")),
    }
}
