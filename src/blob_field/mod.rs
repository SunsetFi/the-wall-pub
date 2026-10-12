mod probe;
mod union_find;

use crate::MicroKind;
use crate::blob_field::probe::Probe;
use crate::blob_field::union_find::UnionFind;
use crate::geometry::Coordinate;
use crate::geometry::grid::Grid;
use crate::micro_entity::{MicroCollection, MicroSlot};

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct FieldLabel(usize);

impl FieldLabel {
  fn new(index: usize) -> Self {
    Self(index)
  }

  pub fn index(self) -> usize {
    self.0
  }
}

#[derive(Copy, Default, Clone, Debug)]
struct BlobMetrics {
  pub kind: Option<MicroKind>,
  pub mass: f64,
  pub centroid_x: f64,
  pub centroid_y: f64,
  pub cell_count: u32,
}

impl BlobMetrics {
  fn new(kind: MicroKind) -> Self {
    Self {
      kind: Some(kind),
      mass: 0.0,
      centroid_x: 0.0,
      centroid_y: 0.0,
      cell_count: 0,
    }
  }
}

pub struct BlobField {
  probes: Vec<Probe>,
  labels: Vec<Option<FieldLabel>>,
  metrics: Vec<BlobMetrics>,
  width: u32,
  height: u32,
  count: u32,
  uf: UnionFind,
}

const PAIR_OFFSETS: [(isize, isize); 2] = [(-1, 0), (0, -1)];

impl BlobField {
  pub fn new(grid: Grid) -> Self {
    let size = grid.size();
    let width = grid.width();
    let height = grid.height();
    Self {
      probes: Probe::from_offsets(PAIR_OFFSETS, width as isize),
      metrics: vec![BlobMetrics::default(); size],
      labels: vec![Option::<FieldLabel>::None; size],
      width,
      height,
      count: 0,
      uf: UnionFind::new(size),
    }
  }

  pub fn process(&mut self, micros: &MicroCollection) {
    self.scan(micros);
    self.flatten(micros);
  }

  pub fn label(&self, ord: u32) -> Option<FieldLabel> {
    let ord = ord as usize;
    if ord >= self.labels.len() {
      return None;
    }
    Some(FieldLabel::new(ord))
  }

  pub fn label_at_cell(&self, x: u32, y: u32) -> Option<FieldLabel> {
    let ordinal = Self::ordinal(x, y, self.width);
    self.labels[ordinal]
  }

  pub fn label_at_ordinal(&self, ordinal: usize) -> Option<FieldLabel> {
    self.labels[ordinal]
  }

  pub fn kind_of(&self, label: FieldLabel) -> Option<MicroKind> {
    self.metric(label).and_then(|m| m.kind)
  }

  pub fn mass_of(&self, label: FieldLabel) -> f64 {
    self.metric(label).map_or(0.0, |m| m.mass)
  }

  pub fn centroid_x_of(&self, label: FieldLabel) -> f64 {
    self.metric(label).map_or(-1.0, |m| m.centroid_x)
  }

  pub fn centroid_y_of(&self, label: FieldLabel) -> f64 {
    self.metric(label).map_or(-1.0, |m| m.centroid_y)
  }

  pub fn cells_at_label(&self, label: FieldLabel) -> u32 {
    self.metric(label).map_or(0, |m| m.cell_count)
  }

  pub fn count(&self) -> u32 {
    self.count
  }

  fn metric(&self, label: FieldLabel) -> Option<&BlobMetrics> {
    self.metrics.get(label.index())
  }

