use std::cell::RefCell;
use std::rc::Rc;

use mal_rust;
use mal_rust::env::Env;

#[test]
fn testing_evaluation_of_arithmetic_operations() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("(+ 1 2)", &env), "3");
    assert_eq!(mal_rust::rep("(+ 5 (* 2 3))", &env), "11");
    assert_eq!(mal_rust::rep("(- (+ 5 (* 2 3)) 3)", &env), "8");
    assert_eq!(mal_rust::rep("(/ (- (+ 5 (* 2 3)) 3) 4)", &env), "2");
    assert_eq!(
        mal_rust::rep("(/ (- (+ 515 (* 87 311)) 302) 27)", &env),
        "1010"
    );
    assert_eq!(mal_rust::rep("(* -3 6)", &env), "-18");
    assert_eq!(
        mal_rust::rep("(/ (- (+ 515 (* -87 311)) 296) 27)", &env),
        "-994"
    );
}

#[test]
fn test_invalid_function_name() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(
        mal_rust::rep("(abc 1 2 3)", &env),
        "Symbol 'abc' not found in any environment"
    )
}

#[test]
fn testing_empty_list() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("()", &env), "()");
}

#[test]
fn testing_repl_env() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("(+ 1 2)", &env), "3");
    assert_eq!(mal_rust::rep("(/ (- (+ 5 (* 2 3)) 3) 4)", &env), "2");
}

#[test]
fn testing_evaluation_within_collection_literals() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("[1 2 (+ 1 2)]", &env), "[1 2 3]");

    assert_eq!(mal_rust::rep("{\"a\" (+ 7 8)}", &env), "{\"a\" 15}");

    assert_eq!(mal_rust::rep("{:a (+ 7 8)}", &env), "{:a 15}");
}

#[test]
fn check_that_evaluation_hasnt_broken_empty_collections() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("[]", &env), "[]");

    assert_eq!(mal_rust::rep("{}", &env), "{}");
}
