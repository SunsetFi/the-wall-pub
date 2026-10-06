use crate::{material::Material, micro_entity::MicroEnt, world::WorldMut};

pub struct CombatProcessor {}

impl CombatProcessor {
  /// Max density lost per tick for a maximum density micro vs a minimum density micro.
  const COMBAT_MICRO_LOSS_STEP: f64 = 0.03;

  /// Max integrity lost per tick for a maximum density micro vs a structure.
  // FIXME: Re-scale this so 1 means something
  const COMBAT_STRUCTURE_LOSS_STEP: f64 = 75.0;

  pub fn tick(world: &mut WorldMut) {
    CombatProcessor::apply_micro_combat(world);
    CombatProcessor::apply_structure_combat(world);
  }

  fn apply_micro_combat(world: &mut WorldMut) {
    let grid = world.grid;
    let micros = &mut world.micros;
    grid.for_each_adjacent_pairs(|a_idx, b_idx| {
      let a = micros.get(a_idx);
      let b = micros.get(b_idx);

      let Some(a) = a.get_ent() else {
        return;
      };

      let Some(b) = b.get_ent() else {
        return;
      };

      if !a.def().alignment.is_hostile_to(b.def().alignment) {
        return;
      }

      let a_strength = CombatProcessor::get_effective_strength(a, b.def().material);
      let b_strength = CombatProcessor::get_effective_strength(b, a.def().material);

      let strength_sum = a_strength + b_strength;
      if strength_sum == 0.0 {
        return;
      }

      let a_ratio = a_strength / strength_sum;
      let b_ratio = b_strength / strength_sum;

      let a_scale = b_ratio / (a_strength + a.def().combat_loss_saturation);
      let b_scale = a_ratio / (b_strength + b.def().combat_loss_saturation);

      micros
        .get_mut(a_idx)
        .offset_density(-a_scale * CombatProcessor::COMBAT_MICRO_LOSS_STEP);
      micros
        .get_mut(b_idx)
        .offset_density(-b_scale * CombatProcessor::COMBAT_MICRO_LOSS_STEP);
    });
  }

  fn apply_structure_combat(world: &mut WorldMut) {
    let grid = world.grid;
    let micros = &mut world.micros;
    let structures = &mut world.structures;
    for idx in grid.iter_idx() {
      let micro = micros.get(idx);
      let Some(micro) = micro.get_ent() else {
        continue;
      };

      for neighbor_idx in grid.neighbors(idx) {
        let mut structure = structures.get_mut(neighbor_idx);
        let Some(structure_def) = structure.def() else {
          continue;
        };

        if !micro.def().alignment.is_hostile_to(structure_def.alignment) {
          continue;
        }

        let effectiveness = CombatProcessor::get_effective_strength(&micro, structure_def.material);

        if effectiveness <= 0.0 {
          continue;
        }

        structure.offset_integrity(-effectiveness * CombatProcessor::COMBAT_STRUCTURE_LOSS_STEP);
      }
    }
  }

  fn get_effective_strength(ent: &MicroEnt, material: Material) -> f64 {
    let combat_power = ent.def().combat_power_vs(material);
    ent.density().value() * combat_power
  }
}
