use enum_iterator::{Sequence, cardinality};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::structure_entity::{StructureDef, StructureType, player_defs, populance_defs};

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive, Sequence, Debug)]
#[repr(u8)]
pub enum StructureKind {
  // Player
  Artery,
  BuildingPart,
  Infrastructure,
  MicroFactory,
  Wall,
  // Populance
  Field,
}

impl StructureKind {
  pub const COUNT: usize = cardinality::<StructureKind>();

  pub const fn def(self) -> &'static StructureDef {
    match self {
      StructureKind::Artery => &player_defs::ArteryStructure::DEF,
      StructureKind::BuildingPart => &player_defs::BuildingPartStructure::DEF,
      StructureKind::Infrastructure => &player_defs::InfrastructureStructure::DEF,
      StructureKind::MicroFactory => &player_defs::MicroFactoryStructure::DEF,
      StructureKind::Wall => &player_defs::WallStructure::DEF,
      StructureKind::Field => &populance_defs::FieldStructure::DEF,
    }
  }

  pub fn type_id(self) -> &'static str {
    match self {
      StructureKind::Artery => "Artery",
      StructureKind::BuildingPart => "BuildingPart",
      StructureKind::Infrastructure => "Infrastructure",
      StructureKind::MicroFactory => "MicroFactory",
      StructureKind::Wall => "Wall",
      StructureKind::Field => "Field",
    }
  }

  pub fn from_type_id(type_id: &str) -> Option<StructureKind> {
    match type_id {
      "Artery" => Some(StructureKind::Artery),
      "BuildingPart" => Some(StructureKind::BuildingPart),
      "Infrastructure" => Some(StructureKind::Infrastructure),
      "MicroFactory" => Some(StructureKind::MicroFactory),
      "Wall" => Some(StructureKind::Wall),
      "Field" => Some(StructureKind::Field),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use enum_iterator::{first, next};

  use super::StructureKind;

  #[test]
  fn it_round_trips_type_ids() {
    let mut kind = first::<StructureKind>();
    while let Some(current_kind) = kind {
      let type_id = current_kind.type_id();
      let round_tripped = StructureKind::from_type_id(type_id).unwrap();
      assert_eq!(current_kind, round_tripped);
      kind = next(&current_kind);
    }
  }
}
