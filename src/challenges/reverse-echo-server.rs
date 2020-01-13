#![allow(unused_imports)]
#![no_main]
#![no_std]

// The following imports are important for compilation
use panic_halt;
use f3::hal::prelude::*;    // provides the memory.x layout
use cortex_m_rt::entry;
use cortex_m::{iprintln, peripheral::ITM};

use f3::hal::{serial::Serial, stm32f30x::{self, USART1, usart1}, time::MonoTimer};
use core::str;
use core::fmt::{self, Write};
use heapless::{consts, Vec};

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

impl SerialPort {
    fn receive_byte(&mut self) -> u8 {
        while self.usart1.isr.read().rxne().bit_is_clear() {}
        self.usart1.rdr.read().rdr().bits() as u8
    }
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

fn init() -> &'static mut usart1::RegisterBlock {
    let dp = stm32f30x::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb); // SPLIT the GPIO block into independent pins and registers
    
    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    
    Serial::usart1(dp.USART1, (tx, rx), 115200.bps(), clocks, &mut rcc.apb2);
    
    unsafe { &mut *(USART1::ptr() as *mut _) }
}


#[entry]
fn main() -> ! {
    let usart1 = init();
    
    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U32> = Vec::new();
    let mut serial = SerialPort { usart1 };
    
    loop {
        // Each user requeset ends with ENTER
        // NOTE `buffer.push` returns a `Result`. Handle the error
        // by responding with an error message.
        buffer.clear();
        
        // Receive the byte (this is blocking)
        let c = serial.receive_byte();
        
        if c == b'\n' && !buffer.is_empty() {
            buffer.reverse();
            // Can't use `uprintln!` here. Macro's only work with string literals. 
            serial.write_str(&str::from_utf8(&buffer).unwrap()).unwrap();
            continue;
        }
        
        match buffer.push(c) {
            Err(_) => { uprintln!(serial, "error: buffer is full, press <ENTER> to flush"); },
            Ok(_) => continue,
        }
    }
}
