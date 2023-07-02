#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use rp2040_pac as pac;
use rp2040_boot2;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    unsafe {
        setup_chip(&mut p);
    }

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 8_000_000);
    
    /*p.SIO.gpio_oe_clr.write(|w| unsafe {
        w.bits(1 << 25)
    });
    
    p.SIO.gpio_out_clr.write(|w| unsafe {
        w.bits(1 << 25)
    });*/

    p.PADS_BANK0.gpio[25].write(|w| {
        w.od().clear_bit();
        w.ie().set_bit()
    });

    p.IO_BANK0.gpio[25].gpio_ctrl.write(|w| {
        w.funcsel().sio()
    });

    p.SIO.gpio_oe_set.write(|w| unsafe {
        w.bits(1 << 25)
    });

    p.SIO.gpio_out_set.write(|w| unsafe {
        w.bits(1 << 25)
    });

    //led_on();
    loop {
        continue;
    }
}

fn led_on() {
    let p = rp2040_pac::Peripherals::take().unwrap();
    let oe_set = &p.SIO.gpio_oe_set;
    let out_set = &p.SIO.gpio_out_set;
    let led_ctrl = &p.IO_BANK0.gpio[25].gpio_ctrl;
    
    p.SIO.gpio_oe_clr.write(|w| unsafe {
        w.bits(1 << 25)
    });

    p.SIO.gpio_out_clr.write(|w| unsafe {
        w.bits(1 << 25)
    });

    p.PADS_BANK0.gpio[25].write(|w| {
        w.od().clear_bit();
        w.ie().set_bit()
    });

    p.IO_BANK0.gpio[25].gpio_ctrl.write(|w| {
        w.funcsel().sio()
    });

    p.SIO.gpio_oe_set.write(|w| unsafe {
        w.bits(1 << 25)
    });

    p.SIO.gpio_out_set.write(|w| unsafe {
        w.bits(1 << 25)
    });
}

unsafe fn setup_chip(p: &mut rp2040_pac::Peripherals) {
    // Now reset all the peripherals, except QSPI and XIP (we're using those
    // to execute from external flash!)
    p.RESETS.reset.write(|w| {
        w.adc().set_bit();
        w.busctrl().set_bit();
        w.dma().set_bit();
        w.i2c0().set_bit();
        w.i2c1().set_bit();
        w.io_bank0().set_bit();
        w.io_qspi().clear_bit();
        w.jtag().set_bit();
        w.pads_bank0().set_bit();
        w.pads_qspi().clear_bit();
        w.pio0().set_bit();
        w.pio1().set_bit();
        w.pll_sys().clear_bit();
        w.pll_usb().clear_bit();
        w.pwm().set_bit();
        w.rtc().set_bit();
        w.spi0().set_bit();
        w.spi1().set_bit();
        w.syscfg().set_bit();
        w.sysinfo().set_bit();
        w.tbman().set_bit();
        w.timer().set_bit();
        w.uart0().set_bit();
        w.uart1().set_bit();
        w.usbctrl().set_bit();
        w
    });

    const RESETS_RESET_BITS: u32 = 0x01ffffff;
    const RESETS_RESET_USBCTRL_BITS: u32 = 0x01000000;
    const RESETS_RESET_UART1_BITS: u32 = 0x00800000;
    const RESETS_RESET_UART0_BITS: u32 = 0x00400000;
    const RESETS_RESET_SPI1_BITS: u32 = 0x00020000;
    const RESETS_RESET_SPI0_BITS: u32 = 0x00010000;
    const RESETS_RESET_RTC_BITS: u32 = 0x00008000;
    const RESETS_RESET_ADC_BITS: u32 = 0x00000001;

    // We want to take everything out of reset, except these peripherals:
    //
    // * ADC
    // * RTC
    // * SPI0
    // * SPI1
    // * UART0
    // * UART1
    // * USBCTRL
    //
    // These must stay in reset until the clocks are sorted out.
    const PERIPHERALS_TO_UNRESET: u32 = RESETS_RESET_BITS
        & !(RESETS_RESET_ADC_BITS
            | RESETS_RESET_RTC_BITS
            | RESETS_RESET_SPI0_BITS
            | RESETS_RESET_SPI1_BITS
            | RESETS_RESET_UART0_BITS
            | RESETS_RESET_UART1_BITS
            | RESETS_RESET_USBCTRL_BITS);

    // Write 0 to the reset field to take it out of reset
    p.RESETS.reset.modify(|_r, w| {
        w.busctrl().clear_bit();
        w.dma().clear_bit();
        w.i2c0().clear_bit();
        w.i2c1().clear_bit();
        w.io_bank0().clear_bit();
        w.io_qspi().clear_bit();
        w.jtag().clear_bit();
        w.pads_bank0().clear_bit();
        w.pads_qspi().clear_bit();
        w.pio0().clear_bit();
        w.pio1().clear_bit();
        w.pll_sys().clear_bit();
        w.pll_usb().clear_bit();
        w.pwm().clear_bit();
        w.syscfg().clear_bit();
        w.sysinfo().clear_bit();
        w.tbman().clear_bit();
        w.timer().clear_bit();
        w
    });

    while (!p.RESETS.reset_done.read().bits() & PERIPHERALS_TO_UNRESET) != 0 {
        cortex_m::asm::nop();
    }
}