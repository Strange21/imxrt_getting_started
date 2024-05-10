
#![no_main]
#![no_std]

use imxrt_hal as hal;
use imxrt_ral as ral;

// extern crate imxrt_rt;
use imxrt1060evk_fcb as _;
use imxrt_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Safety: we're the only code that "owns" the IOMUXC and GPIO1 peripherals.
    let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
    let gpio1 = unsafe { ral::gpio::GPIO1::instance() };
    let spi = unsafe {ral::flexspi::FLEXSPI1::instance()};

    let mut gpio1 = hal::gpio::Port::new(gpio1);
    let pads = hal::iomuxc::into_pads(iomuxc);

    // Configures the pad named "GPIO_11" as a GPIO output.
    let led = gpio1.output(pads.gpio_ad_b0.p08);

    // let spi1 = 
    // Turn on the LED
    // led.set();
    loop {
        cortex_m::asm::delay(500000000);
        led.set();
        cortex_m::asm::delay(500000000);
        led.clear();
    }

}