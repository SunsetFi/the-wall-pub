use macros::structure_ent;

use crate::{alignment::Alignment, material::Material, permeability::Permeability};

#[structure_ent(
  alignment = Alignment::Populance,
  material = Material::Organic,
  permeability = Permeability::Infrastructure,
  toughness = 6800.0,
)]
#[derive(Clone, Copy, Default)]
pub struct FieldStructure {}
