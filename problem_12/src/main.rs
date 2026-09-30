fn main() {
    let answer = find_triangle_number_with_divisors(500);
    println!("Answer: {answer}");
}

fn find_triangle_number_with_divisors(num_divisors: u32) -> u32 {
    assert!(num_divisors >= 2);
    let mut triangle_number: u32 = 1;
    let mut to_add = 2;
    loop {
        let mut divisors = 2;
        let sqrt = triangle_number.isqrt();
        for i in 2..sqrt {
            if triangle_number.is_multiple_of(i) {
                // there are 2 divisors, i and the number we multipy it by
                divisors += 2;
            }
        }
        // square root only counts as one divisor
        if sqrt * sqrt == triangle_number {
            divisors += 1;
        }
        if divisors >= num_divisors {
            break triangle_number;
        }

        triangle_number += to_add;
        to_add += 1;
    }
}
