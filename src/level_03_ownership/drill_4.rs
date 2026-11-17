pub fn first_word(s: &str) -> &str {
    s
}

pub fn middle(v: &[i32]) -> &[i32] {
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_word() {
        assert_eq!(first_word("abcd"), "abcd");
        assert_eq!(first_word("ab cd"), "ab");
        assert_eq!(first_word("ab cd ef"), "ab");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word(" "), "");
        assert_eq!(first_word(" ab cd ef"), "");
        assert_eq!(first_word("aaa ab cd ef"), "");
    }

    #[test]
    fn test_middle() {
        middle(&[1, 2, 3]);
    }
}
