use crate::geometry::{Coordinate, Vector};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CoordinateOffset {
  pub dx: i32,
  pub dy: i32,
}

impl CoordinateOffset {
  pub fn new(dx: i32, dy: i32) -> Self {
    Self { dx, dy }
  }

  pub fn distance(&self) -> u32 {
    ((self.dx * self.dx + self.dy * self.dy) as f64).sqrt() as u32
  }

  pub fn normalize(&self) -> Vector {
    self.as_vector().normalize()
  }

  pub fn as_vector(&self) -> Vector {
    Vector::new(self.dx as f64, self.dy as f64)
  }
}

impl std::ops::Add for CoordinateOffset {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      dx: self.dx + other.dx,
      dy: self.dy + other.dy,
    }
  }
}

impl std::ops::Sub for CoordinateOffset {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      dx: self.dx - other.dx,
      dy: self.dy - other.dy,
    }
  }
}

impl From<Coordinate> for CoordinateOffset {
  fn from(coord: Coordinate) -> Self {
    Self {
      dx: coord.x as i32,
      dy: coord.y as i32,
    }
  }
}

impl From<CoordinateOffset> for Vector {
  fn from(offset: CoordinateOffset) -> Self {
    Vector::new(offset.dx as f64, offset.dy as f64)
  }
}
