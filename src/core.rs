use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;

use crate::env::Env;
use crate::error;
use crate::error::Exception;
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
        ns.data.insert(String::from("list?"), listp);
        ns.data.insert(String::from("empty?"), emptyp);
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
        ns.data.insert(String::from("vec"), vec);
        ns.data.insert(String::from("nth"), nth);
        ns.data.insert(String::from("first"), first);
        ns.data.insert(String::from("rest"), rest);
        ns.data.insert(String::from("throw"), throw);
        ns.data.insert(String::from("apply"), apply);
        ns.data.insert(String::from("map"), map);
        ns.data.insert(String::from("nil?"), nilp);
        ns.data.insert(String::from("true?"), truep);
        ns.data.insert(String::from("false?"), falsep);
        ns.data.insert(String::from("symbol?"), symbolp);
        ns.data.insert(String::from("symbol"), symbol);
        ns.data.insert(String::from("keyword?"), keywordp);
        ns.data.insert(String::from("keyword"), keyword);
        ns.data.insert(String::from("vector?"), vectorp);
        ns.data.insert(String::from("vector"), vector);
        ns.data.insert(String::from("sequential?"), sequentialp);
        ns.data.insert(String::from("hash-map"), hash_map);
        ns.data.insert(String::from("map?"), mapp);
        ns.data.insert(String::from("assoc"), assoc);
        ns.data.insert(String::from("dissoc"), dissoc);
        ns.data.insert(String::from("get"), get);
        ns.data.insert(String::from("contains?"), containsp);
        ns.data.insert(String::from("keys"), keys);
        ns.data.insert(String::from("vals"), vals);
        ns.data.insert(String::from("readline"), readline);
        ns.data.insert(String::from("time-ms"), time_ms);
        ns.data.insert(String::from("conj"), conj);
        ns.data.insert(String::from("string?"), stringp);
        ns.data.insert(String::from("number?"), numberp);
        ns.data.insert(String::from("fn?"), fnp);
        ns.data.insert(String::from("macro?"), macrop);
        ns.data.insert(String::from("seq"), seq);
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
    error::nargs_check("+", 2, args.len())?;

    match (&args[0], &args[1]) {
        (Type::Int(a), Type::Int(b)) => Ok(Type::Int(*a + *b)),
        (Type::Float(a), Type::Int(b)) => Ok(Type::Float(*a + *b as f64)),
        (Type::Int(a), Type::Float(b)) => Ok(Type::Float(*a as f64 + *b)),
        (Type::Float(a), Type::Float(b)) => Ok(Type::Float(*a + *b)),
        _ => Err(Exception::numeric_fun("+")),
    }
}

fn sub(args: Args) -> Ret {
    error::nargs_check("-", 2, args.len())?;

    match (&args[0], &args[1]) {
        (Type::Int(a), Type::Int(b)) => Ok(Type::Int(*a - *b)),
        (Type::Float(a), Type::Int(b)) => Ok(Type::Float(*a - *b as f64)),
        (Type::Int(a), Type::Float(b)) => Ok(Type::Float(*a as f64 - *b)),
        (Type::Float(a), Type::Float(b)) => Ok(Type::Float(*a - *b)),
        _ => Err(Exception::numeric_fun("-")),
    }
}

fn mul(args: Args) -> Ret {
    error::nargs_check("*", 2, args.len())?;

    match (&args[0], &args[1]) {
        (Type::Int(a), Type::Int(b)) => Ok(Type::Int(*a * *b)),
        (Type::Float(a), Type::Int(b)) => Ok(Type::Float(*a * *b as f64)),
        (Type::Int(a), Type::Float(b)) => Ok(Type::Float(*a as f64 * *b)),
        (Type::Float(a), Type::Float(b)) => Ok(Type::Float(*a * *b)),
        _ => Err(Exception::numeric_fun("*")),
    }
}

