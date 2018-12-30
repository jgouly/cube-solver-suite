use cube::{CentrePos, Cube};

/// Create an index for a list of edges.
pub fn generic_edge_index(
  cube: &Cube,
  edge_faces: &[(CentrePos, CentrePos)],
) -> u32 {
  let mut edges = Vec::with_capacity(edge_faces.len());

  // Collect the edges.
  for &e in edge_faces {
    edges.push(
      cube.find_edge(cube.centres[e.0 as usize], cube.centres[e.1 as usize])
        as u32,
    );
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

/// Decode `index` and fill in `cube`'s edges.
pub fn generic_edge_index_decode(
  cube: &mut Cube,
  index: u32,
  edge_faces: &[(CentrePos, CentrePos)],
) {
  let num = edge_faces.len();
  let mut edge_div = 24 - ((num as u32 - 1) * 2);
  let mut edges = Vec::with_capacity(num);

  let mut index = index;
  // Extract the digits from the index.
  for _ in 0..num {
    edges.push(index % edge_div);
    index = index / edge_div;
    edge_div += 2;
  }

  // Modify the edge values, such that 0 < edge[n] < 24.
  for i in 0..edges.len() {
    for j in (i + 1)..edges.len() {
      // The shift ignores orientation.
      if (edges[i] >> 1) >= (edges[j] >> 1) {
        edges[i] += 2;
      }
    }
  }

  // Fill in the cube's edges.
  for i in 0..edges.len() {
    cube.edges[edges[i] as usize] = cube.centres[edge_faces[i].0 as usize];
    cube.edges[edges[i] as usize ^ 1] = cube.centres[edge_faces[i].1 as usize];
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::sticker_cube::EdgePos;
  use cube::{Face, Move};

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

  #[test]
  fn generic_edge_decode() {
    {
      let mut c = Cube::invalid();
      c.solve_centres();
      generic_edge_index_decode(&mut c, 0, &[(CentrePos::U, CentrePos::F)]);
      assert_eq!(Face::U, c.edges[EdgePos::UF as usize]);
      assert_eq!(Face::F, c.edges[EdgePos::FU as usize]);
    }

    {
      let mut c = Cube::invalid();
      c.solve_centres();
      generic_edge_index_decode(&mut c, 0, &[(CentrePos::U, CentrePos::L)]);
      assert_eq!(Face::U, c.edges[EdgePos::UF as usize]);
      assert_eq!(Face::L, c.edges[EdgePos::FU as usize]);
    }

    {
      let mut c = Cube::invalid();
      c.solve_centres();
      generic_edge_index_decode(&mut c, 2, &[(CentrePos::U, CentrePos::L)]);
      assert_eq!(Face::U, c.edges[EdgePos::UL as usize]);
      assert_eq!(Face::L, c.edges[EdgePos::LU as usize]);
    }

    {
      let mut c = Cube::invalid();
      c.solve_centres();
      generic_edge_index_decode(&mut c, 3, &[(CentrePos::L, CentrePos::U)]);
      assert_eq!(Face::U, c.edges[EdgePos::UL as usize]);
      assert_eq!(Face::L, c.edges[EdgePos::LU as usize]);
    }

    {
      let mut c = Cube::invalid();
      c.solve_centres();
      generic_edge_index_decode(
        &mut c,
        0,
        &[(CentrePos::U, CentrePos::L), (CentrePos::U, CentrePos::F)],
      );
      assert_eq!(Face::U, c.edges[EdgePos::UF as usize]);
      assert_eq!(Face::F, c.edges[EdgePos::FU as usize]);
      assert_eq!(Face::U, c.edges[EdgePos::UL as usize]);
      assert_eq!(Face::L, c.edges[EdgePos::LU as usize]);
    }

    {
      let mut c = Cube::invalid();
      c.solve_centres();
      generic_edge_index_decode(
        &mut c,
        1,
        &[(CentrePos::U, CentrePos::L), (CentrePos::U, CentrePos::F)],
      );
      assert_eq!(Face::U, c.edges[EdgePos::UF as usize]);
      assert_eq!(Face::F, c.edges[EdgePos::FU as usize]);
      assert_eq!(Face::L, c.edges[EdgePos::UL as usize]);
      assert_eq!(Face::U, c.edges[EdgePos::LU as usize]);
    }

    {
      let mut c = Cube::invalid();
      c.solve_centres();
      generic_edge_index_decode(
        &mut c,
        22,
        &[(CentrePos::U, CentrePos::L), (CentrePos::U, CentrePos::F)],
      );
      assert_eq!(Face::F, c.edges[EdgePos::UF as usize]);
      assert_eq!(Face::U, c.edges[EdgePos::FU as usize]);
      assert_eq!(Face::U, c.edges[EdgePos::UL as usize]);
      assert_eq!(Face::L, c.edges[EdgePos::LU as usize]);
    }

    {
      let mut c = Cube::invalid();
      c.solve_centres();
      generic_edge_index_decode(
        &mut c,
        45,
        &[(CentrePos::U, CentrePos::F), (CentrePos::U, CentrePos::L)],
      );
      assert_eq!(Face::F, c.edges[EdgePos::UF as usize]);
      assert_eq!(Face::U, c.edges[EdgePos::FU as usize]);
      assert_eq!(Face::U, c.edges[EdgePos::UL as usize]);
      assert_eq!(Face::L, c.edges[EdgePos::LU as usize]);
    }
  }
}
