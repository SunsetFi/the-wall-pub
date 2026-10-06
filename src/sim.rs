use crate::blob_field::BlobField;
use crate::combat::CombatProcessor;
use crate::commander_entity::{CommanderIdx, CommanderKind};
use crate::fluid::FlowProcessor;
use crate::generational::Generational;
use crate::geometry::Coordinate;
use crate::js_error::JsError;
use crate::micro_entity::MicroKind;
use crate::structure_entity::{Integrity, StructureKind, StructureState, player_defs};
use crate::world::World;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::geometry::grid::{CellIdx, Grid};
use crate::micro_entity::Density;

#[wasm_bindgen]
pub struct Sim {
  /// The world map, containing micros and such
  world: World,
  /// Micro blobs
  blobs: BlobField,
  /// Processing logic for micro flow
  flow: FlowProcessor,
  /// Processing logic for combat
  // combat: CombatProcessor,
  tick: u64,
}

/// Micro entities
#[wasm_bindgen]
impl Sim {
  #[wasm_bindgen(constructor)]
  pub fn construct(width: u32, height: u32) -> Sim {
    Sim::new(width, height)
  }

  pub fn micro_kind(&self, ord: u32) -> Result<Option<MicroKind>, JsError> {
    let idx = self.cell_idx(ord)?;
    let micro = self.world.micros().get(idx);
    Ok(micro.kind())
  }

  pub fn micro_density(&self, ord: u32) -> Result<Option<f64>, JsError> {
    let idx = self.cell_idx(ord)?;
    let micro = self.world.micros().get(idx);
    Ok(micro.density().map(|m| m.value()))
  }

  pub fn set_micro(&mut self, ord: u32, kind: MicroKind, density: f64) -> Result<(), JsError> {
    let idx = self.cell_idx(ord)?;
    let Some(density) = Density::new(density) else {
      return Ok(());
    };

    self.world.micros_mut().get_mut(idx).set(kind, density);
    Ok(())
  }

  pub fn set_micro_density(&mut self, ord: u32, density: f64) -> Result<bool, JsError> {
    let idx = self.cell_idx(ord)?;

    let mut micro = self.world.micros_mut().get_mut(idx);
    if micro.is_empty() {
      return Ok(false);
    }

    if let Some(density) = Density::new(density) {
      micro.set_density(density);
    } else {
      micro.clear();
    }

    Ok(true)
  }

  pub fn clear_micro(&mut self, ord: u32) -> Result<(), JsError> {
    let idx = self.cell_idx(ord)?;
    self.world.micros_mut().get_mut(idx).clear();
    Ok(())
  }
}

/// Structure entities
#[wasm_bindgen]
impl Sim {
  pub fn structure_generation(&self) -> Result<u32, JsError> {
    Ok(self.world.structures().generation())
  }

  pub fn add_structure(
    &mut self,
    ord: u32,
    kind: StructureKind,
    integrity: f64,
  ) -> Result<(), JsError> {
    let idx = self.cell_idx(ord)?;
    let integrity = Integrity::new(integrity).ok_or(JsError::InvalidValue("Invalid integrity"))?;
    let mut structure = self.world.structures_mut().get_mut(idx);
    structure.set(StructureState::new(kind), Some(integrity));
    Ok(())
  }

  pub fn remove_structure(&mut self, ord: u32) -> Result<(), JsError> {
    let idx = self.cell_idx(ord)?;
    let mut structure = self.world.structures_mut().get_mut(idx);
    structure.clear();
    Ok(())
  }

  pub fn get_structure_kind(&self, ord: u32) -> Result<Option<StructureKind>, JsError> {
    let idx = self.cell_idx(ord)?;
    let structure = self.world.structures().get(idx);
    Ok(structure.kind())
  }

  pub fn get_structure_integrity(&self, ord: u32) -> Result<Option<f64>, JsError> {
    let idx = self.cell_idx(ord)?;
    let structure = self.world.structures().get(idx);
    Ok(structure.integrity().map(|m| m.value()))
  }

  pub fn set_structure_integrity(&mut self, ord: u32, integrity: f64) -> Result<bool, JsError> {
    let idx = self.cell_idx(ord)?;
    let integrity = Integrity::new(integrity);
    let Some(integrity) = integrity else {
      return Ok(false);
    };

    let mut structure = self.world.structures_mut().get_mut(idx);
    Ok(structure.set_integrity(integrity))
  }

