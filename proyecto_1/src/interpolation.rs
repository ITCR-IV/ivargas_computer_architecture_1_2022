use std::arch::global_asm;
use std::ffi::CString;
use std::os::raw::{c_char, c_uchar};

global_asm!(include_str!("bil_interpol.S"), options(raw));

extern "C" {
    fn _bil_interpol(f_name: *const c_char, buf: *mut c_uchar, h: u32, w: u32) -> u64;
}

pub fn bil_interpol(f_name: &str, buf: &mut [u8], h: u32, w: u32) -> u64 {
    let c_str = CString::new(f_name).expect("CString::new failed");
    unsafe { _bil_interpol(c_str.as_ptr(), buf.as_mut_ptr(), h, w) }
}
