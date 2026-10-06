use crate::alignment::Alignment;
use crate::material::Material;
use crate::permeability::Permeability;
use crate::structure_entity::StructureKind;

pub struct StructureDef {
  pub kind: StructureKind,
  pub alignment: Alignment,
  pub material: Material,
  pub permeability: Permeability,
  pub toughness: f64,
  pub construction_density: Option<f64>,
}
