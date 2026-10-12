use crate::entity::Entity;
use crate::entity_slot::EntitySlot;
use crate::fluid::Pressurized;
use crate::micro_entity::MicroEnt;
use crate::micro_entity::{MicroKind, micro_density::Density};

pub type MicroSlot = EntitySlot<MicroEnt>;

impl MicroSlot {
  pub fn kind(&self) -> Option<MicroKind> {
    self.ent().map(MicroEnt::kind)
  }

  pub fn density(&self) -> Option<Density> {
    self.ent().map(MicroEnt::quantity)
  }

  pub fn set_density(&mut self, density: Density) -> bool {
    if let Some(entity) = self.ent_mut() {
      entity.set_quantity(density);
      true
    } else {
      false
    }
  }
}

impl Pressurized for MicroSlot {
  fn pressure(&self) -> f64 {
    self.ent().map_or(0.0, Pressurized::pressure)
  }
}
