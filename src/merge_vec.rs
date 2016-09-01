use std::slice;

/// The type of merge vectors.
#[derive(Clone,Debug,Eq,PartialEq)]
pub struct MergeVec<T> where T: Ord + Clone {
    status: Vec<Option<usize>>,
    sorted: Vec<T>,
    new_vals: Vec<T>
}

impl<T> MergeVec<T> where T: Ord + Clone{
    /// The length of the vector.
    pub fn len(&self) -> usize {
        self.sorted.len()
    }

    /// Append an element to the end of the vector.
    pub fn push(&mut self, value: T) {
        //println!("vec push index {:?}", self.len());
        let index = self.new_vals.len();
        self.sorted.push(value.clone()); // TODO: only need size change
        self.status.push(Some(index));
        self.new_vals.push(value);
    }

    
    /// Set the `i`th element of the vector.
    /// Panics if the vector contains fewer than `i` elements.
    pub fn set(&mut self, index: usize, value: T) {
        //println!("vec set index {:?}", index);
        let status = self.status[index];
        match  status {
            None => {
                let new_index = self.new_vals.len();
                self.new_vals.push(value);
                self.status[index] = Some(new_index);
            },
            Some(new_index) => {
                self.new_vals[new_index] = value;
            }
        }
    }

    
    /// Truncate this vector and reset the sort if necessary.
    pub fn truncate(&mut self, len: usize) {
        //println!("vec truncate to {:?}", len);
        if len >= self.sorted.len() { return; }
        if len <= 0 { *self = MergeVec::new(); return; }
        let old_new_length = self.new_vals.len();
        if old_new_length > 0 {
            let mut new_new_vals = Vec::with_capacity(old_new_length);
            let mut remaining = old_new_length;
            let mut index = 0;
            while remaining > 0 && index < len {
                let status = self.status[index];
                match status {
                    None => {},
                    Some(new_index) => {
                        self.status[index] = Some(new_new_vals.len());
                        new_new_vals.push(self.new_vals[new_index].clone());
                        remaining -= 1;
                    }
                }
                index += 1;
            }
            self.new_vals = new_new_vals;
        }
        self.status.truncate(len);
        self.sorted.truncate(len);
    }
    
    /// Create a new, empty vector.
    pub fn new() -> MergeVec<T> {
        MergeVec {
            status: Vec::new(),
            sorted: Vec::new(),
            new_vals: Vec::new(),
        }
    }

    /// Sort the vector, merging the new values in
    pub fn sort(&mut self) {
        // TODO: use iterators to simplify this code

        let mut new_sorted = Vec::with_capacity(self.sorted.len());
        self.new_vals.sort();
        // 2 degenerate cases
        if self.new_vals.len() == 0 {
            let mut s_index = 0;
            while s_index < self.sorted.len() {
                if self.status[s_index] == None{
                    new_sorted.push(self.sorted[s_index].clone())
                }
                s_index += 1;
            }
        } else if self.sorted.len() == 0 {
            new_sorted = self.new_vals.clone();
        } else {
            // general case
            let mut s_index = 0;
            let mut n_index = 0;
            while new_sorted.len() < self.sorted.len() {
                if self.status[s_index] != None {
                    // skip modified values
                    s_index += 1;
                    if s_index == self.sorted.len() {
                        // write the rest of the sorted values to the new vec
                        while n_index < self.new_vals.len() {
                            new_sorted.push(self.new_vals[n_index].clone());
                            n_index += 1;
                        }
                    }
                } else if self.new_vals[n_index] < self.sorted[s_index] {
                    // merge in the new value
                    new_sorted.push(self.new_vals[n_index].clone());
                    n_index += 1;
                    if n_index == self.new_vals.len() {
                        // write the rest of the old values to the new vec
                        while s_index < self.sorted.len() {
                            if self.status[s_index] == None{
                                new_sorted.push(self.sorted[s_index].clone())
                            }
                            s_index += 1;
                        }
                    }
                } else {
                    // merge in the sorted value
                    new_sorted.push(self.sorted[s_index].clone());
                    s_index += 1;
                    if s_index == self.sorted.len() {
                        // write the rest of the sorted values to the new vec
                        while n_index < self.new_vals.len() {
                            new_sorted.push(self.sorted[s_index].clone());
                            n_index += 1;
                        }
                    }
                }
            }
        }
        // reset with the new sorted values
        self.sorted = new_sorted;
        self.status = vec![None; self.sorted.len()];
        self.new_vals = Vec::new();
    }

    pub fn sorted_iter(&mut self) -> slice::Iter<T> {
        self.sort();
        self.sorted.iter()
    }

    /// Get the `i`th element of the vector.
    /// Returns `None` if the vector contains fewer than `i` elements.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.status.len() { return None; }
        let status = self.status[index];
        match status {
            None => { Some(&self.sorted[index]) },
            Some(new_index) => { Some(&self.new_vals[new_index]) }
        }
    }
}

impl<T> From<Vec<T>> for MergeVec<T> where T: Ord + Clone {
    fn from(vec: Vec<T>) -> MergeVec<T> {
        MergeVec {
            status: (0..vec.len()).map(|v| Some(v)).collect(),
            sorted: vec.clone(),
            new_vals: vec,
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
