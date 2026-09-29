pub fn is_prime_eratosthenes(num: usize) -> bool {
    let primes = get_primes(num);
    println!("{primes:?}");
    Some(num) == dbg!(primes).last().copied()
}

pub fn get_primes(max: usize) -> Vec<usize> {
    let max = max + 1;
    let half = max / 2;
    let mut is_prime: Vec<bool> = vec![true; max];
    let mut primes = Vec::new();

    for i in 2..=half {
        if is_prime[i] {
            for j in (i * i..max).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    println!("{is_prime:?}");
    for (i, is_prime) in is_prime.iter().enumerate().skip(2) {
        if *is_prime {
            primes.push(i);
        }
    }
    primes
}

pub fn fill_primes(primes: &mut Vec<usize>, max: usize) {
    let max = max + 1;
    let half = max / 2;
    let start: usize = primes.last().copied().unwrap_or(1) + 1;
    println!("{max} {start} {}", max - start);
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

    for i in start..=half {
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
        assert_eq!(&PRIMES_TO_100, &get_primes(100).as_slice())
    }

    #[test]
    fn fill_primes_100() {
        let mut primes = Vec::with_capacity(25);
        fill_primes(&mut primes, 100);
        assert_eq!(&PRIMES_TO_100, primes.as_slice())
    }

    #[test]
    fn is_prime_eratosthenes_100() {
        for i in 0..=100 {
            println!("{i}");
            assert!(is_prime_eratosthenes(i) == PRIMES_TO_100.contains(&i));
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
