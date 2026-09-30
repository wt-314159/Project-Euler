/// warning: much slower than testing primality through trial division,
/// use `is_prime` instead.
#[deprecated(note = "Please use `is_prime` instead.")]
pub fn is_prime_eratosthenes(num: usize) -> bool {
    let primes = get_primes(num);
    Some(num) == primes.last().copied()
}

pub fn is_prime(num: usize) -> bool {
    if num < 2 {
        return false;
    } else if num < 4 {
        return true;
    } else if num.is_multiple_of(2) {
        return false;
    } else if num < 9 {
        return true; // have already ruled out all non-primes less than 9
    } else if num.is_multiple_of(3) {
        return false;
    } else {
        let max = num.isqrt();
        let mut i = 5;
        while i <= max {
            if num.is_multiple_of(i) {
                return false;
            }
            if num.is_multiple_of(i + 2) {
                return false;
            }
            i += 6;
        }
        return true;
    }
}

pub fn get_primes(max: usize) -> Vec<usize> {
    let max = max + 1; // Do this to include the specified max
    // Only store the odd numbers, since evens (except 2) aren't prime
    // nth index stores the number 2n + 3 e.g. (3, 5, 7, ...)
    let num_elements = (max - 2) / 2;
    let mut is_prime: Vec<bool> = vec![true; num_elements];
    // Only need to check numbers up to square root of max
    // index of root 2i + 3 = root  ->  i = (root - 3) / 2
    let limit = (max.isqrt() - 1) / 2;
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

    let mut primes = vec![2];
    for (i, is_prime) in is_prime.iter().enumerate() {
        if *is_prime {
            // 2i + 3 = a
            primes.push(2 * i + 3);
        }
    }
    primes
}

/// warning: much slower than getting primes using seive of eratosthenes
/// method, use `get_primes` instead.
#[deprecated(note = "Please use `get_primes` instead.")]
pub fn get_primes_slow(max: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    for i in 2..=max {
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes
}

pub fn fill_primes(primes: &mut Vec<usize>, max: usize) {
    let max = max + 1;
    let root = max.isqrt();
    let start: usize = primes.last().copied().unwrap_or(1) + 1;
    let mut is_prime: Vec<bool> = vec![true; max - start];

    if !primes.is_empty() {
        for p in primes.iter() {
            for j in (*p * *p..max).step_by(*p) {
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
            primes.push(i + start);
        }
    }
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
        assert_eq!(&PRIMES_TO_100, &get_primes(100).as_slice());
    }

    #[test]
    fn fill_primes_100() {
        let mut primes = Vec::with_capacity(25);
        fill_primes(&mut primes, 100);
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
            assert!(is_prime(i) == PRIMES_TO_100.contains(&i));
        }
    }

    #[test]
    fn get_primes_7() {
        assert_eq!(&[2, 3, 5, 7], &get_primes(7).as_slice());
    }

    #[test]
    fn fill_primes_7() {
        let mut primes = Vec::new();
        fill_primes(&mut primes, 7);
        assert_eq!(&[2, 3, 5, 7], &primes.as_slice());
    }
}
