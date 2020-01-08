#![deny(unsafe_code)]
#![allow(unused_imports)]
#![no_main]
#![no_std]

// The following imports are important for compilation
use panic_halt;
use f3::hal::prelude::*;
use cortex_m_rt::entry;

use f3::led::Leds;
use f3::hal::{delay::Delay, stm32f30x};

fn init() -> (Delay, Leds) {
    // Comment information taken from: https://blog.japaric.io/brave-new-io/#freezing-the-clock-configuration

    // dp / cp is an *owned* value, not a reference
    let cp = cortex_m::Peripherals::take().unwrap();    // Obtain exclusive access to all the cortex-m peripherals ONCE
    let dp = stm32f30x::Peripherals::take().unwrap();   // Obtain exclusive access to all the stm32f30x specific peripherals

    // This effectively makes each peripheral a singleton because there can only ever be at most one
    // instance of them at any point in time
    // Note that constrain consumes the original RCC which granted full access to every RCC register;
    // this effectively constrains the operations that can be performed on RCC registers.
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();                   // RCC - Reset and Clock Control

    // The final freeze method makes the configuration effective, consumes CFGR and returns a Clocks value.
    // Clocks is a Copy-able struct that contains the frozen clock configuration.
    // Clocks can be used in the initialization of abstractions like Serial; its very existence holds
    // the invariant that the clock configuration will not change.

    let clocks = rcc.cfgr.freeze(&mut flash.acr); // Accessing the RCC_CFGR register and freezing the clock configuration

    // For the Delay abstraction, we will use the SYSTICK timer, using the clock configuration
    // set previously.
    let delay = Delay::new(cp.SYST, clocks);

    // On the stmf3, the GPIOE register is used to control the 8 LEDs. Thefore, we must
    // provide this peripheral to the LED abstraction.
    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (delay, leds)
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = init();

    let half_period = 500_u16;

    loop {
        leds[0].on();
        delay.delay_ms(half_period);

        leds[0].off();
        delay.delay_ms(half_period);
    }
}
