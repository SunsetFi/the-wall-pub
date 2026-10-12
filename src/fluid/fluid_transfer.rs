use crate::geometry::grid::{CellIdx, Grid};
use crate::micro_entity::{MicroEnt, MicroSlot};
use crate::structure_entity::StructureState;
use crate::world::WorldMut;

pub struct FluidTransfer {
  pub from: FluidTransferNode,
  pub to: FluidTransferTo,
  pub amount: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FluidTransferTo {
  Empty(FluidTransferNode),
  Occupied(FluidTransferNode),
}

impl FluidTransferTo {
  pub fn index(self) -> usize {
    match self {
      FluidTransferTo::Empty(node) => node.index(),
      FluidTransferTo::Occupied(node) => node.index(),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FluidLocation {
  Cell(CellIdx),
  Artery(CellIdx),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FluidTransferNode {
  index: usize,
  location: FluidLocation,
}

impl FluidTransferNode {
  pub const fn size(grid: Grid) -> usize {
    grid.size() * 2
  }

  pub fn iter(grid: Grid) -> impl Iterator<Item = Self> {
    let cells = grid.iter_idx().map(move |idx| Self::cell(idx));
    let arteries = grid.iter_idx().map(move |idx| Self::artery(grid, idx));
    cells.chain(arteries)
  }

  pub const fn cell(idx: CellIdx) -> Self {
    Self {
      index: idx.index(),
      location: FluidLocation::Cell(idx),
    }
  }
  pub const fn artery(grid: Grid, idx: CellIdx) -> Self {
    Self {
      index: grid.size() + idx.index(),
      location: FluidLocation::Artery(idx),
    }
  }

  pub fn micro<'a>(self, world: &'a WorldMut) -> Option<&'a MicroEnt> {
    self.slot(world)?.ent()
  }

  pub fn slot<'a>(self, world: &'a WorldMut) -> Option<&'a MicroSlot> {
    match self.location {
      FluidLocation::Cell(idx) => Some(world.micros.get(idx)),
      FluidLocation::Artery(idx) => {
        let Some(StructureState::Artery(state)) = world.structures.get(idx).state() else {
          return None;
        };
        Some(state.contents())
      }
    }
  }

  pub fn slot_mut<'a>(self, world: &'a mut WorldMut) -> Option<&'a mut MicroSlot> {
    match self.location {
      FluidLocation::Cell(idx) => Some(world.micros.get_mut(idx)),
      FluidLocation::Artery(idx) => {
        let Some(StructureState::Artery(state)) = world.structures.get_mut(idx).state_mut() else {
          return None;
        };
        Some(state.contents_mut())
      }
    }
  }

  pub const fn idx(self) -> CellIdx {
    match self.location {
      FluidLocation::Cell(idx) => idx,
      FluidLocation::Artery(idx) => idx,
    }
  }

  pub const fn index(self) -> usize {
    self.index
  }

  pub const fn location(self) -> FluidLocation {
    self.location
  }
}
