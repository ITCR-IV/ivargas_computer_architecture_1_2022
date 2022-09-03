use std::arch::global_asm;
global_asm!(include_str!("bil_interpol.S"), options(raw));

extern "C" {
    fn _bil_interpol(a: u64, b: u64) -> u64;
}

pub fn bil_interpol(a: u64, b: u64) -> u64 {
    unsafe { _bil_interpol(a, b) }
}
