pub fn to_fahrenheit(celcius: i32) -> f64 {
    (celcius as f64 * 1.8_f64) + 32_f64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn verify_conversion() {
        const TOLERANCE: f64 = 0.00001;
        assert_eq!(to_fahrenheit(-40), -40.0);
        assert_eq!(to_fahrenheit(0), 32.0);
        let result = to_fahrenheit(37);
        let exp = 98.6;
        assert!((result - exp).abs() < TOLERANCE);
        assert_eq!(to_fahrenheit(100), 212.0);
    }
}
