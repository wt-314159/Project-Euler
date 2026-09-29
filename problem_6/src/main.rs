fn main() {
    let num = 100;
    let sum_squares: i32 = calc_sum_of_squares(num);
    // Sum of all numbers up to n is simply n(n + 1) / 2
    let sum = num * (num + 1) / 2;
    let square_sum = sum * sum;
    println!(
        "Difference is {square_sum} - {sum_squares} = {}",
        square_sum - sum_squares
    );
}

/// For low `n`, we can just brute force this (i.e. iterate
/// from `1` to `n` and sum the square of these numbers)
/// However for very large n this could be quite slow,
/// ideally we want a formula to calculate this value
/// directly from `n`.
///
/// Assume the formula has the form `f(n) = an^3 + bn^2 + cn + d`
/// We know `f(0) = 0 and f(1) = 1`, and can easily calculate
/// `f(2) = 5` and `f(3) = 14` by hand (simply summing the squares)
///
/// So we have:
/// ```
/// f(0) =   0 +  0 +  0 + d = 0    ->  d = 0
/// f(1) =   a +  b +  c = 1        ->  c = 1 - a - b
/// f(2) =  8a + 4b + 2c = 5        ->  b = 3/2 - 3a
/// f(3) = 27a + 9b + 3c = 14       ->  a = 1/3
/// ```
///
/// N.B. Substituting `d = 0` into `f(1)`, we get `c` in terms of `a` and `b`
/// Then substituting `c = 1 -a -b` and `d = 0` into `f(2)`, we get `b` in terms of `a`
/// Finally, subbing in the expressions for `b`, `c`, and `d` into `f(3)`, we get `a`
///
/// `b = 1/2`
/// `c = 1/6`
///
/// Subbing in `a` into the expresion for `b`, we can get `b`
/// Then subbing both into the expression for `c`, we get `c`
///
/// Finally, we have `f(n) = 1/3 * n^3 + 1/2 * n^2 + n/6`
///
/// We then need to prove this holds generally, by induction
/// If this is true for `f(n)`, then `f(n+1)` should equal `f(n) + (n+1)^2`
///
/// ```
/// f(n+1) = 1/3 * (n+1)^3 + 1/2 * (n+1)^2 + (n+1)/6
///        = (n^3 + 3n^2 + 3n + 1)/3 + (n^2 + 2n + 1)/2 + (n+1)/6
///        = 1/3 * n^3 + 3/2 * n^2 + 13n/6 + 1
///
/// f(n) + (n+1)^2 = 1/3 * n^3 + 1/2 * n^2 + n/6 + n^2 + 2n + 1
///                = 1/3 * n^3 + 3/2 * n^2 + 13n/6 + 1
/// ```
///
/// So `f(n+1)` does indeed equal `f(n) + (n+1)^2`, proof by induction.
fn calc_sum_of_squares(n: i32) -> i32 {
    let n = n as f64;
    (n.powi(3) / 3.0 + n.powi(2) / 2.0 + n / 6.0) as i32
}
