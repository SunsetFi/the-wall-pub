use crate::{direction::Direction, geometry::Coordinate};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CellIdx(u32);

impl CellIdx {
  const fn from_ordinal(ordinal: u32) -> Self {
    Self(ordinal)
  }

  pub const fn index(self) -> usize {
    self.0 as usize
  }

  pub const fn ordinal(self) -> u32 {
    self.0
  }
}

#[derive(Clone, Copy)]
pub struct Grid {
  width: u32,
  height: u32,
}

// TODO: Do something like probes here?
const NEIGHBOR_OFFSETS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Grid {
  pub const fn new(width: u32, height: u32) -> Self {
    Self { width, height }
  }

  pub const fn width(self) -> u32 {
    self.width
  }

  pub const fn height(self) -> u32 {
    self.height
  }

  pub const fn size(&self) -> usize {
    Self::value_to_size(self.width()) * Self::value_to_size(self.height())
  }

  pub fn cell(self, x: u32, y: u32) -> Option<CellIdx> {
    if x >= self.width || y >= self.height {
      return None;
    }
    Some(CellIdx::from_ordinal(y * self.width + x))
  }

  pub fn ordinal(self, ord: u32) -> Option<CellIdx> {
    if ord >= self.width * self.height {
      return None;
    }
    Some(CellIdx::from_ordinal(ord))
  }

  pub fn coordinate(self, idx: CellIdx) -> Option<Coordinate> {
    let idx = idx.ordinal();

    let y = idx / self.width;
    if y >= self.height {
      return None;
    }

    // Subtracting with multiplication is faster than dividing a second time.
    // This will always be < width.
    let x = idx - y * self.width;

    Some(Coordinate { x, y })
  }

  pub fn contains_coordinate(self, coordinate: Coordinate) -> bool {
    coordinate.x < self.width && coordinate.y < self.height
  }

  pub fn neighbor(self, x: u32, y: u32, dx: i32, dy: i32) -> Option<CellIdx> {
    // If these go negative, they wrap around, which we will trust
    // gets captured by the width and height check
    let x = x.wrapping_add_signed(dx);
    let y = y.wrapping_add_signed(dy);
    self.cell(x, y)
  }

  pub fn neighbors(self, idx: CellIdx) -> impl Iterator<Item = CellIdx> {
    self
      .coordinate(idx)
      .into_iter()
      .flat_map(move |coordinate| {
        NEIGHBOR_OFFSETS
          .iter()
          .copied()
          .filter_map(move |(dx, dy)| self.neighbor(coordinate.x, coordinate.y, dx, dy))
      })
  }

  pub fn iter_idx(self) -> impl Iterator<Item = CellIdx> {
    let count = self.width * self.height;
    (0..count).map(CellIdx::from_ordinal)
  }

  pub fn for_each_adjacent_pair(self, mut f: impl FnMut(CellIdx, CellIdx)) {
    // Little bit of Process here.  TS used a pairs system, that blob-field also has.
    // Could use it, but its overly complex for the stupid "arbitrary pairs" thing.
    let width = self.width;
    for y in 0..self.height {
      let row = y * width;
      for x in 0..width {
        let ord = row + x;
        let idx = CellIdx::from_ordinal(ord);
        if x > 0 {
          f(idx, CellIdx::from_ordinal(ord - 1));
        }
        if y > 0 {
          f(idx, CellIdx::from_ordinal(ord - width));
        }
      }
    }
  }

  pub fn for_each_adjacent_pair_direction(self, mut f: impl FnMut(CellIdx, CellIdx, Direction)) {
    let width = self.width;
    for y in 0..self.height {
      let row = y * width;
      for x in 0..width {
        let ord = row + x;
        let idx = CellIdx::from_ordinal(ord);
        if x > 0 {
          f(idx, CellIdx::from_ordinal(ord - 1), Direction::Left);
        }
        if y > 0 {
          f(idx, CellIdx::from_ordinal(ord - width), Direction::Up);
        }
      }
    }
  }

  const fn value_to_size(value: u32) -> usize {
    value as usize
  }
}

#[cfg(test)]
mod tests {
  mod new {
    use super::super::Grid;

