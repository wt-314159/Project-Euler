use std::ops::{Add, Mul};

use fractions::find_divisors;
use num_bigint::BigUint;

pub mod combinatorics;
pub mod fractions;
pub mod primes;
pub mod squares;
pub mod triangle_numbers;

pub trait Integer: Sized + Copy + PartialOrd + Mul<Output = Self> + Add<Output = Self> {
    const ZERO: Self;
    const ONE: Self;
    const TEN: Self;
    fn checked_factorial(self) -> Option<Self>;
    fn unchecked_factorial(self) -> Self;
    fn big_factorial(self) -> BigUint;
    fn is_prime(&self) -> bool;
    fn digits_rev(self) -> impl Iterator<Item = Self>;
    fn digits_vec(self) -> Vec<Self>;
    fn gcd(self, other: &Self) -> Self;
}

macro_rules! impl_integer {
    ($($t:ty),* $(,)?) => {
        $(
            impl Integer for $t {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const TEN: Self = 10;

                fn checked_factorial(self) -> Option<Self> {
                    (2..=self).try_fold(Self::ONE, |acc, n| acc.checked_mul(n))
                }

                fn unchecked_factorial(self) -> Self {
                    (2..=self).product()
                }

                fn big_factorial(self) -> BigUint {
                    (2..=self).fold(BigUint::ONE, |acc, n| acc * n)
                }

                fn is_prime(&self) -> bool {
                    if *self < 2 {
                        false
                    } else if *self < 4 {
                        true
                    } else if self.is_multiple_of(2) {
                        false
                    } else if *self < 9 {
                        true // have already ruled out all non-primes less than 9
                    } else if self.is_multiple_of(3) {
                        false
                    } else {
                        let max = self.isqrt();
                        let mut i = 5;
                        while i <= max {
                            if self.is_multiple_of(i) {
                                return false;
                            }
                            if self.is_multiple_of(i + 2) {
                                return false;
                            }
                            i += 6;
                        }
                        true
                    }
                }

                fn digits_vec(self) -> Vec<Self> {
                    let mut digits: Vec<_> = self.digits_rev().collect();
                    digits.reverse();
                    digits
                }

                fn digits_rev(mut self) -> impl Iterator<Item = Self> {
                    std::iter::from_fn(move || {
                        if self > 0 {
                            let digit = self % 10;
                            self /= 10;
                            Some(digit)
                        } else {
                            None
                        }
                    })
                }

                fn gcd(self, other: &Self) -> Self {
                    if self == 0 {
                        return *other;
                    } else if *other == 0 {
                        return self;
                    }

                    let (mut a, mut b) = (self, *other);
                    // Bit shift by number of trailing zeros
                    // (same as repeatedly dividing by 2 until odd)
                    let a_zeros = a.trailing_zeros();
                    a >>= a_zeros;
                    let b_zeros = b.trailing_zeros();
                    b >>= b_zeros;

                    let l_shift = a_zeros.min(b_zeros);

                    loop {
                        // Ensure a <= b
                        if a > b {
                            (a, b) = (b, a);
                        }
                        b -= a;

                        if b == 0 {
                            return a << l_shift;
                        }
                        a >>= a.trailing_zeros();
                    }
                }
            }
        )*
    }
}

impl_integer!(u32, u64, usize);

pub trait IntegerSliceExt<T: Integer> {
    fn gcd(&self) -> T;
    fn from_digits(&self) -> T;
}

impl<T: Integer> IntegerSliceExt<T> for [T] {
    fn gcd(&self) -> T {
        self.iter().fold(T::ZERO, T::gcd)
    }

    fn from_digits(&self) -> T {
        self.iter().fold(T::ZERO, |acc, d| acc * T::TEN + *d)
    }
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

    #[test]
    fn gcd_test() {
        assert_eq!(6, 24u32.gcd(&30));
        assert_eq!(7, 28u32.gcd(&21));
    }

    #[test]
    fn gcd_slice_test() {
        assert_eq!(2, vec![28u32, 34, 14].gcd());
        assert_eq!(6, vec![42u32, 66, 96].gcd());
    }
}
