fn main() {
    let longest = find_longest_collatz_sequence(1_000_000);
    println!(
        "max sequence lengths at {} (sequence length: {})",
        longest.0, longest.1
    );
}

fn find_longest_collatz_sequence(limit: usize) -> (usize, usize) {
    assert!(limit > 2);
    // Have a longer vec than we need, as many chains will go above
    // the limit after starting below it, so we can cache these values
    let mut lengths = vec![0; limit * 10];
    // Important to set value of 1 to 1, otherwise we would recurse endlessly
    lengths[1] = 1;
    let mut longest_chain = 0;
    let mut answer = 0;
    // For any n, 2n is a longer sequence, so none of
    // the integers less than limit / 2 can be longest
    let start = limit / 2;
    for i in start..=limit {
        let len = get_collatz_length(&mut lengths, i);
        if len > longest_chain {
            longest_chain = len;
            answer = i;
        }
    }
    (answer, longest_chain)
}

fn get_collatz_length(lengths: &mut Vec<usize>, num: usize) -> usize {
    let mut value = lengths.get(num).copied().unwrap_or(0usize);
    if value != 0 {
        return value;
    }
    if num.is_multiple_of(2) {
        value = 1 + get_collatz_length(lengths, num / 2);
    } else {
        value = 2 + get_collatz_length(lengths, (3 * num).div_ceil(2));
    }
    if num < lengths.len() {
        lengths[num] = value;
    }
    value
}
