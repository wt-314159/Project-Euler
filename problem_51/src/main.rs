use std::time::Instant;

use common::{Integer, IntegerSliceExt, primes::Primes};

fn main() {
    let start = Instant::now();
    let primes = u32::get_primes(1_000_000);
    let test = primes
        .iter()
        .find_map(|p| match get_digits(p).map(count_primes) {
            Some(n) if n >= 8 => Some(p),
            _ => None,
        });
    let elapsed = Instant::now().duration_since(start);
    println!("Found smallest prime with 8 or more matches: {test:?} in {elapsed:?}");
}

fn get_digits(prime: &u32) -> Option<Vec<u32>> {
    let digits = prime.digits_vec();
    if digits.iter().any(|&d| d == 0 || d == 1 || d == 2) {
        Some(digits)
    } else {
        None
    }
}

fn count_primes(mut prime_digits: Vec<u32>) -> u32 {
    let mut max = 0;
    for d in [0, 1, 2] {
        let indices: Vec<usize> = prime_digits
            .iter()
            .enumerate()
            .filter_map(|(i, &m)| (m == d).then_some(i))
            .collect();
        if indices.len() == 0 {
            continue;
        }

        let mut count = 1;
        for n in d + 1..10 {
            for idx in &indices {
                prime_digits[*idx] = n;
            }
            let candidate = prime_digits.to_number();
            if candidate.is_prime() {
                count += 1;
            }
        }
        if count > max {
            max = count;
        }
        for idx in &indices {
            prime_digits[*idx] = d;
        }
    }
    max
}
