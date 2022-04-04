use std::rc::Rc;

use mal_rust;
use mal_rust::env::Env;

#[test]
fn testing_recursive_tail_call_functions() {
    let env = Rc::new(Env::new_default());

    mal_rust::rep(
        "(def! sum2 (fn* (n acc) (if (= n 0) acc (sum2 (- n 1) (+ n acc)))))",
        &env,
    );
    assert_eq!(mal_rust::rep("(sum2 10 0)", &env), "55");
    assert_eq!(mal_rust::rep("(def! res2 nil)", &env), "nil");
    mal_rust::rep("(def! res2 (sum2 10000 0))", &env);
    assert_eq!(mal_rust::rep("res2", &env), "50005000");
}

#[test]
fn test_mutually_recursive_tail_call_functions() {
    let env = Rc::new(Env::new_default());

    mal_rust::rep("(def! foo (fn* (n) (if (= n 0) 0 (bar (- n 1)))))", &env);
    mal_rust::rep("(def! bar (fn* (n) (if (= n 0) 0 (foo (- n 1)))))", &env);

    assert_eq!(mal_rust::rep("(foo 10000)", &env), "0");
}
