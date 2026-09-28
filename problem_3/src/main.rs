fn main() {
    let num: usize = 600_851_475_143;
    let sqrt = num.isqrt();
    let primes = get_primes(sqrt + 1);
    for i in primes.iter().rev() {
        if num % i == 0 {
            println!("{i} is largest prime factor of {num}");
            break;
        }
    }
}

fn get_primes(limit: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; limit];
    for i in 2..limit {
        if is_prime[i] {
            for j in (i * i..limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    for i in 2..limit {
        if is_prime[i] {
            primes.push(i);
        }
    }
    primes
}
