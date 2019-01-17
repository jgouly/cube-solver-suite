use crate::first_block::FBInfo;
use cube::{Cube, Face, Move::*, Slice};
use solver::iddfs::IDDFSInfo;
use solver::index::generic_corner_index;
use solver::index::generic_corner_index_decode;
use solver::index::Index;

struct CMLL;

impl Index for CMLL {
  const NUM_ELEMS: u32 = 24 * 21 * 18 * 15;

  fn from_cube(&self, c: &Cube) -> u32 {
    generic_corner_index(
      &c,
      &[
        (Face::U, Face::R, Face::F),
        (Face::U, Face::F, Face::L),
        (Face::U, Face::L, Face::B),
        (Face::U, Face::B, Face::R),
      ],
    )
  }

  fn from_index(&self, i: u32) -> Cube {
    let mut c = Cube::invalid();
    generic_corner_index_decode(
      &mut c,
      i,
      &[
        (Face::U, Face::B, Face::R),
        (Face::U, Face::L, Face::B),
        (Face::U, Face::F, Face::L),
        (Face::U, Face::R, Face::F),
      ],
    );
    c
  }
}

pub struct CMLLInfo(FBInfo, Box<[[u32; 7]]>, Box<[u8]>);

impl CMLLInfo {
  pub fn new() -> Self {
    use solver::pruning::*;
    use solver::transition::*;
    let c = Cube::solved();
    let table = gen_transition_table(&CMLL);
    let ptable = gen_prune_table(&table, 7, CMLL.from_cube(&c));
    CMLLInfo(FBInfo::new(), table, ptable)
  }
}

fn sb_edge_index(c: &Cube) -> u32 {
  use solver::index::generic_edge_index;
  generic_edge_index(
    &c,
    &[(Face::D, Face::R), (Face::F, Face::R), (Face::B, Face::R)],
  )
}

fn sb_corner_index(c: &Cube) -> u32 {
  generic_corner_index(
    &c,
    &[(Face::D, Face::F, Face::R), (Face::D, Face::R, Face::B)],
  )
}

fn sb_solved(c: &Cube) -> bool {
  sb_edge_index(c) == sb_edge_index(&Cube::solved())
    && sb_corner_index(c) == sb_corner_index(&Cube::solved())
}

impl IDDFSInfo for CMLLInfo {
  type State = (Cube, u32, <FBInfo as IDDFSInfo>::State);

  fn is_solved(&self, (c, cmll, fb): &Self::State) -> bool {
    self.0.is_solved(fb)
      && *cmll == CMLL.from_cube(&Cube::solved())
      && sb_solved(&c)
  }

  fn transition(
    &self,
    (c, cmll, fb): &Self::State,
    m: usize,
  ) -> Self::State {
    let mut c = *c;
    // note: this matches the order in solver/src/iddfs.rs
    let moves = [
      Face(Face::U, 1),
      Face(Face::D, 1),
      Face(Face::F, 1),
      Face(Face::B, 1),
      Face(Face::R, 1),
      Face(Face::L, 1),
      Slice(Slice::M, 1),
    ];
    c.do_move(moves[m]);
    (c, self.1[*cmll as usize][m], self.0.transition(fb, m))
  }

  fn prune(&self, state: &Self::State, depth_remaining: usize) -> bool {
    self.0.prune(&state.2, depth_remaining)
      || depth_remaining < self.2[state.1 as usize] as usize
  }
}

impl CMLLInfo {
  pub fn get_state(&self, c: &Cube) -> <Self as IDDFSInfo>::State {
    (*c, CMLL.from_cube(&c), self.0.get_state(&c))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::parse_moves;
  use solver::iddfs::iddfs;
  use solver::index::exhaustive_index_check;

  #[test]
  fn exhaustive_cmll() {
    exhaustive_index_check(&CMLL);
  }

  #[test]
  fn basic() {
    let c = Cube::solved();
    let info = CMLLInfo::new();
    let solved = iddfs(info.get_state(&c), &info, 0, &mut Vec::new());
    assert!(solved);

    let mut c = Cube::solved();
    c.do_moves(&parse_moves("R U2 R' U' R U' R'").unwrap());

    let mut solution = Vec::new();
    let solved = iddfs(info.get_state(&c), &info, 7, &mut solution);
    assert!(solved);
    assert_eq!(parse_moves("F' U2 F U F' U F").unwrap(), solution);
  }
}
