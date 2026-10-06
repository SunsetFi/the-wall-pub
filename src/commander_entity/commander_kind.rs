use enum_iterator::{Sequence, cardinality};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::commander_entity::{commander_def::CommanderDef, enemy_defs, player_defs, commander_type::CommanderType};

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive, Sequence, Debug)]
#[repr(u8)]
pub enum CommanderKind {
  FriendlyCommander = 0,
  EnemyCommander = 1,
}

impl CommanderKind {
  pub const COUNT: usize = cardinality::<CommanderKind>();

  pub const fn def(self) -> &'static CommanderDef {
    match self {
      CommanderKind::FriendlyCommander => &player_defs::PlayerCommander::DEF,
      CommanderKind::EnemyCommander => &enemy_defs::EnemyCommander::DEF,
    }
  }

  pub fn type_id(self) -> &'static str {
    match self {
      CommanderKind::FriendlyCommander => "FriendlyCommander",
      CommanderKind::EnemyCommander => "EnemyCommander",
    }
  }

  pub fn from_type_id(type_id: &str) -> Option<CommanderKind> {
    match type_id {
      "FriendlyCommander" => Some(CommanderKind::FriendlyCommander),
      "EnemyCommander" => Some(CommanderKind::EnemyCommander),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use enum_iterator::{first, next};

  use super::CommanderKind;

  #[test]
  fn it_round_trips_type_ids() {
    let mut kind = first::<CommanderKind>();
    while let Some(current_kind) = kind {
      let type_id = current_kind.type_id();
      let round_tripped = CommanderKind::from_type_id(type_id).unwrap();
      assert_eq!(current_kind, round_tripped);
      kind = next(&current_kind);
    }
  }
}
