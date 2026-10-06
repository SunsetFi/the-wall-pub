use num_enum::TryFromPrimitive;

use crate::blob_field::BlobField;
use crate::commander_entity::CommanderCollection;
use crate::fluid::Pressurized;
use crate::fluid::fluid_transfer::FluidTransfer;
use crate::fluid::fluid_transfer_to::FluidTransferTo;
use crate::fluid::pressure_field::PressureField;
use crate::geometry::grid::{CellIdx, Grid};
use crate::micro_entity::{Density, MicroDef, MicroKind};
use crate::world::{World, WorldMut};

struct CellClaim {
  idx: CellIdx,
  kind: MicroKind,
  density: Density,
}

pub struct FlowProcessor {
  /// The world grid
  grid: Grid,

  /// Pressure field for world map cells
  field: PressureField,

  // Cached and reused arrays for processing flow.
  // Only valid during the tick process
  /// All the transfers for this tick
  transfers: Vec<FluidTransfer>,
  /// The number of incoming transfers for each cell by MicroIdx
  degree_in: Vec<u32>,
  /// The number of outgoing transfers for each cell by MicroIdx
  degree_out: Vec<u32>,
  /// The scale of inflow for each cell by MicroIdx.
  /// Starts out as a sum, before scale_transfers scales it.
  desired_inflow: Vec<f64>,
  /// The scale of outflow for each cell by MicroIdx.
  /// Starts out as a sum, before scale_transfers scales it.
  desired_outflow: Vec<f64>,
  /// The net change desired for each cell by MicroIdx.
  delta: Vec<f64>,
  /// Claims for empty cells.
  empty_claims: Vec<CellClaim>,
  /// Total sum of each micro kind trying to claim each cell, by MicroIdx.
  empty_claim_sums: Vec<[f64; MicroKind::COUNT]>,
}

impl FlowProcessor {
  pub fn new(grid: Grid, world: &World) -> FlowProcessor {
    let size = world.grid().size();
    FlowProcessor {
      grid,
      field: PressureField::new(size),
      // Each cell can flow 4 ways, but not every cell will be occupied.
      // Guessing at a sane size
      transfers: Vec::with_capacity(size * 2),
      // TODO: Do we want these always allocated, or is it okay to memory manage these?
      degree_in: vec![0; size],
      degree_out: vec![0; size],
      desired_inflow: vec![0.0; size],
      desired_outflow: vec![0.0; size],
      delta: vec![0.0; size],
      // Guessing at a sane size.  cell_size*4 at worse case.
      empty_claims: Vec::with_capacity(size),
      empty_claim_sums: vec![[0.0_f64; MicroKind::COUNT]; size],
    }
  }

  pub fn tick(&mut self, world: &mut WorldMut, blobs: &BlobField) {
    self.reset();
    self.collect_cell_pressures(world);
    self.process_world(world, blobs);
    self.scale_transfers(world);
    self.compute_delta(world);
    self.apply(world);
  }

  fn reset(&mut self) {
    self.transfers.clear();
    self.desired_inflow.fill(0.0);
    self.desired_outflow.fill(0.0);
    self.degree_in.fill(0);
    self.degree_out.fill(0);
    self.delta.fill(0.0);
    self.empty_claims.clear();
    self.empty_claim_sums.fill([0.0_f64; MicroKind::COUNT]);
  }

  fn collect_cell_pressures(&mut self, world: &WorldMut) {
    for (idx, micro) in world.micros.iter_enumerate() {
      self.field.set(idx, micro.pressure());
    }
  }

  fn process_world(&mut self, world: &WorldMut, blobs: &BlobField) {
    let grid = world.grid;

    // Process grid units
    grid.for_each_adjacent_pairs(|a, b| {
      let flow = self.compute_flow(world, blobs, a, b);
      if let Some(transfer) = flow {
        self.push_transfer(transfer);
      }
    });

    // TODO: Process arteries
    // TODO: Process artery outlets
  }

  fn push_transfer(&mut self, transfer: FluidTransfer) {
    let to = transfer.to.index();
    let from = transfer.from.index();

    self.degree_in[to] += 1;
    self.degree_out[from] += 1;

    self.transfers.push(transfer);
  }

  fn scale_transfers(&mut self, world: &WorldMut) {
    for transfer in self.transfers.iter_mut() {
      if transfer.amount <= 0.0 {
        continue;
      }

      let from = transfer.from.index();
      let to = transfer.to.index();

      let out_count = self.degree_out[from];
      let in_count = self.degree_in[to];
      let restrict_count = out_count.max(in_count);

      let scale = 1.0 / restrict_count as f64;

      // Scale the amount by outflow, so we don't break equilibrium
      transfer.amount *= scale;

      self.desired_inflow[to] += transfer.amount;
      self.desired_outflow[from] += transfer.amount;
    }

    for (idx, micro) in world.micros.iter_enumerate() {
      let index = idx.index();

      let density = micro.density().map(|f| f.value()).unwrap_or(0.0);
      if density == 0.0 {
        continue;
      }

      let outflow = self.desired_outflow[index];
      let inflow = self.desired_inflow[index];

      let egress = outflow.min(density);
      let room = Density::MAX.value() - (density - egress);

      self.desired_outflow[index] = if outflow > 0.0 { egress / outflow } else { 0.0 };

      if inflow <= Density::MIN.value() {
        self.desired_inflow[index] = 0.0;
      } else {
        self.desired_inflow[index] = if inflow > 0.0 {
          (room / inflow).min(1.0)
        } else {
          0.0
        };
      }
    }

    // TODO: We need to scale on other slots.  Arteries, artery outlets.
  }

