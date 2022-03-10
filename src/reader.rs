use regex::Regex;

use crate::types::{MalType, Ast};

type Token = String;

struct Reader {
    tokens: Vec<Token>,
    position: usize,
}

/// Reader allow to permform syntax analysis over a token stream
impl Reader {
    fn new(tokens: Vec<Token>) -> Reader {
        Reader {
            tokens,
            position: 0,
        }
    }
    fn next(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn read_form(&mut self) -> MalType {
        let token = self.peek();

        if token.starts_with("(") {
            self.read_list()
        } else {
            self.read_atom()
        }
    }

    fn read_list(&mut self) -> MalType {
        
        let mut items = Vec::new();

        self.next();            // skip "("

        loop {
            let item = self.peek();

            if item == ")" {
                break
            } else {
                items.push(Box::new(self.read_form()));
            }

            if let None = self.next() {
                break
            }
        }

        MalType::List(items)
    }

    fn read_atom(&mut self) -> MalType {
        // String::parse + cast a f64, i32... per canviar de tipus. Alguna forma millor?

        let token = self.peek();

        if let Ok(number) = token.parse() {
            MalType::Integer(number)
        } else {
            match token.as_str() {
                "nil" => MalType::Nil,
                "true" => MalType::True,
                "false" => MalType::False,
                symbol => MalType::Symbol(String::from(symbol)),
            }
        }
    }
}


/// Reads a string of text and return a correct Abstract Syntax Tree
/// of the tokenized input.
pub fn read_str(input: &str) -> Ast {
    let tokens = tokenize(input);
    let mut reader = Reader::new(tokens);
    Ast::new(reader.read_form())
}

/// Tokenize the input stream and returns a list of tokens
pub fn tokenize(input: &str) -> Vec<Token> {
    let re = Regex::new(
        "[\\s ,]*(~@|[\\[\\]{}()'`~^@]|\"(?:\\\\.|[^\\\\\"])*\"?|;.*|[^\\s\\[\\]{}('\"`,;)]*)"
    ).unwrap();

    let tokens = re.captures_iter(input)
        .map(|capture| capture[1].to_owned())
        .collect();

    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        assert_eq!(
            tokenize("123"),
            vec![
                String::from("123")
            ]
        );

        assert_eq!(
            tokenize("abc"),
            vec![
                String::from("abc")
            ]
        );

        assert_eq!(
            tokenize("(123 456)"),
            vec![
                String::from("("),
                String::from("123"),
                String::from("456"),
                String::from(")"),
            ]
        );

        assert_eq!(
            tokenize("(123 456 789 )"),
            vec![
                String::from("("),
                String::from("123"),
                String::from("456"),
                String::from("789"),
                String::from(")"),
            ]
        );

        assert_eq!(
            tokenize("( + 2 (* 3 4) )"),
            vec![
                String::from("("),
                String::from("+"),
                String::from("2"),
                String::from("("),
                String::from("*"),
                String::from("3"),
                String::from("4"),
                String::from(")"),
                String::from(")"),
            ]
        );
    }

    #[test]
    fn test_read_str() {
        use crate::types::MalType;

        assert_eq!(
            read_str("123"),
            Ast::new(
                MalType::Integer(123)
            )
        );

        assert_eq!(
            read_str("abc"),
            Ast::new(
                MalType::Symbol(String::from("abc"))
            )
        );

        assert_eq!(
            read_str("(123 456)"),
            Ast::new(
                MalType::List(vec![
                    Box::new(MalType::Integer(123)),
                    Box::new(MalType::Integer(456)),
                ])
            )
        );

        assert_eq!(
            read_str("(123 456)"),
            Ast::new(
                MalType::List(vec![
                    Box::new(MalType::Integer(123)),
                    Box::new(MalType::Integer(456)),
                ])
            )
        );


        assert_eq!(
            read_str("( + 2 (* 3 4) )"),
            Ast::new(
                MalType::List(vec![
                    Box::new(MalType::Symbol(String::from("+"))),
                    Box::new(MalType::Integer(2)),
                    Box::new(MalType::List(vec![
                        Box::new(MalType::Symbol(String::from("*"))),
                        Box::new(MalType::Integer(3)),
                        Box::new(MalType::Integer(4)),
                    ])),
                ])
            )
        );
    }
}