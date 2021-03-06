//! Lexical analysis

use std::iter::Peekable;
use std::str::CharIndices;

/// A lexical token.
#[derive(PartialEq, Clone, Debug)]
pub enum Token<'a> {
    // Delimiters
    LeftParen,
    RightParen,
    Comma,
    Semicolon,
    // Keywords
    Do,
    End,
    If,
    Then,
    Else,
    // Special Operators
    Define,
    Arrow,
    // Others
    Name(&'a str),
    Number(&'a str),
    Operator(&'a str),
}

/// Converts a program source into a sequence of lexical tokens.
pub fn scan<'a>(source: &'a str) -> Vec<Token<'a>> {
    let mut scanner = Scanner {
        source: source,
        tokens: Vec::new(),
        remaining: source.char_indices().peekable(),
    };
    let mut f = scan_between(&mut scanner);
    loop {
        f = match f {
            Step::Between => scan_between(&mut scanner),
            Step::Whitespace => scan_whitespace(&mut scanner),
            Step::Name => scan_name(&mut scanner),
            Step::Number => scan_number(&mut scanner),
            Step::Symbol => scan_symbol(&mut scanner),
            Step::EndOfFile => break,
        }
    }
    scanner.tokens
}

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    remaining: Peekable<CharIndices<'a>>,
}

enum Step {
    Between,
    Whitespace,
    Name,
    Number,
    Symbol,
    EndOfFile,
}

fn scan_between(scanner: &mut Scanner) -> Step {
    match scanner.remaining.peek() {
        None => Step::EndOfFile,
        Some((_, c)) => {
            if c.is_whitespace() {
                return Step::Whitespace;
            } else if c.is_lowercase() {
                return Step::Name;
            } else if c.is_ascii_digit() {
                return Step::Number;
            } else if c.is_ascii_punctuation() {
                return Step::Symbol;
            } else {
                println!("invalid character: {}", c); //TODO
                scanner.remaining.next();
                Step::Between
            }
        }
    }
}

fn scan_whitespace(scanner: &mut Scanner) -> Step {
    while let Some((_, c)) = scanner.remaining.peek() {
        if c.is_whitespace() {
            scanner.remaining.next();
        } else {
            break;
        }
    }
    Step::Between
}

fn scan_name(scanner: &mut Scanner) -> Step {
    if let Some(&(start, _)) = scanner.remaining.peek() {
        let mut end = start;
        while let Some(&(i, c)) = scanner.remaining.peek() {
            if c.is_alphanumeric() || c == '_' {
                scanner.remaining.next();
            } else {
                end = i;
                break;
            }
        }
        let s = &scanner.source[start..end];
        if s == "do" {
            scanner.tokens.push(Token::Do);
        } else if s == "end" {
            scanner.tokens.push(Token::End);
        } else if s == "if" {
            scanner.tokens.push(Token::If);
        } else if s == "then" {
            scanner.tokens.push(Token::Then);
        } else if s == "else" {
            scanner.tokens.push(Token::Else);
        } else {
            scanner.tokens.push(Token::Name(s));
        }
    }
    Step::Between
}

fn scan_number(scanner: &mut Scanner) -> Step {
    if let Some(&(start, _)) = scanner.remaining.peek() {
        let mut end = start;
        while let Some(&(i, c)) = scanner.remaining.peek() {
            if c.is_ascii_digit() || c == '_' {
                scanner.remaining.next();
            } else {
                end = i;
                break;
            }
        }
        let s = &scanner.source[start..end];
        scanner.tokens.push(Token::Number(s));
    }
    Step::Between
}

fn scan_symbol(scanner: &mut Scanner) -> Step {
    if let Some(&(start, c)) = scanner.remaining.peek() {
        let mut end = start;
        if is_delimiter(c) {
            match c {
                '(' => scanner.tokens.push(Token::LeftParen),
                ')' => scanner.tokens.push(Token::RightParen),
                ',' => scanner.tokens.push(Token::Comma),
                ';' => scanner.tokens.push(Token::Semicolon),
                _ => {}
            }
            scanner.remaining.next();
        } else {
            while let Some(&(i, c)) = scanner.remaining.peek() {
                if is_delimiter(c) {
                    break;
                } else if c.is_ascii_punctuation() {
                    scanner.remaining.next();
                } else {
                    end = i;
                    break;
                }
            }
            let s = &scanner.source[start..end];
            if s == "=" {
                scanner.tokens.push(Token::Define);
            } else if s == "->" {
                scanner.tokens.push(Token::Arrow);
            } else {
                scanner.tokens.push(Token::Operator(s));
            }
        }
    }
    Step::Between
}

fn is_delimiter(c: char) -> bool {
    match c {
        '(' | ')' | ',' | ';' => true,
        _ => false,
    }
}

// TESTS

#[cfg(test)]
mod test {
    use super::scan;
    use super::Token::*;

    #[test]
    fn names() {
        assert_eq!(scan(String::from("")), []);
        assert_eq!(scan(String::from("foo")), [Name("foo".to_string())]);
        assert_eq!(
            scan(String::from("floccinaucinihilipilification")),
            [Name("floccinaucinihilipilification".to_string())]
        );
        assert_eq!(
            scan(String::from("another_foo")),
            [Name("another_foo".to_string())]
        );
        assert_eq!(scan(String::from("bar123")), [Name("bar123".to_string())]);
        assert_eq!(
            scan(String::from("b_a__z456f_")),
            [Name("b_a__z456f_".to_string())]
        );
    }

    #[test]
    fn keywords() {
        assert_eq!(
            scan(String::from("do if then else end")),
            [Do, If, Then, Else, End]
        );
    }

    #[test]
    fn simple_factorial() {
        let source = "simple_factorial = n ->
	if is_leq(n, 1) then 1
	else add(n, simple_factorial(sub(n, 1)))
	end;";
        assert_eq!(
            scan(String::from(source)),
            [
                Name("simple_factorial".to_string()),
                Define,
                Name("n".to_string()),
                Arrow,
                If,
                Name("is_leq".to_string()),
                LeftParen,
                Name("n".to_string()),
                Comma,
                Number("1".to_string()),
                RightParen,
                Then,
                Number("1".to_string()),
                Else,
                Name("add".to_string()),
                LeftParen,
                Name("n".to_string()),
                Comma,
                Name("simple_factorial".to_string()),
                LeftParen,
                Name("sub".to_string()),
                LeftParen,
                Name("n".to_string()),
                Comma,
                Number("1".to_string()),
                RightParen,
                RightParen,
                RightParen,
                End,
                Semicolon
            ]
        );
    }
}
