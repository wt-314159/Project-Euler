use common::{primes::Primes, squares::Squares};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let exception = find_smallest_exception();
    let elapsed = Instant::now().duration_since(start);
    println!("Found exception {exception} in {elapsed:?}");
}

fn find_smallest_exception() -> u64 {
    let mut start = 9;
    let mut max = 10_000;
    let mut primes = u64::get_primes(max);
    let mut squares = u64::get_squares(max);
    // the index of the next prime bigger than our starting value
    let mut prime_idx = primes.iter().position(|p| *p >= start).unwrap();
    loop {
        let mut next_prime = primes.get(prime_idx).copied().unwrap_or(1);
        for i in (start..max).step_by(2) {
            // Skip any primes (only need to track if current number is
            // equal to the next largest prime in primes, and update
            // the next largest prime to check if so)
            if i == next_prime {
                prime_idx += 1;
                next_prime = primes.get(prime_idx).copied().unwrap_or(1);
                continue;
            }
            // pass the prime idx, allowing us to work backwards
            // from the largest prime less than i to 0
            if !fits_conjecture(i, &primes, prime_idx) {
                return i;
            }
        }
        // double the maximum, find squares and primes up to it,
        // set the start to the prev maximum, and loop
        start = max + 1; // To ensure it's always odd
        max *= 2;
        u64::fill_primes(&mut primes, max);
        u64::fill_squares(&mut squares, max);
    }
}

fn fits_conjecture(num: u64, primes: &[u64], prime_idx: usize) -> bool {
    // prime is much more likely to be closer to the number than to 0,
    // as then there are more options for square numbers that could
    // fit in the difference between them, so start with the
    // largest prime it could be, and work backwords
    for i in (0..prime_idx).rev() {
        let p = primes[i];
        let diff = num - p;
        if diff.is_multiple_of(2) && (diff / 2).is_square() {
            return true;
        }
    }
    false
}
