use macros::structure_ent;

use crate::{
  alignment::Alignment, material::Material, permeability::Permeability,
  structure_entity::StructureState,
};

#[structure_ent(
  alignment = Alignment::Player,
  material = Material::Concrete,
  permeability = Permeability::Solid,
  toughness = 6800.0,
)]
#[derive(Clone, Copy, Default)]
pub struct WallStructure {}

impl From<WallStructure> for StructureState {
  fn from(wall: WallStructure) -> StructureState {
    StructureState::Wall(wall)
  }
}
