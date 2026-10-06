#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Integrity(f64);

impl Integrity {
  // Janky code to keep clear-on-zero behavior
  const MIN_VALUE: f64 = 0.000001;
  const MAX_VALUE: f64 = f64::MAX;

  pub const MIN: Integrity = Integrity(Self::MIN_VALUE);
  pub const MAX: Integrity = Integrity(Self::MAX_VALUE);

  pub fn new(value: f64) -> Option<Self> {
    // Handles min enforcement, max clamping, and NaN
    (value >= Self::MIN_VALUE).then(|| Self(value.min(Self::MAX_VALUE)))
  }

  pub const fn value(self) -> f64 {
    self.0
  }

  pub const fn offset(self, offset: f64) -> Option<Integrity> {
    if self.0 + offset < Self::MIN_VALUE {
      None
    } else if self.0 + offset > Self::MAX_VALUE {
      Some(Self::MAX)
    } else {
      Some(Integrity(self.0 + offset))
    }
  }
}

impl From<Integrity> for f64 {
  fn from(integrity: Integrity) -> Self {
    integrity.0
  }
}

#[cfg(test)]
mod tests {
  use super::Integrity;

  #[test]
  fn it_returns_none_below_min() {
    assert_eq!(Integrity::new(Integrity::MIN_VALUE / 2.0), None);
  }

  #[test]
  fn it_returns_none_for_nan() {
    assert_eq!(Integrity::new(f64::NAN), None);
  }

  #[test]
  fn it_returns_some_at_min() {
    assert_eq!(
      Integrity::new(Integrity::MIN_VALUE),
      Some(Integrity(Integrity::MIN_VALUE))
    );
  }

  #[test]
  fn it_returns_some_above_min() {
    let value = Integrity::MIN_VALUE + 0.001;
    assert_eq!(Integrity::new(value), Some(Integrity(value)));
  }

  #[test]
  fn it_returns_max_at_max() {
    assert_eq!(
      Integrity::new(Integrity::MAX_VALUE),
      Some(Integrity(Integrity::MAX_VALUE))
    );
  }

  #[test]
  fn it_returns_max_above_max() {
    let value = Integrity::MAX_VALUE + 0.001;
    assert_eq!(Integrity::new(value), Some(Integrity(Integrity::MAX_VALUE)));
  }
}
