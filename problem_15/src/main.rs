fn main() {
    println!(
        "Number of routes through 20x20 grid: {}",
        count_lattice_options(20)
    );
}

fn count_lattice_options(size: u128) -> u128 {
    // number of ways to get to a point is sum of ways to get to 2 points
    // to left and above the current point, this gives binomial expansion
    // coefficients. So for square of size n, we need central binomial
    // coefficient of order 2n, which is 2n choose n

    // n choose k is given by n! / k!(n - k)!
    // but here, n is 2n, and k is n
    // so we have (2n)! / n! * n!
    // To avoid having to store huge numbers (for n = 20, we would have
    // 40!, which would overflow even u128) we can calculate just the first
    // half of the factorial, (i.e. from 2n down to n), as the second half
    // gets cancelled out anyway

    let mut total: u128 = 1;
    for i in size + 1..=2 * size {
        total = total.checked_mul(i).expect("Overflowed u128!");
    }
    // now to divide by n!
    // (if above didn't overflow, this shouldn't do)
    let mut f: u128 = 1;
    for i in 2..=size {
        f *= i;
    }
    total / f
}
