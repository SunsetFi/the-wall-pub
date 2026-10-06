use crate::{MicroKind, alignment::Alignment, material::Material, permeability::Permeability};

pub struct MicroDef {
  pub kind: MicroKind,
  pub alignment: Alignment,
  pub permeability: Permeability,
  pub material: Material,
  pub cohesion: f64,
  pub spread_threshold: f64,
  pub empty_conductance: f64,
  pub peer_conductance: f64,
  pub combat_power: f64,
  pub combat_power_vs: &'static [(Material, f64)],
  pub combat_loss_saturation: f64,
}

impl MicroDef {
  /// Gets the micro's combat power against a specific material.
  /// If no specific entry exists for the material, returns the default combat power.
  pub fn combat_power_vs(&self, material: Material) -> f64 {
    for &(m, power) in self.combat_power_vs {
      if m == material {
        return power;
      }
    }
    self.combat_power
  }

  pub fn spread_yield(&self) -> f64 {
    // Computed on the hot path, possibly multiple times.
    // This really feels like it should be const, but rust doesn't support
    // powf in const right now...
    self.spread_threshold.powf(self.cohesion)
  }
}

#[cfg(test)]
mod test {
  mod micro_def {
    use super::super::{Alignment, Material, MicroDef, MicroKind, Permeability};

    fn default() -> MicroDef {
      MicroDef {
        kind: MicroKind::Friendly,
        alignment: Alignment::Player,
        permeability: Permeability::Infrastructure,
        material: Material::NanoMetal,
        cohesion: 1.0,
        spread_threshold: 0.4,
        empty_conductance: 1.0,
        peer_conductance: 1.0,
        combat_power: 0.0,
        combat_power_vs: &[],
        combat_loss_saturation: 0.0,
      }
    }

    mod combat_power_vs {
      use super::super::super::Material;
      use super::default;

      #[test]
      fn it_returns_combat_power_vs_specific_material() {
        let mut micro = default();
        micro.combat_power_vs = &[(Material::NanoMetal, 5.0)];
        micro.combat_power = 2.0;
        assert_eq!(micro.combat_power_vs(Material::NanoMetal), 5.0);
      }

      #[test]
      fn it_returns_default_combat_power_if_no_specific_entry() {
        let mut micro = default();
        micro.combat_power_vs = &[(Material::NanoMetal, 5.0)];
        micro.combat_power = 2.0;
        assert_eq!(micro.combat_power_vs(Material::Concrete), 2.0);
      }
    }
  }
}
