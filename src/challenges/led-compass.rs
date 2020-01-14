#![allow(unused_imports)]
#![no_main]
#![no_std]

// The following imports are important for compilation
use panic_halt;

// Let's use math to detemine the correct direction
use core::f32::consts::PI;

use f3::hal::prelude::*;    // provides the memory.x layout
use cortex_m_rt::entry;
use cortex_m::{iprint, iprintln, peripheral::ITM};
use f3::hal::{stm32f30x::{self, USART1, usart1, I2C1, i2c1}, delay::Delay, i2c::I2c};
use f3::hal::gpio::{AF4, gpiob::{PB6, PB7}};
use f3::led::{Direction, Leds};

// Unlike exercise 07 - here we are using the external crate to provide
// the abstraction of the LSM sensor. As you can see in our init routine below
// this means we must specify more information on which peripherals/pins (and their modes)
// are used to initialise the sensor.
use lsm303dlhc::{Lsm303dlhc, I16x3};

use m::Float; // this trait provides the `atan2` method

fn init() -> (Leds, Lsm303dlhc<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>>, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb); // SPLIT the GPIO block into independent pins and registers
    
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
    
    let lsm303 = Lsm303dlhc::new(i2c).unwrap(); // Initialise the sensor - usually we would return this, but we want to see the raw i2c operations
    let delay = Delay::new(cp.SYST, clocks);
    let leds = Leds::new(gpioe);
    
    (leds, lsm303, delay, cp.ITM)
}


#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303, mut delay, mut itm) = init();
    
    loop {
        let I16x3 {x, y, ..} = lsm303.mag().unwrap(); 
        
        let theta = (y as f32).atan2(x as f32); // radians
        let dir = if theta < ((-7.0 * PI) / 8.0) {
            Direction::North
        } else if theta < ((-5.0 * PI) / 8.0) {
            Direction::Northwest
        } else if theta < ((-3.0 * PI) / 8.0) {
            Direction::West
        } else if theta < ((-PI) / 8.0) {
            Direction::Southwest
        } else if theta < ((PI) / 8.0) {
            Direction::South
        } else if theta < ((3.0 * PI) / 8.0) {
            Direction::Southeast
        } else if theta < ((5.0 * PI) / 8.0) {
            Direction::East
        } else if theta < ((7.0 * PI) / 8.0) {
            Direction::Northeast
        } else {
            Direction::North
        };
        
        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();
        
        iprintln!(&mut itm.stim[0], "{:?}", lsm303.mag().unwrap());
        
        delay.delay_ms(1_000_u16);
    }
    
}
