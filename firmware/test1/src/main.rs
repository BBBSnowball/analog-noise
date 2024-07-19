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
mod usb_serial;

use cortex_m_rt::entry;
#[allow(unused_imports)]
//use defmt_rtt as _;
use stm32f0xx_hal::pac::STK;

use cortex_m_semihosting::hio;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().map_err(|_| core::fmt::Error).unwrap();
    let num = 42;
    write!(stdout, "Answer: {}\r\n", num).unwrap();
    usb_serial::main()
}

//defmt::timestamp!("{=u32:us}", {
//    unsafe { (*STK::ptr()).cvr.read().bits() }
//});
