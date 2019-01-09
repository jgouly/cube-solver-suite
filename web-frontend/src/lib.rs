use lazy_static::lazy_static;
use roux::first_block::*;

mod interop;

use crate::interop::*;

lazy_static! {
  static ref FB_INFO: FBInfo = { FBInfo::new() };
}

#[no_mangle]
pub fn solve_fb(s: JSString) {
  let mut s = s.as_string().clone();
  s.push_str(&format!(" fbinfo = {:p}", &*FB_INFO));
  stack_push_str(&s);
}
