use common::squares::find_pythagorean_triplet;

fn main() {
    let target = 1000;
    if let Some((a, b, c)) = find_pythagorean_triplet(target, |a, b, c| a + b + c == target) {
        println!("{a} + {b} + {c} = 1000");
        println!("Product is {}", a * b * c);
    } else {
        println!("No pythagorean triplet found where a + b + c = 1000!");
    }
}
