use phf::phf_map;
use std::str::FromStr;
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
}

fn complete_quote<'a>(iter: &mut Chars<'a>, line: &mut usize) -> Result<Token<'a>, ScannerError> {
    let raw = iter.as_str();
    let mut length: usize = 0;
    loop {
        let c = iter.next();
        if c == Some('"') {
            let token = Token::String(raw.get(0..length).unwrap());
            return Ok(token);
        } else if let Some(c) = c {
            length += 1;
            if c == '\n' {
                *line += 1;
            }
        } else {
            // c == None
            return Err(ScannerError::UnterminatedString);
        }
    }
}

impl<'a: 'b, 'b> Iterator for &'b mut Scanner<'a> {
    type Item = (Location, Result<Token<'a>, ScannerError>);

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        let mut iter = self.iter.clone();
        while let Some(c1) = iter.next() {
            if c1.is_whitespace() {
                self.iter = iter.clone();
                if c1 == '\n' {
                    self.line += 1;
                }
            } else {
                let loc = Location { line: self.line }; // pull location now since matching can advance it

                if let Some(parsed) = match c1 {
                    // single character tokens
                    '(' => Some(Ok(LeftParen)),
                    ')' => Some(Ok(RightParen)),
                    '{' => Some(Ok(LeftBrace)),
                    '}' => Some(Ok(RightBrace)),
                    ',' => Some(Ok(Comma)),
                    '-' => Some(Ok(Minus)),
                    '+' => Some(Ok(Plus)),
                    ';' => Some(Ok(Semicolon)),
                    '*' => Some(Ok(Star)),
                    '"' => Some(complete_quote(&mut iter, &mut self.line)),
                    '0'..='9' => {
                        let mut last = iter.clone();
                        let mut length = 1;
                        let mut period = false;
                        while let Some(c) = iter.next() {
                            if c.is_ascii_digit() {
                                length += 1;
                                last = iter.clone();
                            } else if !period
                                && c == '.'
                                && iter.next().map_or(false, |c| '0' <= c && c <= '9')
                            {
                                period = true;
                                length += 2;
                                last = iter.clone();
                            } else {
                                break;
                            }
                        }

                        let number = self.iter.as_str().get(0..length).unwrap();
                        let number = f64::from_str(number)
                            .expect("invalid numbers shouldn't get past the string splitting");
                        iter = last;
                        Some(Ok(Number(number)))
                    }
                    c if c.is_alphabetic() || c == '_' => {
                        let mut last = iter.clone();
                        let mut length = 1;
                        while let Some(c) = iter.next() {
                            if c.is_alphanumeric() || c == '_' {
                                length += 1;
                                last = iter.clone();
                            } else {
                                break;
                            }
                        }

                        let identifier = self.iter.as_str().get(0..length).unwrap();
                        iter = last;

                        if let Some(&keyword) = KEYWORDS.get(identifier) {
                            Some(Ok(keyword))
                        } else {
                            Some(Ok(Identifier(identifier)))
                        }
                    }
                    _ => None,
                } {
                    self.iter = iter.clone();
                    return Some((loc, parsed));
                }

                let iter_1 = iter.clone();
                let c2 = iter.next();
                if let Some(token) = match (c1, c2) {
                    ('!', Some('=')) => Some(BangEqual),
                    ('=', Some('=')) => Some(EqualEqual),
                    ('<', Some('=')) => Some(LessEqual),
                    ('>', Some('=')) => Some(GreaterEqual),
                    ('.', Some('0'..='9')) => {
                        let mut last = iter.clone();
                        let mut length = 2;
                        while let Some(c) = iter.next() {
                            if c.is_ascii_digit() {
                                length += 1;
                                last = iter.clone();
                            } else {
                                break;
                            }
                        }

                        let number = self.iter.as_str().get(0..length).unwrap();
                        let number = f64::from_str(number)
                            .expect("invalid numbers shouldn't get past the string splitting");
                        iter = last;
                        Some(Number(number))
                    }
                    ('/', Some('/')) => {
                        // comment goes till end of line
                        let raw = iter.as_str();
                        let mut length = 0;
                        while let Some(c) = iter.next() {
                            if c == '\n' {
                                self.line += 1;
                                break;
                            } else {
                                length += 1;
                            }
                        }
                        let comment = raw.get(0..length).unwrap();
                        Some(Comment(comment))
                    }
                    _ => None,
                } {
                    self.iter = iter;
                    return Some((loc, Ok(token)));
                } else if let Some(token) = match c1 {
                    '.' => Some(Dot),
                    '!' => Some(Bang),
                    '=' => Some(Equal),
                    '<' => Some(Less),
                    '>' => Some(Greater),
                    '/' => Some(Slash),
                    _ => None,
                } {
                    self.iter = iter_1;
                    return Some((loc, Ok(token)));
                } else {
                    return Some((loc, Err(ScannerError::UnexpectedCharacter(c1))));
                };
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
    Number(f64),
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

static KEYWORDS: phf::Map<&'static str, Token<'static>> = phf_map! {
    "and" => Token::And,
    "class" => Token::Class,
    "else" => Token::Else,
    "false" => Token::False,
    "for" => Token::For,
    "fun" => Token::Fun,
    "if" => Token::If,
    "nil" => Token::Nil,
    "or" => Token::Or,
    "print" => Token::Print,
    "return" => Token::Return,
    "super" => Token::Super,
    "this" => Token::This,
    "true" => Token::True,
    "var" => Token::Var,
    "while" => Token::While,
};

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

    #[test_case("first+", Token::Identifier("first"), Some(Token::Plus); "first_plus_no_whitespace")]
    #[test_case("first +", Token::Identifier("first"), Some(Token::Plus); "first_plus_whitespace")]
    #[test_case("first", Token::Identifier("first"), None; "first_EOF")]
    #[test_case("first ", Token::Identifier("first"), None; "first_whitespace")]
    #[test_case("_first ", Token::Identifier("_first"), None; "underscore")]
    fn identifier(input: &str, t1: Token, t2: Option<Token>) {
        let tokens = scan(input);
        let expected = if let Some(t2) = t2 {
            vec![t1, t2]
        } else {
            vec![t1]
        };
        assert_eq!(tokens, expected)
    }

    #[test_case("and", Token::And)]
    #[test_case("and ", Token::And; "and space")]
    #[test_case("ands", Token::Identifier("ands"))]
    #[test_case("class", Token::Class)]
    #[test_case("else", Token::Else)]
    #[test_case("false", Token::False)]
    #[test_case("for", Token::For)]
    #[test_case("fun", Token::Fun)]
    #[test_case("if", Token::If)]
    #[test_case("nil", Token::Nil)]
    #[test_case("or", Token::Or)]
    #[test_case("print", Token::Print)]
    #[test_case("return", Token::Return)]
    #[test_case("super", Token::Super)]
    #[test_case("this", Token::This)]
    #[test_case("true", Token::True)]
    #[test_case("var", Token::Var)]
    #[test_case("while", Token::While)]
    fn keyword(input: &str, t: Token) {
        let tokens = scan(input);
        assert_eq!(tokens, vec![t])
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

    #[test_case("1.2", Token::Number(1.2f64), None)]
    #[test_case("1.2.", Token::Number(1.2f64), Some(Token::Dot))]
    #[test_case(".2", Token::Number(0.2f64), None)]
    #[test_case("2", Token::Number(2f64), None)]
    #[test_case("2.", Token::Number(2f64), Some(Token::Dot))]
    fn number(input: &str, t1: Token, t2: Option<Token>) {
        let tokens = scan(input);
        let expected = if let Some(t2) = t2 {
            vec![t1, t2]
        } else {
            vec![t1]
        };
        assert_eq!(tokens, expected)
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

        let input = "//+";
        let tokens = scan(input);
        assert_eq!(tokens, vec![Token::Comment("+")]);

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
