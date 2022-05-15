use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{types::Type, utils::unescape_string};

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

    fn peek(&self) -> Result<&Token, String> {
        if self.position < self.tokens.len() {
            Ok(&self.tokens[self.position])
        } else {
            Err("Syntax error: unexpected EOF while parsing".to_string())
        }
    }

    fn read_form(&mut self) -> Result<Type, String> {
        let token = self.peek()?;

        match token.chars().next().unwrap() {
            '(' => Ok(self.read_list()?),
            '[' => Ok(self.read_vector()?),
            '{' => Ok(self.read_hash_map()?),
            '"' => Ok(self.read_string()?),
            ':' => Ok(self.read_keyword()?),
            ')' => Err("Syntax error: unexpected ')'".to_string()),
            ']' => Err("Syntax error: unexpected ']'".to_string()),
            '}' => Err("Syntax error: unexpected '}}'".to_string()),
            _ => Ok(self.read_atom()?),
        }
    }

    fn read_list(&mut self) -> Result<Type, String> {
        Ok(Type::List(self.read_seq(")")?))
    }

    fn read_vector(&mut self) -> Result<Type, String> {
        Ok(Type::Vector(self.read_seq("]")?))
    }

    fn read_seq(&mut self, end: &str) -> Result<Vec<Type>, String> {
        let mut items = Vec::new();

        self.next(); // skip "(", "["

        loop {
            let item = self.peek()?;

            if item == end {
                break;
            } else {
                items.push(self.read_form()?);
            }

            if self.next().is_none() {
                break;
            }
        }

        Ok(items)
    }

    fn read_hash_map(&mut self) -> Result<Type, String> {
        let mut hash_map = HashMap::new();

        self.next(); // skip "{"

        let mut key = None;
        loop {
            let item = self.peek()?;

            if item == "}" {
                break;
            } else {
                match key {
                    Some(k) => {
                        hash_map.insert(k, Box::new(self.read_form()?));
                        key = None
                    }
                    None => key = Some(item.to_owned()),
                }
            }
            if self.next().is_none() {
                break;
            }
        }

        Ok(Type::HashMap(hash_map))
    }

    fn read_keyword(&mut self) -> Result<Type, String> {
        let token = self.peek()?;
        Ok(Type::Keyword(token[1..].to_string()))
    }

    fn read_atom(&mut self) -> Result<Type, String> {
        let token = self.peek()?;

        Ok(match token.as_str() {
            "nil" => Type::Nil,
            "true" => Type::Bool(true),
            "false" => Type::Bool(false),
            "@" => {
                self.next();
                Type::List(vec![Type::Symbol(String::from("deref")), self.read_form()?])
            }

            other => {
                if let Ok(number) = token.parse() {
                    if token.contains('.') {
                        Type::Float(number)
                    } else {
                        Type::Int(number as i32)
                    }
                } else {
                    Type::Symbol(other.to_owned())
                }
            }
        })
    }

    fn read_string(&mut self) -> Result<Type, String> {
        let token = self.peek()?;
        let token = token[1..token.len() - 1].to_string();
        let string = unescape_string(&token);
        Ok(Type::String(string))
    }
}

/// Reads a string of text and return a correct Abstract Syntax Tree
/// of the tokenized input.
pub fn read_str(input: &str) -> Result<Option<Type>, String> {
    let tokens = tokenize(input);
    if tokens.is_empty() {
        return Ok(None);
    }
    let mut reader = Reader::new(tokens);
    let ast = reader.read_form()?;
    Ok(Some(ast))
}

/// Tokenize the input stream and returns a list of tokens
pub fn tokenize(input: &str) -> Vec<Token> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"#)
                .unwrap();
    }

    let tokens = RE
        .captures_iter(input)
        .filter_map(|capture| {
            let token = capture[1].to_owned();
            if token.starts_with(';') {
                // comment
                None
            } else {
                Some(token)
            }
        })
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

        assert_eq!(read_str("123"), Ok(Some(Type::Int(123))));

        assert_eq!(read_str("abc"), Ok(Some(Type::Symbol(String::from("abc")))));

        assert_eq!(
            read_str("\"hello\""),
            Ok(Some(Type::String(String::from("hello"))))
        );

        assert_eq!(
            read_str("(123 456)"),
            Ok(Some(Type::List(vec![Type::Int(123), Type::Int(456),])))
        );

        assert_eq!(
            read_str("[123 456]"),
            Ok(Some(Type::Vector(vec![Type::Int(123), Type::Int(456),])))
        );

        assert_eq!(
            read_str("( + 2 (* 3 4) )"),
            Ok(Some(Type::List(vec![
                Type::Symbol(String::from("+")),
                Type::Int(2),
                Type::List(vec![
                    Type::Symbol(String::from("*")),
                    Type::Int(3),
                    Type::Int(4),
                ]),
            ])))
        );

        assert_eq!(read_str(";; comments"), Ok(None));
    }
}
