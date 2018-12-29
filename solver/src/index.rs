use cube::{CentrePos, Cube};

/// Create an index for a list of edges.
pub fn generic_edge_index(
  cube: &Cube,
  edge_faces: &[(CentrePos, CentrePos)],
) -> u32 {
  let mut edges = Vec::with_capacity(edge_faces.len());

  // Collect the edges.
  for &e in edge_faces {
    edges.push(cube.find_edge(
      cube.centres[e.0 as usize],
      cube.centres[e.1 as usize],
    ) as u32);
  }

  // Modify the edge values such that:
  //   0 < edge[n] < (24 - 2n)
  //
  // e.g.
  //   0 < edge[0] < 24
  //   0 < edge[1] < 22
  for i in 0..edges.len() {
    for j in 0..i {
      if edges[i] > edges[j] {
        edges[i] -= 2;
      }
    }
    debug_assert!(edges[i] < (24 - (2 * i as u32)));
  }

  // Combine the values into the index.
  let mut edge_mult = 24;
  let mut coord = 0;
  for &e in &edges {
    debug_assert!(e < edge_mult);
    edge_mult -= 2;
    coord += e;
    coord *= edge_mult;
  }
  // Undo the last multiplication.
  coord /= edge_mult;

  coord
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::Move;

  #[test]
  fn generic_edge() {
    let c = Cube::solved();
    let uf_index = generic_edge_index(&c, &[(CentrePos::U, CentrePos::F)]);
    assert_eq!(0, uf_index);

    let ul_index = generic_edge_index(&c, &[(CentrePos::U, CentrePos::L)]);
    assert_eq!(2, ul_index);

    let lu_index = generic_edge_index(&c, &[(CentrePos::L, CentrePos::U)]);
    assert_eq!(3, lu_index);

    let uful_index = generic_edge_index(
      &c,
      &[(CentrePos::U, CentrePos::F), (CentrePos::U, CentrePos::L)],
    );
    assert_eq!(0, uful_index);

    {
      let mut c = Cube::solved();
      // Flip UL, L U' F U
      c.do_moves(&[Move::L(1), Move::U(3), Move::F(1), Move::U(1)]);

      let uful_index = generic_edge_index(
        &c,
        &[(CentrePos::U, CentrePos::F), (CentrePos::U, CentrePos::L)],
      );
      assert_eq!(1, uful_index);
    }

    {
      let mut c = Cube::solved();
      // Flip UF, F U' R U
      c.do_moves(&[Move::F(1), Move::U(3), Move::R(1), Move::U(1)]);

      let uful_index = generic_edge_index(
        &c,
        &[(CentrePos::U, CentrePos::F), (CentrePos::U, CentrePos::L)],
      );

      // UF at FU = 1 * 22
      // UL at UL = 0
      assert_eq!(22, uful_index);
    }

    {
      let mut c = Cube::solved();
      // Flip UF, F U' R U
      c.do_moves(&[Move::F(1), Move::U(3), Move::R(1), Move::U(1)]);

      let uluf_index = generic_edge_index(
        &c,
        &[(CentrePos::U, CentrePos::L), (CentrePos::U, CentrePos::F)],
      );

      // UL at UL = 2 * 22
      // UF at FU = 1
      assert_eq!(45, uluf_index);
    }
  }
}
