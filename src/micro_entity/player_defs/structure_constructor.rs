use macros::micro_ent;

use crate::{alignment::Alignment, material::Material, permeability::Permeability};

#[micro_ent(
  alignment = Alignment::Player,
  permeability = Permeability::Infrastructure,
  material = Material::NanoMetal,
  cohesion = 1.0,
  spread_threshold = 0.4,
  empty_conductance = 1.0,
  peer_conductance = 1.0,
  combat_power = 0.0,
  combat_loss_saturation = 0.0,
)]
pub struct StructureConstructorMicro {}
