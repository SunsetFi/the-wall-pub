use crate::geometry::grid::Grid;
use crate::micro_entity::MicroCollection;
use crate::structure_entity::structure_think::StructureThinkArgs;
use crate::structure_entity::structure_think::StructureUpdate;
use crate::structure_entity::structure_think::StructureUpdates;

pub type StructureCollection = crate::grid_collection::GridCollection<super::StructureSlot>;

impl StructureCollection {
  pub fn think_all(&mut self, grid: Grid, micros: &mut MicroCollection) {
    let mut updates = StructureUpdates::new();

    for (idx, structure) in self.iter_enumerate() {
      structure.think(StructureThinkArgs {
        grid,
        self_idx: idx,
        structures: self,
        micros,
        updates: &mut updates,
      });
    }

    for (idx, update) in updates.iter() {
      match update {
        StructureUpdate::Replace(state) => {
          self.get_mut(idx).set(*state, None);
        }
        StructureUpdate::Destroy => {
          self.get_mut(idx).clear();
        }
      }
    }
  }
}
