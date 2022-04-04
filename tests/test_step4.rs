use std::rc::Rc;

use mal_rust;
use mal_rust::env::Env;

#[test]
fn testing_list_functions() {
    let env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(list)", &env), "()");
    assert_eq!(mal_rust::rep("(list? (list))", &env), "true");
    assert_eq!(mal_rust::rep("(empty? (list))", &env), "true");
    assert_eq!(mal_rust::rep("(empty? (list 1))", &env), "false");
    assert_eq!(mal_rust::rep("(list 1 2 3)", &env), "(1 2 3)");
    assert_eq!(mal_rust::rep("(count (list 1 2 3))", &env), "3");
    assert_eq!(mal_rust::rep("(count (list))", &env), "0");
    assert_eq!(mal_rust::rep("(count nil)", &env), "0");
    assert_eq!(
        mal_rust::rep("(if (> (count (list 1 2 3)) 3) 89 78)", &env),
        "78"
    );
    assert_eq!(
        mal_rust::rep("(if (>= (count (list 1 2 3)) 3) 89 78)", &env),
        "89"
    );
}

#[test]
fn testing_if_form() {
    let env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(if true 7 8)", &env), "7");
    assert_eq!(mal_rust::rep("(if false 7 8)", &env), "8");
    assert_eq!(mal_rust::rep("(if false 7 false)", &env), "false");
    assert_eq!(mal_rust::rep("(if true (+ 1 7) (+ 1 8))", &env), "8");
    assert_eq!(mal_rust::rep("(if false (+ 1 7) (+ 1 8))", &env), "9");
    assert_eq!(mal_rust::rep("(if nil 7 8)", &env), "8");
    assert_eq!(mal_rust::rep("(if 0 7 8)", &env), "7");
    assert_eq!(mal_rust::rep("(if (list) 7 8)", &env), "7");
    assert_eq!(mal_rust::rep("(if (list 1 2 3) 7 8)", &env), "7");
    assert_eq!(mal_rust::rep("(= (list) nil)", &env), "false");
}

#[test]
fn testing_1_way_if_form() {
    let env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(if false (+ 1 7))", &env), "nil");
    assert_eq!(mal_rust::rep("(if nil 8)", &env), "nil");
    assert_eq!(mal_rust::rep("(if nil 8 7)", &env), "7");
    assert_eq!(mal_rust::rep("(if true (+ 1 7))", &env), "8");
}

#[test]
fn testing_basic_conditionals() {
    let env = Rc::new(Env::new_default());

    // Test '='
    assert_eq!(mal_rust::rep("(= 2 1)", &env), "false");
    assert_eq!(mal_rust::rep("(= 1 1)", &env), "true");
    assert_eq!(mal_rust::rep("(= 1 2)", &env), "false");
    assert_eq!(mal_rust::rep("(= 1 (+ 1 1))", &env), "false");
    assert_eq!(mal_rust::rep("(= 2 (+ 1 1))", &env), "true");
    assert_eq!(mal_rust::rep("(= nil 1)", &env), "false");
    assert_eq!(mal_rust::rep("(= nil nil)", &env), "true");

    // Test '>'
    assert_eq!(mal_rust::rep("(> 2 1)", &env), "true");
    assert_eq!(mal_rust::rep("(> 1 1)", &env), "false");
    assert_eq!(mal_rust::rep("(> 1 2)", &env), "false");

    // Test '>='
    assert_eq!(mal_rust::rep("(>= 2 1)", &env), "true");
    assert_eq!(mal_rust::rep("(>= 1 1)", &env), "true");
    assert_eq!(mal_rust::rep("(>= 1 2)", &env), "false");

    // Test '<'
    assert_eq!(mal_rust::rep("(< 2 1)", &env), "false");
    assert_eq!(mal_rust::rep("(< 1 1)", &env), "false");
    assert_eq!(mal_rust::rep("(< 1 2)", &env), "true");

    // Test: '<='
    assert_eq!(mal_rust::rep("(<= 2 1)", &env), "false");
    assert_eq!(mal_rust::rep("(<= 1 1)", &env), "true");
    assert_eq!(mal_rust::rep("(<= 1 2)", &env), "true");
}

#[test]
fn testing_equality() {
    let env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(= 1 1)", &env), "true");
    assert_eq!(mal_rust::rep("(= 0 0)", &env), "true");
    assert_eq!(mal_rust::rep("(= 1 0)", &env), "false");
    assert_eq!(mal_rust::rep("(= true true)", &env), "true");
    assert_eq!(mal_rust::rep("(= false false)", &env), "true");
    assert_eq!(mal_rust::rep("(= nil nil)", &env), "true");

    assert_eq!(mal_rust::rep("(= (list) (list))", &env), "true");
    assert_eq!(mal_rust::rep("(= (list) ())", &env), "true");
    assert_eq!(mal_rust::rep("(= (list 1 2) (list 1 2))", &env), "true");
    assert_eq!(mal_rust::rep("(= (list 1) (list))", &env), "false");
    assert_eq!(mal_rust::rep("(= (list) (list 1))", &env), "false");
    assert_eq!(mal_rust::rep("(= 0 (list))", &env), "false");
    assert_eq!(mal_rust::rep("(= (list) 0)", &env), "false");
    assert_eq!(mal_rust::rep("(= (list nil) (list))", &env), "false");
}

