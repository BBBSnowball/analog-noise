#![no_std]
#![no_main]

extern crate panic_halt;
//extern crate panic_semihosting;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f0xx_hal;
extern crate usb_device;
extern crate usbd_serial;

mod bootloader;
mod usb_serial;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    usb_serial::main()
}
