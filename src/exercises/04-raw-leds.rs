#![allow(unused_imports)]
#![no_main]
#![no_std]

use panic_halt;
use cortex_m::asm::bkpt;
use cortex_m_rt::entry;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{self, GPIOE, RCC, gpioc, rcc};

fn init() -> (&'static gpioc::RegisterBlock, &'static rcc::RegisterBlock) {
    // restrict access to the other peripherals
    (stm32f30x::Peripherals::take().unwrap());

    // Return the raw gpioe and rcc registers
    unsafe { (&*GPIOE::ptr(), &*RCC::ptr()) }
}

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = init();

    // enable the GPIOE peripheral
    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());

    // configure the pins as outputs
    gpioe.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output();
        w.moder10().output();
        w.moder11().output();
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    bkpt();

    loop {}
}
