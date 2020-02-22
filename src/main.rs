#![no_std]
#![no_main]
#![feature(asm)]

use xtensa_lx6_rt as _;

use core::panic::PanicInfo;
use esp8266_hal::ehal::digital::v2::OutputPin;
use esp8266_hal::ehal::timer::CountDown;
use esp8266_hal::gpio::GpioExt;
use esp8266_hal::timer::{Nanoseconds, Timer, TimerExt};

const TEXT: &'static str = "Hello world!\r\n";

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
const CORE_HZ: u32 = 80_000_000;

extern "C" {
    fn rom_i2c_writeReg(block: u8, host_id: u8, reg_add: u8, data: u8);
}

#[no_mangle]
fn main() -> ! {
    // Initialize PLL.
    // I'm not quite sure what this magic incantation means, but it does set the
    // esp8266 to the right clock speed. Without this, it is running too slow.
    unsafe {
        rom_i2c_writeReg(103, 4, 1, 136);
        rom_i2c_writeReg(103, 4, 2, 145);
    }

    let dp = unsafe { esp8266::Peripherals::steal() };
    let pins = dp.GPIO.split();
    let mut pin = pins.gpio2.into_push_pull_output();
    let mut timer = dp.TIMER.timer(CORE_HZ);

    loop {
        pin.set_high().unwrap();
        sleep_ms(100_000_000, &mut timer);
        for byte in TEXT.bytes() {
            dp.UART.uart_fifo.write(|w| unsafe { w.bits(byte as u32) })
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
