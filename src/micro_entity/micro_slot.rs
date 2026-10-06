use crate::fluid::Pressurized;
use crate::micro_entity::{MicroDef, MicroEnt};
use crate::micro_entity::{MicroKind, micro_density::Density};

#[derive(Clone, PartialEq, Default, Debug)]
pub struct MicroSlot {
  micro_ent: Option<MicroEnt>,
}

impl MicroSlot {
  pub fn new() -> Self {
    Self { micro_ent: None }
  }

  pub fn is_empty(&self) -> bool {
    self.micro_ent.is_none()
  }

  pub fn get_ent(&self) -> Option<&MicroEnt> {
    self.micro_ent.as_ref()
  }

  pub fn get_ent_mut(&mut self) -> Option<&mut MicroEnt> {
    self.micro_ent.as_mut()
  }

  pub fn set(&mut self, kind: MicroKind, density: Density) {
    self.micro_ent = Some(MicroEnt::new(kind, density));
  }

  pub fn kind(&self) -> Option<MicroKind> {
    self.micro_ent.map(|e| e.kind())
  }

  pub fn def(&self) -> Option<&MicroDef> {
    self.micro_ent.as_ref().map(|e| e.def())
  }

  pub fn density(&self) -> Option<Density> {
    self.micro_ent.map(|e| e.density())
  }

  pub fn set_density(&mut self, density: Density) -> bool {
    if let Some(ref mut e) = self.micro_ent {
      e.set_density(density);
      true
    } else {
      false
    }
  }

  pub fn offset_density(&mut self, offset: f64) {
    if let Some(ref mut e) = self.micro_ent {
      if let Some(density) = e.density().offset(offset) {
        e.set_density(density);
      } else {
        self.clear();
      }
    }
  }

  pub fn clear(&mut self) {
    self.micro_ent = None;
  }
}

impl Pressurized for MicroSlot {
  fn pressure(&self) -> f64 {
    if let Some(ent) = self.micro_ent {
      ent.pressure()
    } else {
      0.0
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{MicroKind, micro_entity::Density};

  use super::MicroSlot;

  #[test]
  fn it_starts_empty() {
    let slot = MicroSlot::new();
    assert!(slot.is_empty());
  }

  #[test]
  fn it_can_be_filled() {
    let mut slot = MicroSlot::new();
    slot.set(MicroKind::Friendly, Density::new(1.0).unwrap());
    assert!(!slot.is_empty());
  }

  #[test]
  fn it_can_be_cleared() {
    let mut slot = MicroSlot::new();
    slot.set(MicroKind::Friendly, Density::new(1.0).unwrap());
    assert!(!slot.is_empty());
    slot.clear();
    assert!(slot.is_empty());
  }

  mod density {
    use super::super::MicroSlot;
    use crate::{MicroKind, micro_entity::micro_density::Density};

    #[test]
    fn it_can_set_density() {
      let mut slot = MicroSlot::new();
      slot.set(MicroKind::Friendly, Density::MAX);
      assert_eq!(slot.density(), Some(Density::MAX));

      let new_density = Density::new(Density::MAX.value() / 2.0).unwrap();
      slot.set_density(new_density);
      assert_eq!(slot.density(), Some(new_density));
    }

    #[test]
    fn it_can_set_min_density() {
      let mut slot = MicroSlot::new();
      slot.set(MicroKind::Friendly, Density::MIN);
      assert_eq!(slot.density(), Some(Density::MIN));
    }

    #[test]
    fn it_clears_on_offset_below_min() {
      let mut slot = MicroSlot::new();
      slot.set(MicroKind::Friendly, Density::MIN);
      assert!(!slot.is_empty());
      slot.offset_density(-Density::MIN.value() / 2.0);
      assert!(slot.is_empty());
    }

    #[test]
    fn it_caps_at_max_density() {
      let mut slot = MicroSlot::new();
      slot.set(MicroKind::Friendly, Density::MAX);
      assert_eq!(slot.density(), Some(Density::MAX));

      slot.offset_density(Density::MAX.value() * 2.0);
      assert_eq!(slot.density(), Some(Density::MAX));
    }
  }
}
