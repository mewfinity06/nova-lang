use crate::lexer::{token::Token, Lexer};

pub struct Parser {
    _tokens: Vec<Token>,             // Store tokens directly
    iter: std::vec::IntoIter<Token>, // Store an iterator over the tokens
}

impl Parser {
    pub fn new(file_name: String, contents: String) -> Self {
        let tokens = Lexer::new(file_name, contents).lex(); // Get owned tokens
        let iter = tokens.clone().into_iter(); // Create an iterator

        Self {
            _tokens: tokens,
            iter,
        }
    }

    pub fn parse_lr1(mut self) {
        while let Some(token) = self.next_token() {
            println!("This token   : {:?}", token);
            println!("Peeked token : {:?}\n", self.peek().unwrap_or(&Token::none()));
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.iter.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.iter.as_slice().first()
    }
}
