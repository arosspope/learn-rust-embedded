#![allow(unused_imports)]
#![no_main]
#![no_std]

// The following imports are important for compilation
use panic_halt;
// use panic_itm;

use f3::hal::prelude::*;    // provides the memory.x layout
use cortex_m_rt::entry;
use cortex_m::{iprint, iprintln, peripheral::ITM};
use f3::hal::{stm32f30x::{self, USART1, usart1, i2c1, I2C1}, delay::Delay, i2c::I2c};
use f3::Lsm303dlhc;

fn init() -> (&'static mut i2c1::RegisterBlock, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb); // SPLIT the GPIO block into independent pins and registers
    
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
    
    Lsm303dlhc::new(i2c).unwrap(); // Initialise the sensor - usually we would return this, but we want to see the raw i2c operations
    
    let delay = Delay::new(cp.SYST, clocks);
    
    unsafe {(&mut *(I2C1::ptr() as *mut _), delay, cp.ITM)}
}

//slave
const MAGNETOMETER: u8 = 0b001_1110;

//Addresses of magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = init();
    
    loop {
        // Broadcast START
        // Broadcast the Slave address with R/W bit set to Write
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd1().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });
        
        // Wait until sent
        while i2c1.isr.read().txis().bit_is_clear() {}
        
        // Send address of register we want to read
        i2c1.txdr.write(|w| w.txdata().bits(OUT_X_H_M));
        
        // Wait until the previous byte has been transmitted
        while i2c1.isr.read().tc().bit_is_clear() {}
        
        // Broadcast RESART
        // Set R/W bit set to Read
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(6);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });
        
        let mut buffer = [0u8; 6];
        for b in &mut buffer {
            // Wait until we have received something
            while i2c1.isr.read().rxne().bit_is_clear() {}
            
            *b = i2c1.rxdr.read().rxdata().bits();
        }
        
        // Normalise Gyroscope readings
        let x_h = u16::from(buffer[0]);
        let x_l = u16::from(buffer[1]);
        let x = ((x_h << 8) + x_l) as i16;
        
        let z_h = u16::from(buffer[2]);
        let z_l = u16::from(buffer[3]);
        let z = ((z_h << 8) + z_l) as i16;
        
        let y_h = u16::from(buffer[4]);
        let y_l = u16::from(buffer[5]);
        let y = ((y_h << 8) + y_l) as i16;
        
        
        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));
        
        delay.delay_ms(1_000_u16);
    }
    
}
