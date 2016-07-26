//! A crate for presorted vectors.
//!
//! Presorted vectors provide similar functionality to vectors, but are designed
//! to make sorting cheap in the case that mutating the vector preserves sort
//! order.
//!
//! Define an update establishes sorting as:
//!
//! * `vec.presort()` establishes sorting.
//!
//! * `vec.sorted_iter()` establishes sorting.
//!
//! Define an update preserves sorting as:
//!
//! * `vec.set(i,v)` preserves sorting whenever, for any `j != i`,
//!   we have `vec[i] < vec[j]` if any only if `v < vec[j]`.
//!
//! * `vec.push(v)` preserves sorting whenever, for any `i`,
//!   we have `vec[i] < v`.
//!
//! A presorted vector is definitely sorted when its update history contains
//! an update which establishes sorting, and every subsequent update
//! preserves sorting.
//!
//! For definitely sorted vectors, `vec.presort()` and `vec.sorted_iter()` are constant time functions.

/// The type of presorted vectors.
#[derive(Clone,Debug)]
pub struct PresortedVec<T> {
    // The contents of the vector, possibly permuted.
    contents: Vec<T>,
    // If this is true, contents is sorted.
    is_sorted: bool,
    // A permutation `p`, such that `vec[i]` is the same as `vec[p[i]]`.
    permute_original_to_contents: Vec<usize>,
    // The inverse permutation.
    permute_contents_to_original: Vec<usize>,
}

impl<T> PresortedVec<T>
where T: Ord {
    /// Create a new, empty presorted vector.
    pub fn new() -> PresortedVec<T> {
        PresortedVec {
            contents: Vec::new(),
            is_sorted: true,
            permute_original_to_contents: Vec::new(),
            permute_contents_to_original: Vec::new(),
        }
    }

    /// The invariant maintained by the datatype
    fn invariant(&self) -> bool {
        if self.is_sorted {
            let iter_1 = self.contents.iter();
            let mut iter_2 = self.contents.iter();
            iter_2.next();
            for (value_1, value_2) in iter_1.zip(iter_2) {
                if value_1 > value_2 { return false; }
            }
        }
        for (contents_index, &original_index) in self.permute_contents_to_original.iter().enumerate() {
            if self.permute_original_to_contents[original_index] != contents_index { return false; }
        }
        return true;
    }

    /// Make the vector definitely sorted.
    /// If the vector is already definitely sorted, this is a constant time operation.
    pub fn presort(&mut self) {
        if !self.is_sorted {
            let mut new_permute_contents_to_original = self.permute_contents_to_original.clone();
            new_permute_contents_to_original.sort_by(
                |&original_index_1, &original_index_2|
                    self.contents[self.permute_original_to_contents[original_index_1]]
                        .cmp(&self.contents[self.permute_original_to_contents[original_index_2]])
            );
            for (new_contents_index, &original_index) in new_permute_contents_to_original.iter().enumerate() {
                let old_contents_index = self.permute_original_to_contents[original_index];
                self.permute_original_to_contents.swap(self.permute_contents_to_original[new_contents_index], self.permute_contents_to_original[old_contents_index]);
                self.permute_contents_to_original.swap(new_contents_index, old_contents_index);
                self.contents.swap(new_contents_index, old_contents_index);
            }
            self.is_sorted = true;
        }
        debug_assert!(self.invariant());
    }

    /// A sorted iterator over the vector.
    /// Makes the vector definitely sorted.
    /// If the vector is already definitely sorted, this is a constant time operation.
    pub fn sorted_iter(&mut self) -> std::slice::Iter<T> {
        self.presort();
        self.contents.iter()
    }

    /// Check to see if the vector is definitely sorted.
    pub fn is_presorted(&self) -> bool {
        self.is_sorted
    }

    /// Get the `i`th element of the vector.
    /// Returns `None` if the vector contains fewer than `i` elements.
    pub fn get(&self, original_index: usize) -> Option<&T> {
        self.permute_original_to_contents
            .get(original_index)
            .and_then(|&contents_index| self.contents.get(contents_index))
    }

    /// Set the `i`th element of the vector.
    /// Panics if the vector contains fewer than `i` elements.
    pub fn set(&mut self, original_index: usize, value: T) {
        let contents_index = self.permute_original_to_contents[original_index];
        self.is_sorted =
            self.is_sorted &&
            self.contents.get(contents_index.wrapping_sub(1)).map(|before| before <= &value).unwrap_or(true) &&
            self.contents.get(contents_index.wrapping_add(1)).map(|after| &value <= after).unwrap_or(true);
        self.contents[contents_index] = value;
        debug_assert!(self.invariant());
    }

    /// Append an element to the end of the vector.
    pub fn push(&mut self, value: T) {
        let original_index = self.contents.len();
        let contents_index = original_index;
        self.is_sorted =
            self.is_sorted &&
            self.contents.get(contents_index.wrapping_sub(1)).map(|before| before <= &value).unwrap_or(true);
        self.contents.push(value);
        self.permute_original_to_contents.push(contents_index);
        self.permute_contents_to_original.push(original_index);
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
            is_sorted: false,
            permute_original_to_contents: (0..len).collect(),
            permute_contents_to_original: (0..len).collect(),
        }
    }
}

#[test]
fn test_push() {
    let mut vec = PresortedVec::new();
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.get(0), None);
    assert_eq!(vec.is_presorted(), true);

    vec.push(0);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), None);
    assert_eq!(vec.is_presorted(), true);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0]);

    vec.push(30);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), None);
    assert_eq!(vec.is_presorted(), true);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &30]);

    vec.push(20);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), None);
    assert_eq!(vec.is_presorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &20, &30]);

    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), None);
    assert_eq!(vec.is_presorted(), true);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &20, &30]);

    vec.push(10);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_presorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &20, &30]);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_presorted(), true);
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
    assert_eq!(vec.is_presorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &20, &30]);

    vec.set(2, 21);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&21));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_presorted(), true);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &21, &30]);

    vec.set(2, 31);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&31));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_presorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &30, &31]);

    vec.set(2, 1);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&1));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_presorted(), false);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &1, &10, &30]);
}
