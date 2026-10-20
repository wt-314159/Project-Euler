use common::{Integer, IntegerSliceExt, combinatorics::PermutationsRevEx};

fn main() {
    if let Some(large) = find_largest_pandigital_multiple() {
        println!("Largest pandigital multiple: {large}");
    } else {
        println!("None found!");
    }
}

fn find_largest_pandigital_multiple() -> Option<u32> {
    let mut digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut answer = None;
    digits.try_for_each_permutation_rev(|d| {
        if is_pandigital_multiple(d) {
            answer = Some(d.from_digits());
            None
        } else {
            Some(())
        }
    });
    answer
}

fn is_pandigital_multiple(digits: &[u32]) -> bool {
    // take the first n digits of the number (for n from 1 to 4)
    for i in 1..=4 {
        // Concatenate those digits into a number
        let num = digits[0..i].from_digits();
        let mut idx = i;
        // Then progressively multiply it by 2, 3, 4... and match
        // the results against the remaining digits
        // If all the remaining digits can be matched in this way,
        // return true, otherwise, go to next outer loop and try again
        for j in 2.. {
            let next = num * j;
            let next_digits: Vec<_> = next.digits_vec();
            if digits[idx..].starts_with(&next_digits) {
                if next_digits.len() == digits[idx..].len() {
                    return true;
                }
                idx += next_digits.len();
                continue;
            } else {
                break;
            }
        }
    }
    false
}
