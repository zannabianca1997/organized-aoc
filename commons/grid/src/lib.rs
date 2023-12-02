use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    height: usize,
    width: usize,
    memory: Box<[T]>,
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.memory[y * self.width + x])
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.memory[y * self.width + x])
        } else {
            None
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.memory.iter()
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(height: usize, width: usize, fill: T) -> Self {
        Self {
            height,
            width,
            memory: vec![fill; height * width].into_boxed_slice(),
        }
    }
    pub fn new_like<O>(other: &Grid<O>, fill: T) -> Self {
        Self::new(other.height, other.width, fill)
    }
}
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1)
            .unwrap_or_else(|| panic!("Index ({},{}) is out of range!", index.0, index.1))
    }
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
            .unwrap_or_else(|| panic!("Index ({},{}) is out of range!", index.0, index.1))
    }
}