fn div(args: Args) -> Ret {
    error::nargs_check("/", 2, args.len())?;

    match (&args[0], &args[1]) {
        (Type::Int(a), Type::Int(b)) => Ok(Type::Int(*a / *b)),
        (Type::Float(a), Type::Int(b)) => Ok(Type::Float(*a / *b as f64)),
        (Type::Int(a), Type::Float(b)) => Ok(Type::Float(*a as f64 / *b)),
        (Type::Float(a), Type::Float(b)) => Ok(Type::Float(*a / *b)),
        _ => Err(Exception::numeric_fun("/")),
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
    error::nargs_check("read-string", 1, args.len())?;

    match &args[0] {
        Type::String(input) => match read_str(input) {
            Ok(Some(s)) => Ok(s),
            Ok(None) => Err(Exception::builtin("")),
            Err(e) => Err(Exception::builtin(&e)),
        },
        _ => Err(Exception::string_fun("read-string")),
    }
}

fn slurp(args: Args) -> Ret {
    error::nargs_check("slurp", 1, args.len())?;

    match &args[0] {
        Type::String(ref filename) => {
            let mut file = File::open(filename).map_err(|exc| {
                format!(
                    "IOError trying to open \"{}\": {}",
                    filename,
                    exc.to_string()
                )
            })?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(|exc| {
                format!(
                    "IOError trying to read \"{}\": {}",
                    filename,
                    exc.to_string()
                )
            })?;
            Ok(Type::String(contents))
        }
        _ => Err(Exception::string_fun("slurp")),
    }
}

fn list(args: Args) -> Ret {
    Ok(Type::List(args.to_vec()))
}

fn listp(args: Args) -> Ret {
    error::nargs_check("list?", 1, args.len())?;

    match &args[0] {
        Type::List(_) => Ok(Type::Bool(true)),
        _ => Ok(Type::Bool(false)),
    }
}

fn emptyp(args: Args) -> Ret {
    error::nargs_check("empty?", 1, args.len())?;

    match &args[0] {
        Type::List(seq) | Type::Vector(seq) => Ok(Type::Bool(seq.is_empty())),
        _ => Err(Exception::seq_fun("empty?")),
    }
}

fn count(args: Args) -> Ret {
    error::nargs_check("count", 1, args.len())?;

    match &args[0] {
        Type::List(seq) | Type::Vector(seq) => Ok(Type::Int(seq.len() as i64)),
        Type::Nil => Ok(Type::Int(0)),
        _ => Err(Exception::seq_fun("count")),
    }
}

fn eq(args: Args) -> Ret {
    error::nargs_check("=", 2, args.len())?;
    Ok(Type::Bool(args[0] == args[1]))
}

fn lt(args: Args) -> Ret {
    error::nargs_check("<", 2, args.len())?;

    let a = args[0].convert_to_f64();
    let b = args[1].convert_to_f64();

    if a.is_err() || b.is_err() {
        return Err(Exception::numeric_fun("<"));
    }
    let a = a.unwrap();
    let b = b.unwrap();

    Ok(Type::Bool(a < b))
}

fn lte(args: Args) -> Ret {
    error::nargs_check("<=", 2, args.len())?;

    let a = args[0].convert_to_f64();
    let b = args[1].convert_to_f64();

    if a.is_err() || b.is_err() {
        return Err(Exception::numeric_fun("<="));
    }
    let a = a.unwrap();
    let b = b.unwrap();

    Ok(Type::Bool(a <= b))
}

fn gt(args: Args) -> Ret {
    error::nargs_check(">", 2, args.len())?;

    let a = args[0].convert_to_f64();
    let b = args[1].convert_to_f64();

    if a.is_err() || b.is_err() {
        return Err(Exception::numeric_fun(">"));
    }
    let a = a.unwrap();
    let b = b.unwrap();

    Ok(Type::Bool(a > b))
}

fn gte(args: Args) -> Ret {
    error::nargs_check(">=", 2, args.len())?;

    let a = args[0].convert_to_f64();
    let b = args[1].convert_to_f64();

    if a.is_err() || b.is_err() {
        return Err(Exception::numeric_fun(">="));
    }
    let a = a.unwrap();
    let b = b.unwrap();

    Ok(Type::Bool(a >= b))
}

fn atom(args: Args) -> Ret {
    error::nargs_check("atom", 1, args.len())?;
    Ok(Type::Atom(Rc::new(RefCell::new(args[0].clone()))))
}

fn atomp(args: Args) -> Ret {
    error::nargs_check("atom?", 1, args.len())?;

    Ok(Type::Bool(matches!(args[0], Type::Atom(_))))
}

fn deref(args: Args) -> Ret {
    error::nargs_check("deref", 1, args.len())?;

    match &args[0] {
        Type::Atom(atom) => Ok(atom.borrow().clone()),
        _ => Err(Exception::atom_fun("deref")),
    }
}

fn reset(args: Args) -> Ret {
    error::nargs_check("reset", 2, args.len())?;

    match (&args[0], &args[1]) {
        (Type::Atom(atom), value) => {
            let mut atom_ref = (*atom).borrow_mut();
            *atom_ref = value.clone();
            Ok(value.clone())
        }
        (_, Type::Atom(_)) => Err(Exception::type_error(
            "Must pass a non atom as second parameter",
        )),
        _ => Err(Exception::type_error(
            "Must pass an atom and a non atom to 'reset'",
        )),
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
        return Err(Exception::type_error("'swap' takes at least two argument"));
    }

    let (atom, atom_value) = match args.get(0) {
        Some(Type::Atom(a)) => (a, a.borrow().clone()),
        _ => {
            return Err(Exception::type_error(
                "first argument to 'swap!' must be an atom",
            ))
        }
    };

    let mut f_args = Vec::with_capacity(1 + args[2..].len());
    f_args.push(atom_value);
    for arg in args[2..].iter() {
        f_args.push(arg.to_owned())
    }

    let new_atom_value = match args.get(1) {
        Some(Type::Fun(fun)) => fun(f_args)?,
        Some(Type::Closure {
            env, params, body, ..
        }) => {
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
                _ => return Err(Exception::interpreter_error("malformed closure!")),
            };

            let fun_env = Env::new(Some(env.clone()), &params, f_args.as_slice());

            eval(*body.to_owned(), &Rc::new(fun_env))?
        }
        _ => {
            return Err(Exception::type_error(
                "first argument to 'swap!' must be an function",
            ))
        }
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
    error::nargs_check("cons", 2, args.len())?;

    match (&args[0], &args[1]) {
        (head, Type::List(tail)) | (head, Type::Vector(tail)) => {
            let mut list = tail.clone();
            list.insert(0, head.to_owned());
            Ok(Type::List(list))
        }
        _ => Err(Exception::type_error(
            "second 'cons' argument must be a sequence",
        )),
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
            _ => return Err(Exception::seq_fun("concat")),
        }
    }
    Ok(Type::List(list))
}

