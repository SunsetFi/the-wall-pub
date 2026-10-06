#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Density(f64);

impl Density {
  const MIN_VALUE: f64 = 0.0004;
  const MAX_VALUE: f64 = 1.0;

  pub const MIN: Density = Density(Self::MIN_VALUE);
  pub const MAX: Density = Density(Self::MAX_VALUE);

  pub fn new(value: f64) -> Option<Self> {
    // Handles min enforcement, max clamping, and NaN
    (value >= Self::MIN_VALUE).then(|| Self(value.min(Self::MAX_VALUE)))
  }

  pub const fn value(self) -> f64 {
    self.0
  }

  pub const fn offset(self, offset: f64) -> Option<Density> {
    if self.0 + offset < Self::MIN_VALUE {
      None
    } else if self.0 + offset > Self::MAX_VALUE {
      Some(Self::MAX)
    } else {
      Some(Density(self.0 + offset))
    }
  }
}

impl From<Density> for f64 {
  fn from(density: Density) -> Self {
    density.0
  }
}

#[cfg(test)]
mod tests {
  use super::Density;

  #[test]
  fn it_returns_none_below_min() {
    assert_eq!(Density::new(Density::MIN_VALUE / 2.0), None);
  }

  #[test]
  fn it_returns_none_for_nan() {
    assert_eq!(Density::new(f64::NAN), None);
  }

  #[test]
  fn it_returns_some_at_min() {
    assert_eq!(
      Density::new(Density::MIN_VALUE),
      Some(Density(Density::MIN_VALUE))
    );
  }

  #[test]
  fn it_returns_some_above_min() {
    let value = Density::MIN_VALUE + 0.001;
    assert_eq!(Density::new(value), Some(Density(value)));
  }

  #[test]
  fn it_returns_max_at_max() {
    assert_eq!(
      Density::new(Density::MAX_VALUE),
      Some(Density(Density::MAX_VALUE))
    );
  }

  #[test]
  fn it_returns_max_above_max() {
    let value = Density::MAX_VALUE + 0.001;
    assert_eq!(Density::new(value), Some(Density(Density::MAX_VALUE)));
  }
}
