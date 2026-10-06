use crate::commander_entity::commander_def::CommanderDef;

pub trait CommanderType {
  const DEF: CommanderDef;
}
