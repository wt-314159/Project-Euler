pub fn find_permutations_recursive(digts: &mut [u32], limit: usize) {
    find_permutations_recursive_impl(digts, &mut 1, limit);
}

fn find_permutations_recursive_impl(digits: &mut [u32], count: &mut usize, limit: usize) {
    if *count == limit {
        return;
    }
    if digits.len() == 2 {
        digits.swap(0, 1);
        *count += 1;
        return;
    }
    // clone current state, so we can undo after recursion
    let clone = digits.to_vec();
    find_permutations_recursive_impl(
        digits
            .get_mut(1..digits.len())
            .expect("Failed to slice digits"),
        count,
        limit,
    );
    if *count == limit {
        return;
    }
    for i in 1..digits.len() {
        // undo changes just done by recursion
        digits
            .iter_mut()
            .enumerate()
            .for_each(|(i, d)| *d = clone[i]);
        // bring the ith digit to the "front" of current slice
        for j in (1..=i).rev() {
            digits.swap(j, j - 1);
        }
        // increment the count, then recurse again
        *count += 1;
        find_permutations_recursive_impl(
            digits
                .get_mut(1..digits.len())
                .expect("Failed to slice digits"),
            count,
            limit,
        );
        if *count == limit {
            return;
        }
    }
}

#[derive(Debug)]
pub struct Permutations<'a, T>
where
    T: Ord,
{
    items: &'a mut [T],
    started: bool,
}

impl<'a, T> Iterator for Permutations<'a, T>
where
    T: Ord,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(());
        }
        self.items.next_permutation().then_some(())
    }
}

impl<'a, T> Permutations<'a, T>
where
    T: Ord,
{
    pub fn new(items: &'a mut [T]) -> Self {
        items.sort();
        Permutations {
            items,
            started: false,
        }
    }

    pub fn items(&self) -> &[T] {
        self.items
    }
}

pub trait PermutationsEx<T>
where
    T: Ord,
{
    fn permutate(&mut self) -> Permutations<'_, T>;

    fn for_each_permutation(&mut self, mut f: impl FnMut(&[T])) {
        let mut perms = self.permutate();
        while perms.next().is_some() {
            f(perms.items());
        }
    }

    /// Steps to the next lexically ordered permutation in place.
    /// Returns false (leaving the items unchanged) if already at the last permutation.
    fn next_permutation(&mut self) -> bool;

    fn nth_permutation(&mut self, k: usize) {
        // nth would return (k - 1)th permutation, so use take(k).count()
        self.permutate().take(k).count();
    }
}

impl<T> PermutationsEx<T> for [T]
where
    T: Ord,
{
    fn permutate(&mut self) -> Permutations<'_, T> {
        Permutations::new(self)
    }

    fn next_permutation(&mut self) -> bool {
        // find the rightmost item that is smaller than the item after it
        // (everything after it is in descending order, i.e. fully permutated);
        // if there isn't one, the whole slice is descending (the last permutation)
        let Some(pivot) = self.windows(2).rposition(|w| w[0] < w[1]) else {
            return false;
        };
        // find the smallest item after the pivot that is larger than it
        // (as the tail is descending, this is the rightmost larger item)
        let successor = pivot
            + 1
            + self[pivot + 1..]
                .iter()
                .rposition(|item| *item > self[pivot])
                .expect("Item after pivot must be larger than pivot");
        // bring it to the pivot position, then put the (still descending) tail
        // into ascening order, giving the lowest ordering of the tail
        self.swap(pivot, successor);
        self[pivot + 1..].reverse();
        true
    }
}

pub fn count_permutations(digits: &[u32]) -> usize {
    let mut count = 1;
    for idx in 0..digits.len() - 1 {
        count *= digits.len() - idx;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_matches_recursive() {
        for limit in 1..=120 {
            let mut expected = vec![0, 1, 2, 3, 4];
            find_permutations_recursive(&mut expected, limit);
            let mut items = vec![0, 1, 2, 3, 4];
            items.permutate().take(limit).count();
            assert_eq!(items, expected, "permutation {limit}");
        }
    }

    #[test]
    fn nth_permutation_matches_recursive() {
        for limit in 1..=120 {
            let mut expected = vec![0, 1, 2, 3, 4];
            find_permutations_recursive(&mut expected, limit);
            let mut items = vec![0, 1, 2, 3, 4];
            items.nth_permutation(limit);
            assert_eq!(items, expected, "permutation {limit}");
        }
    }

    #[test]
    fn for_each_visits_all_in_order() {
        let mut seen = Vec::new();
        [2, 1, 0].for_each_permutation(|p| seen.push(p.to_vec()));
        assert_eq!(
            seen,
            vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
            ]
        );
    }
}
