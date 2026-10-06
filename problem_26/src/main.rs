use std::collections::HashMap;

fn main() {
    let mut max_length = 0;
    let mut num = 0;
    for i in 2..=1000 {
        if let Some(cycle_len) = get_fraction_cycle_length(i)
            && cycle_len > max_length
        {
            max_length = cycle_len;
            num = i;
        }
    }
    println!("Number {num} has a cycle length of {max_length}");
}

fn get_fraction_cycle_length(denom: usize) -> Option<usize> {
    let mut count = 0;
    let mut remain: usize = 1;
    let mut map = HashMap::new();
    map.insert(1, 0);
    loop {
        count += 1;
        remain *= 10;
        // remainder is fully divisible by denominator, no cycle
        if remain.is_multiple_of(denom) {
            return None;
        }
        let digit = remain / denom;
        remain -= digit * denom;
        if map.contains_key(&remain) {
            return Some(count - map[&remain]);
        } else {
            map.insert(remain, count);
        }
    }
}
