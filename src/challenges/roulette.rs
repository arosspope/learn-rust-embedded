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
    let cp = cortex_m::Peripherals::take().unwrap();    // Obtain exclusive access to all the cortex-m peripherals ONCE
    let dp = stm32f30x::Peripherals::take().unwrap();   // Obtain exclusive access to all the stm32f30x specific peripherals

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();                   // RCC - Reset and Clock Control

    let clocks = rcc.cfgr.freeze(&mut flash.acr); // Accessing the RCC_CFGR register and freezing the clock configuration
    let delay = Delay::new(cp.SYST, clocks);
    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (delay, leds)
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = init();

    const PERIOD_MS: u8             = 25;

    let total_headings              = leds.len();

    let mut heading                 = 0;
    let mut active_heading_count    = 2;

    leds[heading].on();
    leds[(heading + 1) % total_headings].on();

    loop {
        delay.delay_ms(PERIOD_MS);

        match active_heading_count {
            0 => {
                leds[heading].off();
                heading = (heading + 1) % total_headings;
                active_heading_count = 2;
            },
            1 => {
                leds[(heading + 1) % total_headings].on();
                active_heading_count -= 1;
            },
            _ => active_heading_count -= 1,
        }
    }
}
