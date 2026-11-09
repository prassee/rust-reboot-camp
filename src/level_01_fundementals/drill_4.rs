pub fn divide(dividend: i32, divisor: i32) -> Result<(i32, i32), String> {
    if divisor == 0 {
        return Err("division by zero".to_string());
    }
    Ok((dividend / divisor, dividend % divisor))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_divide() {
        let (q, r) = divide(23, 4).unwrap();
        assert_eq!(q, 5);
        assert_eq!(r, 3);
        assert_eq!(divide(23, 0), Err("division by zero".to_string()));
        let qr = divide(20, 10).unwrap();
        assert_eq!(qr.0, 2);
        assert_eq!(qr.1, 0);
        assert_eq!(divide(-7, 2), Ok((-3, -1)));
        assert_eq!(divide(7, -2), Ok((-3, 1)));
    }
}
