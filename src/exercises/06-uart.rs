#![allow(unused_imports)]
#![no_main]
#![no_std]

// The following imports are important for compilation
use panic_halt;
use f3::hal::prelude::*;    // provides the memory.x layout
use cortex_m_rt::entry;
use cortex_m::{iprintln, peripheral::ITM};
use f3::hal::{serial::Serial, stm32f30x::{self, USART1, usart1}, time::MonoTimer};

fn init() -> (&'static mut usart1::RegisterBlock, MonoTimer, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb); // SPLIT the GPIO block into independent pins and registers
    
    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    
    Serial::usart1(dp.USART1, (tx, rx), 115200.bps(), clocks, &mut rcc.apb2);
    
    unsafe {
        (
            &mut *(USART1::ptr() as *mut _),
            MonoTimer::new(cp.DWT, clocks),
            cp.ITM,
        )
    }
}

// Following macros are for simplifying serial printing
use core::fmt::{self, Write};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(b)));
        }
        Ok(())
    }
}


#[entry]
fn main() -> ! {
    let (usart1, mono_timer, _itm) = init();
    
    // Send a single character
    // by writing to the TDR register
    usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));
    
    // MonoTimer (monotonic timer) exposes an Instant API that's similar to the one
    // in std::time.
    let instant = mono_timer.now();
    
    
    // Send multiple
    for b in b"The quick brown fox jumps over the lazy dog.".iter() {
        usart1.tdr.write(|w| w.tdr().bits(u16::from(*b)));
    }
    //
    // However, the above code would probably print nonsense... Given the
    // baud rate of 115200, to transfer 45 bytes - it will take approximately
    // 3906us -> Our for loop will probably complete by then.
    
    let _elapsed = instant.elapsed();
    // iprintln!(
    //     &mut itm.stim[0],
    //     "`for` loop took {} ticks ({} us)",
    //     elapsed,
    //     elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    // );
    
    //Send the String but wait for the status register bit 'TXE' indicating safe to write
    for b in b"The quick brown fox jumps over the lazy dog.".iter() {
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1.tdr.write(|w| w.tdr().bits(u16::from(*b)));
    }
    
    let mut serial = SerialPort { usart1 };
    uprintln!(serial, "The answer is {}", 40 + 2);


    loop {}
}
