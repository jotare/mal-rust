use rustyline::Editor;
use rustyline::error::ReadlineError;

use mal_rust;


fn main() {
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
                rl.add_history_entry(input.as_str());
                let output = mal_rust::rep(&input);
                if output.len() > 0 {
                    println!("{}", output);
                }
            },
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                println!("Failed to read: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(&history).unwrap();
}
