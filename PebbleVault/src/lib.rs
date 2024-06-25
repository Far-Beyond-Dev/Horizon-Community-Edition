#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // silence u128 being not FFI safe warnings.
                           // N.B. If any undefined behaviour occurs, it may be worthwhile to look
                           // into this FIRST.

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{c_char, CStr, CString};

pub fn greet(name: &str) -> String {
    let name = CString::new(name).unwrap();

    unsafe {
        let cstr = CStr::from_ptr(Greet(name.as_ptr() as *mut c_char));
        let s = String::from_utf8_lossy(cstr.to_bytes()).to_string();
        GoFree(cstr.as_ptr() as *mut c_char);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = greet("Rust");
        assert_eq!(result, "Hello from Go, Rust!");
    }
}
