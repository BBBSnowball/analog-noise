// copied from https://github.com/stm32-rs/stm32f0xx-hal/blob/master/examples/usb_serial.rs
// and modified a lot

//! CDC-ACM serial port example using polling in a busy loop.

use bootloader;
use crate::hal::usb::{Peripheral, UsbBus};
use crate::hal::prelude::*;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use crate::hal::gpio::*;

pub struct UsbHardware {
    pub led_pin: gpiob::PB2<Input<Floating>>,
    pub pin_dm: gpioa::PA11<Input<Floating>>,
    pub pin_dp: gpioa::PA12<Input<Floating>>,
    pub usb: crate::pac::USB,
}

pub fn main(hw: UsbHardware) -> ! {
    let UsbHardware { led_pin, pin_dm, pin_dp, usb } = hw;
    let mut led = cortex_m::interrupt::free(|cs| led_pin.into_push_pull_output(cs));
    led.set_low().ok(); // Turn off

    let usb = Peripheral { usb, pin_dm, pin_dp };

    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        if serial.line_coding().data_rate() == 1200 {
            bootloader::jump_to_bootloader()
        }
    
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led.set_high().ok(); // Turn on

                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        led.set_low().ok(); // Turn off
    }
}
