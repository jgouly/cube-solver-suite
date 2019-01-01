pub mod sticker_cube;

pub use crate::sticker_cube::CentrePos;
pub use crate::sticker_cube::Cube;

/// Represents a face of the cube.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Face {
  U,
  D,
  F,
  B,
  R,
  L,
}

impl Face {
  fn slice(&self) -> Slice {
    match self {
      Face::U => Slice::E,
      Face::D => Slice::E,

      Face::R => Slice::M,
      Face::L => Slice::M,

      Face::F => Slice::S,
      Face::B => Slice::S,
    }
  }

  /// Test if `face` is an opposite `Face`.
  pub fn is_opposite(&self, face: Face) -> bool {
    self.slice() == face.slice()
  }
}

/// Represents a slice of the cube.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Slice {
  M,
  E,
  S,
}

/// Represents a move of the cube.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Move {
  Face(Face, u8),
  Slice(Slice, u8),
}

impl Move {
  /// Get the amount of 90 degree turns. Returns 1, 2 or 3.
  pub fn amount(&self) -> u8 {
    match self {
      Move::Face(_, a) | Move::Slice(_, a) => *a,
    }
  }

  /// Create a new move with the same movement, but different amount.
  pub fn with_amount(&self, amount: u8) -> Move {
    match self {
      Move::Face(f, _) => Move::Face(*f, amount),
      Move::Slice(s, _) => Move::Slice(*s, amount),
    }
  }
}
