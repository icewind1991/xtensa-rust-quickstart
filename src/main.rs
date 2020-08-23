#![no_std]
#![no_main]
#![feature(asm)]

use xtensa_lx106_rt::entry;

use core::panic::PanicInfo;
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;

#[entry]
fn main() -> ! {
    // The default clock source is the onboard crystal
    // In most cases 40mhz (but can be as low as 2mhz depending on the board)
    // Clock speed is then doubled from the crystal frequency
    let clock_frequency = 80.mhz();

    let dp = unsafe { Peripherals::steal() };
    let pins = dp.GPIO.split();
    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer1, _) = dp.TIMER.timers(clock_frequency);

    led.set_high().unwrap();

    loop {
        timer1.delay_ms(1000);
        led.toggle().unwrap();
    }
}

/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
