pub fn simplify_fraction(num: u32, denom: u32) -> (u32, u32) {
    // Find largest common denominator of numerator and denominator
    // Divide both by this largest common denominator
    if denom.is_multiple_of(num) {
        return (1, denom / num);
    }
    for i in (2..=(num / 2)).rev() {
        if num.is_multiple_of(i) && denom.is_multiple_of(i) {
            return (num / i, denom / i);
        }
    }
    return (num, denom);
}

pub fn find_all_simplifications(num: u32, denom: u32) -> impl Iterator<Item = (u32, u32)> {
    let self_opt = if denom.is_multiple_of(num) {
        Some((1u32, denom / num))
    } else {
        None
    };
    (2..=(num / 2))
        .into_iter()
        .filter(move |i| num.is_multiple_of(*i) && denom.is_multiple_of(*i))
        .map(move |i| (num / i, denom / i))
        .chain(self_opt)
}

/// Finds all non-trivial divisors for `num` (i.e. ignoring `1` and `num`)
pub fn find_divisors(num: usize) -> impl Iterator<Item = usize> {
    let limit = num.isqrt(); // Only need to check up to square root
    // check if number is a multiple of limit, and if so, if it's
    // square (since isqrt rounds down, being a multiple of isqrt
    // doesn't necessarily mean it's a square number)
    let limit_opt: Option<_> = match num.is_multiple_of(limit) {
        true if num < 4 => None,
        true => match num / limit {
            other_divisor if other_divisor == limit => Some(vec![limit]),
            other_divisor => Some(vec![limit, other_divisor]),
        },
        false => None,
    };
    // For odd numbers, only check odd divisors, starting with 3
    let (start, step) = match num.is_multiple_of(2) {
        true => (2, 1),
        false => (3, 2),
    };
    (start..limit)
        .step_by(step)
        .filter(move |i| num.is_multiple_of(*i))
        .flat_map(move |i| [i, num / i])
        .chain(limit_opt.into_iter().flatten())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisors_test_12() {
        let twelve_div = vec![2, 3, 4, 6];
        let mut found_div = find_divisors(12).collect::<Vec<_>>();
        found_div.sort();
        assert_eq!(twelve_div, found_div);
    }

    #[test]
    fn divisors_test_100() {
        let hundred_div = vec![2, 4, 5, 10, 20, 25, 50];
        let mut found_div = find_divisors(100).collect::<Vec<_>>();
        found_div.sort();
        assert_eq!(hundred_div, found_div);
    }

    #[test]
    fn divisors_test() {
        for i in 2..=100 {
            let actual_divs = find_actual_divisors(i);
            let mut found_divs = find_divisors(i).collect::<Vec<_>>();
            found_divs.sort();
            assert_eq!(actual_divs, found_divs, "testing finding divisors for {i}");
        }
    }

    #[test]
    fn simplify_fract_test_49_98() {
        let (num, denom) = simplify_fraction(49, 98);
        assert_eq!((1, 2), (num, denom));
    }

    #[test]
    fn simplify_fract_test_30_50() {
        let (num, denom) = simplify_fraction(30, 50);
        assert_eq!((3, 5), (num, denom));
    }

    #[test]
    fn find_all_simplifications_49_98() {
        let actual = vec![(7, 14), (1, 2)];
        let test = find_all_simplifications(49, 98).collect::<Vec<_>>();
        assert_eq!(actual, test);
    }

    #[test]
    fn find_all_simplifications_12_48() {
        let actual = vec![(6, 24), (4, 16), (3, 12), (2, 8), (1, 4)];
        let test = find_all_simplifications(12, 48).collect::<Vec<_>>();
        assert_eq!(actual, test);
    }

    /// Check every single divisor from 2 to num / 2 inclusive
    fn find_actual_divisors(num: usize) -> Vec<usize> {
        (2..=(num / 2)).filter(|i| num.is_multiple_of(*i)).collect()
    }
}
