use std::collections::HashMap;

use crate::{Integer, IntegerSliceExt, combinatorics::RotationsEx};

/// warning: much slower than testing primality through trial division,
/// use `is_prime` instead.
#[deprecated(note = "Please use `is_prime` instead.")]
pub fn is_prime_eratosthenes(num: usize) -> bool {
    let primes = usize::get_primes(num);
    Some(num) == primes.last().copied()
}

pub trait Primes: Integer {
    fn is_prime(&self) -> bool;
    fn get_primes(max: Self) -> Vec<Self>;
    fn fill_primes(primes: &mut Vec<Self>, max: Self);
}

macro_rules! impl_primes {
    ($($t:ty),* $(,)?) => {
        $(
            impl Primes for $t {
                fn is_prime(&self) -> bool {
                    if *self < 2 {
                        false
                    } else if *self < 4 {
                        true
                    } else if self.is_multiple_of(2) {
                        false
                    } else if *self < 9 {
                        true    // have already rules out all non-primes less than 9
                    } else if self.is_multiple_of(3) {
                        false
                    } else {
                        let max = self.isqrt();
                        let mut i = 5;
                        while i <= max {
                            if self.is_multiple_of(i) {
                                return false;
                            }
                            if self.is_multiple_of(i + 2){
                                return false;
                            }
                            i += 6;
                        }
                        true
                    }
                }

                fn get_primes(max: Self) -> Vec<Self> {
                    if max < 2 {
                        return vec![];
                    }
                    // Only store the odd numbers, since evens (except 2) aren't prime
                    // nth index stores the number 2n + 3 e.g. (3, 5, 7, ...)
                    let num_elements = (max - 1) / 2;
                    // This will fail if Self is u64, we're on a 32 bit machine,
                    // and we pass a value for max that is greater than u32::MAX
                    let num_elements: usize = num_elements.try_into().expect("Failed to parse to usize");
                    let mut is_prime: Vec<bool> = vec![true; num_elements];
                    // Only need to check numbers up to square root of max
                    // index of root 2i + 3 = root  ->  i = (root - 3) / 2
                    let limit = (max.isqrt() / 2) as usize;     // if above cast to usize didn't panic, this won't
                    for i in 0..limit {
                        if is_prime[i] {
                            // number `a` stored in index `i` is `a = 2i + 3`
                            // we need `a^2`, which is `4i^2 + 12i + 9`
                            // The index for this should be 2x + 3 = 4i^2 + 12i + 9
                            // So index is `x = 2i^2 + 6i + 3`
                            let start = 2 * i * i + 6 * i + 3;
                            // We want to increment by steps of a, which = 2i + 3
                            for j in (start..num_elements).step_by(2 * i + 3) {
                                is_prime[j] = false;
                            }
                        }
                    }

                    let mut primes: Vec<Self> = vec![2];
                    for (i, is_prime) in is_prime.iter().enumerate() {
                        if *is_prime {
                            // 2i + 3 = a
                            primes.push((2 * i + 3).try_into().expect("Failed to parse to Self"));
                        }
                    }
                    primes
                }

                fn fill_primes(primes: &mut Vec<Self>, max: Self) {
                    let max: usize = (max + 1).try_into().expect("Failed to cast to usize");
                    let root = max.isqrt();
                    let start: usize = (primes.last().copied().unwrap_or(1) + 1)
                        .try_into().expect("Failed to cast to usize");
                    let mut is_prime: Vec<bool> = vec![true; (max - start) as usize];

                    if !primes.is_empty() {
                        for p in primes.iter() {
                            let p: usize = (*p).try_into().expect("Failed to cast to usize");
                            for j in (p * p..max).step_by(p) {
                                if j >= start {
                                    is_prime[j - start] = false;
                                }
                            }
                        }
                    }

                    for i in start..=root {
                        if is_prime[i - start] {
                            for j in (i * i..max).step_by(i) {
                                is_prime[j - start] = false;
                            }
                        }
                    }

                    for (i, is_prime) in is_prime.iter().enumerate() {
                        if *is_prime {
                            primes.push((i + start).try_into().expect("Failed to cast to Self"));
                        }
                    }
                }
            }
        )*
    };
}

impl_primes!(u32, u64, usize);

pub fn get_circular_primes(max: usize) -> Vec<usize> {
    let primes = usize::get_primes(max);
    let prime_map: HashMap<_, _> = primes
        .iter()
        .copied()
        .enumerate()
        .map(|(i, val)| (val, i))
        .collect();
    let mut checked = vec![false; primes.len()];
    let mut circular_primes = Vec::new();
    for (i, prime) in primes.iter().enumerate() {
        if checked[i] {
            continue;
        }
        // All single digit primes are circular
        if *prime < 10 {
            circular_primes.push(*prime);
            continue;
        }

        let mut digits: Vec<_> = prime.digits_rev().collect();
        // digits gets digits of number in reverse order (least significant first)
        digits.reverse();
        let mut circ_primes = Vec::new();
        let mut all_circular = true;
        // Iterate through each rotation of digits
        digits.for_each_rotation(|rotate| {
            let rotate = rotate.to_number();
            if prime_map.contains_key(&(rotate)) {
                // Make sure we don't get duplicates for primes like 11,
                // where changing the order of the digits doesn't change the number
                if rotate != *prime || circ_primes.is_empty() {
                    circ_primes.push(rotate);
                }
                // Mark the prime as checked (whether circular or not)
                checked[prime_map[&rotate]] = true;
            } else {
                all_circular = false;
            }
        });

        if all_circular {
            circular_primes.extend(circ_primes);
        }
    }
    circular_primes
}

/// warning: much slower than getting primes using seive of eratosthenes
/// method, use `get_primes` instead.
#[deprecated(note = "Please use `get_primes` instead.")]
pub fn get_primes_slow(max: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    for i in 2..=max {
        if i.is_prime() {
            primes.push(i);
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;
    const PRIMES_TO_100: [usize; 25] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    #[test]
    fn get_primes_100() {
        assert_eq!(&PRIMES_TO_100, usize::get_primes(100).as_slice());
    }

    #[test]
    fn fill_primes_100() {
        let mut primes = Vec::with_capacity(25);
        usize::fill_primes(&mut primes, 100);
        assert_eq!(&PRIMES_TO_100, primes.as_slice())
    }

    #[allow(deprecated)]
    #[test]
    fn is_prime_eratosthenes_100() {
        for i in 0..=100 {
            assert!(is_prime_eratosthenes(i) == PRIMES_TO_100.contains(&i));
        }
    }

    #[test]
    fn is_prime_100() {
        for i in 0..=100 {
            assert!(i.is_prime() == PRIMES_TO_100.contains(&i));
        }
    }

    #[test]
    fn get_primes_7() {
        assert_eq!(&[2, 3, 5, 7], usize::get_primes(7).as_slice());
    }

    #[test]
    fn fill_primes_7() {
        let mut primes = Vec::new();
        usize::fill_primes(&mut primes, 7);
        assert_eq!(&[2, 3, 5, 7], &primes.as_slice());
    }
}
