use crate::micro_entity::{MicroDef, MicroType, enemy_defs, player_defs};
use enum_iterator::{Sequence, cardinality};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive, Sequence, Debug)]
#[repr(u8)]
pub enum MicroKind {
  Friendly = 0,
  StructureConstructor = 1,
  Enemy = 2,
}

impl MicroKind {
  pub const COUNT: usize = cardinality::<MicroKind>();

  pub const fn def(self) -> &'static MicroDef {
    match self {
      MicroKind::Friendly => &player_defs::FriendlyMicro::DEF,
      MicroKind::StructureConstructor => &player_defs::StructureConstructorMicro::DEF,
      MicroKind::Enemy => &enemy_defs::EnemyMicro::DEF,
    }
  }

  pub fn type_id(self) -> &'static str {
    match self {
      MicroKind::Friendly => "Friendly",
      MicroKind::StructureConstructor => "StructureConstructor",
      MicroKind::Enemy => "Enemy",
    }
  }

  pub fn from_type_id(type_id: &str) -> Option<MicroKind> {
    match type_id {
      "Friendly" => Some(MicroKind::Friendly),
      "StructureConstructor" => Some(MicroKind::StructureConstructor),
      "Enemy" => Some(MicroKind::Enemy),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use enum_iterator::{first, next};

  use super::MicroKind;

  #[test]
  fn it_round_trips_type_ids() {
    let mut kind = first::<MicroKind>();
    while let Some(current_kind) = kind {
      let type_id = current_kind.type_id();
      let round_tripped = MicroKind::from_type_id(type_id).unwrap();
      assert_eq!(current_kind, round_tripped);
      kind = next(&current_kind);
    }
  }
}
