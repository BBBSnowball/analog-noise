#![no_std]
#![no_main]

extern crate panic_halt;
//extern crate panic_semihosting;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
//extern crate defmt_rtt;
extern crate rtt_target;
extern crate stm32f0xx_hal;
extern crate usb_device;
extern crate usbd_serial;

mod bootloader;
mod ims;
mod usb_serial;

use stm32f0xx_hal as hal;
pub use stm32f0xx_hal::pac as pac;

use cortex_m_rt::entry;
//#[allow(unused_imports)]
//use defmt_rtt as _;
//use stm32f0xx_hal::pac::STK;

use cortex_m_semihosting::hio;
use core::fmt::Write;
use crate::hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().map_err(|_| core::fmt::Error).unwrap();
    let num = 42;
    write!(stdout, "Answer: {}\r\n", num).unwrap();

    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp
        .RCC
        .configure()
        .hsi48()
        .enable_crs(dp.CRS)
        .sysclk(48.mhz())
        .pclk(24.mhz())
        .freeze(&mut dp.FLASH);
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let mut ims = ims::IMS::new(gpiob.pb12, gpioa.pa9, gpiob.pb13, gpiob.pb14, gpiob.pb15, dp.SPI2, &mut rcc); 
    if let Err(err) = ims::test_ims(stdout, &mut ims) {
        write!(stdout, "ERROR: {:?}\r\n", err).unwrap();
    }

    usb_serial::main(usb_serial::UsbHardware {
        led_pin: gpiob.pb2,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
        usb: dp.USB,
    });
}

//defmt::timestamp!("{=u32:us}", {
//    unsafe { (*STK::ptr()).cvr.read().bits() }
//});
