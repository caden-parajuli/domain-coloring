use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

pub enum LexError {
    DoubleDecimal,
    InvalidCharacter,
}

pub enum Token {
    Float(f64),
    ComplexI,
    ID(String),
    Add,
    Sub,
    Mult,
    Div,
    Pow,
    LParen,
    RParen,
    Error(LexError),
}

pub struct Lexer<'a> {
    buffer_iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(buffer: &'a str) -> Lexer<'a> {
        Lexer {
            buffer_iter: buffer.chars().peekable(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.buffer_iter.next()? {
            // Remove whitespace
            ws if ws.is_whitespace() => self.next(),
            '+' => Some(Token::Add),
            '-' => Some(Token::Sub),
            '*' => Some(Token::Mult),
            '/' => Some(Token::Div),
            '^' => Some(Token::Pow),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            // Parse float
            mut digit if is_digit_char(&digit) => {
                let mut has_dot = false;
                let mut digit_str: String = String::from("");
                loop {
                    if digit == '.' {
                        if has_dot {
                            return Some(Token::Error(LexError::DoubleDecimal));
                        }
                        has_dot = true;
                    }
                    digit_str.push(digit);
                    // Get the next char, break if it isn't a digit char
                    match self.buffer_iter.next_if(is_digit_char) {
                        Some(c) => digit = c,
                        None => break,
                    }
                }
                // Parse the characters into a float
                Some(Token::Float(f64::from_str(&digit_str).unwrap()))
            }
            // Parse Identifier
            mut id_char if is_id_char(&id_char) => {
                let mut id_str: String = String::from("");
                loop {
                    id_str.push(id_char);
                    match self.buffer_iter.next_if(is_id_char) {
                        Some(c) => id_char = c,
                        None => break,
                    }
                }
                Some(Token::ID(id_str))
            }

            // Error: Invalid character
            _ => Some(Token::Error(LexError::InvalidCharacter)),
        }
    }
}

#[inline]
fn is_digit_char(c: &char) -> bool {
    matches!(*c, '0'..='9' | '.')
}

#[inline]
fn is_id_char(c: &char) -> bool {
    matches!(*c, 'a'..='z' | 'A'..='Z' | '_')
}
