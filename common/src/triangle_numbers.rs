use super::primes::get_primes;

pub fn find_triangle_number_divisors(num_divisors: usize) -> usize {
    // A number N can be expressed as `N = p1^a1 * p2^a2 + ...`,
    // where `pn` is a prime factor with exponent `an`
    // The number of divisors can then be computed by:
    // `D(N) = (a1 + 1) * (a2 + 1) * ...
    //
    // The triangle numbers can be obtained by:
    // t = n * (n+1) / 2
    // the n and n + 1 factors are necessarily co-prime (cannot
    // have a common prime factor), since the difference between
    // them is just 1, so the largest number they are both divisible
    // by is also 1.
    // So the number of divisors can be found from:
    // D(t) = D(n/2) * D(n+1)       if n is even
    // D(t) = D(n) * D((n+1)/2)     if n is odd
    //
    // Use this together with prime factorisation to greatly speed up process
    // Also once we've calculated D(n+1), we can use that for next value as D(n)

    // get primes
    let primes = get_primes(num_divisors * 10);
    let mut n: usize = 3;
    let mut dn: usize = 2; // Number of divisors for any prime
    let mut count: usize = 0;

    while count < num_divisors {
        n += 1;
        let mut n1 = if n.is_multiple_of(2) { n / 2 } else { n };
        let mut dn1 = 1;
        for prime in primes.iter() {
            if prime * prime > n1 {
                dn1 *= 2;
                break;
            }

            let mut exponent = 1;
            while n1 % prime == 0 {
                n1 /= prime;
                exponent += 1;
            }
            dn1 *= exponent;
            if n1 == 1 {
                break;
            }
        }
        count = dn * dn1;
        dn = dn1;
    }
    n * (n + 1) / 2
}

pub fn find_triangle_number_naive(num_divisors: usize) -> usize {
    assert!(num_divisors >= 2);
    let mut triangle_number: usize = 1;
    let mut to_add = 2;
    loop {
        let mut divisors = 2;
        let sqrt = triangle_number.isqrt();
        for i in 2..sqrt {
            if triangle_number.is_multiple_of(i) {
                // there are 2 divisors, i and the number we multipy it by
                divisors += 2;
            }
        }
        // square root only counts as one divisor
        if sqrt * sqrt == triangle_number {
            divisors += 1;
        }
        if divisors >= num_divisors {
            break triangle_number;
        }

        triangle_number += to_add;
        to_add += 1;
    }
}
