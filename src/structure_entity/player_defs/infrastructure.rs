use macros::structure_ent;

use crate::{
  alignment::Alignment,
  material::Material,
  micro_entity::MicroKind,
  permeability::Permeability,
  structure_entity::{
    StructureKind, StructureState, StructureThink, structure_think::StructureThinkArgs,
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
  pub fn new_from_construction(kind: StructureKind) -> Option<InfrastructureStructure> {
    let Some(_) = kind.def().construction_density else {
      return None;
    };
    Some(InfrastructureStructure {
      construction: Some(InfrastructureConstruction { kind, density: 0.0 }),
    })
  }

  fn copy_update_density(&self, density: f64) -> StructureState {
    let mut copy = *self;
    if let Some(construction) = &mut copy.construction {
      construction.density = density;
    }
    StructureState::Infrastructure(copy)
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

    let new_density: f64 = {
      let Some(micro) = args.micros.get(args.self_idx).get_ent() else {
        return;
      };

      if micro.kind() != MicroKind::StructureConstructor {
        return;
      }

      let micro_density = micro.density().value();
      let consumed = micro_density.min(target_density - construction.density);

      let construction_density = construction.density + consumed;
      if construction_density >= target_density {
        args
          .updates
          .replace(args.self_idx, StructureState::new(construction.kind));
      } else {
        args.updates.replace(
          args.self_idx,
          self.copy_update_density(construction_density),
        );
      }
      consumed
    };

    args
      .micros
      .get_mut(args.self_idx)
      .offset_density(new_density);
  }
}
