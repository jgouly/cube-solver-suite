use std::ffi::CString;
use std::os::raw::c_char;

/// Imports from JS.
#[no_mangle]
extern "C" {
  fn stack_push(val: usize) -> ();
}

/// Push a Rust string onto the JS stack.
pub fn stack_push_str(s: &str) {
  let len = s.len();
  let s = CString::new(s).unwrap();
  unsafe {
    stack_push(len);
    stack_push(s.into_raw() as usize);
  }
}

/// Deallocate a CString that was originally allocated on the Rust side.
#[no_mangle]
extern "C" fn dealloc_str(ptr: *mut c_char) {
  unsafe {
    let _ = CString::from_raw(ptr);
  }
}

/// This struct is used to pass Strings from JS into Rust.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct JSString(*mut String);

impl JSString {
  /// Allocate a new JSString with a particular size.
  fn with_capacity(size: usize) -> Self {
    let d = vec![0; size];
    let s = Box::new(String::from_utf8(d).unwrap());
    JSString(Box::into_raw(s))
  }

  /// Return a reference to the `String`.
  pub fn as_string(&self) -> &String {
    unsafe { &*self.0 }
  }

  /// Consume the JSString and return a `Box<String>`.
  pub fn into_boxed_string(self) -> Box<String> {
    unsafe { Box::from_raw(self.0) }
  }

  /// Returns a mutable pointer to the `String`s backing memory.
  unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
    (&mut *self.0).as_mut_vec().as_mut_ptr()
  }
}

#[no_mangle]
extern "C" fn alloc_rust_string(size: usize) -> JSString {
  JSString::with_capacity(size)
}

#[no_mangle]
extern "C" fn get_rust_string_ptr(mut s: JSString) -> *mut u8 {
  unsafe { s.as_mut_ptr() }
}

/// Deallocate a JSString.
#[no_mangle]
extern "C" fn dealloc_rust_string(s: JSString) {
  s.into_boxed_string();
}
