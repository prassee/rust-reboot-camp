pub fn first_word(s: &str) -> &str {
    s.split(" ").next().unwrap()
}

pub fn middle(v: &[i32]) -> &[i32] {
    if v.len() < 3 { &[] } else { &v[1..v.len() - 1] }
}

// Experiments

// returns a value referencing data owned by the current function
// fn make_greeting(name: &str) -> &str {
//     format!("hello, {name}").as_str() // E0515 cannot return value referencing temporary value
// }
// correct version of above
// fn make_greeting(name: &str) -> String {
//     format!("hello, {name}")
// }

// fn make_greeting(name: &str) -> String {
//     // let s = name.to_string();
//     let x = format!("hello, {name}");
//     x
// }
// fn make_greeting(name: &str) -> String {
//     format!("hello, {name}")
// }

// fn make_greeting_1(name: &str) -> &str {
//     name.split_whitespace().next().unwrap_or("")
// }

//
// fn default_name() -> &str {
//     // E0106 missing lifetime specifier
//     "default"
// }
//
// fn default_name() -> String {
//     "default".to_string()
// }
// fn default_name() -> &'static str {
//     "default"
// }

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
        assert_eq!(first_word(" a   aaa ab cd ef"), "");
        assert_eq!(first_word("1     a   aaa ab cd ef"), "1");
    }

    #[test]
    fn test_middle() {
        assert_eq!(middle(&[1, 2, 3]), [2]);
        assert_eq!(middle(&[1, 2]), []);
        assert_eq!(middle(&[1]), []);
        assert_eq!(middle(&[]), []);
        assert_eq!(middle(&[1, 2, 3, 4]), [2, 3]);
    }

    #[test]
    fn test_experiment_c() {
        let mut s = String::from("ping pong");
        let w = first_word(&s);
        // make w to live longer than s
        println!("{w}");
        s.clear();
        // throws E0502 this is a error since s does not
        // live long enough as it gets dropped in previous line.
        // println!("{w}");
    }
}
