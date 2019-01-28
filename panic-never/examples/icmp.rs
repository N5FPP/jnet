#![feature(asm)]
#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception};
use panic_never::force_eval;

use jnet::icmp;

const LEN: usize = 128;
static mut BUFFER: [u8; LEN] = [0; LEN];

#[exception]
unsafe fn SysTick() {
    if let Ok(p) = icmp::Packet::parse(&mut BUFFER[..]) {
        force_eval!(p.get_type());
        force_eval!(p.get_code());
        force_eval!(p.payload());
        force_eval!(p.len());
    } else {
        asm!("NOP" : : : : "volatile");
    }
}

#[entry]
fn main() -> ! {
    loop {}
}
