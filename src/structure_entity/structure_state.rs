use crate::structure_entity::StructureKind;
use crate::structure_entity::StructureThink;
use crate::structure_entity::player_defs;
use crate::structure_entity::populance_defs;
use crate::structure_entity::structure_think::StructureThinkArgs;

#[derive(Clone, Copy)]
pub enum StructureState {
  Artery(player_defs::ArteryStructure),
  BuildingPart(player_defs::BuildingPartStructure),
  Infrastructure(player_defs::InfrastructureStructure),
  MicroFactory(player_defs::MicroFactoryStructure),
  Wall(player_defs::WallStructure),
  Field(populance_defs::FieldStructure),
}

impl StructureState {
  pub fn new(kind: StructureKind) -> Self {
    match kind {
      StructureKind::Artery => StructureState::Artery(player_defs::ArteryStructure::default()),
      StructureKind::BuildingPart => {
        StructureState::BuildingPart(player_defs::BuildingPartStructure::default())
      }
      StructureKind::Infrastructure => {
        StructureState::Infrastructure(player_defs::InfrastructureStructure::default())
      }
      StructureKind::MicroFactory => {
        StructureState::MicroFactory(player_defs::MicroFactoryStructure::default())
      }
      StructureKind::Wall => StructureState::Wall(player_defs::WallStructure::default()),
      StructureKind::Field => StructureState::Field(populance_defs::FieldStructure::default()),
    }
  }

  pub const fn kind(&self) -> StructureKind {
    match self {
      StructureState::Artery(_) => StructureKind::Artery,
      StructureState::BuildingPart(_) => StructureKind::BuildingPart,
      StructureState::Infrastructure(_) => StructureKind::Infrastructure,
      StructureState::MicroFactory(_) => StructureKind::MicroFactory,
      StructureState::Wall(_) => StructureKind::Wall,
      StructureState::Field(_) => StructureKind::Field,
    }
  }
}

impl StructureThink for StructureState {
  fn think(&self, args: StructureThinkArgs) {
    match self {
      StructureState::Infrastructure(s) => s.think(args),
      _ => (),
    }
  }
}
