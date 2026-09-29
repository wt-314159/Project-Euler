fn main() {
    let num: usize = 600_851_475_143;
    let sqrt = num.isqrt();
    let primes = get_primes(sqrt + 1);
    for i in primes.iter().rev() {
        if num.is_multiple_of(*i) {
            println!("{i} is largest prime factor of {num}");
            break;
        }
    }
}

fn get_primes(limit: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; limit];
    #[allow(clippy::needless_range_loop)]
    for i in 2..limit {
        if is_prime[i] {
            for j in (i * i..limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    for (i, is_prime) in is_prime.iter().enumerate().skip(2) {
        if *is_prime {
            primes.push(i);
        }
    }
    primes
}
