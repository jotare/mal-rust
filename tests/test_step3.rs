use mal_rust;
use mal_rust::env::Env;

#[test]
fn testing_def() {
    let mut env = Env::new_default();
    assert_eq!(mal_rust::rep("(def! x 3)", &mut env), "3");
    assert_eq!(mal_rust::rep("x", &mut env), "3");
    assert_eq!(mal_rust::rep("(def! x 4)", &mut env), "4");
    assert_eq!(mal_rust::rep("x", &mut env), "4");
    assert_eq!(mal_rust::rep("(def! y (+ 1 7))", &mut env), "8");
    assert_eq!(mal_rust::rep("y", &mut env), "8");
}

#[test]
fn verifying_symbols_are_case_sensitive() {
    let mut env = Env::new_default();
    assert_eq!(mal_rust::rep("(def! mynum 111)", &mut env), "111");
    assert_eq!(mal_rust::rep("(def! MYNUM 222)", &mut env), "222");
    assert_eq!(mal_rust::rep("mynum", &mut env), "111");
    assert_eq!(mal_rust::rep("MYNUM", &mut env), "222");
}

#[test]
fn check_that_error_aborts_def() {
    let mut env = Env::new_default();
    assert_eq!(
        mal_rust::rep("(abc 1 2 3)", &mut env),
        "Symbol 'abc' not found in any environment"
    );
    assert_eq!(
        mal_rust::rep("(def! w 123)\n(def! w (abc))\nw", &mut env),
        "123"
    );
}

#[test]
fn testing_let() {
    let mut env = Env::new_default();
    assert_eq!(mal_rust::rep("(let* (z 9) z)", &mut env), "9");
    assert_eq!(mal_rust::rep("(let* (x 9) x)", &mut env), "9");
    assert_eq!(mal_rust::rep("(let* (z (+ 2 3)) (+ 1 z))", &mut env), "6");
    assert_eq!(
        mal_rust::rep("(let* (p (+ 2 3) q (+ 2 p)) (+ p q))", &mut env),
        "12"
    );
    assert_eq!(mal_rust::rep("(def! y (let* (z 7) z))\ny", &mut env), "7");
}

#[test]
fn testing_outer_environment() {
    let mut env = Env::new_default();
    assert_eq!(mal_rust::rep("(def! a 4)", &mut env), "4");
    assert_eq!(mal_rust::rep("(let* (q 9) q)", &mut env), "9");
    assert_eq!(mal_rust::rep("(let* (q 9) a)", &mut env), "4");
    assert_eq!(mal_rust::rep("(let* (z 2) (let* (q 9) a))", &mut env), "4");
}

#[test]
#[ignore]
fn testing_let_with_vector_bindings() {
    let mut env = Env::new_default();
    assert_eq!(mal_rust::rep("(let* [z 9] z)", &mut env), "9");
    assert_eq!(
        mal_rust::rep("(let* [p (+ 2 3) q (+ 2 p)] (+ p q))", &mut env),
        "12"
    );
}

#[test]
#[ignore]
fn testing_vector_evaluation() {
    let mut env = Env::new_default();
    assert_eq!(
        mal_rust::rep("(let* (a 5 b 6) [3 4 a [b 7] 8])", &mut env),
        "[3 4 5 [6 7] 8]"
    );
}

#[test]
fn check_that_last_assignment_takes_priority() {
    let mut env = Env::new_default();
    assert_eq!(mal_rust::rep("(let* (x 2 x 3) x)", &mut env), "3");
}
