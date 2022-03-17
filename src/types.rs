#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f64),
    Symbol(String),
    List(Vec<Box<Type>>),
    Fun(Function),
}

pub type Args = Vec<Type>;
pub type Ret = Result<Type, String>;
pub type Function = fn(Args) -> Ret;
