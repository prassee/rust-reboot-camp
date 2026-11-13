// Eq is not implemented for floating-point types & hence only PartialEq
// Debug to print the struct
// PartialEq derived to allow for operations like ==, !=
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Celsius(pub f64);

// u32 satisfies all the 6 traits
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UserId(pub u32);

// cannot derive Copy since theme of type String does not implement Copy trait
// Eq allowed as long as a field of floating point type like f64 is included
#[derive(Debug, PartialEq, Clone, Default, Eq)]
pub struct Settings {
    pub theme: String,
    pub font_size: u8,
    pub autosave: bool,
}

#[cfg(test)]
mod tests {

    use super::*;

    fn print_user(user: UserId) {
        println!("{:?}", user);
    }

    fn print_celsius(celcius: Celsius) {
        println!("{:?}", celcius);
    }

    #[test]
    fn verify_celsius_derives() {
        let d6_tempr = Celsius(23.3);
        assert_eq!(d6_tempr, Celsius(23.3000));
        assert_ne!(d6_tempr, Celsius(-23.3000));
        assert_ne!(d6_tempr, Celsius(23.30000009));
        print_celsius(d6_tempr);
        // println! borrows d6_tempr for formatting & UserId derives the Copy and Clone trait still usable.
        println!("{:?}", d6_tempr);
        assert_eq!(Celsius::default().0, 0.0);
    }

    #[test]
    fn verify_userid_derives() {
        let d6_user = UserId(23);
        assert_eq!(d6_user, UserId(23));
        print_user(d6_user);
        // println! borrows d6_user for formatting & UserId derives the Copy and Clone trait still usable.
        println!("{:?}", d6_user);
    }

    #[test]
    fn verify_settings_derives() {
        assert_eq!(
            Settings::default(),
            Settings {
                theme: String::new(),
                font_size: 0,
                autosave: false,
            }
        );
        let drill6_settings = Settings {
            theme: "ping".to_string(),
            font_size: 2,
            autosave: false,
        };
        let mut drill6_settings_clone = drill6_settings.clone();
        drill6_settings_clone.theme = "pong".to_string();
        drill6_settings_clone.font_size = 4;

        assert_eq!(
            drill6_settings,
            Settings {
                theme: "ping".to_string(),
                font_size: 2,
                autosave: false,
            }
        );

        assert_eq!(
            drill6_settings_clone,
            Settings {
                theme: "pong".to_string(),
                font_size: 4,
                autosave: false,
            }
        );
    }
}
