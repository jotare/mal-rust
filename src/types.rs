#[derive(Debug,PartialEq)]
pub enum Type {
    Nil,
    Bool(bool),
    Int(i32),
    Symbol(String),
    List(Vec<Box<Type>>),
}

