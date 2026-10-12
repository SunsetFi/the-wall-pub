use crate::quantity::QuantityOffset;

pub trait Entity {
  type Kind: Copy + PartialEq;
  type Def: 'static;
  type Quantity: QuantityOffset;

  fn kind(&self) -> Self::Kind;
  fn def(&self) -> &'static Self::Def;
  fn quantity(&self) -> Self::Quantity;
  fn set_quantity(&mut self, quantity: Self::Quantity);
}
