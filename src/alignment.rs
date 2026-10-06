#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Alignment {
  Player,
  Enemy,
  Populance,
}

impl Alignment {
  pub fn is_hostile_to(&self, other: Alignment) -> bool {
    match self {
      Alignment::Player => other == Alignment::Enemy,
      Alignment::Enemy => other != Alignment::Enemy,
      Alignment::Populance => false,
    }
  }
}
