use mal_rust;

#[test]
fn testing_evaluation_of_arithmetic_operations() {
    assert_eq!(mal_rust::rep("(+ 1 2)"), "3");
    assert_eq!(mal_rust::rep("(+ 5 (* 2 3))"), "11");
    assert_eq!(mal_rust::rep("(- (+ 5 (* 2 3)) 3)"), "8");
    assert_eq!(mal_rust::rep("(/ (- (+ 5 (* 2 3)) 3) 4)"), "2");
    assert_eq!(mal_rust::rep("(/ (- (+ 515 (* 87 311)) 302) 27)"), "1010");
    assert_eq!(mal_rust::rep("(* -3 6)"), "-18");
    assert_eq!(mal_rust::rep("(/ (- (+ 515 (* -87 311)) 296) 27)"), "-994");
}

#[test]
fn test_invalid_function_name() {
    assert_eq!(mal_rust::rep("(abc 1 2 3)"), "Symbol 'abc' not found in the environment")
}

#[test]
fn testing_empty_list() {
    assert_eq!(mal_rust::rep("()"), "()");
}
