use cube::parse_moves;
use cube::Cube;
use lazy_static::lazy_static;
use roux::first_block::*;

mod interop;

use crate::interop::*;

lazy_static! {
  static ref FB_INFO: FBInfo = { FBInfo::new() };
}

#[no_mangle]
pub fn solve_fb(s: JSString) {
  let info = &*FB_INFO;

  let mut c = Cube::solved();
  let scramble = parse_moves(&s.as_string()).unwrap();
  c.do_moves(&scramble);

  let mut solution = Vec::new();
  let solved = solver::iddfs::iddfs(info.get_state(&c), info, 1, &mut solution);
  assert!(solved);

  let mut ret = String::new();
  for m in solution {
    ret.push_str(&format!("{:?}", m));
  }
  stack_push_str(&ret);
}
