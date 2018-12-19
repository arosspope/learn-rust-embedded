#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux::a0::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux::a0::init();

    let half_period = 500_u16;

    loop {
        leds[0].on();
        delay.delay_ms(half_period);

        leds[0].off();
        delay.delay_ms(half_period);
    }
}
