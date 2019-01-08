mod interop;

use crate::interop::*;

#[no_mangle]
pub fn solve_fb(s: JSString) {
  stack_push_str(&s.as_string());
}
