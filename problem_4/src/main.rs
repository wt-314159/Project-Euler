fn main() {
    let mut largest_palindrome = 0;
    let (mut largest_i, mut largest_j) = (0, 0);
    for i in (500..=999).rev() {
        for j in (500..=999).rev() {
            let product = i * j;
            if product > largest_palindrome && is_palindrome(product) {
                largest_palindrome = product;
                largest_i = i;
                largest_j = j;
            }
        }
    }
    println!("Largest palindrome: {largest_i} * {largest_j} = {largest_palindrome}");
}

fn is_palindrome(num: usize) -> bool {
    // Probably a faster way of checking is a number is palindromic
    // than converting to a string and back, but this is simplest
    // and more than fast enough for us.
    let reversed: String = num.to_string().chars().rev().collect();
    let rev_num = reversed
        .parse::<usize>()
        .expect("Failed to parse reversed number");
    rev_num == num
}
