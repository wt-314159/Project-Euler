use crate::Integer;

pub trait Squares: Integer {
    fn is_square(&self) -> bool;
    /// Gets the integer square root of `self`,
    /// or `None` if self isn't a square number
    fn exact_sqrt(&self) -> Option<Self>;
    fn get_squares(max: Self) -> Vec<Self>;
    fn fill_squares(squares: &mut Vec<Self>, max: Self);
}

macro_rules! impl_squares {
    ($($t:ty),* $(,)?) => {
        $(
            impl Squares for $t {

                fn is_square(&self) -> bool {
                    let root = self.isqrt();
                    root * root == *self
                }

                fn exact_sqrt(&self) -> Option<Self> {
                    let root = self.isqrt();
                    if root * root == *self {
                        Some(root)
                    } else {
                        None
                    }
                }

                fn get_squares(max: Self) -> Vec<Self> {
                    (1..=max).map(|n| n * n).collect()
                }

                fn fill_squares(squares: &mut Vec<Self>, max: Self) {
                    // Start with the number after the square root of the last entry
                    // (or at 1, if there are no entries yet)
                    let start = squares.last().map(|s| s.isqrt() + 1).unwrap_or(1);
                    for n in start..=max {
                        squares.push(n * n)
                    }
                }
            }
        )*
    };
}

impl_squares!(u32, u64, usize);

pub fn find_pythagorean_triplet<F>(max: usize, predicate: F) -> Option<(usize, usize, usize)>
where
    F: Fn(usize, usize, usize) -> bool,
{
    let squares = (1..=max).map(|i| i * i).collect::<Vec<usize>>();
    let max = squares.last().unwrap();

    // Iterate through the vec and add every square to each other
    for (a, s_a) in squares.iter().enumerate() {
        for (b, s_b) in squares.iter().enumerate().skip(a) {
            let (a, b) = (a + 1, b + 1);
            if a + b >= *max {
                // a + b + c can't equal max
                break;
            }
            let sum = s_a + s_b;
            // Check if the sum is too big first of all
            if sum > *max {
                break;
            }
            // Check if sum is contained in list, if so, it's a pythagorean triplet
            if squares.contains(&sum) {
                // Now we just need to determine if a + b + c = 1000
                let c = sum.isqrt();
                if predicate(a, b, c) {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}
