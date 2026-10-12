/// Represents the permeability of one type to another.
/// Permeabilities are in increasing order of restrictiveness.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Permeability {
  /// Fully permeable, no restrictions.
  Open = 0,
  /// Permeable at a Wall-infrastructure level.
  Infrastructure = 1,
  /// Fully solid.  Not typically permeable.
  Solid = 2,
}

impl Permeability {
  pub fn is_permeable_to(self, to: Permeability) -> bool {
    self <= to
  }
}

#[cfg(test)]
mod test {
  mod is_permeable_to {
    use super::super::Permeability;

    #[test]
    fn it_is_permeable_to_next() {
      assert!(Permeability::Open.is_permeable_to(Permeability::Infrastructure));
    }

    #[test]
    fn it_is_permeable_to_self() {
      assert!(Permeability::Infrastructure.is_permeable_to(Permeability::Infrastructure));
    }

    #[test]
    fn it_is_not_permeable_to_next() {
      assert!(!Permeability::Solid.is_permeable_to(Permeability::Infrastructure));
    }
  }
}
