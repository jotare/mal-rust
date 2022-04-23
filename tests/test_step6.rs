use std::rc::Rc;
use std::error::Error;

use mal_rust;
use mal_rust::env::Env;

mod common;


#[test]
fn testing_that_do_do_is_not_broken_by_tco() {
    let env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("(do (do 1 2))", &env),
        "2"
    );
}

#[test]
fn testing_read_string_eval_and_slurp() {
    let env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("(read-string \"(1 2 (3 4) nil)\")", &env),
        "(1 2 (3 4) nil)"
    );
    assert_eq!(
        mal_rust::rep("(= nil (read-string \"nil\"))", &env),
        "true"
    );
    assert_eq!(
        mal_rust::rep("(read-string \"(+ 2 3)\")", &env),
        "(+ 2 3)"
    );
    assert_eq!(
        mal_rust::rep("(read-string \"\\\"\n\\\"\")", &env),
        "\"\\n\""
    );
    assert_eq!(
        mal_rust::rep("(read-string \"7 ;; comment\")", &env),
        "7"
    );
    assert_eq!(
        mal_rust::rep("(read-string \";; comment\")", &env),
        ""
    );
    assert_eq!(
        mal_rust::rep("(eval (read-string \"(+ 2 3)\"))", &env),
        "5"
    );
    assert_eq!(
        mal_rust::rep("(slurp \"tests/res/test.txt\")", &env),
        "\"A line of text\\n\""
    );
    assert_eq!(
        mal_rust::rep("(slurp \"tests/res/test.txt\")", &env),
        "\"A line of text\\n\""
    );
}

#[test]
fn testing_load_file() -> Result<(), Box<dyn Error>> {
    let env = Rc::new(Env::new_default());
    common::load_core(&env)?;

    assert_eq!(
        mal_rust::rep("(load-file \"tests/res/inc.mal\")", &env),
        "nil"
    );
    assert_eq!(
        mal_rust::rep("(inc1 7)", &env),
        "8"
    );
    assert_eq!(
        mal_rust::rep("(inc2 7)", &env),
        "9"
    );
    assert_eq!(
        mal_rust::rep("(inc3 9)", &env),
        "12"
    );

    Ok(())
}

#[test]
fn testing_atoms() {
    let env = Rc::new(Env::new_default());
    mal_rust::rep("(def! inc3 (fn* (a) (+ 3 a)))", &env);
    assert_eq!(
        mal_rust::rep("(def! a (atom 2))", &env),
        "(atom 2)"
    );
    assert_eq!(
        mal_rust::rep("(atom? a)", &env),
        "true"
    );
    assert_eq!(
        mal_rust::rep("(atom? 1)", &env),
        "false"
    );
    assert_eq!(
        mal_rust::rep("(deref a)", &env),
        "2"
    );
    assert_eq!(
        mal_rust::rep("(reset! a 3)", &env),
        "3"
    );
    assert_eq!(
        mal_rust::rep("(deref a)", &env),
        "3"
    );
    assert_eq!(
        mal_rust::rep("(swap! a inc3)", &env),
        "6"
    );
    assert_eq!(
        mal_rust::rep("(deref a)", &env),
        "6"
    );
    assert_eq!(
        mal_rust::rep("(swap! a (fn* (a) a))", &env),
        "6"
    );
    assert_eq!(
        mal_rust::rep("(swap! a (fn* (a) (* 2 a)))", &env),
        "12"
    );
    assert_eq!(
        mal_rust::rep("(swap! a (fn* (a b) (* a b)) 10)", &env),
        "120"
    );
    assert_eq!(
        mal_rust::rep("(swap! a + 3)", &env),
        "123"
    );
}

#[test]
fn testing_swap_closure_interaction() {
    let env = Rc::new(Env::new_default());
    mal_rust::rep("(def! inc-it (fn* (a) (+ 1 a)))", &env);
    mal_rust::rep("(def! atm (atom 7))", &env);
    mal_rust::rep("(def! f (fn* () (swap! atm inc-it)))", &env);
    assert_eq!(
        mal_rust::rep("(f)", &env),
        "8"
    );
    assert_eq!(
        mal_rust::rep("(f)", &env),
        "9"
    );
}

