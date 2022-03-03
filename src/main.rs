use std::io::{self,Write};

use mal_rust;


fn main() {
    let prompt = "mal-rust> ";

    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let read = io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        if read == 0 {          // EOF
            println!();
            break
        }

        let output = mal_rust::rep(&input);
        print!("{}", output);
        io::stdout().flush().unwrap();
    }
}
