use common::{Integer, IntegerSliceExt, combinatorics::PermutationsRevEx};

fn main() {
    let start = std::time::Instant::now();
    let mut largest_prime = 0;
    let mut digits = 987654321u32.digits_vec();
    while largest_prime == 0 {
        digits.try_for_each_permutation_rev(|d| {
            let num = d.from_digits();
            if num.is_prime() {
                largest_prime = num;
                println!("Found prime {num}");
                None
            } else {
                Some(())
            }
        });
        // Remove largest number and try again
        if largest_prime == 0 {
            digits.sort();
            digits.pop();
        }
    }
    let duration = std::time::Instant::now().duration_since(start);
    println!("found largest pandigital prime {largest_prime} - in {duration:?}");
}
