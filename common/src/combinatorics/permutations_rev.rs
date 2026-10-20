use std::ops::ControlFlow;

use crate::combinatorics::Fallible;

#[derive(Debug)]
pub struct PermutationsRev<'a, T>
where
    T: Ord,
{
    items: &'a mut [T],
    started: bool,
}

impl<'a, T> Iterator for PermutationsRev<'a, T>
where
    T: Ord,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.items.sort_by(|a, b| b.cmp(a)); // reverse sort
            self.started = true;
            return Some(());
        }
        self.items.next_permutation().then_some(())
    }
}

impl<'a, T> PermutationsRev<'a, T>
where
    T: Ord,
{
    pub fn new(items: &'a mut [T]) -> Self {
        PermutationsRev {
            items,
            started: false,
        }
    }

    pub fn items(&self) -> &[T] {
        self.items
    }
}

pub trait PermutationsRevEx<T>
where
    T: Ord,
{
    fn permutate_rev(&mut self) -> PermutationsRev<'_, T>;

    fn for_each_permutation_rev(&mut self, mut f: impl FnMut(&[T])) {
        let mut perms = self.permutate_rev();
        while perms.next().is_some() {
            f(perms.items());
        }
    }

    fn try_for_each_permutation_rev<F, R>(&mut self, mut f: F) -> bool
    where
        F: FnMut(&[T]) -> R,
        R: Fallible<Output = ()>,
    {
        let mut perms = self.permutate_rev();
        while perms.next().is_some() {
            match f(perms.items).branch() {
                ControlFlow::Continue(_) => (),
                ControlFlow::Break(_) => return false,
            }
        }
        true
    }

    /// Steps to the next lexically ordered permutation in place.
    /// Returns false (leaving the items unchanged) if already at the last permutation.
    fn next_permutation(&mut self) -> bool;

    fn nth_permutation(&mut self, k: usize) {
        // nth would return (k - 1)th permutation, so use take(k).count()
        self.permutate_rev().take(k).count();
    }
}

impl<T> PermutationsRevEx<T> for [T]
where
    T: Ord,
{
    fn permutate_rev(&mut self) -> PermutationsRev<'_, T> {
        PermutationsRev::new(self)
    }

    fn next_permutation(&mut self) -> bool {
        // find the rightmost item that is larger than the item after it
        // (everything after it is in ascending order, i.e. fully permutated);
        // if there isn't one, the whole slice is ascending (the last permutation)
        let Some(pivot) = self.windows(2).rposition(|w| w[0] > w[1]) else {
            return false;
        };
        // find the largest item after the pivot that is smaller than it
        // (as the tail is ascending, this is the rightmost smaller item)
        let successor = pivot
            + 1
            + self[pivot + 1..]
                .iter()
                .rposition(|item| *item < self[pivot])
                .expect("Item after pivot must be smaller than pivot");
        // bring it to the pivot position, then put the (still ascending) tail
        // into descendng order, giving the highest ordering of the tail
        self.swap(pivot, successor);
        self[pivot + 1..].reverse();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_each_visits_all_in_order() {
        let mut seen = Vec::new();
        [0, 1, 2].for_each_permutation_rev(|p| seen.push(p.to_vec()));
        assert_eq!(
            seen,
            vec![
                vec![2, 1, 0],
                vec![2, 0, 1],
                vec![1, 2, 0],
                vec![1, 0, 2],
                vec![0, 2, 1],
                vec![0, 1, 2],
            ]
        );
    }
}

// 3, 2, 1, 0
// 3, 2, 0, 1
// 3, 1, 2, 0
// 3, 1, 0, 2
// 3, 0, 2, 1
// 3, 0, 1, 2
//
