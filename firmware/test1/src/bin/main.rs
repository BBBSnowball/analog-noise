#![no_std]
#![no_main]

extern crate analog_noise_test1;
extern crate cortex_m_rt;
extern crate rtt_target;
extern crate usb_device;
extern crate usbd_serial;

use analog_noise_test1::{dac, epd, ims, touch, usb_serial};
use analog_noise_test1::hal::{self, pac};
use analog_noise_test1::hal::delay::Delay;
use analog_noise_test1::hal::gpio::*;
use analog_noise_test1::hal::prelude::*;
use rtt_target::rprintln;

use cortex_m_rt::entry;

fn test_pwm(led2: gpiob::PB3<Input<Floating>>, led3: gpiob::PB4<Input<Floating>>, _tim2: pac::TIM2, tim3: pac::TIM3, rcc: &mut hal::rcc::Rcc, delay: &mut Delay) {
    let (_led2, led3) = cortex_m::interrupt::free(move |cs| {
        (
            led2.into_alternate_af2(cs),
            led3.into_alternate_af1(cs),
        )
    });

    //let pwm2 = hal::pwm::tim2(tim2, led2, rcc, 20u32.khz());
    let mut led3 = hal::pwm::tim3(tim3, led3, rcc, 20u32.khz());
    let max_duty = led3.get_max_duty();  // 2400
    led3.set_duty(max_duty / 2);
    led3.enable();
    for _ in 0..4 {
        for i in 0..max_duty {
            led3.set_duty(i);
            delay.delay_ms(1_u16);
        }
    }
}

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
    let gpioc = dp.GPIOC.split(&mut rcc);
    let gpiof = dp.GPIOF.split(&mut rcc);

    let mut delay = Delay::new(cp.SYST, &rcc);

    dac::test_dac(dp.DAC, gpioa.pa4, gpioa.pa5, dp.TIM6, &mut rcc);

    test_pwm(gpiob.pb3, gpiob.pb4, dp.TIM2, dp.TIM3, &mut rcc, &mut delay);

    if false {
        let mut ims = ims::IMS::new(gpiob.pb12, gpioa.pa9, gpiob.pb13, gpiob.pb14, gpiob.pb15, dp.SPI2, &mut rcc); 
        if let Err(err) = ims::test_ims(&mut ims) {
            rprintln!("ERROR: {:?}", err);
        }

        touch::test_touch(dp.TSC, &mut rcc, gpioa.pa6, gpioa.pa7, gpioa.pa2, gpioa.pa0);

        ims::start_writing_to_rtt(ims, channels.up.1, &mut dp.SYSCFG, &mut dp.EXTI, &mut cp.NVIC);
    } else {
        let mut epd = epd::EpaperDisplay::new(
            gpioc.pc15, gpiof.pf0, gpiof.pf1, gpioc.pc14, gpiob.pb13, gpiob.pb14, gpiob.pb15, dp.SPI2, &mut rcc, &mut delay);
        if let Err(err) = epd::test_epd(&mut epd, &mut delay) {
            rprintln!("ERROR: {:?}", err);
        }
    }

    usb_serial::main(usb_serial::UsbHardware {
        led_pin: gpiob.pb2,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
        usb: dp.USB,
    });
}
