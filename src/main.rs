#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate f3;
extern crate panic_semihosting;

use core::fmt::Write;

use rt::ExceptionFrame;
use sh::hio;

use f3::hal::delay::Delay;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::led::Leds;

entry!(main);

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

    loop {
        for i in 0..leds.len() {
            let next = (i + 1) % leds.len();
            leds[i].off();
            leds[next].on();
            delay.delay_ms(100_u16);
        }
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
