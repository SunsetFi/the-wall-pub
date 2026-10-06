pub struct Vector {
  pub dx: f64,
  pub dy: f64,
}

impl Vector {
  pub fn new(dx: f64, dy: f64) -> Self {
    Self { dx, dy }
  }

  pub fn magnitude(&self) -> f64 {
    (self.dx * self.dx + self.dy * self.dy).sqrt()
  }

  pub fn normalize(&self) -> Self {
    let magnitude = self.magnitude();
    if magnitude == 0.0 {
      Self { dx: 0.0, dy: 0.0 }
    } else {
      Self {
        dx: self.dx / magnitude,
        dy: self.dy / magnitude,
      }
    }
  }

  pub fn resize(&self, new_magnitude: f64) -> Self {
    let magnitude = self.magnitude();
    if magnitude == 0.0 {
      Self { dx: 0.0, dy: 0.0 }
    } else {
      let scale = new_magnitude / magnitude;
      Self {
        dx: self.dx * scale,
        dy: self.dy * scale,
      }
    }
  }

  pub fn scale(&self, factor: f64) -> Self {
    Self {
      dx: self.dx * factor,
      dy: self.dy * factor,
    }
  }
}

impl std::ops::Add for Vector {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      dx: self.dx + other.dx,
      dy: self.dy + other.dy,
    }
  }
}

impl std::ops::Sub for Vector {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      dx: self.dx - other.dx,
      dy: self.dy - other.dy,
    }
  }
}