  fn compute_delta(&mut self, world: &WorldMut) {
    for transfer in self.transfers.iter() {
      let from = transfer.from.index();
      let to = transfer.to.index();
      let amount = transfer.amount;

      let in_scale = self.desired_inflow[to];
      let out_scale = self.desired_outflow[from];
      let amount = amount * in_scale.min(out_scale);
      if amount == 0.0 {
        continue;
      }

      self.delta[from] -= amount;

      if let FluidTransferTo::Empty(_) = transfer.to {
        let from_kind = world.micros.get(transfer.from).kind().unwrap();
        self.empty_claim_sums[to][from_kind as usize] += amount;
      } else {
        self.delta[to] += amount;
      }
    }

    for idx in world.micros.iter_idx() {
      let index = idx.index();
      let mut max_kind: MicroKind = MicroKind::try_from_primitive(0).unwrap();
      let mut max_amount = 0.0;
      for (kind_idx, &amount) in self.empty_claim_sums[index].iter().enumerate() {
        if amount > max_amount {
          max_amount = amount;
          max_kind = MicroKind::try_from_primitive(kind_idx as u8).unwrap();
        }
      }

      if let Some(max_amount) = Density::new(max_amount) {
        self.empty_claims.push(CellClaim {
          idx,
          kind: max_kind,
          density: max_amount,
        });
      }
    }
  }

  fn apply(&self, world: &mut WorldMut) {
    let micros = &mut world.micros;

    for (idx, micro) in micros.iter_enumerate_mut() {
      let index = idx.index();

      let delta = self.delta[index];
      if delta == 0.0 {
        continue;
      }

      micro.offset_density(delta);
    }

    for claim in self.empty_claims.iter() {
      micros.get_mut(claim.idx).set(claim.kind, claim.density);
    }
  }

  fn compute_flow(
    &self,
    world: &WorldMut,
    blobs: &BlobField,
    a: CellIdx,
    b: CellIdx,
  ) -> Option<FluidTransfer> {
    // TODO: Commander pressure
    // TODO: Is precomputing these even buying us anything?  Its saving us a powf call
    let difference = self.field.get(a) - self.field.get(b)
      + self.compute_commander_flow(blobs, world.commanders, a, b);

    if difference == 0.0 {
      return None;
    }

    if difference < 0.0 {
      return self.compute_flow_from(world, b, a, -difference);
    }

    self.compute_flow_from(world, a, b, difference)
  }

  fn compute_commander_flow(
    &self,
    blobs: &BlobField,
    commanders: &CommanderCollection,
    from: CellIdx,
    to: CellIdx,
  ) -> f64 {
    let label_from_opt = blobs.label_at_ordinal(from.index());
    let label_to_opt = blobs.label_at_ordinal(to.index());

    if label_from_opt.is_none() && label_to_opt.is_none() {
      return 0.0;
    }

    let mut total: f64 = 0.0;

    let from_c = self.grid.coordinate(from).unwrap();
    let to_c = self.grid.coordinate(to).unwrap();

    if let Some(label_from) = label_from_opt {
      for commander in commanders.commanders_in(label_from) {
        total += commander.pressure_at(from_c) - commander.pressure_at(to_c);
      }
    } else if let Some(label_to) = label_to_opt {
      for commander in commanders.commanders_in(label_to) {
        total += commander.pressure_at(from_c) - commander.pressure_at(to_c);
      }
    };

    total
  }

  fn compute_flow_from(
    &self,
    world: &WorldMut,
    from: CellIdx,
    to: CellIdx,
    pressure_delta: f64,
  ) -> Option<FluidTransfer> {
    let micros = &world.micros;

    let from_def = micros.get(from).def()?;
    let to_def = micros.get(to).def();

    if let Some(to_def) = to_def
      && to_def.kind != from_def.kind
    {
      return None;
    }

    // If there's a structure there, check the structure's permeability.
    let structure = world.structures.get(to);
    if let Some(structure_def) = structure.def() {
      if !structure_def
        .permeability
        .is_permeable_to(from_def.permeability)
      {
        return None;
      }
    }

    let spread_yield = if to_def.is_none() {
      from_def.spread_yield()
    } else {
      0.0
    };
    let excess = pressure_delta - spread_yield;
    if excess <= 0.0 {
      return None;
    }

    let conductance = Self::edge_conductance(from_def, to_def);
    let amount = excess * conductance;

    if to_def.is_some() {
      return Some(FluidTransfer {
        from,
        to: FluidTransferTo::Occupied(to),
        amount,
      });
    }

    Some(FluidTransfer {
      from,
      to: FluidTransferTo::Empty(to),
      amount,
    })
  }

  /// Gets the fraction against the fastest rate two cells can hit equillibrium
  fn edge_conductance(from_def: &MicroDef, to_def: Option<&MicroDef>) -> f64 {
    let conductance = {
      // Original had conductance into cells that are opposed
      // Ignoring that for now.
      if let Some(to_def) = to_def
        && to_def.kind == from_def.kind
      {
        from_def.peer_conductance
      } else {
        from_def.empty_conductance
      }
    };

    conductance / (2.0 * from_def.cohesion)
  }
}
