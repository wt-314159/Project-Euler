fn main() {
    let max = 20;
    let lcm = get_least_common_multiple(max);
    println!("Least common multiple of numbers up to {max}: {lcm}");
}

fn get_primes(max: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; max];
    for i in 2..max {
        if is_prime[i] {
            for j in (i * i..max).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    for i in 2..max {
        if is_prime[i] {
            primes.push(i);
        }
    }
    primes
}

fn get_least_common_multiple(max: usize) -> usize {
    // Get the minimum prime factors of all numbers up to the max
    // In other words, the minimum set of prime factors for all
    // numbers up to the max.
    // Then multiply these together
    //
    // N.B. we could actually skip calculating the numbers of primes
    // in the prime factors of all the numbers up to max. We can simply
    // take each number from 2 to max, and see what's the max power it
    // can be raised to before it "overflows" max (for 2, this would be 4,
    // since 2^4 = 16 and 2^5 = 32). This can just be directly calculated
    // using floor( log(20) / log(x))
    // Of course, for x > sqrt(20), this can only be 1 anyway, so we only
    // need to calculate up to sqrt(20) (up to and including 4)

    let primes = get_primes(max);
    let mut prime_factors = vec![0; primes.len()];
    // find the prime factors for all numbers from 2 to max
    for i in 2..=max {
        let mut i = i;
        let mut curr_prime_factors: Vec<usize> = vec![0; primes.len()];
        for (idx, prime) in primes.iter().enumerate() {
            if i == 0 {
                break;
            }
            while i % prime == 0 {
                i /= prime;
                curr_prime_factors[idx] += 1;
            }
        }
        for (idx, p) in curr_prime_factors.iter().enumerate() {
            prime_factors[idx] = std::cmp::max(*p, prime_factors[idx]);
        }
    }

    println!("");
    let mut sum = 1;
    for (idx, count) in prime_factors.iter().enumerate() {
        let to_add = primes[idx].pow((*count).try_into().unwrap());
        sum *= to_add;
    }
    sum
}
