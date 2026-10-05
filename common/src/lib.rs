use num_bigint::BigUint;

pub mod combinatorics;
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

pub struct BigFibonacci {
    prev: BigUint,
    curr: BigUint,
}

impl Iterator for BigFibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == BigUint::ZERO {
            self.curr = BigUint::ONE;
        } else {
            let mut temp = self.curr.clone();
            std::mem::swap(&mut self.prev, &mut temp);
            self.curr += temp;
        }
        Some(self.curr.clone())
    }
}

impl BigFibonacci {
    pub fn new() -> Self {
        BigFibonacci {
            prev: BigUint::ZERO,
            curr: BigUint::ZERO,
        }
    }
}

pub struct Fibonacci {
    prev: usize,
    curr: usize,
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == 0 {
            self.curr = 1;
        } else {
            let temp = self.prev;
            self.prev = self.curr;
            self.curr += temp;
        }
        Some(self.curr)
    }
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { prev: 0, curr: 0 }
    }
}
