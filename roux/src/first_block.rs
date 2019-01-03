use cube::{Cube, Face};
use solver::index::{generic_edge_index, generic_edge_index_decode, Index};

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

#[cfg(test)]
mod tests {
  use super::*;
  use solver::index::exhaustive_index_check;

  #[test]
  fn exhaustive_fbe() {
    exhaustive_index_check::<FBEdges>();
  }
}
