#[derive(Clone,Debug)]
pub struct PresortedVec<T> {
    unsorted: Vec<T>,
    sorted: Vec<T>,
    permutation: Vec<usize>,
    is_sorted: bool,
}

impl<T> PresortedVec<T>
where T: Ord + Clone {
    pub fn new() -> PresortedVec<T> {
        PresortedVec {
            unsorted: Vec::new(),
            sorted: Vec::new(),
            permutation: Vec::new(),
            is_sorted: true,
        }
    }
    pub fn sorted(&mut self) -> &[T] {
        if !self.is_sorted {
            self.sorted.clear();
            self.permutation.resize(self.unsorted.len(), 0);
            let mut pairs: Vec<(T, usize)> = self.unsorted.iter().enumerate().map(|(x,y)| (y.clone(),x)).collect();
            pairs.sort();
            for (sorted_index, (value, index)) in pairs.drain(..).enumerate() {
                self.permutation[index] = sorted_index;
                self.sorted.push(value);
            }
            self.is_sorted = true;
        }
        &*self.sorted
    }
    pub fn unsorted(&self) -> &[T] {
        &*self.unsorted
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.unsorted.get(index)
    }
    pub fn last(&self) -> Option<&T> {
        self.unsorted.last()
    }
    pub fn set(&mut self, index: usize, value: T) {
        if self.is_sorted {
            let sorted_index = self.permutation[index];
            let sorted_before = self.sorted.get(sorted_index.wrapping_sub(1)).map(|before| before <= &value).unwrap_or(true);
            let sorted_after = self.sorted.get(sorted_index.wrapping_add(1)).map(|after| &value <= after).unwrap_or(true);
            if sorted_before && sorted_after {
                self.sorted[sorted_index] = value.clone();
            } else {
                self.is_sorted = false;
            }
        }
        self.unsorted[index] = value;
    }
    pub fn push(&mut self, value: T) {
        if self.is_sorted {
            let sorted_index = self.sorted.len();
            let sorted_before = self.sorted.get(sorted_index.wrapping_sub(1)).map(|before| before <= &value).unwrap_or(true);
            if sorted_before {
                self.permutation.push(sorted_index);
                self.sorted.push(value.clone());
            } else {
                self.is_sorted = false;
            }
        }
        self.unsorted.push(value);
    }
    pub fn len(&self) -> usize {
        self.unsorted.len()
    }
    pub fn truncate(&mut self, length: usize) {
        self.unsorted.truncate(length);
        self.is_sorted = false;
    }
}
