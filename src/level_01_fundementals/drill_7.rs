pub fn parse_and_double(num_txt: &str) -> Option<i32> {
    let x = num_txt.trim();
    let x = x.parse::<i32>().ok();
    x.and_then(|x| x.checked_mul(2))
}

pub fn next_power_of_two(limit: u32) -> u32 {
    let mut n = 1;
    loop {
        if n >= limit {
            break n;
        }
        n *= 2
    }
}

pub fn first_negative(grid: &[[i32; 3]]) -> Option<(usize, usize)> {
    let mut x = None;
    'outer: for (ri, ra) in grid.iter().enumerate() {
        for (ci, cv) in ra.iter().enumerate() {
            if *cv < 0 {
                x = Some((ri, ci));
                break 'outer;
            }
        }
    }
    x
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn verify_parse_and_double() {
        assert_eq!(parse_and_double(" 23 "), Some(46));
        assert_eq!(parse_and_double(" -23 "), Some(-46));
        assert_eq!(parse_and_double(".23.   "), None);
        assert_eq!(parse_and_double(" 30000_000"), None);
        assert_eq!(parse_and_double(" 2000000000"), None);
        assert_eq!(parse_and_double("two_50"), None);
        assert_eq!(parse_and_double(" "), None);
        assert_eq!(parse_and_double(""), None);
    }

    #[test]
    fn verify_next_power_of_two() {
        assert_eq!(next_power_of_two(0), 1);
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(8), 8);
        assert_eq!(next_power_of_two(16), 16);
        assert_eq!(next_power_of_two(100), 128);
        assert_eq!(next_power_of_two(300), 512);
    }

    #[test]
    fn verify_first_negative() {
        assert_eq!(first_negative(&[]), None);
        assert_eq!(first_negative(&[[1, 2, 3], [2, 3, 4], [10, 20, 30]]), None);
        assert_eq!(
            first_negative(&[[-1, 2, 3], [2, 3, 4], [10, 20, 30]]),
            Some((0, 0))
        );
        assert_eq!(
            first_negative(&[[1, 2, 3], [2, 3, 4], [-10, 20, 30]]),
            Some((2, 0))
        );
        assert_eq!(
            first_negative(&[[1, 2, -3], [2, 3, 4], [-10, 20, 30]]),
            Some((0, 2))
        );
    }
}
