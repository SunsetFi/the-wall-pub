use macros::structure_ent;

use crate::{
  alignment::Alignment, material::Material, permeability::Permeability,
  structure_entity::StructureState,
};

#[structure_ent(
  alignment = Alignment::Populance,
  material = Material::Organic,
  permeability = Permeability::Infrastructure,
  toughness = 6800.0,
)]
#[derive(Clone, Copy, Default)]
pub struct FieldStructure {}

impl From<FieldStructure> for StructureState {
  fn from(field: FieldStructure) -> StructureState {
    StructureState::Field(field)
  }
}
