fn main() {
    let limit = 10_000;
    let mut proper_divisor_sums = vec![0; limit + 1];
    for i in 1..=limit {
        let divisors = find_divisors(i);
        let sum = divisors.iter().sum();
        proper_divisor_sums[i] = sum;
    }

    // find amicable numbers
    let mut sum = 0;
    for i in 1..=limit {
        let sum_at_i = proper_divisor_sums[i];
        if let Some(candiate_amicable) = proper_divisor_sums.get(sum_at_i)
            && *candiate_amicable == i
            && i != sum_at_i
        {
            println!("Found amicable numbers: {i}, {sum_at_i}");
            sum += sum_at_i;
            // Other amicable number will be added later
        }
    }

    println!("Sum of amicable numbers: {sum}");
}

fn find_divisors(num: usize) -> Vec<usize> {
    let mut divisors = vec![1]; // 1 divides all numbers
    let limit = num.isqrt(); // Only need to check up to square root
    for i in 2..limit {
        if num.is_multiple_of(i) {
            let other_divisor = num / i;
            divisors.push(i);
            divisors.push(other_divisor);
        }
    }
    // check if square number, if so, add square root
    if num.is_multiple_of(limit) {
        divisors.push(limit);
    }
    divisors
}
