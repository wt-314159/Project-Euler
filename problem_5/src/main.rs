fn main() {
    for i in 2520..usize::MAX {
        if is_evenly_divisible_by_all(i, 20) {
            println!("smallest product {i}");
            return;
        }
    }
    println!("No smallest product found!");
}

fn is_evenly_divisible_by_all(num: usize, up_to: usize) -> bool {
    // Only have to check the upper half of the divisors
    // (if num is divisible by 18, say, it's also divisible by 9)
    let half = up_to / 2;
    for i in half..=up_to {
        if num % i != 0 {
            return false;
        }
    }
    true
}
