use std::fmt::Debug;

use crate::{lexer::token::Token, lexer::token::TokenKind, parser::common};

#[derive(Clone)]
pub enum Expr {
    Binary(Token, Box<Expr>, Box<Expr>), // Operator, Left Operand, Right Operand
    Unary(Token, Box<Expr>),             // Operator, Operand
    Literal(Token),                      // Literal value (e.g., numbers, strings)
    Group(Box<Expr>),                    // Grouped expression (e.g., (a + b))
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(op, lhs, rhs) => {
                write!(f, "(Binary {} lhs {:?} rhs {:?})", op.word, lhs, rhs)
            },
            Expr::Unary(op, expr) => {
                write!(f, "(Unary {} {:?})", op.word, expr)
            },
            Expr::Literal(token) => {
                write!(f, "(Literal {})", token.word)
            },
            Expr::Group(expr) => {
                write!(f, "(Group {:?})", expr)
            },
        }
    }
}

use std::iter::Peekable;

impl common::Parseable for Expr {
    fn from_tokens(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Self> {
        parse_expression(tokens)
    }
}

fn parse_expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Expr> {
    let mut lhs = parse_unary(tokens)?; // Start with unary or primary expression

    while let Some(op) = tokens.peek() {
        if !matches!(
            op.kind,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Mult | TokenKind::Div
        ) {
            break; // Stop parsing if not an operator
        }

        let op = tokens.next()?; // Now we safely consume the operator
        let rhs = parse_unary(tokens)?; // Ensure we get a valid right-hand expression
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
    }

    Some(lhs)
}

fn parse_unary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Expr> {
    if let Some(op) = tokens.peek() {
        if matches!(op.kind, TokenKind::Plus | TokenKind::Minus) {
            let op = tokens.next()?; // Consume the unary operator
            let operand = parse_unary(tokens)?; // Recursively parse operand
            return Some(Expr::Unary(op, Box::new(operand)));
        }
    }
    parse_primary(tokens) // Otherwise, parse a primary expression
}

fn parse_primary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Expr> {
    let token = tokens.next()?; // Consume the token

    let mut expr = match token.kind {
        TokenKind::Ident | TokenKind::Number => Some(Expr::Literal(token)),
        TokenKind::OpenParen => {
            let inner_expr = parse_expression(tokens)?;
            if let Some(close) = tokens.next() {
                if close.kind == TokenKind::CloseParen {
                    return Some(Expr::Group(Box::new(inner_expr)));
                }
            }
            None // Mismatched parentheses
        }
        _ => None,
    }?;

    // Check for postfix unary operators (`x!`, `y++`, `z--`)
    while let Some(postfix_op) = tokens.peek() {
        if matches!(postfix_op.kind, TokenKind::Bang | TokenKind::PlusPlus | TokenKind::MinusMinus) {
            let postfix_op = tokens.next()?; // Consume the postfix operator
            expr = Expr::Unary(postfix_op, Box::new(expr));
        } else {
            break; // Stop if it's not a postfix operator
        }
    }

    Some(expr)
}



