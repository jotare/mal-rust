use std::cell::RefCell;
use std::rc::Rc;

use mal_rust;
use mal_rust::env::Env;

#[test]
fn testing_def() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("(def! x 3)", &env), "3");
    assert_eq!(mal_rust::rep("x", &env), "3");
    assert_eq!(mal_rust::rep("(def! x 4)", &env), "4");
    assert_eq!(mal_rust::rep("x", &env), "4");
    assert_eq!(mal_rust::rep("(def! y (+ 1 7))", &env), "8");
    assert_eq!(mal_rust::rep("y", &env), "8");
}

#[test]
fn verifying_symbols_are_case_sensitive() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("(def! mynum 111)", &env), "111");
    assert_eq!(mal_rust::rep("(def! MYNUM 222)", &env), "222");
    assert_eq!(mal_rust::rep("mynum", &env), "111");
    assert_eq!(mal_rust::rep("MYNUM", &env), "222");
}

#[test]
fn check_that_error_aborts_def() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(
        mal_rust::rep("(abc 1 2 3)", &env),
        "Symbol 'abc' not found in any environment"
    );
    assert_eq!(
        mal_rust::rep("(def! w 123)\n(def! w (abc))\nw", &env),
        "123"
    );
}

#[test]
fn testing_let() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("(let* (z 9) z)", &env), "9");
    assert_eq!(mal_rust::rep("(let* (x 9) x)", &env), "9");
    assert_eq!(mal_rust::rep("(let* (z (+ 2 3)) (+ 1 z))", &env), "6");
    assert_eq!(
        mal_rust::rep("(let* (p (+ 2 3) q (+ 2 p)) (+ p q))", &env),
        "12"
    );
    assert_eq!(mal_rust::rep("(def! y (let* (z 7) z))\ny", &env), "7");
}

#[test]
fn testing_outer_environment() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("(def! a 4)", &env), "4");
    assert_eq!(mal_rust::rep("(let* (q 9) q)", &env), "9");
    assert_eq!(mal_rust::rep("(let* (q 9) a)", &env), "4");
    assert_eq!(mal_rust::rep("(let* (z 2) (let* (q 9) a))", &env), "4");
}

#[test]
fn testing_let_with_vector_bindings() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("(let* [z 9] z)", &env), "9");
    assert_eq!(
        mal_rust::rep("(let* [p (+ 2 3) q (+ 2 p)] (+ p q))", &env),
        "12"
    );
}

#[test]
fn testing_vector_evaluation() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(
        mal_rust::rep("(let* (a 5 b 6) [3 4 a [b 7] 8])", &env),
        "[3 4 5 [6 7] 8]"
    );
}

#[test]
fn check_that_last_assignment_takes_priority() {
    let env = Rc::new(RefCell::new(Env::new_default()));
    assert_eq!(mal_rust::rep("(let* (x 2 x 3) x)", &env), "3");
}
