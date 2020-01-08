#![deny(unsafe_code)]
#![allow(unused_imports)]
#![no_main]
#![no_std]

// The following imports are important for compilation
use panic_halt;
use f3::hal::prelude::*;
use cortex_m_rt::entry;

use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};

fn init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();
    p.ITM
}

#[entry]
fn main() -> ! {
    let mut itm = init();

    iprintln!(&mut itm.stim[0], "Hello, world!");
    loop {}
}
