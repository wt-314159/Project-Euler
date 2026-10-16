use common::{digits, fractions::simplify_fraction};

fn main() {
    let fracts = find_cancelling_fracts();
    println!("Found {} fractions that match:", fracts.len());
    println!("{:?}", fracts);
    let num_prod: u32 = fracts.iter().map(|(n, _)| n).product();
    let denom_prod: u32 = fracts.iter().map(|(_, d)| d).product();
    let simplified_fract = simplify_fraction(num_prod, denom_prod);
    println!(
        "Product (simplified) is {}/{}",
        simplified_fract.0, simplified_fract.1
    );
}

fn find_cancelling_fracts() -> Vec<(u32, u32)> {
    let mut fracts = Vec::new();
    for num in 10..=98 {
        let num_digits: Vec<_> = digits(num).collect();
        for denom in (num + 1)..=99 {
            // First check if num and denom share a digit, if not, we can't cancel it
            let denom_digits: Vec<_> = digits(denom).collect();
            let common_digits: Vec<_> = denom_digits
                .iter()
                .filter(|x| num_digits.contains(x))
                .copied()
                .collect();
            // Only interested if there's exactly one common digit
            if common_digits.len() != 1 {
                continue;
            }
            let common_digit = common_digits[0];
            // ignore the trivial cases
            if common_digit == 0 {
                continue;
            }
            let num_r = get_other_digit(num_digits.iter(), common_digit);
            let denom_r = get_other_digit(denom_digits.iter(), common_digit);

            // can't have numerator or denominator being 0
            if num_r == 0 || denom_r == 0 {
                continue;
            }

            // Check if both fractions (before and after cancelling common digit)
            // simplify down to the same thing. if so, we have a match
            let (simple_num, simple_denom) = simplify_fraction(num, denom);
            let (simple_num_r, simple_denom_r) = simplify_fraction(num_r, denom_r);

            if simple_num == simple_num_r && simple_denom == simple_denom_r {
                fracts.push((num, denom));
            }
        }
    }
    fracts
}

fn get_other_digit<'a>(digits: impl Iterator<Item = &'a u32>, common_digit: u32) -> u32 {
    let mut skipped = false;
    for d in digits {
        if !skipped && *d == common_digit {
            skipped = true;
        } else {
            return *d;
        }
    }
    panic!("No other digits found");
}
