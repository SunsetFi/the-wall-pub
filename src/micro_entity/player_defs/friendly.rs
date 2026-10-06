use macros::micro_ent;

use crate::{alignment::Alignment, material::Material};

#[micro_ent(
  alignment = Alignment::Player,
  material = Material::NanoMetal,
  cohesion = 2.0,
  spread_threshold = 0.12,
  empty_conductance = 1.0,
  peer_conductance = 1.0,
  combat_power = 4.0,
  combat_loss_saturation = 0.24,
)]
pub struct FriendlyMicro {}
