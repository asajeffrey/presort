//! A crate for permuted vectors.
//!
//! Permuted vectors consist of a vector, together with a permutation of its elements.
//! In particular, `vec.sort()` sorts the permutation, not the vector.
//! This allows the vector to be updated, and
//! if the updates preserve sort order, then the next `vec.sort()`
//! will be O(n) rather than O(n log n).

use std::cmp::Ordering;

/// The type of permuted vectors.
#[derive(Clone,Debug)]
pub struct PermutedVec<T> {
    // The contents of the vector.
    contents: Vec<T>,
    // The permutation
    permutation: Vec<usize>,
}

/// The type of permuted iterators over a permuted vector.
#[derive(Clone,Debug)]
pub struct PermutedIter<'a, T> where T: 'a {
    // Where are we in the iterator
    index: usize,
    // The contents of the iterator
    contents: &'a[T],
    // The permutation
    permutation: &'a[usize],
}

impl<'a, T> Iterator for PermutedIter<'a, T> where T: 'a {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let sorted_index = self.index;
        self.index = self.index + 1;
        self.permutation.get(sorted_index).and_then(|&index| self.contents.get(index))
    }
}

impl<T> PermutedVec<T> where T: Ord {
    /// Create a new, empty presorted vector.
    pub fn new() -> PermutedVec<T> {
        PermutedVec {
            contents: Vec::new(),
            permutation: Vec::new(),
        }
    }

    /// An iterator over the permuted vector
    pub fn permuted_iter(&self) -> PermutedIter<T> {
        PermutedIter {
            index: 0,
            contents: &self.contents,
            permutation: &self.permutation,
        }
    }

    /// Is the permuted vector already sorted?
    pub fn is_sorted(&self) -> bool {
        let iter_1 = self.permuted_iter();
        let mut iter_2 = self.permuted_iter();
        iter_2.next();
        iter_1.zip(iter_2).all(|(value_1, value_2)|value_1 <= value_2)
    }

    /// Sort the permutation on the vector
    pub fn sort(&mut self) {
        if !self.is_sorted() {
            let contents = &self.contents;
            self.permutation.sort_by(|&index_1, &index_2|
                match contents[index_1].cmp(&contents[index_2]) {
                    Ordering::Equal => index_1.cmp(&index_2),
                    ord => ord,
                }
            );
            debug_assert!(self.is_sorted());
        }
    }

    /// A sorted iterator over the vector.
    /// If the vector is already definitely sorted, this is a constant time operation.
    pub fn sorted_iter(&mut self) -> PermutedIter<T> {
        self.sort();
        self.permuted_iter()
    }

    /// Get the `i`th element of the vector.
    /// Returns `None` if the vector contains fewer than `i` elements.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.contents.get(index)
    }

    /// Set the `i`th element of the vector.
    /// Panics if the vector contains fewer than `i` elements.
    pub fn set(&mut self, index: usize, value: T) {
        self.contents[index] = value;
    }

    /// Append an element to the end of the vector.
    pub fn push(&mut self, value: T) {
        let index = self.contents.len();
        self.contents.push(value);
        self.permutation.push(index);
    }

    /// The length of the vector.
    pub fn len(&self) -> usize {
        self.contents.len()
    }
}

impl<T> From<Vec<T>> for PermutedVec<T> {
    fn from(vec: Vec<T>) -> PermutedVec<T> {
        let len = vec.len();
        PermutedVec {
            contents: vec,
            permutation: (0..len).collect(),
        }
    }
}

#[test]
fn test_push() {
    let mut vec = PermutedVec::new();
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.get(0), None);
    assert_eq!(vec.is_sorted(), true);

    vec.push(0);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), None);
    assert_eq!(vec.is_sorted(), true);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0]);

    vec.push(30);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), None);
    assert_eq!(vec.is_sorted(), true);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &30]);

    vec.push(20);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), None);
    assert_eq!(vec.is_sorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &20, &30]);

    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), None);
    assert_eq!(vec.is_sorted(), true);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &20, &30]);

    vec.push(10);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &20, &30]);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted(), true);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &20, &30]);
}

#[test]
fn test_set() {
    let mut vec = PermutedVec::from(vec![0, 30, 20, 10]);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &20, &30]);

    vec.set(2, 21);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&21));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted(), true);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &21, &30]);

    vec.set(2, 31);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&31));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &30, &31]);

    vec.set(2, 1);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&1));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &1, &10, &30]);
}