/// Convert a List into a Vector with the same elements
fn vec(args: Args) -> Ret {
    error::nargs_check("vec", 1, args.len())?;

    match &args[0] {
        Type::List(v) => Ok(Type::Vector(v.to_owned())),
        Type::Vector(v) => Ok(Type::Vector(v.to_owned())),
        _ => Err(Exception::type_error(
            "'vec' only accepts arguments of sequence types",
        )),
    }
}

/// Take a list/vector and an index and return the element at the
/// given index. If the index is out of range, raises an error.
fn nth(args: Args) -> Ret {
    error::nargs_check("nth", 2, args.len())?;

    match (&args[0], &args[1]) {
        (Type::List(seq), Type::Int(idx)) | (Type::Vector(seq), Type::Int(idx)) => {
            if *idx < 0 {
                return Err(Exception::negative_index());
            }

            let idx = *idx as usize;
            if idx >= seq.len() {
                return Err(Exception::index_out_of_bounds(idx, seq.len()));
            }

            Ok(seq[idx].clone())
        }
        (_, Type::Int(_)) => Err(Exception::type_error("first argument must be an sequence")),
        (Type::List(_), _) | (Type::Vector(_), _) => {
            Err(Exception::type_error("second argument must be an integer"))
        }
        _ => Err(Exception::type_error("must pass a sequence and an integer")),
    }
}

