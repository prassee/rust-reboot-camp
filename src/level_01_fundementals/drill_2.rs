pub fn largest(numbers: &[i32]) -> Option<i32> {
    match numbers {
        [] => None,
        [first, tail @ ..] => {
            let mut large = first;
            for n in tail {
                if *n >= *large {
                    large = n;
                }
            }
            Some(*large)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn verify_largest() {
        // empty collection
        assert!(largest(&[]).is_none());
        // a single element
        assert_eq!(largest(&[1]), Some(1));
        // all negatives
        assert_eq!(largest(&[-1, -18, -100]), Some(-1));
        // the maximum value appearing more than once
        assert_eq!(
            largest(&[1, 18, 100, 90, 67, 100, 90, 34, 56, 23]),
            Some(100)
        );
        // the maximum at the first position, and the maximum at the last
        assert_eq!(largest(&[100, 18, 90, 34, 56, 23]), Some(100));
        assert_eq!(largest(&[18, 23, 90, 34, 56, 100]), Some(100));
    }
}
