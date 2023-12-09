use native::ast::Node;
use native::lexer::Token;
use native::parser;
use num::Complex;

const TREE_STR: &str = "flowchart TD
    0[*]
    1[z]
    2[+]
    3[3+0i]
    4[0+1i]
    0 --> 1
    0 --> 2
    2 --> 3
    2 --> 4";

#[test]
fn test_mermaid() {
    println!("Testing Mermaid output...");
    let tree = Node::Binary {
        op: Token::Mult,
        left: Some(Box::new(Node::Var)),
        right: Some(Box::new(Node::Binary {
            op: Token::Add,
            left: Some(Box::new(Node::Const {
                val: Complex::new(3.0, 0.0),
            })),
            right: Some(Box::new(Node::Const {
                val: Complex::new(0.0, 1.0),
            })),
        })),
    };
    println!("{}", tree.to_mermaid());
    assert_eq!(tree.to_mermaid(), TREE_STR);
}

#[test]
fn test_parser() {
    println!("Testing parser...");
    let tree = parser::parse("z (3 + i)").unwrap();
    println!("{}", tree.to_mermaid());
    assert_eq!(tree.to_mermaid(), TREE_STR);
}

#[test]
fn test_closure() {
    println!("Testing parser...");
    let f = parser::parse("z (3 + i)").unwrap().to_closure();
    assert_eq!(f(Complex::new(1.0, 0.0)), Complex::new(3.0, 1.0));
    assert_eq!(f(Complex::new(2.0, 0.0)), Complex::new(6.0, 2.0));
}
