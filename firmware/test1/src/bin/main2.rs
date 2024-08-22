#![no_std]
#![no_main]

// This is similar to main but without USB and some other features
// and we use a lower clock, so this should hopefully be more efficient.

extern crate analog_noise_test1;
extern crate cortex_m_rt;
extern crate rtt_target;
extern crate usb_device;
extern crate usbd_serial;

use analog_noise_test1::{dac, ims};
use analog_noise_test1::hal::pac;
use analog_noise_test1::hal::gpio::*;
use analog_noise_test1::hal::prelude::*;
use analog_noise_test1::spi::*;
use rtt_target::rprintln;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    //let mut cp = cortex_m::peripheral::Peripherals::take().unwrap();
    dp.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());  // Enable clock for SYSCFG (used for EXTI?)
    let mut rcc = dp
        .RCC
        .configure()
        //.sysclk(500.khz())
        .sysclk(8.mhz())  // same as HSI to disable PLL
        .pclk(500.khz())
        .freeze(&mut dp.FLASH);
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    dac::test_dac(dp.DAC, gpioa.pa4, gpioa.pa5, dp.TIM6, &mut rcc);

    let spi = TimesharedSpi::new(gpiob.pb13, gpiob.pb14, gpiob.pb15, dp.SPI2, &mut rcc);

    let pb12 = gpiob.pb12;
    let cs = cortex_m::interrupt::free(move |cs| {
        pb12.into_push_pull_output(cs)
    });
    let spidev = spi.make_device(cs);

    let mut ims = ims::IMS::new(spidev, gpioa.pa9); 
    if let Err(err) = ims::sleep(&mut ims) {
        rprintln!("ERROR: {:?}", err);
    }

    loop {
        cortex_m::asm::wfi();
    }
}
