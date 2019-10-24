pub(crate) struct TwoDVec<T> {
    storage: Vec<T>,
    width: usize,
}

impl<T: Clone> TwoDVec<T> {
    pub fn new(initial: T, width: usize, height: usize) -> Self {
        Self {
            storage: vec![initial; width * height],
            width,
        }
    }
}

impl<T> TwoDVec<T> {
    pub fn height(&self) -> usize {
        self.storage.len() / self.width
    }
    pub fn width(&self) -> usize {
        self.width
    }
}

impl<T> std::ops::Index<usize> for TwoDVec<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.storage.as_slice()[self.width * index..][..self.width]
    }
}

impl<T> std::ops::IndexMut<usize> for TwoDVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.storage.as_mut_slice()[self.width * index..][..self.width]
    }
}
