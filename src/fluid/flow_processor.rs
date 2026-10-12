use num_enum::TryFromPrimitive;

use crate::blob_field::BlobField;
use crate::commander_entity::CommanderCollection;
use crate::direction::Direction;
use crate::entity::Entity;
use crate::fluid::Pressurized;
use crate::fluid::fluid_transfer::{
  FluidLocation, FluidTransfer, FluidTransferNode, FluidTransferTo,
};
use crate::geometry::grid::Grid;
use crate::micro_entity::{Density, MicroDef, MicroEnt, MicroKind};
use crate::structure_entity::StructureState;
use crate::world::WorldMut;

struct CellClaim {
  to: FluidTransferNode,
  kind: MicroKind,
  density: Density,
}

pub struct FlowProcessor {
  /// The world grid
  grid: Grid,

  // Cached and reused arrays for processing flow.
  // Only valid during the tick process
  /// Pressure field for world map cells
  pressures: Vec<f64>,
  /// All the transfers for this tick
  transfers: Vec<FluidTransfer>,
  /// The number of incoming transfers for each cell by `MicroIdx`.
  degree_in: Vec<u32>,
  /// The number of outgoing transfers for each cell by `MicroIdx`.
  degree_out: Vec<u32>,
  /// The scale of inflow for each cell by `MicroIdx`.
  /// Starts out as a sum, before scale_transfers scales it.
  desired_inflow: Vec<f64>,
  /// The scale of outflow for each cell by `MicroIdx`.
  /// Starts out as a sum, before `scale_transfers` scales it.
  desired_outflow: Vec<f64>,
  /// The net change desired for each cell by `MicroIdx`.
  delta: Vec<f64>,
  /// Claims for empty cells.
  empty_claims: Vec<CellClaim>,
  /// Total sum of each micro kind trying to claim each cell, by `MicroIdx`.
  empty_claim_sums: Vec<[f64; MicroKind::COUNT]>,
}

