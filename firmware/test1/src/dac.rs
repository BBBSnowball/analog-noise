use crate::hal::{
    self,
    dac::*,
    gpio::*, gpio::gpioa::*,
    pac::DAC,
    rcc::Rcc,
};

enum Direction {
    Upcounting,
    Downcounting,
}

pub fn test_dac(dac: DAC, pa4: PA4<Input<Floating>>, pa5: PA5<Input<Floating>>, rcc: &mut Rcc) {
    let (pa4, pa5) = cortex_m::interrupt::free(move |cs| (pa4.into_analog(cs), pa5.into_analog(cs)));

    let (mut dac1, mut dac2) = hal::dac::dac(dac, (pa4, pa5), rcc);

    dac1.enable();
    dac2.enable();

    let mut dir = Direction::Upcounting;
    let mut val = 0;

    dac1.set_value(2058);
    dac2.set_value(0);
    cortex_m::asm::bkpt();

    dac1.set_value(4095);
    dac2.set_value(4095);
    cortex_m::asm::bkpt();

    loop {
        dac1.set_value(val);
        match val {
            0 => dir = Direction::Upcounting,
            4095 => dir = Direction::Downcounting,
            _ => (),
        };

        match dir {
            Direction::Upcounting => val += 1,
            Direction::Downcounting => val -= 1,
        }
    }
}
