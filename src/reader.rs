use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::types::Type;

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

    fn read_form(&mut self) -> Type {
        let token = self.peek();

        match token.chars().nth(0).unwrap() {
            '(' => self.read_list(),
            '[' => self.read_vector(),
            '{' => self.read_hash_map(),
            '"' => self.read_string(),
            ':' => self.read_keyword(),
            _ => self.read_atom(),
        }
    }

    fn read_list(&mut self) -> Type {
        Type::List(self.read_seq(")"))
    }

    fn read_vector(&mut self) -> Type {
        Type::Vector(self.read_seq("]"))
    }

    fn read_seq(&mut self, end: &str) -> Vec<Box<Type>> {
        let mut items = Vec::new();

        self.next(); // skip "(", "["

        loop {
            let item = self.peek();

            if item == end {
                break;
            } else {
                items.push(Box::new(self.read_form()));
            }

            if let None = self.next() {
                break;
            }
        }

        items
    }

    fn read_hash_map(&mut self) -> Type {
        let mut hash_map = HashMap::new();

        self.next(); // skip "{"

        let mut key = None;
        loop {
            let item = self.peek();

            if item == "}" {
                break;
            } else {
                match key {
                    Some(k) => {
                        hash_map.insert(k, Box::new(self.read_form()));
                        key = None
                    }
                    None => key = Some(item.to_owned()),
                }
            }
            if let None = self.next() {
                break;
            }
        }

        Type::HashMap(hash_map)
    }

    fn read_keyword(&mut self) -> Type {
        let token = self.peek();
        Type::Keyword(token[1..].to_string())
    }

    fn read_atom(&mut self) -> Type {
        let token = self.peek();

        match token.as_str() {
            "nil" => Type::Nil,
            "true" => Type::Bool(true),
            "false" => Type::Bool(false),
            other => {
                if let Ok(number) = token.parse() {
                    if token.contains(".") {
                        Type::Float(number)
                    } else {
                        Type::Int(number as i32)
                    }
                } else {
                    Type::Symbol(other.to_owned())
                }
            }
        }
    }

    fn read_string(&mut self) -> Type {
        let token = self.peek();
        let token = token[1..token.len() - 1].to_string();
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"(\\n|\\\\|\\")"#).unwrap();
        }
        let string = RE
            .replace_all(&token, |cap: &Captures| match &cap[0] {
                "\\\"" => "\"",
                "\\n" => "\n",
                "\\\\" => "\\",
                _ => panic!("Impossible capture {}", &cap[0]),
            })
            .to_string();
        Type::String(string)
    }
}

/// Reads a string of text and return a correct Abstract Syntax Tree
/// of the tokenized input.
pub fn read_str(input: &str) -> Option<Type> {
    if input.starts_with(";") || input.len() == 0 {
        return None;
    }

    let tokens = tokenize(input);
    let mut reader = Reader::new(tokens);
    Some(reader.read_form())
}

/// Tokenize the input stream and returns a list of tokens
pub fn tokenize(input: &str) -> Vec<Token> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            "[\\s ,]*(~@|[\\[\\]{}()'`~^@]|\"(?:\\\\.|[^\\\\\"])*\"?|;.*|[^\\s\\[\\]{}('\"`,;)]*)",
        )
            .unwrap();
    }

    let tokens = RE
        .captures_iter(input)
        .map(|capture| capture[1].to_owned())
        .collect();

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        assert_eq!(tokenize("123"), vec![String::from("123")]);

        assert_eq!(tokenize("abc"), vec![String::from("abc")]);

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
            tokenize("[123 456 789 ]"),
            vec![
                String::from("["),
                String::from("123"),
                String::from("456"),
                String::from("789"),
                String::from("]"),
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

        assert_eq!(
            tokenize("(abc 123 \"xyz\")"),
            vec![
                String::from("("),
                String::from("abc"),
                String::from("123"),
                String::from("\"xyz\""),
                String::from(")"),
            ]
        );
    }

    #[test]
    fn test_read_str() {
        use crate::types::Type;

        assert_eq!(read_str("123"), Some(Type::Int(123)));

        assert_eq!(read_str("abc"), Some(Type::Symbol(String::from("abc"))));

        assert_eq!(read_str("\"hello\""), Some(Type::String(String::from("hello"))));

        assert_eq!(
            read_str("(123 456)"),
            Some(Type::List(vec![Box::new(Type::Int(123)), Box::new(Type::Int(456)),]))
        );

        assert_eq!(
            read_str("[123 456]"),
            Some(Type::Vector(vec![Box::new(Type::Int(123)), Box::new(Type::Int(456)),]))
        );

        assert_eq!(
            read_str("( + 2 (* 3 4) )"),
            Some(Type::List(vec![
                Box::new(Type::Symbol(String::from("+"))),
                Box::new(Type::Int(2)),
                Box::new(Type::List(vec![
                    Box::new(Type::Symbol(String::from("*"))),
                    Box::new(Type::Int(3)),
                    Box::new(Type::Int(4)),
                ])),
            ]))
        );

        assert_eq!(
            read_str(";; comments"),
            None
        );
    }
}
