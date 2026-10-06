use crate::structure_entity::{StructureKind, StructureState, structure_think::StructureThinkArgs};

use super::{Integrity, StructureDef, StructureEnt};

#[derive(Default)]
pub struct StructureSlot {
  structure_ent: Option<StructureEnt>,
}

impl StructureSlot {
  pub fn new() -> Self {
    Self {
      structure_ent: None,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.structure_ent.is_none()
  }

  pub fn kind(&self) -> Option<StructureKind> {
    self.structure_ent.as_ref().map(|ent| ent.kind())
  }

  pub fn def(&self) -> Option<&StructureDef> {
    self.structure_ent.as_ref().map(|ent| ent.def())
  }

  pub fn state(&self) -> Option<&StructureState> {
    self.structure_ent.as_ref().map(|ent| ent.state())
  }

  pub fn set(&mut self, state: StructureState, integrity: Option<Integrity>) {
    let state_toughness = state.kind().def().toughness;
    let integrity = {
      if let Some(integrity) = integrity {
        integrity
      } else if let Some(integrity) = self.integrity() {
        Integrity::new(integrity.value().min(state_toughness)).unwrap()
      } else {
        Integrity::new(state_toughness).unwrap()
      }
    };
    self.structure_ent = Some(StructureEnt::new(state, integrity));
  }

  pub fn integrity(&self) -> Option<Integrity> {
    self.structure_ent.as_ref().map(|e| e.integrity())
  }

  pub fn set_integrity(&mut self, integrity: Integrity) -> bool {
    if let Some(ref mut e) = self.structure_ent {
      e.set_integrity(integrity);
      true
    } else {
      false
    }
  }

  pub fn offset_integrity(&mut self, offset: f64) {
    let Some(ref mut ent) = self.structure_ent else {
      return;
    };

    if let Some(new_integrity) = ent.integrity().offset(offset) {
      ent.set_integrity(new_integrity);
    } else {
      self.clear();
    }
  }

  pub fn clear(&mut self) {
    self.structure_ent = None;
  }

  pub fn think(&self, args: StructureThinkArgs) {
    if let Some(ent) = &self.structure_ent {
      ent.think(args)
    }
  }
}

#[cfg(test)]
mod tests {

  mod new {
    use super::super::*;

    #[test]
    fn it_creates_an_empty_structure_slot() {
      let slot = StructureSlot::new();
      assert!(slot.is_empty());
    }
  }
}
