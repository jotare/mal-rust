use mal_rust;

#[test]
fn testing_basic_string() {
    assert_eq!(mal_rust::rep("abcABC123"), "abcABC123");
}

#[test]
fn testing_string_containing_spaces() {
    assert_eq!(mal_rust::rep("hello mal world"), "hello mal world");
}

#[test]
fn testing_string_containing_symbols() {
    assert_eq!(mal_rust::rep("[]{}\"'* ;:()"), "[]{}\"'* ;:()");
}

#[test]
fn test_long_string() {
    assert_eq!(mal_rust::rep("hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}\"'* ;:() []{}\"'* ;:() []{}\"'*)"), "hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}\"'* ;:() []{}\"'* ;:() []{}\"'*)");
}

#[test]
fn non_alphanumeric_characters() {
    assert_eq!(mal_rust::rep("!"), "!");
    assert_eq!(mal_rust::rep("&"), "&");
    assert_eq!(mal_rust::rep("+"), "+");
    assert_eq!(mal_rust::rep(","), ",");
    assert_eq!(mal_rust::rep("-"), "-");
    assert_eq!(mal_rust::rep("/"), "/");
    assert_eq!(mal_rust::rep("<"), "<");
    assert_eq!(mal_rust::rep("="), "=");
    assert_eq!(mal_rust::rep(">"), ">");
    assert_eq!(mal_rust::rep("?"), "?");
    assert_eq!(mal_rust::rep("@"), "@");
    assert_eq!(mal_rust::rep("^"), "^");
    assert_eq!(mal_rust::rep("_"), "_");
    assert_eq!(mal_rust::rep("`"), "`");
    assert_eq!(mal_rust::rep("~"), "~");
}

#[test]
fn non_alphanumeric_characters_2() {
    assert_eq!(mal_rust::rep("#"), "#");
    assert_eq!(mal_rust::rep("$"), "$");
    assert_eq!(mal_rust::rep("%"), "%");
    assert_eq!(mal_rust::rep("."), ".");
    assert_eq!(mal_rust::rep("|"), "|");
}
