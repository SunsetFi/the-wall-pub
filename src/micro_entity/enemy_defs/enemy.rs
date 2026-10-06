use macros::micro_ent;

use crate::{alignment::Alignment, material::Material};

#[micro_ent(
  alignment = Alignment::Enemy,
  material = Material::Organic,
  cohesion = 1.4,
  spread_threshold = 0.04,
  empty_conductance = 1.0,
  peer_conductance = 1.0,
  combat_loss_saturation = 0.03,
)]
pub struct EnemyMicro {}
