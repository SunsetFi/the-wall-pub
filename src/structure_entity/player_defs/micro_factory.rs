use macros::structure_ent;

use crate::{alignment::Alignment, material::Material, permeability::Permeability};

#[structure_ent(
  alignment = Alignment::Player,
  material = Material::StructuralMetal,
  permeability = Permeability::Infrastructure,
  toughness = 400.0,
  construction_density = 3.0,
)]
#[derive(Clone, Copy, Default)]
pub struct MicroFactoryStructure {}
