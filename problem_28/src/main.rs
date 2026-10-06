fn main() {
    let count = count_spiral_diagnoals(1001);
    println!("1001x1001 grid diagonals sum to {count}");
}

fn count_spiral_diagnoals(size: usize) -> usize {
    let mut curr_size = 3;
    let mut count = 1;
    let mut num = 1;
    while curr_size <= size {
        let incr = curr_size - 1;
        for _ in 0..4 {
            num += incr;
            count += num;
        }
        curr_size += 2;
    }
    count
}
