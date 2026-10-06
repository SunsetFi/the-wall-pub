pub struct UnionFind {
  parent: Vec<usize>,
}

impl UnionFind {
  pub fn new(size: usize) -> Self {
    Self {
      // Initialize each element to be its own parent
      parent: (0..size).collect(),
    }
  }

  pub fn reset(&mut self) {
    for (i, value) in self.parent.iter_mut().enumerate() {
      *value = i;
    }
  }

  pub fn set(&mut self, index: usize, parent: usize) {
    self.parent[index] = parent;
  }

  pub fn find(&mut self, label: usize) -> usize {
    let parent = &mut self.parent;
    let mut root = label;
    while parent[root] != root {
      parent[root] = parent[parent[root]];
      root = parent[root];
    }

    root
  }

  pub fn union(&mut self, a: usize, b: usize) -> usize {
    let root_a = self.find(a);
    let root_b = self.find(b);
    if root_a == root_b {
      return root_a;
    }

    if root_a < root_b {
      self.parent[root_b] = root_a;
      root_a
    } else {
      self.parent[root_a] = root_b;
      root_b
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn from_parents(parent: &[usize]) -> UnionFind {
    UnionFind {
      parent: parent.to_vec(),
    }
  }

  mod new {
    use super::super::*;

    #[test]
    fn it_creates_new_of_size() {
      let uf = UnionFind::new(3);
      assert_eq!(uf.parent.len(), 3);
    }

    #[test]
    fn it_creates_new_with_own_parents() {
      let uf = UnionFind::new(3);
      assert_eq!(uf.parent, vec![0, 1, 2]);
    }
  }

  mod reset {
    use super::*;

    #[test]
    fn it_sets_parents() {
      let mut uf = from_parents(&[2, 0, 1]);
      uf.reset();
      assert_eq!(uf.parent, vec![0, 1, 2]);
    }
  }

  mod find {
    use super::*;

    #[test]
    fn it_finds_root() {
      let mut uf = from_parents(&[0, 0, 1]);

      assert_eq!(uf.find(2), 0);
    }

    #[test]
    fn it_finds_itself() {
      let mut uf = from_parents(&[0]);

      assert_eq!(uf.find(0), 0);
    }

    #[test]
    fn it_halves_paths() {
      let mut uf = from_parents(&[0, 0, 1, 2]);

      uf.find(3);
      assert_eq!(uf.parent, vec![0, 0, 1, 1]);
    }
  }
  mod union {
    use super::*;

    #[test]
    fn it_returns_root_when_same() {
      let mut uf = from_parents(&[0, 0]);
      assert_eq!(uf.union(0, 1), 0);
      assert_eq!(uf.parent, vec![0, 0]);
    }

    #[test]
    fn it_picks_smaller_left() {
      let mut uf = from_parents(&[0, 1]);
      assert_eq!(uf.union(0, 1), 0);
    }

    #[test]
    fn it_picks_smaller_right() {
      let mut uf = from_parents(&[0, 1]);
      assert_eq!(uf.union(1, 0), 0);
    }

    #[test]
    fn it_links_left() {
      let mut uf = from_parents(&[0, 0, 2, 2]);

      uf.union(1, 2);
      assert_eq!(uf.parent, vec![0, 0, 0, 2]);
    }

    #[test]
    fn it_links_right() {
      let mut uf = from_parents(&[0, 0, 2, 2]);

      uf.union(2, 1);
      assert_eq!(uf.parent, vec![0, 0, 0, 2]);
    }
  }
}
