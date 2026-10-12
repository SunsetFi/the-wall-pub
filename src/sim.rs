use crate::blob_field::{BlobField, FieldLabel};
use crate::combat;
use crate::commander_entity::{CommanderIdx, CommanderKind};
use crate::entity::Entity;
use crate::fluid::FlowProcessor;
use crate::geometry::Coordinate;
use crate::js_error::JsError;
use crate::micro_entity::{MicroEnt, MicroKind, MicroSlot};
use crate::structure_entity::{
  Integrity, StructureEnt, StructureKind, StructureSlot, StructureState,
};
use crate::world::World;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::geometry::grid::{CellIdx, Grid};
use crate::micro_entity::Density;

#[wasm_bindgen]
pub struct Sim {
  /// The world map, containing all entities.
  world: World,
  /// Pre-processed micro blobs.
  blobs: BlobField,
  /// Processing logic for micro flow.
  flow: FlowProcessor,
  /// The current tick while processing.
  /// The next tick while idle.
  tick: u64,
}

#[wasm_bindgen]
impl Sim {
  #[wasm_bindgen(constructor)]
  pub fn construct(width: u32, height: u32) -> Sim {
    Sim::new(width, height)
  }
}

/// Micro entities
#[wasm_bindgen]
impl Sim {
  pub fn micro_kind(&self, ord: u32) -> Result<Option<MicroKind>, JsError> {
    let slot = self.micro_slot(ord)?;
    Ok(slot.kind())
  }

  pub fn micro_density(&self, ord: u32) -> Result<f64, JsError> {
    let micro = self.micro_ent(ord)?;
    Ok(micro.quantity().value())
  }

  pub fn set_micro(&mut self, ord: u32, kind: MicroKind, density: f64) -> Result<(), JsError> {
    let slot = self.micro_slot_mut(ord)?;
    let Some(density) = Density::new(density) else {
      slot.clear();
      return Ok(());
    };

    slot.set_ent(MicroEnt::new(kind, density));
    Ok(())
  }

  pub fn set_micro_density(&mut self, ord: u32, density: f64) -> Result<bool, JsError> {
    let slot = self.micro_slot_mut(ord)?;
    if let Some(density) = Density::new(density) {
      slot.set_density(density);
    } else {
      slot.clear();
    }

    Ok(true)
  }

  pub fn clear_micro(&mut self, ord: u32) -> Result<(), JsError> {
    let idx = self.cell_idx(ord)?;
    self.world.micros_mut().get_mut(idx).clear();
    Ok(())
  }
}

/// micro shared internals
impl Sim {
  pub fn micro_slot(&self, ord: u32) -> Result<&MicroSlot, JsError> {
    let idx = self.cell_idx(ord)?;
    Ok(self.world.micros().get(idx))
  }

  pub fn micro_slot_mut(&mut self, ord: u32) -> Result<&mut MicroSlot, JsError> {
    let idx = self.cell_idx(ord)?;
    Ok(self.world.micros_mut().get_mut(idx))
  }

  pub fn micro_ent(&self, ord: u32) -> Result<&MicroEnt, JsError> {
    let idx = self.cell_idx(ord)?;
    if let Some(micro) = self.world.micros().get(idx).ent() {
      Ok(micro)
    } else {
      Err(JsError::EmptyOrdinal(ord))
    }
  }
}

/// Structure entities
#[wasm_bindgen]
impl Sim {
  pub fn add_structure(
    &mut self,
    ord: u32,
    kind: StructureKind,
    integrity: f64,
  ) -> Result<(), JsError> {
    let slot = self.structure_slot_mut(ord)?;
    let integrity = Integrity::new(integrity).ok_or(JsError::InvalidValue("Invalid integrity"))?;
    slot.set_ent(StructureEnt::new(StructureState::new(kind), integrity));
    Ok(())
  }

  pub fn remove_structure(&mut self, ord: u32) -> Result<bool, JsError> {
    let slot = self.structure_slot_mut(ord)?;
    Ok(slot.clear())
  }

  pub fn structure_kind(&self, ord: u32) -> Result<Option<StructureKind>, JsError> {
    let slot = self.structure_slot(ord)?;
    Ok(slot.kind())
  }

  pub fn structure_integrity(&self, ord: u32) -> Result<Option<f64>, JsError> {
    let slot = self.structure_slot(ord)?;
    Ok(slot.integrity().map(From::<Integrity>::from))
  }

  pub fn set_structure_integrity(&mut self, ord: u32, integrity: f64) -> Result<bool, JsError> {
    let slot = self.structure_slot_mut(ord)?;
    let integrity = Integrity::new(integrity);
    let Some(integrity) = integrity else {
      return Ok(false);
    };

    Ok(slot.set_integrity(integrity))
  }
}

/// Structure shared internals
impl Sim {
  pub fn structure_slot(&self, ord: u32) -> Result<&StructureSlot, JsError> {
    let idx = self.cell_idx(ord)?;
    Ok(self.world.structures().get(idx))
  }

  pub fn structure_slot_mut(&mut self, ord: u32) -> Result<&mut StructureSlot, JsError> {
    let idx = self.cell_idx(ord)?;
    Ok(self.world.structures_mut().get_mut(idx))
  }

  pub fn structure_state(&self, ord: u32) -> Result<&StructureState, JsError> {
    let slot = self.structure_slot(ord)?;
    if let Some(structure) = slot.state() {
      Ok(structure)
    } else {
      Err(JsError::EmptyOrdinal(ord))
    }
  }

