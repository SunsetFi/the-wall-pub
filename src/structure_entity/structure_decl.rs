use crate::structure_entity::{
  StructureDef, StructureThink, StructureType, player_defs, populance_defs,
  structure_think::{StructureApply, StructureThinkArgs},
};
use enum_iterator::{Sequence, cardinality};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use wasm_bindgen::prelude::wasm_bindgen;

macro_rules! structure_declare {
  ( $( $variant:ident ($state:ty) $( => $update:ty)? ),* $(,)?) => {

    #[wasm_bindgen]
    #[derive(Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive, Sequence, Debug)]
    #[repr(u8)]
    pub enum StructureKind {
      $( $variant, )*
    }

    impl StructureKind {
      pub const COUNT: usize = cardinality::<StructureKind>();

      pub const fn def(self) -> &'static StructureDef {
        match self {
          $( Self::$variant => &<$state>::DEF, )*
        }
      }

      pub const fn type_id(self) -> &'static str {
        match self {
          $( Self::$variant => stringify!($variant), )*
        }
      }

      pub fn from_type_id(type_id: &str) -> Option<Self> {
        match type_id {
          $( stringify!($variant) => Some(Self::$variant), )*
          _ => None,
        }
      }
    }

    pub enum StructureState {
      $( $variant($state), )*
    }

    impl StructureState {
      pub fn new(kind: StructureKind) -> Self {
        match kind {
          $(
            StructureKind::$variant => <$state>::default().into(),
          )*
        }
      }

      pub const fn kind(&self) -> StructureKind {
        match self {
          $(Self::$variant(_) => StructureKind::$variant, )*
        }
      }
    }

    pub enum StructureStateUpdate {
      $( $( $variant($update), )? )*
    }

    impl StructureState {
      pub fn apply(&mut self, update: StructureStateUpdate) {
        match (self, update) {
          $(
            $(
              (Self::$variant(state), StructureStateUpdate::$variant(update)) => {
                let update: $update = update;
                state.apply(update);
              }
            )?
          )*
          _ => {}
        }
      }
    }

    $(
      $(
        impl From<$update> for StructureStateUpdate {
          fn from(update: $update) -> Self {
            Self::$variant(update)
          }
        }
      )?
    )*
  };
}

structure_declare! {
  Artery(player_defs::ArteryStructure) => player_defs::ArteryUpdate,
  BuildingPart(player_defs::BuildingPartStructure),
  Infrastructure(player_defs::InfrastructureStructure) => player_defs::InfrastructureUpdate,
  MicroFactory(player_defs::MicroFactoryStructure) => player_defs::MicroFactoryUpdate,
  Wall(player_defs::WallStructure),
  Field(populance_defs::FieldStructure),
}

impl StructureThink for StructureState {
  fn think(&self, args: StructureThinkArgs) {
    match self {
      Self::Infrastructure(s) => s.think(args),
      Self::MicroFactory(s) => s.think(args),
      Self::Artery(s) => s.think(args),
      _ => (),
    }
  }
}

#[cfg(test)]
mod tests {
  mod structure_kind {
    use enum_iterator::{first, next};

    use super::super::StructureKind;

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
}
