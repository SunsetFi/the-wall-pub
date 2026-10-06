use crate::{
  generational::Generational,
  geometry::grid::{CellIdx, Grid},
};

pub struct GridCollection<T: Default> {
  grid: Grid,
  items: Vec<T>,
  generation: u32,
}

impl<T: Default> GridCollection<T> {
  pub fn new(grid: Grid) -> Self {
    let size = grid.size();
    Self {
      grid,
      items: std::iter::repeat_with(T::default).take(size).collect(),
      generation: 0,
    }
  }

  pub fn width(&self) -> u32 {
    self.grid.width()
  }

  pub fn height(&self) -> u32 {
    self.grid.height()
  }

  /// The size of the world in terms of the number of micro cells
  pub fn cell_size(&self) -> usize {
    Self::value_to_size(self.width()) * Self::value_to_size(self.height())
  }

  /// The size of the range of MicroIdx values
  pub fn item_idx_size(&self) -> usize {
    // TODO: This needs to include MicroIdx used for Arteries
    self.cell_size()
  }

  pub fn size(&self) -> usize {
    self.items.len()
  }

  pub fn count(&self) -> u32 {
    self.items.len() as u32
  }

  pub fn cell_idx(&self, x: u32, y: u32) -> Option<CellIdx> {
    self.grid.cell(x, y)
  }

  pub fn cell(&self, x: u32, y: u32) -> Option<&T> {
    self.cell_idx(x, y).map(|idx| self.get(idx))
  }

  pub fn cell_mut(&mut self, x: u32, y: u32) -> Option<CellMut<'_, T>> {
    self.cell_idx(x, y).map(move |idx| self.get_mut(idx))
  }

  pub fn ordinal_idx(&self, ord: u32) -> Option<CellIdx> {
    self.grid.ordinal(ord)
  }

  pub fn ordinal(&self, ord: u32) -> Option<&T> {
    self.ordinal_idx(ord).map(|idx| self.get(idx))
  }

  pub fn get(&self, idx: CellIdx) -> &T {
    let idx = idx.index();
    &self.items[idx]
  }

  pub fn get_mut(&mut self, idx: CellIdx) -> CellMut<'_, T> {
    let idx = idx.index();
    CellMut {
      target: self.items.get_mut(idx).expect("Micro index out of range"),
      generation: &mut self.generation,
    }
  }

  pub fn take(&mut self, idx: CellIdx) -> Option<T> {
    let idx = idx.index();
    if idx < self.items.len() {
      self.generation += 1;
      Some(std::mem::take(&mut self.items[idx]))
    } else {
      None
    }
  }

  pub fn restore(&mut self, idx: CellIdx, item: T) {
    let idx = idx.index();
    self.generation += 1;
    self.items[idx] = item;
  }

  pub fn iter_idx(&self) -> impl Iterator<Item = CellIdx> {
    self.grid.iter_idx()
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

  pub fn for_each_adjacent_pairs(&self, mut f: impl FnMut(CellIdx, CellIdx)) {
    self.grid.for_each_adjacent_pairs(&mut f);
  }

  fn value_to_size(value: u32) -> usize {
    value as usize
  }
}

impl<T: Default> Generational for GridCollection<T> {
  fn generation(&self) -> u32 {
    self.generation
  }
}

pub struct CellMut<'a, T> {
  target: &'a mut T,
  generation: &'a mut u32,
}

impl<'a, T> std::ops::Deref for CellMut<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.target
  }
}

impl<'a, T> std::ops::DerefMut for CellMut<'a, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    *self.generation = self.generation.wrapping_add(1);
    self.target
  }
}

#[cfg(test)]
mod tests {

  mod world {
    mod new {
      use crate::geometry::grid::Grid;

      use super::super::super::GridCollection;

      #[test]
      fn it_sets_width_and_height() {
        let width = 10;
        let height = 20;
        let world = GridCollection::<()>::new(Grid::new(width, height));
        assert_eq!(world.width(), width);
        assert_eq!(world.height(), height);
      }
    }

    mod micro {
      use super::super::super::GridCollection;
      use crate::geometry::grid::Grid;

      #[test]
      fn it_round_trips_micro_cell_slot() {
        let mut world = GridCollection::<i32>::new(Grid::new(1, 1));
        let idx = world.cell_idx(0, 0).unwrap();
        *world.get_mut(idx) = 42;
        let micro = world.get(idx);
        assert_eq!(*micro, 42);
      }
    }
  }
}
