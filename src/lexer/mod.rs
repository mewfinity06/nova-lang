pub mod token;
pub mod token_word;

use token::*;
use token_word::*;

#[derive(Debug)]
pub struct Lexer {
    file_name: String,
    source: Vec<char>,
    cur: usize,
    line: usize,  // Current line number
    column: usize, // Current column number
}

impl Lexer {
    pub fn new(file_name: String, source: String) -> Self {
        Self {
            file_name,
            source: source.chars().collect(),
            cur: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut res = Vec::<Token>::new();

        while let Some(token) = self.next_token() {
            // Update token location with the current line and column
            res.push(token);
        }

        // Add the EOF token with the final position
        let eof_token = Token::eof();
        res.push(eof_token);

        res
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.cur >= self.source.len() {
            return None; // End of source
        }

        let c = self.source[self.cur];

        if c.is_alphabetic() || c == '_' {
            Some(self.lex_identifier())
        } else if c.is_numeric() {
            Some(self.lex_number())
        } else if c == '"' {
            Some(self.lex_string())
        } else if c == '/' && self.peek() == Some('/') {
            Some(self.lex_comment())
        } else if c == '#' && self.peek().is_some_and(|l| l.is_alphabetic() || l == '_') {
            Some(self.lex_directive())
        } else {
            self.lex_symbol()
        }
    }

    fn skip_whitespace(&mut self) {
        while self.cur < self.source.len() && self.source[self.cur].is_whitespace() {
            if self.source[self.cur] == '\n' {
                self.line += 1; // Increment the line number
                self.column = 1; // Reset the column number
            } else {
                self.column += 1; // Move to the next column
            }
            self.cur += 1; // Advance the cursor
        }
    }

    fn advance(&mut self) {
        // Advance the cursor and update the column
        self.cur += 1;
        self.column += 1;
    }

    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    // Update existing lexing functions to use `advance` when moving the cursor
    fn lex_identifier(&mut self) -> Token {
        let start = self.cur;
        while self.cur < self.source.len()
            && (self.source[self.cur].is_alphanumeric() || self.source[self.cur] == '_')
        {
            self.advance();
        }

        let word: String = self.source[start..self.cur].iter().collect();

        if let Some(keyword_token) = Token::is_keyword(&word, self.file_name.clone(), self.line, self.column) {
            return keyword_token;
        }

        if let Some(type_token) = Token::is_type(&word, self.file_name.clone(), self.line, self.column) {
            return type_token;
        }

        Token::new(word, TokenKind::Ident, self.file_name.clone(), self.line, self.column)
    }

    fn lex_directive(&mut self) -> Token {
        self.cur += 1;
        let start = self.cur;
        while self.cur < self.source.len()
            && (self.source[self.cur].is_alphanumeric() || self.source[self.cur] == '_')
        {
            self.advance();
        }

        let word: String = self.source[start..self.cur].iter().collect();

        Token::new(word, TokenKind::Directive, self.file_name.clone(), self.line, self.column)
    }

    fn lex_number(&mut self) -> Token {
        let start = self.cur;
        while self.cur < self.source.len() && self.source[self.cur].is_numeric() {
            self.advance();
        }

        let number: String = self.source[start..self.cur].iter().collect();
        Token::new(number, TokenKind::Number, self.file_name.clone(), self.line, self.column)
    }

    fn lex_string(&mut self) -> Token {
        self.advance(); // Skip opening quote
        let start = self.cur;

        while self.cur < self.source.len() && self.source[self.cur] != '"' {
            self.advance();
        }

        let string: String = self.source[start..self.cur].iter().collect();
        self.advance(); // Skip closing quote
        Token::new(string, TokenKind::StringLiteral, self.file_name.clone(), self.line, self.column)
    }

    fn lex_comment(&mut self) -> Token {
        self.advance_n(2); // Skip the "//"

        let start = self.cur;

        while self.cur < self.source.len() && self.source[self.cur] != '\n' {
            self.advance();
        }

        let comment: String = self.source[start..self.cur].iter().collect();
        Token::new(comment, TokenKind::Comment, self.file_name.clone(), self.line, self.column)
    }

    fn lex_symbol(&mut self) -> Option<Token> {
        if self.cur + 1 < self.source.len() {
            let three_char_symbol: String = self.source[self.cur..self.cur + 3].iter().collect();
            if let Some(token) = TokWord::get_kind(&three_char_symbol) {
                self.cur += 3;
                self.column += 3;
                return Some(Token::new(three_char_symbol, token, self.file_name.clone(), self.line, self.column));
            }

            let two_char_symbol: String = self.source[self.cur..self.cur + 2].iter().collect();
            if let Some(token) = TokWord::get_kind(&two_char_symbol) {
                self.cur += 2;
                self.column += 2;
                return Some(Token::new(two_char_symbol, token, self.file_name.clone(), self.line, self.column));
            }
        }

        let one_char_symbol: String = self.source[self.cur..self.cur + 1].iter().collect();
        self.advance();

        if let Some(token) = TokWord::get_kind(&one_char_symbol) {
            return Some(Token::new(one_char_symbol, token, self.file_name.clone(), self.line, self.column));
        }

        Some(Token::new(one_char_symbol, TokenKind::Unknown, self.file_name.clone(), self.line, self.column))
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.cur + 1).copied()
    }
}