/// Takes a list/vector as argument and return its first element. If
/// list is empty or nil, nil is returned.
fn first(args: Args) -> Ret {
    error::nargs_check("first", 1, args.len())?;

    match &args[0] {
        Type::List(seq) | Type::Vector(seq) => {
            if seq.is_empty() {
                Ok(Type::Nil)
            } else {
                Ok(seq[0].clone())
            }
        }
        _ => Ok(Type::Nil),
    }
}

/// Takes a list (or vector) as its argument and returns a new list
/// containing all the elements except the first. If the list (or
/// vector) is empty or is nil then () (empty list) is returned.
fn rest(args: Args) -> Ret {
    error::nargs_check("rest", 1, args.len())?;

    match &args[0] {
        Type::List(seq) | Type::Vector(seq) => {
            if seq.is_empty() {
                Ok(Type::List(vec![]))
            } else {
                Ok(Type::List(seq[1..].to_vec()))
            }
        }
        Type::Nil => Ok(Type::List(vec![])),
        _ => Ok(Type::Nil),
    }
}

/// Take a value and throw it as an exception
fn throw(args: Args) -> Ret {
    error::nargs_check("throw", 1, args.len())?;

    Err(Exception::custom(args[0].to_owned()))
}

fn apply(args: Args) -> Ret {
    if args.len() < 2 {
        return Err(Exception::type_error("'apply' takes at least two argument"));
    }

    let mut arguments = vec![];
    for arg in args[1..].iter() {
        match arg {
            Type::List(seq) | Type::Vector(seq) => arguments.extend(seq.to_owned()),
            t => arguments.push(t.to_owned()),
        }
    }

    args[0].apply(arguments)
}

fn map(args: Args) -> Ret {
    error::nargs_check("map", 2, args.len())?;

    let fun = args[0].to_owned();
    let arguments = match &args[1] {
        Type::List(seq) | Type::Vector(seq) => seq,
        _ => {
            return Err(Exception::type_error(
                "must pass a function and a list to 'map'",
            ))
        }
    };

    let mut result = vec![];
    for arg in arguments.iter() {
        result.push(fun.apply(vec![arg.to_owned()])?);
    }

    Ok(Type::List(result))
}

fn nilp(args: Args) -> Ret {
    error::nargs_check("nil?", 1, args.len())?;

    Ok(Type::Bool(matches!(args[0], Type::Nil)))
}

fn truep(args: Args) -> Ret {
    error::nargs_check("true?", 1, args.len())?;

    Ok(Type::Bool(matches!(args[0], Type::Bool(true))))
}

fn falsep(args: Args) -> Ret {
    error::nargs_check("false?", 1, args.len())?;

    Ok(Type::Bool(matches!(args[0], Type::Bool(false))))
}

fn symbolp(args: Args) -> Ret {
    error::nargs_check("symbol?", 1, args.len())?;

    Ok(Type::Bool(matches!(args[0], Type::Symbol(_))))
}

/// Convert a String in a symbol with the same content
fn symbol(args: Args) -> Ret {
    error::nargs_check("symbol", 1, args.len())?;

    match &args[0] {
        Type::String(s) => Ok(Type::Symbol(s.to_owned())),
        _ => Err(Exception::string_fun("symbol")),
    }
}

fn keywordp(args: Args) -> Ret {
    error::nargs_check("keyword?", 1, args.len())?;

    Ok(Type::Bool(matches!(args[0], Type::Keyword(_))))
}

/// Convert a String in a keyword with the same content
fn keyword(args: Args) -> Ret {
    error::nargs_check("keyword", 1, args.len())?;

    match &args[0] {
        Type::String(s) => Ok(Type::Keyword(s.to_owned())),
        Type::Keyword(_) => Ok(args[0].to_owned()),
        _ => Err(Exception::string_fun("keyword")),
    }
}

