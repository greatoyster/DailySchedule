use super::context::Context;
use riscv::register::stvec;

global_asm!(include_str!("./interrupt.asm"));
pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
        }
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}
