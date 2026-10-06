use crate::{
  alignment::Alignment,
  commander_entity::{CommanderKind, commander_def::CommanderDef, commander_type::CommanderType},
};

pub struct PlayerCommander {}

const PLAYER_COMMANDER_DEF: CommanderDef = CommanderDef {
  kind: CommanderKind::FriendlyCommander,
  alignment: Alignment::Player,
};

impl CommanderType for PlayerCommander {
  const DEF: CommanderDef = PLAYER_COMMANDER_DEF;
}