fn vectorp(args: Args) -> Ret {
    error::nargs_check("vector?", 1, args.len())?;

    Ok(Type::Bool(args[0].is_vector()))
}

/// Convert a String in a vector with the same content
fn vector(args: Args) -> Ret {
    Ok(Type::Vector(args))
}

fn sequentialp(args: Args) -> Ret {
    error::nargs_check("sequential?", 1, args.len())?;

    Ok(Type::Bool(args[0].is_sequence()))
}

fn hash_map(args: Args) -> Ret {
    if args.len() % 2 != 0 {
        return Err(Exception::type_error(
            "must pass an even number of arguments to 'hash-map'",
        ));
    }

    let mut hm = HashMap::new();

    if args.len() >= 2 {
        for i in 0..(args.len() / 2) {
            let key_idx = i * 2;
            let value_idx = i * 2 + 1;

            let key = match &args[key_idx] {
                Type::String(_) | Type::Keyword(_) => pr_str(args[key_idx].to_owned(), true),
                e => {
                    println!("Wrong type: {:?}", e);
                    return Err(Exception::type_error(
                        "must pass string or keyword as key in a hash map",
                    ));
                }
            };
            let value = Box::new(args[value_idx].to_owned());

            hm.insert(key, value);
        }
    }

    Ok(Type::HashMap(hm))
}

fn mapp(args: Args) -> Ret {
    error::nargs_check("map?", 1, args.len())?;

    Ok(Type::Bool(args[0].is_map()))
}

fn assoc(args: Args) -> Ret {
    if args.len() < 2 {
        return Err(Exception::type_error("'assoc' takes at least two argument"));
    } else if (args.len() - 1) % 2 != 0 {
        return Err(Exception::type_error(
            "must pass key value pairs to 'assoc'",
        ));
    }

    let mut hm = args[0].convert_to_map()?;
    for i in 1..(args.len() / 2 + 1) {
        let key_idx = (i - 1) * 2 + 1;
        let value_idx = i * 2;

        let key = match &args[key_idx] {
            Type::String(_) | Type::Keyword(_) => pr_str(args[key_idx].to_owned(), true),
            _ => {
                return Err(Exception::type_error(
                    "must pass string or keyword as key in a hash map (assoc)",
                ))
            }
        };
        let value = Box::new(args[value_idx].to_owned());

        hm.insert(key, value);
    }

    Ok(Type::HashMap(hm))
}

fn dissoc(args: Args) -> Ret {
    if args.len() < 2 {
        return Err(Exception::type_error("'assoc' takes at least two argument"));
    }

    let mut hm = args[0].convert_to_map()?;
    for i in 1..args.len() {
        let key = match &args[i] {
            Type::String(_) | Type::Keyword(_) => pr_str(args[i].to_owned(), true),
            _ => {
                return Err(Exception::type_error(
                    "must pass string or keyword as key in a hash map (dissoc)",
                ))
            }
        };
        hm.remove(&key);
    }

    Ok(Type::HashMap(hm))
}

fn get(args: Args) -> Ret {
    error::nargs_check("get", 2, args.len())?;

    match (&args[0], &args[1]) {
        (Type::HashMap(hm), Type::String(_)) | (Type::HashMap(hm), Type::Keyword(_)) => {
            let key = pr_str(args[1].to_owned(), true);
            match hm.get(&key) {
                Some(value) => Ok((**value).to_owned()),
                None => Ok(Type::Nil),
            }
        }
        (Type::Nil, _) => Ok(Type::Nil),
        _ => Err(Exception::type_error(
            "must pass a hash map and a string/keyword as arguments to 'get'",
        )),
    }
}

