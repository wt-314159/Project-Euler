use std::collections::HashMap;

use common::unordered_eq;
use itertools::Itertools;

fn main() {
    let power = 5;
    let digit_powers = get_digit_powers(power);
    let max_digits = get_max_digits(power);
    println!("Max number of digits possible: {max_digits}");
    let numbers = find_numbers(digit_powers, max_digits);
    println!(
        "Numbers are: {numbers:?}, sum is {}",
        numbers.iter().sum::<u32>()
    );
}

fn get_digit_powers(power: u32) -> HashMap<u32, u32> {
    HashMap::from_iter((0u32..=9).map(|d| (d, d.pow(power))))
}

fn get_max_digits(power: u32) -> u32 {
    // Find the maximum number of digits the number can be
    let mut n = 0;
    loop {
        let len = (n * 9u32.pow(power)).to_string().len();
        if len as u32 > n { n += 1 } else { break n }
    }
}

fn find_numbers(map: HashMap<u32, u32>, max_digits: u32) -> Vec<u32> {
    let mut nums = Vec::new();
    for i in 1usize..=max_digits as usize {
        for mut x in (0u32..10).combinations_with_replacement(i) {
            let sum: u32 = x.iter().map(|d| map.get(d).unwrap()).sum();
            let mut digits: Vec<_> = digits(sum).collect();
            if unordered_eq(&mut x, &mut digits) {
                nums.push(sum);
            }
        }
    }
    nums
}

fn digits(mut n: u32) -> impl Iterator<Item = u32> {
    std::iter::from_fn(move || {
        if n > 0 {
            let digit = n % 10;
            n /= 10;
            Some(digit)
        } else {
            None
        }
    })
}
