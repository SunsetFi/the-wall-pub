use crate::quantity::{Bounds, Quantity};

#[derive(Copy, Clone, PartialEq, Debug)]

pub struct DensityBounds {}

impl Bounds for DensityBounds {
  const MIN: f64 = 0.0004;
  const MAX: f64 = 1.0;
}

pub type Density = Quantity<DensityBounds>;
