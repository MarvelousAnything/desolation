use desolationc_proc_macros::{stringify_lower, length, split_lexeme};

macro_rules! keyword_enum {
    ($($keyword:ident),+) => {
        #[derive(Debug, Eq, PartialEq, Clone)]
        pub(crate) enum KeywordToken {
            $($keyword),+
        }

        impl KeywordToken {
            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(stringify_lower!($keyword) => Some(KeywordToken::$keyword),)+
                    _ => None,
                }
            }

            pub fn to_string(&self) -> String {
                match self {
                    $(KeywordToken::$keyword => stringify_lower!($keyword).to_string(),)+
                }
            }

            pub fn length(&self) -> usize {
                match self {
                    $(KeywordToken::$keyword => length!($keyword),)+
                }
            }
        }
    };
}

macro_rules! syntax_enum {
    ($($token:ident => $lexeme:literal),+) => {
        #[derive(Debug, Eq, PartialEq, Clone)]
        pub(crate) enum SyntaxToken {
            $($token),+
        }

        impl SyntaxToken {
            pub fn from_char(c: char, next: Option<char>) -> Option<Self> {
                match (c, next) {
                    $(split_lexeme!($lexeme) => Some(SyntaxToken::$token),)+
                    _ => None,
                }
            }

            pub fn length(&self) -> usize {
                match self {
                    $(SyntaxToken::$token => $lexeme.len(),)+
                }
            }
        }
    };
}

keyword_enum!(Var, Fun, If, Else, Until, Loop, Return);

syntax_enum!{
    LBrace => "{",
    RBrace => "}",
    LParen => "(",
    RParen => ")",
    Assign => ":",
    Comma => ",",
    Dot => ".",
    Neq => "!=",
    Not => "!",
    Plus => "+",
    Times => "*",
    Slash => "/",
    Mod => "%",
    And => "&",
    Or => "|",
    Xor => "^",
    Eq => "==",
    Leq => "<=",
    Geq => ">=",
    LShift => "<<",
    RShift => ">>",
    Lt => "<",
    Gt => ">"
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum LiteralToken {
    Character(char),
    Integer(i64),
    StringToken(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum TokenType {
    Keyword(KeywordToken),
    Syntax(SyntaxToken),
    Identifier(String),
    Literal(LiteralToken),
    Unknown(char),
    Eof,
    NL,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_can_be_created_from_string_representations() {
        let keyword = KeywordToken::from_str("var").expect("Var created from string \"var\" ");
        assert_eq!(KeywordToken::Var, keyword)
    }

    #[test]
    fn keywords_have_the_correct_string_representation() {
        assert_eq!("var", KeywordToken::Var.to_string());
        assert_eq!("return", KeywordToken::Return.to_string());
    }

    #[test]
    fn keywords_have_the_correct_length() {
        assert_eq!(3, KeywordToken::Var.length());
        assert_eq!(6, KeywordToken::Return.length());
    }
}
