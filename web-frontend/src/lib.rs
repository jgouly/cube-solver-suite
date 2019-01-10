use cube::parse_moves;
use cube::Cube;
use lazy_static::lazy_static;
use miniserde::{json, MiniSerialize};
use roux::first_block::*;
use solver::iddfs::iddfs;

mod interop;

use crate::interop::*;

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

#[no_mangle]
pub fn solve_fb(s: JSString, orientations: u32) {
  let info = &*FB_INFO;

  let mut solutions = Vec::with_capacity(24);

  let mut c = Cube::solved();
  let scramble = parse_moves(&s.as_string()).unwrap();
  c.do_moves(&scramble);

  for &o in cube::sticker_cube::EdgePos::natural_order()
    .iter()
    .filter(|&&o| skip_orientation(o as usize, orientations))
  {
    let mut c = c;
    c.do_moves(&roux::DL_ORIENTATIONS[o as usize]);
    let mut solution = Vec::with_capacity(10);
    for i in 0..10 {
      let solved = iddfs(info.get_state(&c), info, i, &mut solution);
      if solved {
        // A move is 1 or 2 characters, and a space between moves.
        // So allocate 3 * i, for the maximum solution length.
        let solution_len_max =
          3 * (i + roux::DL_ORIENTATIONS[o as usize].len());
        let mut ret = String::with_capacity(solution_len_max);
        for m in roux::DL_ORIENTATIONS[o as usize] {
          ret.push_str(&format!("{} ", m));
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
  }

  solutions.sort_by_key(|a| a.len);
  stack_push_str(&json::to_string(&solutions));
}
