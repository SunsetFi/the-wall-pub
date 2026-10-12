use crate::{
  entity::Entity,
  entity_slot::EntitySlot,
  structure_entity::{Integrity, StructureEnt, StructureKind, StructureState},
};

pub type StructureSlot = EntitySlot<StructureEnt>;

impl StructureSlot {
  pub fn kind(&self) -> Option<StructureKind> {
    self.ent().map(StructureEnt::kind)
  }

  pub fn integrity(&self) -> Option<Integrity> {
    self.ent().map(StructureEnt::quantity)
  }

  pub fn set_integrity(&mut self, integrity: Integrity) -> bool {
    if let Some(entity) = self.ent_mut() {
      entity.set_quantity(integrity);
      true
    } else {
      false
    }
  }

  pub fn state(&self) -> Option<&StructureState> {
    let Some(entity) = self.ent() else {
      return None;
    };
    Some(entity.state())
  }

  pub fn state_mut(&mut self) -> Option<&mut StructureState> {
    let Some(entity) = self.ent_mut() else {
      return None;
    };
    Some(entity.state_mut())
  }
}
