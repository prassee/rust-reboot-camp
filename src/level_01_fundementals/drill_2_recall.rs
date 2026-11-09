pub fn smallest(numbers: &[i32]) -> Option<i32> {
    match numbers {
        [] => None,
        [first, rest @ ..] => {
            let mut small = first;
            for n in rest {
                if *n <= *small {
                    small = n;
                }
            }
            Some(*small)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn verify_smallest() {
        // empty collection
        assert!(smallest(&[]).is_none());
        // a single element
        assert_eq!(smallest(&[1]), Some(1));
        // all negatives
        assert_eq!(smallest(&[-1, -18, -100]), Some(-100));
        // the minimum value appearing more than once
        assert_eq!(smallest(&[100, 18, 1, 90, 67, 1, 90, 34, 56, 23]), Some(1));
        // the minimum at the first position, and the minimum at the last
        assert_eq!(smallest(&[1, 18, 90, 34, 56, 23]), Some(1));
        assert_eq!(smallest(&[18, 23, 90, 34, 56, 1]), Some(1));
    }
}
