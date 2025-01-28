mod common;
mod expr;

use common::Parseable;
use expr::Expr;

use crate::lexer::{token::Token, Lexer};
use std::iter::Peekable;

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    pub fn new(file_name: String, contents: String) -> Self {
        let tokens = Lexer::new(file_name, contents).lex().into_iter().peekable();

        Self { tokens }
    }

    pub fn parse_lr1(&mut self) -> Vec<Box<dyn Parseable>> {
        let mut results: Vec<Box<dyn Parseable>> = Vec::new();

        while self.tokens.peek().is_some() {
            // Clone iterator state to avoid consuming tokens prematurely
            let mut temp_iter = self.tokens.clone();
            
            if let Some(parsed) = Expr::from_tokens(&mut temp_iter) {
                let parsed = Box::new(parsed);
                println!("Parsed: {:#?}", &parsed);
                results.push(parsed);
                
                // Advance the real iterator by consuming the used tokens
                for _ in 0..3 {
                    self.tokens.next();
                }
            } else {
                self.tokens.next(); // Consume token to prevent infinite loops
            }
        }

        results
    }
}
