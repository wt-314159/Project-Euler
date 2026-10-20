use std::collections::HashSet;

use common::{Integer, unordered_eq};
use itertools::Itertools;

fn main() {
    let all_products = find_pandigital_products();
    // Use a HashSet to avoid duplicate products
    let set: HashSet<u32> = HashSet::from_iter(all_products.iter().map(|p| p.2));
    let sum: u32 = set.iter().sum();
    println!("Found all products, sum is {sum}");
}

fn find_pandigital_products() -> Vec<(u32, u32, u32)> {
    let mut products = Vec::new();
    let digits = vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in 1..=3 {
        // Get every permutation of i digits as our first number
        for multiplicand in digits.iter().permutations(i) {
            // Combine the digits into an actual number and remove used digits
            let num = from_digits(&multiplicand);
            let remaining: Vec<_> = digits
                .iter()
                .filter(|d| !multiplicand.contains(d))
                .collect();

            // Must be at least 5 total digits in multiplier and multiplicand,
            // with 4 total digits we can't make a 5 digit product and use all 9 digits.
            // Can't be more than 5 digits, since with 6 total digits, we can't
            // make a 3 digit product. So must be exactly 5 digits total.
            let multiplier_digit_count = 5 - i;
            for multiplier in remaining.iter().permutations(multiplier_digit_count) {
                let num2 = multiplier.iter().fold(0, |acc, d| acc * 10 + **d);
                let product = num * num2;
                // product must have 4 digits, maximum number is therefor less than 9876
                if product > 9876 {
                    break;
                }

                let mut remaining: Vec<_> = remaining
                    .iter()
                    .filter(|d| !multiplier.contains(d))
                    .copied()
                    .copied()
                    .collect();
                let mut digits: Vec<_> = product.digits_rev().collect();
                if unordered_eq(&mut remaining, &mut digits) {
                    products.push((num, num2, product));
                }
            }
        }
    }
    products
}

fn from_digits(digits: &[&u32]) -> u32 {
    digits.iter().fold(0, |acc, d| acc * 10 + **d)
}
