use common::primes::{fill_primes, get_primes};

fn main() {
    let mut checker = PrimeChecker::with_initial_max(10_000);
    checker.find_most_primes();
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

    fn find_most_primes(&mut self) {
        let mut max_primes = 0;
        let mut max_a = 0;
        let mut max_b = 0isize;
        // b must be prime, as when n = 0, the result of n^2 + an + b
        // is just equal to b
        let b_range: Vec<isize> = self
            .primes
            .iter()
            .take_while(|p| **p <= 1000usize)
            .map(|p| *p as isize)
            .collect();

        for a in -999..1000 {
            for b in b_range.iter() {
                let num_primes = self.count_primes(a, *b);
                if num_primes > max_primes {
                    max_primes = num_primes;
                    max_a = a;
                    max_b = *b;
                }
            }
        }
        println!(
            "Found {max_primes} for a: {max_a} and b: {max_b}, with product {}",
            max_a * max_b
        );
    }

    fn count_primes(&mut self, a: isize, b: isize) -> usize {
        let mut counter = 0;
        for i in 0.. {
            let output = i * i + a * i + b;
            // No primes less than 2 (also prevents us from trying to
            // convert a negative number to a usize)
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
