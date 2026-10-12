use macros::structure_ent;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
  JsError, Sim,
  alignment::Alignment,
  direction::{Direction, DirectionFlags},
  entity::Entity,
  material::Material,
  micro_entity::{Density, MicroEnt, MicroKind, MicroSlot},
  permeability::Permeability,
  quantity::Quantity,
  structure_entity::{
    StructureState, StructureThink,
    structure_think::{StructureApply, StructureThinkArgs},
  },
};

#[structure_ent(
  alignment = Alignment::Player,
  material = Material::Concrete,
  permeability = Permeability::Infrastructure,
  toughness = 100.0,
)]
#[derive(Clone, Copy, Default)]
pub struct ArteryStructure {
  direction: DirectionFlags,
  pulse_tick: Option<u64>,
  // TODO: Make this a MicroSlot.  Currently can't because of our think copy nonsense.
  contents: MicroSlot,
}

const PULSE_BAND: u32 = 8;
const PULSE_PRESSURE: f64 = 16.5;
const PULSE_EXCITATION_TICKS: u32 = 1;
const PULSE_REFACTORY_TICKS: u32 = 3;

impl ArteryStructure {
  pub fn direction(&self) -> DirectionFlags {
    self.direction
  }

  pub fn is_connected_to(&self, direction: Direction, other: &ArteryStructure) -> bool {
    let self_direction = self.direction();
    let other_direction = other.direction();

    self_direction.contains(direction) && other_direction.contains(direction.opposite())
  }

  pub fn contents(&self) -> &MicroSlot {
    &self.contents
  }

  pub fn contents_mut(&mut self) -> &mut MicroSlot {
    &mut self.contents
  }

  pub fn pulse_age(&self, current_tick: u64) -> Option<u32> {
    self.pulse_tick.map(|tick| (current_tick - tick) as u32)
  }

  pub fn can_pulse(&self, current_tick: u64) -> bool {
    if let Some(neighbor_age) = self.pulse_age(current_tick)
      && neighbor_age < PULSE_REFACTORY_TICKS
    {
      false
    } else {
      true
    }
  }

  pub fn pulse_pressure(&self, current_tick: u64) -> f64 {
    let Some(age) = self.pulse_age(current_tick) else {
      return 0.0;
    };

    if age > PULSE_BAND {
      return 0.0;
    }

    PULSE_PRESSURE * f64::from(age) / f64::from(PULSE_BAND)
  }

  pub fn pulse(&mut self, current_tick: u64) {
    self.pulse_tick = Some(current_tick);
  }

  pub fn fill(&mut self, kind: MicroKind, density: Density) {
    if let Some(content) = self.contents.ent()
      && content.kind() == kind
    {
      self.contents.offset_quantity(density.value());
    } else {
      self.contents.set_ent(MicroEnt::new(kind, density));
    }
  }

  pub fn set_contents(&mut self, kind: MicroKind, density: Density) {
    self.contents.set_ent(MicroEnt::new(kind, density));
  }

  pub fn set_direction(&mut self, direction: DirectionFlags) {
    self.direction = direction;
  }

  pub fn offset_contents(&mut self, delta: f64) {
    let contents = &mut self.contents;
    let Some(density) = contents.density() else {
      return;
    };

    if let Some(new_density) = density.offset(delta) {
      contents.set_density(new_density);
    } else {
      contents.clear();
    }
  }
}

impl StructureThink for ArteryStructure {
  fn think(&self, args: StructureThinkArgs) {
    let current_tick = args.current_tick;

    let Some(age) = self.pulse_age(current_tick) else {
      return;
    };

    if age < 1 || age > PULSE_EXCITATION_TICKS {
      return;
    }

    let self_coord = args.grid.coordinate(args.self_idx).unwrap();
    for neighbor in DirectionFlags::iter_offsets(self.direction) {
      let Some(neighbor_idx) =
        args
          .grid
          .neighbor(self_coord.x, self_coord.y, neighbor.dx, neighbor.dy)
      else {
        continue;
      };

      let Some(StructureState::Artery(neighbor)) = args.structures.get(neighbor_idx).state() else {
        continue;
      };

      if !neighbor.can_pulse(current_tick) {
        continue;
      }

      args
        .updates
        .mutate(neighbor_idx, ArteryUpdate::Pulse(current_tick).into());
    }
  }
}

pub enum ArteryUpdate {
  Pulse(u64),
  Fill(MicroKind, Density),
}
impl StructureApply for ArteryStructure {
  type Update = ArteryUpdate;

  fn apply(&mut self, update: Self::Update) {
    match update {
      ArteryUpdate::Pulse(current_tick) => {
        self.pulse(current_tick);
      }
      ArteryUpdate::Fill(kind, density) => {
        self.fill(kind, density);
      }
    }
  }
}

impl From<ArteryStructure> for StructureState {
  fn from(artery: ArteryStructure) -> StructureState {
    StructureState::Artery(artery)
  }
}

#[wasm_bindgen]
impl Sim {
  pub fn set_structure_artery_direction(&mut self, ord: u32, direction: u8) -> Result<(), JsError> {
    let StructureState::Artery(artery) = self.structure_state_mut(ord)? else {
      return Err(JsError::InvalidKind(
        ord,
        "Set artery direction used on non-artery",
      ));
    };

    artery.set_direction(DirectionFlags::from(direction));

    Ok(())
  }

  pub fn structure_artery_content_kind(&self, ord: u32) -> Result<Option<MicroKind>, JsError> {
    let StructureState::Artery(artery) = self.structure_state(ord)? else {
      return Err(JsError::InvalidKind(
        ord,
        "Get artery content kind used on non-artery",
      ));
    };

    Ok(artery.contents().ent().map(MicroEnt::kind))
  }

  pub fn structure_artery_content_density(&self, ord: u32) -> Result<Option<f64>, JsError> {
    let StructureState::Artery(artery) = self.structure_state(ord)? else {
      return Err(JsError::InvalidKind(
        ord,
        "Get artery content kind used on non-artery",
      ));
    };

    Ok(
      artery
        .contents()
        .ent()
        .map(MicroEnt::quantity)
        .map(Quantity::value),
    )
  }
}
