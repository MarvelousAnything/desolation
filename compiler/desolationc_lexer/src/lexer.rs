use miette::Result;

use desolationc_tokens::{tokens::Token, types::TokenType};

use crate::{cursor::Cursor, tokenstream::TokenStream};

pub struct Lexer<'a> {
    source: String,
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn read_str(source: &'a str) -> Self {
        Self {
            source: source.to_string(),
            cursor: Cursor::new(source),
        }
    }

    pub fn lex(&self) -> Option<TokenStream> {
        None
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();



        Ok(Token::new(
            TokenType::Unknown(self.cursor.first()),
            self.cursor.position().index(),
            self.cursor.position().line_no(),
            self.cursor.position().col_no(),
        ))
    }


    pub fn skip_whitespace(&mut self) {
        while self.cursor.first().is_ascii_whitespace() {
            self.cursor.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let source = "fun init() {\n\tprinti(1)\n}\n";
        let lexer = Lexer::read_str(source);
    }
}
