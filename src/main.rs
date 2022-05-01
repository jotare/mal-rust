use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use mal_rust;
use mal_rust::env::Env;

fn main() -> Result<(), Box<dyn Error>> {
    let env = Rc::new(Env::new_default());

    // Definitions using the interpreter itself
    let mut file = File::open("src/core.mal")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        mal_rust::rep(line, &env);
    }

    let argv: Vec<String> = std::env::args().skip(1).collect();

    if argv.len() == 0 {        // interactive interpreter
        let prompt = "mal-rust> ";
        let history = ".history";

        let mut rl = Editor::<()>::new();
        if rl.load_history(&history).is_err() {
            println!("Creating history at '{}'", history);
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

    } else {                    // run file
        let script = &argv[0];
        let def_argv = format!( // redefine *ARGV* with command line args
            "(def! *ARGV* (list {}))",
            argv[1..]
                .iter()
                .map(|arg| format!("\"{}\"", arg))
                .collect::<Vec<String>>()
                .join(" ")
        );
        mal_rust::rep(&def_argv, &env);

        let output = mal_rust::rep(&format!("(load-file \"{}\")", script), &env);
        if !output.is_empty() && output != "nil" {
            println!("{}", output);
        }
    }

    Ok(())
}
