#![no_std]
#![no_main]
#![feature(asm)]

use xtensa_lx106_rt as _;

use core::panic::PanicInfo;
use esp8266_hal::ehal::digital::v2::OutputPin;
use esp8266_hal::ehal::serial::Write;
use esp8266_hal::ehal::timer::CountDown;
use esp8266_hal::gpio::GpioExt;
use esp8266_hal::timer::{Nanoseconds, Timer, TimerExt};
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
    let mut timer = dp.TIMER.timer(CORE_HZ);
    let mut serial = dp
        .UART0
        .serial(pins.gpio1, pins.gpio3, pins.gpio13, pins.gpio15);

    loop {
        pin.set_high().unwrap();
        sleep_ms(100_000_000, &mut timer);
        for byte in TEXT.bytes() {
            serial.write(byte).unwrap();
        }
        pin.set_low().unwrap();
        sleep_ms(900_000_000, &mut timer);
    }
}

fn sleep_ms(duration: u32, timer: &mut Timer) {
    timer.start(Nanoseconds(duration));
    nb::block!(timer.wait()).unwrap();
}

/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
