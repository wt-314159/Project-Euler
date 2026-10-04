use common::find_abundant_numbers;

fn main() {
    // smallest abundant number is 12, so no point finding
    // numbers greater than limit - 12
    let limit = 28123;
    let abundant_nums = find_abundant_numbers(limit - 12);
    let non_sums = find_non_sum_nums(limit, &abundant_nums);
    let sum: usize = non_sums.iter().sum();
    println!("Sum of all numbers that are not the sum of 2 abundant numbers: {sum}");
}

fn find_non_sum_nums(limit: usize, abundant_nums: &[usize]) -> Vec<usize> {
    let mut is_sum = vec![false; limit + 1];
    for (i, a) in abundant_nums.iter().enumerate() {
        for b in abundant_nums.iter().skip(i) {
            let sum = a + b;
            if sum <= limit {
                is_sum[sum] = true;
            }
        }
    }

    let mut non_sums = Vec::new();
    for (i, is_sum) in is_sum.iter().enumerate().skip(1) {
        if !is_sum {
            non_sums.push(i);
        }
    }
    non_sums
}
