// Horrible left-over hack stuff inherited from the typescript side when we were iffy between
// 8 way and 4 way interactions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Probe {
  pub dx: isize,
  pub dy: isize,
  pub delta: usize,
}

impl Probe {
  pub fn from_offset((dx, dy): (isize, isize), width: isize) -> Self {
    let delta = dx + dy * width;
    if delta == 0 {
      panic!("Delta cannot be zero");
    }

    let (dx, dy) = if delta > 0 { (-dx, -dy) } else { (dx, dy) };

    Self {
      dx,
      dy,
      delta: delta.unsigned_abs(),
    }
  }

  pub fn from_offsets<T>(offsets: T, width: isize) -> Vec<Self>
  where
    T: IntoIterator<Item = (isize, isize)>,
  {
    offsets
      .into_iter()
      .map(|offset| Self::from_offset(offset, width))
      .collect()
  }
}

#[cfg(test)]
mod tests {

  mod from_offset {
    use super::super::*;

    #[test]
    fn it_creates_a_probe_from_offset() {
      let probe = Probe::from_offset((-1, -1), 3);
      assert_eq!(
        probe,
        Probe {
          dx: -1,
          dy: -1,
          delta: 4
        }
      );
    }

    #[test]
    fn it_orients_backwards() {
      let probe = Probe::from_offset((1, 1), 3);
      assert_eq!(
        probe,
        Probe {
          dx: -1,
          dy: -1,
          delta: 4
        }
      );
    }

    #[test]
    #[should_panic(expected = "Delta cannot be zero")]
    fn it_panics_on_zero_offsets() {
      Probe::from_offset((0, 0), 3);
    }

    #[test]
    #[should_panic(expected = "Delta cannot be zero")]
    fn it_panics_on_range_zero() {
      Probe::from_offset((3, -1), 3);
    }
  }

  mod from_offsets {
    use super::super::*;

    #[test]
    fn it_creates_probes_from_offsets() {
      let probes = Probe::from_offsets(vec![(1, -1), (1, 1)], 3);
      assert_eq!(probes.len(), 2);
      assert_eq!(
        probes[0],
        Probe {
          dx: 1,
          dy: -1,
          delta: 2
        }
      );
      assert_eq!(
        probes[1],
        Probe {
          dx: -1,
          dy: -1,
          delta: 4
        }
      );
    }

    #[test]
    #[should_panic(expected = "Delta cannot be zero")]
    fn it_panics_on_zero_offsets() {
      Probe::from_offsets(vec![(1, 1), (0, 0)], 3);
    }
  }
}
