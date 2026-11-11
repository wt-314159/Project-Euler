use common::primes::Primes;

fn main() {
    let primes = usize::get_primes(2_000_000);
    let sum: usize = primes.iter().sum();
    println!("Sum of primes under 2,000,000 = {sum}");
}