#[test]
fn testing_whether_closures_can_retain_atoms() {
    let env = Rc::new(Env::new_default());
    mal_rust::rep("(def! g (let* (atm (atom 0)) (fn* () (deref atm))))", &env);
    mal_rust::rep("(def! atm (atom 1))", &env);
    assert_eq!(
        mal_rust::rep("(g)", &env),
        "0"
    );
}

#[test]
fn testing_reading_of_large_files() -> Result<(), Box<dyn Error>> {
    let env = Rc::new(Env::new_default());
    common::load_core(&env)?;

    assert_eq!(
        mal_rust::rep("(load-file \"tests/res/computations.mal\")", &env),
        "nil"
    );
    assert_eq!(
        mal_rust::rep("(sumdown 2)", &env),
        "3"
    );
    assert_eq!(
        mal_rust::rep("(fib 2)", &env),
        "1"
    );

    Ok(())
}

#[test]
fn testing_arroba_reader_macro_short_for_deref() {
    let env = Rc::new(Env::new_default());

    mal_rust::rep("(def! atm (atom 9))", &env);
    assert_eq!(
        mal_rust::rep("@atm", &env),
        "9"
    );
}

#[test]
fn testing_that_vector_params_not_broken_by_tco() {
    let env = Rc::new(Env::new_default());

    mal_rust::rep("(def! g (fn* [] 78))", &env);
    assert_eq!(
        mal_rust::rep("(g)", &env),
        "78"
    );

    mal_rust::rep("(def! g (fn* [a] (+ a 78)))", &env);
    assert_eq!(
        mal_rust::rep("(g 3)", &env),
        "81"
    );
}

#[test]
fn testing_that_argv_exists_and_is_an_empty_list() {
    let env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("(list? *ARGV*)", &env),
        "true"
    );
    assert_eq!(
        mal_rust::rep("*ARGV*", &env),
        "()"
    );
}

#[test]
fn testing_that_eval_sets_aa_in_root_scope_and_that_it_is_found_in_nested_scope() {
    let env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("(let* (b 12) (do (eval (read-string \"(def! aa 7)\")) aa ))", &env),
        "7"
    );
}

#[test]
fn testing_comments_in_a_file() -> Result<(), Box<dyn Error>> {
    let env = Rc::new(Env::new_default());
    common::load_core(&env)?;

    assert_eq!(
        mal_rust::rep("(load-file \"tests/res/incB.mal\")", &env),
        "nil"
    );
    assert_eq!(
        mal_rust::rep("(inc4 7)", &env),
        "11"
    );
    assert_eq!(
        mal_rust::rep("(inc5 7)", &env),
        "12"
    );

    Ok(())
}

#[test]
fn testing_map_literal_across_multiple_lines_in_a_file() -> Result<(), Box<dyn Error>>  {
    let env = Rc::new(Env::new_default());
    common::load_core(&env)?;

    assert_eq!(
        mal_rust::rep("(load-file \"tests/res/incC.mal\")", &env),
        "nil"
    );
    assert_eq!(
        mal_rust::rep("mymap", &env),
        "{\"a\" 1}"
    );

    Ok(())
}

#[test]
fn checking_that_eval_does_not_use_local_environments() {
    let env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep("(def! a 1)", &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep("(let* (a 2) (eval (read-string \"a\")))", &env),
        "1"
    );
}

#[test]
fn non_alphanumeric_characters_in_comments_in_read_string() {
    let env = Rc::new(Env::new_default());
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;!")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;\"")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;#")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;$")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;%")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;'")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;\\")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;\\\\")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;\\\\\\")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1;`")"#, &env),
        "1"
    );
    assert_eq!(
        mal_rust::rep(r#"(read-string "1; &()*+,-./:;<=>?@[]^_{|}~")"#, &env),
        "1"
    );
}

