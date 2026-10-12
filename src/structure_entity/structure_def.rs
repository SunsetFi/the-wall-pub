use crate::alignment::Alignment;
use crate::material::Material;
use crate::permeability::Permeability;
use crate::structure_entity::{Integrity, StructureKind};

pub struct StructureDef {
  pub kind: StructureKind,
  pub alignment: Alignment,
  pub material: Material,
  pub permeability: Permeability,
  pub toughness: Integrity,
  pub construction_density: Option<f64>,
}
