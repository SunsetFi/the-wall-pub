use crate::{
  entity::Entity,
  structure_entity::{
    Integrity, StructureDef, StructureKind, StructureState, StructureThink,
    structure_think::StructureThinkArgs,
  },
};

pub struct StructureEnt {
  integrity: Integrity,
  state: StructureState,
}

impl Entity for StructureEnt {
  type Kind = StructureKind;
  type Def = StructureDef;
  type Quantity = Integrity;

  fn kind(&self) -> StructureKind {
    self.state.kind()
  }

  fn def(&self) -> &'static StructureDef {
    self.kind().def()
  }

  fn quantity(&self) -> Integrity {
    self.integrity
  }

  fn set_quantity(&mut self, quantity: Integrity) {
    self.integrity = quantity;
  }
}

impl StructureEnt {
  pub fn new(state: StructureState, integrity: Integrity) -> Self {
    Self { integrity, state }
  }

  pub const fn state(&self) -> &StructureState {
    &self.state
  }

  pub fn state_mut(&mut self) -> &mut StructureState {
    &mut self.state
  }

  pub fn think(&self, args: StructureThinkArgs) {
    self.state.think(args);
  }
}
