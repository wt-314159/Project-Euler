use std::time::Instant;

use common::primes::Primes;

fn main() {
    let start = Instant::now();
    let num = find_consecutive_numbers(647, 4);
    let elapsed = Instant::now().duration_since(start);
    println!("Found consecutive numbers starting at: {num} in {elapsed:?}");
}

fn find_consecutive_numbers(start: usize, num_factors: usize) -> usize {
    let mut count = 0;
    let mut i = start;
    loop {
        let factors = i.get_distinct_prime_factors();
        if factors.len() == num_factors {
            count += 1;
            if count == num_factors {
                return i - num_factors + 1;
            }
        } else {
            count = 0;
        }
        i += 1;
    }
}
