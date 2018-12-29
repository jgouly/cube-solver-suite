use crate::{Face, Move};

/// Represents a 3x3x3 cube using a representation that is similar to storing
/// sticker colours. This representation includes centre pieces so can
/// represent slice turns and rotations.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Cube {
  pub edges: [Face; 24],
  pub corners: [Face; 24],
  pub centres: [Face; 6],
}

impl Cube {
  /// Creates a solved cube.
  pub fn solved() -> Cube {
    use crate::Face::*;

    let edges = [
      U, F, U, L, U, B, U, R, D, F, D, L, D, B, D, R, F, R, F, L, B, L, B, R,
    ];

    let corners = [
      U, R, F, U, F, L, U, L, B, U, B, R, D, F, R, D, L, F, D, B, L, D, R, B,
    ];

    let centres = [U, R, F, D, B, L];
    Cube {
      edges,
      corners,
      centres,
    }
  }

  /// Do the move `m` on the cube.
  pub fn do_move(&mut self, m: Move) {
    let amt = m.amount();
    for _ in 0..amt {
      match &m {
        Move::U(_) => self.do_u(),
        Move::D(_) => self.do_d(),
        Move::F(_) => self.do_f(),
        Move::B(_) => self.do_b(),
        Move::R(_) => self.do_r(),
        Move::L(_) => self.do_l(),
        Move::M(_) => self.do_m(),
      }
    }
  }

  /// Do all the moves in the slice `moves` on the cube.
  pub fn do_moves(&mut self, moves: &[Move]) {
    for &m in moves {
      self.do_move(m);
    }
  }

  fn do_u(&mut self) {
    use self::EdgePos::*;
    edge4(UF, UL, UB, UR, &mut self.edges);
    edge4(FU, LU, BU, RU, &mut self.edges);

    use self::CornerPos::*;
    corner4(URF, UFL, ULB, UBR, &mut self.corners);
    corner4(RFU, FLU, LBU, BRU, &mut self.corners);
    corner4(FUR, LUF, BUL, RUB, &mut self.corners);
  }

  fn do_d(&mut self) {
    use self::EdgePos::*;
    edge4(DF, DR, DB, DL, &mut self.edges);
    edge4(FD, RD, BD, LD, &mut self.edges);

    use self::CornerPos::*;
    corner4(DFR, DRB, DBL, DLF, &mut self.corners);
    corner4(FRD, RBD, BLD, LFD, &mut self.corners);
    corner4(RDF, BDR, LDB, FDL, &mut self.corners);
  }

  fn do_r(&mut self) {
    use self::EdgePos::*;
    edge4(UR, BR, DR, FR, &mut self.edges);
    edge4(RU, RB, RD, RF, &mut self.edges);

    use self::CornerPos::*;
    corner4(URF, BRU, DRB, FRD, &mut self.corners);
    corner4(RFU, RUB, RBD, RDF, &mut self.corners);
    corner4(FUR, UBR, BDR, DFR, &mut self.corners);
  }

  fn do_l(&mut self) {
    use self::EdgePos::*;
    edge4(UL, FL, DL, BL, &mut self.edges);
    edge4(LU, LF, LD, LB, &mut self.edges);

    use self::CornerPos::*;
    corner4(UFL, FDL, DBL, BUL, &mut self.corners);
    corner4(FLU, DLF, BLD, ULB, &mut self.corners);
    corner4(LUF, LFD, LDB, LBU, &mut self.corners);
  }

  fn do_f(&mut self) {
    use self::EdgePos::*;
    edge4(UF, RF, DF, LF, &mut self.edges);
    edge4(FU, FR, FD, FL, &mut self.edges);

    use self::CornerPos::*;
    corner4(URF, RDF, DLF, LUF, &mut self.corners);
    corner4(RFU, DFR, LFD, UFL, &mut self.corners);
    corner4(FUR, FRD, FDL, FLU, &mut self.corners);
  }