  fn scan(&mut self, micros: &MicroCollection) {
    self.labels.fill(None);
    self.metrics.fill(BlobMetrics::default());

    // We don't reset uf, as we only ever walk cells backwards,
    // so all ordinals we might interact with will have been
    // set by then.

    for y in 0..self.height {
      for x in 0..self.width {
        let ordinal = Self::ordinal(x, y, self.width);

        let Some(key) = micros.cell(x, y).and_then(|m| m.kind()) else {
          self.labels[ordinal] = None;
          continue;
        };

        let mut label: Option<usize> = None;
        for probe in &self.probes {
          // If delta drops us negative, we are up against the y boundary.
          let Some(neighbor) = ordinal.checked_sub(probe.delta) else {
            continue;
          };

          // We do not need to check delta being too high, as it's always
          // pointing backwards.
          // This also means we don't need to check y >= height

          // We still need to check the x boundary
          if x == 0 && probe.dx < 0 {
            continue;
          }
          if x >= self.width - 1 && probe.dx > 0 {
            continue;
          }

          if micros.ordinal(neighbor as u32).and_then(|s| s.kind()) != Some(key) {
            continue;
          }

          // Our key is not empty, so if the keys match, the neighbor
          // is not empty.
          // Because we only scan backwards through the ordinals,
          // we WILL have visited other before, which means other
          // is already labeled.
          let other = self.labels[neighbor].unwrap().index();

          label = Some(match label {
            Some(current) => self.uf.union(current, other),
            None => self.uf.find(other),
          });
        }

        let final_label = label.unwrap_or_else(|| {
          self.uf.set(ordinal, ordinal);
          ordinal
        });

        self.labels[ordinal] = Some(FieldLabel::new(final_label));
      }
    }
  }

  fn flatten(&mut self, micros: &MicroCollection) {
    let width = self.width;
    let height = self.height;

    let mut remap = vec![Option::<FieldLabel>::None; self.labels.len()];
    let mut next: usize = 0;
    for y in 0..height {
      for x in 0..width {
        let idx = micros.grid().cell(x, y).unwrap();
        let index = idx.index();
        let label = self.labels[index];
        let Some(label) = label.map(|l| l.index()) else {
          continue;
        };

        let root = self.uf.find(label);
        let dense = remap[root];

        let micro = micros.get(idx);

        let (dense, blob_index) = if let Some(dense) = dense {
          (dense, dense.index())
        } else {
          let label = FieldLabel::new(next);
          next += 1;

          let dense = Some(label);
          remap[root] = dense;

          let field = label.index();
          self.metrics[field] = BlobMetrics::new(micro.kind().unwrap());
          (label, field)
        };

        self.labels[index] = Some(dense);

        Self::merge_metric(&mut self.metrics[blob_index], micro, Coordinate::new(x, y));
      }
    }

    self.count = next.try_into().unwrap();

    for metric in &mut self.metrics {
      if metric.cell_count > 0 {
        metric.centroid_x /= metric.mass;
        metric.centroid_y /= metric.mass;
      }
    }
  }

  fn merge_metric(metrics: &mut BlobMetrics, micro: &MicroSlot, p: Coordinate) {
    let mass: f64 = micro.density().unwrap().into();
    metrics.mass += mass;
    metrics.centroid_x += (p.x as f64) * mass;
    metrics.centroid_y += (p.y as f64) * mass;
    metrics.cell_count += 1;
  }

  fn ordinal(x: u32, y: u32, width: u32) -> usize {
    (x + y * width) as usize
  }
}

#[cfg(test)]
mod tests {

  mod new {
    use crate::geometry::grid::Grid;

    use super::super::*;

    #[test]
    fn it_creates_a_field() {
      let grid = Grid::new(3, 2);
      let bf = BlobField::new(grid);
      assert_eq!(bf.probes.len(), 2, "Probe length should be 2");
      assert_eq!(bf.width, 3, "width should be 3");
      assert_eq!(bf.height, 2, "height should be 2");
    }
  }

  mod process {
    use crate::{
      MicroKind,
      geometry::grid::Grid,
      micro_entity::{Density, MicroEnt},
    };

    use super::super::*;

    // One-letter alias to fit with one-digit array grids.
    const E: Option<MicroKind> = None;
    const A: Option<MicroKind> = Some(MicroKind::Friendly);
    const B: Option<MicroKind> = Some(MicroKind::Enemy);
    const C: Option<MicroKind> = Some(MicroKind::StructureConstructor);

    fn row(items: Vec<Option<MicroKind>>) -> Vec<Option<MicroKind>> {
      items
    }

    fn field(rows: Vec<Vec<Option<MicroKind>>>) -> Vec<Vec<Option<MicroKind>>> {
      rows
    }

