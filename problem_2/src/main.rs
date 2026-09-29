fn main() {
    let mut sum: u64 = 0;
    let mut a = 0;
    let mut b = 1;
    while b <= 4_000_000 {
        if b % 2 == 0 {
            sum += b;
        }
        let temp = b;
        b += a;
        a = temp;
    }
    println!("{sum}");
}
