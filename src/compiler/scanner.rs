use std::{fmt::Display, str::Chars};
use thiserror::Error;

pub struct Scanner<'a> {
    // source: &'a str,
    iter: Chars<'a>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            // source,
            iter: source.chars(),
            line: 1,
        }
    }

    fn match_single_character_token(&mut self, c: char) -> Option<Result<Token<'a>, ScannerError>> {
        use Token::*;

        match c {
            // single character tokens
            '(' => Some(Ok(LeftParen)),
            ')' => Some(Ok(RightParen)),
            '{' => Some(Ok(LeftBrace)),
            '}' => Some(Ok(RightBrace)),
            ',' => Some(Ok(Comma)),
            '.' => Some(Ok(Dot)),
            '-' => Some(Ok(Minus)),
            '+' => Some(Ok(Plus)),
            ';' => Some(Ok(Semicolon)),
            '*' => Some(Ok(Star)),
            '"' => Some(self.complete_quote()),
            _ => None,
        }
    }

    fn complete_quote(&mut self) -> Result<Token<'a>, ScannerError> {
        let raw = self.iter.as_str();
        let mut length = 0;
        loop {
            let c = self.iter.next();
            if c == Some('"') {
                return Ok(Token::String(raw.get(0..length).unwrap()));
            } else if let Some(c) = c {
                length += 1;
                if c == '\n' {
                    self.line += 1;
                }
            } else {
                // c == None
                return Err(ScannerError::UnterminatedString);
            }
        }
    }

    fn match_single_or_double_character_token(&mut self, c1: char) -> Option<Token<'a>> {
        use Token::*;

        let c2 = self.iter.clone().next();
        if let Some(token) = match (c1, c2) {
            ('!', Some('=')) => Some(BangEqual),
            ('=', Some('=')) => Some(EqualEqual),
            ('<', Some('=')) => Some(LessEqual),
            ('>', Some('=')) => Some(GreaterEqual),
            _ => None,
        } {
            self.iter.next();
            Some(token)
        } else if (c1, c2) == ('/', Some('/')) {
            self.iter.next();

            // comment goes till end of line
            let raw = self.iter.as_str();
            let mut length = 0;
            while let Some(c) = self.iter.next() {
                if c == '\n' {
                    self.line += 1;
                    break;
                } else {
                    length += 1;
                }
            }
            let comment = raw.get(0..length).unwrap();

            Some(Comment(comment))
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
    type Item = (Location, Result<Token<'a>, ScannerError>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c1) = self.iter.next() {
            if c1.is_whitespace() {
                if c1 == '\n' {
                    self.line += 1;
                }
            } else {
                let loc = Location { line: self.line }; // pull location now since matching can advance it
                let parsed = if let Some(parsed) = self.match_single_character_token(c1) {
                    parsed
                } else if let Some(token) = self.match_single_or_double_character_token(c1) {
                    Ok(token)
                } else {
                    Err(ScannerError::UnexpectedCharacter(c1))
                };
                return Some((loc, parsed));
            }
        }

        None // EOF
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

    #[test_case("(", Token::LeftParen)]
    #[test_case(")", Token::RightParen)]
    #[test_case(",", Token::Comma)]
    #[test_case(".", Token::Dot)]
    #[test_case("-", Token::Minus)]
    #[test_case("+", Token::Plus)]
    #[test_case(";", Token::Semicolon)]
    #[test_case("*", Token::Star)]
    #[test_case("\"test\"", Token::String("test"))]
    #[test_case("\"\"", Token::String(""); "empty string")]
    #[test_case("\"te\nst\"", Token::String("te\nst"); "newline string")]
    fn single_character_token(input: &str, t: Token) {
        let tokens = scan(input);
        assert_eq!(tokens, vec![t,])
    }

    #[test]
    fn unterminated_string() {
        let tokens: Vec<_> = Scanner::new("\"test").map(|(_, parsed)| parsed).collect();
        assert_eq!(tokens, vec![Err(ScannerError::UnterminatedString)])
    }

    #[test_case("!=", Token::BangEqual)]
    #[test_case("==", Token::EqualEqual)]
    #[test_case(">=", Token::GreaterEqual)]
    #[test_case("<=", Token::LessEqual)]
    #[test_case("// comment", Token::Comment(" comment"))]
    #[test_case("// comment\n", Token::Comment(" comment"); "newline")]
    #[test_case("//", Token::Comment(""); "empty")]
    #[test_case("//\n", Token::Comment(""); "empty newline")]
    #[test_case("// ", Token::Comment(" "); "single whitespace")]
    #[test_case("// \n", Token::Comment(" "); "single whitespace newline")]
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
