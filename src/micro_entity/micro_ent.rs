use super::{Density, MicroDef, MicroKind};
use crate::{entity::Entity, fluid::Pressurized};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MicroEnt {
  kind: MicroKind,
  density: Density,
}

impl Entity for MicroEnt {
  type Kind = MicroKind;
  type Def = MicroDef;
  type Quantity = Density;

  fn kind(&self) -> MicroKind {
    self.kind
  }

  fn def(&self) -> &'static MicroDef {
    self.kind.def()
  }

  fn quantity(&self) -> Density {
    self.density
  }

  fn set_quantity(&mut self, quantity: Density) {
    self.density = quantity;
  }
}

impl MicroEnt {
  pub const fn new(kind: MicroKind, density: Density) -> Self {
    Self { kind, density }
  }
}

impl Pressurized for MicroEnt {
  fn pressure(&self) -> f64 {
    let cohesion = self.def().cohesion;
    self.density.value().powf(cohesion)
  }
}
