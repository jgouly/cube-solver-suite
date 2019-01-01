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

/// Represents a move of the cube.
#[derive(Clone, Copy, Debug)]
pub enum Move {
  U(u8),
  D(u8),
  F(u8),
  B(u8),
  R(u8),
  L(u8),
  M(u8),
}

impl Move {
  /// Get the amount of 90 degree turns. Returns 1, 2 or 3.
  pub fn amount(&self) -> u8 {
    match self {
      Move::U(a)
      | Move::D(a)
      | Move::F(a)
      | Move::B(a)
      | Move::R(a)
      | Move::L(a)
      | Move::M(a) => *a,
    }
  }

  /// Create a new move with the same movement, but different amount.
  pub fn with_amount(&self, amount: u8) -> Move {
    match self {
      Move::U(_) => Move::U(amount),
      Move::D(_) => Move::D(amount),
      Move::F(_) => Move::F(amount),
      Move::B(_) => Move::B(amount),
      Move::R(_) => Move::R(amount),
      Move::L(_) => Move::L(amount),
      Move::M(_) => Move::M(amount),
    }
  }
}
