use crate::lexer::token::Token;
use std::iter::Peekable;

pub trait Parseable: std::fmt::Debug {
    fn from_tokens(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Self>
    where
        Self: Sized;
}