  fn do_b(&mut self) {
    use self::EdgePos::*;
    edge4(UB, LB, DB, RB, &mut self.edges);
    edge4(BU, BL, BD, BR, &mut self.edges);

    use self::CornerPos::*;
    corner4(UBR, LBU, DBL, RBD, &mut self.corners);
    corner4(BRU, BUL, BLD, BDR, &mut self.corners);
    corner4(RUB, ULB, LDB, DRB, &mut self.corners);
  }

  fn do_m(&mut self) {
    use self::EdgePos::*;
    edge4(UF, FD, DB, BU, &mut self.edges);
    edge4(FU, DF, BD, UB, &mut self.edges);

    let centres = self.centres;
    self.centres[CentrePos::F as usize] = centres[CentrePos::U as usize];
    self.centres[CentrePos::D as usize] = centres[CentrePos::F as usize];
    self.centres[CentrePos::B as usize] = centres[CentrePos::D as usize];
    self.centres[CentrePos::U as usize] = centres[CentrePos::B as usize];
  }

  /// Find the `EdgePos` for a particular edge piece.
  pub fn find_edge(&self, f1: Face, f2: Face) -> EdgePos {
    let edge_pos = EdgePos::natural_order();
    for (e, ep) in self.edges.chunks(2).zip(edge_pos.chunks(2)) {
      if (e[0], e[1]) == (f1, f2) {
        return ep[0];
      }
      if (e[1], e[0]) == (f1, f2) {
        return ep[1];
      }
    }
    unreachable!("{:?}{:?} not found", f1, f2)
  }

  /// Find the `CornerPos` for a particular corner piece.
  pub fn find_corner(&self, f1: Face, f2: Face, f3: Face) -> CornerPos {
    let corner_pos = CornerPos::natural_order();
    for (c, cp) in self.corners.chunks(3).zip(corner_pos.chunks(3)) {
      if (c[0], c[1], c[2]) == (f1, f2, f3) {
        return cp[0];
      }

      if (c[1], c[2], c[0]) == (f1, f2, f3) {
        return cp[1];
      }

      if (c[2], c[0], c[1]) == (f1, f2, f3) {
        return cp[2];
      }
    }
    unreachable!("{:?}{:?}{:?} not found", f1, f2, f3)
  }
}

fn edge4(
  e1: EdgePos,
  e2: EdgePos,
  e3: EdgePos,
  e4: EdgePos,
  edges: &mut [Face; 24],
) {
  let oe1 = edges[e1 as usize];
  let oe2 = edges[e2 as usize];
  let oe3 = edges[e3 as usize];
  let oe4 = edges[e4 as usize];
  edges[e1 as usize] = oe4;
  edges[e2 as usize] = oe1;
  edges[e3 as usize] = oe2;
  edges[e4 as usize] = oe3;
}

fn corner4(
  e1: CornerPos,
  e2: CornerPos,
  e3: CornerPos,
  e4: CornerPos,
  corners: &mut [Face; 24],
) {
  let oe1 = corners[e1 as usize];
  let oe2 = corners[e2 as usize];
  let oe3 = corners[e3 as usize];
  let oe4 = corners[e4 as usize];
  corners[e1 as usize] = oe4;
  corners[e2 as usize] = oe1;
  corners[e3 as usize] = oe2;
  corners[e4 as usize] = oe3;
}

/// Represents a particular edge position on a cube.
/// Note: This represents a position, not a particular piece.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(PartialEq))]
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

impl EdgePos {
  pub fn natural_order() -> &'static [EdgePos; 24] {
    use self::EdgePos::*;
    static EDGES: [EdgePos; 24] = [
      UF, FU, UL, LU, UB, BU, UR, RU, DF, FD, DL, LD, DB, BD, DR, RD, FR, RF,
      FL, LF, BL, LB, BR, RB,
    ];
    return &EDGES;
  }
}

