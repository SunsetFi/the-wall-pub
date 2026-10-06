/// An entity that has a pressure value associated with it.
pub trait Pressurized {
  fn pressure(&self) -> f64;
}
