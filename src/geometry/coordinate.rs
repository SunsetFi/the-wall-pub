use wasm_bindgen::prelude::wasm_bindgen;

use crate::geometry::{CoordinateOffset, Vector};

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coordinate {
  pub x: u32,
  pub y: u32,
}

impl Coordinate {
  pub fn new(x: u32, y: u32) -> Self {
    Self { x, y }
  }

  pub fn offset(self, dx: i32, dy: i32) -> CoordinateOffset {
    CoordinateOffset::new(self.x as i32 + dx, self.y as i32 + dy)
  }
}

impl std::ops::Add for Coordinate {
  type Output = Coordinate;

  fn add(self, other: Self) -> Coordinate {
    Coordinate {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl std::ops::Sub for Coordinate {
  type Output = CoordinateOffset;

  fn sub(self, other: Self) -> CoordinateOffset {
    CoordinateOffset {
      dx: self.x as i32 - other.x as i32,
      dy: self.y as i32 - other.y as i32,
    }
  }
}

impl From<Coordinate> for (u32, u32) {
  fn from(coord: Coordinate) -> Self {
    (coord.x, coord.y)
  }
}
impl From<(u32, u32)> for Coordinate {
  fn from(tuple: (u32, u32)) -> Self {
    Self {
      x: tuple.0,
      y: tuple.1,
    }
  }
}
impl From<Coordinate> for Vector {
  fn from(coord: Coordinate) -> Self {
    Vector::new(coord.x.into(), coord.y.into())
  }
}
