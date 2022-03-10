#[derive(Debug,PartialEq)]
pub enum MalType {
    Nil,
    True,
    False,
    Integer(i32),
    Symbol(String),
    List(Vec<Box<MalType>>),
}

#[derive(Debug,PartialEq)]
pub struct Ast {
    root: MalType,
}

impl Ast {
    pub fn new(root: MalType) -> Ast {
        Ast {
            root: root
        }
    }

    pub fn root(&self) -> &MalType {
        &self.root
    }
}
