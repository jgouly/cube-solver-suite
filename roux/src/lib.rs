use cube::{Move, Rotation};

pub mod first_block;

macro_rules! r {
  ($($r: ident $n: expr)*) => {
    &[$(Move::Rotation(Rotation::$r, $n),)*]
  }
}

pub static DL_ORIENTATIONS: [&[Move]; 24] = [
  r!(X 2 Y 3),
  r!(X 3 Y 1),
  r!(X 2),
  r!(Y 1 X 1 Y 3),
  r!(X 2 Y 1),
  r!(X 1 Y 3),
  r!(X 2 Y 2),
  r!(Y 1 X 3 Y 1),
  r!(Y 1),
  r!(X 3 Y 3),
  r!(),
  r!(Y 1 X 1 Y 1),
  r!(Y 3),
  r!(X 1 Y 1),
  r!(Y 2),
  r!(Y 1 X 3 Y 3),
  r!(X 3 Y 2),
  r!(Y 3 X 1 Y 2),
  r!(X 3),
  r!(Y 3 X 3 Y 2),
  r!(X 1),
  r!(Y 1 X 1 Y 2),
  r!(X 1 Y 2),
  r!(Y 1 X 3 Y 2),
];

#[cfg(test)]
mod tests {
  use super::*;
  use cube::sticker_cube::EdgePos;
  use cube::Cube;

  #[test]
  fn orientations() {
    let solved = Cube::solved();

    for &e in EdgePos::natural_order() {
      let mut c = Cube::solved();
      c.do_moves(DL_ORIENTATIONS[e as usize]);

      assert_eq!(solved.edges[e as usize], c.edges[EdgePos::DL as usize]);
      assert_eq!(solved.edges[e as usize ^ 1], c.edges[EdgePos::LD as usize]);
    }
  }
}
