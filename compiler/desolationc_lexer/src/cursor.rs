use std::str::Chars;

pub(crate) const EOF_CHAR: char = '\0';

#[derive(Debug, Default)]
pub struct Position {
    index: usize,
    col_no: usize,
    line_no: usize
}

impl Position {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn col_no(&self) -> usize {
        self.col_no
    }

    pub fn line_no(&self) -> usize {
        self.line_no
    }
}

#[derive(Debug)]
pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
    position: Position
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            len_remaining: source.len(),
            chars: source.chars(),
            position: Position::default()
        }
    }

    pub(crate) fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.position.index += 1;
        if ch == '\n' {
            self.position.col_no = 0;
            self.position.line_no += 1;
        } else {
            self.position.col_no += 1;
        }
        Some(ch)
    }

    pub(crate) fn first(&mut self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn second(&mut self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn position(&self) -> &Position {
        &self.position
    }
}

