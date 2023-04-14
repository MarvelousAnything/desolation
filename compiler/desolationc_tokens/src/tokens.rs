use crate::types::TokenType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    token_type: TokenType,
    index: usize,
    line_no: usize,
    col_no: usize
}
