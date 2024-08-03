//use stm32f0xx_hal::pac::dma1;

use crate::hal::{
    self,
    dac::*,
    gpio::*, gpio::gpioa::*,
    pac::DAC,
    pac::DMA1,
    pac::TIM6,
    rcc::Rcc,
    time::*,
};

enum Direction {
    Upcounting,
    Downcounting,
}

/*
const fn genSine<const n: usize>(offset: u16, amplitude: u16) -> [u16; n] {
    let mut buf = [0; n];
    //for i in 0..n {
    let mut i =0;
    while i < n {
        buf[n] = offset + ((amplitude as f32) * ((i as f32)*2.*consts::PI/(n as f32))) as u16;
        i += 1;
    }
    return buf;
}
*/

// see gen_sine.py
static SINE: [u16; 512] = [
    2048, 2069, 2091, 2113, 2135, 2157, 2179, 2201, 2223, 2245, 2267, 2289, 2310, 2332, 2354, 2376,
    2397, 2419, 2440, 2462, 2483, 2504, 2525, 2547, 2568, 2589, 2610, 2630, 2651, 2672, 2692, 2713,
    2733, 2754, 2774, 2794, 2814, 2834, 2853, 2873, 2892, 2912, 2931, 2950, 2969, 2988, 3006, 3025,
    3043, 3061, 3079, 3097, 3115, 3133, 3150, 3167, 3184, 3201, 3218, 3235, 3251, 3267, 3283, 3299,
    3315, 3330, 3345, 3360, 3375, 3390, 3404, 3419, 3433, 3447, 3460, 3474, 3487, 3500, 3513, 3525,
    3537, 3550, 3561, 3573, 3585, 3596, 3607, 3617, 3628, 3638, 3648, 3658, 3667, 3677, 3686, 3695,
    3703, 3711, 3719, 3727, 3735, 3742, 3749, 3756, 3762, 3769, 3775, 3780, 3786, 3791, 3796, 3801,
    3805, 3809, 3813, 3817, 3820, 3823, 3826, 3829, 3831, 3833, 3835, 3836, 3837, 3838, 3839, 3839,
    3839, 3839, 3839, 3838, 3837, 3836, 3835, 3833, 3831, 3829, 3826, 3823, 3820, 3817, 3813, 3809,
    3805, 3801, 3796, 3791, 3786, 3780, 3775, 3769, 3762, 3756, 3749, 3742, 3735, 3727, 3719, 3711,
    3703, 3695, 3686, 3677, 3667, 3658, 3648, 3638, 3628, 3617, 3607, 3596, 3585, 3573, 3561, 3550,
    3537, 3525, 3513, 3500, 3487, 3474, 3460, 3447, 3433, 3419, 3404, 3390, 3375, 3360, 3345, 3330,
    3315, 3299, 3283, 3267, 3251, 3235, 3218, 3201, 3184, 3167, 3150, 3133, 3115, 3097, 3079, 3061,
    3043, 3025, 3006, 2988, 2969, 2950, 2931, 2912, 2892, 2873, 2853, 2834, 2814, 2794, 2774, 2754,
    2733, 2713, 2692, 2672, 2651, 2630, 2610, 2589, 2568, 2547, 2525, 2504, 2483, 2462, 2440, 2419,
    2397, 2376, 2354, 2332, 2310, 2289, 2267, 2245, 2223, 2201, 2179, 2157, 2135, 2113, 2091, 2069,
    2048, 2026, 2004, 1982, 1960, 1938, 1916, 1894, 1872, 1850, 1828, 1806, 1785, 1763, 1741, 1719,
    1698, 1676, 1655, 1633, 1612, 1591, 1570, 1548, 1527, 1506, 1485, 1465, 1444, 1423, 1403, 1382,
    1362, 1341, 1321, 1301, 1281, 1261, 1242, 1222, 1203, 1183, 1164, 1145, 1126, 1107, 1089, 1070,
    1052, 1034, 1016, 998, 980, 962, 945, 928, 911, 894, 877, 860, 844, 828, 812, 796,
    780, 765, 750, 735, 720, 705, 691, 676, 662, 648, 635, 621, 608, 595, 582, 570,
    558, 545, 534, 522, 510, 499, 488, 478, 467, 457, 447, 437, 428, 418, 409, 400,
    392, 384, 376, 368, 360, 353, 346, 339, 333, 326, 320, 315, 309, 304, 299, 294,
    290, 286, 282, 278, 275, 272, 269, 266, 264, 262, 260, 259, 258, 257, 256, 256,
    256, 256, 256, 257, 258, 259, 260, 262, 264, 266, 269, 272, 275, 278, 282, 286,
    290, 294, 299, 304, 309, 315, 320, 326, 333, 339, 346, 353, 360, 368, 376, 384,
    392, 400, 409, 418, 428, 437, 447, 457, 467, 478, 488, 499, 510, 522, 534, 545,
    558, 570, 582, 595, 608, 621, 635, 648, 662, 676, 691, 705, 720, 735, 750, 765,
    780, 796, 812, 828, 844, 860, 877, 894, 911, 928, 945, 962, 980, 998, 1016, 1034,
    1052, 1070, 1089, 1107, 1126, 1145, 1164, 1183, 1203, 1222, 1242, 1261, 1281, 1301, 1321, 1341,
    1362, 1382, 1403, 1423, 1444, 1465, 1485, 1506, 1527, 1548, 1570, 1591, 1612, 1633, 1655, 1676,
    1698, 1719, 1741, 1763, 1785, 1806, 1828, 1850, 1872, 1894, 1916, 1938, 1960, 1982, 2004, 2026,
];

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
        .tsel1().tim6_trgo()
        //.wave1().triangle()
        //.mamp1().bits(10)  // 0..2047
        .wave1().noise()
        .mamp1().bits(7)
        //.dmaen1().clear_bit()
        .dmaen1().set_bit()
        .dmaudrie1().clear_bit()

        .en2().set_bit()
        .boff2().clear_bit()  // output buffer shouldn't be needed
        .ten2().set_bit()
        //.ten2().clear_bit()
        .tsel2().tim6_trgo()
        .wave2().noise()
        .mamp2().bits(11)  // 0..2047
        .dmaen2().clear_bit()
        .dmaudrie2().clear_bit()
    );
    // output value is offset for generated waveform, so set it to zero
    dac.dhr12rd.write(|w| w.dacc1dhr().bits(0).dacc2dhr().bits(0));

    let rcc_regs = unsafe { &(*hal::pac::RCC::ptr()) };
    // enable DMA clock
    rcc_regs.ahbenr.modify(|_, w| w.dmaen().set_bit());
    // There doesn't seem to be any reset for DMA..?

    // DMA1, channel3 is for TIM6/DAC, channel4 is for TIM7/DAC
    let dma1_regs = unsafe { &(*DMA1::ptr()) };
    dma1_regs.ch3.cr.write(|w| w.en().disabled());
    dma1_regs.ch3.mar.write(|w| unsafe { w.ma().bits(SINE.as_ptr() as u32) });
    dma1_regs.ch3.ndtr.write(|w| w.ndt().bits(SINE.len() as u16));
    dma1_regs.ch3.par.write(|w| unsafe { w.pa().bits(dac.dhr12r1.as_ptr() as u32) });
    dma1_regs.ch3.cr.write(|w|
        w.msize().bits16()
        .psize().bits16()
        .minc().enabled()
        .pinc().disabled()
        .circ().enabled()
        .dir().from_memory()
        .en().disabled()
    );
    dma1_regs.ch3.cr.write(|w|
        w.msize().bits16()
        .psize().bits16()
        .minc().enabled()
        .pinc().disabled()
        .circ().enabled()
        .dir().from_memory()
        .en().enabled()
    );
}
