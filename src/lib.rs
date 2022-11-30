use core::slice::{Iter, IterMut};
use std::{error::Error, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Grid<T: Clone> {
    grid: Vec<T>,
    axes: usize,
    dimensions: Vec<usize>,
}

impl<T: Clone> Grid<T> {
    pub fn new(default_value: T, dimensions: Vec<usize>) -> Self {
        let axes = dimensions.len();

        let size = dimensions.iter().product();

        let mut grid = Vec::with_capacity(size);
        for _ in 0..size {
            grid.push(default_value.clone());
        }

        Self {
            grid,
            axes,
            dimensions,
        }
    }

    pub fn get(&self, target: Vec<usize>) -> Result<&T, Box<dyn Error>> {
        let target = self.translate_index(target)?;
        let val = &self.grid[target];
        Ok(val)
    }

    pub fn get_mut(&mut self, target: Vec<usize>) -> Result<&mut T, Box<dyn Error>> {
        let target = self.translate_index(target)?;
        let val = &mut self.grid[target];
        Ok(val)
    }

    pub fn set(&mut self, target: Vec<usize>, val: T) -> Result<(), Box<dyn Error>> {
        let target = self.translate_index(target)?;
        self.grid[target] = val;
        Ok(())
    }

    pub fn iter(&self) -> GridIter<'_, T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> GridIterMut<'_, T> {
        self.into_iter()
    }

    fn translate_index(&self, target: Vec<usize>) -> Result<usize, Box<dyn Error>> {
        if target.len() != self.axes {
            return Err(format!(
                "ERROR: Tried to index with {} dimensions when grid only has {} dimensions",
                target.len(),
                &self.axes
            )
            .into());
        }

        let mut index = 0;
        for (i, v) in target.iter().enumerate() {
            let step: usize = v * self.dimensions.iter().skip(i + 1).product::<usize>();
            index += step;
        }

        if index >= self.grid.len() {
            return Err(format!(
                "ERROR: Index ({}) out of bounds ({})",
                index,
                self.grid.len()
            )
            .into());
        }

        Ok(index)
    }
}

// TODO: Add custom Iterator to handle .position()

impl<'a, T: Clone> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter::new(self)
    }
}

impl<'a, T: Clone> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;
    type IntoIter = GridIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterMut::new(self)
    }
}

pub struct GridIter<'a, T: Clone> {
    grid: Iter<'a, T>,
}

impl<'a, T: Clone> GridIter<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        let grid = grid.grid.iter();
        Self { grid }
    }
}

impl<'a, T: Clone> Iterator for GridIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.next()
    }
}

pub struct GridIterMut<'a, T: Clone> {
    grid: IterMut<'a, T>,
}

impl<'a, T: Clone> GridIterMut<'a, T> {
    fn new(grid: &'a mut Grid<T>) -> Self {
        let grid = grid.grid.iter_mut();
        Self { grid }
    }
}

impl<'a, T: Clone> Iterator for GridIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid() {
        let grid = Grid::new("default_value", vec![5, 5]);
        // Test grid size
        assert_eq!(grid.grid.len(), 25);
        // Test defualt values
        for i in grid.grid.iter() {
            assert_eq!(*i, "default_value");
        }
    }

    #[test]
    fn translate_index() {
        // 2d grid (10x10)
        let mut grid = Grid::new(0, vec![10, 10]);

        grid.set(vec![5, 9], 5).unwrap();
        grid.set(vec![0, 5], 32).unwrap();
        grid.set(vec![5, 0], 25).unwrap();
        grid.set(vec![9, 9], 56).unwrap();
        grid.set(vec![0, 0], 7).unwrap();

        assert_eq!(grid.grid[59], 5);
        assert_eq!(grid.grid[5], 32);
        assert_eq!(grid.grid[50], 25);
        assert_eq!(grid.grid[99], 56);
        assert_eq!(grid.grid[0], 7);

        // 3d grid (10x10x10)
        let mut grid = Grid::new(0, vec![10, 10, 10]);

        grid.set(vec![3, 7, 6], 12).unwrap();
        grid.set(vec![0, 4, 3], 23).unwrap();
        grid.set(vec![5, 0, 7], 32).unwrap();
        grid.set(vec![4, 6, 0], 63).unwrap();
        grid.set(vec![9, 9, 9], 87).unwrap();
        grid.set(vec![0, 0, 0], 34).unwrap();

        assert_eq!(grid.grid[376], 12);
        assert_eq!(grid.grid[43], 23);
        assert_eq!(grid.grid[507], 32);
        assert_eq!(grid.grid[460], 63);
        assert_eq!(grid.grid[999], 87);
        assert_eq!(grid.grid[0], 34);

        // 4d grid (10x10x10x10)
        let mut grid = Grid::new(0, vec![10, 10, 10, 10]);

        grid.set(vec![5, 3, 7, 9], 20).unwrap();
        grid.set(vec![9, 9, 9, 9], 24).unwrap();
        grid.set(vec![0, 0, 0, 0], 10).unwrap();

        assert_eq!(grid.grid[5379], 20);
        assert_eq!(grid.grid[9999], 24);
        assert_eq!(grid.grid[0], 10);

        // Test default values
        assert_eq!(grid.grid[8654], 0);
        assert_eq!(grid.grid[23], 0);
    }

    #[test]
    fn into_iterator() {
        let mut grid = Grid::new(0, vec![10, 10]);

        for i in grid.iter_mut() {
            println!("{i}");
            *i += 1;
        }

        for i in grid.iter().enumerate() {
            println!("{}: {}", i.0, i.1);
        }
    }
}
