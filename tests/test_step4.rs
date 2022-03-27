use std::rc::Rc;

use mal_rust;
use mal_rust::env::Env;

#[test]
fn testing_list_functions() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(list)", &mut env), "()");
    assert_eq!(mal_rust::rep("(list? (list))", &mut env), "true");
    assert_eq!(mal_rust::rep("(empty? (list))", &mut env), "true");
    assert_eq!(mal_rust::rep("(empty? (list 1))", &mut env), "false");
    assert_eq!(mal_rust::rep("(list 1 2 3)", &mut env), "(1 2 3)");
    assert_eq!(mal_rust::rep("(count (list 1 2 3))", &mut env), "3");
    assert_eq!(mal_rust::rep("(count (list))", &mut env), "0");
    assert_eq!(mal_rust::rep("(count nil)", &mut env), "0");
    assert_eq!(
        mal_rust::rep("(if (> (count (list 1 2 3)) 3) 89 78)", &mut env),
        "78"
    );
    assert_eq!(
        mal_rust::rep("(if (>= (count (list 1 2 3)) 3) 89 78)", &mut env),
        "89"
    );
}

#[test]
fn testing_if_form() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(if true 7 8)", &mut env), "7");
    assert_eq!(mal_rust::rep("(if false 7 8)", &mut env), "8");
    assert_eq!(mal_rust::rep("(if false 7 false)", &mut env), "false");
    assert_eq!(mal_rust::rep("(if true (+ 1 7) (+ 1 8))", &mut env), "8");
    assert_eq!(mal_rust::rep("(if false (+ 1 7) (+ 1 8))", &mut env), "9");
    assert_eq!(mal_rust::rep("(if nil 7 8)", &mut env), "8");
    assert_eq!(mal_rust::rep("(if 0 7 8)", &mut env), "7");
    assert_eq!(mal_rust::rep("(if (list) 7 8)", &mut env), "7");
    assert_eq!(mal_rust::rep("(if (list 1 2 3) 7 8)", &mut env), "7");
    assert_eq!(mal_rust::rep("(= (list) nil)", &mut env), "false");
}

#[test]
fn testing_1_way_if_form() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(if false (+ 1 7))", &mut env), "nil");
    assert_eq!(mal_rust::rep("(if nil 8)", &mut env), "nil");
    assert_eq!(mal_rust::rep("(if nil 8 7)", &mut env), "7");
    assert_eq!(mal_rust::rep("(if true (+ 1 7))", &mut env), "8");
}

#[test]
fn testing_basic_conditionals() {
    let mut env = Rc::new(Env::new_default());

    // Test '='
    assert_eq!(mal_rust::rep("(= 2 1)", &mut env), "false");
    assert_eq!(mal_rust::rep("(= 1 1)", &mut env), "true");
    assert_eq!(mal_rust::rep("(= 1 2)", &mut env), "false");
    assert_eq!(mal_rust::rep("(= 1 (+ 1 1))", &mut env), "false");
    assert_eq!(mal_rust::rep("(= 2 (+ 1 1))", &mut env), "true");
    assert_eq!(mal_rust::rep("(= nil 1)", &mut env), "false");
    assert_eq!(mal_rust::rep("(= nil nil)", &mut env), "true");

    // Test '>'
    assert_eq!(mal_rust::rep("(> 2 1)", &mut env), "true");
    assert_eq!(mal_rust::rep("(> 1 1)", &mut env), "false");
    assert_eq!(mal_rust::rep("(> 1 2)", &mut env), "false");

    // Test '>='
    assert_eq!(mal_rust::rep("(>= 2 1)", &mut env), "true");
    assert_eq!(mal_rust::rep("(>= 1 1)", &mut env), "true");
    assert_eq!(mal_rust::rep("(>= 1 2)", &mut env), "false");

    // Test '<'
    assert_eq!(mal_rust::rep("(< 2 1)", &mut env), "false");
    assert_eq!(mal_rust::rep("(< 1 1)", &mut env), "false");
    assert_eq!(mal_rust::rep("(< 1 2)", &mut env), "true");

    // Test: '<='
    assert_eq!(mal_rust::rep("(<= 2 1)", &mut env), "false");
    assert_eq!(mal_rust::rep("(<= 1 1)", &mut env), "true");
    assert_eq!(mal_rust::rep("(<= 1 2)", &mut env), "true");
}

