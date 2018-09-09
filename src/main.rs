#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate f3;
extern crate panic_semihosting;

use core::fmt::Write;

use rt::{entry, exception, ExceptionFrame};
use sh::hio;

use f3::hal::delay::Delay;
use f3::hal::i2c::I2c;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::led::Leds;
use f3::Lsm303dlhc;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();
    writeln!(stdout, "led 0 -> on").unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut delay = Delay::new(cp.SYST, clocks);
    let mut leds = Leds::new(gpioe);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let i2c1 = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
    let mut lsm = Lsm303dlhc::new(i2c1).unwrap();

    loop {
        for i in 0..leds.len() {
            let next = (i + 1) % leds.len();
            leds[i].off();
            leds[next].on();
            delay.delay_ms(2000_u16);
            let temp = lsm.temp().unwrap();
            writeln!(stdout, "temperature: {}", temp).unwrap();
        }
    }
}

#[exception()]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
