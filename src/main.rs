#![no_std]
#![no_main]
#![feature(asm)]

use xtensa_lx106_rt::entry;

use core::panic::PanicInfo;
use esp8266_hal::ehal::digital::v2::OutputPin;
use esp8266_hal::ehal::digital::v2::ToggleableOutputPin;
use esp8266_hal::ehal::timer::{CountDown};
use esp8266_hal::gpio::GpioExt;
use esp8266_hal::timer::{TimerExt, Nanoseconds};

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
/// Clock speed is then doubled from the crystal frequency
const CORE_HZ: u32 = 80_000_000;

#[entry]
fn main() -> ! {
    let dp = unsafe { esp8266::Peripherals::steal() };
    let pins = dp.GPIO.split();
    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer1, _) = dp.TIMER.timers(CORE_HZ);
    timer1.start(Nanoseconds(100_000_000));

    led.set_high().unwrap();

    loop {
        nb::block!(timer1.wait()).unwrap();
        led.toggle().unwrap();
    }
}

/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
