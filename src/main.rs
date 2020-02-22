#![no_std]
#![no_main]
#![feature(asm)]

use xtensa_lx6_rt as _;

use core::panic::PanicInfo;
use esp8266;
use esp8266::TIMER;

const TEXT: &'static str = "Hello world!\r\n";

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
const CORE_HZ: u32 = 40_000_000;

const BLINKY_GPIO: u32 = 2; // the GPIO hooked up to the onboard LED

#[no_mangle]
fn main() -> ! {
    let dp = unsafe { esp8266::Peripherals::steal() };
    let mut gpio = dp.GPIO;

    // Set pin 2 to function GPIO.
    dp.IO_MUX.io_mux_gpio2.write(|w| unsafe { w.bits(0) });

    // FRC1->CTRL = (1<<7) | (1<<6) | (2<<2);
    // FRC1->LOAD = 0x3fffffu;  // set all 22 bits to 1
    // FRC1->COUNT = 0x3fffffu; // set all 22 bits to 1

    dp.TIMER
        .frc1_ctrl
        .write(|w| unsafe { w.bits((1 << 7) | (1 << 6) | (2 << 2)) });
    dp.TIMER.frc1_load.write(|w| unsafe { w.bits(0x3fffff) });
    dp.TIMER.frc1_count.write(|w| unsafe { w.bits(0x3fffff) });

    configure_pin_as_output(&mut gpio, BLINKY_GPIO);
    loop {
        set_led(&mut gpio, BLINKY_GPIO, true);
        sleep_ns(100000000, &dp.TIMER);
        for byte in TEXT.bytes() {
            dp.UART.uart_fifo.write(|w| unsafe { w.bits(byte as u32) })
        }
        set_led(&mut gpio, BLINKY_GPIO, false);
        sleep_ns(500000000, &dp.TIMER);
    }
}

pub fn set_led(reg: &mut esp8266::GPIO, idx: u32, val: bool) {
    if val {
        reg.gpio_out_w1ts
            .modify(|_, w| unsafe { w.bits(0x1 << idx) });
    } else {
        reg.gpio_out_w1tc
            .modify(|_, w| unsafe { w.bits(0x1 << idx) });
    }
}

/// Configure the pin as an output
pub fn configure_pin_as_output(reg: &mut esp8266::GPIO, gpio: u32) {
    reg.gpio_enable_w1ts
        .modify(|_, w| unsafe { w.bits(0x1 << gpio) });
}

pub fn sleep_ns(ns: u64, timer: &TIMER) {
    // 3600 = 1e9 / (80MHz / 256)
    let ticks = (ns / 3600) as u32;
    let start = timer.frc1_count.read().bits();
    loop {
        if (start - timer.frc1_count.read().bits() & 0x3fffff) > ticks {
            break;
        }
    }
}

/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
