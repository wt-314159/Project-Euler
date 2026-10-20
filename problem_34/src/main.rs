use common::Integer;

fn main() {
    println!("Max number of digits: {}", get_max_digits());
    let digit_facts: Vec<_> = find_digit_factorials().collect();
    for digit_fact in digit_facts.iter() {
        println!("{digit_fact} is a digit factorial");
    }
    let sum: u32 = digit_facts.iter().sum();
    println!("Sum of digit factorials is {sum}");
}

fn get_max_digits() -> u32 {
    let mut n: u64 = 0;
    loop {
        let len = (n * 9u64.checked_factorial().unwrap())
            .to_string()
            .chars()
            .count();
        if len as u64 > n {
            n += 1
        } else {
            break n as u32;
        }
    }
}

fn find_digit_factorials() -> impl Iterator<Item = u32> {
    let max_digits = get_max_digits();
    let max_num = 10u32.pow(max_digits);
    let digit_facts: [u32; 10] = core::array::from_fn(|i| i.checked_factorial().unwrap() as u32);
    (11..max_num).filter(move |n| is_sum_of_digit_factorials(*n, &digit_facts))
}

fn is_sum_of_digit_factorials(num: u32, digit_facts: &[u32; 10]) -> bool {
    num.digits_rev().map(|d| digit_facts[d as usize]).sum::<u32>() == num
}
