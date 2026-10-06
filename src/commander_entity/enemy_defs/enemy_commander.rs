use crate::{
  alignment::Alignment,
  commander_entity::{CommanderKind, commander_def::CommanderDef, commander_type::CommanderType},
};

pub struct EnemyCommander {}

const ENEMY_COMMANDER_DEF: CommanderDef = CommanderDef {
  kind: CommanderKind::EnemyCommander,
  alignment: Alignment::Enemy,
};

impl CommanderType for EnemyCommander {
  const DEF: CommanderDef = ENEMY_COMMANDER_DEF;
}
