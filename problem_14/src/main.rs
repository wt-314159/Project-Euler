fn main() {
    let lengths = find_collatz_sequence_lengths(1_000_000);
    let max = lengths
        .iter()
        .enumerate()
        .max_by(|(_, n1), (_, n2)| n1.cmp(n2))
        .expect("Failed to find max");
    println!(
        "Max sequence length at: {} (sequence length: {})",
        max.0, max.1
    );
}

fn find_collatz_sequence_lengths(limit: usize) -> Vec<usize> {
    assert!(limit > 2);
    let mut lengths = vec![0; limit + 1];
    for i in 2..lengths.len() {
        // we've already found the sequence length of i
        if lengths[i] != 0 {
            continue;
        }
        let mut nums_in_sequence = Vec::new();
        let mut n = i;
        while n != 1 {
            nums_in_sequence.push(n);

            n = if n.is_multiple_of(2) {
                n / 2
            } else {
                3 * n + 1
            }
        }

        let mut len = 2;
        for j in nums_in_sequence.iter().rev() {
            if *j <= limit {
                lengths[*j] = len;
            }
            len += 1;
        }
    }
    lengths
}
