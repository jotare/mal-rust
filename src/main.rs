use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use mal_rust;
use mal_rust::env::Env;

fn main() -> Result<(), Box<dyn Error>> {
    let prompt = "mal-rust> ";
    let history = ".history";

    let mut rl = Editor::<()>::new();
    if rl.load_history(&history).is_err() {
        println!("Creating history at '{}'", history);
    }

    let env = Rc::new(Env::new_default());

    // Definitions using the interpreter itself
    let mut file = File::open("src/core.mal")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        mal_rust::rep(line, &env);
    }

    loop {
        let input = rl.readline(&prompt);
        match input {
            Ok(input) => {
                if input.trim().is_empty() {
                    continue;
                }
                rl.add_history_entry(input.as_str());
                let output = mal_rust::rep(&input, &env);
                if output.len() > 0 {
                    println!("{}", output);
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Failed to read: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(&history).unwrap();

    Ok(())
}
