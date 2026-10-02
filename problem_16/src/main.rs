use num_bigint::BigUint;

fn main() {
    let mut bits = [0; 1001];
    bits[0] = 1;
    let num = BigUint::from_radix_be(&bits, 2).expect("Failed to create big int.");
    println!("{num}");
    println!("sum of digits: {}", add_digits(num));
}

fn add_digits(big_num: BigUint) -> usize {
    big_num.to_radix_be(10).iter().map(|d| *d as usize).sum()
}