impl FlowProcessor {
  pub fn new(grid: Grid) -> FlowProcessor {
    let size = FluidTransferNode::size(grid);
    FlowProcessor {
      grid,
      pressures: vec![0.0; size],
      // Each cell can flow 4 ways, but not every cell will be occupied.
      // Guessing at a sane size
      transfers: Vec::with_capacity(size * 2),
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

  pub fn tick(&mut self, world: &mut WorldMut, blobs: &BlobField, current_tick: u64) {
    self.reset();
    self.collect_cell_pressures(world, current_tick);
    self.process_world(world, blobs);
    self.process_arteries(world);
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

  fn collect_cell_pressures(&mut self, world: &WorldMut, current_tick: u64) {
    for idx in FluidTransferNode::iter(world.grid) {
      // Pressurized trait being useless because we need it for arteries not micros
      let mut pressure = idx
        .slot(world)
        .map_or(0.0, |s| s.ent().map_or(0.0, |f| f.pressure()));

      if let FluidLocation::Artery(idx) = idx.location() {
        if let Some(StructureState::Artery(artery)) = world.structures.get(idx).state() {
          pressure += artery.pulse_pressure(current_tick)
        }
      }

      self.pressures[idx.index()] = pressure;
    }
  }

  fn process_world(&mut self, world: &WorldMut, blobs: &BlobField) {
    let grid = world.grid;

    // Process grid units
    grid.for_each_adjacent_pair(|a, b| {
      let a = FluidTransferNode::cell(a);
      let b = FluidTransferNode::cell(b);
      let flow = self.compute_flow(world, Some(blobs), a, b);
      if let Some(transfer) = flow {
        self.push_transfer(transfer);
      }
    });
  }

  fn process_arteries(&mut self, world: &WorldMut) {
    for (idx, structure) in world.structures.iter_enumerate() {
      let Some(structure) = structure.ent() else {
        continue;
      };

      let StructureState::Artery(artery) = structure.state() else {
        continue;
      };

      let from = FluidTransferNode::artery(world.grid, idx);
      let from_coords = world.grid.coordinate(idx).unwrap();

      for (dir, offset) in artery.direction().iter_direction_offset_pairs() {
        let Some(to_idx) = world
          .grid
          .neighbor(from_coords.x, from_coords.y, offset.dx, offset.dy)
        else {
          continue;
        };

        if let Some(StructureState::Artery(other_artery)) = world.structures.get(to_idx).state() {
          if dir != Direction::Left && dir != Direction::Down {
            // Arteries shouldn't be checked twice, so limit us to a pair walk only.
            continue;
          }

          if !artery.is_connected_to(dir, other_artery) {
            continue;
          }

          // Artery connected to artery

          let to = FluidTransferNode::artery(world.grid, to_idx);
          let flow = self.compute_flow(world, None, from, to);

          if let Some(transfer) = flow {
            self.push_transfer(transfer);
          }
        } else {
          // Artery open to the world
          let to = FluidTransferNode::cell(to_idx);
          let flow = self.compute_flow(world, None, from, to);
          if let Some(transfer) = flow {
            self.push_transfer(transfer);
          }
        }
      }
    }
  }

  fn push_transfer(&mut self, transfer: FluidTransfer) {
    let to = transfer.to.index();
    let from = transfer.from.index();

    self.degree_in[to] += 1;
    self.degree_out[from] += 1;

    self.transfers.push(transfer);
  }

  fn scale_transfers(&mut self, world: &WorldMut) {
    for transfer in &mut self.transfers {
      if transfer.amount <= 0.0 {
        continue;
      }

      let from = transfer.from.index();
      let to = transfer.to.index();

      let out_count = self.degree_out[from];
      let in_count = self.degree_in[to];
      let restrict_count = out_count.max(in_count);

      let scale = 1.0 / restrict_count as f64;

      // Scale the amount by the cells acting on us, so we don't break equilibrium
      transfer.amount *= scale;

      self.desired_inflow[to] += transfer.amount;
      self.desired_outflow[from] += transfer.amount;
    }

    for idx in FluidTransferNode::iter(world.grid) {
      let index = idx.index();

      let outflow = self.desired_outflow[index];
      let inflow = self.desired_inflow[index];

      let density = idx
        .slot(world)
        .map_or(0.0, |s| s.ent().map_or(0.0, |e| e.quantity().value()));
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
  }

  fn compute_delta(&mut self, world: &WorldMut) {
    for transfer in &self.transfers {
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

      // TODO: Pick based on combat power.  This lets non-combat micros win against combat ones
      // on inflow alone.
      if let FluidTransferTo::Empty(_) = transfer.to {
        let from_kind = transfer.from.micro(world).unwrap().kind();
        self.empty_claim_sums[to][from_kind as usize] += amount;
      } else {
        self.delta[to] += amount;
      }
    }

    for to in FluidTransferNode::iter(world.grid) {
      let mut max_kind: Option<MicroKind> = None;
      let mut max_density = 0.0;
      for (kind_idx, &amount) in self.empty_claim_sums[to.index()].iter().enumerate() {
        if amount > max_density {
          max_density = amount;
          max_kind = Some(MicroKind::try_from_primitive(kind_idx as u8).unwrap());
        }
      }

      if let Some(density) = Density::new(max_density)
        && let Some(kind) = max_kind
      {
        self.empty_claims.push(CellClaim { to, kind, density });
      }
    }
  }

  fn apply(&self, world: &mut WorldMut) {
    for to in FluidTransferNode::iter(world.grid) {
      let delta = self.delta[to.index()];
      if delta == 0.0 {
        continue;
      }

      to.slot_mut(world).unwrap().offset_quantity(delta);
    }

    for claim in &self.empty_claims {
      let to = claim.to;
      to.slot_mut(world)
        .unwrap()
        .set_ent(MicroEnt::new(claim.kind, claim.density));
    }
  }

  fn compute_flow(
    &self,
    world: &WorldMut,
    blobs: Option<&BlobField>,
    a: FluidTransferNode,
    b: FluidTransferNode,
  ) -> Option<FluidTransfer> {
    let difference = self.pressures[a.index()] - self.pressures[b.index()]
      + blobs.map_or(0.0, |blobs| {
        self.compute_commander_flow(blobs, world.commanders, a, b)
      });

    if difference == 0.0 {
      return None;
    }

    if difference < 0.0 {
      return Self::compute_flow_from(world, b, a, -difference);
    }

    Self::compute_flow_from(world, a, b, difference)
  }

  fn compute_commander_flow(
    &self,
    blobs: &BlobField,
    commanders: &CommanderCollection,
    from: FluidTransferNode,
    to: FluidTransferNode,
  ) -> f64 {
    let label_from = blobs.label_at_ordinal(from.idx().index());
    let label_to = blobs.label_at_ordinal(to.idx().index());

    let Some(label) = label_from.or(label_to) else {
      return 0.0;
    };

    let from_c = self.grid.coordinate(from.idx()).unwrap();
    let to_c = self.grid.coordinate(to.idx()).unwrap();

    let mut total: f64 = 0.0;
    for commander in commanders.commanders_in(label) {
      total += commander.pressure_at(from_c) - commander.pressure_at(to_c);
    }

    total
  }

  fn compute_flow_from(
    world: &WorldMut,
    from: FluidTransferNode,
    to: FluidTransferNode,
    pressure_delta: f64,
  ) -> Option<FluidTransfer> {
    let from_def = from.micro(world).map(MicroEnt::def)?;
    let to_def = to.micro(world).map(MicroEnt::def);

    if let Some(to_def) = to_def
      && to_def.kind != from_def.kind
    {
      return None;
    }

    // If there's a structure there, check the structure's permeability.
    let from_artery = if let FluidLocation::Artery(_) = from.location() {
      true
    } else {
      false
    };

    let to_artery = if let FluidLocation::Artery(_) = to.location() {
      true
    } else {
      false
    };

    if !from_artery && to_artery {
      // Can't backflow into an artery.
      return None;
    }

    // Can flow between arteries.  Bypass structure permeability check.
    if !from_artery || !to_artery {
      if let Some(structure) = world.structures.get(to.idx()).ent()
        && !structure
          .def()
          .permeability
          .is_permeable_to(from_def.permeability)
      {
        return None;
      }
    }

    let amount = if from_artery {
      // Pumping arteries have a conductance of 1, and ignore spread yield.
      pressure_delta
    } else {
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
      excess * conductance
    };

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
