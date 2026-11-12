use common::{Integer, IntegerSliceExt, combinatorics::PermutationsEx, primes::Primes};
use std::{collections::HashSet, time::Instant};

fn main() {
    let start = Instant::now();
    if let Some(seq) = find_sequence() {
        let elapsed = Instant::now().duration_since(start);
        println!("Found sequence: {seq:?}  in {elapsed:?}");
    } else {
        println!("Didn't find sequence");
    }
}

fn find_sequence() -> Option<Vec<u64>> {
    let primes = u64::get_primes(9999);
    let mut checked: HashSet<u64> = [
        1478, 1487, 1748, 1784, 1847, 1874, 4178, 4187, 4718, 4781, 4817, 4871, 7148, 7184, 7418,
        7481, 7814, 7841, 8147, 8174, 8417, 8471, 8714, 8741,
    ]
    .into_iter()
    .collect();

    // Skip all the 3 digit primes (we want first prime after
    // 1009, which also won't work because of the 2 zeros)
    for p in primes.iter().skip(169) {
        // skip primes we've already checked
        if checked.contains(p) {
            continue;
        }
        let mut digits = p.digits_vec();
        let mut perm_primes = vec![];
        digits.for_each_permutation(|digits| {
            if digits.starts_with(&[0]) || digits.ends_with(&[0]) {
                return;
            }
            let perm = digits.to_number();
            // search for permutation in primes (binary search for speed)
            if primes.binary_search(&perm).is_ok() {
                perm_primes.push(perm);
                checked.insert(perm);
            }
        });

        if perm_primes.len() >= 3 {
            if let Some(answ) = check_prime_diffs(&mut perm_primes) {
                return Some(answ);
            }
        }
    }
    None
}

fn check_prime_diffs(primes: &mut Vec<u64>) -> Option<Vec<u64>> {
    for (i, p1) in primes.iter().enumerate().take(primes.len() - 2) {
        // Check each prime after p1 except for last
        for j in i + 1..primes.len() - 1 {
            let p2 = primes[j];
            let diff = p2 - p1;
            // find the difference, predict what the next prime would be
            // with the same difference after p2, see if it's in the vec
            let next = p2 + diff;
            if primes[j + 1..].contains(&next) {
                return Some(vec![*p1, p2, next]);
            }
        }
    }
    None
}