  pub fn structure_state_mut(&mut self, ord: u32) -> Result<&mut StructureState, JsError> {
    let slot = self.structure_slot_mut(ord)?;
    if let Some(structure) = slot.state_mut() {
      Ok(structure)
    } else {
      Err(JsError::EmptyOrdinal(ord))
    }
  }
}

/// Commander entities
#[wasm_bindgen]
impl Sim {
  pub fn commander_generation(&self) -> u32 {
    self.world.commanders().generation()
  }

  pub fn add_commander(
    &mut self,
    kind: CommanderKind,
    x: u32,
    y: u32,
    gx: u32,
    gy: u32,
  ) -> Result<u32, JsError> {
    self
      .world
      .commanders_mut()
      .add(kind, Coordinate { x, y }, Coordinate { x: gx, y: gy })
      .ok_or(JsError::CoordinateOutOfBounds(Coordinate { x, y }))
      .map(u32::from)
  }

  pub fn remove_commander(&mut self, index: u32) -> bool {
    self
      .world
      .commanders_mut()
      .remove(CommanderIdx::from(index))
  }

  pub fn commander_kind(&self, index: u32) -> Option<CommanderKind> {
    self
      .world
      .commanders()
      .get(CommanderIdx::from(index))
      .map(|f| f.kind)
  }

  pub fn commander_x(&self, index: u32) -> Option<u32> {
    self
      .world
      .commanders()
      .get(CommanderIdx::from(index))
      .map(|f| f.pos.x)
  }

  pub fn commander_y(&self, index: u32) -> Option<u32> {
    self
      .world
      .commanders()
      .get(CommanderIdx::from(index))
      .map(|f| f.pos.y)
  }

  pub fn set_commander_goal_x(&mut self, index: u32, gx: u32) -> Result<(), JsError> {
    if let Some(commander) = self
      .world
      .commanders_mut()
      .get_mut(CommanderIdx::from(index))
    {
      commander.goal.x = gx;
      Ok(())
    } else {
      Err(JsError::InvalidCommanderIndex(index))
    }
  }

  pub fn set_commander_goal_y(&mut self, index: u32, gy: u32) -> Result<(), JsError> {
    if let Some(commander) = self
      .world
      .commanders_mut()
      .get_mut(CommanderIdx::from(index))
    {
      commander.goal.y = gy;
      Ok(())
    } else {
      Err(JsError::InvalidCommanderIndex(index))
    }
  }
}

/// Blob field
#[wasm_bindgen]
impl Sim {
  pub fn blob_label_at_cell(&self, x: u32, y: u32) -> Option<u32> {
    let label = self.blobs.label_at_cell(x, y);
    label.map(|f| f.index() as u32)
  }

  pub fn blob_label_at_ordinal(&self, ord: u32) -> Option<u32> {
    self
      .blobs
      .label_at_ordinal(ord as usize)
      .map(|f| f.index() as u32)
  }

  pub fn mass_of(&self, label: u32) -> Result<f64, JsError> {
    let label = self.field_label(label)?;
    Ok(self.blobs.mass_of(label) as f64)
  }

  pub fn centroid_x_of(&self, label: u32) -> Result<f64, JsError> {
    let label = self.field_label(label)?;
    Ok(self.blobs.centroid_x_of(label))
  }

  pub fn centroid_y_of(&self, label: u32) -> Result<f64, JsError> {
    let label = self.field_label(label)?;
    Ok(self.blobs.centroid_y_of(label))
  }

  pub fn cells_at_label(&self, label: u32) -> Result<u32, JsError> {
    let label = self.field_label(label)?;
    Ok(self.blobs.cells_at_label(label))
  }

  pub fn cell_count_of(&self, label: u32) -> Result<u32, JsError> {
    let label = self.field_label(label)?;
    Ok(self.blobs.cells_at_label(label))
  }

  pub fn kind_of(&self, label: u32) -> Result<Option<MicroKind>, JsError> {
    let label = self.field_label(label)?;
    Ok(self.blobs.kind_of(label))
  }
}

/// Blob field utils
impl Sim {
  fn field_label(&self, ord: u32) -> Result<FieldLabel, JsError> {
    self
      .blobs
      .label(ord)
      .ok_or(JsError::FieldLabelNotFound(ord))
  }
}

/// Ticks
#[wasm_bindgen]
impl Sim {
  pub fn link(&mut self) {
    self.blobs.process(self.world.micros());
  }

  pub fn tick(&mut self) {
    let world_mut = &mut self.world.mutable();
    self.blobs.process(world_mut.micros);
    world_mut.commanders.assign_blobs(&self.blobs);
    self.flow.tick(world_mut, &self.blobs, self.tick);
    world_mut.commanders.think_all(&self.blobs, self.tick);
    combat::apply_combat(world_mut);
    world_mut.structures.think_all(world_mut.micros, self.tick);
    self.tick += 1;
  }
}

/// Utilities
impl Sim {
  pub fn new(width: u32, height: u32) -> Sim {
    let grid = Grid::new(width, height);
    let world = World::new(grid);
    let blobs = BlobField::new(grid);
    let flow = FlowProcessor::new(grid);
    Sim {
      world,
      blobs,
      flow,
      tick: 0,
    }
  }

  fn cell_idx(&self, ord: u32) -> Result<CellIdx, JsError> {
    self
      .world
      .grid()
      .ordinal(ord)
      .ok_or(JsError::CellOrdinalOutOfBounds(ord))
  }

  pub fn blobs(&self) -> &BlobField {
    &self.blobs
  }
}
