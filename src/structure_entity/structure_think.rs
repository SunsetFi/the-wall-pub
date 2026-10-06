use std::collections::BTreeMap;

use crate::{
  geometry::grid::{CellIdx, Grid},
  micro_entity::MicroCollection,
  structure_entity::{StructureCollection, StructureState},
};

pub trait StructureThink {
  fn think(&self, args: StructureThinkArgs);
}

pub struct StructureThinkArgs<'a> {
  pub grid: Grid,
  pub self_idx: CellIdx,
  pub structures: &'a StructureCollection,
  pub micros: &'a mut MicroCollection,
  pub updates: &'a mut StructureUpdates,
}

pub struct StructureUpdates {
  updates: BTreeMap<CellIdx, StructureUpdate>,
}

impl StructureUpdates {
  pub fn new() -> Self {
    Self {
      updates: BTreeMap::new(),
    }
  }

  pub fn replace(&mut self, idx: CellIdx, state: StructureState) {
    self.insert(idx, StructureUpdate::Replace(state));
  }

  pub fn destroy(&mut self, idx: CellIdx) {
    self.insert(idx, StructureUpdate::Destroy);
  }

  pub fn iter(&self) -> impl Iterator<Item = (CellIdx, &StructureUpdate)> {
    self
      .updates
      .iter()
      .map(|(idx, update)| (idx.clone(), update))
  }

  fn insert(&mut self, idx: CellIdx, update: StructureUpdate) {
    self.updates.insert(idx, update);
  }
}

pub enum StructureUpdate {
  // WARN: This requires cloning the state all over the place
  // States may get Heavy...
  // Could swap this out with some StructureStateUpdate trait?  Would by dyn but should be fine?  Make a macro instead?
  Replace(StructureState),
  Destroy,
}
