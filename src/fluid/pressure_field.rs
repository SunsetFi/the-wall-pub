use crate::geometry::grid::CellIdx;

pub struct PressureField {
  pressures: Vec<f64>,
}

impl PressureField {
  pub fn new(size: usize) -> Self {
    Self {
      pressures: vec![0.0; size],
    }
  }

  pub fn get(&self, idx: CellIdx) -> f64 {
    self.pressures[idx.index()]
  }

  pub fn set(&mut self, idx: CellIdx, value: f64) {
    self.pressures[idx.index()] = value;
  }
}

#[cfg(test)]
mod tests {
  use super::PressureField;
  use crate::geometry::grid::Grid;
  use crate::micro_entity::MicroCollection;

  #[test]
  fn it_initializes_with_zeros() {
    let micros = MicroCollection::new(Grid::new(5, 5));
    let field = PressureField::new(micros.size());

    for i in micros.iter_idx() {
      assert_eq!(field.get(i), 0.0);
    }
  }

  #[test]
  fn it_sets_and_gets_values() {
    let micros = MicroCollection::new(Grid::new(5, 5));
    let mut field = PressureField::new(micros.size());

    for i in micros.iter_idx() {
      field.set(i, 42.0);
    }

    for i in micros.iter_idx() {
      assert_eq!(field.get(i), 42.0);
    }
  }
}
