use anyhow::{self, Error};
use num::Complex;

use crate::ast::Node;
use crate::lexer::{self, Lexer, Token};

type ComplexFnBox = Box<dyn Fn(Complex<f64>) -> Complex<f64>>;

pub fn parse_to_fn(fun_str: &str) -> Result<ComplexFnBox, Error> {
    Ok(parse(fun_str)?.to_closure())
}

pub fn parse(str_buf: &str) -> Result<Node, Error> {
    let mut lexer = lexer::new_lexer(str_buf);
    expr(&mut lexer)
}

macro_rules! bin_node_from_op {
    ($lexer:expr, $op:ident, $left:expr, $right_fun:ident) => {
        Ok(match $op {
            Some(some_op) => Node::Binary {
                op: some_op,
                left: Some(Box::new($left)),
                right: Some(Box::new($right_fun($lexer)?)),
            },
            None => $left,
        })
    };
}

/// Currently this sets the new node to the left child, and then
/// at the end checks if it's the last one, and if so sets the last right
/// to the right of the previous
fn expr(lexer: &mut Lexer) -> Result<Node, Error> {
    let mut current: Node = term(lexer)?;
    while matches!(lexer.peek(), Some(&Token::Add) | Some(&Token::Sub)) {
        let op = lexer.next().unwrap();
        current = Node::Binary {
            op,
            left: Some(Box::new(current)),
            right: Some(Box::new(term(lexer)?)),
        }
    }
    Ok(current)
}

fn term(lexer: &mut Lexer) -> Result<Node, Error> {
    let mut current: Node = factor(lexer)?;
    loop {
        if matches!(lexer.peek(), Some(&Token::Mult) | Some(&Token::Div)) {
            let op = lexer.next().unwrap();
            current = Node::Binary {
                op,
                left: Some(Box::new(current)),
                right: Some(Box::new(factor(lexer)?)),
            }
        } else if let Ok(next) = factor(lexer) {
            current = Node::Binary {
                op: Token::Mult,
                left: Some(Box::new(current)),
                right: Some(Box::new(next)),
            }
        } else {
            break;
        }
    }
    Ok(current)
}

fn factor(lexer: &mut Lexer) -> Result<Node, Error> {
    let left = base(lexer)?;
    let op = lexer.next_if(|&op| op == Token::Pow);
    bin_node_from_op!(lexer, op, left, factor)
}

fn base(lexer: &mut Lexer) -> Result<Node, Error> {
    let first_tok = match lexer.peek() {
        Some(tok) => *tok,
        None => {
            return Err(anyhow::anyhow!(
            "Expected a constant, \"z\", \"i\", an expression in parentheses, or a function call. "
        ))
        }
    };
    match first_tok {
        // <FLOAT>
        Token::Float(value) => {
            lexer.next();
            Ok(Node::Const {
                val: Complex::new(value, 0.0),
            })
        }
        // <COMPLEXI>
        Token::ComplexI => {
            lexer.next();
            Ok(Node::Const {
                val: Complex::new(0.0, 1.0),
            })
        }
        // <VARZ>
        Token::VarZ => {
            lexer.next();
            Ok(Node::Var)
        }
        // <FUNCTION> <par_expr>
        Token::Fun(fun) => {
            lexer.next();
            Ok(Node::Fun {
                fun,
                arg: Some(Box::new(par_expr(lexer)?)),
            })
        }
        // <par_expr>
        Token::LParen => par_expr(lexer),
        // Error: does not match a production for base
        _ => Err(anyhow::anyhow!(
            "Expected a constant, \"z\", \"i\", an expression in parentheses, or a function call. "
        )),
    }
}

fn par_expr(lexer: &mut Lexer) -> Result<Node, Error> {
    let l_paren = lexer.next_if(|&tok| tok == Token::LParen);
    match l_paren {
        Some(_) => {
            let expr = expr(lexer);
            let r_paren = lexer.next_if(|&tok| tok == Token::RParen);
            match r_paren {
                Some(_) => expr,
                None => Err(anyhow::anyhow!("Expected a closing parenthesis")),
            }
        }
        None => Err(anyhow::anyhow!("Expected an opening parenthesis")),
    }
}
