use common::{
    digits_big_endian, from_digits,
    primes::{fill_primes, get_primes, is_prime},
};

fn main() {
    let trunc_primes = find_left_right_truncatable_primes(11);
    println!("{trunc_primes:?}");
    println!("Sum: {}", trunc_primes.iter().sum::<u64>())
}

fn find_left_right_truncatable_primes(count: usize) -> Vec<u64> {
    // take a guess at the max number of primes
    let mut max = 1_000_000;
    let mut trunc_primes = Vec::new();
    let mut primes = get_primes(max);
    // Skip primes less than 10
    let mut skip = 4;
    while trunc_primes.len() < count {
        println!("Skipping first {skip} primes");
        for prime in primes.iter().skip(skip) {
            let digits: Vec<_> = digits_big_endian(*prime as u32).collect();
            if is_right_truncatable(&digits) && is_left_truncatable(&digits) {
                // it's a right-truncatable prime and left-truncatable
                trunc_primes.push(*prime as u64);
                if trunc_primes.len() == count {
                    break;
                }
            }
        }
        if trunc_primes.len() < count {
            skip = *primes.last().unwrap();
            max *= 2;
            println!("Expanding primes up to {max}");
            fill_primes(&mut primes, max);
        }
    }
    trunc_primes
}

fn is_right_truncatable(digits: &[u32]) -> bool {
    (1..digits.len()).map(|i| &digits[0..i]).all(|trunc| {
        let trunc = from_digits(trunc);
        is_prime(trunc as usize)
    })
}

fn is_left_truncatable(digits: &[u32]) -> bool {
    (1..digits.len()).map(|i| &digits[i..]).all(|trunc| {
        let trunc = from_digits(trunc);
        is_prime(trunc as usize)
    })
}
