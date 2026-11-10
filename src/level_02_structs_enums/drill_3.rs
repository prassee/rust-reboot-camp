#[derive(Debug, PartialEq)]
pub enum Event {
    Close,
    KeyPress(char),
    Paste(String),
    Resize(u32, u32),
    Click { x: i32, y: i32 },
}

impl Event {
    pub fn describe(&self) -> String {
        match self {
            Event::Close => "called Close".to_string(),
            Event::KeyPress(k) => format!("pressed key at {}", k),
            Event::Paste(x) => format!("pasted at {}", x),
            Event::Resize(w, h) => format!("resize at {} , {}", w, h),
            Event::Click { x, y } => format!("click at {} , {}", x, y),
        }
    }

    pub fn typed_chars(&self) -> usize {
        match self {
            Event::KeyPress(_) => 1,
            Event::Paste(s) => s.chars().count(),
            Event::Resize(..) | Event::Click { x: _, y: _ } | Event::Close => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_event_describe() {
        assert_eq!(Event::Close.describe(), "called Close");
        assert_eq!(Event::KeyPress('a').describe(), "pressed key at a");
        assert_eq!(
            Event::Paste("ping".to_string()).describe(),
            "pasted at ping"
        );
        assert_eq!(Event::Resize(23, 34).describe(), "resize at 23 , 34");
        assert_eq!(Event::Click { x: 23, y: 34 }.describe(), "click at 23 , 34");
    }

    #[test]
    fn verify_event_typed_chars() {
        assert_eq!(Event::Close.typed_chars(), 0);
        assert_eq!(Event::KeyPress('a').typed_chars(), 1);
        assert_eq!(Event::Paste("héllo".to_string()).typed_chars(), 5);
        assert_eq!(Event::Resize(23, 67).typed_chars(), 0);
        assert_eq!(Event::Click { x: 23, y: 34 }.typed_chars(), 0);
    }
}
