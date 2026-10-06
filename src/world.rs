use crate::commander_entity::CommanderCollection;
use crate::geometry::grid::Grid;
use crate::micro_entity::MicroCollection;
use crate::structure_entity::StructureCollection;

pub struct World {
  grid: Grid,
  micros: MicroCollection,
  structures: StructureCollection,
  commanders: CommanderCollection,
}

pub struct WorldMut<'a> {
  pub grid: Grid,
  pub micros: &'a mut MicroCollection,
  pub structures: &'a mut StructureCollection,
  pub commanders: &'a mut CommanderCollection,
}

impl World {
  pub fn new(grid: Grid) -> Self {
    Self {
      grid,
      micros: MicroCollection::new(grid),
      structures: StructureCollection::new(grid),
      commanders: CommanderCollection::new(grid),
    }
  }

  pub fn grid(&self) -> Grid {
    self.grid
  }

  pub fn mutable(&mut self) -> WorldMut<'_> {
    WorldMut {
      grid: self.grid,
      micros: &mut self.micros,
      structures: &mut self.structures,
      commanders: &mut self.commanders,
    }
  }

  pub fn micros(&self) -> &MicroCollection {
    &self.micros
  }

  pub fn micros_mut(&mut self) -> &mut MicroCollection {
    &mut self.micros
  }

  pub fn structures(&self) -> &StructureCollection {
    &self.structures
  }

  pub fn structures_mut(&mut self) -> &mut StructureCollection {
    &mut self.structures
  }

  pub fn commanders(&self) -> &CommanderCollection {
    &self.commanders
  }

  pub fn commanders_mut(&mut self) -> &mut CommanderCollection {
    &mut self.commanders
  }
}
