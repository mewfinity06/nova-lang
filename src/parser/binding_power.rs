use crate::lexer::token::{Token, TokenKind};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
enum BindingPower {
    Lowest = 0,      // For the lowest precedence (e.g., `=` in assignment)
    Sum = 10,        // `+` and `-` have lower precedence than `*` and `/`
    Product = 20,    // `*` and `/` have higher precedence than `+` and `-`
    Prefix = 30,     // Unary operators (e.g., `-a`)
    Call = 40,       // Function calls, array indexing (`()` and `[]`)
    Highest = 50,    // Highest precedence, typically parentheses override
}

impl BindingPower {
    fn infix_binding_power(op: &Token) -> Option<(BindingPower, BindingPower)> {
        match op.kind {
            TokenKind::Plus | TokenKind::Minus => Some((BindingPower::Sum, BindingPower::Sum)),  // `+` and `-`
            TokenKind::Mult | TokenKind::Div => Some((BindingPower::Product, BindingPower::Product)),  // `*` and `/`
            _ => None,
        }
    }
}
