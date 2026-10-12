use std::cmp;

use crate::{
  micro_entity::MicroCollection,
  structure_entity::{
    StructureEnt, StructureThink,
    structure_slot::StructureSlot,
    structure_think::{StructureThinkArgs, StructureUpdate, StructureUpdateCollection},
  },
};

pub type StructureCollection = crate::grid_collection::GridCollection<StructureSlot>;

impl StructureCollection {
  pub fn think_all(&mut self, micros: &mut MicroCollection, current_tick: u64) {
    let mut updates = StructureUpdateCollection::new();
    let grid = self.grid();
    for (idx, structure) in self.iter_enumerate() {
      let Some(state) = structure.state() else {
        continue;
      };
      state.think(StructureThinkArgs {
        grid,
        self_idx: idx,
        current_tick,
        structures: self,
        micros,
        updates: &mut updates,
      });
    }

    for (idx, update) in updates.drain() {
      let slot = self.get_mut(idx);
      match update {
        StructureUpdate::Mutate(state_update) => {
          if let Some(state) = slot.state_mut() {
            state.apply(state_update);
          }
        }
        StructureUpdate::Replace(state, integrity) => {
          let integrity = integrity
            .or_else(|| {
              slot
                .integrity()
                .map(|i| cmp::min(i, state.kind().def().toughness))
            })
            .unwrap_or_else(|| state.kind().def().toughness);

          slot.set_ent(StructureEnt::new(state, integrity));
        }
        StructureUpdate::Destroy => {
          slot.clear();
        }
      }
    }
  }
}
