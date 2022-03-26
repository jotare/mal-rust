use std::rc::Rc;

use mal_rust;
use mal_rust::env::Env;

#[test]
fn testing_evaluation_of_arithmetic_operations() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(+ 1 2)", &mut env), "3");
    assert_eq!(mal_rust::rep("(+ 5 (* 2 3))", &mut env), "11");
    assert_eq!(mal_rust::rep("(- (+ 5 (* 2 3)) 3)", &mut env), "8");
    assert_eq!(mal_rust::rep("(/ (- (+ 5 (* 2 3)) 3) 4)", &mut env), "2");
    assert_eq!(
        mal_rust::rep("(/ (- (+ 515 (* 87 311)) 302) 27)", &mut env),
        "1010"
    );
    assert_eq!(mal_rust::rep("(* -3 6)", &mut env), "-18");
    assert_eq!(
        mal_rust::rep("(/ (- (+ 515 (* -87 311)) 296) 27)", &mut env),
        "-994"
    );
}

#[test]
fn test_invalid_function_name() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("(abc 1 2 3)", &mut env),
        "Symbol 'abc' not found in any environment"
    )
}

#[test]
fn testing_empty_list() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("()", &mut env), "()");
}

#[test]
fn testing_repl_env() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(+ 1 2)", &mut env), "3");
    assert_eq!(mal_rust::rep("(/ (- (+ 5 (* 2 3)) 3) 4)", &mut env), "2");
}

#[test]
#[ignore]
fn testing_evaluation_within_collection_literals() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("[1 2 (+ 1 2)]", &mut env), "[1 2 3]");

    assert_eq!(mal_rust::rep("{\"a\" (+ 7 8)}", &mut env), "{\"a\" 15}");

    assert_eq!(mal_rust::rep("{:a (+ 7 8)}", &mut env), "{:a 15}");
}

#[test]
#[ignore]
fn check_that_evaluation_hasnt_broken_empty_collections() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("[]", &mut env), "[]");

    assert_eq!(mal_rust::rep("{}", &mut env), "{}");
}
