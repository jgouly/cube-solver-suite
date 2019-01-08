use std::ffi::CString;
use std::os::raw::c_char;

/// Imports from JS.
#[no_mangle]
extern "C" {
  fn stack_push(val: usize) -> ();
}

pub fn stack_push_str(s: &str) {
  let len = s.len();
  let s = CString::new(s).unwrap();
  unsafe {
    stack_push(len);
    stack_push(s.into_raw() as usize);
  }
}

#[no_mangle]
extern "C" fn dealloc_str(ptr: *mut c_char) {
  unsafe {
    let _ = CString::from_raw(ptr);
  }
}
