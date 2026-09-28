fn main() {
    let mut sum: u64 = 0;
    for i in (1..=595_000).step_by(2) {
        sum += i * i;
    }
    println!("{}", sum);
}
