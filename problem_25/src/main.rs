use common::BigFibonacci;
use num_bigint::BigUint;

fn main() {
    let mut fib = BigFibonacci::new();
    let num_digits = 1000;
    let limit = BigUint::from(10u32).pow(num_digits - 1);
    let mut count = 0;
    while let Some(n) = fib.next() {
        count += 1;
        if n >= limit {
            println!("{count}th index ({n}) has more than {num_digits} digits");
            break;
        }
    }
}
