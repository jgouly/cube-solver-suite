use crate::Face;

/// Represents a 3x3x3 cube using a representation that is similar to storing
/// sticker colours. This representation includes centre pieces so can
/// represent slice turns and rotations.
#[derive(Clone, Copy, Debug)]
pub struct Cube {
  pub edges: [Face; 24],
  pub corners: [Face; 24],
  pub centres: [Face; 6],
}

impl Cube {
  pub fn solved() -> Cube {
    let edges = [
      Face::U,
      Face::F,
      Face::U,
      Face::L,
      Face::U,
      Face::B,
      Face::U,
      Face::R,
      Face::D,
      Face::F,
      Face::D,
      Face::L,
      Face::D,
      Face::B,
      Face::D,
      Face::R,
      Face::F,
      Face::R,
      Face::F,
      Face::L,
      Face::B,
      Face::L,
      Face::B,
      Face::R,
    ];

    let corners = [
      Face::U,
      Face::R,
      Face::F,
      Face::U,
      Face::F,
      Face::L,
      Face::U,
      Face::L,
      Face::B,
      Face::U,
      Face::B,
      Face::R,
      Face::D,
      Face::F,
      Face::R,
      Face::D,
      Face::L,
      Face::F,
      Face::D,
      Face::B,
      Face::L,
      Face::D,
      Face::R,
      Face::B,
    ];

    let centres = [Face::U, Face::R, Face::F, Face::D, Face::B, Face::L];
    Cube {
      edges,
      corners,
      centres,
    }
  }
}

/// Represents a particular edge position on a cube.
/// Note: This represents a position, not a particular piece.
#[derive(Clone, Copy, Debug)]
pub enum EdgePos {
  UF,
  FU,
  UL,
  LU,
  UB,
  BU,
  UR,
  RU,
  DF,
  FD,
  DL,
  LD,
  DB,
  BD,
  DR,
  RD,
  FR,
  RF,
  FL,
  LF,
  BL,
  LB,
  BR,
  RB,
}

/// Represents a particular corner position on a cube.
/// Note: This represents a position, not a particular piece.
#[derive(Clone, Copy, Debug)]
pub enum CornerPos {
  URF,
  RFU,
  FUR,
  UFL,
  FLU,
  LUF,
  ULB,
  LBU,
  BUL,
  UBR,
  BRU,
  RUB,
  DFR,
  FRD,
  RDF,
  DLF,
  LFD,
  FDL,
  DBL,
  BLD,
  LDB,
  DRB,
  RBD,
  BDR,
}

/// Represents a particular centre position on a cube.
/// Note: This represents a position, not a particular piece.
#[derive(Clone, Copy, Debug)]
pub enum CentrePos {
  U,
  R,
  F,
  D,
  B,
  L,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solved() {
    let c = Cube::solved();

    macro_rules! assert_edge {
      ($edge_pos: ident, $face: ident) => {
        assert_eq!(Face::$face, c.edges[EdgePos::$edge_pos as usize]);
      };
    }

    assert_edge!(UF, U);
    assert_edge!(FU, F);
    assert_edge!(UL, U);
    assert_edge!(LU, L);
    assert_edge!(UB, U);
    assert_edge!(BU, B);
    assert_edge!(UR, U);
    assert_edge!(RU, R);
    assert_edge!(DF, D);
    assert_edge!(FD, F);
    assert_edge!(DL, D);
    assert_edge!(LD, L);
    assert_edge!(DB, D);
    assert_edge!(BD, B);
    assert_edge!(DR, D);
    assert_edge!(RD, R);
    assert_edge!(FR, F);
    assert_edge!(RF, R);
    assert_edge!(FL, F);
    assert_edge!(LF, L);
    assert_edge!(BL, B);
    assert_edge!(LB, L);
    assert_edge!(BR, B);
    assert_edge!(RB, R);

    macro_rules! assert_corner {
      ($corner_pos: ident, $face: ident) => {
        assert_eq!(Face::$face, c.corners[CornerPos::$corner_pos as usize]);
      };
    }

    assert_corner!(URF, U);
    assert_corner!(RFU, R);
    assert_corner!(FUR, F);
    assert_corner!(UFL, U);
    assert_corner!(FLU, F);
    assert_corner!(LUF, L);
    assert_corner!(ULB, U);
    assert_corner!(LBU, L);
    assert_corner!(BUL, B);
    assert_corner!(UBR, U);
    assert_corner!(BRU, B);
    assert_corner!(RUB, R);
    assert_corner!(DFR, D);
    assert_corner!(FRD, F);
    assert_corner!(RDF, R);
    assert_corner!(DLF, D);
    assert_corner!(LFD, L);
    assert_corner!(FDL, F);
    assert_corner!(DBL, D);
    assert_corner!(BLD, B);
    assert_corner!(LDB, L);
    assert_corner!(DRB, D);
    assert_corner!(RBD, R);
    assert_corner!(BDR, B);

    macro_rules! assert_centre {
      ($centre: ident) => {
        assert_eq!(Face::$centre, c.centres[CentrePos::$centre as usize]);
      };
    }

    assert_centre!(U);
    assert_centre!(R);
    assert_centre!(F);
    assert_centre!(D);
    assert_centre!(B);
    assert_centre!(L);
  }
}
