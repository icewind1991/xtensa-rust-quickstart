#![no_std]
#![no_main]
#![feature(asm)]

use xtensa_lx106_rt as _;

use core::panic::PanicInfo;
use esp8266_hal::ehal::digital::v2::OutputPin;
use esp8266_hal::ehal::serial::Write;
use esp8266_hal::ehal::timer::CountDown;
use esp8266_hal::gpio::GpioExt;
use esp8266_hal::timer::{Nanoseconds, TimerExt};
use esp8266_hal::uart::UART0Ext;

const TEXT: &'static str = "Hello world!\r\n";

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
/// Clock speed is then doubled from the crystal frequency
const CORE_HZ: u32 = 80_000_000;

#[no_mangle]
fn main() -> ! {
    let dp = unsafe { esp8266::Peripherals::steal() };
    let pins = dp.GPIO.split();
    let mut pin = pins.gpio2.into_push_pull_output();
    let (_, mut timer2) = dp.TIMER.timers(CORE_HZ);
    let mut serial = dp
        .UART0
        .serial(pins.gpio1, pins.gpio3, pins.gpio13, pins.gpio15);

    timer2.start(Nanoseconds(500_000_000));
    loop {
        pin.set_high().unwrap();
        nb::block!(timer2.wait()).unwrap();
        for byte in TEXT.bytes() {
            serial.write(byte).unwrap();
        }
        pin.set_low().unwrap();
        nb::block!(timer2.wait()).unwrap();
    }
}

/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
