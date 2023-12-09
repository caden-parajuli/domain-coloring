use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LexError {
    DoubleDecimal,
    InvalidCharacter,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Function {
    Sqrt,
    Exp,
    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Csc,
    Sinh,
    Cosh,
    Tanh,
    Coth,
    Sech,
    Csch,
    Re,
    Im,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Token {
    Float(f64),
    ComplexI,
    VarZ,
    Fun(Function),
    Add,
    Sub,
    Mult,
    Div,
    Pow,
    LParen,
    RParen,
    Error(LexError),
}

pub type Lexer<'a> = Peekable<LexerBaseIter<'a>>;

pub fn new_lexer(buffer: &str) -> Lexer {
    LexerBaseIter::new(buffer).peekable()
}

pub struct LexerBaseIter<'a> {
    buffer_iter: Peekable<Chars<'a>>,
}

impl<'a> LexerBaseIter<'a> {
    pub fn new(buffer: &'a str) -> LexerBaseIter<'a> {
        LexerBaseIter {
            buffer_iter: buffer.chars().peekable(),
        }
    }
}

impl Iterator for LexerBaseIter<'_> {
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
            // Parse Function, i, or z
            mut id_char if is_id_char(&id_char) => {
                let mut id_str: String = String::from("");
                loop {
                    id_str.push(id_char);
                    match self.buffer_iter.next_if(is_id_char) {
                        Some(c) => id_char = c,
                        None => break,
                    }
                }
                if id_str.len() == 1 {
                    if id_str.starts_with('i') {
                        return Some(Token::ComplexI);
                    } else if id_str.starts_with('z') {
                        return Some(Token::VarZ);
                    }
                }
                Some(Token::Fun(fun_from_str(&id_str)?))
            }
            // Error: Invalid character
            _ => Some(Token::Error(LexError::InvalidCharacter)),
        }
    }
}

fn fun_from_str(str: &str) -> Option<Function> {
    match str {
        "sqrt" => Some(Function::Sqrt),
        "exp" => Some(Function::Exp),
        "sin" => Some(Function::Sin),
        "cos" => Some(Function::Cos),
        "tan" => Some(Function::Tan),
        "cot" => Some(Function::Cot),
        "sec" => Some(Function::Sec),
        "csc" => Some(Function::Csc),
        "sinh" => Some(Function::Sinh),
        "cosh" => Some(Function::Cosh),
        "tanh" => Some(Function::Tanh),
        "coth" => Some(Function::Coth),
        "sech" => Some(Function::Sech),
        "csch" => Some(Function::Csch),
        "Re" => Some(Function::Re),
        "Im" => Some(Function::Im),
        _ => None,
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

// `Display` implementations

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LexError::InvalidCharacter => "Invalid Character",
                LexError::DoubleDecimal => "Double Decimal",
            }
        )
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Function::Sqrt => "sqrt",
                Function::Exp => "exp",
                Function::Sin => "sin",
                Function::Cos => "cos",
                Function::Tan => "tan",
                Function::Cot => "cot",
                Function::Sec => "sec",
                Function::Csc => "csc",
                Function::Sinh => "sinh",
                Function::Cosh => "cosh",
                Function::Tanh => "tanh",
                Function::Coth => "coth",
                Function::Sech => "sech",
                Function::Csch => "csch",
                Function::Re => "Re",
                Function::Im => "Im",
            }
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Add => "+".to_string(),
                Token::Float(r) => r.to_string(),
                Token::ComplexI => "i".to_string(),
                Token::VarZ => "z".to_string(),
                Token::Fun(fun) => fun.to_string(),
                Token::Sub => "-".to_string(),
                Token::Mult => "*".to_string(),
                Token::Div => "/".to_string(),
                Token::Pow => "^".to_string(),
                Token::LParen => "(".to_string(),
                Token::RParen => ")".to_string(),
                Token::Error(err) => err.to_string(),
            }
        )
    }
}
