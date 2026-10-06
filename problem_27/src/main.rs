use common::primes::{fill_primes, get_primes};

fn main() {
    let mut checker = PrimeChecker::with_initial_max(10_000);
    let mut max_primes = 0;
    let mut max_a = 0;
    let mut max_b = 0;

    for a in -999..1000 {
        for b in -1000..=1000 {
            let num_primes = checker.count_primes(a, b);
            if num_primes > max_primes {
                max_primes = num_primes;
                max_a = a;
                max_b = b;
            }
        }
    }
    println!(
        "Found {max_primes} for a: {max_a} and b: {max_b}, with product {}",
        max_a * max_b
    );
}

struct PrimeChecker {
    max: usize,
    primes: Vec<usize>,
}

impl PrimeChecker {
    fn with_initial_max(max: usize) -> Self {
        let primes = get_primes(max);
        PrimeChecker { max, primes }
    }

    fn is_prime(&mut self, num: usize) -> bool {
        if num > self.max {
            fill_primes(&mut self.primes, num);
            self.max = num
        }
        self.primes.contains(&num)
    }

    fn count_primes(&mut self, a: isize, b: isize) -> usize {
        let mut counter = 0;
        for i in 0.. {
            let output = i * i + a * i + b;
            if output < 2 {
                break;
            }
            if self.is_prime(output as usize) {
                counter += 1;
            } else {
                break;
            }
        }
        counter
    }
}
