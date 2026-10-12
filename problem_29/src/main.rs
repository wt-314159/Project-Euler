use num_bigint::BigUint;
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    for i in 2u32..=100u32 {
        for j in 2..=100 {
            let result = BigUint::from(i).pow(j);
            set.insert(result);
        }
    }
    println!("Number of unique entries: {}", set.len());
}
