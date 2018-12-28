pub mod sticker_cube;

/// Represents a face of the cube.
#[derive(Clone, Copy, Debug)]
pub enum Face {
  U,
  D,
  F,
  B,
  R,
  L,
}
