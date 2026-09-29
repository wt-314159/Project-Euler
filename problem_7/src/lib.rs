use common::fill_primes;

pub fn find_nth_prime(n: usize) -> usize {
    find_nth_prime_using_ratio(n, 2)
}

pub fn find_nth_prime_using_ratio(n: usize, upper_bound_ratio: usize) -> usize {
    let mut primes = Vec::with_capacity(n);
    let mut max = n * upper_bound_ratio;
    while primes.len() < n {
        if let Some(biggest_prime) = primes.last()
            && max < biggest_prime * upper_bound_ratio
        {
            max = biggest_prime * upper_bound_ratio;
        }
        fill_primes(&mut primes, max);
    }
    primes
        .get(n - 1)
        .copied()
        .expect("Weren't enough primes, loop condition should ensure this doesn't happen")
}
