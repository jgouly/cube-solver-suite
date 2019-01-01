use cube::Move;

/// A trait to customise the IDDFS algorithm.
pub trait IDDFSInfo {
  /// Type of the state for the current search.
  type State: Copy;

  /// Check if `state` is solved.
  fn is_solved(&self, state: &Self::State) -> bool;

  /// Used to transition from one state to the next.
  fn transition(&self, state: &Self::State, m: usize) -> Self::State;
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

  for (i, &f) in [
    Move::U(1),
    Move::D(1),
    Move::F(1),
    Move::B(1),
    Move::R(1),
    Move::L(1),
    Move::M(1),
  ]
  .iter()
  .enumerate()
  {
    let mut next = state;
    for n in 1..4 {
      next = info.transition(&next, i);
      solution.push(f.with_amount(n));
      if iddfs::<I>(next, info, depth_remaining - 1, solution) {
        return true;
      }
      solution.pop();
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::index::example::UF;
  use crate::transition::gen_transition_table;
  use cube::sticker_cube::EdgePos;

  #[test]
  fn minimal_uf() {
    struct UFInfo(Box<[[u32; 7]]>);;

    impl IDDFSInfo for UFInfo {
      type State = u32;
      fn is_solved(&self, &s: &Self::State) -> bool {
        s == 0
      }
      fn transition(&self, state: &Self::State, m: usize) -> Self::State {
        self.0[*state as usize][m]
      }
    }

    let info = UFInfo(gen_transition_table::<UF>());;

    // This can use an EdgePos here since this index only looks at a single edge.
    let solved = iddfs(EdgePos::UF as u32, &info, 0, &mut Vec::new());
    assert!(solved);

    let solved = iddfs(EdgePos::FU as u32, &info, 1, &mut Vec::new());
    assert!(!solved);

    let mut solution = Vec::new();
    let solved = iddfs(EdgePos::FU as u32, &info, 2, &mut solution);
    assert!(solved);
    assert!(match &solution[..] {
      [Move::U(2), Move::M(1)] => true,
      _ => false,
    });
  }
}
