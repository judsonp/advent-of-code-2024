use grid::Grid;

pub type Index = (usize, usize);
pub type Offset = (isize, isize);

pub struct GridStrideIterator<'a, T> {
    grid: &'a Grid<T>,
    index: Index,
    stride: Offset,
    done: bool,
}

impl<'a, T> GridStrideIterator<'a, T> {
    pub fn new(grid: &'a Grid<T>, start: Index, stride: Offset) -> Self {
        Self {
            grid,
            index: start,
            stride,
            done: false,
        }
    }
}

impl<'a, T> Iterator for GridStrideIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let value = &self.grid[self.index];

        let (x, y) = self.index;
        let (dx, dy) = self.stride;
        let (rows, cols) = self.grid.size();

        let next_x = x.checked_add_signed(dx);
        let next_y = y.checked_add_signed(dy);

        if next_x.is_none() || next_y.is_none() {
            self.done = true;
            return Some(value);
        }

        let next_x = next_x.unwrap();
        let next_y = next_y.unwrap();

        if next_x >= cols || next_y >= rows {
            self.done = true;
            return Some(value);
        }

        self.index = (next_x, next_y);

        Some(value)
    }
}

pub trait IntoGridStrideIterator<'a, T> {
    fn stride_iter(self, start: Index, stride: Offset) -> GridStrideIterator<'a, T>;
}

impl<'a, T> IntoGridStrideIterator<'a, T> for &'a Grid<T> {
    fn stride_iter(self, start: Index, stride: Offset) -> GridStrideIterator<'a, T> {
        GridStrideIterator::new(self, start, stride)
    }
}