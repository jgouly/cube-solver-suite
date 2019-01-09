mod parser;
pub mod sticker_cube;

pub use crate::parser::parse_moves;
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

  /// Get the `Face` that is opposite to `self`.
  pub fn opposite(&self) -> Face {
    match self {
      Face::U => Face::D,
      Face::D => Face::U,
      Face::F => Face::B,
      Face::B => Face::F,
      Face::R => Face::L,
      Face::L => Face::R,
    }
  }

  /// Test if `face` is an opposite `Face`.
  pub fn is_opposite(&self, face: Face) -> bool {
    self.slice() == face.slice()
  }
}

impl std::fmt::Display for Face {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/// Represents a slice of the cube.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Slice {
  M,
  E,
  S,
}

impl std::fmt::Display for Slice {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/// Represents a rotation of the cube.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rotation {
  X,
  Y,
  Z,
}

impl std::fmt::Display for Rotation {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let rotations = ["x", "y", "z"];
    write!(f, "{}", rotations[*self as usize])
  }
}

/// Represents a move of the cube.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Move {
  Face(Face, u8),
  Slice(Slice, u8),
  Rotation(Rotation, u8),
}

impl Move {
  /// Get the amount of 90 degree turns. Returns 1, 2 or 3.
  pub fn amount(&self) -> u8 {
    match self {
      Move::Face(_, a) | Move::Slice(_, a) | Move::Rotation(_, a) => *a,
    }
  }

  /// Create a new move with the same movement, but different amount.
  pub fn with_amount(&self, amount: u8) -> Move {
    match self {
      Move::Face(f, _) => Move::Face(*f, amount),
      Move::Slice(s, _) => Move::Slice(*s, amount),
      Move::Rotation(s, _) => Move::Rotation(*s, amount),
    }
  }

  /// Compare `m` to `self`, returning true if they have the same 'movement'.
  /// This essentially compares the moves, but ignoring the amount field.
  pub fn is_same_movement(&self, m: &Move) -> bool {
    self.with_amount(0) == m.with_amount(0)
  }
}

impl std::fmt::Display for Move {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let amounts = ["", "2", "'"];
    match self {
      Move::Face(face, amt) => {
        write!(f, "{}{}", face, amounts[*amt as usize - 1])
      }
      Move::Slice(slice, amt) => {
        write!(f, "{}{}", slice, amounts[*amt as usize - 1])
      }
      Move::Rotation(slice, amt) => {
        write!(f, "{}{}", slice, amounts[*amt as usize - 1])
      }
    }
  }
}
