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
        get_primes(&mut primes, max);
    }
    primes
        .get(n - 1)
        .copied()
        .expect("Weren't enough primes, loop condition should ensure this doesn't happen")
}

fn get_primes(primes: &mut Vec<usize>, max: usize) {
    let half = max / 2;
    let start: usize = primes.last().copied().unwrap_or(1) + 1;
    let mut is_prime: Vec<bool> = vec![true; max - start];

    if !primes.is_empty() {
        for p in primes.iter() {
            for j in (*p * *p..max).step_by(*p) {
                if j >= start {
                    is_prime[j - start] = false;
                }
            }
        }
    }

    for i in start..=half {
        if is_prime[i - start] {
            for j in (i * i..max).step_by(i) {
                is_prime[j - start] = false;
            }
        }
    }

    for (i, is_prime) in is_prime.iter().enumerate() {
        if *is_prime {
            primes.push(i + start);
        }
    }
}
