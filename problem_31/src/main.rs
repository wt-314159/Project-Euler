fn main() {
    let num_options = find_number_of_options();
    println!("There are {} ways of making £2.00", num_options);
}

fn find_number_of_options() -> usize {
    let coins = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let mut count = 0;
    find_number_of_options_rec(&coins, 200, &mut count);
    count
}

fn find_number_of_options_rec(coins: &[i32], remaining: i32, count: &mut usize) {
    for (i, coin) in coins.iter().enumerate() {
        let new_remaining = remaining - coin;
        // println!("coin: {coin}, remaining: {remaining}, new_remaining: {new_remaining}");
        if new_remaining == 0 {
            *count += 1;
        } else if new_remaining < 0 {
            continue;
        } else {
            find_number_of_options_rec(&coins[i..], new_remaining, count)
        }
    }
}
