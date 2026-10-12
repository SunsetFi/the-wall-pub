use crate::geometry::{Coordinate, Vector};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CoordinateOffset {
  pub dx: i32,
  pub dy: i32,
}

impl CoordinateOffset {
  pub const fn new(dx: i32, dy: i32) -> Self {
    Self { dx, dy }
  }

  pub fn distance(self) -> u32 {
    let distance_squared: f64 = (self.dx * self.dx + self.dy * self.dy).into();
    distance_squared.sqrt() as u32
  }

  pub fn normalize(self) -> Vector {
    self.as_vector().normalize()
  }

  pub fn as_vector(self) -> Vector {
    Vector::new(self.dx.into(), self.dy.into())
  }

  pub fn as_coordinate(self) -> Option<Coordinate> {
    if self.dx < 0 || self.dy < 0 {
      return None;
    }
    Some(Coordinate::new(self.dx as u32, self.dy as u32))
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
    Vector::new(offset.dx.into(), offset.dy.into())
  }
}
