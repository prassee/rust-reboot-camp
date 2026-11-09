pub fn reverse(text: &str) -> String {
    if text.is_empty() || text.len() == 1 {
        text.to_string()
    } else {
        let dynamic_size = text.chars().count();
        let mut rev_chars = vec![' '; dynamic_size];
        let mut pointer = dynamic_size;
        for i in text.chars() {
            rev_chars[pointer - 1] = i;
            pointer -= 1;
        }
        rev_chars.iter().collect::<String>()
    }
}

pub fn vec_reverse(text: &str) -> String {
    let mut input: Vec<char> = text.chars().collect();
    let mut output: Vec<char> = Vec::new();
    while let Some(a) = input.pop() {
        output.push(a)
    }
    output.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::level_01_fundementals::drill_3::{reverse, vec_reverse};

    #[test]
    fn verify_reversing() {
        // "an empty string"
        assert_eq!(reverse(""), "".to_string());
        // "a single character"
        assert_eq!(reverse("a"), "a".to_string());
        // "plain ASCII, such as "rust""
        assert_eq!(reverse("rust"), "tsur".to_string());
        // "text with accented characters, such as "héllo""
        assert_eq!(reverse("héllo"), "olléh".to_string());
        // "text with an emoji, such as "ab🦀""
        assert_eq!(reverse("ab🦀"), "🦀ba".to_string());
    }

    #[test]
    fn vec_approach() {
        // "an empty string"
        assert_eq!(vec_reverse(""), "".to_string());
        // "a single character"
        assert_eq!(vec_reverse("a"), "a".to_string());
        // "plain ASCII, such as "rust""
        assert_eq!(vec_reverse("rust"), "tsur".to_string());
        // "text with accented characters, such as "héllo""
        assert_eq!(vec_reverse("héllo"), "olléh".to_string());
        // "text with an emoji, such as "ab🦀""
        assert_eq!(vec_reverse("ab🦀"), "🦀ba".to_string());
    }
}
