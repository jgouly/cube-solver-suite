use cube::{Cube, Face};
use solver::iddfs::IDDFSInfo;
use solver::index::{
  generic_corner_index, generic_corner_index_decode, generic_edge_index,
  generic_edge_index_decode, Index,
};

/// Edges of the first block (DL, FL, BL).
pub struct FBEdges(Face, Face, Face);

impl Default for FBEdges {
  fn default() -> Self {
    FBEdges(Face::D, Face::L, Face::F)
  }
}

impl Index for FBEdges {
  const NUM_ELEMS: u32 = 24 * 22 * 20;

  fn from_cube(&self, c: &Cube) -> u32 {
    generic_edge_index(
      &c,
      &[
        (self.0, self.1),
        (self.2, self.1),
        (self.2.opposite(), self.1),
      ],
    )
  }

  fn from_index(&self, i: u32) -> Cube {
    let mut c = Cube::invalid();
    generic_edge_index_decode(
      &mut c,
      i,
      &[
        (self.2.opposite(), self.1),
        (self.2, self.1),
        (self.0, self.1),
      ],
    );
    c
  }
}

/// Corners of the first block (DLF, DBL).
pub struct FBCorners(Face, Face, Face);

impl Default for FBCorners {
  fn default() -> Self {
    FBCorners(Face::D, Face::L, Face::F)
  }
}

impl Index for FBCorners {
  const NUM_ELEMS: u32 = 24 * 21;

  fn from_cube(&self, c: &Cube) -> u32 {
    generic_corner_index(
      &c,
      &[
        (self.0, self.1, self.2),
        (self.0, self.2.opposite(), self.1),
      ],
    )
  }

  fn from_index(&self, i: u32) -> Cube {
    let mut c = Cube::invalid();
    generic_corner_index_decode(
      &mut c,
      i,
      &[
        (self.0, self.2.opposite(), self.1),
        (self.0, self.1, self.2),
      ],
    );
    c
  }
}

/// IDDFS Info for the first block.
pub struct FBInfo(Box<[[u32; 7]]>, Box<[[u32; 7]]>, Box<[u8]>, Box<[u8]>);

impl IDDFSInfo for FBInfo {
  type State = (u32, u32);

  fn is_solved(&self, state: &Self::State) -> bool {
    let c = Cube::solved();
    let fbe = FBEdges::default();
    let fbc = FBCorners::default();
    *state == (fbe.from_cube(&c), fbc.from_cube(&c))
  }

  fn transition(&self, state: &Self::State, m: usize) -> Self::State {
    (self.0[state.0 as usize][m], self.1[state.1 as usize][m])
  }

  fn prune(&self, state: &Self::State, depth_remaining: usize) -> bool {
    depth_remaining
      < std::cmp::max(self.2[state.0 as usize], self.3[state.1 as usize])
        as usize
  }
}

impl FBInfo {
  pub fn get_indexes(&self, c: &Cube) -> (FBEdges, FBCorners) {
    use cube::sticker_cube::CentrePos;
    let fbe = FBEdges(
      c.centres[CentrePos::D as usize],
      c.centres[CentrePos::L as usize],
      c.centres[CentrePos::F as usize],
    );
    let fbc = FBCorners(
      c.centres[CentrePos::D as usize],
      c.centres[CentrePos::L as usize],
      c.centres[CentrePos::F as usize],
    );
    (fbe, fbc)
  }

  pub fn get_state(&self, c: &Cube) -> <Self as IDDFSInfo>::State {
    let (fbe, fbc) = self.get_indexes(&c);
    (fbe.from_cube(&c), fbc.from_cube(&c))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::Move;
  use solver::iddfs::iddfs;
  use solver::index::exhaustive_index_check;
  use solver::pruning::gen_prune_table;
  use solver::transition::gen_transition_table;

  #[test]
  fn exhaustive_fbe() {
    exhaustive_index_check(&FBEdges::default());
  }

  #[test]
  fn exhaustive_fbc() {
    exhaustive_index_check(&FBCorners::default());
  }

  #[test]
  fn basic_fb() {
    let c = Cube::solved();
    let fbe = FBEdges::default();
    let e_table = gen_transition_table(&fbe);
    let e_ptable = gen_prune_table(&e_table, 7, fbe.from_cube(&c));
    let fbc = FBCorners::default();
    let c_table = gen_transition_table(&fbc);
    let c_ptable = gen_prune_table(&c_table, 4, fbc.from_cube(&c));
    let info = FBInfo(e_table, c_table, e_ptable, c_ptable);

    let c = Cube::solved();
    let solved = iddfs(info.get_state(&c), &info, 0, &mut Vec::new());
    assert!(solved);

    let mut c = Cube::solved();
    c.do_move(Move::Face(cube::Face::R, 1));
    let solved = iddfs(info.get_state(&c), &info, 0, &mut Vec::new());
    assert!(solved);

    let mut c = Cube::solved();
    c.do_move(Move::Face(cube::Face::D, 1));
    let solved = iddfs(info.get_state(&c), &info, 0, &mut Vec::new());
    assert!(!solved);

    let mut c = Cube::solved();
    c.do_move(Move::Face(cube::Face::D, 1));
    let solved = iddfs(info.get_state(&c), &info, 1, &mut Vec::new());
    assert!(solved);

    let mut c = Cube::solved();
    c.do_move(Move::Rotation(cube::Rotation::Y, 1));
    let (fbe, fbc) = info.get_indexes(&c);
    let solved = iddfs(
      (fbe.from_cube(&c), fbc.from_cube(&c)),
      &info,
      0,
      &mut Vec::new(),
    );
    assert!(solved);

    let mut solution = Vec::new();
    c.do_move(Move::Rotation(cube::Rotation::X, 1));
    let solved = iddfs(
      (fbe.from_cube(&c), fbc.from_cube(&c)),
      &info,
      1,
      &mut solution,
    );
    assert!(solved);
    assert_eq!(Move::Face(Face::L, 1), solution[0]);
  }
}
