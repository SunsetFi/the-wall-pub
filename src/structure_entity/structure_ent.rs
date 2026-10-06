use crate::structure_entity::{
  Integrity, StructureDef, StructureKind, StructureState, StructureThink,
  structure_think::StructureThinkArgs,
};

pub struct StructureEnt {
  integrity: Integrity,
  state: StructureState,
}

impl StructureEnt {
  pub fn new(state: StructureState, integrity: Integrity) -> Self {
    Self { integrity, state }
  }

  pub const fn kind(&self) -> StructureKind {
    self.state.kind()
  }

  pub const fn def(&self) -> &'static StructureDef {
    self.kind().def()
  }

  pub const fn integrity(&self) -> Integrity {
    self.integrity
  }

  pub fn set_integrity(&mut self, integrity: Integrity) {
    self.integrity = integrity;
  }

  pub const fn state(&self) -> &StructureState {
    &self.state
  }

  pub fn think(&self, args: StructureThinkArgs) {
    self.state.think(args)
  }
}
