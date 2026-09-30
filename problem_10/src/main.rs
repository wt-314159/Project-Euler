use common::primes::get_primes;

fn main() {
    let primes = get_primes(2_000_000);
    let sum: usize = primes.iter().sum();
    println!("Sum of primes under 2,000,000 = {sum}");
}
