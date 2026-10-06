use crate::structure_entity::StructureDef;

pub trait StructureType {
  const DEF: StructureDef;
}
