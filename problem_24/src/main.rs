use common::combinatorics::PermutationsEx;

fn main() {
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let limit = 1_000_000;
    digits.nth_permutation(limit);
    println!("{limit}th permutation of digits is: {digits:?}");
    // let num_perms = count_permutations(&mut vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 1_000_000);
    // println!("Number of permutations for 10 digits: {num_perms}");
}