    #[test]
    fn it_sets_width_and_height() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      assert_eq!(grid.width, width);
      assert_eq!(grid.height, height);
    }
  }

  mod cell {
    use super::super::Grid;

    #[test]
    fn it_returns_index_for_valid_coordinates() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 3;
      let y = 4;
      let cell_idx = grid.cell(x, y);
      assert!(cell_idx.is_some());
    }

    #[test]
    fn it_returns_none_for_invalid_coordinates() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 11;
      let y = 4;
      let cell_idx = grid.cell(x, y);
      assert!(cell_idx.is_none());
    }
  }

  mod point {
    use super::super::CellIdx;
    use super::super::Coordinate;
    use super::super::Grid;

    #[test]
    fn it_returns_coordinates_for_valid_index() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let index = grid.cell(2, 3).unwrap();
      let point = grid.coordinate(index);
      assert_eq!(point, Some(Coordinate { x: 2, y: 3 }));
    }

    #[test]
    fn it_returns_none_for_invalid_index() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let index = CellIdx::from_ordinal(999);
      let point = grid.coordinate(index);
      assert!(point.is_none());
    }
  }
  mod neighbor {
    use super::super::CellIdx;
    use super::super::Grid;

    #[test]
    fn it_returns_index_for_valid_positive_x_neighbor() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 0;
      let y = 0;
      let dx = 1;
      let dy = 0;
      let neighbor_idx = grid.neighbor(x, y, dx, dy);
      assert_eq!(neighbor_idx, Some(CellIdx(1)));
    }

    #[test]
    fn it_returns_none_for_invalid_positive_x_neighbor() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 9;
      let y = 1;
      let dx = 1;
      let dy = 0;
      let neighbor_idx = grid.neighbor(x, y, dx, dy);
      assert!(neighbor_idx.is_none());
    }

    #[test]
    fn it_returns_index_for_valid_negative_x_neighbor() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 1;
      let y = 1;
      let dx = -1;
      let dy = 0;
      let neighbor_idx = grid.neighbor(x, y, dx, dy);
      assert_eq!(neighbor_idx, Some(CellIdx(10)));
    }

    #[test]
    fn it_returns_none_for_invalid_negative_x_neighbor() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 0;
      let y = 1;
      let dx = -1;
      let dy = 0;
      let neighbor_idx = grid.neighbor(x, y, dx, dy);
      assert!(neighbor_idx.is_none());
    }

    #[test]
    fn it_returns_index_for_valid_positive_y_neighbor() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 2;
      let y = 1;
      let dx = 0;
      let dy = 1;
      let neighbor_idx = grid.neighbor(x, y, dx, dy);
      assert_eq!(neighbor_idx, Some(CellIdx(22)));
    }

    #[test]
    fn it_returns_none_for_invalid_positive_y_neighbor() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 2;
      let y = 19;
      let dx = 0;
      let dy = 1;
      let neighbor_idx = grid.neighbor(x, y, dx, dy);
      assert!(neighbor_idx.is_none());
    }

    #[test]
    fn it_returns_index_for_valid_negative_y_neighbor() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 2;
      let y = 1;
      let dx = 0;
      let dy = -1;
      let neighbor_idx = grid.neighbor(x, y, dx, dy);
      assert_eq!(neighbor_idx, Some(CellIdx(2)));
    }

    #[test]
    fn it_returns_none_for_invalid_negative_y_neighbor() {
      let width = 10;
      let height = 20;
      let grid = Grid::new(width, height);
      let x = 2;
      let y = 0;
      let dx = 0;
      let dy = -1;
      let neighbor_idx = grid.neighbor(x, y, dx, dy);
      assert!(neighbor_idx.is_none());
    }
  }

  mod for_each_adjacent_pairs {
    use super::super::Grid;

    #[test]
    fn it_calls_callback_for_each_adjacent_pair() {
      let grid = Grid::new(2, 2);
      let mut pairs = Vec::new();
      grid.for_each_adjacent_pair(|a, b| pairs.push((a, b)));
      assert_eq!(
        pairs,
        vec![
          (grid.cell(1, 0).unwrap(), grid.cell(0, 0).unwrap()),
          (grid.cell(0, 1).unwrap(), grid.cell(0, 0).unwrap()),
          (grid.cell(1, 1).unwrap(), grid.cell(0, 1).unwrap()),
          (grid.cell(1, 1).unwrap(), grid.cell(1, 0).unwrap()),
        ]
      );
    }
  }
}
