use crate::printer;
use crate::types::Type;

#[derive(Debug)]
pub enum Exception {
    Builtin(String),
    Custom(Type),
}

impl Exception {
    pub fn builtin(exc: &str) -> Exception {
        Exception::Builtin(exc.to_string())
    }

    pub fn custom(exc: Type) -> Exception {
        Exception::Custom(exc)
    }

    pub fn interpreter_error(exc: &str) -> Exception {
        Exception::builtin(&format!("Interpreter error: {}", exc))
    }

    pub fn type_error(exc: &str) -> Exception {
        Exception::builtin(&format!("Type error: {}", exc))
    }

    pub fn nargs_error(fun_name: &str, nargs: usize, given: usize) -> Exception {
        Exception::type_error(&format!(
            "'{}' takes exactly {} argument ({} given)",
            fun_name, nargs, given
        ))
    }

    pub fn numeric_fun(fun_name: &str) -> Exception {
        Exception::type_error(&format!("{} only accepts numeric types", fun_name))
    }

    pub fn string_fun(fun_name: &str) -> Exception {
        Exception::type_error(&format!("{} only accepts string type", fun_name))
    }

    pub fn seq_fun(fun_name: &str) -> Exception {
        Exception::type_error(&format!("{} only accepts sequence types", fun_name))
    }

    pub fn atom_fun(fun_name: &str) -> Exception {
        Exception::type_error(&format!("{} only accepts atom type", fun_name))
    }

    pub fn fun_fun(fun_name: &str) -> Exception {
        Exception::type_error(&format!("{} only accepts function type", fun_name))
    }

    pub fn index_error(exc: &str) -> Exception {
        Exception::builtin(&format!("Index error: {}", exc))
    }

    pub fn index_out_of_bounds(idx: usize, length: usize) -> Exception {
        Exception::index_error(&format!(
            "index {} is out of bouns for sequence of length {}",
            idx, length
        ))
    }

    pub fn negative_index() -> Exception {
        Exception::index_error("can't use a negative index")
    }
}

impl From<String> for Exception {
    fn from(s: String) -> Self {
        Exception::builtin(&s)
    }
}

impl ToString for Exception {
    fn to_string(&self) -> String {
        match self {
            Exception::Builtin(s) => s.to_owned(),
            Exception::Custom(t) => printer::pr_str(t.to_owned(), true),
        }
    }
}

pub fn nargs_check(fun_name: &str, required: usize, given: usize) -> Result<(), Exception> {
    if required != given {
        Err(Exception::nargs_error(fun_name, required, given))
    } else {
        Ok(())
    }
}
