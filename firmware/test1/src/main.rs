#![no_std]
#![no_main]

extern crate panic_halt;
//extern crate panic_semihosting;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate defmt_rtt;
extern crate rtt_target;
extern crate stm32f0xx_hal;
extern crate usb_device;
extern crate usbd_serial;

mod bootloader;
mod usb_serial;

use cortex_m_rt::entry;
#[allow(unused_imports)]
use defmt_rtt as _;
use stm32f0xx_hal::pac::STK;

#[entry]
fn main() -> ! {
    let num = 42;
    defmt::println!("Answer: {}", num);
    usb_serial::main()
}

defmt::timestamp!("{=u32:us}", {
    unsafe { (*STK::ptr()).cvr.read().bits() }
});
