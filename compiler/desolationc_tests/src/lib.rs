#[cfg(test)]
mod tests {
    use desolationc_proc_macros::*;

    #[test]
    pub fn test_split_lexeme() {
        assert_eq!(('>', Some('=')), split_lexeme!(">="));
    }
}
