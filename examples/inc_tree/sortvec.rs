use presort::{PresortedVec, PermutedVec};
use presort::permuted_vec::PermutedIter;
use presort::presorted_vec::PresortedIter;

pub trait SortVec<T: Ord> {
    /// The length of the vector.
    fn len(&self) -> usize;

    /// Append an element to the end of the vector.    
    fn push(&mut self, value: T);
    
    /// Set the `i`th element of the vector.
    /// Panics if the vector contains fewer than `i` elements.
    fn set(&mut self, index: usize, value: T);
    
    /// Truncate this vector and reset the sort if necessary.
    fn truncate(&mut self, len: usize);
    
    /// Sort the vector
    fn sort(&mut self);
}

pub trait IntoSortedIterator {
    type Item: Ord;
    type IntoSortedIter: Iterator<Item=Self::Item>;

    /// A sorted iterator over the vector.
    fn into_sorted_iter(self) -> Self::IntoSortedIter;
}

impl<T: Ord> SortVec<T> for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
    fn push(&mut self, val: T) {
        self.push(val);
    }
    fn set(&mut self, index: usize, val: T) {
        self[index] = val;
    }
    fn truncate(&mut self, size: usize) {
        self.truncate(size);
    }
    fn sort(&mut self) {
        (**self).sort();
    }
}

impl<T: Ord> SortVec<T> for PresortedVec<T> {
    fn len(&self) -> usize {
        self.len()
    }
    fn push(&mut self, val: T) {
        self.push(val);
    }
    fn set(&mut self, index: usize, val: T) {
        self.set(index, val);
    }
    fn truncate(&mut self, size: usize) {
        self.truncate(size);
    }
    fn sort(&mut self) {
        self.sort();
    }
}

impl<T: Ord> SortVec<T> for PermutedVec<T> {
    fn len(&self) -> usize {
        self.len()
    }
    fn push(&mut self, val: T) {
        self.push(val);
    }
    fn set(&mut self, index: usize, val: T) {
        self.set(index, val);
    }
    fn truncate(&mut self, size: usize) {
        self.truncate(size);
    }
    fn sort(&mut self) {
        self.sort_by(|a,b| { a.cmp(b) })
    }
}

impl<'a, T: Ord> IntoSortedIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoSortedIter = ::std::vec::IntoIter<&'a T>;

    fn into_sorted_iter(self) -> ::std::vec::IntoIter<&'a T>{
        let mut sorted: Vec<&T> = self.iter().collect();
        sorted.sort_by(|a,b| {(*a).cmp(*b)});
        sorted.into_iter()
    }
}

impl<'a, T: Ord> IntoSortedIterator for &'a mut PermutedVec<T> {
    type Item = &'a T;
    type IntoSortedIter = PermutedIter<'a, T>;

    fn into_sorted_iter(self) -> Self::IntoSortedIter {
        self.sorted_iter_by(|a,b|{a.cmp(b)})
    }
}

impl<'a, T: Ord> IntoSortedIterator for &'a mut PresortedVec<T> {
    type Item = &'a T;
    type IntoSortedIter = PresortedIter<'a, T>;

    fn into_sorted_iter(self) -> Self::IntoSortedIter {
        self.sorted_iter()
    }

}
