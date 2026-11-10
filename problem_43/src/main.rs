use common::{IntegerSliceExt, combinatorics::PermutationsEx};

const PRIMES: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];

fn main() {
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut sum = 0;
    let start = std::time::Instant::now();
    digits.for_each_permutation(|d| {
        if is_substring_divisible(d) {
            let num = d.to_number();
            sum += num as u64;
        }
    });
    let duration = std::time::Instant::now().duration_since(start);
    println!("Found answer: {sum} in {duration:?}");
}

fn is_substring_divisible(digits: &[u32]) -> bool {
    assert!(digits.len() == 10);
    digits.windows(3).skip(1).enumerate().rev().all(|(i, d)| {
        let num = d.to_number();
        num % PRIMES[i] == 0
    })
}
