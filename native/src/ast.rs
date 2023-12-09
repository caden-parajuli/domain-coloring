use crate::lexer::{Function, Token};
use num::Complex;
use std::{collections::VecDeque, fmt};

#[derive(Clone)]
pub enum Node {
    Const {
        val: Complex<f64>,
    },
    Var,
    Binary {
        op: Token,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    },
    Unary {
        op: Token,
        child: Option<Box<Node>>,
    },
    Fun {
        fun: Function,
        arg: Option<Box<Node>>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Const { val } => write!(f, "{}", val),
            Node::Var => write!(f, "z"),
            Node::Binary {
                op,
                left: _,
                right: _,
            } => write!(f, "{}", op),
            Node::Unary { op, child: _ } => write!(f, "{}", op),
            Node::Fun { fun, arg: _ } => write!(f, "{}", fun),
        }
    }
}

impl Node {
    pub fn to_closure<'a>(self) -> Box<dyn Fn(Complex<f64>) -> Complex<f64> + 'a> {
        match self {
            Node::Const { val } => Box::new(move |_z| val),
            Node::Var => Box::new(|z| z),
            Node::Binary {
                op: Token::Add,
                left,
                right,
            } => {
                let left_fun = left.unwrap().to_closure();
                let right_fun = right.unwrap().to_closure();
                Box::new(move |z| left_fun(z) + right_fun(z))
            }
            Node::Binary {
                op: Token::Sub,
                left,
                right,
            } => {
                let left_fun = left.unwrap().to_closure();
                let right_fun = right.unwrap().to_closure();
                Box::new(move |z| left_fun(z) - right_fun(z))
            }
            Node::Binary {
                op: Token::Mult,
                left,
                right,
            } => {
                let left_fun = left.unwrap().to_closure();
                let right_fun = right.unwrap().to_closure();
                Box::new(move |z| left_fun(z) * right_fun(z))
            }
            Node::Binary {
                op: Token::Div,
                left,
                right,
            } => {
                let left_fun = left.unwrap().to_closure();
                let right_fun = right.unwrap().to_closure();
                Box::new(move |z| left_fun(z) / right_fun(z))
            }
            Node::Binary {
                op: Token::Pow,
                left,
                right,
            } => {
                let left_fun = left.unwrap().to_closure();
                let right_fun = right.unwrap().to_closure();
                Box::new(move |z| left_fun(z).powc(right_fun(z)))
            }
            Node::Binary {
                op: _,
                left: _,
                right: _,
            } => panic!("Error in closure construction (invalid binary operator), please report this to program maintainer"),
            Node::Unary {
                op: Token::Sub,
                child,
            } => {
                let child_fun = child.unwrap().to_closure();
                Box::new(move |z| -child_fun(z))
            }
            Node::Unary { op: _, child: _ } => panic!("Error in closure construction (invalid unary operator), please report this to program maintainer"),
            Node::Fun { fun: _, arg: _ } => Box::new(move |z| z),
        }
    }

    pub fn to_mermaid(&self) -> String {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut counter: usize = 0;
        let mut queue = VecDeque::from([(self, counter)]);
        let mut curr;
        while !queue.is_empty() {
            curr = queue.pop_front().unwrap();
            match curr.0 {
                Node::Const { val: _ } => (),
                Node::Var => (),
                Node::Binary { op: _, left, right } => {
                    queue.push_back((left.as_ref().unwrap(), counter));
                    queue.push_back((right.as_ref().unwrap(), counter));
                }
                Node::Unary { op: _, child } => queue.push_back((child.as_ref().unwrap(), counter)),
                Node::Fun { fun: _, arg } => queue.push_back((arg.as_ref().unwrap(), counter)),
            }
            nodes.push(format!("{counter}[{}]", curr.0));
            // We don't want an incoming edge for the root node
            if counter != 0 {
                edges.push(format!("{} --> {}", curr.1, counter));
            }
            counter += 1;
        }
        [
            "flowchart TD".to_string(),
            nodes.join("\n    "),
            edges.join("\n    "),
        ]
        .join("\n    ")
    }
}
