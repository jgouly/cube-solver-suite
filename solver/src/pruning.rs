/// Generate a pruning table for the `Index` represented by `trans_table`.
pub fn gen_prune_table(
  trans_table: &[[u32; 7]],
  max_depth: u32,
  init_index: u32,
) -> Box<[u8]> {
  let mut table = vec![std::u8::MAX; trans_table.len()];
  gen_prune_table_inner(init_index, &mut table, trans_table, max_depth + 1, 0);
  debug_assert!(table.iter().all(|&v| v != std::u8::MAX));
  debug_assert!(table.iter().all(|&v| v < (max_depth + 1) as u8));
  table.into_boxed_slice()
}

fn gen_prune_table_inner(
  index: u32,
  prune_table: &mut [u8],
  trans_table: &[[u32; 7]],
  max_depth: u32,
  depth: u32,
) {
  // End the current search branch if max_depth is reached or the current
  // index was already reached at a lower depth.
  if depth == max_depth || prune_table[index as usize] as u32 <= depth {
    return;
  }
  // Save the current depth for this index.
  prune_table[index as usize] = depth as u8;
  for i in 0..7 {
    let mut new_index = index;
    for _ in 0..3 {
      new_index = trans_table[new_index as usize][i];
      gen_prune_table_inner(
        new_index,
        prune_table,
        trans_table,
        max_depth,
        depth + 1,
      );
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::index::example::UF;
  use crate::index::Index;
  use crate::transition::gen_transition_table;
  use cube::{Cube, Move};

  #[test]
  fn minimal_uf() {
    let uf = UF;
    let table = gen_transition_table(&uf);
    let ptable = gen_prune_table(&table, 2, 0);
    assert_eq!(2, *ptable.iter().max().unwrap());

    let mut c = Cube::solved();
    assert_eq!(0, ptable[uf.from_cube(&c) as usize]);
    c.do_move(Move::Face(cube::Face::U, 1));
    assert_eq!(1, ptable[uf.from_cube(&c) as usize]);
    c.do_move(Move::Face(cube::Face::U, 1));
    assert_eq!(1, ptable[uf.from_cube(&c) as usize]);
    c.do_move(Move::Face(cube::Face::B, 1));
    assert_eq!(2, ptable[uf.from_cube(&c) as usize]);
    c.do_move(Move::Face(cube::Face::B, 1));
    assert_eq!(1, ptable[uf.from_cube(&c) as usize]);
  }
}
