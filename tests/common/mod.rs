use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use mal_rust;
use mal_rust::env::Env;

pub fn load_core(env: &Rc<Env>) -> Result<(), Box<dyn Error>> {
    load_mal_file(env, "src/core.mal")
}

pub fn load_mal_file(env: &Rc<Env>, name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        mal_rust::rep(line, &env);
    }

    Ok(())
}
