use std::time::Instant;

use common::Integer;

fn main() {
    let start = Instant::now();
    if let Some((big, little)) = find_pentagonal_numbers(10_000_000) {
        let elapsed = Instant::now().duration_since(start);
        println!(
            "Found pentagonal numbers where {big} - {little} = {} in {elapsed:?}",
            big - little
        );
    }
}

fn find_pentagonal_numbers(limit: u32) -> Option<(u32, u32)> {
    let limit = get_pentagonal_root(limit);
    let numbers: Vec<_> = (1..=limit).map(|n| get_pentagonal(n)).collect();

    for (idx, i) in numbers.iter().enumerate() {
        for j in numbers.iter().skip(idx + 1) {
            if (j - i).is_pentagonal() && (j + i).is_pentagonal() {
                return Some((*j, *i));
            }
        }
    }
    None
}

fn get_pentagonal_root(p: u32) -> u32 {
    (1 + (1 + 24 * p).isqrt()) / 6
}

fn get_pentagonal(n: u32) -> u32 {
    n * (3 * n - 1) / 2
}
