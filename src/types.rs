#[derive(Debug,PartialEq)]
pub enum Type {
    Nil,
    Bool(bool),
    Int(i32),
    Symbol(String),
    List(Vec<Box<Type>>),
}

#[derive(Debug,PartialEq)]
pub struct Ast {
    root: Type,
}

impl Ast {
    pub fn new(root: Type) -> Ast {
        Ast {
            root: root
        }
    }

    pub fn root(&self) -> &Type {
        &self.root
    }
}
