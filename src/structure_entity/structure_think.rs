use crate::{
  geometry::grid::{CellIdx, Grid},
  micro_entity::MicroCollection,
  structure_entity::{
    Integrity, StructureCollection, StructureState, structure_decl::StructureStateUpdate,
  },
};

pub trait StructureThink {
  fn think(&self, args: StructureThinkArgs);
}

pub struct StructureThinkArgs<'a> {
  pub grid: Grid,
  pub self_idx: CellIdx,
  pub current_tick: u64,
  pub structures: &'a StructureCollection,
  pub micros: &'a mut MicroCollection,
  pub updates: &'a mut StructureUpdateCollection,
}

pub enum StructureUpdate {
  Mutate(StructureStateUpdate),
  Replace(StructureState, Option<Integrity>),
  Destroy,
}

pub trait StructureApply {
  type Update;
  fn apply(&mut self, update: Self::Update);
}

pub struct StructureUpdateCollection {
  updates: Vec<(CellIdx, StructureUpdate)>,
}

impl StructureUpdateCollection {
  pub fn new() -> Self {
    Self {
      updates: Vec::new(),
    }
  }

  pub fn has_updates(&self) -> bool {
    !self.updates.is_empty()
  }

  pub fn mutate(&mut self, idx: CellIdx, update: StructureStateUpdate) {
    self.insert(idx, StructureUpdate::Mutate(update));
  }

  pub fn replace(&mut self, idx: CellIdx, state: StructureState, integrity: Option<Integrity>) {
    self.insert(idx, StructureUpdate::Replace(state, integrity));
  }

  pub fn destroy(&mut self, idx: CellIdx) {
    self.insert(idx, StructureUpdate::Destroy);
  }

  pub fn drain(&mut self) -> impl Iterator<Item = (CellIdx, StructureUpdate)> {
    self.updates.drain(..)
  }

  fn insert(&mut self, idx: CellIdx, update: StructureUpdate) {
    self.updates.push((idx, update));
  }
}
