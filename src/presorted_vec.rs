use permuted_vec::{PermutedIter, PermutedVec};

/// The type of presorted vectors.
#[derive(Clone,Debug,Eq,PartialEq)]
pub struct PresortedVec<T> where T: Ord {
    // The contents of the vector.
    contents: PermutedVec<T>,
    // The inverse permutation
    inverse: Vec<usize>,
    // Is the permiuted vector sorted?
    is_sorted: bool,
}

/// The type of presorted iterators over a presorted vector.
#[derive(Clone,Debug)]
pub struct PresortedIter<'a, T> where T: Ord+'a {
    // The underlying iterator
    contents: PermutedIter<'a, T>,
}

impl<'a, T> Iterator for PresortedIter<'a, T> where T: 'a+Ord {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.contents.next()
    }
}

impl<T> PresortedVec<T> where T:Ord {
    /// Create a new, empty presorted vector.
    pub fn new() -> PresortedVec<T> {
        PresortedVec {
            contents: PermutedVec::new(),
            inverse: Vec::new(),
            is_sorted: true,
        }
    }

    /// An iterator over the presorted vector
    pub fn presorted_iter(&self) -> PresortedIter<T> {
        PresortedIter {
            contents: self.contents.permuted_iter(),
        }
    }

    /// Is the presorted vector already sorted?
    pub fn is_sorted(&self) -> bool {
        self.is_sorted || self.contents.is_sorted_by(&mut |value_1, value_2| value_1.cmp(value_2))
    }

    /// Sort the permutation on the vector
    pub fn sort(&mut self) {
        if !self.is_sorted {
            self.contents.sort_by(|value_1, value_2| value_1.cmp(value_2));
            for (i, &j) in self.contents.permutation_iter().enumerate() {
                self.inverse[j] = i;
            }
            self.is_sorted = true;
        }
    }

    /// A sorted iterator over the vector.
    pub fn sorted_iter(&mut self) -> PresortedIter<T> {
        self.sort();
        self.presorted_iter()
    }

    /// Get the `i`th element of the vector.
    /// Returns `None` if the vector contains fewer than `i` elements.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.contents.get(index)
    }

    /// Get the `i`th element of the permuted vector.
    /// Returns `None` if the vector contains fewer than `i` elements.
    pub fn get_permuted(&self, permuted: usize) -> Option<&T> {
        self.contents.get_permuted(permuted)
    }

    /// Set the `i`th element of the vector.
    /// Panics if the vector contains fewer than `i` elements.
    pub fn set(&mut self, index: usize, value: T) {
        let permuted = self.inverse[index];
        self.is_sorted =
            self.is_sorted &&
            self.contents.get_permuted(permuted.wrapping_sub(1)).map(|before| before <= &value).unwrap_or(true) &&
            self.contents.get_permuted(permuted.wrapping_add(1)).map(|after| &value <= after).unwrap_or(true);
        self.contents.set(index, value);
    }

    /// Append an element to the end of the vector.
    pub fn push(&mut self, value: T) {
        //println!("vec push index {:?}", self.len());
        let permuted = self.contents.len();
        self.is_sorted =
            self.is_sorted &&
            self.contents.get_permuted(permuted.wrapping_sub(1)).map(|before| before <= &value).unwrap_or(true);
        self.contents.push(value);
        self.inverse.push(permuted);
    }

    /// Truncate this vector and reset the sort if necessary.
    pub fn truncate(&mut self, len: usize) {
        //println!("vec truncate to {:?}", len);
        if len < self.len() {
            self.contents.truncate(len);
            self.inverse.clear();
            self.inverse.extend(0..len);
            self.is_sorted = false;
        }
    }

    /// The length of the vector.
    pub fn len(&self) -> usize {
        self.contents.len()
    }
}

impl<T> From<Vec<T>> for PresortedVec<T> where T: Ord {
    fn from(vec: Vec<T>) -> PresortedVec<T> {
        let len = vec.len();
        PresortedVec {
            contents: PermutedVec::from(vec),
            inverse: (0..len).collect(),
            is_sorted: false,
        }
    }
}


#[cfg(feature = "serde")]
impl<T> serde::Serialize for PresortedVec<T>
    where T: serde::Serialize+Ord
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        (&self.contents, &self.inverse, &self.is_sorted).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Deserialize for PresortedVec<T>
    where T: serde::Deserialize+Ord
{
    fn deserialize<D>(deserializer: &mut D) -> Result<PresortedVec<T>, D::Error>
        where D: serde::Deserializer
    {
        let (contents, inverse, is_sorted) = try!(serde::Deserialize::deserialize(deserializer));
        Ok(PresortedVec { contents: contents, inverse: inverse, is_sorted: is_sorted })
    }
}

#[cfg(feature = "heapsize")]
impl<T> heapsize::HeapSizeOf for PresortedVec<T>
    where T: heapsize::HeapSizeOf
{
    fn heap_size_of_children(&self) -> usize {
        self.contents.heap_size_of_children() + self.inverse.heap_size_of_children()
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

#[cfg(feature = "serde_json")]
#[test]
fn test_serialize() {
    extern crate serde_json;
    
    let original = PresortedVec::from(vec![0,3,2,1]);
    let serialized = serde_json::to_string(&original).unwrap();
    let roundtrip: PresortedVec<usize> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(original, roundtrip);
}

