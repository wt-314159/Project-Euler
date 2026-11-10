use std::time::Instant;

use common::Integer;

fn main() {
    let start = Instant::now();
    if let Some(hex) = find_matching_nums() {
        let elapsed = Instant::now().duration_since(start);
        println!("Matching number (hex) {hex} found in {elapsed:?}");
    }
}

fn find_matching_nums() -> Option<u64> {
    for n in 144.. {
        let hex = get_hex_num(n);
        if hex.is_pentagonal() && hex.is_triangular() {
            return Some(hex);
        }
    }
    None
}

fn get_tri_num(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn get_pent_num(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

fn get_hex_num(n: u64) -> u64 {
    n * (2 * n - 1)
}
