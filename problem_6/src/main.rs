use problem_6::calc_sum_of_squares;

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
