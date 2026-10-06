use crate::fluid::fluid_transfer_to::FluidTransferTo;
use crate::geometry::grid::CellIdx;

pub struct FluidTransfer {
  pub from: CellIdx,
  pub to: FluidTransferTo,
  pub amount: f64,
}
