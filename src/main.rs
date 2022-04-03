use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use mal_rust;
use mal_rust::env::Env;

fn main() {
    let prompt = "mal-rust> ";
    let history = ".history";

    let mut rl = Editor::<()>::new();
    if rl.load_history(&history).is_err() {
        println!("Creating history at '{}'", history);
    }

    let env = Rc::new(Env::new_default());

    // Define `not` using the interpreter itself
    mal_rust::rep("(def! not (fn* (a) (if a false true)))", &env);

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
}
