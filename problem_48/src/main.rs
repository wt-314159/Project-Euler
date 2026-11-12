use std::time::Instant;

use num_bigint::BigInt;

fn main() {
    let start = Instant::now();
    let modulus = BigInt::from(10).pow(10);
    let sum: BigInt = (1..=1000)
        .map(|n| to_power_of_self(n.into(), &modulus))
        .sum();
    let elapsed = Instant::now().duration_since(start);

    println!("Result: {sum} found in {elapsed:?}");
    let pow_10 = BigInt::from(10).pow(10);
    let last_10 = sum % &pow_10;
    println!("last 10 digits: {last_10}");
}

fn to_power_of_self(num: BigInt, modulus: &BigInt) -> BigInt {
    // num.pow(num.clone().try_into().unwrap())
    num.modpow(&num, modulus)
}
