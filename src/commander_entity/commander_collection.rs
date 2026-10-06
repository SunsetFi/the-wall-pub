use crate::blob_field::{BlobField, FieldLabel};
use crate::commander_entity::commander_think::CommanderThinkResult;
use crate::commander_entity::{CommanderEnt, CommanderKind};
use crate::geometry::Coordinate;
use crate::geometry::grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CommanderIdx(usize);

impl From<CommanderIdx> for u32 {
  fn from(index: CommanderIdx) -> Self {
    index.0 as u32
  }
}

pub struct CommanderCollection {
  grid: Grid,
  generation: u32,
  commanders: Vec<CommanderEnt>,
  head_by_label: Vec<Option<usize>>,
  next_by_slot: Vec<Option<usize>>,
  commander_by_slot: Vec<usize>,
  next_id: u32,
}

impl CommanderCollection {
  pub fn new(grid: Grid) -> Self {
    Self {
      grid,
      generation: 0,
      commanders: Vec::new(),
      head_by_label: Vec::new(),
      next_by_slot: Vec::new(),
      commander_by_slot: Vec::new(),
      next_id: 0,
    }
  }

  pub fn generation(&self) -> u32 {
    self.generation
  }

  pub fn idx(&self, index: u32) -> Option<CommanderIdx> {
    if (index as usize) < self.commanders.len() {
      Some(CommanderIdx(index as usize))
    } else {
      None
    }
  }

  pub fn add(&mut self, kind: CommanderKind, pos: Coordinate, goal: Coordinate) -> CommanderIdx {
    let commander = CommanderEnt {
      id: self.next_id,
      kind,
      pos,
      goal,
    };
    self.next_id += 1;
    self.commanders.push(commander);
    self.generation += 1;
    self.idx_unchecked(self.commanders.len() - 1)
  }

  pub fn remove(&mut self, index: CommanderIdx) -> bool {
    self.commanders.remove(index.0);
    self.generation += 1;
    true
  }

  pub fn get(&self, index: CommanderIdx) -> Option<&CommanderEnt> {
    self.commanders.get(index.0)
  }

  pub fn get_by_id(&self, id: u32) -> Option<(CommanderIdx, &CommanderEnt)> {
    self
      .commanders
      .iter()
      .enumerate()
      .find(|(_, commander)| commander.id == id)
      .map(|(i, commander)| (self.idx_unchecked(i), commander))
  }

  pub fn get_mut(&mut self, index: CommanderIdx) -> Option<&mut CommanderEnt> {
    self.commanders.get_mut(index.0)
  }

  pub fn commanders_in(&self, label: FieldLabel) -> impl Iterator<Item = &CommanderEnt> + '_ {
    let head = self.head_by_label.get(label.index()).copied().flatten();
    std::iter::successors(head, |&slot| self.next_by_slot[slot])
      .map(move |slot| &self.commanders[self.commander_by_slot[slot]])
  }

  pub fn assign_blobs(&mut self, blobs: &BlobField) {
    let label_size = blobs.count() as usize;

    self.head_by_label.clear();
    self.next_by_slot.clear();
    self.commander_by_slot.clear();
    self.head_by_label.resize(label_size, None);

    for (i, commander) in self.commanders.iter().enumerate() {
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
      self.commander_by_slot.push(i);
      self.next_by_slot.push(self.head_by_label[label_index]);
      self.head_by_label[label_index] = Some(slot);
    }
  }

  pub fn think_all(&mut self, blobs: &BlobField, tick: u64) {
    let mut drop_indexes = Vec::<usize>::new();
    for (i, commander) in self.commanders.iter_mut().enumerate() {
      match commander.think(self.grid, blobs, tick) {
        CommanderThinkResult::Ok => {}
        CommanderThinkResult::DeleteSelf => {
          drop_indexes.push(i);
        }
      }
    }

    for i in (0..drop_indexes.len()).rev() {
      self.commanders.swap_remove(drop_indexes[i]);
    }

    self.generation += 1;
  }

  fn idx_unchecked(&self, index: usize) -> CommanderIdx {
    CommanderIdx(index)
  }
}