#[test]
fn testing_equality() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(= 1 1)", &mut env), "true");
    assert_eq!(mal_rust::rep("(= 0 0)", &mut env), "true");
    assert_eq!(mal_rust::rep("(= 1 0)", &mut env), "false");
    assert_eq!(mal_rust::rep("(= true true)", &mut env), "true");
    assert_eq!(mal_rust::rep("(= false false)", &mut env), "true");
    assert_eq!(mal_rust::rep("(= nil nil)", &mut env), "true");

    assert_eq!(mal_rust::rep("(= (list) (list))", &mut env), "true");
    assert_eq!(mal_rust::rep("(= (list) ())", &mut env), "true");
    assert_eq!(mal_rust::rep("(= (list 1 2) (list 1 2))", &mut env), "true");
    assert_eq!(mal_rust::rep("(= (list 1) (list))", &mut env), "false");
    assert_eq!(mal_rust::rep("(= (list) (list 1))", &mut env), "false");
    assert_eq!(mal_rust::rep("(= 0 (list))", &mut env), "false");
    assert_eq!(mal_rust::rep("(= (list) 0)", &mut env), "false");
    assert_eq!(mal_rust::rep("(= (list nil) (list))", &mut env), "false");
}

#[test]
fn testing_builtin_and_user_defined_functions() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(+ 1 2)", &mut env), "3");
    assert_eq!(mal_rust::rep("( (fn* (a b) (+ b a)) 3 4)", &mut env), "7");
    assert_eq!(mal_rust::rep("( (fn* () 4) )", &mut env), "4");

    assert_eq!(
        mal_rust::rep("( (fn* (f x) (f x)) (fn* (a) (+ 1 a)) 7)", &mut env),
        "8"
    );
}

#[test]
fn testing_closures() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("( ( (fn* (a) (fn* (b) (+ a b))) 5) 7)", &mut env),
        "12"
    );

    assert_eq!(
        mal_rust::rep(
            "(def! gen-plus5 (fn* () (fn* (b) (+ 5 b))))\n(def! plus5 (gen-plus5))\n(plus5 7)",
            &mut env
        ),
        "12"
    );

    assert_eq!(
        mal_rust::rep(
            "(def! gen-plusX (fn* (x) (fn* (b) (+ x b))))\n(def! plus7 (gen-plusX 7))\n(plus7 8)",
            &mut env
        ),
        "15"
    );
}

#[test]
fn testing_do_form() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(do (prn 101))", &mut env), "101");
    assert_eq!(mal_rust::rep("", &mut env), "nil");
    assert_eq!(mal_rust::rep("(do (prn 102) 7)", &mut env), "102");
    assert_eq!(mal_rust::rep("", &mut env), "7");
    assert_eq!(
        mal_rust::rep("(do (prn 101) (prn 102) (+ 1 2))", &mut env),
        "101\n102\n3"
    );

    assert_eq!(mal_rust::rep("(do (def! a 6) 7 (+ a 8))", &mut env), "14");
    assert_eq!(mal_rust::rep("a", &mut env), "6");
}

#[test]
fn testing_special_form_case_sensitivity() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("(def! DO (fn* (a) 7))\n(DO 3)", &mut env),
        "7"
    );
}

#[test]
fn testing_recursive_sumdown_function() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep(
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))\n(sumdown 1)",
            &mut env
        ),
        "1"
    );
    assert_eq!(mal_rust::rep("(sumdown 2)", &mut env), "3");
    assert_eq!(mal_rust::rep("(sumdown 6)", &mut env), "21");
}

#[test]
fn testing_recursive_fibonacci_function() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))\n(fib 1)", &mut env),
        "1"
    );
    assert_eq!(mal_rust::rep("(fib 2)", &mut env), "2");
    assert_eq!(mal_rust::rep("(fib 4)", &mut env), "5");
}

#[test]
fn testing_recursive_function_in_environment() {
    let mut env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("(let* (f (fn* () x) x 3) (f))", &mut env),
        "3"
    );
    assert_eq!(
        mal_rust::rep(
            "(let* (cst (fn* (n) (if (= n 0) nil (cst (- n 1))))) (cst 1))",
            &mut env
        ),
        "nil"
    );
    assert_eq!(
        mal_rust::rep(
            "(let* (f (fn* (n) (if (= n 0) 0 (g (- n 1)))) g (fn* (n) (f n))) (f 2))",
            &mut env
        ),
        "0"
    );
}
