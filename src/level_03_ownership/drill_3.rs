pub fn double_all(num: &mut [i32]) {
    let iter = 0..num.len();
    for i in iter {
        num[i] *= 2;
    }
}

pub fn zero_first(num: &mut [i32]) -> bool {
    if num.is_empty() {
        false
    } else {
        num[0] = 0;
        true
    }
}

pub fn append_total(num: &mut Vec<i32>) {
    let mut sum = 0;
    for i in num.iter() {
        sum += i;
        // using this inside the loop runs in to E0502 - i.e num borrowed as imm inside loop and push needs a mutable ref.
        // num.push(sum);
    }
    num.push(sum);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_double_all() {
        let mut numbers = [1, 2, 3];
        // double all elements in numbers in-place
        double_all(&mut numbers);
        assert_eq!(numbers, [2, 4, 6]);
    }

    #[test]
    fn test_zero_first() {
        let mut numbers = [1, 2, 3];
        let is_zero = zero_first(&mut numbers);
        assert!(is_zero);
        assert_eq!(numbers[0], 0);
        let mut numbers = [];
        let is_zero = zero_first(&mut numbers);
        assert!(!is_zero);
        // Throws index out of bounds error
        // assert_eq!(numbers[0], 0);
        let mut numbers = [1];
        let is_zero = zero_first(&mut numbers);
        assert!(is_zero);
        assert_eq!(numbers[0], 0);
    }

    #[test]
    fn test_append_all() {
        let mut numbers = vec![1, 2, 3];
        let len_b4_app = numbers.len();
        // double all elements in numbers in-place
        append_total(&mut numbers);
        assert_eq!(numbers.len(), len_b4_app + 1);
        assert_eq!(numbers[numbers.len() - 1], 6);
        assert_eq!(numbers, [1, 2, 3, 6]);

        let mut numbers = vec![];
        let len_b4_app = numbers.len();
        // double all elements in numbers in-place
        append_total(&mut numbers);
        assert_eq!(numbers.len(), len_b4_app + 1);
        assert_eq!(numbers[numbers.len() - 1], 0);
        assert_eq!(numbers, [0]);

        let mut numbers = vec![1];
        let len_b4_app = numbers.len();
        // double all elements in numbers in-place
        append_total(&mut numbers);
        assert_eq!(numbers.len(), len_b4_app + 1);
        assert_eq!(numbers[numbers.len() - 1], 1);
        assert_eq!(numbers, [1, 1]);
    }
}