  pub fn set_structure_infrastructure_construction(
    &mut self,
    ord: u32,
    kind: StructureKind,
    progress: f64,
  ) -> Result<bool, JsError> {
    let idx = self.cell_idx(ord)?;
    let mut structure = self.world.structures_mut().get_mut(idx);
    let Some(state) = structure.state() else {
      return Ok(false);
    };
    let StructureState::Infrastructure(state) = state else {
      Err(JsError::InvalidKind(
        "Set construction used on non-infrastructure",
      ))?
    };
    let Some(infrastructure) = player_defs::InfrastructureStructure::new_from_construction(kind)
    else {
      Err(JsError::InvalidKind("Kind is not constructable"))?
    };
    structure.set(StructureState::Infrastructure(infrastructure), None);
    Ok(true)
  }
}

/// Commander entities
#[wasm_bindgen]
impl Sim {
  pub fn commander_generation(&self) -> u32 {
    self.world.commanders().generation()
  }

  pub fn add_commander(&mut self, kind: CommanderKind, x: u32, y: u32, gx: u32, gy: u32) -> u32 {
    self
      .world
      .commanders_mut()
      .add(kind, Coordinate { x, y }, Coordinate { x: gx, y: gy })
      .into()
  }

  pub fn remove_commander(&mut self, index: u32) -> bool {
    let idx = match self.commander_idx(index) {
      Ok(idx) => idx,
      Err(_) => return false,
    };
    self.world.commanders_mut().remove(idx)
  }

  pub fn find_commander_by_id(&mut self, id: u32) -> Result<Option<u32>, JsError> {
    Ok(
      self
        .world
        .commanders()
        .get_by_id(id)
        .map(|(id, _)| id.into()),
    )
  }

  pub fn get_commander_id(&self, index: u32) -> Result<u32, JsError> {
    let idx = self.commander_idx(index)?;
    self
      .world
      .commanders()
      .get(idx)
      .map(|f| f.id)
      .ok_or(JsError::InvalidCommanderIndex(index))
  }

  pub fn get_commander_kind(&self, index: u32) -> Result<CommanderKind, JsError> {
    let idx = self.commander_idx(index)?;
    self
      .world
      .commanders()
      .get(idx)
      .map(|f| f.kind)
      .ok_or(JsError::InvalidCommanderIndex(index))
  }

  pub fn get_commander_x(&self, index: u32) -> Result<u32, JsError> {
    let idx = self.commander_idx(index)?;
    self
      .world
      .commanders()
      .get(idx)
      .map(|f| f.pos.x)
      .ok_or(JsError::InvalidCommanderIndex(index))
  }

  pub fn get_commander_y(&self, index: u32) -> Result<u32, JsError> {
    let idx = self.commander_idx(index)?;
    self
      .world
      .commanders()
      .get(idx)
      .map(|f| f.pos.y)
      .ok_or(JsError::InvalidCommanderIndex(index))
  }

  pub fn set_commander_goal_x(&mut self, index: u32, gx: u32) -> Result<(), JsError> {
    let idx = self.commander_idx(index)?;
    if let Some(commander) = self.world.commanders_mut().get_mut(idx) {
      commander.goal.x = gx;
      Ok(())
    } else {
      Err(JsError::InvalidCommanderIndex(index))
    }
  }

  pub fn set_commander_goal_y(&mut self, index: u32, gy: u32) -> Result<(), JsError> {
    let idx = self.commander_idx(index)?;
    if let Some(commander) = self.world.commanders_mut().get_mut(idx) {
      commander.goal.y = gy;
      Ok(())
    } else {
      Err(JsError::InvalidCommanderIndex(index))
    }
  }
}

/// Ticks
#[wasm_bindgen]
impl Sim {
  pub fn tick(&mut self) {
    let grid = self.world.grid();
    let world_mut = &mut self.world.mutable();
    self.blobs.process(&world_mut.micros);
    world_mut.commanders.assign_blobs(&self.blobs);
    self.flow.tick(world_mut, &self.blobs);
    world_mut.commanders.think_all(&self.blobs, self.tick);
    CombatProcessor::tick(world_mut);
    world_mut.structures.think_all(grid, world_mut.micros);
    self.tick += 1;
  }
}

/// Utilities
impl Sim {
  pub fn new(width: u32, height: u32) -> Sim {
    let grid = Grid::new(width, height);
    let world = World::new(grid);
    let blobs = BlobField::new(&world.micros());
    let flow = FlowProcessor::new(grid, &world);
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
      .ok_or(JsError::OutOfBounds(ord))
  }

  fn commander_idx(&self, index: u32) -> Result<CommanderIdx, JsError> {
    self
      .world
      .commanders()
      .idx(index)
      .ok_or(JsError::InvalidCommanderIndex(index))
  }

  pub fn blobs(&self) -> &BlobField {
    &self.blobs
  }
}
