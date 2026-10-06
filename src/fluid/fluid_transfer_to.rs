use crate::geometry::grid::CellIdx;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FluidTransferTo {
  Empty(CellIdx),
  Occupied(CellIdx),
}

impl FluidTransferTo {
  pub fn index(self) -> usize {
    match self {
      FluidTransferTo::Empty(idx) => idx.index(),
      FluidTransferTo::Occupied(idx) => idx.index(),
    }
  }
}
