use cube::{
  Face,
  Move::{self, *},
  Slice,
};

/// A trait to customise the IDDFS algorithm.
pub trait IDDFSInfo {
  /// Type of the state for the current search.
  type State: Copy;

  /// Check if `state` is solved.
  fn is_solved(&self, state: &Self::State) -> bool;

  /// Used to transition from one state to the next.
  fn transition(&self, state: &Self::State, m: usize) -> Self::State;

  /// Should the search prune the search for `state`.
  fn prune(&self, state: &Self::State, depth_remaining: usize) -> bool;
}

/// Iterative deepening depth first search.
pub fn iddfs<I: IDDFSInfo>(
  state: I::State,
  info: &I,
  depth_remaining: usize,
  solution: &mut Vec<Move>,
) -> bool {
  if depth_remaining == 0 {
    return info.is_solved(&state);
  }

  for (i, &m) in [
    Face(Face::U, 1),
    Face(Face::D, 1),
    Face(Face::F, 1),
    Face(Face::B, 1),
    Face(Face::R, 1),
    Face(Face::L, 1),
    Slice(Slice::M, 1),
  ]
  .iter()
  .enumerate()
  {
    if skip_face(m, solution) {
      continue;
    }

    let mut next = state;
    for n in 1..4 {
      next = info.transition(&next, i);

      if info.prune(&next, depth_remaining - 1) {
        continue;
      }

      solution.push(m.with_amount(n));
      if iddfs::<I>(next, info, depth_remaining - 1, solution) {
        return true;
      }
      solution.pop();
    }
  }
  false
}

/// Should the `Move` `m` be skipped.
fn skip_face(m: Move, solution: &[Move]) -> bool {
  let len = solution.len();
  if len > 0 {
    // Check for A A.
    let prev_move = solution[len - 1];
    if m.is_same_movement(&prev_move) {
      return true;
    }

    if len > 1 {
      // Check for A B A where A and B are opposite faces.
      match (&m, &solution[len - 2..]) {
        (&Move::Face(f, _), &[Move::Face(f1, _), Move::Face(f2, _)])
          if f1.is_opposite(f2) && f1 == f =>
        {
          return true;
        }
        _ => (),
      }
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::index::example::UF;
  use crate::pruning::gen_prune_table;
  use crate::transition::gen_transition_table;
  use cube::sticker_cube::EdgePos;

  #[test]
  fn minimal_uf() {
    struct UFInfo(Box<[[u32; 7]]>, Box<[u8]>);

    impl IDDFSInfo for UFInfo {
      type State = u32;

      fn is_solved(&self, &s: &Self::State) -> bool {
        s == 0
      }

      fn transition(&self, state: &Self::State, m: usize) -> Self::State {
        self.0[*state as usize][m]
      }

      fn prune(&self, state: &Self::State, depth_remaining: usize) -> bool {
        (depth_remaining as u8) < self.1[*state as usize]
      }
    }

    let table = gen_transition_table(&UF);
    let ptable = gen_prune_table(&table, 2, 0);
    let info = UFInfo(table, ptable);

    // This can use an EdgePos here since this index only looks at a single edge.
    let solved = iddfs(EdgePos::UF as u32, &info, 0, &mut Vec::new());
    assert!(solved);

    let solved = iddfs(EdgePos::FU as u32, &info, 1, &mut Vec::new());
    assert!(!solved);

    let mut solution = Vec::new();
    let solved = iddfs(EdgePos::FU as u32, &info, 2, &mut solution);
    assert!(solved);
    assert!(match &solution[..] {
      [Face(Face::U, 2), Slice(Slice::M, 1)] => true,
      _ => false,
    });
  }

  #[test]
  fn skip_move_test() {
    assert!(skip_face(Face(Face::R, 2), &[Face(Face::R, 1)]));
    assert!(skip_face(
      Face(Face::R, 1),
      &[Face(Face::R, 1), Face(Face::L, 1)]
    ));
  }
}
