use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn escape_string(s: &str) -> String {
    let escaped = s
        .chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            _ => c.to_string(),
        })
        .reduce(|acc, part| acc + &part);

    escaped.unwrap_or_default()
}

pub fn unescape_string(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(\\n|\\\\|\\")"#).unwrap();
    }
    let unescaped = RE
        .replace_all(s, |cap: &Captures| match &cap[0] {
            "\\\"" => "\"",
            "\\n" => "\n",
            "\\\\" => "\\",
            _ => panic!("Impossible capture {}", &cap[0]),
        })
        .to_string();
    unescaped
}

pub fn balanced_string(s: &str) -> bool {
    if s.len() < 2 || !s.starts_with('"') || !s.ends_with('"') {
        return false;
    }

    let mut backslash = false;
    let mut t = Vec::with_capacity(2);

    for c in s.chars() {
        if backslash {
            backslash = false;
            continue;
        }

        backslash = c == '\\';

        if !backslash {
            t.push(c);
        }
    }

    t.len() >= 2 && t[0] == t[t.len() - 1] && t[0] == '"'
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_escaping() {
        let original = String::from(
            r#"An "string" with specia\ characters
"#,
        );

        let escaped = escape_string(&original);
        assert_eq!(escaped, r#"An \"string\" with specia\\ characters\n"#);

        let unescaped = unescape_string(&original);
        assert_eq!(unescaped, original);
    }
}
