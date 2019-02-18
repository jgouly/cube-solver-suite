use cube::parse_moves;
use cube::Cube;
use lazy_static::lazy_static;
use miniserde::{json, MiniSerialize};
use roux::first_block::*;
use solver::iddfs::iddfs;
use solver::index::Index;
use wasm_bindgen::prelude::*;

#[derive(MiniSerialize)]
struct FBSolution {
  moves: String,
  len: usize,
  dl: String,
}

lazy_static! {
  static ref FB_INFO: FBInfo = { FBInfo::new() };
}

fn skip_orientation(o: usize, orientations: u32) -> bool {
  ((1 << o) & orientations) != 0
}

#[wasm_bindgen]
pub fn solve_fb(s: String, orientations: u32) -> String {
  let info = &*FB_INFO;

  let mut solutions = Vec::with_capacity(24);

  let mut c = Cube::solved();
  let scramble = parse_moves(&s).unwrap();
  c.do_moves(&scramble);

  for &o in cube::sticker_cube::EdgePos::natural_order()
    .iter()
    .filter(|&&o| skip_orientation(o as usize, orientations))
  {
    let mut c = c;
    c.do_moves(&roux::DL_ORIENTATIONS[o as usize]);
    let (fbe, fbc) = info.get_indexes(&c);
    for x in 0..4 {
      let mut solution = Vec::with_capacity(10);
      for i in 0..10 {
        let solved = iddfs(
          (fbe.from_cube(&c), fbc.from_cube(&c)),
          info,
          i,
          &mut solution,
        );
        if solved {
          // A move is 1 or 2 characters, and a space between moves.
          // So allocate 3 * i, for the maximum solution length.
          let solution_len_max =
            3 * (i + roux::DL_ORIENTATIONS[o as usize].len());
          let mut ret = String::with_capacity(solution_len_max);
          for m in roux::DL_ORIENTATIONS[o as usize] {
            ret.push_str(&format!("{} ", m));
          }
          if x > 0 {
            ret.push_str(&format!(
              "{} ",
              cube::Move::Rotation(cube::Rotation::X, x)
            ));
          }
          for m in &solution {
            ret.push_str(&format!("{} ", m));
          }
          solutions.push(FBSolution {
            moves: ret,
            len: solution.len(),
            dl: String::from(format!("{:?}", o)),
          });
          break;
        }
      }
      c.do_move(cube::Move::Rotation(cube::Rotation::X, 1));
    }
  }

  solutions.sort_by_key(|a| a.len);
  json::to_string(&solutions)
}
