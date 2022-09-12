use std::arch::global_asm;
use std::ffi::CString;
use std::os::raw::{c_char, c_uchar};

global_asm!(include_str!("bil_interpol.S"), options(raw));

extern "C" {
    fn _bil_interpol(f_name: *const c_char, buf: *const c_uchar, h: u32, w: u32) -> i32;
}

/// Rust interface function that calls the external ASM function bil_interpol to carry out bilineal
/// interpolation over `buf` and saves it at the path given by `f_name`
pub fn bil_interpol(f_name: &str, buf: &[u8], h: u32, w: u32) -> i32 {
    let c_str = CString::new(f_name).expect("CString::new failed");
    unsafe { _bil_interpol(c_str.as_ptr(), buf.as_ptr(), h, w) }
}