fn containsp(args: Args) -> Ret {
    error::nargs_check("contains?", 2, args.len())?;

    match (&args[0], &args[1]) {
        (Type::HashMap(hm), Type::String(_)) | (Type::HashMap(hm), Type::Keyword(_)) => {
            let key = pr_str(args[1].to_owned(), true);
            Ok(Type::Bool(hm.contains_key(&key)))
        }
        _ => Err(Exception::type_error(
            "must pass a hash map and a string/keyword as arguments to 'contains?'",
        )),
    }
}

fn keys(args: Args) -> Ret {
    error::nargs_check("keys", 1, args.len())?;

    if !args[0].is_map() {
        return Err(Exception::map_fun("keys"));
    }

    let hm = args[0].convert_to_map()?;
    Ok(Type::List(
        hm.keys()
            .map(|k| {
                if k.starts_with(':') {
                    Type::Keyword(k[1..].to_string())
                } else {
                    Type::String(k[1..(k.len() - 1)].to_string())
                }
            })
            .collect::<Vec<Type>>(),
    ))
}

fn vals(args: Args) -> Ret {
    error::nargs_check("keys", 1, args.len())?;

    if !args[0].is_map() {
        return Err(Exception::map_fun("keys"));
    }

    let hm = args[0].convert_to_map()?;
    Ok(Type::List(
        hm.values().map(|v| (**v).to_owned()).collect::<Vec<Type>>(),
    ))
}

fn readline(args: Args) -> Ret {
    error::nargs_check("readline", 1, args.len())?;

    let prompt = args[0].convert_to_string()?;
    print!("{}", prompt);
    if std::io::stdout().flush().is_err() {
        return Err(Exception::builtin("IOError printing prompt"));
    };

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("IOError reading a line");
    if line.is_empty() {
        return Ok(Type::Nil);
    }

    Ok(Type::String(line.trim().to_string()))
}

fn time_ms(args: Args) -> Ret {
    error::nargs_check("time-ms", 0, args.len())?;

    use std::time::SystemTime;
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|exc| format!("OS error: {}", exc))?
        .as_secs();
    Ok(Type::Int(since_epoch as i64))
}

fn conj(args: Args) -> Ret {
    match &args[0] {
        Type::List(list) => Ok(Type::List(
            args[1..]
                .iter()
                .rev()
                .chain(list.iter())
                .map(|t| t.to_owned())
                .collect::<Vec<Type>>(),
        )),

        Type::Vector(vector) => Ok(Type::Vector(
            vector
                .iter()
                .chain(args[1..].iter())
                .map(|t| t.to_owned())
                .collect::<Vec<Type>>(),
        )),

        _ => Ok(Type::Nil),
    }
}

fn stringp(args: Args) -> Ret {
    error::nargs_check("string?", 1, args.len())?;

    Ok(Type::Bool(args[0].is_string()))
}

fn numberp(args: Args) -> Ret {
    error::nargs_check("number?", 1, args.len())?;

    Ok(Type::Bool(args[0].is_number()))
}

fn fnp(args: Args) -> Ret {
    error::nargs_check("fn?", 1, args.len())?;

    Ok(Type::Bool(args[0].is_callable()))
}

fn macrop(args: Args) -> Ret {
    error::nargs_check("macro?", 1, args.len())?;

    Ok(Type::Bool(args[0].is_macro()))
}

fn seq(args: Args) -> Ret {
    error::nargs_check("seq", 1, args.len())?;

    match &args[0] {
        Type::Nil => Ok(Type::Nil),

        Type::List(l) if l.is_empty() => Ok(Type::Nil),
        Type::List(_) => Ok(args[0].to_owned()),

        Type::String(s) if s.is_empty() => Ok(Type::Nil),
        Type::String(s) => Ok(Type::List(
            s.chars()
                .map(|c| Type::String(c.to_string()))
                .collect::<Vec<Type>>(),
        )),

        Type::Vector(v) if v.is_empty() => Ok(Type::Nil),
        Type::Vector(v) => Ok(Type::List(v.to_owned())),

        _ => Err(Exception::type_error(
            "Must pass a list, vector, string or nil to 'seq'",
        )),
    }
}