    fn populate_world(world: &mut MicroCollection, field: Vec<Vec<Option<MicroKind>>>) {
      for (y, row) in field.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
          if let Some(value) = value {
            world
              .cell_mut(x as u32, y as u32)
              .unwrap()
              .set_ent(MicroEnt::new(value, Density::MAX));
          }
        }
      }
    }

    fn assert_field(bf: &BlobField, expected: &[&[Option<FieldLabel>]]) {
      for (y, row) in expected.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
          assert_eq!(
            bf.label_at_cell(x as u32, y as u32),
            value,
            "Value at position ({x}, {y}) does not match."
          );
        }
      }
    }

    fn assert_unique<T: Eq + std::hash::Hash + std::fmt::Debug>(values: &[T]) {
      let mut seen = std::collections::HashSet::new();
      for value in values {
        assert!(seen.insert(value), "Value {value:?} is not unique");
      }
    }

    #[test]
    fn it_groups_like_keys() {
      let grid = Grid::new(3, 2);
      let mut bf = BlobField::new(grid);

      let field = field(vec![row(vec![A, E, A]), row(vec![A, A, E])]);
      let mut micros = MicroCollection::new(grid);
      populate_world(&mut micros, field);
      bf.process(&micros);

      let l1 = bf.label_at_cell(0, 0);
      let l2 = bf.label_at_cell(2, 0);
      assert!(l1.is_some());
      assert!(l2.is_some());
      assert_unique(&[l1, l2]);

      assert_field(&bf, &[&[l1, None, l2], &[l1, l1, None]]);
    }

    #[test]
    fn it_does_not_combine_unlike_keys() {
      let grid = Grid::new(2, 2);
      let mut micros = MicroCollection::new(grid);
      let mut bf = BlobField::new(grid);

      let field = field(vec![row(vec![A, E]), row(vec![A, B])]);
      populate_world(&mut micros, field);
      bf.process(&micros);

      let l1 = bf.label_at_cell(0, 0);
      let l2 = bf.label_at_cell(1, 1);
      assert!(l1.is_some());
      assert!(l2.is_some());
      assert_unique(&[l1, l2]);

      assert_ne!(l1, l2);

      assert_field(&bf, &[&[l1, None], &[l1, l2]]);
    }

    #[test]
    fn it_handles_all_unique_keys() {
      let grid = Grid::new(2, 2);
      let mut micros = MicroCollection::new(grid);
      let mut bf = BlobField::new(grid);
      let field = field(vec![row(vec![A, B]), row(vec![C, E])]);
      populate_world(&mut micros, field);
      bf.process(&micros);

      let l1 = bf.label_at_cell(0, 0);
      let l2 = bf.label_at_cell(1, 0);
      let l3 = bf.label_at_cell(0, 1);
      let l4 = bf.label_at_cell(1, 1);
      assert!(l1.is_some());
      assert!(l2.is_some());
      assert!(l3.is_some());
      assert!(l4.is_none());
      assert_unique(&[l1, l2, l3, l4]);

      assert_field(&bf, &[&[l1, l2], &[l3, l4]]);
    }

    #[test]
    fn it_joins_blobs() {
      let grid = Grid::new(5, 3);
      let mut micros = MicroCollection::new(grid);
      let mut bf = BlobField::new(grid);
      let field = field(vec![
        row(vec![E, E, E, E, A]),
        row(vec![A, A, E, A, A]),
        row(vec![E, A, A, A, B]),
      ]);
      populate_world(&mut micros, field);
      bf.process(&micros);
      let l1 = bf.label_at_cell(4, 0);
      let l2 = bf.label_at_cell(4, 2);
      assert!(l1.is_some());
      assert!(l2.is_some());
      assert_unique(&[l1, l2]);

      assert_field(
        &bf,
        &[
          &[None, None, None, None, l1],
          &[l1, l1, None, l1, l1],
          &[None, l1, l1, l1, l2],
        ],
      );
    }

    #[test]
    fn it_flattens_labels() {
      let grid = Grid::new(5, 3);
      let mut micros = MicroCollection::new(grid);
      let mut bf = BlobField::new(grid);
      let field = field(vec![
        row(vec![E, E, E, E, A]),
        row(vec![A, A, E, A, A]),
        row(vec![E, A, A, A, B]),
      ]);
      populate_world(&mut micros, field);
      bf.process(&micros);

      let l1 = bf.label_at_cell(4, 0);
      let l2 = bf.label_at_cell(4, 2);

      assert_eq!(l1.map(|l| l.index()), Some(0));
      assert_eq!(l2.map(|l| l.index()), Some(1));
    }
  }
}
