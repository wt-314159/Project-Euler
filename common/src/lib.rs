use fractions::find_divisors;
use num_bigint::BigUint;

pub mod combinatorics;
pub mod fractions;
pub mod primes;
pub mod squares;
pub mod triangle_numbers;

pub fn factorial(num: u64) -> u64 {
    (2..=num).product()
}

pub fn big_factorial(num: u32) -> BigUint {
    (2..=num).fold(BigUint::ONE, |prod, n| prod * n)
}

pub fn digits(mut n: u32) -> impl Iterator<Item = u32> {
    std::iter::from_fn(move || {
        if n > 0 {
            let digit = n % 10;
            n /= 10;
            Some(digit)
        } else {
            None
        }
    })
}

pub fn digits_big_endian(n: u32) -> impl Iterator<Item = u32> {
    let pow = if n == 0 { 0 } else { n.ilog10() };
    let mut div = 10u32.pow(pow);
    let mut prev = 0;
    let mut last = true;
    std::iter::from_fn(move || {
        if div == 1 && !last {
            None
        } else {
            let temp = prev * 10;
            prev = n / div;
            let digit = prev - temp;
            if div == 1 {
                last = false;
            } else {
                div /= 10;
            }
            Some(digit)
        }
    })
}

pub fn from_digits(digits: &[u32]) -> u32 {
    digits.iter().fold(0, |acc, d| acc * 10 + d)
}

pub fn get_bits(num: u32) -> impl DoubleEndedIterator<Item = u8> {
    (0..32).rev().map(move |i| ((num >> i) & 1) as u8)
}

pub fn unordered_eq<T: Ord>(a: &mut [T], b: &mut [T]) -> bool {
    a.sort();
    b.sort();
    a == b
}

pub fn find_abundant_numbers(limit: usize) -> Vec<usize> {
    let mut abundant_numbers = Vec::new();
    for i in 12..=limit {
        let sum: usize = find_divisors(i).sum();
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

impl Default for BigFibonacci {
    fn default() -> Self {
        Self::new()
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

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unordered_eq_test() {
        let mut a = vec![4, 2, 9, 8, 2, 7, 4];
        let mut b = vec![2, 9, 4, 7, 4, 2, 8];
        assert!(unordered_eq(&mut a, &mut b))
    }

    #[test]
    fn unordered_eq_different() {
        let mut a = vec![1, 2, 3, 4];
        let mut b = vec![5, 6, 7, 8];
        assert!(!unordered_eq(&mut a, &mut b));
    }

    #[test]
    fn ordered_digits_test() {
        let num = 1234567890;
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            digits_big_endian(num).collect::<Vec<_>>()
        )
    }
}
