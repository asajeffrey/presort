#[derive(Clone,Debug)]
pub struct PresortedVec<T> {
    contents: Vec<T>,
    is_sorted: bool,
    permute_original_to_contents: Vec<usize>,
    permute_contents_to_original: Vec<usize>,
}

impl<T> PresortedVec<T>
where T: Ord {
    pub fn new() -> PresortedVec<T> {
        PresortedVec {
            contents: Vec::new(),
            is_sorted: true,
            permute_original_to_contents: Vec::new(),
            permute_contents_to_original: Vec::new(),
        }
    }
    pub fn sorted_slice(&mut self) -> &[T] {
        if !self.is_sorted {
            let permute_contents_to_original = &mut *self.permute_contents_to_original;
            let permute_original_to_contents = &mut *self.permute_original_to_contents;
            let mut pairs: Vec<(T, usize)> = self.contents.drain(..)
                .enumerate()
                .map(|(contents_index, value)| (value, permute_contents_to_original[contents_index]))
                .collect();
            pairs.sort();
            self.is_sorted = true;
            for (contents_index, (value, original_index)) in pairs.drain(..).enumerate() {
                self.contents.push(value);
                permute_contents_to_original[contents_index] = original_index;
                permute_original_to_contents[original_index] = contents_index;
            }
        }
        &*self.contents
    }
    pub fn get(&self, original_index: usize) -> Option<&T> {
        self.contents.get(self.permute_original_to_contents[original_index])
    }
    pub fn set(&mut self, original_index: usize, value: T) {
        let contents_index = self.permute_original_to_contents[original_index];
        self.is_sorted =
            self.is_sorted &&
            self.contents.get(contents_index.wrapping_sub(1)).map(|before| before <= &value).unwrap_or(true) &&
            self.contents.get(contents_index.wrapping_add(1)).map(|after| &value <= after).unwrap_or(true);
        self.contents[contents_index] = value;
    }
    pub fn push(&mut self, value: T) {
        let original_index = self.contents.len();
        let contents_index = original_index;
        self.is_sorted =
            self.is_sorted &&
            self.contents.get(contents_index.wrapping_sub(1)).map(|before| before <= &value).unwrap_or(true);
        self.contents.push(value);
        self.permute_original_to_contents.push(contents_index);
        self.permute_contents_to_original.push(original_index);
    }
    pub fn len(&self) -> usize {
        self.contents.len()
    }
}
