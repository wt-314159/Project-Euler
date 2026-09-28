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

    let primes = get_primes(max);
    println!("primes: {primes:?}");
    let mut prime_factors = vec![0; primes.len()];
    // find the prime factors for all numbers from 2 to max
    for i in 2..=max {
        let mut i = i;
        print!("{i} has primes ");
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
        println!("{curr_prime_factors:?}");
        for (idx, p) in curr_prime_factors.iter().enumerate() {
            prime_factors[idx] = std::cmp::max(*p, prime_factors[idx]);
        }
    }

    println!("");
    println!("{prime_factors:?}");
    let mut sum = 1;
    for (idx, count) in prime_factors.iter().enumerate() {
        let to_add = primes[idx].pow((*count).try_into().unwrap());
        println!("{}^{} = {}", primes[idx], count, to_add);
        sum *= to_add;
    }
    sum
}
