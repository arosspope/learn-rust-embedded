#![deny(unsafe_code)]
#![allow(unused_imports)]
#![no_main]
#![no_std]

// The following imports are important for compilation
use panic_halt;
use f3::hal::prelude::*;    // provides the memory.x layout
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
