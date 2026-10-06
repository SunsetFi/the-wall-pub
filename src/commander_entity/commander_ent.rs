use crate::{
  blob_field::BlobField,
  commander_entity::{CommanderKind, commander_def::CommanderDef, commander_think::CommanderThinkResult},
  geometry::{Coordinate, grid::Grid},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CommanderEnt {
  pub id: u32,
  pub kind: CommanderKind,
  pub pos: Coordinate,
  pub goal: Coordinate,
}

const PRESSURE_MAGNITUDE: f64 = 0.9;
const PRESSURE_RADIUS: f64 = 20.0;
const PRESSURE_FALLOFF: f64 = 2.0;

const PRESSURE_RANGE: f64 = 3.0;

impl CommanderEnt {
  pub const fn def(&self) -> &'static CommanderDef {
    self.kind.def()
  }
}

impl CommanderEnt {
  pub fn pressure_at(&self, p: Coordinate) -> f64 {
    let goal_offset = (self.goal - self.pos).as_vector();

    let pressure_point_dist = goal_offset.magnitude().min(PRESSURE_RANGE);
    let pressure_point = goal_offset.resize(pressure_point_dist);

    let point_offset = (p - self.pos).as_vector();
    let distance = (point_offset - pressure_point).magnitude();

    if distance > PRESSURE_RADIUS {
      return 0.0;
    }

    -PRESSURE_MAGNITUDE * (1.0 - distance / PRESSURE_RADIUS).powf(PRESSURE_FALLOFF)
  }

  pub fn think(&mut self, grid: Grid, blobs: &BlobField, tick: u64) -> CommanderThinkResult {
    if !tick.is_multiple_of(20) {
      return CommanderThinkResult::Ok;
    }

    let label = blobs.label_at_cell(self.pos.x, self.pos.y);
    let Some(label) = label else {
      return CommanderThinkResult::DeleteSelf;
    };

    let cmx = blobs.centroid_x_of(label);
    let cmy = blobs.centroid_y_of(label);

    let dist_x = cmx - self.pos.x as f64;
    let dist_y = cmy - self.pos.y as f64;

    let delta_x: i32 = if dist_x.abs() > 0.5 {
      dist_x.signum() as i32
    } else {
      0
    };

    let delta_y: i32 = if dist_y.abs() > 0.5 {
      dist_y.signum() as i32
    } else {
      0
    };

    self.pos = self.pos.add(delta_x, delta_y);
    if grid.cell(self.pos.x, self.pos.y).is_none() {
      return CommanderThinkResult::DeleteSelf;
    }
    CommanderThinkResult::Ok
  }
}
