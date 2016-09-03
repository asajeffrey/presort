
/// The type of merge vectors.
#[derive(Clone,Debug)]
pub struct MergeVec<T> where T: Ord {
    content: Vec<T>,                // the content changed only by user
    sort_index: Vec<usize>,         // where the sort reference is stored
    sorted: Vec<SortTarget>,        //  holes if changed
    unsorted: Vec<Option<usize>>    // new, unsorted content
}

#[derive(Clone,Copy,Debug)]
enum SortTarget {
    Content(usize),
    Unsorted(usize),
    Removed,
}

struct ContentIter<'a> {
    index: usize,
    vec: &'a Vec<SortTarget>,
}

impl<'a> Iterator for ContentIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        while self.index < self.vec.len() {
            let i = self.index;
            self.index += 1;
            if let SortTarget::Content(c) = self.vec[i] {
                return Some(c);
            }
        }
        return None
    }
}

struct OpContentIter<'a> {
    index: usize,
    vec: &'a Vec<Option<usize>>,
}

impl<'a> Iterator for OpContentIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        while self.index < self.vec.len() {
            let i = self.index;
            self.index += 1;
            if let Some(c) = self.vec[i] {
                return Some(c);
            }
        }
        return None
    }
}

#[derive(Clone,Debug)]
pub struct MergeVecIter<'a, T> where T: 'a {
    index: usize,
    content: &'a[T],
    sort_order: &'a[SortTarget],
}

impl<'a, T> Iterator for MergeVecIter<'a, T> where T: 'a {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let sorted_index = self.index;
        self.index += 1;
        self.sort_order.get(sorted_index).and_then(|&target| {
            match target {
                SortTarget::Content(i) => self.content.get(i),
                _ => None
            }  
        })
    }
}

impl<T> MergeVec<T> where T: Ord {
    /// The length of the vector.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Append an element to the end of the content.
    pub fn push(&mut self, value: T) {
        //println!("vec push index {:?}", self.len());

        let content_index = self.content.len();
        if value >= self.content[content_index] {
            // in order
            let sort_index = self.sorted.len();
            self.sort_index.push(sort_index);  
            self.sorted.push(SortTarget::Content(content_index));
        } else {
            // out of order
            let new_index = self.unsorted.len();
            let sort_index = self.sorted.len();
            self.sort_index.push(sort_index);  
            self.sorted.push(SortTarget::Unsorted(new_index));
            self.unsorted.push(Some(content_index));
        }
        self.content.push(value);
    }

    
    /// Set the `i`th element of the vector.
    /// Panics if the vector contains fewer than `i` elements.
    pub fn set(&mut self, index: usize, value: T) {
        //println!("vec set index {:?}", index);
        let sort_index = self.sort_index[index];
        let old_loc = self.sorted[sort_index];
        match old_loc {
            SortTarget::Content(_) => {
                if !self.would_be_sorted(sort_index, &value) {
                    let new_index = self.unsorted.len();
                    self.sorted[sort_index] = SortTarget::Unsorted(new_index);
                    self.unsorted.push(Some(index));
                }
            }
            SortTarget::Unsorted(unsort_index) => {
                if self.would_be_sorted(sort_index, &value) {
                    self.unsorted[unsort_index] = None;
                    self.sorted[sort_index] = SortTarget::Content(index);
                }
            }
            SortTarget::Removed => {
                if self.would_be_sorted(sort_index, &value) {
                    self.sorted[sort_index] = SortTarget::Content(index);
                } else {
                    let new_index = self.unsorted.len();
                    self.sorted[sort_index] = SortTarget::Unsorted(new_index);
                    self.unsorted.push(Some(index));
                }
            }
        }
        self.content[index] = value;
    }

