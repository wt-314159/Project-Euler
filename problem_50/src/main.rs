use std::time::Instant;

use common::primes::Primes;

fn main() {
    let max = 1_000_000u32;
    // we know the sequence has at least 21 primes
    // so even if they sum to max, the highest their
    // average value could be is max / 21
    // the average gap between primes near this value is
    // around ln(max / 21), but since we're using log2, this
    // will have a safety margin added in anyway
    // So then the max prime we need to get should be
    // max / 21 + (ln(max / 21) + 5) * 20
    let start = Instant::now();
    let temp = max / 21;
    let prime_max = temp + temp.ilog2() * 11;

    let primes = u32::get_primes(prime_max);
    let (longest_seq, prime) = find_longest_sequence(&primes, max, 20);
    let elapsed = Instant::now().duration_since(start);
    println!("Found sequence of length: {longest_seq} for prime: {prime} in {elapsed:?}");
}

fn find_longest_sequence(primes: &[u32], max: u32, longer_than: usize) -> (usize, u32) {
    let mut longest = longer_than;
    let mut for_prime = 0;
    // All possible starting primes of our sequence
    let mut i = 0;
    while i < primes.len() - longest {
        // start by summing the longest sequence number of primes together
        // (no need to bother checking less than this, since that wouldn't
        // beat our current longest sequence anyway)
        let mut curr_sum: u32 = primes[i..i + longest].iter().sum();
        let mut count = longest;
        for p in primes.iter().skip(i + longest) {
            curr_sum += p;
            if curr_sum > max {
                break;
            }

            count += 1;
            if curr_sum.is_prime() {
                // if primes.binary_search(&curr_sum).is_ok() {
                longest = count;
                for_prime = curr_sum;
            }
        }
        i += 1;
    }
    (longest, for_prime)
}
