use crate::hal::{
    self,
    dac::*,
    gpio::*, gpio::gpioa::*,
    pac::DAC,
    pac::TIM6,
    rcc::Rcc,
    time::*,
};

enum Direction {
    Upcounting,
    Downcounting,
}

pub fn test_dac(dac: DAC, pa4: PA4<Input<Floating>>, pa5: PA5<Input<Floating>>, tim6: TIM6, rcc: &mut Rcc) {
    let (pa4, pa5) = cortex_m::interrupt::free(move |cs| (pa4.into_analog(cs), pa5.into_analog(cs)));

    let (mut dac1, mut dac2) = hal::dac::dac(dac, (pa4, pa5), rcc);

    dac1.enable();
    dac2.enable();

    let mut dir = Direction::Upcounting;
    let mut val = 0;

    if false {
        dac1.set_value(2058);
        dac2.set_value(0);
        cortex_m::asm::bkpt();

        dac1.set_value(4095);
        dac2.set_value(4095);
        cortex_m::asm::bkpt();
    }

    if false {
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

    let _tim6 = hal::timers::Timer::tim6(tim6, 48.khz(), rcc);
    let tim6_regs = unsafe { &(*TIM6::ptr()) };
    tim6_regs.cr2.write(|w| w.mms().update());

    let dac = unsafe { &(*DAC::ptr()) };
    //dac.dhr12r1.write(|w| unsafe { w.bits(val as u32) });
    //dac.dhr12rd.write(|w| w.dacc1dhr().bits(512).dacc2dhr().bits(1024));

    dac.cr.write(|w| w
        .en1().set_bit()
        .boff1().clear_bit()  // output buffer shouldn't be needed
        .ten1().set_bit()
        //.ten1().clear_bit()
        .tsel1().tim6_trgo()
        .wave1().triangle()
        .mamp1().bits(10)  // 0..2047
        .dmaen1().clear_bit()
        .dmaudrie1().clear_bit()

        .en2().set_bit()
        .boff2().clear_bit()  // output buffer shouldn't be needed
        .ten2().set_bit()
        //.ten2().clear_bit()
        .tsel2().tim6_trgo()
        .wave2().noise()
        .mamp2().bits(10)  // 0..2047
        .dmaen2().clear_bit()
        .dmaudrie2().clear_bit()
    );
    dac.dhr12rd.write(|w| w.dacc1dhr().bits(0).dacc2dhr().bits(0));
}
