use common::Integer;
use std::collections::HashSet;

fn main() {
    if let Some((big, little)) = find_pentagonal_numbers() {
        println!(
            "Found pentagonal numbers where {big} - {little} = {}",
            big - little
        );
    }
}

fn find_pentagonal_numbers() -> Option<(i32, i32)> {
    let mut numbers = HashSet::new();
    let mut sorted_numbers = Vec::new();
    for n in 1.. {
        let pent = n * (3 * n - 1) / 2;
        for other in sorted_numbers.iter().rev() {
            let diff = pent - *other;
            if numbers.contains(&diff) && is_pentagonal(pent + *other) {
                return Some((pent, *other));
            }
        }
        numbers.insert(pent);
        sorted_numbers.push(pent);
    }
    None
}

fn is_pentagonal(num: i32) -> bool {
    // use quadratic formula, first check
    // that the square root resolves to an integer
    // then check it's a multiple of 2a (which is 6)
    if let Ok(num) = u32::try_from(1 + 24 * num) {
        if !num.is_square() {
            false
        } else {
            (1 + num.isqrt()).is_multiple_of(6)
        }
    } else {
        false
    }
}
