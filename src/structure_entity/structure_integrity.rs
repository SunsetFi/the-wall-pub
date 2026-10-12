use crate::quantity::{Bounds, Quantity};

#[derive(Copy, Clone, PartialEq, Debug)]

pub struct IntegrityBounds {}

impl Bounds for IntegrityBounds {
  const MIN: f64 = 0.000001;
  const MAX: f64 = f64::MAX;
}

pub type Integrity = Quantity<IntegrityBounds>;
