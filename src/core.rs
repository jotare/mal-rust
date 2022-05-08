use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use crate::env::Env;
use crate::eval;
use crate::printer::pr_str;
use crate::reader::read_str;
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
        ns.data.insert(String::from("read-string"), read_string);
        ns.data.insert(String::from("slurp"), slurp);
        ns.data.insert(String::from("list"), list);
        ns.data.insert(String::from("list?"), is_list);
        ns.data.insert(String::from("empty?"), is_empty);
        ns.data.insert(String::from("count"), count);
        ns.data.insert(String::from("="), eq);
        ns.data.insert(String::from("<"), lt);
        ns.data.insert(String::from("<="), lte);
        ns.data.insert(String::from(">"), gt);
        ns.data.insert(String::from(">="), gte);
        ns.data.insert(String::from("atom"), atom);
        ns.data.insert(String::from("atom?"), atomp);
        ns.data.insert(String::from("deref"), deref);
        ns.data.insert(String::from("reset!"), reset);
        ns.data.insert(String::from("swap!"), swap);
        ns.data.insert(String::from("cons"), cons);
        ns.data.insert(String::from("concat"), concat);
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
        .map(|arg| pr_str(arg.to_owned(), false))
        .collect::<Vec<String>>()
        .join("");

    Ok(Type::String(s))
}

fn prn(args: Args) -> Ret {
    let s = args
        .iter()
        .map(|arg| pr_str(arg.to_owned(), true))
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", s);
    Ok(Type::Nil)
}

fn println(args: Args) -> Ret {
    let s = args
        .iter()
        .map(|arg| pr_str(arg.to_owned(), false))
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", s);
    Ok(Type::Nil)
}

fn read_string(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::String(input)) => match read_str(input) {
            Ok(Some(s)) => Ok(s),
            Ok(None) => Err(String::new()),
            Err(e) => Err(e),
        },
        _ => Err("Type error: must pass a string to read-string".to_string()),
    }
}

fn slurp(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::String(ref filename)) => {
            let mut file = File::open(filename).map_err(|e| e.to_string())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| e.to_string())?;
            Ok(Type::String(contents))
        }
        _ => Err("Type error: must pass a string to slurp".to_string()),
    }
}

fn list(args: Args) -> Ret {
    Ok(Type::List(args.to_vec()))
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
        _ => Err("Type error: 'empty?' is only supported for sequences".to_string()),
    }
}

fn count(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::List(seq) | Type::Vector(seq)) => Ok(Type::Int(seq.len() as i32)),
        Some(Type::Nil) => Ok(Type::Int(0)),
        _ => Err("Type error: 'count' is only supported for List".to_string()),
    }
}

fn eq(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => Ok(Type::Bool(a == b)),
        _ => Err("Type error: must pass 2 arguments to '='".to_string()),
    }
}

fn lt(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a < b))
        }
        _ => Err("Type error: must pass 2 arguments to '<'".to_string()),
    }
}

fn lte(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a <= b))
        }
        _ => Err("Type error: must pass 2 arguments to '<='".to_string()),
    }
}

fn gt(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a > b))
        }
        _ => Err("Type error: must pass 2 arguments to '>'".to_string()),
    }
}

fn gte(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(a), Some(b)) => {
            let a = a.convert_to_f64()?;
            let b = b.convert_to_f64()?;
            Ok(Type::Bool(a >= b))
        }
        _ => Err("Type error: must pass 2 arguments to '>='".to_string()),
    }
}

fn atom(args: Args) -> Ret {
    match args.get(0) {
        Some(a) => Ok(Type::Atom(Rc::new(RefCell::new(a.clone())))),
        None => Err("Type error: must pass an argument to 'atom'".to_string()),
    }
}

fn atomp(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::Atom(_)) => Ok(Type::Bool(true)),
        Some(_) => Ok(Type::Bool(false)),
        None => Err("Type error: must pass an argument to 'atom?'".to_string()),
    }
}

fn deref(args: Args) -> Ret {
    match args.get(0) {
        Some(Type::Atom(atom)) => Ok(atom.borrow().clone()),
        Some(_) => Err("Type error: must pass an atom to 'deref'".to_string()),
        None => Err("Type error: must pass an argument to 'deref'".to_string()),
    }
}

fn reset(args: Args) -> Ret {
    match (args.get(0), args.get(1)) {
        (Some(_), Some(Type::Atom(_))) => {
            Err("Type error: must pass a non atom type as second parameter".to_string())
        }
        (Some(Type::Atom(atom)), Some(value)) => {
            let mut atom_ref = (**atom).borrow_mut();
            *atom_ref = value.clone();
            Ok(value.clone())
        }
        _ => Err("Type error: must pass two arguments to 'reset!'".to_string()),
    }
}

/// Takes an atom, a function and zero or more arguments. Modifies
/// the atoms value to the result of applying the function as the
/// first parameter and the optionally given function arguments as
/// the rest of the arguments.
///
/// Examples:
/// (swap! atom (fn* (a) (* 2 a))) -- atom is now its old value x2
/// (swap! atom (fn* (a b) (+ a b)) 10) -- atom is now its old value +10
fn swap(args: Args) -> Ret {
    if args.len() < 2 {
        return Err("Type error: must pass at least two arguments to 'swap!'".to_string());
    }

    let (atom, atom_value) = match args.get(0) {
        Some(Type::Atom(a)) => (a, a.borrow().clone()),
        _ => return Err("Type error: first argument to 'swap!' must be an atom".to_string()),
    };

    let mut f_args = Vec::with_capacity(1 + args[2..].len());
    f_args.push(atom_value);
    for arg in args[2..].iter() {
        f_args.push(arg.to_owned())
    }

    let new_atom_value = match args.get(1) {
        Some(Type::Fun(fun)) => fun(f_args)?,
        Some(Type::Closure { env, params, body }) => {
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
                _ => return Err("Interpreter error: malformed closure!".to_string()),
            };

            let fun_env = Env::new(Some(env.clone()), &params, f_args.as_slice());

            eval(*body.to_owned(), &Rc::new(fun_env))?
        }
        _ => return Err("Type error: first argument to 'swap!' must be an function".to_string()),
    };

    *atom.borrow_mut() = new_atom_value.clone();

    Ok(new_atom_value)
}

/// Takes as its second parameter and returns a new list that has the
/// first argument prepended to it
///
/// Example:
/// (cons 1 (list 2 3)) -> (1 2 3)
fn cons(args: Args) -> Ret {
    if args.len() != 2 {
        return Err("Type error: must pass two arguments to 'cons'".to_string());
    }

    match (args.get(0), args.get(1)) {
        (Some(head), Some(Type::List(tail))) | (Some(head), Some(Type::Vector(tail))) => {
            let mut list = tail.clone();
            list.insert(0, head.to_owned());
            Ok(Type::List(list))
        }
        _ => Err("Type error: second 'cons' argument must be a sequence".to_string()),
    }
}

/// Take 0 or more lists as parameters and returns a new list that is
/// a concatenation of all the list parameters.
///
/// Example:
/// (concat (list 1 2) (list 3 4)) -> (1 2 3 4)
fn concat(args: Args) -> Ret {
    let mut list = Vec::new();
    for arg in args {
        match arg {
            Type::List(arg) | Type::Vector(arg) => {
                list.extend(arg);
            }
            _ => return Err("Type error: 'concat' arguments must be sequences".to_string()),
        }
    }
    Ok(Type::List(list))
}
