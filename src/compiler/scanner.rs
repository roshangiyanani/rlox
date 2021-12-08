use std::fmt::Display;

pub struct Scanner {
    source: String,
    position: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            position: 0,
            line: 1,
        }
    }
}

impl<'a> Iterator for &'a mut Scanner {
    type Item = (Location, Result<Token<'a>, ScannerError>);

    fn next(&mut self) -> Option<Self::Item> {
        let loc = Location { line: self.line };
        if self.position >= self.source.len() {
            None
        } else {
            self.position += 1;
            Some((loc, Err(ScannerError::UnexpectedCharacter)))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScannerError {
    UnterminatedString,
    UnexpectedCharacter,
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
