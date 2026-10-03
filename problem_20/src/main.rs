use num_bigint::BigUint;

fn main() {
    let factorial = factorial_brute_force(100);
    let sum: u128 = factorial.to_radix_be(10).iter().map(|i| *i as u128).sum();
    println!("Sum of digits: {sum}");
}

fn factorial_brute_force(factorial: usize) -> BigUint {
    assert!(factorial > 1 && factorial < 1000);
    let mut big_int = BigUint::from(1u32);
    for i in 2..=factorial {
        big_int *= i;
    }
    big_int
}
