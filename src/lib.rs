//! A crate for presorted vectors.
//!
//! Presorted vectors provide similar functionality to vectors, but are designed
//! to make sorting cheap in the case that mutating the vector preserves sort
//! order.

/// The type of presorted vectors.
#[derive(Clone,Debug)]
pub struct PresortedVec<T> {
    // The contents of the vector.
    contents: Vec<T>,
    // A permutation `p`, such that `vec.get(p[i])` is the same as `vec.get_sorted(i)`.
    permute_sorted_to_contents: Vec<usize>,
    // The inverse permutation.
    permute_contents_to_sorted: Vec<usize>,
}

/// The type of sorted iterators over a presorted vector.
#[derive(Clone,Debug)]
pub struct SortedIter<'a, T> where T: 'a {
    // Where are we in the iterator
    index: usize,
    // The contents of the iterator
    contents: &'a[T],
    // A permutation giving the sorting
    permute_sorted_to_contents: &'a[usize],
}

impl<'a, T> Iterator for SortedIter<'a, T> where T: 'a {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let sorted_index = self.index;
        self.index = self.index + 1;
        self.permute_sorted_to_contents.get(sorted_index).and_then(|&index| self.contents.get(index))
    }
}

impl<T> PresortedVec<T> where T: Ord {
    /// Create a new, empty presorted vector.
    pub fn new() -> PresortedVec<T> {
        PresortedVec {
            contents: Vec::new(),
            permute_sorted_to_contents: Vec::new(),
            permute_contents_to_sorted: Vec::new(),
        }
    }

    fn to_sorted_iter(&self) -> SortedIter<T> {
        SortedIter {
            index: 0,
            contents: &self.contents,
            permute_sorted_to_contents: &self.permute_sorted_to_contents,
        }
    }

    /// Is the vector already sorted?
    pub fn is_sorted(&self) -> bool {
        let iter_1 = self.to_sorted_iter();
        let mut iter_2 = self.to_sorted_iter();
        iter_2.next();
        iter_1.zip(iter_2).all(|(value_1, value_2)|value_1 <= value_2)
    }

    /// The invariant maintained by the datatype
    fn invariant(&self) -> bool {
        self.permute_contents_to_sorted.iter().enumerate().all(|(index, &sorted_index)|
            self.permute_sorted_to_contents[sorted_index] == index
        ) && self.permute_sorted_to_contents.iter().enumerate().all(|(sorted_index, &index)|
            self.permute_contents_to_sorted[index] == sorted_index
        )
    }
    
    /// Sort the vector
    pub fn presort(&mut self) {
        if !self.is_sorted() {
            let contents = &self.contents;
            self.permute_sorted_to_contents.sort_by(|&index_1, &index_2| contents[index_1].cmp(&contents[index_2]));
            for (sorted_index, &index) in self.permute_sorted_to_contents.iter().enumerate() {
                self.permute_contents_to_sorted[index] = sorted_index;
            }
            debug_assert!(self.invariant());
            debug_assert!(self.is_sorted());
        }
    }

    /// A sorted iterator over the vector.
    /// Makes the vector definitely sorted.
    /// If the vector is already definitely sorted, this is a constant time operation.
    pub fn sorted_iter(&mut self) -> SortedIter<T> {
        self.presort();
        self.to_sorted_iter()
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
        self.permute_sorted_to_contents.push(index);
        self.permute_contents_to_sorted.push(index);
        debug_assert!(self.invariant());
    }

    /// The length of the vector.
    pub fn len(&self) -> usize {
        self.contents.len()
    }
}

impl<T> From<Vec<T>> for PresortedVec<T> {
    fn from(vec: Vec<T>) -> PresortedVec<T> {
        let len = vec.len();
        PresortedVec {
            contents: vec,
            permute_sorted_to_contents: (0..len).collect(),
            permute_contents_to_sorted: (0..len).collect(),
        }
    }
}

#[test]
fn test_push() {
    let mut vec = PresortedVec::new();
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
    let mut vec = PresortedVec::from(vec![0, 30, 20, 10]);
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
