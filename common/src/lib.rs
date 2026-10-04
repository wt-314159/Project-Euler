pub mod primes;
pub mod squares;
pub mod triangle_numbers;

pub fn find_divisors_sum(num: usize) -> usize {
    let mut sum = 1;
    let limit = num.isqrt(); // only need to check up to square root
    let start = 3;
    // for odd numbers, only check odd divisors
    let mut step = 2;
    if num.is_multiple_of(2) {
        let other_divisor = num / 2;
        sum += 2;
        sum += other_divisor;
        // for even numbers, check all divisors (odd and even)
        step = 1;
    }
    for i in (start..limit).step_by(step) {
        if num.is_multiple_of(i) {
            let other_divisor = num / i;
            sum += i;
            sum += other_divisor;
        }
    }
    if num.is_multiple_of(limit) {
        let other_divisor = num / limit;
        sum += limit;
        // check if square number
        if other_divisor != limit {
            sum += other_divisor;
        }
    }
    sum
}

pub fn find_abundant_numbers(limit: usize) -> Vec<usize> {
    let mut abundant_numbers = Vec::new();
    for i in 12..=limit {
        let sum = find_divisors_sum(i);
        if sum > i {
            abundant_numbers.push(i);
        }
    }
    abundant_numbers
}
