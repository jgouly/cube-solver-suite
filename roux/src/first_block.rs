use cube::{Cube, Face};
use solver::iddfs::IDDFSInfo;
use solver::index::{
  generic_corner_index, generic_corner_index_decode, generic_edge_index,
  generic_edge_index_decode, Index,
};

/// Edges of the first block (DL, FL, BL).
pub struct FBEdges;

impl Index for FBEdges {
  const NUM_ELEMS: u32 = 24 * 22 * 20;

  fn from_cube(c: &Cube) -> u32 {
    generic_edge_index(
      &c,
      &[(Face::D, Face::L), (Face::F, Face::L), (Face::B, Face::L)],
    )
  }

  fn from_index(i: u32) -> Cube {
    let mut c = Cube::invalid();
    generic_edge_index_decode(
      &mut c,
      i,
      &[(Face::B, Face::L), (Face::F, Face::L), (Face::D, Face::L)],
    );
    c
  }
}

/// Corners of the first block (DLF, DBL).
pub struct FBCorners;

impl Index for FBCorners {
  const NUM_ELEMS: u32 = 24 * 21;

  fn from_cube(c: &Cube) -> u32 {
    generic_corner_index(
      &c,
      &[(Face::D, Face::L, Face::F), (Face::D, Face::B, Face::L)],
    )
  }

  fn from_index(i: u32) -> Cube {
    let mut c = Cube::invalid();
    generic_corner_index_decode(
      &mut c,
      i,
      &[(Face::D, Face::B, Face::L), (Face::D, Face::L, Face::F)],
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
    *state == (FBEdges::from_cube(&c), FBCorners::from_cube(&c))
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
  pub fn get_state(&self, c: &Cube) -> <Self as IDDFSInfo>::State {
    (FBEdges::from_cube(&c), FBCorners::from_cube(&c))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use solver::iddfs::iddfs;
  use solver::index::exhaustive_index_check;
  use solver::pruning::gen_prune_table;
  use solver::transition::gen_transition_table;

  #[test]
  fn exhaustive_fbe() {
    exhaustive_index_check::<FBEdges>();
  }

  #[test]
  fn exhaustive_fbc() {
    exhaustive_index_check::<FBCorners>();
  }

  #[test]
  fn basic_fb() {
    let c = Cube::solved();
    let e_table = gen_transition_table::<FBEdges>();
    let e_ptable = gen_prune_table(&e_table, 7, FBEdges::from_cube(&c));
    let c_table = gen_transition_table::<FBCorners>();
    let c_ptable = gen_prune_table(&c_table, 4, FBCorners::from_cube(&c));
    let info = FBInfo(e_table, c_table, e_ptable, c_ptable);

    let c = Cube::solved();
    let solved = iddfs(info.get_state(&c), &info, 0, &mut Vec::new());
    assert!(solved);

    let mut c = Cube::solved();
    c.do_move(cube::Move::Face(cube::Face::R, 1));
    let solved = iddfs(info.get_state(&c), &info, 0, &mut Vec::new());
    assert!(solved);

    let mut c = Cube::solved();
    c.do_move(cube::Move::Face(cube::Face::D, 1));
    let solved = iddfs(info.get_state(&c), &info, 0, &mut Vec::new());
    assert!(!solved);

    let mut c = Cube::solved();
    c.do_move(cube::Move::Face(cube::Face::D, 1));
    let solved = iddfs(info.get_state(&c), &info, 1, &mut Vec::new());
    assert!(solved);
  }
}
