use cube::parse_moves;
use cube::Cube;
use lazy_static::lazy_static;
use miniserde::json;
use roux::first_block::*;
use solver::iddfs::iddfs;

mod interop;

use crate::interop::*;

lazy_static! {
  static ref FB_INFO: FBInfo = { FBInfo::new() };
}

#[no_mangle]
pub fn solve_fb(s: JSString) {
  let info = &*FB_INFO;

  let mut solutions = Vec::new();

  let mut c = Cube::solved();
  let scramble = parse_moves(&s.as_string()).unwrap();
  c.do_moves(&scramble);

  for o in 0..24 {
    let mut c = c;
    c.do_moves(&roux::DL_ORIENTATIONS[o]);
    let mut solution = Vec::with_capacity(10);
    for i in 0..10 {
      let solved = iddfs(info.get_state(&c), info, i, &mut solution);
      if solved {
        let mut ret = String::new();
        for m in roux::DL_ORIENTATIONS[o] {
          ret.push_str(&format!("{} ", m));
        }
        for m in solution {
          ret.push_str(&format!("{} ", m));
        }
        solutions.push(ret);
        break;
      }
    }
  }

  stack_push_str(&json::to_string(&solutions));
}
