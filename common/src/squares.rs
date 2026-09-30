pub fn find_pythagorean_triplet<F>(max: usize, predicate: F) -> Option<(usize, usize, usize)>
where
    F: Fn(usize, usize, usize) -> bool,
{
    let squares = (1..=max).map(|i| i * i).collect::<Vec<usize>>();
    let max = squares.last().unwrap();

    // Iterate through the vec and add every square to each other
    for (a, s_a) in squares.iter().enumerate() {
        for (b, s_b) in squares.iter().enumerate().skip(a) {
            let (a, b) = (a + 1, b + 1);
            if a + b >= *max {
                // a + b + c can't equal max
                break;
            }
            let sum = s_a + s_b;
            // Check if the sum is too big first of all
            if sum > *max {
                break;
            }
            // Check if sum is contained in list, if so, it's a pythagorean triplet
            if squares.contains(&sum) {
                // Now we just need to determine if a + b + c = 1000
                let c = sum.isqrt() as usize;
                if predicate(a, b, c) {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}

pub fn find_pythagorean_triplet_fast<F>(max: usize, predicate: F) -> Option<(usize, usize, usize)>
where
    F: Fn(usize, usize, usize) -> bool,
{
    None
}
