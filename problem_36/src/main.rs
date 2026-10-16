use common::{digits, digits_big_endian, get_bits};

fn main() {
    let palindromes = find_double_palindromes(1_000_000).collect::<Vec<_>>();
    println!("{palindromes:?}");
    println!(
        "Sum of all double palindromes: {}",
        palindromes.iter().sum::<u32>()
    );
}

fn find_double_palindromes(max: u32) -> impl Iterator<Item = u32> {
    // palindromic cannot have leading zero in either base,
    // which means it cannot have trailing zero in either base,
    // which means it must be odd (last binary digit must be 1)
    (1..=max)
        .step_by(2)
        .filter(|n| is_palindrome(*n) && is_binary_palindrome(*n))
}

fn is_palindrome(num: u32) -> bool {
    digits(num).zip(digits_big_endian(num)).all(|(a, b)| a == b)
}

fn is_binary_palindrome(num: u32) -> bool {
    get_bits(num)
        .skip_while(|b| *b == 0)
        .zip(get_bits(num).rev())
        .all(|(a, b)| a == b)
}
