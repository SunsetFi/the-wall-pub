use super::{Density, MicroDef, MicroKind};
use crate::fluid::Pressurized;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MicroEnt {
  kind: MicroKind,
  density: Density,
}

impl MicroEnt {
  pub const fn new(kind: MicroKind, density: Density) -> Self {
    Self { kind, density }
  }

  pub const fn def(self) -> &'static MicroDef {
    self.kind.def()
  }

  pub fn kind(&self) -> MicroKind {
    self.kind
  }

  pub fn density(&self) -> Density {
    self.density
  }

  pub fn set_density(&mut self, density: Density) {
    self.density = density;
  }
}

impl Pressurized for MicroEnt {
  fn pressure(&self) -> f64 {
    let cohesion = self.def().cohesion;
    self.density.value().powf(cohesion)
  }
}
