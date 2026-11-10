pub fn describe(value: Option<i32>) -> String {
    if let Some(e) = value {
        format!("got {e}")
    } else {
        "nothing".to_string()
    }
}

pub fn double_first(numbers: &[i32]) -> Option<i32> {
    // let Some(x) = numbers.first() else {
    //     return None;
    // };
    let x = numbers.first()?;
    Some(x * 2)
}

pub fn classify(value: Option<i32>) -> &'static str {
    match value {
        Some(0) => "zero",
        Some(n) if n < 0 => "negative",
        Some(_) => "positive",
        None => "missing",
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn verify_describe() {
        assert_eq!(describe(None), "nothing".to_string());
        assert_eq!(describe(Some(23)), "got 23".to_string());
        assert_eq!(double_first(&[]), None);
        assert_eq!(double_first(&[2, 3]), Some(4));
        assert_eq!(classify(None), "missing");
        assert_eq!(classify(Some(-1)), "negative");
        assert_eq!(classify(Some(10)), "positive");
        assert_eq!(classify(Some(0)), "zero");
    }
}
