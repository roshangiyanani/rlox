use peekmore::{PeekMore, PeekMoreIterator};
use std::{fmt::Display, str::Chars};
use thiserror::Error;

pub struct Scanner<'a> {
    source: PeekMoreIterator<Chars<'a>>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source: source.chars().peekmore(),
            line: 1,
        }
    }

    fn skip_whitespace(&mut self) {
        let mut new_lines = 0;

        loop {
            if let Some(&c) = self.source.peek() {
                if c == '\n' {
                    new_lines += 1;
                }
                self.source.next();
            } else {
                break;
            }
        }

        if new_lines > 0 {
            self.line += new_lines;
            log::trace!("consumed {} new line(s)", new_lines);
        }
    }

    fn current_loc(&self) -> Location {
        Location { line: self.line }
    }
}

impl<'a: 'b, 'b> Iterator for &'b mut Scanner<'a> {
    type Item = (Location, Result<Token<'a>, anyhow::Error>);

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        self.skip_whitespace();
        let loc = self.current_loc();

        if let Some(c) = self.source.next() {
            let token = match c {
                '(' => Ok(LeftParen),
                ')' => Ok(RightParen),
                '{' => Ok(LeftBrace),
                '}' => Ok(RightBrace),
                ',' => Ok(Comma),
                '.' => Ok(Dot),
                '-' => Ok(Minus),
                '+' => Ok(Plus),
                ';' => Ok(Semicolon),
                '/' => Ok(Semicolon),
                '*' => Ok(Star),
                c => Err(ScannerError::UnexpectedCharacter(c).into()),
            };
            Some((loc, token))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ScannerError {
    #[error("unterminated string")]
    UnterminatedString,
    #[error("unexpected character '{0}'")]
    UnexpectedCharacter(char),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    line: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}", self.line)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // literals
    Identifier(&'a str),
    String(&'a str),
    Number(&'a str),
    // keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn whitespace() {
        let input = "\r(\n\t)\n\n/{ /}";
        let tokens: Vec<_> = Scanner::new(&input)
            .map(|(loc, parsed)| (loc, parsed.unwrap()))
            .collect();

        assert_eq!(
            tokens,
            vec![
                (Location { line: 1 }, Token::LeftParen),
                (Location { line: 2 }, Token::RightParen),
                (Location { line: 4 }, Token::LeftBrace),
                (Location { line: 4 }, Token::RightBrace),
            ]
        );
    }
}
