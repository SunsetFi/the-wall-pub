use crate::geometry::grid::{CellIdx, Grid};

pub struct GridCollection<T: Default> {
  grid: Grid,
  items: Vec<T>,
}

impl<T: Default> GridCollection<T> {
  pub fn new(grid: Grid) -> Self {
    let size = grid.size();
    Self {
      grid,
      items: std::iter::repeat_with(T::default).take(size).collect(),
    }
  }

  pub fn grid(&self) -> Grid {
    self.grid
  }

  pub fn cell(&self, x: u32, y: u32) -> Option<&T> {
    self.grid.cell(x, y).map(|idx| self.get(idx))
  }

  pub fn cell_mut(&mut self, x: u32, y: u32) -> Option<&mut T> {
    self.grid.cell(x, y).map(move |idx| self.get_mut(idx))
  }

  pub fn ordinal(&self, ord: u32) -> Option<&T> {
    self.grid.ordinal(ord).map(|idx| self.get(idx))
  }

  pub fn get(&self, idx: CellIdx) -> &T {
    let idx = idx.index();
    &self.items[idx]
  }

  pub fn get_mut(&mut self, idx: CellIdx) -> &mut T {
    let idx = idx.index();
    self.items.get_mut(idx).expect("Micro index out of range")
  }

  pub fn iter_enumerate(&self) -> impl Iterator<Item = (CellIdx, &T)> + '_ {
    self.grid.iter_idx().map(move |idx| {
      let m = &self.items[idx.index()];
      (idx, m)
    })
  }

  pub fn iter_enumerate_mut(&mut self) -> impl Iterator<Item = (CellIdx, &mut T)> + '_ {
    self.grid.iter_idx().zip(self.items.iter_mut())
  }
}
