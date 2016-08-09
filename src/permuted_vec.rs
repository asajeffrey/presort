use std::cmp::Ordering;
use std::slice::Iter;
use sortvec::{SortVec,IntoSortedIterator};

/// The type of permuted vectors.
#[derive(Clone,Debug,Eq,PartialEq)]
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

impl<T: Ord> SortVec<T> for PermutedVec<T> {
    /// The length of the vector.
    fn len(&self) -> usize {
        self.contents.len()
    }

    /// Append an element to the end of the vector.
    fn push(&mut self, value: T) {
        //println!("vec push index {:?}", self.len());
        let index = self.contents.len();
        self.contents.push(value);
        self.permutation.push(index);
    }

    
    /// Set the `i`th element of the vector.
    /// Panics if the vector contains fewer than `i` elements.
    fn set(&mut self, index: usize, value: T) {
        //println!("vec set index {:?}", index);
        self.contents[index] = value;
    }

    
    /// Truncate this vector and reset the sort if necessary.
    fn truncate(&mut self, len: usize) {
        //println!("vec truncate to {:?}", len);
        if len < self.len() {
            self.contents.truncate(len);
            self.permutation.clear();
            self.permutation.extend(0..len);
        }
    }
    
    /// Sort the permutation on the vector
    fn sort(&mut self) {
        self.sort_by(|a,b| { a.cmp(b) })
    }
}

impl<'a, T: Ord> IntoSortedIterator for &'a mut PermutedVec<T> {
    type Item = &'a T;
    type IntoSortedIter = PermutedIter<'a, T>;

    fn into_sorted_iter(self) -> Self::IntoSortedIter {
        self.sorted_iter_by(|a,b|{a.cmp(b)})
    }
}

impl<T> PermutedVec<T> {
    /// Create a new, empty presorted vector.
    pub fn new() -> PermutedVec<T> {
        PermutedVec {
            contents: Vec::new(),
            permutation: Vec::new(),
        }
    }

    /// An iterator over the permutation
    pub fn permutation_iter(&self) -> Iter<usize> {
        self.permutation.iter()
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
    pub fn is_sorted_by<F>(&self, f: &mut F) -> bool where F: FnMut(&T, &T) -> Ordering {
        let iter_1 = self.permuted_iter();
        let mut iter_2 = self.permuted_iter();
        iter_2.next();
        iter_1.zip(iter_2).all(|(value_1, value_2)| f(value_1, value_2) != Ordering::Greater)
    }

    /// Sort the permutation on the vector
    pub fn sort_by<F>(&mut self, mut f: F) where F: FnMut(&T, &T) -> Ordering {
        if !self.is_sorted_by(&mut f) {
            let contents = &self.contents;
            self.permutation.sort_by(|&index_1, &index_2|
                match f(&contents[index_1], &contents[index_2]) {
                    Ordering::Equal => index_1.cmp(&index_2),
                    ord => ord,
                }
            );
            debug_assert!(self.is_sorted_by(&mut f));
        }
    }

    /// A sorted iterator over the vector.
    pub fn sorted_iter_by<F>(&mut self, f: F) -> PermutedIter<T> where F: FnMut(&T, &T) -> Ordering {
        self.sort_by(f);
        self.permuted_iter()
    }

    /// Get the `i`th element of the vector.
    /// Returns `None` if the vector contains fewer than `i` elements.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.contents.get(index)
    }

    /// Get the `i`th element of the permuted vector.
    /// Returns `None` if the vector contains fewer than `i` elements.
    pub fn get_permuted(&self, permuted: usize) -> Option<&T> {
        self.permutation.get(permuted).and_then(|&index| self.contents.get(index))
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


#[cfg(feature = "serde")]
impl<T> serde::Serialize for PermutedVec<T>
    where T: serde::Serialize
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        (&self.contents, &self.permutation).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Deserialize for PermutedVec<T>
    where T: serde::Deserialize
{
    fn deserialize<D>(deserializer: &mut D) -> Result<PermutedVec<T>, D::Error>
        where D: serde::Deserializer
    {
        let (contents, permutation) = try!(serde::Deserialize::deserialize(deserializer));
        Ok(PermutedVec { contents: contents, permutation: permutation })
    }
}

#[cfg(feature = "heapsize")]
impl<T> heapsize::HeapSizeOf for PermutedVec<T>
    where T: heapsize::HeapSizeOf
{
    fn heap_size_of_children(&self) -> usize {
        self.contents.heap_size_of_children() + self.permutation.heap_size_of_children()
    }
}

#[test]
fn test_push() {
    let mut vec = PermutedVec::new();
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.get(0), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), true);

    vec.push(0);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), true);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0]);

    vec.push(30);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), true);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0, &30]);

    vec.push(20);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), false);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0, &20, &30]);

    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), true);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0, &20, &30]);

    vec.push(10);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), false);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0, &10, &20, &30]);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), true);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0, &10, &20, &30]);
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
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), false);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0, &10, &20, &30]);

    vec.set(2, 21);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&21));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), true);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0, &10, &21, &30]);

    vec.set(2, 31);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&31));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), false);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0, &10, &30, &31]);

    vec.set(2, 1);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&1));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.is_sorted_by(&mut usize::cmp), false);
    assert_eq!(vec.sorted_iter_by(usize::cmp).collect::<Vec<&usize>>(), vec![&0, &1, &10, &30]);
}

#[cfg(feature = "serde_json")]
#[test]
fn test_serialize() {
    extern crate serde_json;
    
    let original = PermutedVec::from(vec![0,3,2,1]);
    let serialized = serde_json::to_string(&original).unwrap();
    let roundtrip: PermutedVec<usize> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(original, roundtrip);
}

