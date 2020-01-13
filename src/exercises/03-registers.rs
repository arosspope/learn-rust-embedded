#![allow(unused_imports)]
#![no_main]
#![no_std]

// The following imports are important for compilation
use panic_halt;
use f3::hal::prelude::*;    // provides the memory.x layout
use cortex_m_rt::entry;

use f3::led::Leds;
use f3::hal::{delay::Delay, stm32f30x};

#[entry]
fn main() -> ! {
    // let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    // (cp.ITM, unsafe { &*GPIOE::ptr() })

    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED (red)
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // Turn on the "East" LED (green)
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // Turn off the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // Turn off the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
    }

    loop {}
}
