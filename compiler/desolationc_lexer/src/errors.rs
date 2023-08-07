use miette::{Diagnostic, SourceSpan, SourceOffset};
use thiserror::Error;

use crate::cursor::Position;

#[derive(Debug, Error, Diagnostic)]
pub enum CursorError {
    #[error("unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("invalid character: {0}")]
    InvalidCharacter(char),
    #[error("invalid token at position")]
    InvalidToken {
        #[source_code]
        src: String,
        #[label("position")]
        position: SourceOffset,
    }
}
