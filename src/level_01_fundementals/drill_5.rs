pub fn get_code_label(code: u16) -> &'static str {
    match code {
        100..=199 => "informational",
        200..=299 => "success",
        300..=399 => "redirect",
        404 | 410 => "gone or missing",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn verify_label() {
        for code in [200, 299] {
            assert_eq!(get_code_label(code), "success");
        }
        for code in [404, 410] {
            assert_eq!(get_code_label(code), "gone or missing");
        }
        for code in [400, 403, 405, 499] {
            assert_eq!(get_code_label(code), "client error");
        }
        for code in [100, 101, 199] {
            assert_eq!(get_code_label(code), "informational");
        }
        for code in [500, 599] {
            assert_eq!(get_code_label(code), "server error");
        }
        for code in [300, 301, 399] {
            assert_eq!(get_code_label(code), "redirect");
        }
        for code in [90, 0, 600] {
            assert_eq!(get_code_label(code), "unknown");
        }
    }
}
