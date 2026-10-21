fn main() {
    let mut max_solutions = 0;
    let mut max_sol_num = 0;
    for perim in 3..=1000 {
        let solutions = find_solutions(perim).len();
        if solutions > max_solutions {
            max_solutions = solutions;
            max_sol_num = perim;
        }
    }

    println!("Perimeter {max_sol_num} has {max_solutions} solutions");
}

fn find_solutions(perimeter: usize) -> Vec<(usize, usize, usize)> {
    // a^2 + b^2 = c^2   a + b + c = p
    // Iterate through a and b, see if p - a - b = c matches pythagoras
    let mut solutions = Vec::new();
    for a in 1..=perimeter / 2 {
        for b in a..=perimeter / 2 {
            let c = perimeter - a - b;
            if c * c == a * a + b * b {
                solutions.push((a, b, c));
            }
        }
    }
    solutions
}
