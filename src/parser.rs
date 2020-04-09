use crate::lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub enum Expr {
    BinOp(Token, Box<Expr>, Box<Expr>),
    Number(i64)
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    build_addition(&mut tokens.iter().peekable())
}

fn build_addition(tokens: &mut Peekable<Iter<Token>>) -> Expr {
    let mut left = build_multiplication(tokens);
    loop {
        match tokens.peek() {
            Some(&token) if token == &Token::Plus || token == &Token::Minus => {
                tokens.next();
                let right = build_multiplication(tokens);
                left = Expr::BinOp(*token, Box::new(left), Box::new(right));
            },
            Some(&token) if token == &Token::CloseParen => {
                tokens.next();
            },
            _ => break,
        }
    }
    left
}

fn build_multiplication(tokens: &mut Peekable<Iter<Token>>) -> Expr {
    let mut left = build_leaf_term(tokens);
    loop {
        match tokens.peek() {
            Some(&token) if token == &Token::Division || token == &Token::Multiply => {
                tokens.next();
                let right = build_leaf_term(tokens);
                left = Expr::BinOp(*token, Box::new(left), Box::new(right));
            },
            _ => break,
        }
    }
    left
}

fn build_leaf_term(tokens: &mut Peekable<Iter<Token>>) -> Expr {
    match tokens.next() {
        Some(Token::OpenParen) => build_addition(tokens),
        Some(Token::Number(num)) => Expr::Number(*num),
        _ => panic!("Term was expected"),
    }
}