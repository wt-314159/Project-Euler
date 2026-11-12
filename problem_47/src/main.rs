use std::time::Instant;

fn main() {
    let limit = 150_000;
    let start = Instant::now();
    let factors_count = get_prime_factors_count(limit);
    let num = find_consecutive_nums(647, 4, factors_count);
    let elapsed = Instant::now().duration_since(start);
    match num {
        Some(answer) => println!("found consecutive numbers starting at: {answer} in {elapsed:?}"),
        None => println!("didn't find an answer in {limit}"),
    }
}

fn find_consecutive_nums(
    start: usize,
    num_factors: usize,
    factor_counts: Vec<usize>,
) -> Option<usize> {
    let mut count = 0;
    for (i, factors) in factor_counts.iter().enumerate().skip(start) {
        if *factors == num_factors {
            count += 1;
            if count == num_factors {
                return Some(i - num_factors + 1);
            }
        } else {
            count = 0;
        }
    }
    None
}

fn get_prime_factors_count(limit: usize) -> Vec<usize> {
    let mut counts = vec![0; limit + 1];
    for n in 2..counts.len() {
        if counts[n] == 0 {
            for product in (n + n..counts.len()).step_by(n) {
                counts[product] += 1;
            }
        }
    }
    counts
}
