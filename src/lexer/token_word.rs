use super::TokenKind;

pub struct TokWord<'a>(&'a str, TokenKind);

impl<'a> TokWord<'a> {
    const TOKEN_WORDS: &'a [Self] = &[
        TokWord("==", TokenKind::DoubleEq),
        TokWord("!=", TokenKind::NotEq),
        TokWord("<=", TokenKind::LessEq),
        TokWord(">=", TokenKind::GreatEq),
        TokWord("<", TokenKind::Less),
        TokWord(">", TokenKind::Great),
        TokWord("=", TokenKind::Assign),
        TokWord(";", TokenKind::Semicolon),
        TokWord("->", TokenKind::ThinArrow),
        TokWord("=>", TokenKind::FatArrow),
        TokWord("+", TokenKind::Plus),
        TokWord("-", TokenKind::Minus),
        TokWord("*", TokenKind::Mult),
        TokWord("/", TokenKind::Div),
        TokWord("%", TokenKind::Mod),
        TokWord("<<", TokenKind::ShiftLeft),
        TokWord(">>", TokenKind::ShiftRight),
        TokWord("|>", TokenKind::PipeLine),
        TokWord("|", TokenKind::Pipe),
        TokWord(":=", TokenKind::InferAssign),
        TokWord("+=", TokenKind::PlusAssign),
        TokWord("-=", TokenKind::MinusAssign),
        TokWord("*=", TokenKind::MultAssign),
        TokWord("/=", TokenKind::DivAssign),
        TokWord("&=", TokenKind::ModAssign),
        TokWord("<<=", TokenKind::ShiftLeftAssign),
        TokWord(">>=", TokenKind::ShiftRightAssign),
        TokWord("++", TokenKind::PlusPlus),
        TokWord("--", TokenKind::MinusMinus),
        TokWord(":", TokenKind::Colon),
        TokWord("(", TokenKind::OpenParen),
        TokWord(")", TokenKind::CloseParen),
        TokWord("{", TokenKind::OpenCurly),
        TokWord("}", TokenKind::CloseCurly),
        TokWord("[", TokenKind::OpenBrack),
        TokWord("]", TokenKind::CloseBrack),
        TokWord("@", TokenKind::At),
        TokWord("!", TokenKind::Bang),
        TokWord("?", TokenKind::Question),
        TokWord("$", TokenKind::Dollar),
        TokWord("^", TokenKind::Carrot),
        TokWord("&", TokenKind::Amper),
        TokWord(".", TokenKind::Dot),
        TokWord(",", TokenKind::Comma),
        TokWord("::", TokenKind::DoubleColon),
        TokWord("=+=", TokenKind::Test),
    ];

    pub fn get_kind(word: &str) -> Option<TokenKind> {
        Self::TOKEN_WORDS.iter().find(|t| t.0 == word).map(|t| t.1)
    }
}
