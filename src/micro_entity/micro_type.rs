use crate::micro_entity::MicroDef;

pub trait MicroType {
  const DEF: MicroDef;
}
