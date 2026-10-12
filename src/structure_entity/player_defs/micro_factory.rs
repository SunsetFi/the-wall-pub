use macros::structure_ent;

use crate::{
  alignment::Alignment,
  material::Material,
  micro_entity::{Density, MicroKind},
  permeability::Permeability,
  structure_entity::{
    StructureState, StructureThink,
    player_defs::ArteryUpdate,
    structure_think::{StructureApply, StructureThinkArgs},
  },
};

#[structure_ent(
  alignment = Alignment::Player,
  material = Material::StructuralMetal,
  permeability = Permeability::Infrastructure,
  toughness = 400.0,
  construction_density = 3.0,
)]
#[derive(Clone, Copy, Default)]
pub struct MicroFactoryStructure {
  last_production_tick: Option<u64>,
}

const PRODUCTION_INTERVAL_TICKS: u64 = 10;
const PRODUCTION_PER_TICK: Density = Density::new(0.2).unwrap();

const PRODUCTION_OFFSETS: [(i32, i32); 4] = [(0, 2), (2, 0), (0, -2), (-2, 0)];

impl From<MicroFactoryStructure> for StructureState {
  fn from(micro_factory: MicroFactoryStructure) -> StructureState {
    StructureState::MicroFactory(micro_factory)
  }
}

impl MicroFactoryStructure {
  pub fn set_production_tick(&mut self, current_tick: u64) {
    self.last_production_tick = Some(current_tick);
  }
}

impl StructureThink for MicroFactoryStructure {
  fn think(&self, args: StructureThinkArgs) {
    let current_tick = args.current_tick;

    let last_production_tick = self.last_production_tick.unwrap_or(0);
    if last_production_tick + PRODUCTION_INTERVAL_TICKS > current_tick {
      return;
    }

    args.updates.mutate(
      args.self_idx,
      MicroFactoryUpdate::SetProductionTick(current_tick).into(),
    );

    let self_pos = args.grid.coordinate(args.self_idx).unwrap();
    for offset in PRODUCTION_OFFSETS {
      let Some(connected_idx) = args
        .grid
        .neighbor(self_pos.x, self_pos.y, offset.0, offset.1)
      else {
        continue;
      };

      let Some(StructureState::Artery(artery)) = args.structures.get(connected_idx).state() else {
        continue;
      };

      if !artery.can_pulse(current_tick) {
        continue;
      }

      args.updates.mutate(
        connected_idx,
        ArteryUpdate::Fill(MicroKind::Friendly, PRODUCTION_PER_TICK).into(),
      );

      args
        .updates
        .mutate(connected_idx, ArteryUpdate::Pulse(current_tick).into());
    }
  }
}

pub enum MicroFactoryUpdate {
  SetProductionTick(u64),
}

impl StructureApply for MicroFactoryStructure {
  type Update = MicroFactoryUpdate;

  fn apply(&mut self, update: Self::Update) {
    match update {
      MicroFactoryUpdate::SetProductionTick(current_tick) => {
        self.set_production_tick(current_tick);
      }
    }
  }
}
