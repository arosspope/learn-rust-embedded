#![allow(unused_imports)]
#![no_main]
#![no_std]

use panic_halt;
use cortex_m::asm::bkpt;
use cortex_m_rt::entry;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{self, RCC, rcc, TIM6, tim6};
use f3::led::Leds;

fn init() -> (Leds, &'static rcc::RegisterBlock, &'static tim6::RegisterBlock) {
    // restrict access to the other peripherals
    let p = stm32f30x::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let leds = Leds::new(p.GPIOE.split(&mut rcc.ahb));
    
    // Return the raw gpioe and rcc registers
    (leds, unsafe {&*RCC::ptr()}, unsafe {&*TIM6::ptr()})
}

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Set the timer to go off in `ms` ticks
    // 1 tick = 1 ms
    tim6.arr.write(|w| w.arr().bits(ms));
    
    // CEN: Enable counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());
    
    // Wait until the alarm goes off
    while !tim6.sr.read().uif().bit_is_set() {}
    
    // Clear the alarm
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}


#[entry]
fn main() -> ! {
    let (mut leds, rcc, tim6) = init();
    
    // Enable the TIM6 peripheral by enabling the APB1 clock that drives it
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
    
    // OPM Select one pulse mode
    // CEN Keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    
    //Configure the prescaler to have the counter operate at 1 KHz
    // APB1_CLOCK = 8 MHz
    // PSC = 7999
    // 8 MHz / (7999 + 1) = 1 KHz
    // The counter (CNT) will increase on every millisecond
    tim6.psc.write(|w| w.psc().bits(7999));
    
    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;
            
            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}
