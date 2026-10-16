use std::ops::ControlFlow;

use crate::combinatorics::Fallible;

#[derive(Debug)]
pub struct Rotations<'a, T> {
    items: &'a mut [T],
    rotation: usize,
}

impl<'a, T> Iterator for Rotations<'a, T> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.rotation == 0 {
            // Don't rotate items, current items will be first iteration
        } else if self.rotation >= self.items.len() {
            return None;
        } else {
            self.items.rotate_left(1);
        }
        self.rotation += 1;
        Some(())
    }
}

impl<'a, T> Rotations<'a, T> {
    pub fn new(items: &'a mut [T]) -> Self {
        Rotations { items, rotation: 0 }
    }

    pub fn items(&self) -> &[T] {
        self.items
    }
}

pub trait RotationsEx<T> {
    fn rotate(&mut self) -> Rotations<'_, T>;

    fn for_each_rotation(&mut self, mut f: impl FnMut(&[T])) {
        let mut rotate = self.rotate();
        while rotate.next().is_some() {
            f(rotate.items);
        }
    }

    fn try_for_each_rotation<F, R>(&mut self, mut f: F) -> bool
    where
        F: FnMut(&[T]) -> R,
        R: Fallible<Output = ()>,
    {
        let mut rotate = self.rotate();
        while rotate.next().is_some() {
            match f(rotate.items).branch() {
                ControlFlow::Continue(_) => (),
                ControlFlow::Break(_) => return false,
            }
        }
        true
    }

    fn rotate_fold<B, F>(&mut self, init: B, mut f: F) -> B
    where
        F: FnMut(B, &[T]) -> B,
    {
        let mut rotate = self.rotate();
        let mut result = init;
        while rotate.next().is_some() {
            result = f(result, rotate.items);
        }
        result
    }

    /// Rotates items to the kth rotation in place.
    fn nth_rotation(&mut self, k: usize) {
        // nth would return (k - 1)th rotation, so use take(k).count()
        self.rotate().take(k).count();
    }
}

impl<T> RotationsEx<T> for [T] {
    fn rotate(&mut self) -> Rotations<'_, T> {
        Rotations::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotations_test() {
        let mut expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut test = expected.clone();
        test.for_each_rotation(|rotate| {
            assert_eq!(&expected, rotate);
            expected.rotate_left(1);
        })
    }

    #[test]
    fn nth_rotation_test() {
        let mut expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 1..=10 {
            let mut test = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            test.nth_rotation(i);
            assert_eq!(
                expected, test,
                "testing rotations match after {i} rotations"
            );
            expected.rotate_left(1);
        }
    }

    #[test]
    fn first_rotation_test() {
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut test = expected.clone();
        test.nth_rotation(1);
        assert_eq!(expected, test);
    }

    #[test]
    fn tenth_rotation_test() {
        let mut test = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        test.nth_rotation(10);
        assert_eq!(vec![9, 0, 1, 2, 3, 4, 5, 6, 7, 8], test);
    }

    #[test]
    fn rotation_order() {
        let mut test = vec![1, 3];
        test.for_each_rotation(|rotate| {
            println!("{rotate:?}");
        });
        assert!(false);
    }
}
