fn main() {
    let num = 100;
    let sum_squares: i32 = (1..=num).into_iter().map(|i| i * i).sum();
    println!("Sum of squares: {sum_squares}");
    let sum = (num + 1) * num / 2;
    let square_sum = sum * sum;
    println!("Square of sum: {square_sum}");
    println!(
        "Difference is {square_sum} - {sum_squares} = {}",
        square_sum - sum_squares
    );
}
