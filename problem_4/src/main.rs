fn main() {
    let mut largest_palindrome = 0;
    let (mut largest_i, mut largest_j) = (0, 0);
    for i in (500..=999).rev() {
        for j in (500..=i).rev() {
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
    let mut num_2 = num;
    let mut rev = 0;
    while num_2 > 0 {
        rev = 10 * rev + num_2 % 10;
        num_2 /= 10;
    }
    rev == num
}
