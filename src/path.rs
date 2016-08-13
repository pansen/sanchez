use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use std::path::{Path};

/// taken from https://github.com/rust-lang/rust/issues/11857#issuecomment-55329505
///
/// changes via:
///
/// - https://github.com/rust-lang/rust/pull/20507
/// - https://doc.rust-lang.org/std/ffi/struct.CStr.html
/// - http://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/std/ffi/fn.c_str_to_bytes_with_nul.html
#[cfg(unix)]
pub fn realpath(p: &Path) -> &Path {
    extern {
        fn realpath(path: *const c_char, resolved: *mut c_char) -> *const c_char;
    }
    let path_sting = p.to_str().unwrap();
    debug!("path_string: {}", path_sting);
    let new_p = unsafe {
        realpath(CString::new(path_sting).unwrap().as_ptr() as *const c_char, 0 as *mut c_char)
    };
    unsafe {
        Path::new(CStr::from_ptr(new_p).to_str().unwrap())
    }
}