use crate::index::Index;
use cube::Move::*;
use cube::{Face, Slice};

/// Generate a transition table for the `Index` `I`.
pub fn gen_transition_table<I: Index>() -> Box<[[u32; 7]]> {
  let mut res = Vec::<[u32; 7]>::with_capacity(I::NUM_ELEMS as usize);

  for n in 0..I::NUM_ELEMS {
    res.push(Default::default());

    let c = I::from_index(n);

    for (i, &f) in [
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
      let mut c2 = c;
      c2.do_move(f);
      let n2 = I::from_cube(&c2);
      res[n as usize][i] = n2;
    }
  }
  res.into_boxed_slice()
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::Cube;

  #[test]
  fn minimal_uf() {
    use crate::index::example::UF;
    let table = gen_transition_table::<UF>();

    let mut c = Cube::solved();
    let solved_index = UF::from_cube(&c);
    c.do_move(Face(Face::U, 1));
    let u_index = UF::from_cube(&c);
    assert_eq!(u_index, table[solved_index as usize][0]);
    assert_eq!(solved_index, table[solved_index as usize][1]);

    let mut c = Cube::solved();
    c.do_move(Face(Face::F, 1));
    let f_index = UF::from_cube(&c);
    assert_eq!(f_index, table[solved_index as usize][2]);
    assert_eq!(f_index, table[f_index as usize][0]);
  }
}
