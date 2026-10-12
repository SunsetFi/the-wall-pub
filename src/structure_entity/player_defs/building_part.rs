use macros::structure_ent;

use crate::{
  alignment::Alignment, material::Material, permeability::Permeability,
  structure_entity::StructureState,
};

#[structure_ent(
  alignment = Alignment::Player,
  material = Material::Concrete,
  permeability = Permeability::Infrastructure,
  toughness = 200.0,
  construction_density = 0.1,
)]
#[derive(Clone, Copy, Default)]
pub struct BuildingPartStructure {}

impl From<BuildingPartStructure> for StructureState {
  fn from(building_part: BuildingPartStructure) -> StructureState {
    StructureState::BuildingPart(building_part)
  }
}
