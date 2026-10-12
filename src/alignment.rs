/// Alignments determine which entities enter combat with each other
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Alignment {
  /// Player alignment.
  /// Only hostile to Enemy.
  Player,

  /// Enemy alignment.
  /// Hostile to everyone.
  Enemy,

  /// Populance within the Wall.
  /// Not hostile to anyone.
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
