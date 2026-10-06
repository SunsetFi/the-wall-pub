pub trait Generational {
  fn generation(&self) -> u32;
}
