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
        first_word("av");
    }

    #[test]
    fn test_middle() {
        middle(&[1, 2, 3]);
    }
}
