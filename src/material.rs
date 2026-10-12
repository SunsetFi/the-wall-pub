/// Material types
/// Entities are typically composed of one material
/// Combat effectiveness is graded per-material rather than
/// per-entity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Material {
  Concrete,
  Organic,
  NanoMetal,
  StructuralMetal,
}
