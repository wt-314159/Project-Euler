use problem_7::find_nth_prime;

fn main() {
    let target = 10_001;
    let prime = find_nth_prime(target);
    println!("10,001st prime is {prime}");
}
