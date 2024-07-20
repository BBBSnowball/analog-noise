#![no_std]
#![no_main]

extern crate panic_halt;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate rtt_target;
extern crate stm32f0xx_hal;
extern crate usb_device;
extern crate usbd_serial;

mod bootloader;
mod ims;
mod touch;
mod usb_serial;

use rtt_target::rprintln;
use stm32f0xx_hal as hal;
pub use stm32f0xx_hal::pac as pac;
use crate::hal::prelude::*;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let channels = rtt_target::rtt_init! {
        up: {
            0: {
                size: 1024,
                name: "Terminal"
            }
            1: {
                size: 128,
                // skip data if necessary but only full blocks
                mode: rtt_target::ChannelMode::NoBlockSkip,
                name: "IMS"
            }
        }
        down: {
            0: {
                size: 16,
                name: "Terminal"
            }
            1: {
                size: 16,
                name: "dummy"
            }
        }
    };
    rtt_target::set_print_channel(channels.up.0);
    let num = 42;
    rprintln!("Answer: {}", num);


    let mut dp = pac::Peripherals::take().unwrap();
    let mut cp = cortex_m::peripheral::Peripherals::take().unwrap();
    dp.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());  // Enable clock for SYSCFG (used for EXTI?)
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
    if let Err(err) = ims::test_ims(&mut ims) {
        rprintln!("ERROR: {:?}", err);
    }

    touch::test_touch(dp.TSC, &mut rcc, gpioa.pa6, gpioa.pa7, gpioa.pa2, gpioa.pa0);

    ims::start_writing_to_rtt(ims, channels.up.1, &mut dp.SYSCFG, &mut dp.EXTI, &mut cp.NVIC);

    usb_serial::main(usb_serial::UsbHardware {
        led_pin: gpiob.pb2,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
        usb: dp.USB,
    });
}