#[test]
fn testing_builtin_and_user_defined_functions() {
    let env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(+ 1 2)", &env), "3");
    assert_eq!(mal_rust::rep("( (fn* (a b) (+ b a)) 3 4)", &env), "7");
    assert_eq!(mal_rust::rep("( (fn* () 4) )", &env), "4");

    assert_eq!(
        mal_rust::rep("( (fn* (f x) (f x)) (fn* (a) (+ 1 a)) 7)", &env),
        "8"
    );
}

#[test]
fn testing_closures() {
    let env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("( ( (fn* (a) (fn* (b) (+ a b))) 5) 7)", &env),
        "12"
    );

    mal_rust::rep("(def! gen-plus5 (fn* () (fn* (b) (+ 5 b))))", &env);
    mal_rust::rep("(def! plus5 (gen-plus5))", &env);
    assert_eq!(mal_rust::rep("(plus5 7)", &env), "12");

    mal_rust::rep("(def! gen-plusX (fn* (x) (fn* (b) (+ x b))))", &env);
    mal_rust::rep("(def! plus7 (gen-plusX 7))", &env);
    assert_eq!(mal_rust::rep("(plus7 8)", &env), "15");
}

use std::io::Read;
use gag::BufferRedirect;

#[test]
fn testing_do_form() {
    let env = Rc::new(Env::new_default());

    assert_eq!(mal_rust::rep("(do (+ 2 3) (+ 4 5))", &env), "9");

    assert_eq!(mal_rust::rep("(do (def! a 6) 7 (+ a 8))", &env), "14");
    assert_eq!(mal_rust::rep("a", &env), "6");
}

#[test]
#[ignore]
fn testing_do_form_with_prn() {
    let env = Rc::new(Env::new_default());
    let mut buf = BufferRedirect::stdout().unwrap();
    let mut output = String::new();

    assert_eq!(mal_rust::rep("(do (prn 101))", &env), "nil");
    buf.read_to_string(&mut output).unwrap();
    assert_eq!(&output[..], "101\n");
    output.clear();

    assert_eq!(mal_rust::rep("(do (prn 102) 7)", &env), "7");
    buf.read_to_string(&mut output).unwrap();
    assert_eq!(&output[..], "102\n");
    output.clear();

    assert_eq!(
        mal_rust::rep("(do (prn 101) (prn 102) (+ 1 2))", &env),
        "3"
    );
    buf.read_to_string(&mut output).unwrap();
    assert_eq!(&output[..], "101\n102\n");
    output.clear();
}

#[test]
fn testing_special_form_case_sensitivity() {
    let env = Rc::new(Env::new_default());
    mal_rust::rep("(def! DO (fn* (a) 7))", &env);
    assert_eq!(mal_rust::rep("(DO 3)", &env), "7");
}

#[test]
fn testing_recursive_sumdown_function() {
    let env = Rc::new(Env::new_default());
    mal_rust::rep(
        "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
        &env,
    );
    assert_eq!(mal_rust::rep("(sumdown 1)", &env), "1");
    assert_eq!(mal_rust::rep("(sumdown 2)", &env), "3");
    assert_eq!(mal_rust::rep("(sumdown 6)", &env), "21");
}

#[test]
fn testing_recursive_fibonacci_function() {
    let env = Rc::new(Env::new_default());
    mal_rust::rep(
        "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
        &env,
    );
    assert_eq!(
        mal_rust::rep("(fib 1)", &env),
        "1"
    );
    assert_eq!(mal_rust::rep("(fib 2)", &env), "2");
    assert_eq!(mal_rust::rep("(fib 4)", &env), "5");
}

#[test]
fn testing_recursive_function_in_environment() {
    let env = Rc::new(Env::new_default());
    assert_eq!(mal_rust::rep("(let* (x 4 f (fn* () x) x 3) (f))", &env), "3");
    assert_eq!(
        mal_rust::rep(
            "(let* (cst (fn* (n) (if (= n 0) nil (cst (- n 1))))) (cst 1))",
            &env
        ),
        "nil"
    );
    assert_eq!(
        mal_rust::rep(
            "(let* (f (fn* (n) (if (= n 0) 0 (g (- n 1)))) g (fn* (n) (f n))) (f 2))",
            &env
        ),
        "0"
    );
}
