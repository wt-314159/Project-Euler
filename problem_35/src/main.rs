use common::primes::get_circular_primes;

fn main() {
    let max = 1_000_000;
    let circ_primes = get_circular_primes(max);
    println!("{:?}", circ_primes);
    println!(
        "There are {} circular primes below {max}",
        circ_primes.len()
    );
}
