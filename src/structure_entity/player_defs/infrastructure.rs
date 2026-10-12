use macros::structure_ent;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
  JsError, Sim,
  alignment::Alignment,
  entity::Entity,
  material::Material,
  micro_entity::MicroKind,
  permeability::Permeability,
  structure_entity::{
    StructureKind, StructureState, StructureThink,
    structure_think::{StructureApply, StructureThinkArgs},
  },
};

#[structure_ent(
  alignment = Alignment::Player,
  material = Material::StructuralMetal,
  permeability = Permeability::Infrastructure,
  toughness = 400.0,
)]
#[derive(Copy, Clone, Default)]
pub struct InfrastructureStructure {
  construction: Option<InfrastructureConstruction>,
}
#[derive(Copy, Clone)]
struct InfrastructureConstruction {
  kind: StructureKind,
  density: f64,
}

impl InfrastructureStructure {
  pub fn from_construction_kind(
    kind: StructureKind,
    completed_density: f64,
  ) -> Option<InfrastructureStructure> {
    kind.def().construction_density?;
    Some(InfrastructureStructure {
      construction: Some(InfrastructureConstruction {
        kind,
        density: completed_density,
      }),
    })
  }

  pub fn set_construction(&mut self, kind: StructureKind, density: f64) -> bool {
    if kind.def().construction_density.is_none() {
      return false;
    }

    self.construction = Some(InfrastructureConstruction { kind, density });
    true
  }
}

impl From<InfrastructureStructure> for StructureState {
  fn from(infrastructure: InfrastructureStructure) -> StructureState {
    StructureState::Infrastructure(infrastructure)
  }
}

impl StructureThink for InfrastructureStructure {
  fn think(&self, args: StructureThinkArgs) {
    let Some(construction) = self.construction else {
      return;
    };

    let Some(target_density) = construction.kind.def().construction_density else {
      return;
    };

    let consumed: f64 = {
      let Some(micro) = args.micros.get(args.self_idx).ent() else {
        return;
      };

      if micro.kind() != MicroKind::StructureConstructor {
        return;
      }

      let micro_density = micro.quantity().value();
      let consumed = micro_density.min(target_density - construction.density);

      let construction_density = construction.density + consumed;
      if construction_density >= target_density {
        args
          .updates
          .replace(args.self_idx, StructureState::new(construction.kind), None);
      } else {
        args.updates.mutate(
          args.self_idx,
          InfrastructureUpdate::UpdateConstructDensity(construction_density).into(),
        );
      }
      consumed
    };

    args
      .micros
      .get_mut(args.self_idx)
      .offset_quantity(-consumed);
  }
}

pub enum InfrastructureUpdate {
  UpdateConstructDensity(f64),
}
impl StructureApply for InfrastructureStructure {
  type Update = InfrastructureUpdate;

  fn apply(&mut self, update: Self::Update) {
    match update {
      InfrastructureUpdate::UpdateConstructDensity(density) => {
        if let Some(construction) = &mut self.construction {
          construction.density = density;
        }
      }
    }
  }
}

#[wasm_bindgen]
impl Sim {
  pub fn set_structure_infrastructure_construction(
    &mut self,
    ord: u32,
    kind: StructureKind,
    progress: f64,
  ) -> Result<(), JsError> {
    let StructureState::Infrastructure(infrastructure) = self.structure_state_mut(ord)? else {
      return Err(JsError::InvalidKind(
        ord,
        "Set construction used on non-infrastructure",
      ));
    };

    if !infrastructure.set_construction(kind, progress) {
      Err(JsError::InvalidKind(ord, "Kind is not constructable"))?
    }
    Ok(())
  }
}
