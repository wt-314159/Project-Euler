use std::time::Instant;

use common::primes::Primes;

fn main() {
    let max = 1_000_000u32;

    let start = Instant::now();
    let prime_sums = eratosthenes_sum_in_place(max);
    let (longest_seq, prime) = find_longest_sequence(&prime_sums, 21);
    let elapsed = Instant::now().duration_since(start);
    println!("Found sequence of length: {longest_seq} for prime: {prime} in {elapsed:?}");
}

fn find_longest_sequence(prime_sums: &[u32], longer_than: usize) -> (usize, u32) {
    let length = prime_sums.len();
    let mut longest = longer_than;
    let mut for_prime = 0;
    let mut shift = 0;
    let mut base = 0;
    while longest < length - base {
        let mut last = 0;
        for i in base..length {
            if (prime_sums[i] - shift).is_prime() {
                last = i;
            }
        }
        if last > 0 {
            // One or more primes found
            let distance = last - base + 1;
            if distance > longest {
                longest = distance;
                for_prime = prime_sums[last] - shift;
            }
        }
        shift = prime_sums[base];
        base += 1;
    }
    (longest, for_prime)
}

fn eratosthenes_sum_in_place(max: u32) -> Vec<u32> {
    // rough heuristic for how many primes we need to be able to sum to at least max
    let prime_max: u32 = max.isqrt() * 5;
    let num_elements = (prime_max - 1) / 2;
    let num_elements: usize = num_elements.try_into().expect("Failed to parse to usize");
    let mut is_prime: Vec<bool> = vec![true; num_elements];
    let limit = (prime_max.isqrt() / 2) as usize; // if above cast to usize didn't panic, this won't
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

    let mut prime_sum: Vec<u32> = vec![2];
    let mut curr_sum = 2;
    for (i, is_prime) in is_prime.iter().enumerate() {
        if *is_prime {
            // 2i + 3 = a
            let num = 2 * i + 3;
            curr_sum += num;
            if curr_sum > max as usize {
                println!("Num: {num}");
                break;
            }
            prime_sum.push(curr_sum as u32);
        }
    }
    prime_sum
}
