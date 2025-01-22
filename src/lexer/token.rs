use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::{Display as StrumDisplay, EnumIter};

use colour::*;

#[derive(Debug, StrumDisplay, Default, PartialEq, Clone, Copy, EnumIter)]
pub enum TokenKind {
    Ident,
    Keyword,
    Type,
    Directive,

    // Seperation
    OpenParen,  // (
    CloseParen, // )
    OpenCurly,  // {
    CloseCurly, // }
    OpenBrack,  // [
    CloseBrack, // ]

    // Comparison
    DoubleEq, // ==
    NotEq,    // !=
    LessEq,   // <=
    GreatEq,  // >=
    Less,     // <
    Great,    // >

    // Assignment
    Assign,           // =
    InferAssign,      // :=
    PlusAssign,       // +=
    MinusAssign,      // -=
    MultAssign,       // *=
    DivAssign,        // /=
    ModAssign,        // %=
    ShiftLeftAssign,  // <<=
    ShiftRightAssign, // >>=

    PlusPlus,   // ++
    MinusMinus, // --

    // Binary Ops
    Plus,       // +
    Minus,      // -
    Mult,       // *
    Div,        // /
    Mod,        // %
    ShiftLeft,  // <<
    ShiftRight, // >>

    PipeLine,    // |>
    Pipe,        // |
    Colon,       // :
    DoubleColon, // ::
    Semicolon,   // ;
    FatArrow,    // =>
    ThinArrow,   // ->

    At, // @

    Bang,
    Question,
    Dollar,
    Carrot,
    Amper,
    Dot,
    Comma,

    StringLiteral,
    Number,
    Comment,

    Test, // =+=

    #[default]
    Unknown,
    Eof,
}

impl TokenKind {
    fn get_longest_name() -> usize {
        let mut longest = 0;
        for kind in TokenKind::iter() {
            if kind.to_string().len() > longest {
                longest = kind.to_string().len();
            }
        }
        longest
    }
}

#[derive(Debug, Default, PartialEq, Clone)]

pub struct Location {
    pub file_name: String,
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(file_name: String, line: usize, column: usize) -> Self {
        Location {
            file_name,
            line,
            column,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_red!(f, "{}", self.file_name)?;
        write!(f, ":")?;
        write_yellow!(f, "{}", self.line)?;
        write!(f, ":")?;
        write_green!(f, "{}", self.column)?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Token {
    pub word: String,
    pub kind: TokenKind,
    pub loc: Location,
}

impl Token {
    const KEYWORDS: &'static [&str] = &[
        "func", "return", "use", "defer", "const", "mut", "switch", "if", "else", "while", "for",
        "loop", "struct", "public", "private", "impl", "self", "shared", "ret",
    ];
    const TYPES: &'static [&str] = &[
        "None", "String", "Str", "Char", "Int", "Float", "Bool", "Byte", "Complex", "Compact",
        "Loose", "Enum", "Usize", "Isize", "Self",
    ];

    pub fn new(
        word: String,
        kind: TokenKind,
        file_name: String,
        line: usize,
        column: usize,
    ) -> Self {
        Token {
            word,
            kind,
            loc: Location::new(file_name, line, column),
        }
    }

    pub fn eof() -> Self {
        Self::new(
            String::from("Eof"),
            TokenKind::Eof,
            String::from("EOF"),
            0,
            0,
        )
    }

    pub fn is_keyword(word: &str, file_name: String, line: usize, column: usize) -> Option<Self> {
        if Self::KEYWORDS.contains(&word) {
            Some(Self::new(word.to_string(), TokenKind::Keyword, file_name, line, column))
        } else {
            None
        }
    }

    pub fn is_type(word: &str, file_name: String, line: usize, column: usize) -> Option<Self> {
        if Self::TYPES.contains(&word) {
            Some(Self::new(word.to_string(), TokenKind::Type, file_name, line, column))
        } else {
            None
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kind == TokenKind::Eof {
            return Ok(());
        }

        let offset_kind = {
            let longest = TokenKind::get_longest_name();
            let offset = longest - self.kind.to_string().len();
            " ".repeat(offset)
        };

        // Location -> 
        // file:0:0 -> kind | word

        write!(f, "{}-> {}{} | {:?}", self.loc, offset_kind, self.kind, self.word)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_kind_longest_name() {
        assert_eq!(TokenKind::get_longest_name(), 16);
    }
}
