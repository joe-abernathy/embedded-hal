#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use rp2040_pac::generic::Writable;
use rp2040_pac::{Peripherals, io_bank0, IO_BANK0};
use rp2040_pac;
use rp2040_boot2;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[entry]
fn main() -> ! {
    //led_on();
    loop {
        continue;
    }
}

fn led_on() {
    let peripherals = Peripherals::take().unwrap();
    let led = &peripherals.IO_BANK0.gpio25_ctrl;
    led.modify(|_r, w| {
        w.outover().high()
    });
}

fn led_off() {
    let peripherals = Peripherals::take().unwrap();
    let led = &peripherals.IO_BANK0.gpio25_ctrl;
    led.modify(|_r, w| {
        w.outover().low()
    });
}