use std::marker::PhantomData;

/// A 'Quantity' that may be offset.
/// A quantity may decay to nothing if it breaches
/// the lower bounds.
pub trait QuantityOffset: Copy {
  fn offset(self, offset: f64) -> Option<Self>;
}

pub trait Bounds {
  const MIN: f64;
  const MAX: f64;
}

/// A quantity within a min and a max.
/// Quantities decay to nothing when they breach their
/// lower bounds, and cap out at their upper bounds.
/// Quantities are always positive.
#[derive(Debug)]
pub struct Quantity<B: Bounds>(f64, PhantomData<B>);

impl<B: Bounds> Quantity<B> {
  pub const MIN: Self = Self(B::MIN, PhantomData);
  pub const MAX: Self = Self(B::MAX, PhantomData);

  pub const fn new(value: f64) -> Option<Self> {
    if value.is_nan() || value < B::MIN {
      return None;
    }

    Some(Self(value.min(B::MAX), PhantomData))
  }

  pub const fn value(self) -> f64 {
    self.0
  }

  pub const fn offset(self, offset: f64) -> Option<Self> {
    Self::new(self.0 + offset)
  }
}

impl<B: Bounds> PartialEq for Quantity<B> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<B: Bounds> Eq for Quantity<B> {}

impl<B: Bounds> PartialOrd for Quantity<B> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl<B: Bounds> Ord for Quantity<B> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.partial_cmp(&other.0).unwrap()
  }
}

impl<B: Bounds> From<Quantity<B>> for f64 {
  fn from(quantity: Quantity<B>) -> Self {
    quantity.0
  }
}

impl<B: Bounds> std::ops::Add<f64> for Quantity<B> {
  type Output = Option<Self>;

  fn add(self, rhs: f64) -> Self::Output {
    self.offset(rhs)
  }
}

impl<B: Bounds> std::ops::Add<Quantity<B>> for Quantity<B> {
  type Output = Self;

  fn add(self, rhs: Quantity<B>) -> Self::Output {
    // Quantities must always be positive, so lhs + rhs will always
    // be > min.
    Self((self.0 + rhs.0).min(B::MAX), PhantomData)
  }
}

impl<B: Bounds> Clone for Quantity<B> {
  fn clone(&self) -> Self {
    Self(self.0, PhantomData)
  }
}

impl<B: Bounds> Copy for Quantity<B> {}

impl<B: Bounds> QuantityOffset for Quantity<B> {
  fn offset(self, offset: f64) -> Option<Self> {
    self.offset(offset)
  }
}

#[cfg(test)]
mod tests {
  use std::marker::PhantomData;

  use super::Quantity;

  #[derive(PartialEq, Debug)]
  struct TestBounds;

  impl super::Bounds for TestBounds {
    const MIN: f64 = 0.000001;
    const MAX: f64 = f64::MAX;
  }

  type TestQuantity = Quantity<TestBounds>;

  #[test]
  fn it_returns_none_below_min() {
    assert_eq!(TestQuantity::new(TestQuantity::MIN.value() / 2.0), None);
  }

  #[test]
  fn it_returns_none_for_nan() {
    assert_eq!(TestQuantity::new(f64::NAN), None);
  }

  #[test]
  fn it_returns_some_at_min() {
    assert_eq!(
      TestQuantity::new(TestQuantity::MIN.value()),
      Some(Quantity::<TestBounds>(
        TestQuantity::MIN.value(),
        PhantomData
      ))
    );
  }

  #[test]
  fn it_returns_some_above_min() {
    let value = TestQuantity::MIN.value() + 0.001;
    assert_eq!(
      TestQuantity::new(value),
      Some(Quantity::<TestBounds>(value, PhantomData))
    );
  }

  #[test]
  fn it_returns_max_at_max() {
    assert_eq!(
      TestQuantity::new(TestQuantity::MAX.value()),
      Some(Quantity::<TestBounds>(
        TestQuantity::MAX.value(),
        PhantomData
      ))
    );
  }

  #[test]
  fn it_returns_max_above_max() {
    let value = TestQuantity::MAX.value() + 0.001;
    assert_eq!(
      TestQuantity::new(value),
      Some(Quantity::<TestBounds>(
        TestQuantity::MAX.value(),
        PhantomData
      ))
    );
  }
}
