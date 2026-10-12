use std::collections::BTreeMap;

use crate::blob_field::{BlobField, FieldLabel};
use crate::commander_entity::commander_think::CommanderThinkResult;
use crate::commander_entity::{CommanderEnt, CommanderKind};
use crate::geometry::Coordinate;
use crate::geometry::grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CommanderIdx(u32);

impl From<CommanderIdx> for u32 {
  fn from(index: CommanderIdx) -> Self {
    index.0 as u32
  }
}

impl From<u32> for CommanderIdx {
  fn from(index: u32) -> Self {
    CommanderIdx(index)
  }
}

pub struct CommanderCollection {
  grid: Grid,
  generation: u32,
  commanders: BTreeMap<CommanderIdx, CommanderEnt>,
  commander_by_slot: Vec<CommanderIdx>,
  head_by_label: Vec<Option<usize>>,
  next_by_slot: Vec<Option<usize>>,
  next_id: u32,
}

impl CommanderCollection {
  pub fn new(grid: Grid) -> Self {
    Self {
      grid,
      generation: 0,
      commanders: BTreeMap::new(),
      commander_by_slot: Vec::new(),
      head_by_label: Vec::new(),
      next_by_slot: Vec::new(),
      next_id: 0,
    }
  }

  pub fn generation(&self) -> u32 {
    self.generation
  }

  pub fn add(
    &mut self,
    kind: CommanderKind,
    pos: Coordinate,
    goal: Coordinate,
  ) -> Option<CommanderIdx> {
    if !self.grid.contains_coordinate(pos) {
      return None;
    }

    let commander = CommanderEnt {
      id: self.next_id,
      kind,
      pos,
      goal,
    };
    self.next_id += 1;
    let idx = CommanderIdx(self.next_id);
    self.commanders.insert(idx, commander);
    self.next_id += 1;
    self.generation += 1;
    Some(idx)
  }

  pub fn remove(&mut self, index: CommanderIdx) -> bool {
    let removed = self.commanders.remove(&index).is_some();
    if removed {
      self.generation += 1;
    }
    removed
  }

  pub fn get(&self, idx: CommanderIdx) -> Option<&CommanderEnt> {
    self.commanders.get(&idx)
  }

  pub fn get_mut(&mut self, idx: CommanderIdx) -> Option<&mut CommanderEnt> {
    self.commanders.get_mut(&idx)
  }

  pub fn commanders_in(&self, label: FieldLabel) -> impl Iterator<Item = &CommanderEnt> + '_ {
    let head = self.head_by_label.get(label.index()).copied().flatten();
    std::iter::successors(head, |&slot| self.next_by_slot[slot])
      .map(move |slot| self.commanders.get(&self.commander_by_slot[slot]))
      .filter(Option::is_some)
      .map(Option::unwrap)
  }

  pub fn assign_blobs(&mut self, blobs: &BlobField) {
    let label_size = blobs.count() as usize;

    self.head_by_label.clear();
    self.next_by_slot.clear();
    self.commander_by_slot.clear();
    self.head_by_label.resize(label_size, None);

    for (idx, commander) in self.commanders.iter() {
      let Some(label) = blobs.label_at_cell(commander.pos.x, commander.pos.y) else {
        continue;
      };

      let Some(kind) = blobs.kind_of(label) else {
        continue;
      };

      if kind.def().alignment != commander.def().alignment {
        continue;
      }

      let slot = self.commander_by_slot.len();
      let label_index = label.index();
      self.commander_by_slot.push(*idx);
      self.next_by_slot.push(self.head_by_label[label_index]);
      self.head_by_label[label_index] = Some(slot);
    }
  }

  pub fn think_all(&mut self, blobs: &BlobField, tick: u64) {
    let mut drop_indexes = Vec::<CommanderIdx>::new();
    for (idx, commander) in self.commanders.iter_mut() {
      match commander.think(self.grid, blobs, tick) {
        CommanderThinkResult::Ok => {}
        CommanderThinkResult::DeleteSelf => {
          drop_indexes.push(*idx);
        }
      }
    }

    let drop_count = drop_indexes.len();
    if drop_count > 0 {
      for i in 0..drop_count {
        self.commanders.remove(&drop_indexes[i]);
      }
      self.generation += 1;
    }
  }
}
