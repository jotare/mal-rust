use mal_rust;

#[test]
fn testing_read_of_numbers() {
    assert_eq!(mal_rust::rep("1"), "1");
    assert_eq!(mal_rust::rep("7"), "7");
    assert_eq!(mal_rust::rep("7"), "7");
    assert_eq!(mal_rust::rep("-123"), "-123");
}

#[test]
fn testing_read_of_symbols() {
    assert_eq!(mal_rust::rep("+"), "+");
    assert_eq!(mal_rust::rep("abc"), "abc");
    assert_eq!(mal_rust::rep("abc"), "abc");
    assert_eq!(mal_rust::rep("abc5"), "abc5");
    assert_eq!(mal_rust::rep("abc-def"), "abc-def");
}

#[test]
fn testing_non_numbers_starting_with_a_dash() {
    assert_eq!(mal_rust::rep("-"), "-");
    assert_eq!(mal_rust::rep("-abc"), "-abc");
    assert_eq!(mal_rust::rep("->>"), "->>");
}

#[test]
fn testing_read_of_lists() {
    assert_eq!(mal_rust::rep("(+ 1 2)"), "(+ 1 2)");
    assert_eq!(mal_rust::rep("()"), "()");
    assert_eq!(mal_rust::rep("( )"), "()");
    assert_eq!(mal_rust::rep("(nil)"), "(nil)");
    assert_eq!(mal_rust::rep("((3 4))"), "((3 4))");
    assert_eq!(mal_rust::rep("(+ 1 (+ 2 3))"), "(+ 1 (+ 2 3))");
    assert_eq!(mal_rust::rep("( +   1   (+   2 3   )   )"), "(+ 1 (+ 2 3))");
    assert_eq!(mal_rust::rep("(* 1 2)"), "(* 1 2)");
    assert_eq!(mal_rust::rep("(** 1 2)"), "(** 1 2)");
    assert_eq!(mal_rust::rep("(* -3 6)"), "(* -3 6)");
    assert_eq!(mal_rust::rep("(()())"), "(() ())");
}

#[test]
fn test_commas_as_whitespace() {
    assert_eq!(mal_rust::rep("(1 2, 3,,,,),,"), "(1 2 3)");
}
