use common::fractions::find_divisors;

fn main() {
    let limit = 10_000;
    let proper_divisor_sums: Vec<_> = (0..=limit)
        .map(|i| find_divisors(i).sum::<usize>())
        .collect();

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