/// Represents a particular corner position on a cube.
/// Note: This represents a position, not a particular piece.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(PartialEq))]
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

impl CornerPos {
  pub fn natural_order() -> &'static [CornerPos; 24] {
    use self::CornerPos::*;
    static CORNERS: [CornerPos; 24] = [
      URF, RFU, FUR, UFL, FLU, LUF, ULB, LBU, BUL, UBR, BRU, RUB, DFR, FRD,
      RDF, DLF, LFD, FDL, DBL, BLD, LDB, DRB, RBD, BDR,
    ];
    return &CORNERS;
  }
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
  use super::Face::*;
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

  #[test]
  fn u_move() {
    let mut c = Cube::solved();
    c.do_u();

    assert_eq!(
      Cube {
        edges: [
          U, R, U, F, U, L, U, B, D, F, D, L, D, B, D, R, F, R, F, L, B, L, B,
          R
        ],
        corners: [
          U, B, R, U, R, F, U, F, L, U, L, B, D, F, R, D, L, F, D, B, L, D, R,
          B
        ],
        centres: [U, R, F, D, B, L]
      },
      c
    );
  }

  #[test]
  fn d_move() {
    let mut c = Cube::solved();
    c.do_d();

    assert_eq!(
      Cube {
        edges: [
          U, F, U, L, U, B, U, R, D, L, D, B, D, R, D, F, F, R, F, L, B, L, B,
          R
        ],
        corners: [
          U, R, F, U, F, L, U, L, B, U, B, R, D, L, F, D, B, L, D, R, B, D, F,
          R
        ],
        centres: [U, R, F, D, B, L]
      },
      c
    );
  }

  #[test]
  fn r_move() {
    let mut c = Cube::solved();
    c.do_r();

    assert_eq!(
      Cube {
        edges: [
          U, F, U, L, U, B, F, R, D, F, D, L, D, B, B, R, D, R, F, L, B, L, U,
          R
        ],
        corners: [
          F, R, D, U, F, L, U, L, B, F, U, R, B, D, R, D, L, F, D, B, L, B, R,
          U
        ],
        centres: [U, R, F, D, B, L]
      },
      c
    );
  }

  #[test]
  fn l_move() {
    let mut c = Cube::solved();
    c.do_l();

    assert_eq!(
      Cube {
        edges: [
          U, F, B, L, U, B, U, R, D, F, F, L, D, B, D, R, F, R, U, L, D, L, B,
          R
        ],
        corners: [
          U, R, F, B, U, L, B, L, D, U, B, R, D, F, R, F, L, U, F, D, L, D, R,
          B
        ],
        centres: [U, R, F, D, B, L]
      },
      c
    );
  }

  #[test]
  fn f_move() {
    let mut c = Cube::solved();
    c.do_f();

    assert_eq!(
      Cube {
        edges: [
          L, F, U, L, U, B, U, R, R, F, D, L, D, B, D, R, F, U, F, D, B, L, B,
          R
        ],
        corners: [
          L, U, F, L, F, D, U, L, B, U, B, R, R, F, U, R, D, F, D, B, L, D, R,
          B
        ],
        centres: [U, R, F, D, B, L]
      },
      c
    );
  }

  #[test]
  fn b_move() {
    let mut c = Cube::solved();
    c.do_b();

    assert_eq!(
      Cube {
        edges: [
          U, F, U, L, R, B, U, R, D, F, D, L, L, B, D, R, F, R, F, L, B, U, B,
          D
        ],
        corners: [
          U, R, F, U, F, L, R, U, B, R, B, D, D, F, R, D, L, F, L, B, U, L, D,
          B
        ],
        centres: [U, R, F, D, B, L]
      },
      c
    );
  }

  #[test]
  fn m_move() {
    let mut c = Cube::solved();
    c.do_m();

    assert_eq!(
      Cube {
        edges: [
          B, U, U, L, B, D, U, R, F, U, D, L, F, D, D, R, F, R, F, L, B, L, B,
          R
        ],
        corners: [
          U, R, F, U, F, L, U, L, B, U, B, R, D, F, R, D, L, F, D, B, L, D, R,
          B
        ],
        centres: [B, R, U, F, D, L]
      },
      c
    );
  }

  #[test]
  fn moves() {
    let mut c2 = Cube::solved();
    c2.do_u();
    c2.do_u();
    c2.do_u();

    let mut c = Cube::solved();
    c.do_moves(&[Move::U(3)]);

    assert_eq!(c2, c);

    c.do_moves(&[Move::U(1)]);
    assert_eq!(Cube::solved(), c);
  }

  #[test]
  fn t_perm() {
    let mut c = Cube::solved();
    {
      use crate::Move::*;
      // Optimal T perm: U F2 U' F2 D R2 B2 U B2 D' R2
      c.do_moves(&[
        U(1),
        F(2),
        U(3),
        F(2),
        D(1),
        R(2),
        B(2),
        U(1),
        B(2),
        D(3),
        R(2),
      ]);
    }
    assert_eq!(
      Cube {
        edges: [
          U, F, U, R, U, B, U, L, D, F, D, L, D, B, D, R, F, R, F, L, B, L, B,
          R
        ],
        corners: [
          U, B, R, U, F, L, U, L, B, U, R, F, D, F, R, D, L, F, D, B, L, D, R,
          B
        ],
        centres: [U, R, F, D, B, L]
      },
      c
    );
  }

  #[test]
  fn natural_order() {
    for (i, &e) in EdgePos::natural_order().iter().enumerate() {
      assert_eq!(i, e as usize);
    }

    for (i, &c) in CornerPos::natural_order().iter().enumerate() {
      assert_eq!(i, c as usize);
    }
  }

  #[test]
  fn find_edge() {
    let c = Cube::solved();

    assert_eq!(EdgePos::UF, c.find_edge(Face::U, Face::F));
    assert_eq!(EdgePos::FU, c.find_edge(Face::F, Face::U));

    assert_eq!(EdgePos::FR, c.find_edge(Face::F, Face::R));
    assert_eq!(EdgePos::RF, c.find_edge(Face::R, Face::F));

    let mut c = Cube::solved();
    c.do_u();

    assert_eq!(EdgePos::UF, c.find_edge(Face::U, Face::R));
    assert_eq!(EdgePos::FU, c.find_edge(Face::R, Face::U));

    assert_eq!(EdgePos::UL, c.find_edge(Face::U, Face::F));
    assert_eq!(EdgePos::LU, c.find_edge(Face::F, Face::U));

    assert_eq!(EdgePos::FR, c.find_edge(Face::F, Face::R));
    assert_eq!(EdgePos::RF, c.find_edge(Face::R, Face::F));
  }

  #[test]
  fn find_corner() {
    let c = Cube::solved();

    assert_eq!(CornerPos::URF, c.find_corner(Face::U, Face::R, Face::F));
    assert_eq!(CornerPos::RFU, c.find_corner(Face::R, Face::F, Face::U));
    assert_eq!(CornerPos::FUR, c.find_corner(Face::F, Face::U, Face::R));

    assert_eq!(CornerPos::DRB, c.find_corner(Face::D, Face::R, Face::B));
    assert_eq!(CornerPos::RBD, c.find_corner(Face::R, Face::B, Face::D));
    assert_eq!(CornerPos::BDR, c.find_corner(Face::B, Face::D, Face::R));

    let mut c = Cube::solved();
    c.do_r();

    assert_eq!(CornerPos::URF, c.find_corner(Face::F, Face::R, Face::D));
    assert_eq!(CornerPos::RFU, c.find_corner(Face::R, Face::D, Face::F));
    assert_eq!(CornerPos::FUR, c.find_corner(Face::D, Face::F, Face::R));
  }
}