    fn would_be_sorted(&self, sort_index: usize, value: &T) -> bool {
        if sort_index > 0 {
            if let SortTarget::Content(content_index) = self.sorted[sort_index - 1] {
                if self.content[content_index] > *value { return false; }
            } else {
                // TODO: Should we search through unsorted/removed values?
                return false;
            }
        }
        if sort_index < self.sorted.len() - 1 {
            if let SortTarget::Content(content_index) = self.sorted[sort_index + 1] {
                if self.content[content_index] < *value { return false; }
            } else {
                // TODO: Should we search through unsorted/removed values?
                return false;
            }
        }
        return true;
    }

    
    /// Truncate this vector.
    pub fn truncate(&mut self, len: usize) {
        //println!("vec truncate to {:?}", len);

        // remove all linked values
        for index in len..self.content.len() {
            let sort_index = self.sort_index[index];
            match self.sorted[sort_index] {
                SortTarget::Content(_) => self.sorted[sort_index] = SortTarget::Removed,
                SortTarget::Unsorted(unsort_index) => {
                    self.unsorted[unsort_index] = None;
                    self.sorted[sort_index] = SortTarget::Removed;
                },
                SortTarget::Removed => {}
            }
        }
        //truncate data
        self.content.truncate(len);
        self.sort_index.truncate(len);
    }
    
    /// Create a new, empty vector.
    pub fn new() -> MergeVec<T> {
        MergeVec {
            content: Vec::new(),
            sort_index: Vec::new(),
            sorted: Vec::new(),
            unsorted: Vec::new(),
        }
    }

    /// Consolidate incremental data, in preparation of producing a sorted iterator
    pub fn sort(&mut self) {
        self.unsorted.sort();
        let mut new_sort = Vec::with_capacity(self.content.len());
        {
            let mut a_iter = ContentIter { index: 0, vec: &self.sorted};
            let mut b_iter = OpContentIter { index: 0, vec: &self.unsorted};
            let mut op_a = a_iter.next();
            let mut op_b = b_iter.next();
            // merge
            loop {
                if let Some(index_a) = op_a {
                if let Some(index_b) = op_b {
                    if self.content[index_a] <= self.content[index_b] {
                        self.sort_index[index_a] = new_sort.len();
                        new_sort.push(SortTarget::Content(index_a));
                        op_a = a_iter.next();
                    } else {
                        self.sort_index[index_b] = new_sort.len();
                        new_sort.push(SortTarget::Content(index_b));
                        op_b = b_iter.next();
                    }
                } else { break; }
                } else { break; }
            }
            // add data from longer iter
            while let Some(index_a) = op_a {
                self.sort_index[index_a] = new_sort.len();
                new_sort.push(SortTarget::Content(index_a));
                op_a = a_iter.next();
            }
            while let Some(index_b) = op_b {
                self.sort_index[index_b] = new_sort.len();
                new_sort.push(SortTarget::Content(index_b));
                op_b = b_iter.next();
            }
        }
        //finalize vecs
        self.sorted = new_sort;
        self.unsorted = Vec::new();
    }

    pub fn sorted_iter(&mut self) -> MergeVecIter<T> {
        self.sort();
        MergeVecIter {
            index: 0,
            content: &self.content,
            sort_order: &self.sorted,
        }
    }

    /// Get the `i`th element of the vector.
    /// Returns `None` if the vector contains fewer than `i` elements.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.content.get(index)
    }
}

impl<T> From<Vec<T>> for MergeVec<T> where T: Ord + Clone {
    fn from(vec: Vec<T>) -> MergeVec<T> {
        let length = vec.len();
        MergeVec {
            content: vec,
            sort_index: (0..length).collect(),
            sorted: (0..length).map(|i| SortTarget::Unsorted(i)).collect(),
            unsorted: (0..length).map(|i| Some(i)).collect(),
        }
    }
}

#[test]
fn test_push() {
    let mut vec = MergeVec::new();
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.get(0), None);

    vec.push(0);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), None);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0]);

    vec.push(30);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), None);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &30]);

    vec.push(20);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), None);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &20, &30]);

}

#[test]
fn test_set() {
    let mut vec = MergeVec::from(vec![0, 30, 20, 10]);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&30));
    assert_eq!(vec.get(2), Some(&20));
    assert_eq!(vec.get(3), Some(&10));
    assert_eq!(vec.get(4), None);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&0, &10, &20, &30]);

}
