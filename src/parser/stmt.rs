use crate::parser::common;

#[derive(Copy, Clone, Debug)]
pub enum Stmt {}

impl common::Parseable for Stmt {
    fn from_tokens(tokens: &mut dyn Iterator<Item = crate::lexer::token::Token>) -> Option<Self>
        where
            Self: Sized {
        None
    }
}
