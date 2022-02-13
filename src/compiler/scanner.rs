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

        while let Some(&c) = self.source.peek() {
            if c.is_whitespace() {
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

    fn match_single_or_double_character_token(&mut self, c1: char) -> Option<Token<'a>> {
        use Token::*;

        let c2 = self.source.peek().copied();
        if let Some(token) = match (c1, c2) {
            ('!', Some('=')) => Some(BangEqual),
            ('=', Some('=')) => Some(EqualEqual),
            ('<', Some('=')) => Some(LessEqual),
            ('>', Some('=')) => Some(GreaterEqual),
            _ => None,
        } {
            self.source.next();
            Some(token)
        } else if ('/', Some('/')) == (c1, c2) {
            self.source.next();
            // comment goes till end of line
            while self.source.next().map_or(false, |c| c != '\n') {}
            // todo: return comment string
            Some(Comment(""))
        } else {
            match c1 {
                '!' => Some(Bang),
                '=' => Some(Equal),
                '<' => Some(Less),
                '>' => Some(Greater),
                '/' => Some(Slash),
                _ => None,
            }
        }
    }
}

impl<'a: 'b, 'b> Iterator for &'b mut Scanner<'a> {
    type Item = (Location, Result<Token<'a>, anyhow::Error>);

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let loc = self.current_loc();

        if let Some(c1) = self.source.next() {
            let parsed = if let Some(token) = match_single_character_token(c1) {
                Ok(token)
            } else if let Some(token) = self.match_single_or_double_character_token(c1) {
                Ok(token)
            } else {
                Err(ScannerError::UnexpectedCharacter(c1).into())
            };
            Some((loc, parsed))
        } else {
            None
        }
    }
}

fn match_single_character_token(c1: char) -> Option<Token<'static>> {
    use Token::*;

    match c1 {
        // single character tokens
        '(' => Some(LeftParen),
        ')' => Some(RightParen),
        '{' => Some(LeftBrace),
        '}' => Some(RightBrace),
        ',' => Some(Comma),
        '.' => Some(Dot),
        '-' => Some(Minus),
        '+' => Some(Plus),
        ';' => Some(Semicolon),
        '*' => Some(Star),
        _ => None,
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
    Slash,
    Comment(&'a str),
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

    use test_case::test_case;

    fn scan(input: &str) -> Vec<Token> {
        Scanner::new(&input)
            .map(|(_, parsed)| parsed.unwrap())
            .collect()
    }

    #[test_case("!=", Token::BangEqual)]
    #[test_case("==", Token::EqualEqual)]
    #[test_case(">=", Token::GreaterEqual)]
    #[test_case("<=", Token::LessEqual)]
    #[test_case("// comment", Token::Comment("comment"))]
    fn one_or_two_char_token_as_two_char_token(input: &str, t: Token) {
        let tokens = scan(input);
        assert_eq!(tokens, vec![t,])
    }

    #[test_case("! =", Token::Bang, Token::Equal)]
    #[test_case("= =", Token::Equal, Token::Equal)]
    #[test_case("> =", Token::Greater, Token::Equal)]
    #[test_case("< =", Token::Less, Token::Equal)]
    #[test_case("/ /", Token::Slash, Token::Slash)]
    fn one_or_two_char_token_as_one_char_tokens(input: &str, t1: Token, t2: Token) {
        let tokens = scan(input);
        assert_eq!(tokens, vec![t1, t2]);
    }

    #[test]
    fn comment_parses_line() {
        let input = "//\n+";
        let tokens = scan(input);
        assert_eq!(tokens, vec![Token::Comment(""), Token::Plus]);

        let input = "//\n+";
        let tokens = scan(input);
        assert_eq!(tokens, vec![Token::Comment(""), Token::Plus]);

        let input = "//";
        let tokens = scan(input);
        assert_eq!(tokens, vec![Token::Comment("")]);
    }

    #[test]
    fn whitespace() {
        let input = "\r(\n\t)\n\n{ }";
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
