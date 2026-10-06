use macros::structure_ent;

use crate::{alignment::Alignment, material::Material, permeability::Permeability};

#[structure_ent(
  alignment = Alignment::Player,
  material = Material::Concrete,
  permeability = Permeability::Infrastructure,
  toughness = 200.0,
  construction_density = 0.1,
)]
#[derive(Clone, Copy, Default)]
pub struct BuildingPartStructure {}
